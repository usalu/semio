"""📑️ The numbers of the report of batch MH-6 from the logs: `python3 report-data.py <GEN> <id>…` prints per asset the hard findings by code before (`gate0/<id>.log`) and after (`proof/<id>-gate.log`), the proof lines of `proof/<id>.out` and the matrix cells, as Markdown."""
import re, sys, os, collections
gen = sys.argv[1]
def codes(path):
    out = collections.OrderedDict()
    if not os.path.exists(path): return None
    total = None
    for line in open(path):
        m = re.match(r"^\S+ 1:(\d+) HARD (\S+) ×(\d+) worst (\d+) mm", line)
        if m:
            c = out.setdefault(m.group(2), [0, 0, set()]); c[0] += int(m.group(3)); c[1] = max(c[1], int(m.group(4))); c[2].add(m.group(1))
        m = re.match(r"^hard findings: (\d+)", line)
        if m: total = int(m.group(1))
    return total, out
def show(c):
    if c is None: return "not run"
    total, out = c
    return f"**{total}**" + ("" if not out else ": " + ", ".join(f"`{k}` {v[0]} (≤ {v[1]} mm; 1:{', 1:'.join(sorted(v[2], key=int))})" for k, v in out.items()))
for a in sys.argv[2:]:
    print(f"### `{a}`\n")
    print(f"- Before: {show(codes(f'{gen}/gate0/{a}.log'))}")
    print(f"- After: {show(codes(f'{gen}/proof/{a}-gate.log'))}")
    p = f"{gen}/proof/{a}.out"
    if os.path.exists(p):
        for line in open(p):
            line = line.strip()
            if line: print(f"- `{line[:230]}`")
    print()
