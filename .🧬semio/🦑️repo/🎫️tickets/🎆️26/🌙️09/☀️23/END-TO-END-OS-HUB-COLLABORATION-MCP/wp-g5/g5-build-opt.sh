#!/bin/bash
# g5: measure-only build of semio-os-mcp with wasmtime/cranelift at opt-level 3 (private target dir, shared build-dir).
cd /Users/ueli/Documents/semio
ARGS=()
for p in wasmtime wasmtime-environ wasmtime-internal-core wasmtime-internal-cranelift wasmtime-internal-fiber wasmtime-internal-unwinder wasmtime-internal-cache wasmtime-internal-component-util wasmtime-internal-jit-debug wasmtime-internal-jit-icache-coherence wasmtime-wasi wasmtime-wasi-io cranelift-codegen cranelift-frontend cranelift-entity cranelift-bforest cranelift-bitset cranelift-control cranelift-native cranelift-codegen-shared cranelift-assembler-x64 regalloc2 wasmparser pulley-interpreter gimli object wit-parser; do ARGS+=(--config "profile.dev.package.$p.opt-level=3"); done
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g5/target-opt cargo build "${ARGS[@]}" -p semio-framework-os-mcp --bin semio-os-mcp
