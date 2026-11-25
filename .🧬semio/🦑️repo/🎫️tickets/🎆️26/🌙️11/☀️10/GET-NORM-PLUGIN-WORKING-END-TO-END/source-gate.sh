#!/usr/bin/env bash
# ⏳️ Waits until the framework sources admit a build on this machine: no mount of a ticket file that does not exist here, stable for one minute.
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
broken() {
  grep -rhoE --include='*.rs' '#\[path="/[^"]*generated/[^"]*\.rs"\]' "$ROOT/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned" "$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store" 2>/dev/null \
    | sed -E 's/^#\[path="(.*)"\]$/\1/' | while read -r path; do [ -f "$path" ] || { echo broken; break; }; done
}
calm=0
while [ "$calm" -lt 4 ]; do
  if [ -n "$(broken)" ]; then calm=0; else calm=$((calm+1)); fi
  [ "$calm" -lt 4 ] && sleep 15
done
