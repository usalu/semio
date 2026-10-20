#!/usr/bin/env python3
"""Merged copy of fnsplit4a-finish: one of its 64 modify ops (`🔍️discovery/🔀️rust-non-repo-join-bases/🟦️.ts`) is also rewritten by agents-fix. The copy 3-way merges the two results (base = the live file now,
ours = agents-fix' staged result, theirs = fnsplit4a-finish' blob) with `git merge-file` and re-bases that op on the agents-fix result, so the set applies after agents-fix. Refuses on a textual conflict."""
import hashlib, json, os, shutil, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
LIVE = C
while LIVE != os.path.dirname(LIVE) and not (os.path.isfile(os.path.join(LIVE, "bunfig.toml")) and os.path.isfile(os.path.join(LIVE, "bun.lock"))):
    LIVE = os.path.dirname(LIVE)
SOURCE, TARGET = C + "/fnsplit4a-finish-tools/staged", HERE + "/fnsplit4a-finish/staged"
shutil.rmtree(HERE + "/fnsplit4a-finish", ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__", "backup"))
sha = lambda b: hashlib.sha256(b).hexdigest()
manifest = json.load(open(TARGET + "/manifest.json", encoding="utf8"))
agents = json.load(open(HERE + "/agents-fix/staged/manifest.json", encoding="utf8"))
after = {row["path"]: HERE + "/agents-fix/staged/" + row["after"] for row in agents["files"]}
merged = 0
for op in manifest["ops"]:
    if op["op"] != "modify" or op["path"] not in after:
        continue
    base = open(os.path.join(LIVE, op["path"]), "rb").read()
    assert sha(base) == op["baseSha256"], "live file is not the base of the fnsplit op: " + op["path"]
    ours = open(after[op["path"]], "rb").read()
    theirs = open(TARGET + "/blobs/" + op["sha256"][:24], "rb").read()
    with tempfile.TemporaryDirectory() as tmp:
        for name, data in (("ours", ours), ("base", base), ("theirs", theirs)):
            open(os.path.join(tmp, name), "wb").write(data)
        run = subprocess.run(["git", "merge-file", "-p", os.path.join(tmp, "ours"), os.path.join(tmp, "base"), os.path.join(tmp, "theirs")], capture_output=True)
    if run.returncode != 0:
        sys.exit("CONFLICT merging %s (%d conflicts)" % (op["path"], run.returncode))
    out = run.stdout
    open(TARGET + "/blobs/" + sha(out)[:24], "wb").write(out)
    op.update(baseSha256=sha(ours), sha256=sha(out), bytes=len(out))
    merged += 1
json.dump(manifest, open(TARGET + "/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("merged fnsplit4a-finish: %d op(s) re-based on the agents-fix result" % merged)
