#!/usr/bin/env python3
"""🪠️ Rebases staged batches of lane lintglob onto the CURRENT live bytes: `python3 -I rebuild.py <batch>...` (default: all three).
For every file whose live bytes moved since `base/`: a three-way merge (`git merge-file`, no repository state touched) of my edit (`files/`) over the new live bytes; a conflict stops with exit 1 and leaves the batch unchanged. Then base := live, files := merged, and the manifest is sealed again. Print the moved files; re-run each batch's verify.sh on a fresh mirror afterwards."""
import json, os, shutil, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
def main():
    batches = sys.argv[1:] or ["globproj", "bazelglob", "buildscript"]
    for batch in batches:
        folder = os.path.join(HERE, "staged", batch)
        rows = json.load(open(os.path.join(folder, "rels.json"), encoding="utf8"))
        moved = []
        for rel, name in rows.items():
            live, base, mine = os.path.join(ROOT, rel), os.path.join(folder, "base", name), os.path.join(folder, "files", name)
            if open(live, "rb").read() == open(base, "rb").read(): continue
            with tempfile.TemporaryDirectory() as scratch:
                merged = os.path.join(scratch, "m"); shutil.copyfile(mine, merged)
                code = subprocess.run(["git", "merge-file", merged, base, live]).returncode
                if code != 0: print("CONFLICT", batch, name, rel[-80:]); sys.exit(1)
                moved.append((name, rel, open(merged, "rb").read()))
        for name, rel, data in moved:
            shutil.copyfile(os.path.join(ROOT, rel), os.path.join(folder, "base", name))
            open(os.path.join(folder, "files", name), "wb").write(data)
            print("rebased", batch, name, rel[-70:])
        subprocess.run([sys.executable, "-I", os.path.join(HERE, "mkbatch.py"), "seal", batch], stdout=subprocess.DEVNULL, check=True)
main()
