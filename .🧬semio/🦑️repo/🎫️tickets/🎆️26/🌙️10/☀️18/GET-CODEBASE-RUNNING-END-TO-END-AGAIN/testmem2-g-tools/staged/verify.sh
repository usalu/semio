#!/bin/zsh
# [testmem2-g] verifies the batch on the tree after `stage.py apply`: graph-free single-file suites with their expected pass counts; exit 1 on any difference.
# usage: verify.sh [--long]   --long also runs the measured law (every declared file run alone, peak RSS against its budget; minutes).
cd /Users/ueli/Documents/semio
P="🧰️framework/🔨️modules/🏃️process/🧪️testing"; W="$P/👷️workers"
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/tmp/testmem2-g-art}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
fail=0
run(){ local dir="$1" want="$2" file="${3:-🟦️.ts}"; got=$(cd "$dir" && nice bun test --timeout 280000 ./$file 2>&1 | grep -E "^ *[0-9]+ pass$" | tail -1 | tr -d ' '); if [ "$got" = "${want}pass" ]; then echo "OK   $want  ${dir##*/}"; else echo "FAIL want $want got '$got'  $dir"; fail=1; fi; }
run "$W/📏️file-budgets/🧪️tests" 8
run "$W/📏️file-budgets/🧪️tests/🧱️capped-files" 2
run "$W/🧪️tests" 10
run "$W/🧪️tests/📐️learning" 4
run "$W/🧪️tests/🧱️capped-run" 1
run "$P/🗂️suites/🧪️tests" 7
run "$P/🏖️sandbox/🧪️tests" 21
A="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers"
run "$A/📦️snapshot/🧪️tests" 5
run "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🖼️captured-image" 4
run "$A/🧪️tests/📦️compiler-init" 2
run "🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary" 2
run "🧰️framework/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface" 4
python3 "$W/📏️file-budgets/🧪️tests/🐍️oracle/🐍️.py" --check || fail=1
if [ "$1" = "--long" ]; then (cd "$W/📏️file-budgets/🧪️tests" && SEMIO_TEST_LEVEL=long nice bun test --timeout 3600000 -t "Every declared file" ./🟦️.ts 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" ) || fail=1; fi
exit $fail
