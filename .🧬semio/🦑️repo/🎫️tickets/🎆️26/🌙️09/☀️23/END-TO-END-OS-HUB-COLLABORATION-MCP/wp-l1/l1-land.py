#!/usr/bin/env python3
"""🚆️ L1 window-3 landing wrapper: byte-exact backup + change record + guarded revert around ANY set's write command.

write <set> [--scope <prefix>]… -- <cmd…>  (scopes: only changes under these prefixes are recorded; others are listed)
                       record HEAD + copy every dirty (modified/untracked, non-ignored) source file to
                       `w3-backup/<set>/dirty/`, stat every non-ignored file, run <cmd…> in the repo root, stat again; every
                       file whose (mtime, size) changed, appeared or vanished is recorded in `manifest.json` with its after
                       image under `after/` (pre image = the dirty copy, else `git cat-file` of the recorded HEAD).
                       Files whose bytes did not change are dropped. Exit code = the command's.
revert <set> [--force] restores the pre image of every recorded file whose live bytes still equal the after image
                       (created → removed); a file edited since is KEPT and listed (``--force`` restores it anyway).
files <set>            recorded files (kind, path).    forget <set> <rel…>   drop a peer's concurrent edit from the record.    crates <set>   Cargo packages owning the recorded `.rs` files.
Ignored paths (`.🧬semio/`, `.tmp-ticket`) are never scanned; gitignored files are invisible to the record."""
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(os.environ.get("L1_ROOT", "/Users/ueli/Documents/semio"))
STATE = Path(os.environ.get("L1_STATE", "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-backup"))
SKIP = (".🧬semio/", ".tmp-ticket")


def git(*args, data=None):
    return subprocess.run(["git", *args], cwd=ROOT, input=data, capture_output=True, check=True).stdout


def listing():
    names = git("ls-files", "-co", "--exclude-standard", "-z").decode("utf-8").split("\0")
    return [name for name in names if name and not name.startswith(SKIP)]


def scan():
    seen = {}
    for rel in listing():
        try:
            stat = os.lstat(ROOT / rel)
        except FileNotFoundError:
            continue
        seen[rel] = (stat.st_mtime_ns, stat.st_size)
    return seen


def dirty():
    out = git("status", "--porcelain", "-uall", "-z").decode("utf-8").split("\0")
    paths, skip_next = [], False
    for entry in out:
        if skip_next:
            skip_next = False
            continue
        if not entry:
            continue
        if entry[0] in "RC":
            skip_next = True
        rel = entry[3:]
        if not rel.startswith(SKIP):
            paths.append(rel)
    return paths


def head_blobs(head, rels):
    if not rels:
        return {}
    query = "".join(f"{head}:{rel}\n" for rel in rels).encode("utf-8")
    raw = git("cat-file", "--batch", data=query)
    blobs, cursor = {}, 0
    for rel in rels:
        line_end = raw.index(b"\n", cursor)
        header = raw[cursor:line_end].decode("utf-8")
        cursor = line_end + 1
        if header.endswith("missing"):
            blobs[rel] = None
            continue
        size = int(header.rsplit(" ", 1)[1])
        blobs[rel] = raw[cursor:cursor + size]
        cursor += size + 1
    return blobs


def read(path):
    try:
        return path.read_bytes() if path.is_file() or path.is_symlink() else None
    except FileNotFoundError:
        return None


def write(name, command, scopes):
    home = STATE / name
    if (home / "manifest.json").exists():
        raise SystemExit(f"[l1-land] {name}: already recorded ({home}); revert or move it aside first")
    home.mkdir(parents=True, exist_ok=True)
    while True:
        head = git("rev-parse", "HEAD").decode().strip()
        dirty_paths = dirty()
        if git("rev-parse", "HEAD").decode().strip() == head:
            break
    for rel in dirty_paths:
        source = ROOT / rel
        if source.is_file():
            target = home / "dirty" / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)
    before = scan()
    started = time.strftime("%H:%M:%S")
    print(f"[l1-land] {name}: HEAD {head[:11]}, {len(dirty_paths)} dirty backed up, {len(before)} files scanned; run {' '.join(command)}", flush=True)
    rc = subprocess.run(command, cwd=ROOT).returncode
    after = scan()
    changed = sorted(rel for rel in set(before) | set(after) if before.get(rel) != after.get(rel))
    if scopes:
        outside = [rel for rel in changed if not rel.startswith(tuple(scopes))]
        for rel in outside:
            print(f"[l1-land] {name}: changed OUTSIDE scope (not recorded): {rel}", flush=True)
        changed = [rel for rel in changed if rel.startswith(tuple(scopes))]
    dirty_set = set(dirty_paths)
    blobs = head_blobs(head, [rel for rel in changed if rel not in dirty_set])
    rows = []
    for rel in changed:
        pre = read(home / "dirty" / rel) if rel in dirty_set else blobs.get(rel)
        now = read(ROOT / rel)
        if pre == now:
            continue
        kind = "created" if pre is None else "deleted" if now is None else "modified"
        if now is not None:
            target = home / "after" / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(now)
        if pre is not None and rel not in dirty_set:
            target = home / "dirty" / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(pre)
        rows.append({"rel": rel, "kind": kind})
    (home / "manifest.json").write_text(json.dumps({"set": name, "head": head, "started": started, "ended": time.strftime("%H:%M:%S"), "rc": rc, "command": command, "files": rows}, ensure_ascii=False, indent=1), encoding="utf-8")
    counts = {kind: sum(row["kind"] == kind for row in rows) for kind in ("modified", "created", "deleted")}
    print(f"[l1-land] {name}: rc={rc} recorded {len(rows)} file(s) {counts}", flush=True)
    return rc


def manifest(name):
    return json.loads((STATE / name / "manifest.json").read_text(encoding="utf-8"))


def revert(name, force):
    home = STATE / name
    record = manifest(name)
    restored, kept = 0, []
    for row in record["files"]:
        rel, live = row["rel"], ROOT / row["rel"]
        expected = read(home / "after" / rel) if row["kind"] != "deleted" else None
        if read(live) != expected and not force:
            kept.append(rel)
            continue
        if row["kind"] == "created":
            if live.exists() or live.is_symlink():
                live.unlink()
            parent = live.parent
            while parent != ROOT and parent.exists() and not any(parent.iterdir()):
                parent.rmdir()
                parent = parent.parent
        else:
            live.parent.mkdir(parents=True, exist_ok=True)
            live.write_bytes((home / "dirty" / rel).read_bytes())
        restored += 1
    stamp = time.strftime("%H%M%S")
    (home / "manifest.json").rename(home / f"manifest.reverted-{stamp}.json")
    print(f"[l1-land] {name}: restored {restored}, kept {len(kept)} (edited since)")
    for rel in kept:
        print("  kept", rel)
    return 0 if not kept else 3


def crates(name):
    owners = {}
    for row in manifest(name)["files"]:
        rel = row["rel"]
        directory = (ROOT / rel).parent
        owner = None
        while directory != ROOT:
            for sub, pattern in (("📦️packages/🦀️rust/Cargo.toml", r'^name = "([^"]+)"'), ("📦️packages/🟦️typescript/package.json", r'"name":\s*"([^"]+)"')):
                path = directory / sub
                if path.exists():
                    found = re.search(pattern, path.read_text(encoding="utf-8"), re.M)
                    owner = (("rust" if sub.endswith("toml") else "ts") + ":" + (found.group(1) if found else "?"))
                    break
            if owner:
                break
            directory = directory.parent
        owners.setdefault(owner or "?", []).append(rel)
    for owner, files in sorted(owners.items()):
        print(f"{owner}\t{len(files)}")


if __name__ == "__main__":
    verb, name = sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None
    if verb == "write":
        head = sys.argv[:sys.argv.index("--")]
        scopes = [head[index + 1] for index, arg in enumerate(head) if arg == "--scope"]
        sys.exit(write(name, sys.argv[sys.argv.index("--") + 1:], scopes))
    if verb == "revert":
        sys.exit(revert(name, "--force" in sys.argv))
    if verb == "files":
        for row in manifest(name)["files"]:
            print(row["kind"], row["rel"])
    elif verb == "crates":
        crates(name)
    elif verb == "forget":
        record = manifest(name)
        drop = set(sys.argv[3:])
        record["files"] = [row for row in record["files"] if row["rel"] not in drop]
        record.setdefault("forgotten", []).extend(sorted(drop))
        (STATE / name / "manifest.json").write_text(json.dumps(record, ensure_ascii=False, indent=1), encoding="utf-8")
        print(f"[l1-land] {name}: forgot {len(drop)} file(s) (a peer's concurrent edit, not the set's)")
