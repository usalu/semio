#!/bin/zsh
# 🧹️ disposer-teardown-ownership lane driver: build the FEATURED generation3d `--lib` binary and run
# the teardown ownership laws plus the whole `unit_tests::` module, single threaded (the flow-eval
# neuron kernel cache is process wide). Argument 1 is the log tag.
TAG=${1:-run}
cd /Users/ueli/Documents/semio || exit 1
OUT=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/disposer-teardown"
mkdir -p "$OUT"
echo "=== build start $(date) ===" > "$OUT/$TAG-build.txt"
CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run >> "$OUT/$TAG-build.txt" 2>&1
echo "build-exit=$? $(date)" >> "$OUT/$TAG-build.txt"
echo "=== laws start $(date) ===" > "$OUT/$TAG-laws.txt"
CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1 pays_its_own_flow_frontier >> "$OUT/$TAG-laws.txt" 2>&1
echo "laws-exit=$? $(date)" >> "$OUT/$TAG-laws.txt"
echo "=== suite start $(date) ===" > "$OUT/$TAG-suite.txt"
CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1 unit_tests:: >> "$OUT/$TAG-suite.txt" 2>&1
echo "suite-exit=$? $(date)" >> "$OUT/$TAG-suite.txt"
echo "=== done $(date) ===" >> "$OUT/$TAG-suite.txt"
