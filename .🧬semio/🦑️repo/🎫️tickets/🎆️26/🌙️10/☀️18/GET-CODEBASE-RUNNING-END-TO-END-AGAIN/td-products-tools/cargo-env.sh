#!/bin/sh
# usage: . cargo-env.sh   (sources env so plain cargo works in a mirror: real rustc of the pinned nightly, no repo wrapper, private CARGO_HOME copy, offline)
export CARGO_HOME="${TMPDIR%/}/sm/tp-cargo-home"
export RUSTC="$(rustup which rustc --toolchain nightly-2026-07-20)"
export RUSTC_WRAPPER=""
export CARGO_NET_OFFLINE=true
export RUSTUP_TOOLCHAIN=nightly-2026-07-20
