#!/bin/zsh
# usage: checkapplied.sh <label> ; runs `new crates <pkg> --check-applied --base=HEAD` for cad, layout, norm and en1990 (CENSUS4-PKG-12)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p12-ca
label=$1
export LC_ALL=en_US.UTF-8
ulimit -n 65536
cd /Users/ueli/Documents/semio || exit 2
for art in "📐️cad/🗿️artifacts/📐️cad:cad" "📏️layout/🗿️artifacts/📏️layout:layout" "📕️norm:norm" "📕️norm/🗿️artifacts/⚖️en1990:en1990"; do
  p="${art%:*}"; n="${art##*:}"
  rm -rf "$SCR/$label-$n"; mkdir -p "$SCR/$label-$n"
  nice perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$p/📦️packages/🦀️rust" --check-applied --base=HEAD --sandbox="$SCR/$label-$n" > "$F/🗑️generated/c4p12-checkapplied-$label-$n.log" 2>&1
  echo "$n exit=$?" >> "$F/🗑️generated/c4p12-checkapplied-$label.exit"
done
