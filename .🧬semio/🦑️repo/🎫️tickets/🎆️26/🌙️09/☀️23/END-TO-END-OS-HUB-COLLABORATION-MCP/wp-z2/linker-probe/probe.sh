#!/usr/bin/env bash
# Z2 linker probe: which self-contained linker flags the pinned nightly accepts on this Linux arch.
set -u
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null && apt-get install -y -qq --no-install-recommends curl ca-certificates build-essential mold binutils >/dev/null
curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh -s -- -y -q --profile minimal --default-toolchain nightly-2026-07-07 >/dev/null 2>&1
. "$HOME/.cargo/env"
rustc -vV
sysroot="$(rustc --print sysroot)"; host="$(rustc -vV | sed -n 's/^host: //p')"
ls "$sysroot/lib/rustlib/$host/bin" "$sysroot/lib/rustlib/$host/bin/gcc-ld" 2>&1
mkdir -p /probe && cd /probe && printf 'fn main(){println!("hi");}\n' > main.rs
try() { local name="$1"; shift; rm -f out; if rustc -o out "$@" main.rs 2>err.txt && ./out >/dev/null; then echo "OK   $name :: $(readelf -p .comment out 2>/dev/null | grep -Eo '(LLD|mold|GNU ld|GCC: \([^)]*\))[^]]*' | tr '\n' ' ')"; else echo "FAIL $name :: $(head -c 400 err.txt | tr '\n' ' ')"; fi; }
try default
try C-linker-features "-Clinker-features=+lld" "-Clink-self-contained=+linker" "-Zunstable-options"
try Z-linker-features "-Zlinker-features=+lld" "-Clink-self-contained=+linker"
try mold "-Clink-arg=-fuse-ld=mold"
echo PROBE-DONE
