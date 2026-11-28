#!/bin/bash
# usage: run.sh <variant> <debug> [split]
V=$1; D=$2; SP=$3
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H90
cd $S/meas; export RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR=$S/t-$V CARGO_PROFILE_DEV_OPT_LEVEL=0 CARGO_PROFILE_DEV_CODEGEN_UNITS=256 CARGO_PROFILE_DEV_INCREMENTAL=true CARGO_PROFILE_DEV_DEBUG=$D
[ -n "$SP" ] && export CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=$SP
TC=~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin; export PATH=$TC:$PATH RUSTC=$TC/rustc
rm -rf $CARGO_TARGET_DIR
/usr/bin/time -l nice -n 10 $TC/cargo test --no-run --offline 2>&1 | grep -E "Finished|real|maximum resident|Executable" > $S/$V.cold.txt 2>&1
touch tests/big.rs
echo "// $V" >> tests/big.rs
/usr/bin/time -l nice -n 10 $TC/cargo rustc --offline --test big -- -Z time-passes 2>&1 | grep -E "link_binary|total|real|maximum resident|Finished|LLVM_passes|codegen_crate|^time:.*(codegen|link)" > $S/$V.warm.txt
sed -i '' '$d' tests/big.rs
echo "size_test_target_MB $(du -sm $CARGO_TARGET_DIR | cut -f1)" >> $S/$V.warm.txt
echo "size_test_bin_KB $(ls -l $CARGO_TARGET_DIR/debug/deps/big-* 2>/dev/null | grep -v '\.d$' | grep -v '\.dSYM' | awk '{print $5/1024}' | head -1)" >> $S/$V.warm.txt
