import json, sys, re, collections

targets = collections.defaultdict(list)
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    try:
        m = json.loads(line)
    except Exception:
        continue
    if m.get("reason") != "compiler-message":
        continue
    msg = m["message"]
    if msg.get("level") != "error":
        continue
    t = m["target"]
    name = t["name"] + (" (test)" if t.get("test") else "")
    spans = [s for s in msg.get("spans", []) if s.get("is_primary")]
    loc = ""
    if spans:
        s = spans[0]
        f = re.sub(r".*[/\\]editor[/\\]", "", s["file_name"])
        loc = f"{f}:{s['line_start']}:{s['column_start']}"
    targets[name].append((loc, msg["message"][:200]))

for name, errs in sorted(targets.items()):
    print(f"## {name}: {len(errs)}")
    for loc, text in sorted(set(errs)):
        print(f"  {loc} {text}")
