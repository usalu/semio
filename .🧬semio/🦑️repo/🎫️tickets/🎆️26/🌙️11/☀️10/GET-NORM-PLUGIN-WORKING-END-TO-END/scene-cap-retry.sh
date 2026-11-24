#!/usr/bin/env bash
# 🧪️ Retries a repo cargo test while another editor's change keeps a foreign crate over its source boundary.
# Usage: scene-cap-retry.sh <log> <package> [cargo test args...]
T="$(cd "$(dirname "$0")" && pwd)"
log="$1"; pkg="$2"; shift 2
dir="$(cd "$T/../../../../../../.." && pwd)"
cd "$dir/${CRATE_DIR:?}"
for attempt in $(seq 1 40); do
  repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --offline "$@" > "$log" 2>&1
  code=$?
  grep -qE "exceeds its source boundary|could not compile .semio-(framework|plugin)" "$log" || break
  sleep 120
done
echo "exit $code" >> "$log"
