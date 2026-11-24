"""🧾️ Prints the per-asset table of the report of batch MF as markdown: hard findings by code before (scan report of the start) and after (scan report of the proof), reported findings after, and the matrix cells by status: table.py <before.json> <after.json> <matrix dir>"""
import collections, glob, json, os, sys

before = {r["id"]: r for r in json.load(open(sys.argv[1]))["rows"]}
after = {r["id"]: r for r in json.load(open(sys.argv[2]))["rows"]}


def hard(row):
    c = collections.Counter()
    for g in row["groups"]:
        if g["tier"] == "hard":
            c[g["code"]] += g["count"]
    return ", ".join(f"{k} {v}" for k, v in sorted(c.items())) or "none"


def cells(asset):
    files = glob.glob(os.path.join(sys.argv[3], asset, "matrix.json"))
    if not files:
        return "not run"
    data = json.load(open(files[0]))
    status, checks = collections.Counter(), collections.Counter()
    for cell in data if isinstance(data, list) else data.get("cells", []):
        status[cell["status"]] += 1
        if cell["status"] == "fail":
            for name in {x.get("check", "?") for x in cell.get("failures", cell.get("findings", [])) or [{}]}:
                checks[name] += 1
    text = ", ".join(f"{v} {k}" for k, v in sorted(status.items()))
    return text + (" (" + ", ".join(f"{k} {v}" for k, v in checks.items()) + ")" if checks else "")


print("| Asset | Hard findings before | Hard after | Reported after | Matrix cells after |")
print("|---|---|---|---|---|")
for asset, row in before.items():
    now = after.get(asset)
    print(f"| `{asset}` | {row['summary']['hard']}: {hard(row)} | {hard(now) if now else '?'} | {now['summary']['reported'] if now else '?'} | {cells(asset)} |")
