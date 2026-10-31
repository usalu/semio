#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for p in "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"; do
  n=$(echo "$p" | cksum | cut -d' ' -f1)
  repo tools exec tsc -- -p "$p/tsconfig.json" > "$C/🗑️generated/1030/e2eprep-pkg-$n.txt" 2>&1; echo $? > "$C/🗑️generated/1030/e2eprep-pkg-$n.rc"; echo "$n $p" >> "$C/🗑️generated/1030/e2eprep-pkg-map.txt"
done
echo done > "$C/🗑️generated/1030/e2eprep-pkg-all.rc"
