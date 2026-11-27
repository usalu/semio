#!/bin/zsh
# [DEBUG] sequential kernel wasm check, then test-support tests, under one guard slot
T=${0:A:h}; G="$T/🗑️generated/claude-test-support"
$T/claude-test-support-cargo.sh -p semio-os-kernel --target wasm32-wasip2 > $G/kernel-wasm.txt 2>&1; echo "kernel-wasm exit $?" >> $G/seq.txt
$T/claude-test-support-cargo.sh -p semio-os-store-test-support --tests > $G/ts-native.txt 2>&1; echo "ts-native exit $?" >> $G/seq.txt
