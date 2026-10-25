#!/bin/sh
export SEMIO_SCRIPT_STALL_MS=14400000
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp/📦️packages/🦀️rust" || exit 1
exec nice -n 10 bun ./📜️script.ts wasm
