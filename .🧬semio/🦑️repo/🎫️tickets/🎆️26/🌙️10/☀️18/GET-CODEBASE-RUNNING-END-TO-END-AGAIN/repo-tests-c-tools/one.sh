#!/bin/sh
# usage: one.sh <test file> [extra bun test args after --file]  -> log in 🗑️generated/repo-tests-c/one/
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/repo-tests-c/one"
f="$1"; shift
RUSTUP_TOOLCHAIN=nightly-2026-07-20 nice -n 10 repo "$PWD/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts" --file="$f" "$@" 2>&1
