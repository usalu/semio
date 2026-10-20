#!/usr/bin/env python3
"""Records the pre-edit state of overlay files so edits can be staged against the live tree. usage: track.py REL [REL...]"""
import hashlib, json, os, shutil, sys
O = "/private/tmp/testfix-law"
J = O + "/.orig/index.json"
def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()
def main():
    os.makedirs(O + "/.orig", exist_ok=True)
    idx = json.load(open(J)) if os.path.exists(J) else {}
    for rel in sys.argv[1:]:
        if rel in idx: continue
        p = O + "/" + rel
        if os.path.exists(p):
            n = str(len(idx))
            shutil.copyfile(p, O + "/.orig/" + n)
            idx[rel] = {"orig": n, "sha": sha(p)}
        else:
            idx[rel] = {"orig": None, "sha": None}
    json.dump(idx, open(J, "w"), ensure_ascii=False, indent=1)
main()
