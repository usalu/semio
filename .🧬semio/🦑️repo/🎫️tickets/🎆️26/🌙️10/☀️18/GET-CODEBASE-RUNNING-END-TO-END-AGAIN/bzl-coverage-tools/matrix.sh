#!/bin/bash
# usage: matrix.sh   cold / warm / restore / bazel clean / early cutoff / value change / one-family change / no-cache / failing test / interactive over the fixture families; writes /private/tmp/bzlcov/matrix.txt
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/bzl-coverage-tools"
OUT=/private/tmp/bzlcov/matrix.txt
ALL="fx-bun:generate,fx-tsc:typecheck,fx-vitest:test,fx-buntest:test,fx-vite:build,fx-pw:test-e2e,fx-go:test,fx-go:build,fx-py:generate,fx-py:test,fx-latex:build,fx-wasm:wasm"
FX=/private/tmp/bzlcov-ov/fx
summ(){ python3 -I "$T/exec-summary.py" "/private/tmp/bzlcov/logs/exec-$1.json" /private/tmp/bzlcov/graph.json | sort | awk -F'\t' '{k=$1" "$2" "$3; c[k]++} END{for(k in c) print "    "k" x"c[k]}' | sort; python3 -I "$T/exec-summary.py" "/private/tmp/bzlcov/logs/exec-$1.json" /private/tmp/bzlcov/graph.json | tail -1 | sed 's/^/    /'; command grep -E "^INFO: [0-9]+ process|\(cached\) PASSED|  PASSED|FAILED|Executed [0-9]+ out of" "/private/tmp/bzlcov/logs/$1.log" | sed 's/^/    /' | cut -c1-150; }
phase(){ label=$1; tasks=$2; shift 2; echo "== $label" | tee -a "$OUT"; "$T/go.sh" "$label" "$tasks" "$@" | head -1 | tee -a "$OUT"; summ "$label" | tee -a "$OUT"; }
: > "$OUT"
"$T/bz.sh" shutdown > /dev/null 2>&1; chmod -R u+w /private/tmp/bzlcov/ob 2>/dev/null; rm -rf /private/tmp/bzlcov/ob /private/tmp/bzlcov/ws /private/tmp/bzlcov/disk /private/tmp/bzlcov/logs /private/tmp/bzlcov/publish.log
python3 -I "$T/fixtures.py" > /dev/null
phase p1-cold "$ALL"
phase p2-warm "$ALL"
rm -rf "$FX/vite/dist" "$FX/bun/out" "$FX/wasm/out"
phase p3-restore-deleted-outputs "$ALL"
echo "    restored: $(ls $FX/vite/dist | tr '\n' ' ')| $(ls $FX/bun/out | tr '\n' ' ')| $(ls $FX/wasm/out | tr '\n' ' ')" | tee -a "$OUT"
"$T/bz.sh" clean > /dev/null 2>&1
phase p4-after-bazel-clean "$ALL"
python3 -I "$T/fixtures.py" --mutate bun/input.json '\n' > /dev/null
phase p5-whitespace-change-early-cutoff "$ALL"
echo '{"values":[1,2,3,4,5]}' > "$FX/bun/input.json"
phase p6-value-change-dependents "$ALL"
printf '\n// changed\n' >> "$FX/🐹️go/hello.go"
phase p7-go-source-change "$ALL"
printf 'print("changed")\n' >> "$FX/🐍️python/gen.py"
phase p8-python-source-change "$ALL"
printf '\nexport const extra:number=1;\n' >> "$FX/tsc/a.ts"
phase p9-tsc-source-change "$ALL"
rm -f /private/tmp/bzlcov/publish.log
phase p10-no-cache-publish-1 "fx-bun:publish"
phase p10-no-cache-publish-2 "fx-bun:publish"
echo "    publish executions: $(wc -l < /private/tmp/bzlcov/publish.log)" | tee -a "$OUT"
phase p11-failing-test-1 "fx-buntest:test-fail"
phase p11-failing-test-2 "fx-buntest:test-fail"
export BZL_COV_INTERACTIVE=1
phase p12-interactive-1 "fx-dev:dev"
phase p12-interactive-2 "fx-dev:dev"
unset BZL_COV_INTERACTIVE
"$T/bz.sh" shutdown > /dev/null 2>&1
