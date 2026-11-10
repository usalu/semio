#!/usr/bin/env bash
# 🧲️ The gated validation of the handle drag wiring: the React host typecheck, then the world3d scene Rust tests (marker axis codec and fixtures).
T="$(cd "$(dirname "$0")/.." && pwd)"
repo scheduler run --key=@semio-tech/framework-os-dev:typecheck -- repo run @semio-tech/framework-os-dev:typecheck --output=static > "$T/🗑️generated/host-runtime/handle-drag-typecheck-2.txt" 2>&1
repo scheduler run --key=@semio-tech/ui-scene-rs:test -- repo run @semio-tech/ui-scene-rs:test --output=static > "$T/🗑️generated/host-runtime/handle-drag-scene-test-2.txt" 2>&1
