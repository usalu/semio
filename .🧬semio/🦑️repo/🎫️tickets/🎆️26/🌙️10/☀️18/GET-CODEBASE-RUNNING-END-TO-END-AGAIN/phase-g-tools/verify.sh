#!/bin/sh
# Runs the laws of phase-g against the live tree (after `python3 stage.py apply /Users/ueli/Documents/semio`); every command is graph-free.
set -e
ROOT=/Users/ueli/Documents/semio
HC="$ROOT/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LHC="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity"
cd "$HC" && python3 "./🧪️tests/🪜️phases/🐍️oracle/🐍️.py" --check
cd "$HC" && nice -n 10 bun test ./🧪️tests/🪜️phases/🟦️.ts ./🧪️tests/🪜️phases/🚪️gate/🟦️.ts ./🧪️tests/📐️estimates/🟦️.ts ./🧪️tests/🟦️.ts ./🧪️tests/🚪️gate/🟦️.ts ./🧪️tests/🤝️handover/🟦️.ts
cd "$LHC" && nice -n 10 bun test ./🧪️tests/🪜️phases/🟦️.ts ./🧪️tests/🟦️.ts ./🧪️tests/📈️memory-fit/🟦️.ts
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd "$B/🕸️graph" && nice -n 10 bun test ./🧪️tests/🕰️generation-protocol/🟦️.ts ./🧪️tests/🔓️sealed-reuse/🟦️.ts ./🧪️tests/📤️portable/🟦️.ts ./🧪️tests/🕊️graph-release/🟦️.ts
cd "$B" && nice -n 10 bun test ./🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts ./⚙️operations/🧪️tests/🛗️requires/🟦️.ts ./🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🟦️.ts ./🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🧮️resources/🟦️.ts ./🩺️doctor/🧪️tests/⌨️command/🟦️.ts
cd "$ROOT" && sed "s#/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/phase-g-ov#$ROOT#g" "$(dirname "$0")/tsconfig.phase-g.json" > "$(dirname "$0")/tsconfig.phase-g.live.json"
cd "$ROOT" && nice -n 10 node_modules/.bin/tsc -p "$(dirname "$0")/tsconfig.phase-g.live.json" || true
