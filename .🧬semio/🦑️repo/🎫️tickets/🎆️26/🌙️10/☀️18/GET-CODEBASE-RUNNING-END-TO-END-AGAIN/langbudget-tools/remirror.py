#!/usr/bin/env python3
"""🔁️ Puts a mirror back to the live bytes of every file the langbudget and nolegacy batches touch, then applies them in landing order: loader, nobaseline, lang.
usage: python3 -I remirror.py MIRROR_ROOT [--live ROOT] [--upto base|loader|nobaseline|lang]"""
import json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
mirror = os.path.abspath(sys.argv[1])
live = os.path.abspath(sys.argv[sys.argv.index("--live") + 1]) if "--live" in sys.argv else os.path.abspath(os.path.join(C, "../../../../../../.."))
upto = sys.argv[sys.argv.index("--upto") + 1] if "--upto" in sys.argv else "lang"
order = [("loader", os.path.join(HERE, "staged", "loader")), ("nobaseline", os.path.join(C, "nolegacy-tools", "staged", "nobaseline")), ("lang", os.path.join(HERE, "staged", "lang"))]
touched = set()
for _name, path in order:
    touched |= set(json.load(open(os.path.join(path, "manifest.json"), encoding="utf8")))
for rel in sorted(touched):
    src, dst = os.path.join(live, rel), os.path.join(mirror, rel)
    if os.path.exists(src):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)
    elif os.path.exists(dst):
        os.remove(dst)
for name, path in order:
    if upto == "base":
        break
    subprocess.run([sys.executable, "-I", os.path.join(path, "stage.py"), "apply", "--root", mirror], check=True, stdout=subprocess.DEVNULL)
    print("applied", name)
    if name == upto:
        break
