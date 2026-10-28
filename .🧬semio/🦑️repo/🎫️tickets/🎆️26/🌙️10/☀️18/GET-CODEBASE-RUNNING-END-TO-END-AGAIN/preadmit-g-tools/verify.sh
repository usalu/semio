#!/bin/sh
# 🧪️ Every graph-free law of preadmit-g, run in a root (the live tree after `apply.py apply`, or the overlay). Usage: sh verify.sh [root]
set -e
ROOT=${1:-/Users/ueli/Documents/semio}
HC="$ROOT/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd "$HC/🧪️tests/🔼️upgrade/🐍️oracle" && python3 ./🐍️.py --check
cd "$HC/🧪️tests/🪜️phases/🐍️oracle" && python3 ./🐍️.py --check
cd "$HC" && nice -n 10 bun test ./🧪️tests/🔼️upgrade/🟦️.ts ./🧪️tests/🔼️upgrade/🚪️gate/🟦️.ts ./🧪️tests/🟦️.ts ./🧪️tests/🚪️gate/🟦️.ts ./🧪️tests/🤝️handover/🟦️.ts ./🧪️tests/📐️estimates/🟦️.ts ./🧪️tests/🪜️phases/🟦️.ts ./🧪️tests/🪜️phases/🚪️gate/🟦️.ts ./🧪️tests/🔮️sqlite/🟦️.ts ./🧪️tests/📡️sampling/🟦️.ts
cd "$LIB/🌡️host-capacity" && nice -n 10 bun test ./🧪️tests/🟦️.ts ./🧪️tests/🪜️phases/🟦️.ts ./🧪️tests/📈️memory-fit/🟦️.ts ./🧪️tests/🔼️upgrade/🟦️.ts ./🧪️tests/🧮️arguments/🟦️.ts ./🧪️tests/🔁️restart/🟦️.ts ./🧪️tests/🏊️farm/🟦️.ts ./🧪️tests/🪟️partial/🟦️.ts
cd "$LIB" && nice -n 10 bun test ./🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🟦️.ts ./🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🧮️resources/🟦️.ts ./🩺️doctor/🧪️tests/⌨️command/🟦️.ts
# The two failures of tests/🧭️liveness (inherited branch needs the repo test envelope) are pre-existing and not run here.
