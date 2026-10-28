#!/bin/sh
# 🧪️ [sealflip] Verifies batch `sealflip` in a root where it is applied (a mirror; never live before the gate applies it): every file applied, type check of the touched TypeScript (the count proves tsc saw them), the sealed + door laws (the startup red is the known flip-independent baseline), the dashboard collision probe on the real door, and the graph-free hot-path simulation (digest must stay through peer edits and move only at the explicit promotion). usage: sh verify.sh <root>
R="${1:?root}"; H="$(cd "$(dirname "$0")" && pwd)"; T="$(cd "$H/../../.." && pwd)"; B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; bad=0
REQ="✏️s/🧑‍💻dev/🧩️catalog/📣️publication"; [ -d "$R/$REQ" ] || { echo "the mirror lacks $REQ (the capture of the sealed entry lstats ✏️s): create it with: python3 -I C/mirror-tools/mirror.py create <name> --only library process launcher \"$REQ\""; exit 2; }
PB="${PB:-$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun 2>/dev/null | head -1)}"; [ -x "$PB" ] || PB="$(command -v bun)"
python3 -I - "$H" "$R" <<'PY' || bad=1
import hashlib, json, os, sys
h, root = sys.argv[1:3]; bad = 0
for rel, row in json.load(open(os.path.join(h, "manifest.json"), encoding="utf8")).items():
    data = open(os.path.join(root, rel), "rb").read()
    if rel.endswith("🏛️sealed/🔣️.json"): data = data.replace(b'"defaultMode": "sealed",', b'"defaultMode": "live",')
    if hashlib.sha256(data).hexdigest() != row["new"]: print("NOT APPLIED", rel[-80:]); bad = 1
print("batch applied (policy compared with the flip line set back to live)" if not bad else "batch NOT applied"); sys.exit(bad)
PY
cat > "$R/tsconfig.sf.json" <<JSON
{"extends":"./🧰️framework/🛍️products/🦑️repo/tsconfig.json","compilerOptions":{"noEmit":true,"incremental":false,"skipLibCheck":true},"files":["$B/🏛️sealed/🟦️.ts","$B/🏛️sealed/🚦️trampoline/🟦️.ts","$B/🏛️sealed/🧪️tests/🗂️store/🟦️.ts","$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts","$B/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts","$B/🚪️door/🧪️tests/🟦️.ts"],"include":[]}
JSON
(cd "$R" && echo "tsc saw sealed files: $(node_modules/.bin/tsc -p tsconfig.sf.json --listFiles 2>/dev/null | grep -c '🏛️sealed/')" && node_modules/.bin/tsc -p tsconfig.sf.json 2>&1 | grep -E "^🧰" | grep -E "🏛️sealed|🚪️door" && bad=1); rm -f "$R/tsconfig.sf.json"
for f in "🏛️sealed/🧪️tests/🗂️store" "🏛️sealed/🧪️tests/🚦️trampoline" "🏛️sealed/🧪️tests/👥️processes" "🏛️sealed/🧪️tests/🔀️interleavings" "🏛️sealed/🧭️mode/🧪️tests" "🚪️door/🧪️tests" "🚪️door/🧪️tests/🚦️startup" "🧭️invocation/🕸️graph/🧠️memo/🧪️tests" "🔭️witness/🧪️tests"; do
 out=$(cd "$R" && bun test --timeout 300000 "./$B/$f/🟦️.ts" 2>&1); p=$(echo "$out" | grep -E '^ *[0-9]+ pass' | tr -dc 0-9); n=$(echo "$out" | grep -E '^ *[0-9]+ fail' | tr -dc 0-9)
 echo "${p:-?} pass ${n:-?} fail  $f"; case "$f:${n:-x}" in *startup:1) ;; *:0) ;; *) bad=1;; esac
done
code=$(cd "$R" && SEMIO_REPO_IMPLEMENTATION=rust "$PB" "$B/🚪️door/📜️script.ts" leases status >/dev/null 2>&1; echo $?); echo "dashboard switch rust -> leases status exit $code (want 0)"; [ "$code" = 0 ] || bad=1
python3 -I "$T/sealflip-tools/hotsim.py" "$R" verify --var SEMIO_REPO_SEAL --promote-step > /dev/null || bad=1
python3 -I - "$T/🗑️generated/sealflip/hotsim-verify.jsonl" <<'PY' || bad=1
import json, sys
rows = [json.loads(l) for l in open(sys.argv[1])]; runs = {r["step"]: r for r in rows if r["step"].startswith("run-")}
ok = runs["run-1"]["served"] == runs["run-2"]["served"] == runs["run-3"]["served"] == runs["run-4"]["served"] != runs["run-5"]["served"] and not any(r["promoted"] for r in runs.values()) and runs["run-4"]["live"] != runs["run-1"]["live"] and all(r["exit"] == 0 for r in rows if "exit" in r)
print("hot path: digest", " ".join(runs[k]["served"][:8] for k in sorted(runs)), "| live", " ".join(runs[k]["live"][:8] for k in sorted(runs)), "->", "OK" if ok else "FAIL"); sys.exit(0 if ok else 1)
PY
echo "VERIFY exit $bad"; exit $bad
