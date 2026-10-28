import os, sys, hashlib, subprocess, tempfile, build
H = build.HERE; C = os.path.dirname(H)
others = {"ledger/📖️texts/🔣️.json": "xplat3-g-tools/staged/files", "🗃️unit-cache/🧬️schema/🔣️.json": "jobsrv-g-tools/staged/files", "🧾️ci-report/🗂️suites/📜️script.ts": "ci-g-tools/staged/files", "🧾️ci-report/🗂️suites/🟦️.ts": "ci-g-tools/staged/files"}
for p in build.PATHS:
    if "🔭️read-oracle" in p: others[p] = "oracle2-g-tools/staged/files"
for alias, root in others.items():
    p = alias if alias in build.PATHS else build.resolve(alias)
    key = hashlib.sha256(p.encode()).hexdigest()[:10]
    theirs = os.path.join(C, root, p)
    if not os.path.exists(theirs): continue
    mine, base = os.path.join(H, "new", key), os.path.join(H, "base", key)
    with tempfile.NamedTemporaryFile(delete=False) as f: f.write(open(theirs, "rb").read()); tmp = f.name
    r = subprocess.run(["git", "merge-file", "-p", "-L", "other-staged", "-L", "base", "-L", "conv-g", tmp, base, mine], capture_output=True)
    print("CLEAN   " if r.returncode == 0 else f"CONFLICT({r.returncode})", p[-60:])
    os.unlink(tmp)
