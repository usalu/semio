import glob, json, sys
ROOT = r"C:\Users\Ueli\.claude\projects\C--git-semio"
want_ts, want_file = sys.argv[1], sys.argv[2]
for path in glob.glob(ROOT + "/*/subagents/*.jsonl"):
    if want_file not in path: continue
    for line in open(path, encoding="utf-8"):
        if want_ts not in line: continue
        rec = json.loads(line)
        for part in rec.get("message", {}).get("content", []):
            if isinstance(part, dict) and part.get("type") == "tool_use":
                print(part["input"].get("command") or json.dumps(part["input"], ensure_ascii=False, indent=1))
