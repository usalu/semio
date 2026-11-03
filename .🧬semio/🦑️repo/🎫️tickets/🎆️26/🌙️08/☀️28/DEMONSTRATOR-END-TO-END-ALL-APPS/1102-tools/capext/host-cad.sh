#!/bin/bash
source "$(dirname "$0")/lib.sh"; cd "$ROOT" || exit 1
rm -f "$G/caphost-cad-h3.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" --no-fail-fast -p semio-s-artifact-cad-cad --test cad_editor_unit -- contributions > "$G/caphost-cad-h3.txt" 2>&1
echo $? > "$G/caphost-cad-h3.rc"
