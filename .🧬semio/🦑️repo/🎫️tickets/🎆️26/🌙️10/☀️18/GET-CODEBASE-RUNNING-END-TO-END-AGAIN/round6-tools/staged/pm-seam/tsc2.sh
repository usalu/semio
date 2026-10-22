#!/bin/sh
# usage: tsc2.sh MIRROR OUTFILE   type-checks the production files round 2 touches (those that exist in the mirror) and writes sorted relative diagnostics
M=$1; OUT=$2; cd "$M"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"; K="$D/🔌️managers"
FILES=""
for f in "$K/🟦️.ts" "$K/📖️formats/🟦️.ts" "$K/📚️catalog/🟦️.ts" "$K/🧩️materialize/🟦️.ts" "$K/🧪️tests/🟦️.ts" "$D/💾️storage/🧱️schemas/🟦️.ts" "$D/💾️storage/🟦️.ts" "$D/🧾️incremental/💾️storage/🟦️.ts" "$D/🧾️generation/🟦️.ts" "$D/🧾️generation/💾️storage/🟦️.ts" "$L/🏗️build/🚀️bootstrap/🟦️.ts" "$L/🏗️build/🕸️graph/🟦️.ts" "$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts" $EXTRA "$L/🕸️dependencies/⚖️truth/🟦️.ts" "$L/🕸️dependencies/📇️inventory/🟦️.ts" "$L/🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🟦️.ts" "$L/⚡️caching/🕸️graph/✅️verification/🟦️.ts" "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts" "$D/🏗️native/🟦️.ts"; do
  [ -f "$f" ] && FILES="$FILES\"$f\","
done
FILES=${FILES%,}
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":[$FILES]}
JSON
nice -n 10 node_modules/.bin/tsc -p tsconfig.x.json | command grep -E "^🧰️" | sed -E 's/\(([0-9]+),([0-9]+)\)/(L\1)/' | cut -c1-400 | sort -u > "$OUT"
wc -l < "$OUT"
