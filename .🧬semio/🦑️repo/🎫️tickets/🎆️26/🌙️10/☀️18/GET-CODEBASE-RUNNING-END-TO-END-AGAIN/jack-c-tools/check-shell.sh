#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
SH="✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml"
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --keep-going --all-targets --message-format=short --manifest-path "$SH"
