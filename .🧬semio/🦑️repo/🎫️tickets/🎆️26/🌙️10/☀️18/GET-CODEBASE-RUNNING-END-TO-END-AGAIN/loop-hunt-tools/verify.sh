#!/bin/sh
# Rebuilds the overlay (/private/tmp/loop-hunt/ov) from the live files, applies the staged set there, and runs every proof: repeat law (control must fail, patched must pass), existing settlement law, 7 mutations, real-module integration. Live tree untouched.
set -e
T="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$T" && git rev-parse --show-toplevel)"; cd "$W"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; OV=/private/tmp/loop-hunt/ov; R="$T/🗑️generated/run-test"
rm -rf "$OV"; for f in "$L/🏗️build/👁️observation/🟨️.mjs" "$L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs" "$L/🏗️build/🕸️graph/⏱️phases/🟨️.mjs" "$L/🏗️build/🕸️graph/⏱️phases/🟨️.d.mts"; do mkdir -p "$OV/$(dirname "$f")"; cp "$f" "$OV/$f"; done
python3 "$T/apply.py" apply --root "$OV" --write
mkdir -p "$R/🧬️schema"; cp "$T/staged/new/settle-repeat/test.ts" "$R/🟦️.ts"; cp "$T/staged/new/settle-repeat/fixture.json" "$R/🔣️.json"; cp "$T/staged/new/settle-repeat/schema.json" "$R/🧬️schema/🔣️.json"
python3 "$T/staged/new/settle-repeat/oracle.py" "$T/staged/new/settle-repeat/fixture.json" > "$T/🗑️generated/oracle.json"
python3 - "$T" <<'PY'
import json,sys
t=sys.argv[1];o=json.load(open(t+"/🗑️generated/oracle.json"));f=json.load(open(t+"/staged/new/settle-repeat/fixture.json"))
bad=[s["name"] for s in f["scenarios"] if {k:v for k,v in s["expect"].items() if k!="failure" or s["expect"]["settled"]=="failure"}!={k:v for k,v in o[s["name"]].items()}]
print("python oracle vs fixture mismatches:",bad);sys.exit(1 if bad else 0)
PY
echo "--- repeat law: live (control, must FAIL)"; if (cd "$R" && SETTLE_SOURCE="$W/$L/🏗️build/👁️observation/🟨️.mjs" bun test ./🟦️.ts >/dev/null 2>&1); then echo "CONTROL PASSED (unexpected)"; exit 1; else echo "control fails as expected"; fi
echo "--- repeat law: patched"; (cd "$R" && SETTLE_SOURCE="$OV/$L/🏗️build/👁️observation/🟨️.mjs" bun test ./🟦️.ts 2>&1 | tail -5)
echo "--- existing settlement law"; bun "$T/existing-law.ts" "$OV/$L/🏗️build/👁️observation/🟨️.mjs" "$L/🏗️build/🧪️tests/🕸️graph/🔄️settlement/🔣️.json"
echo "--- mutations"; python3 "$T/mutation-check.py" "$OV/$L/🏗️build/👁️observation/🟨️.mjs"
echo "--- real-module integration"; for v in live patched; do VARIANT=$v bun "$T/integration.ts" | tail -1; done
