#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
JACK="✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml"
FIX="✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/🧪️tests/🧫️fixtures/🔱️mini-tower/🔣️.json"
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- run --manifest-path "$JACK" -p semio-s-artifact-trinity-jack-shell -- "$FIX" "MATCH (a:Piece) RETURN a.name"
