#!/bin/zsh
# 🧱️ LD wasm32 compile-atomic check of the guest-linked crates LD touched (run through the fleet wasm mutex).
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
for target_args in "--target wasm32-wasip2" "--features sync --target wasm32-unknown-unknown"; do
  echo "== kernel ${target_args}"
  cargo check -p semio-framework-os-kernel --lib ${=target_args} --message-format short 2>&1 | /usr/bin/grep -E "^(error|warning: unused)|error\[|Finished|could not compile" ; echo "rc=${pipestatus[1]}"
done
echo "== replication --target wasm32-wasip2"
cargo check -p semio-framework-replication --lib --target wasm32-wasip2 --message-format short 2>&1 | /usr/bin/grep -E "^error|error\[|Finished|could not compile"; echo "rc=${pipestatus[1]}"
