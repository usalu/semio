#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
NATIVE_CARGO_DIR="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx" sh "$here/native-cargo.sh" check -p semio-s-artifact-stdio-xlsx --lib --tests --message-format short > "$here/🗑️generated/mem-check-xlsx.log" 2>&1 || true
NATIVE_CARGO_DIR="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml" sh "$here/native-cargo.sh" check -p semio-s-artifact-stdio-xml --lib --tests --message-format short > "$here/🗑️generated/mem-check-xml.log" 2>&1 || true
