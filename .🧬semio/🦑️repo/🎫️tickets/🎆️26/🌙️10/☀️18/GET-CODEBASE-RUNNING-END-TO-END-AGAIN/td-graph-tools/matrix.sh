#!/bin/sh
# 📊 [td-graph] runs every file of a list bare from <root> in the verify env (no Nx namespace, no parent envelope, short in-repo artifacts): sh matrix.sh <root> <list> -> one "P/F file" line each
ROOT="$(cd "${1:?root}" && pwd -P)"; T="$(cd "$(dirname "$0")" && pwd)"; . "$T/verify.common.sh"
while IFS= read -r f; do [ -n "$f" ] && law "x/x" "$f" | head -1 | sed 's/^FAIL  got //; s/ want x\/x//'; done < "${2:?list}"
