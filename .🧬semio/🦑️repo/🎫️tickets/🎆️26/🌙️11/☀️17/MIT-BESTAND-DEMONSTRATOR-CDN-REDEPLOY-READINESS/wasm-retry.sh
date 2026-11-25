#!/usr/bin/env bash
# Repeats each native-wasm.sh target until it publishes, because peers edit shared sources during a read. Usage: wasm-retry.sh <attempts> <target>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; attempts="$1"; shift; failed=0
for target in "$@"; do
  done=0
  for ((i = 1; i <= attempts; i++)); do
    if MEM="${MEM:-1500}" bash "$T/native-wasm.sh" "$target" > "$T/🗑️generated/retry-$target.out" 2>&1; then echo "OK $target attempt $i"; done=1; break; fi
    echo "RETRY $target attempt $i: $(grep -hoE 'RepoError: .{0,200}|kept editing .{0,160}|^error.{0,160}' "$T/🗑️generated/native-$target.log" | tail -1)"
  done
  [ "$done" = 1 ] || { echo "GAVEUP $target"; failed=1; }
done
exit "$failed"
