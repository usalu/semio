#!/usr/bin/env bash
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\3dargs' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cargo test -j 2 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --lib every_mutation_action_declares_args 2>&1 | tee "$(dirname "$0")/🗑️generated/fem3d-action-args.txt"
