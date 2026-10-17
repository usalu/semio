#!/bin/zsh
# usage: checkapplied-retry.sh <label>   re-runs the artifacts whose `<label>` run died on a foreign module-load error (peer edit in flight), up to 12 attempts 3 minutes apart
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p17
label=$1
cd /Users/ueli/Documents/semio || exit 2
for art in "🀄️wfc/🗿️artifacts/🧊️3d:wfc3d" "🌀️procedural/🗿️artifacts/🌀️generation2d:gen2d" "🌀️procedural/🗿️artifacts/🧊️generation3d:gen3d"; do
  p="${art%:*}"; n="${art##*:}"
  for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
    rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
    perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$p/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" > "$F/🗑️generated/c4p17-checkapplied-$label-$n.log" 2>&1
    rc=$?
    echo "$n attempt=$attempt exit=$rc" >> "$F/🗑️generated/c4p17-checkapplied-$label-retry.exit"
    /usr/bin/grep -q "Cannot find module" "$F/🗑️generated/c4p17-checkapplied-$label-$n.log" || break
    sleep 180
  done
done
