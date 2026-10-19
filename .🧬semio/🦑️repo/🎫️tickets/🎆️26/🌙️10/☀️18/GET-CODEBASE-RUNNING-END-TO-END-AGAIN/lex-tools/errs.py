"""🔎 Lists every rustc diagnostic (stub-involved ones included) of an oracle error log that lands in the given path fragments. usage: errs.py <err-file> <fragment>... [--code E0599]"""
import json, sys, re
args = sys.argv[1:]
code = None
if "--code" in args:
    i = args.index("--code"); code = args[i + 1]; del args[i:i + 2]
path, frags = args[0], args[1:]
seen = set()
for line in open(path, encoding="utf-8"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    try:
        d = json.loads(line)
    except Exception:
        continue
    if d.get("level") != "error":
        continue
    c = (d.get("code") or {}).get("code") or ""
    if code and c != code:
        continue
    for s in d.get("spans", []):
        if s.get("is_primary") and any(f in s.get("file_name", "") for f in frags):
            key = (c, s["file_name"], s["line_start"], d["message"])
            if key in seen:
                continue
            seen.add(key)
            print(f'{c} {s["file_name"].split("/")[-2]}:{s["line_start"]}: {d["message"]} | {(s.get("label") or "")[:140]}')
