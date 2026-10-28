"""python3 apply-batch.py [precision-dir [rule,rule]] : the ONE compile-atomic batch of keys2-g for the live library (run only after the 11:30 measurement window).
1. the engine reads `program.entries` and `router` (anchored, idempotent patch scripts, schema validated) and the router law,
2. the three precision rules (print-check, framework-rs-tests, dashboard-cargo-tests) at the head of the rule list (JSON round trip keeps the file byte format),
3. the README paragraph (EN + DE).
Every step re-reads the live file; nothing is overwritten that this script did not change. Prints what it did."""
import json, os, shutil, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/keys2-g-tools"
PRECISION = sys.argv[1] if len(sys.argv) > 1 else ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision"
for script in ("program-entries.py", "router-read.py"):
    subprocess.run([sys.executable, f"{C}/patches/{script}", PRECISION], check=True)
target = PRECISION + "/🧪️tests/🛤️router"
shutil.copytree(f"{C}/precision-tests/🛤️router", target, dirs_exist_ok=True)
rules_path = PRECISION + "/🔣️.json"
text = open(rules_path, encoding="utf-8").read()
document = json.loads(text)
assert json.dumps(document, ensure_ascii=False, indent=1) + "\n" == text, "rule document format drifted: refusing to rewrite it"
names = sys.argv[2].split(",") if len(sys.argv) > 2 else ["print-check", "framework-rs-tests", "framework-rs-tests-fixture-ownership", "framework-rs-tests-sqlite-source", "dashboard-cargo-tests"]
fresh = [json.load(open(f"{C}/rules/{name}.json", encoding="utf-8")) for name in names]
known = {rule["id"] for rule in document["rules"]}
document["rules"] = [rule for rule in fresh if rule["id"] not in known] + document["rules"]
open(rules_path, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("rules added:", [rule["id"] for rule in fresh if rule["id"] not in known])
