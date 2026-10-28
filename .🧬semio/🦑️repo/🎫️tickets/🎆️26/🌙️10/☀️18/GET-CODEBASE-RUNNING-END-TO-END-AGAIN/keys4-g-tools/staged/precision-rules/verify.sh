#!/bin/sh
# 🧪 verify.sh : graph-free laws of the keys4-g precision batch. K4_ROOT = checkout to verify (default live; use a mirror root for apply tests), K4_EVIDENCED = evidence directory (default evidenced/; a control points it at a copy with one input added). Writes nothing below the root.
# 1. stage.py check. 2. The root document with the staged rule at its head is accepted by the root's precision engine; the eight targets derive a declared set (files + pattern expansion + kept globs) that is a SUPERSET of the set the read oracle ran with (evidenced/*.allow): growth is reported (new imports, mounted sources), a dropped input fails unless evidenced/dropped.json explains it (a gitignored `🤖️generated/` file absent from the root is environment, not a drop); a NEW unresolved derivation fails (evidenced/unresolved.json lists the two the sqlite native class has always had). The checker itself is proven by a negative control. 3. Live precision laws (single files).
R="${K4_ROOT:-/Users/ueli/Documents/semio}"; S="$(cd "$(dirname "$0")" && pwd)"; E="${K4_EVIDENCED:-$S/evidenced}"
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision"
OUT="${TMPDIR:-/tmp}/keys4-g-verify.$$.json"
cd "$R" || exit 1
R="$(pwd -P)"
export K4_ROOT="$R"
python3 -I "$S/stage.py" check --root "$R" || exit 1
nice bun "$S/derive-merged.ts" "$P" "$S/rules/framework-rs-tests.json" @semio-tech/framework-rs ALL "🧰️framework/📦️packages/🦀️rust/📋️project.json" "$OUT" > /dev/null || exit 1
python3 -I - "$OUT" "$E" "$R" <<'PY' || exit 1
import json, os, sys
def lost(evidenced, now, explained, absent=lambda p: False): return sorted(p for p in evidenced - now if p not in explained and not absent(p))
assert lost({"a", "b"}, {"a"}, {}) == ["b"] and lost({"a", "x/🤖️generated/b"}, {"a"}, {}, lambda p: "/🤖️generated/" in p) == [] and lost({"a", "b"}, {"a"}, {"b": "why"}) == [] and lost({"a"}, {"a", "c"}, {}) == [], "checker negative control failed"
results = {r["target"]: r for r in json.load(open(sys.argv[1], encoding="utf-8"))["results"]}
dropped = json.load(open(sys.argv[2] + "/dropped.json", encoding="utf-8"))
known = json.load(open(sys.argv[2] + "/unresolved.json", encoding="utf-8"))
bad, wanted = [], sorted(n[:-6] for n in os.listdir(sys.argv[2]) if n.endswith(".allow"))
for target in wanted:
    row = results.get(target)
    if not row or "error" in row: bad.append((target, "no derivation", (row or {}).get("error"))); continue
    before = set(open("%s/%s.allow" % (sys.argv[2], target), encoding="utf-8").read().split("\n")) - {""}
    now = set(row["declared"])
    missing = lost(before, now, dropped.get(target, {}), lambda p: "/🤖️generated/" in p and not os.path.exists(os.path.join(sys.argv[3], p)))
    print("%-42s evidenced %5d declared %5d  +%d new  -%d explained  -%d LOST" % (target, len(before), len(now), len(now - before), len((before - now) - set(missing)), len(missing)))
    if missing: bad.append((target, "inputs dropped vs the oracle evidence", missing[:5]))
    fresh = [u for u in row["unresolved"] if u not in known.get(target, [])]
    if fresh: bad.append((target, "new unresolved derivation (the native sqlite class names two bun test paths it skips)", fresh[:3]))
print("all %d targets keep every evidenced input" % len(wanted) if not bad else "PRECISION LOSS (evidence must be re-run or the drop explained): %s" % bad)
sys.exit(1 if bad else 0)
PY
rm -f "$OUT"
nice bun test "./$P/🧪️tests/🟦️.ts" "./$P/🧪️tests/🛤️router/🟦️.ts" --timeout 120000 2>&1 | tail -6
