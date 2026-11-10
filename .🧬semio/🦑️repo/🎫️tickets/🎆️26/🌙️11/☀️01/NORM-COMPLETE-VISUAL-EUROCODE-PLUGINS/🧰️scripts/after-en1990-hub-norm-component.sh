#!/usr/bin/env bash
# 🧩 Waits for the en1990 build to exit 0, then queues the norm hub component (semio_hub_norm_component.wasm) through the scheduler.
T="$(cd "$(dirname "$0")/.." && pwd)"
until grep -q '^exit ' "$T/🗑️generated/en1990/1.txt" 2>/dev/null; do sleep 60; done
grep -q '^exit 0' "$T/🗑️generated/en1990/1.txt" || { echo "en1990 build not green; hub component not queued" > "$T/🗑️generated/host-runtime/hub-component.txt"; exit 1; }
exec repo scheduler run --key=@semio-tech/os-hub:trusted-catalog-bootstrap -- repo run @semio-tech/os-hub:trusted-catalog-bootstrap --output=static --packages norm > "$T/🗑️generated/host-runtime/hub-component.txt" 2>&1
