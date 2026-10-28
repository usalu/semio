"""📼 Replays the estimates this machine recorded today (`~/Library/Caches/semio/host-capacity/estimates.json`) through the single-class estimator (what the next request really asked) and through the same samples with the samples a graph build most likely inflated left out (a sample above 9000 bp, i.e. more than 90 % of the default, in a class whose median sits below 60 %): what the class would ask once the graph phase no longer reaches it. A model of the cleaned store, not a measurement; the phased estimator itself is exercised by the simulation."""
import json,os,statistics,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import phase_sim as sim
store=json.load(open(os.path.expanduser("~/Library/Caches/semio/host-capacity/estimates.json")))
now=max(s["atMs"] for c in store["classes"].values() for s in c["samples"])
rows=[]
for key,entry in sorted(store["classes"].items()):
    ratios=sorted(s["ratioBasisPoints"] for s in entry["samples"])
    median=statistics.median(ratios)
    clean={"version":1,"classes":{key:{"samples":[s for s in entry["samples"] if not (s["ratioBasisPoints"]>9000 and median<6000)]}}}
    if not clean["classes"][key]["samples"]:clean["classes"][key]["samples"]=entry["samples"]
    old=sim.base_oracle.estimate(sim.POLICY,{"version":1,"classes":{key:entry}},[key],now)
    new=sim.base_oracle.estimate(sim.POLICY,clean,[key],now)
    dropped=len(entry["samples"])-len(clean["classes"][key]["samples"])
    rows.append((key,len(entry["samples"]),dropped,old["ratioBasisPoints"],new["ratioBasisPoints"]))
print("class | samples | probably graph-inflated | single-class ratio (bp) | without them (bp) | change")
for key,n,d,o,c in rows:
    if d>0 or key.count(":")<=1:print(f"{key} | {n} | {d} | {o} | {c} | {round(100*(c-o)/o)}%")
