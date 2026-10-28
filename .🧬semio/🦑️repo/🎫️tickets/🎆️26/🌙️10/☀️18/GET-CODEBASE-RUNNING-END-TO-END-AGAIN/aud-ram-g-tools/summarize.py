"""📊 Summarise the RAM census samples per role, per run and per VM counter (stdlib only)."""
import json, statistics, sys
from collections import defaultdict

recs = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8")]
N = len(recs)
GB = 1048576.0
print(f"# samples: {N}  ({recs[0]['ts']} .. {recs[-1]['ts']})\n")

print("## per role (over samples)")
print("| role | samples seen | avg count | peak count | avg RSS GB | peak RSS GB | idle share of RSS (avg) |")
print("|---|---|---|---|---|---|---|")
roles = sorted({r for x in recs for r in x["roles"]})
for role in roles:
    cnt, rss, idle_share, seen = [], [], [], 0
    for x in recs:
        s = x["roles"].get(role)
        c = s["n"] if s else 0
        cnt.append(c)
        rss.append(s["rss_kb"] / GB if s else 0.0)
        if s:
            seen += 1
            idle_share.append(s["idle_rss_kb"] / s["rss_kb"] if s["rss_kb"] else 0.0)
    ish = f"{statistics.mean(idle_share)*100:.0f}%" if idle_share else "-"
    print(f"| {role} | {seen}/{N} | {statistics.mean(cnt):.1f} | {max(cnt)} | {statistics.mean(rss):.2f} | {max(rss):.2f} | {ish} |")

tot_rss = [x["rss_kb"] / GB for x in recs]
tot_idle = [x["idle_rss_kb"] / x["rss_kb"] if x["rss_kb"] else 0 for x in recs]
print(f"\nAll repo processes: avg RSS {statistics.mean(tot_rss):.2f} GB, peak {max(tot_rss):.2f} GB, avg idle share {statistics.mean(tot_idle)*100:.0f}%\n")

print("## per run (lane|root pid; tree = root + repo descendants in that sample)")
print("| run | lane | root cmd | samples | median tree size | peak tree size | median RSS GB | peak RSS GB | median CPU % |")
print("|---|---|---|---|---|---|---|---|---|")
runs = defaultdict(lambda: {"lane": "", "cmd": "", "sz": [], "rss": [], "cpu": []})
for x in recs:
    for r in x["runs"]:
        k = f'{r["lane"]}|{r["root"]}'
        d = runs[k]
        d["lane"], d["cmd"] = r["lane"], r["root_cmd"][:70]
        d["sz"].append(r["size"]); d["rss"].append(r["rss_kb"] / GB); d["cpu"].append(r["cpu"])
for k, d in sorted(runs.items(), key=lambda kv: -max(kv[1]["rss"])):
    print(f'| {k} | {d["lane"]} | {d["cmd"]} | {len(d["sz"])} | {statistics.median(d["sz"]):.0f} | {max(d["sz"])} | '
          f'{statistics.median(d["rss"]):.2f} | {max(d["rss"]):.2f} | {statistics.median(d["cpu"]):.0f} |')

print("\n## per lane (all samples)")
lanes = defaultdict(list)
for x in recs:
    for ln, s in x["lanes"].items():
        lanes[ln].append(s["rss_kb"] / GB)
for ln, v in sorted(lanes.items(), key=lambda kv: -max(kv[1])):
    print(f"- {ln}: samples {len(v)}, median RSS {statistics.median(v):.2f} GB, peak {max(v):.2f} GB")

print("\n## VM per sample")
print("| tick | time | repo procs | repo RSS GB | idle procs | swap used MB | compressor GB | free % |")
print("|---|---|---|---|---|---|---|---|")
for x in recs:
    vm = x["vm"]
    comp = vm.get("Pages occupied by compressor", 0) * 16384 / 1073741824
    print(f'| {x["idx"]} | {x["ts"]} | {x["n"]} | {x["rss_kb"]/GB:.2f} | {x["idle_n"]} | {vm.get("swap_used_mb",0):.0f} | {comp:.2f} | {vm.get("mem_free_pct","-")} |')
sw = [x["vm"].get("swap_used_mb", 0) for x in recs]
print(f"\nswap used MB: min {min(sw):.0f}, median {statistics.median(sw):.0f}, peak {max(sw):.0f}")
