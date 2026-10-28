# coding: utf-8
"""🧫️ Runs `helpgen.ts` inside a MIRROR that holds the batch (without a help fixture) and writes the regenerated help snapshot: `python3 -I helpgen_run.py MIRROR_ROOT OUT.json`."""
import os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
root, out = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
if "/sm/" not in root + "/":
    sys.exit("refusing: not a mirror")
VS = "️"
ops = os.path.join(root, "🧰" + VS + "framework/🛍" + VS + "products/🦑" + VS + "repo/🔨" + VS + "modules/📚" + VS + "library/🏗" + VS + "build/⚙" + VS + "operations")
source = open(os.path.join(HERE, "helpgen.ts"), encoding="utf8").read().replace("@@HELP@@", "./📖" + VS + "help/🟦" + VS + ".ts").replace("@@CATALOG@@", "./🔣" + VS + "catalog.json")
target = os.path.join(ops, "zz-helpgen.ts")
open(target, "w", encoding="utf8").write(source)
try:
    done = subprocess.run(["bun", target, out], cwd=root, capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit(done.stderr[-800:])
finally:
    os.remove(target)
print("help snapshot", out)
