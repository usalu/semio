#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"
R="$T/../../../../../../.."
L=/private/tmp/claude-501/-Users-ueli-Documents-semio/9ea38fee-6468-41de-903a-f7fec4948a18/scratchpad
run() {
  name="$1"; dir="$2"; shift 2
  (cd "$dir" && repo capacity run --memory="${MEM:-2300}" --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- "$@" > "$L/verify2-$name.log" 2>&1)
  echo "exit $?" >> "$L/verify2-$name.log"
}
run coldreplay "$R" test --offline --locked -p semio-os-store-paged-one-item --lib --features test-support
run curation "$R/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" check --offline --locked -p semio-s-artifact-sourcing-curation
