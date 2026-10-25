#!/bin/sh
# live proof A: one component build (dictionary) through the component driver (counted compile units)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo --tool=semio exec --projects="directory:✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🧩️component/📦️packages/🦀️rust" -- repo /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts component --manifest "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🧩️component/📦️packages/🦀️rust/Cargo.toml" 2>&1
echo "rc=$?"
