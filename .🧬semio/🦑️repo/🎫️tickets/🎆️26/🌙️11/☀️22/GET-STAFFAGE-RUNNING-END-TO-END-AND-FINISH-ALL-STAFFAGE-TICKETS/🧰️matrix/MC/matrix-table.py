"""📋️ The matrix lines of `matrix.sh` (`<id> | exit <n> | <s> s | peak <MB> MB | status {…} <failing checks>`) as a markdown table, the last line of an asset counting: `python3 matrix-table.py <matrix log> [<id>…]` prints the table and, last, `<clean> of <run> without a fail cell`."""
import ast, re, sys
rows = {}
for line in open(sys.argv[1]):
    parts = [p.strip() for p in line.split(" | ", 4)]
    if len(parts) == 5:
        rows[parts[0]] = parts
ids = sys.argv[2:] or list(rows)
print("| Asset | Run | Peak | Cells by status | Failing checks |\n|---|---|---|---|---|")
clean = 0
for a in ids:
    if a not in rows:
        print(f"| `{a}` | not run | | | |")
        continue
    _, code, secs, peak, rest = rows[a]
    m = re.match(r"status (\{.*?\})\s*(.*)", rest)
    status = ast.literal_eval(m.group(1)) if m else {}
    checks = (m.group(2).strip() if m else rest)[:300]
    clean += bool(m) and status.get("fail", 0) == 0 and code == "exit 0"
    print(f"| `{a}` | {code}, {secs} | {peak.replace('peak ', '')} | {', '.join(f'{n} {k}' for k, n in sorted(status.items()))} | {checks or 'none'} |")
print(f"\n{clean} of {sum(1 for a in ids if a in rows)} without a fail cell")
