#!/usr/bin/env python3
"""Writes the deterministic input of the golden vectors (ids, small tiers, samples, queries) of the metrics member. usage: golden-input.py <fixtures dir>"""
import json,os,sys
T0=1800000000000
seed=20261126
def nxt():
    global seed
    seed=(seed*1103515245+12345)%2147483648
    return seed/2147483648
ids=["host.cpu.busy","host.memory.available","build.units.compiled","ready.stage"]
tiers=[{"id":"t0","bucketMs":1000,"buckets":8},{"id":"t1","bucketMs":4000,"buckets":6},{"id":"t2","bucketMs":16000,"buckets":5}]
samples=[]
for tick in range(120):
    if 40<=tick<48 or 90<=tick<94: continue
    for repeat in range(2 if tick%17==0 else 1):
        values={}
        if nxt()<0.95: values[ids[0]]=int(nxt()*400)/4
        if nxt()<0.9: values[ids[1]]=int(nxt()*64)*1048576+0.5
        if nxt()<0.6: values[ids[2]]=int(nxt()*9)
        if tick%10==0: values[ids[3]]=tick//10%4
        samples.append({"at":T0+tick*1000+repeat*400+int(nxt()*300),"values":values})
queries=[["all",T0-5000,T0+130000],["recent",T0+100000,T0+125000],["misaligned",T0+33333,T0+77777],["tiny",T0+119000,T0+120000],["empty",T0+500000,T0+600000],["old",T0,T0+30000]]
doc={"createdMs":T0,"ids":ids,"tiers":tiers,"samples":samples}
os.makedirs(sys.argv[1],exist_ok=True)
open(os.path.join(sys.argv[1],"🔣️.json"),"w",encoding="utf8").write(json.dumps(doc,separators=(",",":"))+"\n")
open(os.path.join(sys.argv[1],"queries.tsv"),"w",encoding="utf8").write("".join("%s\t%d\t%d\n"%tuple(row) for row in queries))
print(len(samples))
