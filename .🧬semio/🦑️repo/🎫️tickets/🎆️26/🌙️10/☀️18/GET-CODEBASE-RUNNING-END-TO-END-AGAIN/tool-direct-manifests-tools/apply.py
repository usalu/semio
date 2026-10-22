#!/usr/bin/env python3
"""🚚️ Anchored, all-or-nothing apply of the direct-tool-declarations increment (stdlib only, Python 3.9+, cross-platform). It applies AFTER round 6.

usage: apply.py [check|diff|apply] [--write] [--root REPO_ROOT] [--group a,b] [--allow-frozen] [--undo BACKUP_DIR]
  check (default)  per file: pending | applied | WAIT | DRIFT; exit 0 all applied, 1 pending, 2 drift or wait
  diff             unified diff of what apply would write
  apply            DRY RUN unless --write; refuses on any drift, any WAIT, a path over 256 UTF-16 units, or a row in a frozen tree without --allow-frozen
  --backup-dir DIR where backup-<stamp>/ is written (default: next to this script)
  --inject-failure-after N  self-test only: fail after N renames to prove the rollback
  --undo DIR       restores the files saved by an earlier --write (dry run unless --write)
A row is pending while its anchor still occurs the recorded number of times, applied while its result is there and the anchor is gone, WAIT while a text it requires (a set that has to land first) is missing, DRIFT otherwise.
Rows of one file apply in manifest order on the text the earlier rows left. Writes go through a temporary sibling and an atomic rename; modes are kept; the originals are saved under backup-<stamp>/ by content hash.
After apply: run `repo verify tool-declarations`, then `repo tools exec --light rustc -- -vV` (the new `tools exec <command>` form), then regenerate the help snapshot (the one regeneration) because the catalogue changed."""
import difflib
import hashlib
import importlib.util
import json
import os
import re
import stat
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
opt = lambda name, default=None: args[args.index(name) + 1] if name in args else default
if "--root" in args:
    ROOT = os.path.abspath(opt("--root"))
STAGED = os.path.abspath(opt("--staged", os.path.join(HERE, "staged")))
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
groups = set(opt("--group").split(",")) if opt("--group") else None
manifest = json.load(open(os.path.join(STAGED, "manifest.json"), encoding="utf-8"))
sha = lambda data: hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()


def read(path):
    full = os.path.join(ROOT, path)
    if os.path.islink(full):
        raise SystemExit("refusing symlink " + path)
    if not os.path.exists(full):
        return None
    return open(full, encoding="utf-8", newline="").read()


def utf16(path):
    return len(os.path.join(ROOT, path).encode("utf-16-le")) // 2


def evaluate():
    """Returns (rows, files): rows = [(row, state, detail)], files = {path: new text}; the text only follows pending rows."""
    states = []
    working = {}
    for row in manifest["rows"]:
        if groups is not None and row["group"] not in groups:
            continue
        path = row["path"]
        if path not in working:
            working[path] = read(path)
        body = working[path]
        if body is None:
            states.append((row, "DRIFT", "missing file"))
            continue
        if row["kind"] == "literal":
            found = body.count(row["old"])
            if row["new"] != "" and body.count(row["new"]) >= row["count"] and (found == 0 or row["old"] in row["new"]):
                states.append((row, "applied", ""))
            elif found == row["count"]:
                working[path] = body.replace(row["old"], row["new"])
                states.append((row, "pending", ""))
            elif found == 0 and row["new"] == "" and row.get("witness") and row["witness"] in body:
                states.append((row, "applied", ""))
            elif "requires" in row and row["requires"] not in body:
                states.append((row, "WAIT", row["requires"]))
            else:
                states.append((row, "DRIFT", "anchor occurs %d times (expected %d)" % (found, row["count"])))
        else:
            pattern = re.compile(row["pattern"], re.S)
            after, count = pattern.subn(row["repl"], body)
            if count == row["count"]:
                working[path] = after
                states.append((row, "pending", ""))
            elif count == 0 and row["proof"] in body:
                states.append((row, "applied", ""))
            else:
                states.append((row, "DRIFT", "matches %d (expected %d)" % (count, row["count"])))
    for sweep in manifest.get("sweeps", []):
        if groups is not None and sweep["group"] not in groups:
            continue
        pattern = re.compile(sweep["pattern"], re.S)
        regexes = [checker_module().glob_regex(g) for g in sweep["files"]]
        matched = 0
        for path in sweep_files():
            if not any(r.match(path) for r in regexes):
                continue
            if path not in working:
                try:
                    working[path] = read(path)
                except UnicodeDecodeError:
                    working[path] = None
            body = working[path]
            if body is None:
                continue
            after, count = pattern.subn(sweep["repl"], body)
            if count:
                working[path] = after
                matched += count
                states.append(({"kind": "sweep", "id": sweep["id"], "group": sweep["group"], "path": path}, "pending", ""))
        if matched == 0:
            states.append(({"kind": "sweep", "id": sweep["id"], "group": sweep["group"], "path": "(sweep %s)" % sweep["id"]}, "applied", ""))
    for create in manifest["creates"]:
        data = open(os.path.join(STAGED, "blobs", create["blob"]), "rb").read()
        if sha(data) != create["sha256"]:
            states.append(({"kind": "create", "group": "law", "path": create["path"]}, "DRIFT", "staged blob changed"))
            continue
        have = read(create["path"])
        row = {"kind": "create", "group": "law", "path": create["path"]}
        if have is None:
            working[create["path"]] = data.decode("utf-8")
            states.append((row, "pending", ""))
        elif have == data.decode("utf-8"):
            states.append((row, "applied", ""))
        else:
            states.append((row, "DRIFT", "exists with other content"))
    return states, working


_checker = []
_files = []


def checker_module():
    if not _checker:
        spec = importlib.util.spec_from_file_location("checker", os.path.join(HERE, "law", "checker.py"))
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        _checker.append(module)
        _checker.append(module.Law(module.load_patterns(os.path.join(HERE, "law", "patterns.json"))))
    return _checker[0]


def sweep_files():
    if not _files:
        checker_module()
        law = _checker[1]
        out = subprocess.run(["git", "-C", ROOT, "ls-files", "-co", "--exclude-standard", "-z"], capture_output=True, check=True).stdout.decode("utf-8")
        roots = manifest["roots"]
        _files.extend(sorted(p for p in out.split("\0") if p and any(p == r or p.startswith(r + "/") for r in roots) and law.rewritable(p)))
    return _files


def frozen(path):
    return any(path.startswith(prefix) for prefix in manifest["frozen"])


def main():
    if "--undo" in args:
        return undo(opt("--undo"))
    states, working = evaluate()
    drift = [s for s in states if s[1] == "DRIFT"]
    wait = [s for s in states if s[1] == "WAIT"]
    pending = [s for s in states if s[1] == "pending"]
    applied = [s for s in states if s[1] == "applied"]
    longest = max(utf16(s[0]["path"]) for s in states)
    per_file = {}
    for row, state, detail in states:
        per_file.setdefault(row["path"], []).append((row.get("id", "create"), state, detail))
    if mode in ("check", "apply") or not pending:
        for path, items in per_file.items():
            bad = [i for i in items if i[1] not in ("applied",)]
            if bad:
                print("%-8s %s  %s" % (",".join(sorted(set(i[1] for i in bad))), path[-110:], "; ".join("%s %s" % (i[0], i[2]) for i in bad if i[2])[:160]))
    by_group = {}
    for row, state, _ in states:
        by_group.setdefault(row["group"], {}).setdefault(state, 0)
        by_group[row["group"]][state] += 1
    print("rows %d: pending %d, applied %d, WAIT %d, DRIFT %d; files %d; longest path %d UTF-16 units" % (len(states), len(pending), len(applied), len(wait), len(drift), len(per_file), longest))
    print("by group: " + json.dumps(by_group, ensure_ascii=False))
    if longest > 256:
        print("REFUSED: a path exceeds 256 characters")
        return 2
    if drift:
        print("DRIFT: rebuild with `python3 make.py --root <tree>` after the peers' edits to the anchored lines are reconciled")
        return 2
    if wait:
        print("WAIT: apply the set(s) named above first (the rows that need them stay unapplied)")
        return 2
    if mode == "diff":
        for path in sorted(per_file):
            before = read(path) or ""
            after = working.get(path, before)
            if before != after:
                sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), "a/" + path, "b/" + path, n=1))
        return 0
    if mode == "check":
        return 1 if pending else 0
    touched = sorted(set(s[0]["path"] for s in pending))
    if not touched:
        print("nothing to write: every row is applied")
        return 0
    guarded = [p for p in touched if frozen(p)]
    if guarded and "--allow-frozen" not in args:
        print("REFUSED: %d file(s) in the frozen library/process trees (an edit there restarts a running chain); pass --allow-frozen once chain17 is green, e.g. %s" % (len(guarded), guarded[0][-80:]))
        return 2
    if "--write" not in args:
        print("DRY RUN: %d file(s) would be written (use --write)" % len(touched))
        return 1 if touched else 0
    backup = os.path.join(os.path.abspath(opt("--backup-dir", HERE)), "backup-" + time.strftime("%Y%m%d-%H%M%S") + "-%03d" % (int(time.time() * 1000) % 1000))
    os.makedirs(backup)
    index = []
    written = []
    temporaries = []
    try:
        for path in touched:
            full = os.path.join(ROOT, path)
            before = read(path)
            if before is not None:
                blob = sha(before)
                with open(os.path.join(backup, blob), "w", encoding="utf-8", newline="") as handle:
                    handle.write(before)
                index.append({"path": path, "sha256": blob, "mode": stat.S_IMODE(os.stat(full).st_mode)})
            else:
                index.append({"path": path, "sha256": None, "mode": 0o644})
        json.dump(index, open(os.path.join(backup, "index.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        for path in touched:
            full = os.path.join(ROOT, path)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            temporary = full + ".tdm.tmp"
            temporaries.append(temporary)
            with open(temporary, "w", encoding="utf-8", newline="") as handle:
                handle.write(working[path])
            os.chmod(temporary, stat.S_IMODE(os.stat(full).st_mode) if os.path.exists(full) else 0o644)
        for path in touched:
            full = os.path.join(ROOT, path)
            os.replace(full + ".tdm.tmp", full)
            written.append(path)
            if opt("--inject-failure-after") == str(len(written)):
                raise RuntimeError("injected failure after %d rename(s) (self-test)" % len(written))
    except BaseException:
        for temporary in temporaries:
            if os.path.exists(temporary):
                os.remove(temporary)
        restore(backup, written)
        raise
    print("written %d file(s); originals in %s" % (len(touched), backup))
    return 0


def restore(backup, only=None):
    index = json.load(open(os.path.join(backup, "index.json"), encoding="utf-8"))
    for row in index:
        if only is not None and row["path"] not in only:
            continue
        full = os.path.join(ROOT, row["path"])
        if row["sha256"] is None:
            if os.path.exists(full):
                os.remove(full)
            continue
        data = open(os.path.join(backup, row["sha256"]), encoding="utf-8", newline="").read()
        temporary = full + ".tdm.tmp"
        with open(temporary, "w", encoding="utf-8", newline="") as handle:
            handle.write(data)
        os.chmod(temporary, row["mode"])
        os.replace(temporary, full)


def undo(backup):
    index = json.load(open(os.path.join(backup, "index.json"), encoding="utf-8"))
    print("undo: %d file(s) from %s" % (len(index), backup))
    if "--write" not in args:
        print("DRY RUN: nothing restored (use --write)")
        return 1
    restore(backup)
    return 0


if __name__ == "__main__":
    sys.exit(main())
