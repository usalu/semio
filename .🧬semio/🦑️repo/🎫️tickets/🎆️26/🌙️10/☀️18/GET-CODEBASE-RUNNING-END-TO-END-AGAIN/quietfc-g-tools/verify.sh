#!/bin/sh
# 🧪️ quietfc-g: the laws of the staged change, run one test file at a time on the scratch overlay (`sh rebuild.sh` first).
set -e
OV=${QFC_ROOT:-/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/quietfc-g-ov}
HC="$OV/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB="$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
(cd "$HC/🧪️tests/🔼️upgrade/🐍️oracle" && python3 ./🐍️.py --check)
(cd "$HC/🧪️tests/🌤️followup/🐍️oracle" && python3 ./🐍️.py --check)
(cd "$HC/🧪️tests/🪜️phases/🐍️oracle" && python3 ./🐍️.py --check)
cd "$HC"
for f in 🌤️followup 🔼️upgrade 📐️estimates 📡️sampling 🚪️gate 🪜️phases 🤝️handover; do nice -n 10 bun test --timeout 120000 "./🧪️tests/$f/🟦️.ts"; done
nice -n 10 bun test --timeout 120000 ./🧪️tests/🟦️.ts
cd "$LIB"
for f in 🌤️followup 🔼️upgrade 🧮️arguments 🏊️farm 📈️memory-fit 🔁️restart 🪟️partial 🪜️phases; do nice -n 10 bun test --timeout 120000 "./🌡️host-capacity/🧪️tests/$f/🟦️.ts"; done
nice -n 10 bun test --timeout 120000 ./🌡️host-capacity/🧪️tests/🟦️.ts
