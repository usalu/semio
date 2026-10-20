#!/usr/bin/env python3
"""🔀 Anchored, all-or-nothing applier for facade repoint plans (`staged/plans/<tag>.json`).

Default is a dry run. `--write` validates every file first (sha256 of the current text, the exact old text of every edit span, the sha256 of the result), then writes all files atomically (temp + replace) with first-touch backups in the ledger, re-reads and verifies every result, and rolls everything back on any mismatch. A second run on an applied tree refuses. Pure standard library, so it runs on macOS, Linux, Windows and in the devcontainer.

usage: apply.py --plan <tag>[,<tag>...] [--root DIR] [--write] [--ledger DIR] [--status] [--files] [--only-crates a,b]"""
import argparse, hashlib, json, os, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_ROOT = os.path.abspath(HERE + "/" + "/".join([".."] * 9))


def sha(b):
    return hashlib.sha256(b).hexdigest()


def load_plans(tags):
    out = []
    for t in tags.split(","):
        out.append(json.load(open(os.path.join(HERE, "plans", t + ".json"), encoding="utf-8")))
    return out


def collect(plans, only):
    """{path: [entry, ...]} in plan order; several plans may touch one file or manifest and are applied one after the other."""
    files, manifests = {}, {}
    for p in plans:
        for f in p["files"]:
            if only and not (set(f["crates"]) & only):
                continue
            files.setdefault(f["path"], []).append(f)
        for m in p["manifests"]:
            if only and m["crate"] not in only:
                continue
            manifests.setdefault(m["path"], []).append(m)
    return files, manifests


def occurrences(text, sub):
    out, i = [], text.find(sub)
    while i >= 0:
        out.append(i)
        i = text.find(sub, i + 1)
    return out


def new_text_of_file(old, f, pristine):
    """Applies the planned edits. `pristine` is True when the file still has the planned digest (offsets are exact); otherwise every edit is rebased onto the n-th occurrence of its old text, which must exist as often as when the plan was made."""
    spans = []
    for a, b, o, n, k, total in f["edits"]:
        if pristine:
            if old[a:b] != o:
                raise ValueError(f"span {a}:{b} differs from the planned text")
            spans.append((a, b, n))
        else:
            occ = occurrences(old, o)
            if len(occ) != total:
                raise ValueError(f"{o[:50]!r} occurs {len(occ)} times, planned {total}")
            spans.append((occ[k], occ[k] + len(o), n))
    spans.sort(reverse=True)
    for (a, b, _), nxt in zip(spans, spans[1:]):
        if nxt[1] > a:
            raise ValueError("overlapping edits after rebase")
    new = old
    for a, b, n in spans:
        new = new[:a] + n + new[b:]
    return new


def new_text_of_manifest(old, m):
    new = old
    for op in m["ops"]:
        if new.count(op["old"]) != 1:
            raise ValueError(f"manifest line {op['old'][:60]!r} occurs {new.count(op['old'])} times")
        new = new.replace(op["old"], op["new"])
    return new


def validate(root, files, manifests):
    problems, applied, results, rebased = [], 0, {}, []
    for kind, table in (("file", files), ("manifest", manifests)):
        for rel, entries in table.items():
            try:
                raw = open(os.path.join(root, rel), "rb").read()
            except OSError as e:
                problems.append(f"{rel}: {e.strerror}")
                continue
            text, done, err = raw.decode("utf-8"), 0, None
            for e in entries:
                cur = sha(text.encode("utf-8"))
                if cur == e["sha256_after"]:
                    done += 1
                    continue
                try:
                    if kind == "file":
                        pristine = cur == e["sha256"]
                        text = new_text_of_file(text, e, pristine)
                        if pristine and sha(text.encode("utf-8")) != e["sha256_after"]:
                            raise ValueError("result digest differs from the plan")
                    elif cur == e["sha256"]:
                        text = e["new_text"]
                        pristine = True
                    else:
                        text = new_text_of_manifest(text, e)
                        pristine = False
                    if not pristine and rel not in rebased:
                        rebased.append(rel)
                except ValueError as ex:
                    err = str(ex)
                    break
            if err:
                problems.append(f"{rel}: {err}")
            elif done == len(entries):
                applied += 1
            elif done:
                problems.append(f"{rel}: partly applied")
            else:
                results[rel] = text
    return problems, applied, results, rebased


def write_all(root, results, ledger):
    os.makedirs(ledger, exist_ok=True)
    backups = {}
    try:
        for rel, new in results.items():
            p = os.path.join(root, rel)
            b = os.path.join(ledger, "backup", rel)
            os.makedirs(os.path.dirname(b), exist_ok=True)
            if not os.path.exists(b):
                with open(p, "rb") as s, open(b, "wb") as d:
                    d.write(s.read())
            backups[rel] = b
        for rel, new in results.items():
            p = os.path.join(root, rel)
            fd, tmp = tempfile.mkstemp(dir=os.path.dirname(p), prefix=".gh-")
            with os.fdopen(fd, "wb") as w:
                w.write(new.encode("utf-8"))
            os.chmod(tmp, os.stat(p).st_mode & 0o7777 if os.path.exists(p) else 0o644)
            os.replace(tmp, p)
        for rel, new in results.items():
            if open(os.path.join(root, rel), "rb").read() != new.encode("utf-8"):
                raise RuntimeError(rel + ": read-back mismatch")
    except Exception:
        for rel, b in backups.items():
            try:
                with open(b, "rb") as s, open(os.path.join(root, rel), "wb") as d:
                    d.write(s.read())
            except OSError:
                pass
        raise
    json.dump(sorted(results), open(os.path.join(ledger, "ledger.json"), "w"), indent=1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--plan", required=True)
    ap.add_argument("--root", default=DEFAULT_ROOT)
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--ledger", default=None)
    ap.add_argument("--status", action="store_true")
    ap.add_argument("--files", action="store_true", help="list the files the plan touches")
    ap.add_argument("--only-crates", default="")
    a = ap.parse_args()
    plans = load_plans(a.plan)
    only = set(x for x in a.only_crates.split(",") if x)
    files, manifests = collect(plans, only)
    if a.files:
        print("\n".join(sorted(list(files) + list(manifests))))
        return 0
    problems, applied, results, rebased = validate(a.root, files, manifests)
    total = len(files) + len(manifests)
    print(f"plan {a.plan}: {len(files)} source files, {len(manifests)} manifests; already applied {applied}; to write {len(results)} ({len(rebased)} rebased onto concurrent edits); problems {len(problems)}")
    for p in problems[:20]:
        print("  ", p)
    if a.status:
        return 0
    if problems:
        print("refused: nothing written")
        return 1
    if applied and applied != total:
        print("refused: the tree is partly applied")
        return 1
    if applied == total:
        print("refused: already applied")
        return 1
    if not a.write:
        print("dry run valid (use --write)")
        return 0
    ledger = a.ledger or os.path.join(tempfile.gettempdir(), "gran-hubs-ledger")
    write_all(a.root, results, ledger)
    print(f"written {len(results)} files; backups in {ledger}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
