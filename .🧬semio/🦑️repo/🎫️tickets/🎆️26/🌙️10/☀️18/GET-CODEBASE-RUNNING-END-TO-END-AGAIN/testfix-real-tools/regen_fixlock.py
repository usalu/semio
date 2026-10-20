#!/usr/bin/env python3
from uregen import *
d = load("📋️surface-fixture-lock.json")
pk = lock_packages(d["path"])
for row in d["entries"]:
    out = subprocess.run(["bun", ".tfr-toml.ts", O + "/" + row["path"]], cwd=O, capture_output=True, text=True).stdout
    names = json.loads(out)
    lk = pk[row["name"]][0]["dependencies"]
    print(row["name"], len(row["dependencies"]), "->", len(names), "lock agrees:", lk == names)
    row["dependencies"] = names
    row["missingFirstparty"] = [n for n in names if n.startswith("semio-framework-") and n not in pk]
save("📋️surface-fixture-lock.json", d)
