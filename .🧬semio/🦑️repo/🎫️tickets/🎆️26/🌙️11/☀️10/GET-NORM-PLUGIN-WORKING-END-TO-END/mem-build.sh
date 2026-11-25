#!/usr/bin/env bash
# usage: mem-build.sh <manifest-dir> <log-name> <crate>
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
D="$1"; L="$T/🗑️generated/$2.log"; C="$3"
cd "$D" || exit 1
for try in 1 2 3; do
  python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- build -p "$C" --target wasm32-wasip2 --offline > "$L" 2>&1
  rc=$?; echo "EXIT $rc try $try" >> "$L"
  [ $rc -eq 137 ] || grep -q "SIGKILL\|warden\|SIGTERM" "$L" || break
  sleep 30
done
