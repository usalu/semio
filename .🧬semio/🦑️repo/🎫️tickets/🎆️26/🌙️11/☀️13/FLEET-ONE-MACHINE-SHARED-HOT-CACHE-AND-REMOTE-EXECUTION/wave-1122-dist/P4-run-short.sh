#!/bin/zsh
# usage: P3-run.sh <cacheRoot> <targetDir> <offload> <package> [extra env KEY=VAL via P2_ENV] ; builds one hub component package (release component profile, wasm32-wasip2)
set -u
W=/Users/ueli/Documents/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist"
CR=$1 TD=$2 OFF=$3 PKG=$4
RH=$W/.cache/b/t/m/r/9171f2f44c79a380
TC=nightly-2026-07-20-aarch64-apple-darwin
mkdir -p "$CR" "$TD/t" "$TD/b" "$TD/tmp"
SRC=${P2_SRC:-$W}
cd "$SRC"
BUN=$W/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
WRAP=$("$BUN" "$T/P1-seal.ts" "$CR" | tail -1)
if [ ! -f "$CR/offload-rustc.json" ]; then PATH="$RH/toolchains/$TC/bin:$PATH" RUSTUP_HOME=$RH RUSTUP_TOOLCHAIN=$TC python3 -c "import json,subprocess,time;print(json.dumps({'atMs':int(time.time()*1000),'inner':'[\"rustc\"]','text':subprocess.check_output(['rustc','-vV'],text=True)}))" > "$CR/offload-rustc.json"; fi
exec env -i HOME=$HOME USER=$USER LANG=en_US.UTF-8 \
 PATH="$RH/toolchains/$TC/bin:/usr/bin:/bin:/usr/sbin:/sbin" \
 TMPDIR="$TD/tmp" SDKROOT=${P2_SDKROOT:-/Library/Developer/CommandLineTools/SDKs/MacOSX27.2.sdk} \
 RUSTUP_HOME=$RH RUSTUP_TOOLCHAIN=$TC RUSTUP_AUTO_INSTALL=0 CARGO_HOME=${P4_CARGO_HOME:-$W/.cache/b/r/m/cargo/home} CARGO_NET_OFFLINE=true \
 RUSTC=rustc RUSTC_WRAPPER=$WRAP CARGO_ENCODED_RUSTFLAGS=$'-Z\x1fthreads=0\x1f-C\x1flink-arg=--max-memory=536870912\x1f-C\x1flink-arg=-zstack-size=8388608\x1f-Z\x1fthreads=0\x1f-C\x1flink-arg=--max-memory=536870912\x1f-C\x1flink-arg=-zstack-size=8388608' CARGO_TERM_COLOR=never CARGO_RESOLVER_FEATURE_UNIFICATION=selected CARGO_UNSTABLE_FEATURE_UNIFICATION=true \
 CARGO_TARGET_DIR="$TD/t" CARGO_BUILD_BUILD_DIR="$TD/b" \
 SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=release SEMIO_WORKSPACE_ROOT=$SRC SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NETWORK=192.168.178.0/24 SEMIO_FLEET_NFS_HOST=192.168.178.71 \
 SEMIO_UNIT_CACHE=on SEMIO_UNIT_CACHE_ROOT="$CR" SEMIO_UNIT_CACHE_WRAPPER=$WRAP \
 SEMIO_UNIT_CACHE_LANE=p4 SEMIO_UNIT_CACHE_LANE_ROOT="$TD" SEMIO_UNIT_CACHE_INNER='["rustc"]' SEMIO_UNIT_CACHE_SLOTS=${P2_SLOTS:-1} SEMIO_UNIT_CACHE_LIGHT_SLOTS=${P2_LIGHT:-0} \
 SEMIO_UNIT_CACHE_FOCUS=$PKG SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on \
 SEMIO_UNIT_CACHE_WORKSPACE_ROOT=$SRC \
 SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.70:6094 SEMIO_REMOTE_CACHE_INSTANCE=main SEMIO_REMOTE_CACHE_ACCESS=read-write SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS=30 SEMIO_REMOTE_CACHE_RETRIES=3 SEMIO_REMOTE_CACHE_GENERATION=10 SEMIO_REMOTE_CACHE_SIGNED=1 \
 ${OFF:+SEMIO_OFFLOAD=$OFF} SEMIO_REMOTE_CACHE_UPLOAD=${P4_UPLOAD:-true} \
 cargo build --locked -Z trim-paths --offline --manifest-path "$SRC/🌎️hub/Cargo.toml" -p "$PKG" --lib --target wasm32-wasip2 --profile release --keep-going --message-format=short -j${P2_JOBS:-4} \
 --config 'profile.release.opt-level="s"' \
 --config 'profile.release.debug=0' \
 --config 'profile.release.codegen-units=16' \
 --config 'profile.release.incremental=false' \
 --config 'profile.release.lto="thin"' \
 --config 'profile.release.debug-assertions=false' \
 --config 'profile.release.overflow-checks=false' \
 --config 'profile.release.strip="symbols"' \
 --config 'profile.release.trim-paths="object"' \
 --config 'profile.release.build-override.opt-level=0' \
 --config 'profile.release.build-override.debug=0' \
 --config 'profile.release.build-override.codegen-units=256' \
 --config 'profile.release.build-override.incremental=false' \
 --config "build.build-dir=\"$TD/b\""
