#!/usr/bin/env bash
# 🔁️ Re-runs turn-slice-laws.sh while a crate another agent is editing breaks the build (up to 10 tries, 3 minutes apart).
T="$(cd "$(dirname "$0")" && pwd)"
for i in $(seq 1 10); do
  bash "$T/turn-slice-laws.sh" "${1:-}"
  L="$T/🗑️generated/turn-slice-laws${1:-}.log"
  grep -q "^rc-turn=0" "$L" && grep -q "^rc-publication=0" "$L" && grep -q "^rc-patches=0" "$L" && grep -q "^rc-check=0" "$L" && break
  grep -q 'could not compile .semio-framework-value-native-receiver-protocol\|debug-ledger\|STATUS_ACCESS_VIOLATION\|failed to write\|link.exe' "$L" || break
  sleep 180
done
