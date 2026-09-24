#!/usr/bin/env bash
# Z2: capture the os-hub link command format inside the proof volume (toolchain from the home volume).
set -u
export PATH="/root/.bun/bin:/root/.cargo/bin:/root/.local/bin:$PATH" CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 CARGO_TERM_COLOR=never
apt-get update -qq >/dev/null </dev/null && apt-get install -y -qq --no-install-recommends build-essential ca-certificates mold binutils >/dev/null </dev/null
cd /src
cargo rustc -p semio-hub --bin os-hub -- --print link-args -C save-temps --cfg "z2_linkargs_$(date +%s)" >/tmp/la.txt 2>&1
echo "exit=$? lines=$(wc -l </tmp/la.txt)"
line="$(grep -E '"-fuse-ld=lld"' /tmp/la.txt | tail -1)"; echo "len=${#line}"; echo "HEAD: ${line:0:400}"; echo "TAIL: ${line: -400}"
cd /src && start=$(date +%s%N) && eval "$line" > /tmp/l.log 2>&1; echo "relink status=$? ms=$(( ($(date +%s%N)-start)/1000000 ))"; head -c 1500 /tmp/l.log
grep -c '"cc"' /tmp/la.txt
tail -5 /tmp/la.txt | cut -c1-300
