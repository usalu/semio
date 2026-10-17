"""🔎 c4p1 context printer: lines around given 1-based line numbers (read only). usage: ctx.py <file> <line>... [-w N]"""
import sys
a = sys.argv[1:]
w = 3
if "-w" in a:
    i = a.index("-w"); w = int(a[i + 1]); del a[i:i + 2]
t = open(a[0], encoding="utf-8").read().split("\n")
for ln in map(int, a[1:]):
    print(f"--- {ln}")
    for k in range(max(1, ln - w), min(len(t), ln + w) + 1):
        print(f"{k:5}{'>' if k == ln else ' '} {t[k-1][:150]}")
