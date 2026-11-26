#!/bin/sh
# usage: run-freshness-law.sh  (the descriptor freshness law of `semio-plugin-extension` through the memory gate; log 🗑️generated/freshness-law.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
NATIVE_CARGO_DIR="$root" sh "$here/native-cargo.sh" test --manifest-path "$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/📦️packages/🦀️rust/Cargo.toml" --lib a_committed_descriptor_is_fresh_only > "$here/🗑️generated/freshness-law.log" 2>&1
echo "exit $?" > "$here/🗑️generated/freshness-law.summary"
