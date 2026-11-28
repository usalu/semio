#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"
R="$T/../../../../../../.."
L=/private/tmp/claude-501/-Users-ueli-Documents-semio/9ea38fee-6468-41de-903a-f7fec4948a18/scratchpad
run() {
  name="$1"; dir="$2"; shift 2
  (cd "$dir" && repo capacity run --memory="${MEM:-2300}" --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- "$@" > "$L/verify-$name.log" 2>&1)
  echo "exit $?" >> "$L/verify-$name.log"
}
P="$R/✏️s/🔌️plugins"
run process3d "$P/🏭️process/🗿️artifacts/🧊️process3d" check --offline --locked -p semio-s-artifact-process-process3d-host
run playground "$P/🎪️demonstrator/🗿️artifacts/🎪️playground" check --offline --locked -p semio-s-artifact-demonstrator-playground-editor
run curation "$P/🪵️sourcing/🗿️artifacts/🗂️curation" check --offline --locked -p semio-s-artifact-sourcing-curation
run playbook "$P/📖️playbook/🗿️artifacts/📖️playbook" check --offline --locked -p semio-s-artifact-playbook-playbook-v1
run coldreplay "$R" test --offline --locked -p semio-os-store-paged-one-item --lib
