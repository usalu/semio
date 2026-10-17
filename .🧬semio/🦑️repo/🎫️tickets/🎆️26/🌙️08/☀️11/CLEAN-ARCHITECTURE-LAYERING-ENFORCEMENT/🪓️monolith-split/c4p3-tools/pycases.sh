#!/bin/zsh
# usage: pycases.sh <label>   -> G/c4p3-bimpy-<label>.log : one harness run per case of the BIM IFC oracle (exit code and output per case)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/env.sh
FX="$BIM/🧪️tests/🧫️fixtures/🏗️ifc"; PY="$BIM/🧪️tests/🏗️export-bim-1-ifc/🐍️.py"
: > "$G/c4p3-bimpy-$1.log"
cd /tmp
for c in 🏠️house 🪧️notated 🔲️ceilings 🏷️psets 🏗️frame 🧗️wall-depth 🪑️components 🛝️ramps; do
  echo "=== $c" >> "$G/c4p3-bimpy-$1.log"
  /usr/bin/python3 -B "$T/pyharness.py" "$PY" "$FX" "$c" >> "$G/c4p3-bimpy-$1.log" 2>&1
  echo "exit=$?" >> "$G/c4p3-bimpy-$1.log"
done
