#!/bin/sh
set -eu
mount -t overlay overlay -o "lowerdir=/tmp/build,upperdir=$1/upper,workdir=$1/work" /tmp/build
mount --bind /home/deck/semio /tmp/input
mount -o remount,bind,ro /tmp/input
touch /tmp/build/t/debug/.cargo-lock
export CARGO_TARGET_DIR=/tmp/build/t CARGO_BUILD_BUILD_DIR=/tmp/build/b
export CARGO_HOME=/home/deck/semio/.cache/b/r/m/cargo/home
export RUSTUP_HOME=/home/deck/semio/.cache/b/t/m/r/699c2d5d27a9c904
export PATH="$RUSTUP_HOME/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin:$PATH"
export RUSTC="$RUSTUP_HOME/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
export RUSTC_WRAPPER= RUST_MIN_STACK=67108864
export CARGO_ENCODED_RUSTFLAGS="$(printf '%s\037%s\037%s\037%s\037%s\037%s\037%s' -Z threads=0 -C linker-features=+lld -C link-self-contained=+linker -Z)unstable-options"
cd /home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf
exec cargo test --offline --locked -p semio-s-artifact-stdio-pdf-lexer --lib --profile prototype --config 'profile.prototype.inherits="dev"' --config 'profile.prototype.opt-level=0' --config 'profile.prototype.debug=1' --config 'profile.prototype.codegen-units=256' --config 'profile.prototype.incremental=true' --config 'profile.prototype.lto="off"' --config 'profile.prototype.debug-assertions=true' --config 'profile.prototype.overflow-checks=true' --config 'profile.prototype.strip="none"' --config 'profile.prototype.trim-paths="object"' --config 'profile.prototype.build-override.opt-level=0' --config 'profile.prototype.build-override.debug=0' --config 'profile.prototype.build-override.codegen-units=256' --config 'profile.prototype.build-override.incremental=false' --config 'build.build-dir="/tmp/build/b"' --jobs 2
