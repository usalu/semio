"""🧮️ Counts the cells of a matrix output folder by status and the failing cells by check: `python3 cells.py <dir>`."""
import collections, glob, json, sys
files = glob.glob(sys.argv[1] + "/matrix.json") or glob.glob(sys.argv[1] + "/**/*.json", recursive=True)
status, checks = collections.Counter(), collections.Counter()
for f in files:
    try:
        data = json.load(open(f))
    except Exception:
        continue
    cells = data if isinstance(data, list) else data.get("cells", [])
    for cell in cells:
        if not isinstance(cell, dict) or "status" not in cell:
            continue
        status[cell["status"]] += 1
        if cell["status"] == "fail":
            for name in {x if isinstance(x, str) else x.get("check", "?") for x in cell.get("failures", cell.get("findings", [])) or ["?"]}:
                checks[name] += 1
    if status:
        break
print(dict(status), dict(checks))
