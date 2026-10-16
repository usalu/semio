"""Checks candidate folder emoji: used as a folder prefix in the repo (tracked dir names), RGI via bun, uniqueness."""
import collections, json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
import subprocess as _sp
D = f"{W}/dirnames.txt"
if not os.path.exists(D):
    _data = _sp.run(["git", "-c", "core.quotepath=false", "ls-files", "-z"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
    _dirs = set()
    for _p in _data:
        _dirs.update(_p.split("/")[:-1])
    write(D, "\n".join(sorted(_dirs)))
names = open(D, encoding="utf-8").read().split("\n")
pref = collections.Counter()
for n in names:
    i = 0
    while i < len(n) and ord(n[i]) > 127:
        i += 1
    if i:
        pref[n[:i]] += 1
cands = sys.argv[1:]
js = "const c=" + json.dumps(cands, ensure_ascii=False) + ";for(const e of c){const b=e.replace(/\\uFE0F/g,'');console.log(JSON.stringify([e,/^\\p{RGI_Emoji}$/v.test(e)||/^\\p{RGI_Emoji}$/v.test(b)]))}"
out = subprocess.run(["bun", "-e", js], capture_output=True, text=True).stdout.strip().split("\n")
for line in out:
    e, rgi = json.loads(line)
    key = e if e.endswith("️") else e + "️"
    print(e, "rgi=" + str(rgi), "folder-prefix-uses=" + str(pref.get(key, 0) + (pref.get(e, 0) if key != e else 0)))
