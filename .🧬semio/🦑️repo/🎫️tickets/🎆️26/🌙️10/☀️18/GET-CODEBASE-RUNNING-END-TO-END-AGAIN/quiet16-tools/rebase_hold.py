"""🔂️ quiet16: puts the base bytes (live + the batches `hold` is stacked on, see make.py) of every file `edit_hold.py` edits into a mirror root and re-runs `edit_hold.py`, so the batch's sources are rebuilt from the current bases after an anchor text changed or a stacked batch moved.
usage: python3 -I rebase_hold.py <mirror root>"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); root = os.path.abspath(sys.argv[1])
make = {"__name__": "make", "__file__": os.path.join(HERE, "make.py")}; exec(compile(open(os.path.join(HERE, "make.py"), encoding="utf8").read().replace("\nmain()\n", "\n"), "make.py", "exec"), make)
edit = {"__name__": "edit", "__file__": os.path.join(HERE, "edit_hold.py")}; exec(compile(open(os.path.join(HERE, "edit_hold.py"), encoding="utf8").read().replace("\nmain()\n", "\n"), "edit_hold.py", "exec"), edit)
for rel in [*edit["EDITS"], *json.load(open(os.path.join(HERE, "edit_hold_tests.json"), encoding="utf8"))]:
    make["write"](os.path.join(root, rel), make["base_of"](rel, "hold"))
sys.exit(subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit_hold.py"), root]).returncode)
