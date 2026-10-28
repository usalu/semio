#!/bin/sh
# ♨ Laws of batch `warmderive` (audit11 copy of the edit30 batch: parsed files keyed by their compiler options; the idle deriver stops when its closure moves on disk or it outgrows its resident size): sh verify.sh <repository or mirror root>
# 1. the warm laws (corpus: warm == cold after every step with real TypeScript and the planner's observation check, reuse exactly where the guard holds, an always-armed deriver wrong where the corpus says; store; protocol; the detached process refuses an implementation not on disk; wiring);
# 2. strict tsc over the new and edited TypeScript (only the one pre-existing error of 🧪️testing/🟨️bun/🚧️operands may appear);
# 3. the schema-first policy literal law (the new `warm` policy key carries its EN+DE description);
# 4. the memo and script-inputs suites: no failure outside `known-failures.txt` (2 pre-existing router laws on real repository scripts, identical on base).
# Rebase after drift: python3 -I edit_si.py <fresh root> apply && python3 -I edit_memo.py <fresh root> apply, copy the new files, then make-stage.
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; out="$here/../../../🗑️generated/audit11/wd-verify"; mkdir -p "$out"
cd "$root" || exit 2
python3 -I "$here/stage.py" check --root "$root" | grep '^base' && { echo "VERIFY FAIL: batch not applied"; exit 1; }
W=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
bun test --timeout 300000 "$W/🧮️derive/♨️warm/🧪️tests/🟦️.ts" || { echo "VERIFY FAIL: warm laws"; exit 1; }
A="$root/${W#./}"; printf '{"extends":"%s","compilerOptions":{"noEmit":true,"incremental":false,"skipLibCheck":true,"strict":true},"files":["%s","%s","%s","%s"],"include":[]}' "$root/🧰️framework/🛍️products/🦑️repo/tsconfig.json" "$A/🚪️command/🟦️.ts" "$A/🧮️derive/♨️warm/🟦️.ts" "$A/🧮️derive/♨️warm/⌨️main/🟦️.ts" "$A/🧮️derive/♨️warm/🧪️tests/🟦️.ts" > "$out/tsconfig.json"
"$root/node_modules/.bin/tsc" -p "$out/tsconfig.json" > "$out/tsc.out" 2>&1
bad=$(grep "error TS" "$out/tsc.out" | grep -v "🚧️operands" | wc -l | tr -d ' '); echo "tsc errors outside the pre-existing one: $bad"; [ "$bad" = 0 ] || { cat "$out/tsc.out" | head -20; exit 1; }
bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧬️policy-literals/🟦️.ts || { echo "VERIFY FAIL: policy literal law (every memo policy key described in EN and DE, no policy literal in code)"; exit 1; }
SEMIO_TEST_ARTIFACT_DIR="$out" bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/suites.out" 2>&1
python3 -I "$here/../../../edit30-tools/staged/freshmtime/junit-set.py" "$out/suites.xml" > "$out/suites.set"; head -1 "$out/suites.set"
new=$(grep '^FAIL' "$out/suites.set" | sed 's/^FAIL //' | sort | comm -13 "$here/known-failures.txt" -)
if [ -z "$new" ]; then echo "VERIFY OK"; else echo "VERIFY FAIL: new failures:"; echo "$new"; exit 1; fi
