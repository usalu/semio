#!/usr/bin/env python3
"""Builds the per-module pass/fail table of the build-library sweep from the sweep result files (later files override earlier ones)."""
import collections,hashlib,json,os,re,sys
G=sys.argv[1];B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
order=["sweep-full.txt"]
last={}
for name in order:
    p=os.path.join(G,name)
    if not os.path.exists(p):continue
    for line in open(p,encoding="utf8"):
        part=line.rstrip("\n").split(" ",3)
        if len(part)==4:last[part[3]]=(part[0],part[1],part[2])
HELPER=re.compile("🧲️holder|📥️worker|🪆️kernel-role|♻️plan-reuse/🧪️tests/.*fixtures|📊️bench|🌊️burst|🎭️stub|📜️child")
def kind(f,v):
    lp=os.path.join(G,"sweep-logs",hashlib.md5(f.encode()).hexdigest()[:12]+".log")
    t=open(lp,encoding="utf8",errors="replace").read() if os.path.exists(lp) else ""
    if HELPER.search(f):return "helper"
    if v[1]=="0" and v[2]=="0":return "pass"
    if "Original" in t and ("required" in t or "refused" in t):return "original-invocation"
    if "live repository store of the checkout" in t:return "live-store-guard"
    if "timed out" in t or v[2]=="124":return "timeout"
    return "fail"
rows=collections.defaultdict(lambda:collections.Counter());bad=collections.defaultdict(list)
for f,v in sorted(last.items()):
    m=f.replace(B,"").split("/")[0];k=kind(f,v)
    if k=="helper":continue
    rows[m][k]+=1
    if k!="pass":bad[m].append((k,f.replace(B,"")))
print("| module | files | pass | fail | original-invocation only | timeout | live-store |\n|---|---|---|---|---|---|---|")
tot=collections.Counter()
for m in sorted(rows):
    c=rows[m];n=sum(c.values());tot.update(c)
    print(f"| {m} | {n} | {c['pass']} | {c['fail']} | {c['original-invocation']} | {c['timeout']} | {c['live-store-guard']} |")
n=sum(tot.values())
print(f"| **total** | {n} | {tot['pass']} | {tot['fail']} | {tot['original-invocation']} | {tot['timeout']} | {tot['live-store-guard']} |")
print("\nNon-passing files:\n")
for m in sorted(bad):
    for k,f in bad[m]:print(f"- {k}: `{f}`")
