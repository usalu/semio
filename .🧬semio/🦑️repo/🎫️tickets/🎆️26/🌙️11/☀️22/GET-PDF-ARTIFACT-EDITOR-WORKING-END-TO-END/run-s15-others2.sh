#!/bin/sh
# usage: run-s15-others2.sh  (slice 15: gated cargo check of the PDF consumers of the layout, semio and BIM workspaces only; logs in 🗑️generated)
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
G=🗑️generated
rm -f $G/s15-others2.done
NATIVE_CARGO_DIR="$root/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout" sh ./native-cargo.sh check -p semio-s-artifact-layout-layout-io --tests --message-format short > $G/s15-o2-layout.log 2>&1; echo "exit $?" > $G/s15-o2-layout.done
echo done > $G/s15-others2.done
