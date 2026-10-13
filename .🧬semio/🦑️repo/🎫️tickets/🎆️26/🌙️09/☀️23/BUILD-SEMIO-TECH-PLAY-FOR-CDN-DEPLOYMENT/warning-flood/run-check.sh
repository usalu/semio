#!/bin/bash
# usage: run-check.sh <label> <manifest> <extra cargo args...>   -> $S/<label>.json / .err
cd /Users/ueli/Documents/semio || exit 1
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/0f187895-1699-4e68-8c53-7e6030677f5a/scratchpad
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/warn/target
L="$1"; M="$2"; shift 2
bash "$T" warn -- cargo check --manifest-path "$M" --keep-going --message-format json "$@" > "$S/$L.json" 2> "$S/$L.err"
echo "exit=$?" >> "$S/$L.err"
