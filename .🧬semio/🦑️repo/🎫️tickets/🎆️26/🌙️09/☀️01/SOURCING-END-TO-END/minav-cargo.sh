#!/bin/sh
# 🧪️ Admitted native cargo of the sourcing verification: `native-cargo.sh` under `repo cargo`, log under 🗑️generated/verify.
# Usage: minav-cargo.sh <label> <workspace-dir-relative-to-root> <cargo args...>
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
OUT="$here/🗑️generated/verify" exec "$here/native-cargo.sh" "$@"
