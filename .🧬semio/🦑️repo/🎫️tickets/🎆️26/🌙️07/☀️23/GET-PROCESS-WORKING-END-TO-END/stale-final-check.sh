#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
cd /Users/ueli/Documents/semio || exit 1
i=0
for m in "🧰️framework/🔨️modules/🌱️value/🛫️encode" "🧰️framework/🔨️modules/🚪️io/⏱️control" "🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests" "🧰️framework/🔨️modules/🚪️io/🧬️schema" "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot" "🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw"; do
 i=$((i+1)); $T/stale-cargo.sh fin$i check --tests --manifest-path "$m/📦️packages/🦀️rust/Cargo.toml" --message-format short >/dev/null
 echo "$i $(grep -cE ': error' $T/🗑️generated/stale-fin$i.txt) $m"
done
