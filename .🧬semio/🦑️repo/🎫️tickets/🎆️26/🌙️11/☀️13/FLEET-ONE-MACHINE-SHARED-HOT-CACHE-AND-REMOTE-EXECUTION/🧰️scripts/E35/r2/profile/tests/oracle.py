import hashlib
import json
import sys

rows = json.load(sys.stdin)
policy = rows["policy"]
out = []
for case in rows["cases"]:
    path, text = case["path"], case["text"]
    result = None
    if path.split("/")[-1] == policy["fileName"] and len(text.encode()) <= policy["maximumBytes"]:
        try:
            value = json.loads(text)
            if isinstance(value, dict):
                kept = {k: v for k, v in value.items() if k not in policy["excludedTopLevelFields"]}
                result = hashlib.sha256(json.dumps(kept, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()
        except ValueError:
            result = None
    out.append(result)
print(json.dumps(out))
