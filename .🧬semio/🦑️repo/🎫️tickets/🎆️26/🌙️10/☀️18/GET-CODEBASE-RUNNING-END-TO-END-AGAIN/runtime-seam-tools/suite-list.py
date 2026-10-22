#!/usr/bin/env python3
"""Writes suites.json: the test suites next to every site the manual edits touch, plus the round-1 suites. Usage: suite-list.py <tree>"""
import json, os, re, sys
tree = sys.argv[1]
P = "🧰️framework/🔨️modules/🏃️process"; L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
fixed = {
 "lease-physical": P + "/🔒️leases/📁️physical/🧪️tests/🟦️.ts", "lease-baseline": P + "/🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts",
 "directory-storage": P + "/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🟦️.ts", "remote-cache": L + "/🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts",
 "cache-gc": L + "/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts", "cargo-workspaces": L + "/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts",
 "zero-touch": L + "/🏗️build/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts", "selection": L + "/🏗️build/🧭️selection/🧪️tests/🟦️.ts",
 "lease-descriptors": P + "/🔒️leases/📁️physical/🧪️tests/📂️descriptors/🟦️.ts", "ledger": L + "/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts",
 "registry-import-language": L + "/🧪️tests/🌐️registry-import-language/🟦️.ts", "registry-and-cargo-providers": L + "/🧪️tests/🔬️workspace-contract/🔗️registry-and-cargo-providers/🟦️.ts",
 "cargo-toml": L + "/🗂️workspaces/🦀️cargo/📖️toml/🧪️tests/🟦️.ts", "descriptor-publication": L + "/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts",
 "cargo-preparation-storage": L + "/🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🧪️tests/🟦️.ts", "cargo-command": L + "/🗂️workspaces/🦀️cargo/🏃️command/🧪️tests/🟦️.ts",
}
manual = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "manual-edits.json"), encoding="utf8"))
paths = [row["path"] for row in manual["files"]] + [row["from"] for row in manual.get("moves", [])]
suites = dict(fixed)
for path in paths:
    directory = os.path.dirname(path)
    options = [path] if "🧪️tests" in path.split("/") and path.endswith(".ts") else [os.path.join(directory, "🧪️tests/🟦️.ts"), os.path.join(os.path.dirname(directory), "🧪️tests/🟦️.ts")]
    for option in options:
        if os.path.exists(os.path.join(tree, option)) and option not in suites.values():
            suites["r2-" + re.sub(r"[^0-9a-z]+", "-", option.split("🧪️tests")[0].split("/")[-2].lower() + "-" + str(len(suites))).strip("-")] = option
            break
json.dump(suites, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "suites.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(suites))
