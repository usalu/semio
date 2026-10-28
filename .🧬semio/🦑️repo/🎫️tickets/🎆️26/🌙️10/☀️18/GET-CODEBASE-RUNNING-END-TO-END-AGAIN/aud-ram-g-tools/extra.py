"""📈 Extra census statistics: idle shares, concurrency, stale idle holders, peak-sample breakdown."""
import json, statistics, sys
from collections import defaultdict, Counter
GB = 1048576.0
recs = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8")]
tot_r = sum(x["rss_kb"] for x in recs); tot_i = sum(x["idle_rss_kb"] for x in recs)
print(f"pooled idle share of RSS over all samples: {tot_i/tot_r*100:.1f}%")
print("concurrent runs per sample: median", statistics.median(len(x["runs"]) for x in recs), "min", min(len(x["runs"]) for x in recs), "max", max(len(x["runs"]) for x in recs))
print("repo procs per sample: median", statistics.median(x["n"] for x in recs), "max", max(x["n"] for x in recs))
print("repo RSS GB per sample: median %.2f max %.2f; samples above 16 GB: %d" % (statistics.median(x["rss_kb"]/GB for x in recs), max(x["rss_kb"]/GB for x in recs), sum(1 for x in recs if x["rss_kb"]/GB > 16)))
print("samples with repo RSS > 12 GB:", sum(1 for x in recs if x["rss_kb"]/GB > 12))
# per role: pooled idle share and per-process median/peak RSS
per = defaultdict(list); idle = defaultdict(lambda: [0,0])
for x in recs:
    for p in x["procs"]:
        pid, ppid, role, lane, root, rss, cpu, age, cmd = p
        per[role].append(rss/GB)
        idle[role][0] += rss
        if cpu < 1.0: idle[role][1] += rss
print("\nrole | proc-samples | pooled idle share | median proc RSS GB | peak proc RSS GB")
for role in sorted(per, key=lambda r: -sum(per[r])):
    v = per[role]
    print(f"{role} | {len(v)} | {idle[role][1]/max(idle[role][0],1)*100:.0f}% | {statistics.median(v):.2f} | {max(v):.2f}")
# stale idle holders: idle, age > 30 min, RSS > 100 MB
stale = Counter(); stale_rss = defaultdict(float)
for x in recs:
    for p in x["procs"]:
        pid, ppid, role, lane, root, rss, cpu, age, cmd = p
        if cpu < 1.0 and age > 1800 and rss > 100*1024:
            stale[role] += 1; stale_rss[role] += rss/GB
print("\nstale idle (>30 min old, >100 MB, cpu<1%) proc-samples per role:", dict(stale.most_common()))
print("  GB-sum per role (over samples):", {k: round(v,1) for k,v in stale_rss.items()})
# peak sample breakdown
pk = max(recs, key=lambda x: x["rss_kb"])
print(f"\npeak sample tick {pk['idx']} {pk['ts']}: repo RSS {pk['rss_kb']/GB:.2f} GB, procs {pk['n']}")
for role, s in sorted(pk["roles"].items(), key=lambda kv: -kv[1]["rss_kb"]):
    print(f"  {role}: n={s['n']} rss={s['rss_kb']/GB:.2f} GB idle={s['idle_rss_kb']/GB:.2f} GB")
print("  top processes in peak sample:")
for p in pk["procs"][:12]:
    print("   ", p[0], p[2], p[3], p[5]//1024, "MB cpu", p[6], "age", p[7], p[8][:90])
# eval loader details over all samples
loaders = [p for x in recs for p in x["procs"] if p[2] == "loader"]
print("\nloader process-samples:", len(loaders), "RSS>1GB:", sum(1 for p in loaders if p[5] > 1048576), "RSS>3GB:", sum(1 for p in loaders if p[5] > 3*1048576))
ages = [p[7] for p in loaders if p[5] > 1048576]
print("loader >1GB age median s:", statistics.median(ages) if ages else None)
