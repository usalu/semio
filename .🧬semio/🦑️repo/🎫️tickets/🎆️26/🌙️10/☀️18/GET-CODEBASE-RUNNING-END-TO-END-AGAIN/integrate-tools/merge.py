#!/usr/bin/env python3
# 🔀 Folds the per-set results (runset.py) into ONE integrated tree, file by file, in apply order.
# usage: merge.py [--order a,b,c] [--res DIR] [--base DIR] [--out DIR] [--resolutions DIR]
# A file touched by one set is taken verbatim. A file touched by several is folded with `git merge-file` (3-way, base = live snapshot):
#   1. line level; 2. token level (split after ; , { } ( ) and newline) for the long single-line modules of the Bazel adapter;
#   3. a hand resolution in resolutions/<key>-<set>.blocks.json, valid only for the exact token conflict it was written for (sha256 of the conflict output).
# After the fold, semantic.py patches (anchored, exact counts) integrate overlaps that merge cleanly but do not work together.
import hashlib, json, os, re, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ORDER = "cache-fix,rcache,rexec,bworkers,bzl-coverage,bzl-rust,bzl-xplat,zt-e2e-fix,tax6,vit4".split(",")
TOKEN = re.compile(r"[^;,{}()\n]*[;,{}()\n]|[^;,{}()\n]+$")
NL = "␤"
BLOCK = re.compile(r"<<<<<<< integrated\n(.*?)\|\|\|\|\|\|\| base\n(.*?)=======\n(.*?)>>>>>>> [^\n]*\n", re.S)


def key_of(rel):
    return hashlib.sha256(rel.encode()).hexdigest()[:12]


def to_lines(data):
    return "".join(t.replace("\n", NL) + "\n" for t in TOKEN.findall(data.decode("utf8"))).encode("utf8")


def from_lines(data):
    return "".join(line.replace(NL, "\n") for line in data.decode("utf8").split("\n")).encode("utf8")


def three_way(ours, old, theirs, label):
    with tempfile.TemporaryDirectory() as tmp:
        for name, data in (("ours", ours), ("base", old), ("theirs", theirs)):
            open(f"{tmp}/{name}", "wb").write(data)
        run = subprocess.run(["git", "merge-file", "-p", "--diff3", "-L", "integrated", "-L", "base", "-L", label, f"{tmp}/ours", f"{tmp}/base", f"{tmp}/theirs"], capture_output=True)
        return run.stdout, run.returncode


def resolve_blocks(fine, choices):
    queue = iter(choices)

    def pick(match):
        choice = next(queue)
        if choice == "ours":
            return match.group(1)
        if choice == "theirs":
            return match.group(3)
        return choice.replace("\n", NL) + "\n"

    joined = BLOCK.sub(pick, fine.decode("utf8"))
    if next(queue, None) is not None:
        raise SystemExit("more choices than conflict blocks")
    return from_lines(joined.encode("utf8"))


def main():
    args = sys.argv[1:]
    opt = lambda flag, default: args[args.index(flag) + 1] if flag in args else default
    order = opt("--order", ",".join(ORDER)).split(",")
    res, base, out = opt("--res", "/private/tmp/ig"), opt("--base", "/private/tmp/ig/base0"), opt("--out", "/private/tmp/ig/merged")
    resolutions = opt("--resolutions", os.path.join(HERE, "resolutions"))
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(out)
    prefix = opt("--prefix", "r-")
    applied = [name for name in opt("--applied", "").split(",") if name]
    orig = opt("--orig", "/private/tmp/ig/orig0")
    base_of = {s: json.load(open(f"{res}/{prefix}{s}/meta.json"))["base"] for s in order if os.path.exists(f"{res}/{prefix}{s}/meta.json")}
    changes = {s: json.load(open(f"{res}/{prefix}{s}/changes.json")) for s in order if os.path.exists(f"{res}/{prefix}{s}/changes.json")}
    touched = {}
    for s in order:
        for rel, c in changes.get(s, {}).items():
            touched.setdefault(rel, []).append((s, c))
    report, conflicts = [], []
    for rel in sorted(touched):
        rows = touched[rel]
        dest = os.path.join(out, rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        live_path = os.path.join(base, rel)
        live = open(live_path, "rb").read() if os.path.exists(live_path) else None
        first_state = "created" if live is None else "modified"
        current = live
        trail, stuck = [], False
        for s, c in rows:
            if c["state"] == "deleted":
                raise SystemExit(f"{rel}: deletion by {s} not supported")
            theirs = open(f"{res}/{prefix}{s}/files/{c['file']}", "rb").read()
            set_base = base_of.get(s, orig)
            if current is None:
                if c["state"] != "created":
                    raise SystemExit(f"{rel}: modified by {s} but absent from the live tree")
                current = theirs
                trail.append(f"{s}:create")
                continue
            if c["state"] == "created":
                if theirs != current:
                    raise SystemExit(f"{rel}: created by {s} but present with other content")
                trail.append(f"{s}:same")
                continue
            if theirs == current:
                trail.append(f"{s}:same")
                continue
            old = open(os.path.join(set_base, rel), "rb").read()
            if current == old:
                current = theirs
                trail.append(f"{s}:take")
                continue
            merged, bad = three_way(current, old, theirs, s)
            how = "merge"
            if bad:
                fine, fine_bad = three_way(to_lines(current), to_lines(old), to_lines(theirs), s)
                if fine_bad == 0:
                    merged, bad, how = from_lines(fine), 0, "merge(token)"
                else:
                    digest = hashlib.sha256(fine).hexdigest()
                    meta = os.path.join(resolutions, f"{key_of(rel)}-{s}.blocks.json")
                    if os.path.exists(meta) and json.load(open(meta))["tokenConflictSha256"] == digest:
                        merged, bad, how = resolve_blocks(fine, json.load(open(meta))["choices"]), 0, "resolved(hand)"
                    else:
                        os.makedirs(os.path.join(out, "_conflicts"), exist_ok=True)
                        open(os.path.join(out, "_conflicts", f"{key_of(rel)}-{s}.token"), "wb").write(fine)
                        conflicts.append({"rel": rel, "set": s, "key": f"{key_of(rel)}-{s}", "tokenConflictSha256": digest, "stale": os.path.exists(meta)})
                        stuck = True
            if stuck:
                trail.append(f"{s}:CONFLICT")
                break
            current = merged
            trail.append(f"{s}:{how}")
        if not stuck:
            open(dest, "wb").write(current)
        report.append({"rel": rel, "trail": trail, "sha256": hashlib.sha256(current).hexdigest(), "created": first_state == "created", "stuck": stuck})
    sys.path.insert(0, HERE)
    import semantic
    by_rel = {row["rel"]: row for row in report}
    for index, patch in enumerate(semantic.PATCHES):
        needs = patch.get("after") or []
        needs = [needs] if isinstance(needs, str) else needs
        if any(name not in order and name not in applied for name in needs):
            continue
        if applied and not any(name in order for name in needs):
            continue
        rel = semantic.expand(patch["rel"])
        if rel not in by_rel and os.path.exists(os.path.join(base, rel)):
            os.makedirs(os.path.dirname(os.path.join(out, rel)), exist_ok=True)
            shutil.copyfile(os.path.join(base, rel), os.path.join(out, rel))
            row = {"rel": rel, "trail": ["integrate:seed"], "sha256": "", "created": False, "stuck": False}
            report.append(row)
            by_rel[rel] = row
        if rel not in by_rel:
            raise SystemExit(f"semantic patch {index} ({patch['why'][:50]}): file not in the integrated set: {rel}")
        if by_rel[rel]["stuck"]:
            continue
        path = os.path.join(out, rel)
        text = open(path, encoding="utf8").read()
        if "fn" in patch:
            text = patch["fn"](text)
        else:
            found = text.count(patch["old"])
            if found != patch.get("count", 1):
                raise SystemExit(f"semantic patch {index} ({patch['why'][:60]}): anchor found {found} times (want {patch.get('count', 1)}): {patch['old'][:100]!r}")
            text = text.replace(patch["old"], patch["new"])
        open(path, "w", encoding="utf8").write(text)
        by_rel[rel]["trail"].append(f"patch#{index}")
        by_rel[rel]["sha256"] = hashlib.sha256(text.encode("utf8")).hexdigest()
    for rel, text in semantic.CREATES.items():
        rel = semantic.expand(rel)
        if rel in by_rel:
            raise SystemExit(f"hand-written file already produced by a set: {rel}")
        os.makedirs(os.path.dirname(os.path.join(out, rel)), exist_ok=True)
        open(os.path.join(out, rel), "w", encoding="utf8").write(text)
        row = {"rel": rel, "trail": ["integrate:create"], "sha256": hashlib.sha256(text.encode("utf8")).hexdigest(), "created": True, "stuck": False}
        report.append(row)
        by_rel[rel] = row
    for rel, why in semantic.EXCLUDES.items():
        rel = semantic.expand(rel)
        if rel in by_rel and not by_rel[rel]["stuck"]:
            os.remove(os.path.join(out, rel))
            report.remove(by_rel.pop(rel))
    json.dump({"order": order, "files": report, "conflicts": conflicts}, open(f"{out}/_report.json", "w"), ensure_ascii=False, indent=1)
    L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
    for row in report:
        if len(row["trail"]) > 1:
            print(row["rel"].replace(L, "L"), " ".join(row["trail"]))
    print(f"{len(report)} files, {len(conflicts)} unresolved conflicts")
    for c in conflicts:
        print("CONFLICT", c["rel"].replace(L, "L"), c["set"], c["key"], c["tokenConflictSha256"], "(stale resolution)" if c["stale"] else "")
    sys.exit(1 if conflicts else 0)


main()
