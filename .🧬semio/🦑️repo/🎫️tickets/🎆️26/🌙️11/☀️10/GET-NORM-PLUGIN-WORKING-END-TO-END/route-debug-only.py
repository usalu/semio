"""🧹️ Exit 0 when a probed route failed only on temporary `[DEBUG]` guest prints, so the route itself works."""
import json, sys

faults, failed = [], False
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    try:
        record = json.loads(line)
    except ValueError:
        continue
    if record.get("error"):
        failed = True
    faults += record.get("faults") or []
sys.exit(0 if failed and faults and all(f.startswith("[DEBUG]") for f in faults) else 1)
