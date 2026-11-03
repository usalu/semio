"""🛡️ Runs every guarded script raw in the sandbox, with and without --apply: it must exit 2 and change nothing.

usage: python3 verify_guards.py   (prints one line per script; exit 1 on a violation)"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sandbox as S

VENV = "/home/deck/semio/.venv/bin/python"
bad = 0
for rel in open(os.path.dirname(os.path.abspath(__file__)) + "/guarded.txt").read().split():
    for args in ([], ["--apply"]):
        mark = S.marker()
        r = S.run((["bash", rel] if rel.endswith(".sh") else [VENV, rel]) + args, 120)
        d = S.delta(mark)
        ok = r.returncode == 2 and not d
        bad += not ok
        print("%s %s %s rc=%s changed=%d" % ("ok " if ok else "BAD", rel, args, r.returncode, len(d)), flush=True)
        S.restore(d)
sys.exit(1 if bad else 0)
