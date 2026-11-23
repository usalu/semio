#!/bin/bash
fam="$1"; dir="$2"; T="$(cd "$(dirname "$0")" && pwd)"; log="$T/🗑️generated/$fam-assert.log"
mkdir -p "$T/🗑️generated"; : > "$log"
cd "$dir" || exit 1
for try in $(seq 1 30); do
  repo cargo --store=shared --configuration=prototype -- test -p "semio-s-artifact-norm-$fam-examples" --offline -- --nocapture >> "$log" 2>&1
  grep -qE "^test result" "$log" && break
  echo "[retry $try]" >> "$log"; sleep 90
done
echo "[run-finished]" >> "$log"
