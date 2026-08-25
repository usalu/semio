#!/bin/zsh
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-audit"
cd /Users/ueli/Documents/semio
: > "$D/08-plugin-matrix.txt"
for p in writer mathematical procedural flow gis vcs animate shooting demonstrator sequence fem architect process lowpoly reasoning-mindmap forms layout cad norm playbook imperative remodel energy trinity dag draw raster stdio note puzzle block space sourcing; do
  pkg="semio-s-plugin-$p"
  out="$D/chk-$p.txt"
  cargo check -p "$pkg" --lib -j 2 --message-format short > "$out" 2>&1
  st=$?
  errs=$(grep -cE '^[^ ].*: error(\[|:)' "$out")
  echo "$pkg exit=$st errorlines=$errs" >> "$D/08-plugin-matrix.txt"
done
echo DONE >> "$D/08-plugin-matrix.txt"
