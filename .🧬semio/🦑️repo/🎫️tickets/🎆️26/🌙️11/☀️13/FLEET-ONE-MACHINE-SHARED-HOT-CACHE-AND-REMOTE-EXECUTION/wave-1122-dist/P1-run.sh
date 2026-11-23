#!/bin/zsh
# usage: P1-run.sh <cacheRoot> <targetDir> <offload> [cargo args...]
set -u
W=/Users/ueli/Documents/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist"
CR=$1 TD=$2 OFF=$3; shift 3
RH=$W/.cache/b/t/m/r/9171f2f44c79a380
TC=nightly-2026-07-20-aarch64-apple-darwin
mkdir -p "$CR" "$TD/t" "$TD/b" "$TD/tmp"
cd "$T/proof"
BUN=$W/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
WRAP=$("$BUN" "$T/P1-seal.ts" "$CR" | tail -1)
if [ ! -f "$CR/offload-rustc.json" ]; then PATH="$RH/toolchains/$TC/bin:$PATH" RUSTUP_HOME=$RH RUSTUP_TOOLCHAIN=$TC python3 -c "import json,subprocess,time,sys;print(json.dumps({'atMs':int(time.time()*1000),'inner':'[\"rustc\"]','text':subprocess.check_output(['rustc','-vV'],text=True)}))" > "$CR/offload-rustc.json"; fi
exec env -i HOME=$HOME USER=$USER LANG=en_US.UTF-8 \
 PATH="$RH/toolchains/$TC/bin:/usr/bin:/bin:/usr/sbin:/sbin" \
 TMPDIR="$TD/tmp" SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX27.2.sdk \
 RUSTUP_HOME=$RH RUSTUP_TOOLCHAIN=$TC RUSTUP_AUTO_INSTALL=0 CARGO_HOME=$W/.cache/b/r/m/cargo/home CARGO_NET_OFFLINE=true \
 RUSTC=rustc RUSTC_WRAPPER=$WRAP \
 CARGO_TARGET_DIR="$TD/t" CARGO_BUILD_BUILD_DIR="$TD/b" \
 CARGO_ENCODED_RUSTFLAGS=$'-Z\x1fthreads=0' \
 SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=prototype SEMIO_WORKSPACE_ROOT=$W SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NETWORK=192.168.178.0/24 SEMIO_FLEET_NFS_HOST=192.168.178.71 \
 SEMIO_UNIT_CACHE=on SEMIO_UNIT_CACHE_ROOT="$CR" SEMIO_UNIT_CACHE_WRAPPER=$WRAP \
 SEMIO_UNIT_CACHE_LANE=p1 SEMIO_UNIT_CACHE_LANE_ROOT="$TD" SEMIO_UNIT_CACHE_INNER='["rustc"]' SEMIO_UNIT_CACHE_SLOTS=2 \
 SEMIO_UNIT_CACHE_FOCUS=p-top SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on SEMIO_UNIT_CACHE_TOOLCHAIN=${P1_TOOLCHAIN:-p1-default} \
 SEMIO_UNIT_CACHE_WORKSPACE_ROOT=$W \
 SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.70:6094 SEMIO_REMOTE_CACHE_INSTANCE=main SEMIO_REMOTE_CACHE_ACCESS=read-write SEMIO_REMOTE_CACHE_UPLOAD=true SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS=30 SEMIO_REMOTE_CACHE_RETRIES=3 SEMIO_REMOTE_CACHE_GENERATION=10 SEMIO_REMOTE_CACHE_SIGNED=1 \
 SEMIO_OFFLOAD="$OFF" \
 cargo "$@" -Z trim-paths --offline --manifest-path "$T/proof/Cargo.toml" -j2 --release \
 --config 'profile.release.opt-level=3' --config 'profile.release.debug=0' --config 'profile.release.codegen-units=16' --config 'profile.release.incremental=false' --config 'profile.release.lto=false' --config 'profile.release.debug-assertions=false' --config 'profile.release.overflow-checks=false' --config 'profile.release.strip="debuginfo"' --config 'profile.release.trim-paths="object"' \
 --config 'profile.release.build-override.opt-level=0' --config 'profile.release.build-override.debug=0' --config 'profile.release.build-override.codegen-units=256' --config 'profile.release.build-override.incremental=false' --config "build.build-dir=\"$TD/b\""
