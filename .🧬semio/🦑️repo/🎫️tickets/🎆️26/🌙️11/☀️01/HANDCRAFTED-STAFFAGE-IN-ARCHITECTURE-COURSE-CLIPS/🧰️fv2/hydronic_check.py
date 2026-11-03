import importlib.util
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("hydronic_oracle", path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
fresh = module.build()
stored = json.loads(module.FIXTURE.read_text(encoding="utf-8"))


def close(a, b, path="$"):
    if isinstance(a, dict) and isinstance(b, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k], path + "." + k) for k in a)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(close(x, y, path + "[]") for x, y in zip(a, b))
    if isinstance(a, (int, float)) and isinstance(b, (int, float)) and not isinstance(a, bool):
        ok = abs(a - b) <= 1e-9 * max(1.0, abs(a), abs(b))
        if not ok:
            print("MISMATCH", path, a, b)
        return ok
    ok = a == b
    if not ok:
        print("MISMATCH", path)
    return ok


print("hydronic oracle fixture fresh:", close(fresh, stored))
print("plates", len(fresh["plates"]), "coupled rows", len(fresh["coupled"]["expected"]["rows"]))
