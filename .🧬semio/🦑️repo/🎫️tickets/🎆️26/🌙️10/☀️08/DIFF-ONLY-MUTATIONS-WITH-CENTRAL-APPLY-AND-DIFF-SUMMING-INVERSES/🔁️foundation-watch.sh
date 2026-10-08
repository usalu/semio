#!/usr/bin/env bash
# 🔁️ Foundation watch: every 10 min checks the framework crates every plugin depends on (native + wasm32-wasip2) through the
# gate and publishes `GREEN <time>` or `RED <time>` + the first 12 error locations to `🗑️generated/coord/foundation.status`.
T="$(cd "$(dirname "$0")" && pwd)"; out="$T/🗑️generated/coord"; mkdir -p "$out"
cd /Users/ueli/Documents/semio
while :; do
  log="$out/foundation.log"
  "$T/🚦️gate.sh" foundation -- cargo check -p semio-framework-value -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --message-format=short > "$log" 2>&1
  native=$?
  "$T/🚦️gate.sh" foundation -- cargo check -p semio-framework-value -p semio-framework-replication -p semio-framework-plugin --target wasm32-wasip2 --message-format=short >> "$log" 2>&1
  wasm=$?
  if [ $native -eq 0 ] && [ $wasm -eq 0 ]; then echo "GREEN $(date +%T)" > "$out/foundation.status.tmp"
  else { echo "RED $(date +%T) native=$native wasm=$wasm"; grep -E "^\S+\.rs:[0-9]+:[0-9]+: error" "$log" | head -12; } > "$out/foundation.status.tmp"; fi
  mv "$out/foundation.status.tmp" "$out/foundation.status"
  sleep 600
done
