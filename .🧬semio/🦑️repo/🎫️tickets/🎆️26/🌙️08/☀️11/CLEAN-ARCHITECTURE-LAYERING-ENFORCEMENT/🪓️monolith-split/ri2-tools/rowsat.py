#!/usr/bin/env python3
# 📍️ Signal rows of an oracle tag/config minus a baseline, with the file each row sits in: rowsat.py <tag> <cfg> <baseline>
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import uo
from common import *
tag, cfg, base = sys.argv[1:4]
def rows(t):
    sb = f"{RS}/sb-{t}-crates/A"
    err = open(f"{GEN}/ri2-oracle-{t}-A-{cfg}.jsonl").read()
    return uo.parse(err, sb)
a, b = rows(tag), rows(base)
bk = collections.Counter()
for k, v in b.items():
    if not uo.noise(k): bk[k[:4]] += v
new = collections.defaultdict(list)
cnt = collections.Counter()
for k, v in a.items():
    if uo.noise(k): continue
    cnt[k[:4]] += v
    new[k[:4]].append((uo.short(k[4]), v))
for k, v in sorted(cnt.items(), key=lambda kv: (kv[0][1], kv[0][2])):
    if v - bk.get(k, 0) <= 0: continue
    if "action_args_json" in k[1]: continue
    fs = collections.Counter()
    for f, n in new[k]: fs[f] += n
    print(v - bk.get(k, 0), k[0], k[1][:110], "|", [(f.replace("el/🐚️Shell/T/📥️staging/", "S/"), n) for f, n in fs.most_common(3)])
