#!/usr/bin/env bash
# 🔁️ Reruns the fem component and activation steps until they pass, each attempt after the plugin and value trees were quiet for two minutes: `bash rebuild-until-green.sh [attempts]`.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
W=/c/semio/🧰️framework
for attempt in $(seq 1 "${1:-8}"); do
  for i in $(seq 1 30); do [ -z "$(find "$W/🛍️products/💻️os/🔨️modules/🔌️plugin" "$W/🔨️modules/🌱️value" -name '*.rs' -newermt '-120 seconds' 2>/dev/null | head -1)" ] && break; sleep 30; done
  echo "[$(date +%H:%M:%S)] attempt $attempt"
  bash "$T/native-chain.sh" 13 && { echo "[$(date +%H:%M:%S)] GREEN"; exit 0; }
done
exit 1
