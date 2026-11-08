#!/bin/bash
# usage: model.sh <n> — builds the neural-engine-model test binary (--no-run) through the slot, then runs each test alone under a 40 s kill timer
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N="valuetests-$1"; M="🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/📦️packages/🦀️rust/Cargo.toml"
G="$C/🗑️generated/1105b"
"$C/1105b-tools/runtime/tst.sh" "$N" "$M" native -p semio-os-kernel-neural-engine-model --lib --no-run
BIN=$(/usr/bin/grep -o "Executable unittests[^(]*(\([^)]*\))" "$G/$N.txt" | tail -1 | sed 's/.*(\(.*\))/\1/')
OUT="$G/$N-each.txt"; : > "$OUT"
[ -x "$BIN" ] || { echo "no binary: $BIN" >> "$OUT"; echo 1 > "$G/$N-each.rc"; exit 1; }
FAIL=0
for t in $("$BIN" --list 2>/dev/null | /usr/bin/grep ": test$" | sed 's/: test$//'); do
  "$BIN" --exact "$t" --test-threads=1 > "$G/$N-one.txt" 2>&1 & P=$!
  ( sleep 40; kill -9 $P 2>/dev/null ) & W=$!
  wait $P; R=$?; kill $W 2>/dev/null
  echo "$R $t" >> "$OUT"; [ $R -ne 0 ] && FAIL=1
done
echo $FAIL > "$G/$N-each.rc"
