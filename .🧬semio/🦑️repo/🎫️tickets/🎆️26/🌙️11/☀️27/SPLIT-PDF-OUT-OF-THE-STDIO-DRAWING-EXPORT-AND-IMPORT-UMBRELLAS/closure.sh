#!/usr/bin/env bash
# 🌳️ Local-crate closure of one package: closure.sh <manifest> <package> <out> [extra cargo-tree args]
set -euo pipefail
manifest="$1"; package="$2"; out="$3"; shift 3
SEMIO_LANGUAGE=en repo cargo --configuration=prototype --parallelism=2 --store=shared -- tree --manifest-path "$manifest" -p "$package" --prefix none --offline -f '{p}' "$@" \
  | sed 's/ (\*)$//; s/ (proc-macro)$//' | grep '^semio' | sort -u > "$out"
printf '%s local=%s pdf=%s\n' "$package" "$(wc -l < "$out" | tr -d ' ')" "$(grep -c '^semio-s-artifact-stdio-pdf' "$out" || true)"
