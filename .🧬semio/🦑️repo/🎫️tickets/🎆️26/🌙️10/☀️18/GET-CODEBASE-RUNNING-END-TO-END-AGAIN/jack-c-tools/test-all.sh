#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
JACK="✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml"
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --message-format=short --manifest-path "$JACK" -p semio-s-artifact-trinity-jack-shell -p semio-s-artifact-trinity-jack-lsp
