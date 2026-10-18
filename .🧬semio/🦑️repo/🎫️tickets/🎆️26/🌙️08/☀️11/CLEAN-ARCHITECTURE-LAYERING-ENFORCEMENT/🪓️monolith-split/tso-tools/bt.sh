#!/bin/zsh
# usage: bt.sh <path-relative-to-os> ... -> runs bun test per file with the artifact env, prints the tail
OS=/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tso-artifacts
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"; export SEMIO_TICKET_DIR="$SEMIO_TEST_ARTIFACT_DIR"
export REPO_ROOT=/Users/ueli/Documents/semio; cd /Users/ueli/Documents/semio
for f in "$@"; do echo "##### $f"; perl -e 'use POSIX; POSIX::setsid(); alarm 240; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$OS/$f" 2>&1 | /usr/bin/grep -v '^$' | tail -${TSO_TAIL:-14} | cut -c1-${TSO_COLS:-260}; done
