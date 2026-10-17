#!/bin/zsh
# usage: checkapplied.sh <label>   runs `new crates <pkg> --check-applied --base=f856dff0813` for the eight artifact packages of CENSUS4-PKG-19
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p19
label=$1
cd /Users/ueli/Documents/semio || exit 2
for art in "📕️norm/🗿️artifacts/📇️iso16757:iso16757" "📕️norm/🗿️artifacts/🧩️en1994:en1994" "📕️norm/🗿️artifacts/🧱️din4108:din4108" "📕️norm/🗿️artifacts/🪨️en1996:en1996" "📕️norm/🗿️artifacts/🪵️en1995:en1995" "📕️norm/🗿️artifacts/🪶️en1999:en1999" "📕️norm/🗿️artifacts/🫨️en1998:en1998" "📖️playbook/🧩️extensions/🌀️procedural:procedural"; do
  p="${art%:*}"; n="${art##*:}"
  rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
  perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$p/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" > "$F/🗑️generated/c4p19-checkapplied-$label-$n.log" 2>&1
  echo "$n exit=$?" >> "$F/🗑️generated/c4p19-checkapplied-$label.exit"
done
