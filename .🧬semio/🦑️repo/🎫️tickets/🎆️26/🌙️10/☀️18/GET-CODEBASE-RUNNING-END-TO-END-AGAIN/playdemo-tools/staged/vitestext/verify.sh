#!/bin/sh
# 🧪️ verify.sh <root>: batch `vitestext`. On <root> (a mirror): parse check + the externalization pattern selects library module ids and no product id (POSIX and
# Windows-style ids, Vite normalizes both to "/"). Product law (read-only on LIVE sources, writes only the Vitest cache): both unit suites run with the config
# text of <root> (wrapper re-roots it at the live product), the suites must pass and never report "Captured inventory Source module load failed".
R="${1:?usage: verify.sh <root>}"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
LIVE=/Users/ueli/Documents/semio; G="$C/🗑️generated/playdemo/vx"; mkdir -p "$G/art"; rc=0
for p in "🏢️semio-tech/🎡️play" "♻️mit-bestand/🧺️demonstrator"; do echo "$p/🧪️tests/🎚️config/🟦️.ts"; done > "$G/scan.list"
(cd "$R" && bun "$C/gate-g-tools/scan.ts" "$G/scan.list" "$G/scan") || rc=1
for p in "🏢️semio-tech/🎡️play" "♻️mit-bestand/🧺️demonstrator"; do
  n=$(basename "$p"); cp "$R/$p/🧪️tests/🎚️config/🟦️.ts" "$G/$n.under-test.ts"
  printf 'import { mergeConfig } from "vitest/config";\nimport base from "./%s.under-test.ts";\nexport default mergeConfig(base, { root: "%s", test: { root: "%s" } });\n' "$n" "$LIVE/$p" "$LIVE/$p" > "$G/$n.config.ts"
  (cd "$LIVE/$p" && bun -e "const c=(await import('$G/$n.under-test.ts')).default,x=c.test?.server?.deps?.external??[],hit=(id)=>x.some(e=>e instanceof RegExp?e.test(id):id.includes(e));const lib='/r/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs',own='/r/$p/🔨️modules/🧩️runtime/🟦️.ts';const ok=hit(lib)&&hit('C:'+lib)&&!hit(own);console.log('[DEBUG] pattern',ok?'ok':'WRONG');process.exit(ok?0:1)") || rc=1
  out=$(cd "$LIVE/$p" && SEMIO_TEST_ARTIFACT_DIR="$G/art" bun "$LIVE/node_modules/vitest/vitest.mjs" run --config "$G/$n.config.ts" 2>&1)
  echo "vitest $p: $(echo "$out" | /usr/bin/grep -E 'Test Files|Tests  ' | tr -s ' \n' ' ')"; echo "$out" | /usr/bin/grep -E '^ *FAIL|^Error:' | sort -u | cut -c1-200
  echo "$out" | /usr/bin/grep -q "Captured inventory Source module load failed" && rc=1
  echo "$out" | /usr/bin/grep -qE "Test Files .*failed" && rc=1
done
echo "[DEBUG] verify vitestext rc=$rc"; exit $rc
