#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
B=$(find "$here"/🗑️generated/native-target/lane-0/b/debug/build/semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests -type f -perm +111 -name 'semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests-*' ! -name '*.*' -print0 | xargs -0 ls -t | head -1)
log="$here/🗑️generated/$1.log"; shift
cd "$here/../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf" || exit 1
RUST_MIN_STACK=67108864 "$B" --test-threads=1 --nocapture "$@" > "$log" 2>&1
