"""👀️ W14: per worker, from the W13 poll file argv[1], the samples after the landing epoch argv[2] (seconds): time, state, report age, taken, CLI phase; then the newest stack sample per worker taken after the landing with the share of `stat` in the main thread's own time."""
import json, os, re, sys, time, collections
rows, since = collections.defaultdict(list), float(sys.argv[2])
for line in open(sys.argv[1]):
    r = json.loads(line)
    if r["t"] >= since: rows[r["w"]].append(r)
for worker, list_ in sorted(rows.items()):
    marks, last = [], None
    for r in list_:
        sign = (r["state"], r["taken"], (r["tail"] or "")[:60])
        if sign != last: marks.append(f'{time.strftime("%H:%M", time.localtime(r["t"]))} {r["state"]} age={r["reportAge"]} taken={r["taken"]} | {(r["tail"] or "").replace("Reading the project graph slice ", "")[:70]}'); last = sign
    print(worker[3:], *marks[-6:], sep="\n   ")
print("ready:", sorted(w[3:] for w, l in rows.items() if l and l[-1]["ready"]))
for worker in sorted(rows):
    path = f"/Users/Shared/semio-store/v1/origins/{worker}/ready-log-workspace-sample.txt"
    if not os.path.exists(path) or os.path.getmtime(path) < since: continue
    text = open(path, encoding="utf-8").read().split("\n")
    total = re.search(r"(\d+) samples of its main thread", text[1]); own = [l for l in text[3:14] if l.startswith("  ")]
    stat = sum(int(l.split()[0]) for l in own if " stat " in l + " ")
    print(worker[3:], text[0][:24], "stat", stat, "of", total.group(1) if total else "?", "| top:", own[0].strip()[:60] if own else "")
