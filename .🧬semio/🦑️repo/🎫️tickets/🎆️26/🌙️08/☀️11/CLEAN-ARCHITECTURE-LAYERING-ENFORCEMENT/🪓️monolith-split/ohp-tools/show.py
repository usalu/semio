"""🔎 Compact rustc diagnostics from a pos final.err whose primary file contains a substring (skips __ext/stub noise).
usage: show.py <final.err> <file-substr> [--codes E0425,..] [--max N] [--full]"""
import json, sys, re
a = sys.argv[1:]
sub = a[1]
codes = set(a[a.index("--codes") + 1].split(",")) if "--codes" in a else None
mx = int(a[a.index("--max") + 1]) if "--max" in a else 40
full = "--full" in a
n = 0
for line in open(a[0], encoding="utf-8", errors="replace"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    try:
        d = json.loads(line)
    except Exception:
        continue
    if d.get("level") != "error":
        continue
    code = (d.get("code") or {}).get("code") or ""
    if codes and code not in codes:
        continue
    prim = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
    if not prim or sub not in prim["file_name"]:
        continue
    r = d.get("rendered", "")
    if "__ext" in r and not full:
        continue
    txt = " ".join(t.get("text", "").strip() for t in prim.get("text", []))[:200]
    fn = prim["file_name"]
    fn = "/".join(fn.split("/")[-3:])
    kids = " | ".join(c.get("message", "")[:100] for c in d.get("children", []))[:200]
    if full:
        print(r[:1800])
    else:
        print(f"{code} {fn}:{prim['line_start']}: {d['message'][:160]}\n    > {txt}\n    ~ {kids}")
    n += 1
    if n >= mx:
        break
