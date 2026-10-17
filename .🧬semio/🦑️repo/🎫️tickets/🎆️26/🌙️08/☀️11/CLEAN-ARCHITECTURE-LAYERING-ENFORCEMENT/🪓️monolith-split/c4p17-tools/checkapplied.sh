#!/bin/zsh
# usage: checkapplied.sh <label>   runs `new crates <pkg> --check-applied --base=f856dff0813` for the five artifacts of CENSUS4-PKG-17
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p17
label=$1
cd /Users/ueli/Documents/semio || exit 2
for art in "➗️mathematical/🗿️artifacts/➗️equation:equation" "🀄️wfc/🗿️artifacts/🖼️bitmap:wfcbitmap" "🀄️wfc/🗿️artifacts/🧊️3d:wfc3d" "🌀️procedural/🗿️artifacts/🌀️generation2d:gen2d" "🌀️procedural/🗿️artifacts/🧊️generation3d:gen3d"; do
  p="${art%:*}"; n="${art##*:}"
  rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
  perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$p/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" > "$F/🗑️generated/c4p17-checkapplied-$label-$n.log" 2>&1
  echo "$n exit=$?" >> "$F/🗑️generated/c4p17-checkapplied-$label.exit"
done
