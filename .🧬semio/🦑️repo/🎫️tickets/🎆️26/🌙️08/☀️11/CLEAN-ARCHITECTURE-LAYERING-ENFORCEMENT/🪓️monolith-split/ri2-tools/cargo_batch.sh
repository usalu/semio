#!/bin/bash
# usage: cargo_batch.sh <log-tag> <extra cargo args...> -- <crate suffix>...   (fleet mutex per WAVE5-BRIEF, one launcher call)
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
TAG="$1"; shift
EXTRA=(); while [ "$1" != "--" ]; do EXTRA+=("$1"); shift; done; shift
PK=(); for c in "$@"; do PK+=(-p "semio-framework-os-renderer-wgpu-$c"); done
cd /Users/ueli/Documents/semio
until mkdir "$G/cargo-mutex.lock" 2>/dev/null; do sleep 60; done
echo "ri2 $$ $(date)" > "$G/cargo-mutex.lock/owner"
SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check "${PK[@]}" "${EXTRA[@]}" --keep-going --message-format=short > "$G/ri2-cargo-$TAG.log" 2>&1
echo "exit $?" >> "$G/ri2-cargo-$TAG.log"
rm -f "$G/cargo-mutex.lock/owner"; rmdir "$G/cargo-mutex.lock"
