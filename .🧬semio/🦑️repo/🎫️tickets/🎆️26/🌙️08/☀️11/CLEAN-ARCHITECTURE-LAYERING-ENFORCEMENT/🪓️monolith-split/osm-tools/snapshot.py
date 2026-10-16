import os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
for key in ["services", "mcp"]:
    tree = TARGETS["services"]["tree"] if key == "services" else TARGETS["transport"]["tree"]
    d = f"{W}/snap-{key}"
    if os.path.exists(d):
        print("exists", d); continue
    n = copy_tree(tree, d)
    print(key, n, "files")
hashes = {k: sha(read(os.path.join(ROOT, t["file"]))) for k, t in TARGETS.items()}
hashes["services-tests"] = sha(read(os.path.join(ROOT, M, "🛎️services/🧪️tests/🔬️component-unit/🦀️.rs")))
write(f"{W}/snap-hashes.json", json.dumps(hashes, indent=1))
print(hashes)
