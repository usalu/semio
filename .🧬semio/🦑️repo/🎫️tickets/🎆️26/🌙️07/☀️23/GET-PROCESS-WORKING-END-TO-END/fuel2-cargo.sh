#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-fuel
mkdir -p $N
export PATH="$HOME/.bun/bin:$PATH"; export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
avail_mb(){ vm_stat | awk '/page size of/{ps=$8} /Pages free/{f=$3} /Pages inactive/{i=$3} END{gsub(/\./,"",f);gsub(/\./,"",i);print int((f+i)*ps/1048576)}'; }
while (( $(avail_mb) < 4096 )); do sleep 30; done
cd "$1" || exit 1
L=$2; shift 2
cargo "$@" > /private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/fuel-$L.txt 2>&1; echo exit $? >> /private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/fuel-$L.txt
