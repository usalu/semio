#!/bin/sh
# usage: verify-tsc.sh  runs tsc for the staffage library, React target and site projects one after another (what their typecheck targets run), logs under 🗑️generated/verify
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/verify"
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
for p in "$S/📦️packages/🟦️typescript" "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" "$S/🌐️site/📦️packages/🟦️typescript"; do
  n=$(echo "$p" | sed 's#.*staffage/##; s#/📦️packages/🟦️typescript##; s#[^a-zA-Z]#_#g')
  (cd "$p" && nice -n 12 "$R/node_modules/.bin/tsc" --noEmit -p tsconfig.json > "$G/tsc-$n.log" 2>&1; echo "exit $?" >> "$G/tsc-$n.log")
done
