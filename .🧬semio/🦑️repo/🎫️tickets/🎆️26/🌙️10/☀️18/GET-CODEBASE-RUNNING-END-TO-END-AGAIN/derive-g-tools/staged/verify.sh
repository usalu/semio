#!/bin/zsh
# [derive-g] verifies the batch on the tree: every suite named with its expected pass count (graph-free, single files, `bun test`); exit 1 on any difference.
cd /Users/ueli/Documents/semio
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; G="$B/🕸️graph"; I="$G/📥️inference"; GN="$G/🕰️generations"
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/tmp/derive-g-art}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
fail=0
run(){ local dir="$1" want="$2"; got=$(cd "$dir" && nice bun test --timeout 120000 ./🟦️.ts 2>&1 | grep -E "^ *[0-9]+ pass$" | tr -d ' ' ); if [ "$got" = "${want}pass" ]; then echo "OK   $want  ${dir##*/}"; else echo "FAIL want $want got '$got'  $dir"; fail=1; fi; }
run "$I/🧪️tests/🧷️closure-facts" 22
run "$I/🧪️tests/✂️derived-walks" 4
run "$I/🧭️script-inputs/🧪️tests" 4
run "$I/🧭️script-inputs/🔀️router/🧪️tests" 25
run "$I/🧪️tests/🔌️providers" 4
run "$I/🧪️tests/🔗️late-closures" 4
run "$I/🧪️tests/🚧️withheld-rows" 6
run "$I/🕸️dependency-edges/🧪️tests" 4
run "$I/🕸️dependency-edges/🧪️tests/🪣️re-edge-chunks" 3
run "$I/🕸️dependency-edges/🧪️tests/🌊️streaming-edges" 3
run "$G/🎯️test-owners/🧪️tests" 67
run "$G/🚪️admission/🧪️tests/🗂️relative-owners" 6
run "$B/👁️observation/🧪️tests/🔂️validation-round" 4
run "$GN/🧪️tests/🪪️row-keys" 2
run "$GN/🧪️tests/🌱️patch-growth" 2
run "$GN/🧪️tests" 27
run "$GN/🔌️load/🧪️tests" 68
run "$GN/✂️cutoff/🧪️tests" 138
run "$G/🧪️tests/🕰️generation-protocol" 47
run "$G/🧪️tests/♻️tagged-retention" 74
run "$G/🧪️tests/🔗️edge-refresh" 7
run "$G/🧪️tests/🕊️graph-release" 18
exit $fail
