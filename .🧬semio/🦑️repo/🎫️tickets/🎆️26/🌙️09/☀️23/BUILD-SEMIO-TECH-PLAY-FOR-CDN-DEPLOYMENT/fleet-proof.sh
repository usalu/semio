#!/bin/sh
# usage: fleet-proof.sh <a|b> <always|never> ; builds the cdylib and prints sha256
set -e
D="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-oBWDvy/f4929b72-7dd9-49d0-a049-49cc06207962/scratchpad/proof-$1"; cd "$D"; rm -rf target
export SEMIO_UNIT_CACHE=on SEMIO_OFFLOAD=$2 SEMIO_UNIT_CACHE_ROOT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/fleet-proof-cache" SEMIO_UNIT_CACHE_WORKSPACE_ROOT="/Users/ueli/Documents/semio" SEMIO_UNIT_CACHE_LANE=d SEMIO_UNIT_CACHE_LANE_ROOT="$D/target" SEMIO_UNIT_CACHE_INNER='["rustc"]' SEMIO_UNIT_CACHE_SLOTS=4 SEMIO_FLEET_ROOT="/Users/ueli/Documents/semio" SEMIO_WORKSPACE_ROOT="/Users/ueli/Documents/semio"
W="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/fleet-proof-wrapper.sh"
printf '#!/bin/sh\nexec "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun" "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts" "$@"\n' > "$W"; chmod +x "$W"
[ "$2" = never ] && export SEMIO_UNIT_CACHE=off
export RUSTUP_TOOLCHAIN=nightly-2026-07-20 RUSTC_WRAPPER="$W"
export CARGO_TARGET_DIR="$D/target"
cargo build --config "target.wasm32-unknown-unknown.rustflags=['-Z','threads=1','-Z','no-parallel-backend','--remap-path-prefix=$D=.','--remap-path-scope=all','-C','link-arg=--max-memory=536870912','-C','link-arg=-zstack-size=8388608']" --release --target wasm32-unknown-unknown 2>&1 | tail -15
shasum -a 256 target/wasm32-unknown-unknown/release/fleet_proof.wasm
