#!/bin/sh
# 🧪 FOR gate-g: verifies the staged batch `seal2` on a mirror, never on the live tree. usage: sh verify.sh [mirror-name] [--keep]   (default name s2gv; the mirror is dropped at the end unless --keep)
# Steps: `apply.py check` on the live tree (must say base, or applied once landed), mirror with the batch applied (`apply.py apply --root`), type check (tsc strict) of the three changed TypeScript modules, the laws (sealed store, sealed trampoline with real processes, loader stream laws through the graph-free official envelope), revert, final check. Exit 1 on any failure; the known-red `actual ...` tests of the stream file (4, red at baseline in every mirror) are reported as BASELINE.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/seal2-g-tools"; N="${1:-s2gv}"; MM="python3 -I $C/mirror-tools/mirror.py"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; R="${TMPDIR:-/tmp}/sm/$N"; fail=0
python3 -I "$T/staged/seal2/apply.py" check | tail -1
$MM create "$N" --only library process launcher "✏️s/🧑‍💻dev/🧩️catalog/📣️publication" >/dev/null 2>&1 || { echo "mirror create FAILED"; exit 1; }
python3 -I "$T/staged/seal2/apply.py" check --root "$R" | tail -1
python3 -I "$T/staged/seal2/apply.py" apply --root "$R" | tail -1 || fail=1
cat > "$R/tsconfig.s2g.json" <<JSON
{"extends":"./🧰️framework/🛍️products/🦑️repo/tsconfig.json","compilerOptions":{"noEmit":true,"incremental":false,"skipLibCheck":true},"files":["$B/🏛️sealed/🟦️.ts","$B/🏛️sealed/🚦️trampoline/🟦️.ts","$B/📥️source/🔏️closure/🍞️bun/🟦️.ts","$B/🏛️sealed/🧪️tests/🗂️store/🟦️.ts","$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts"],"include":[]}
JSON
(cd "$R" && node_modules/.bin/tsc -p tsconfig.s2g.json --listFiles | grep -c "sealed/🚦️trampoline/🟦️.ts" | sed 's/^/tsc checked trampoline: /'; node_modules/.bin/tsc -p tsconfig.s2g.json | head -5; [ "$(node_modules/.bin/tsc -p tsconfig.s2g.json | wc -l)" = 0 ]) || { echo "tsc FAILED"; fail=1; }
$MM run "$N" --peers-ok -- bun test --timeout 120000 "./$B/🏛️sealed/🧪️tests/🗂️store/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)" | tr '\n' ' '; echo " store (want 14 pass 0 fail)"
$MM run "$N" --peers-ok -- bun test --timeout 300000 "./$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)" | tr '\n' ' '; echo " trampoline (want 6 pass 0 fail)"
out=$($MM run "$N" --peers-ok -- env VF_WALL=900 bun "$PWD/$C/ver-f-tools/proc-run.ts" file "$R/$B/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)")
echo "$out" | tr '\n' ' '; echo " stream (want 16 pass 4 fail, the 4 are BASELINE reds: actual auxiliary / both CommonJS / CommonJS facades / unnamed)"
case "$out" in *" 4 fail"*) ;; *) fail=1;; esac
python3 -I "$T/staged/seal2/apply.py" revert --root "$R" | tail -1 || fail=1
python3 -I "$T/staged/seal2/apply.py" check --root "$R" | tail -1
[ "$2" = "--keep" ] || $MM drop "$N" >/dev/null 2>&1
echo "VERIFY exit $fail"; exit $fail
