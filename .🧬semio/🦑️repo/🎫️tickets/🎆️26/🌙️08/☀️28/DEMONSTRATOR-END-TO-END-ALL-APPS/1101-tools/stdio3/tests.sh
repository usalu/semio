#!/bin/bash
# usage: tests.sh <round>  — native tests per stdio artifact workspace, serial
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
R=$1; shift
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
for pair in csv:📊️csv json:🧾️json xlsx:📕️xlsx zip:🎒️zip docx:📜️docx tiff:🖼️tiff png:📷️png; do
  n=${pair%%:*}; d=${pair#*:}
  [ -n "$ONLY" ] && [ "$ONLY" != "$n" ] && continue
  O="$C/🗑️generated/1101/stdio3-t$R-$n"; rm -f "$O.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$A/$d/Cargo.toml" --workspace --no-fail-fast > "$O.txt" 2>&1
  echo $? > "$O.rc"
done
touch "$C/🗑️generated/1101/stdio3-t$R.done"
