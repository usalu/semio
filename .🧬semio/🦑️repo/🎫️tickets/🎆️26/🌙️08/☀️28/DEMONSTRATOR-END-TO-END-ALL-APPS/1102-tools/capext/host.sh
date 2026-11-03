#!/bin/bash
# usage: host.sh <tag>   native host-confirmation tests (pane install, built-ins win) run one at a time
source "$(dirname "$0")/lib.sh"
tag=$1; cd "$ROOT" || exit 1
rm -f "$G/caphost-run-$tag.rc"
run(){ n=$1; shift; rm -f "$G/caphost-$n-$tag.rc"; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$@" > "$G/caphost-$n-$tag.txt" 2>&1; echo $? > "$G/caphost-$n-$tag.rc"; }
run process3d "$ROOT/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" --no-fail-fast -p semio-s-artifact-process-process3d-editor --test process3d_editor_unit -- contributions_and_examples
run cad "$ROOT/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" --no-fail-fast -p semio-s-artifact-cad-cad-editor --lib -- contributions
run os "$ROOT/Cargo.toml" --no-fail-fast -p semio-framework-os --lib -- extension_package extension_capabilities
echo done > "$G/caphost-run-$tag.rc"
