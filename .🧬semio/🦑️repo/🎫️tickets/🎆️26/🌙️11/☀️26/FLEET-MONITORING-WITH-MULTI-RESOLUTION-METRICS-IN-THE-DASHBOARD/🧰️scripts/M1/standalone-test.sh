#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/m1; cd $S
TC=/Users/ueli/Documents/semio/.cache/b/t/m/r/9171f2f44c79a380/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
export PATH=$TC:$PATH CARGO_HOME=/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=$S/target RUSTC_WRAPPER=
cargo test --test charts -- --nocapture --test-threads=1 "$@" > run.txt 2>&1
grep -n "^error" -A14 run.txt | head -80
grep -n "panicked" -A6 run.txt | cut -c1-300 | head -60
grep "^\[DEBUG\]\|^test \|test result" run.txt | cut -c1-220
