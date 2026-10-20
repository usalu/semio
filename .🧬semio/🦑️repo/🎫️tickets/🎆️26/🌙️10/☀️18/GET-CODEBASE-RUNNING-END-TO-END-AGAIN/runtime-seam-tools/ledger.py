#!/usr/bin/env python3
"""Classifies the findings left after the migration into ledger rows (path, rule, count, class, owner, reason)."""
import json, re, sys, collections
report = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "../🗑️generated/runtime-seam/final.json", encoding="utf8"))
def classify(path, rule, snippet):
    if "/🧩️semantic-census/" in path: return ("condition-vocabulary", "pm-seam", "package export condition names are ecosystem vocabulary, not the host runtime")
    if re.search(r'\["bun", "node_modules"|runCmd\("bun", \["add"', snippet): return ("package-manager", "pm-seam", "dependency store and install commands name the package manager")
    if "/📏️headroom/" in path or "/🏭️inventory/" in path or "owned-file-approvals" in path or "bundleRuntime" in snippet or "versions.bun" in snippet: return ("launcher", "repo-launcher", "runtime choice and expected invocations in repository policy")
    if "/⏱️profile/" in path: return ("instrumentation", "tool-iface", "spawn census wrapper installed over the runtime's spawn; follows the tool registry")
    if path.endswith("/🏃️process/🟦️.ts") or "/🏃️process/🍞️bun/" in path or "authority-bans" in path or "/⚙️configuration/" in path or "/🦀️cargo/🏃️execution/" in path or "/🩺️environment/" in path: return ("launcher", "repo-launcher", "runtime alias or executable name chosen in repository policy; resolved by the launcher registry")
    if "browser-bundle" in path and "runtime:{kind" in snippet: return ("toolchain-pin", "pm-seam", "pinned toolchain policy names its runtime kind")
    if re.search(r'runOwnedCommand\("bun",\s*\[\s*"x"', snippet): return ("package-manager", "pm-seam", "bun x package runner")
    if re.search(r'"bun",\s*\[?\s*$', snippet) or '"bun"' in snippet and path.endswith("📜️script.ts"): return ("test-runner-command", "test-runner", "invokes the runtime's own test runner subcommand")
    if path.endswith(".cjs"): return ("config-data", "runtime-seam", "builtin-module name list in a CommonJS data module")
    if path.endswith("📜️script.ts") and rule == "specifier": return ("test-runner-command", "test-runner", "invokes the runtime's own test runner subcommand")
    if path.endswith(".d.ts"): return ("ambient-types", "runtime-seam", "type-only module augmentation of the bun sqlite statement; goes with the sqlite caller types")
    if rule == "embedded": return ("embedded-payload", "runtime-seam", "script text executed by the runtime in a child; needs a real module that imports the facade")
    if "bun:test" in snippet or "/🧪️testing/" in path or "/🛤️lane/" in path or "/🧪️inference-close/" in path or "/🧮️compute/🧪️testing/" in path or "sandbox" in path: return ("test-lane", "test-runner", "test infrastructure bound to the Bun test runner")
    if re.search(r'"bun",\s*\[?"test"', snippet) or re.search(r'"bun",\s*\[\s*"test"', snippet): return ("test-runner-command", "test-runner", "invokes the runtime's own test runner subcommand")
    if re.search(r'"bun"', snippet) and re.search(r'\["repo"|"repo"', snippet): return ("launcher", "repo-launcher", "starts the repo CLI through the runtime; replaced by the native repo launcher")
    if "/🔏️authorizers/" in path or "/⚡️caching/🌐️vite/" in path or "/⚡️caching/📦️bundle/" in path or "/🛠️tools/🟦️bun/" in path or "/🔏️code/" in path: return ("nx-adapter", "backend-contract", "Nx adapter code retired with the backend contract")
    if path.endswith(".cjs"): return ("config-data", "runtime-seam", "builtin-module name list in a CommonJS data module")
    if "/📦️dependencies/" in path or "/🚀️bootstrap/" in path: return ("package-manager", "pm-seam", "dependency installer and lock vocabulary")
    if "/🍎️events/" in path or "/📁️directory/" in path or "/🔌️port/" in path: return ("storage-substrate", "storage-seam", "storage backend FFI and service payload")
    return None
rows = collections.OrderedDict(); unclassified = []
for entry in report["remaining"]:
    for f in entry["findings"]:
        c = classify(entry["path"], f["rule"], f["snippet"])
        if c is None: unclassified.append((entry["path"], f)); continue
        key = (entry["path"], f["rule"])
        rows.setdefault(key, [c, 0])[1] += 1
out = [{"path": p, "rule": r, "count": v[1], "class": v[0][0], "owner": v[0][1], "reason": v[0][2]} for (p, r), v in sorted(rows.items(), key=lambda kv: (kv[0][0], kv[0][1]))]
json.dump({"version": 1, "rows": out}, open("src/ledger.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(out), "rows;", sum(r["count"] for r in out), "findings;", len(unclassified), "unclassified")
for p, f in unclassified: print("  UNCLASSIFIED", p.split("/", 3)[-1][-90:], f["line"], f["rule"], f["snippet"][:120])
by = collections.Counter(r["class"] for r in out for _ in range(r["count"]))
print(dict(by))
