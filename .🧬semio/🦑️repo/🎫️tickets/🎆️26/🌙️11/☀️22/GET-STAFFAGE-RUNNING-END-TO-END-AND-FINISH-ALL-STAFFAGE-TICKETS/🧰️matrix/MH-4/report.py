"""📝️ The measured part of the report of the batch: `python3 report.py <GEN> <ops dir> <id>…` prints per asset a markdown section with the hard findings of the gate before (baseline log `gate0/<id>.log`) and after (`try/<id>.gate-full.log`) by code with count and worst delta, the remaining findings with their bones, the proof lines (`proof/<id>.out`) and the ops by kind."""
import json, re, sys, os, collections
gen, opsdir, ids = sys.argv[1], sys.argv[2], sys.argv[3:]
def codes(path):
    out = collections.OrderedDict(); bones = collections.defaultdict(list); total = None; cur = None
    if not os.path.exists(path): return out, bones, None
    for line in open(path):
        m = re.match(r"\S+ 1:(\d+) HARD (\S+) ×(\d+) worst (\d+) mm", line)
        if m:
            cur = m.group(2); e = out.setdefault(cur, [0, 0, set()]); e[0] += int(m.group(3)); e[1] = max(e[1], int(m.group(4))); e[2].add(m.group(1)); continue
        if line.startswith("    ") and cur:
            b = re.match(r"\s+(\S+) ×(\d+) (\d+) mm", line)
            if b: bones[cur].append((b.group(1), int(b.group(2)), int(b.group(3))))
            continue
        cur = None
        m = re.match(r"hard findings: (\d+)", line)
        if m: total = int(m.group(1))
    return out, bones, total
def fmt(c):
    return ", ".join(f"`{k}` {v[0]} (worst {v[1]} mm; 1:{', 1:'.join(sorted(v[2], key=int))})" for k, v in c.items()) or "none"
for a in ids:
    before, _, tb = codes(f"{gen}/gate0/{a}.log")
    after, bones, ta = codes(f"{gen}/try/{a}.gate-full.log")
    ops = json.load(open(f"{opsdir}/{a}.json"))
    kinds = collections.Counter(o["op"] for o in ops)
    print(f"### `{a}`\n")
    print(f"- Before (gate, every subject): **{tb}** hard: {fmt(before)}.")
    print(f"- After: **{ta}** hard: {fmt(after)}.")
    for code, rows in bones.items():
        agg = collections.OrderedDict()
        for b, n, mm in rows:
            e = agg.setdefault(b, [0, 0]); e[0] += n; e[1] = max(e[1], mm)
        print(f"  - open `{code}`: " + ", ".join(f"{b} ×{n} {mm} mm" for b, (n, mm) in agg.items()))
    print(f"- Ops (`🧰️matrix/MH-4/ops/{a}.json`, {len(ops)}): " + ", ".join(f"{k} {n}" for k, n in kinds.most_common()) + ".")
    proof = f"{gen}/proof/{a}.out"
    if os.path.exists(proof):
        print("- Proof:")
        for line in open(proof):
            line = line.rstrip()
            if line: print(f"  - `{line[:230]}`")
    print()
