#!/bin/sh
# 🧪️ Admitted native cargo of the sourcing ticket: `native-cargo.sh` under `repo cargo`, log under 🗑️generated.
# Usage: typed-refusal-wasm-cargo.sh <label> <workspace-dir-relative-to-root> <cargo args...>
set -eu
exec "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)/native-cargo.sh" "$@"
