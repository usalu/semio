#!/usr/bin/env bash
# 🧲️ The gated validation of the handle drag wiring: the React host typecheck, then the world3d scene Rust tests (marker axis codec and fixtures).
T="$(cd "$(dirname "$0")/.." && pwd)"
bash "$T/🧰️scripts/build-slot.sh" '@semio-tech/framework-os-dev:typecheck' "$T/🗑️generated/host-runtime/handle-drag-typecheck-2.txt"
bash "$T/🧰️scripts/build-slot.sh" '@semio-tech/ui-scene-rs:test' "$T/🗑️generated/host-runtime/handle-drag-scene-test-2.txt"
