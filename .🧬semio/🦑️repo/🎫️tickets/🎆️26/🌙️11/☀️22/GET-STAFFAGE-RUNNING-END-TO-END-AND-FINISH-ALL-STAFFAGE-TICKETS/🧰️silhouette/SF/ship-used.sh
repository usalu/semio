#!/bin/zsh
# 🚚️ Ships a used-object asset whose generator in its collection ticket was repaired and run: checks that the catalog file is still the one this batch started from or already the generator output (nobody else wrote it), copies the generator output `assets/<id>/🔣️.json` of the collection ticket to the catalog, renders the 1:50 and 1:500 sheets and runs the proof. The manifest is not touched: ship-used.sh <id>
source "${0:A:h}/env.sh"
id="$1"; src=$(ls -d "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22"/USED-*/assets/"$id" | head -1)/🔣️.json
cmp -s "$CAT/$id/🔣️.json" "$GEN/backup/$id.json" || cmp -s "$CAT/$id/🔣️.json" "$GEN/shipped/$id.json" || { echo "$id: the catalog file changed under this batch, not copied"; exit 1; }
mkdir -p "$GEN/shipped"; cp "$src" "$CAT/$id/🔣️.json"; cp "$src" "$GEN/shipped/$id.json"
"$W/look.sh" after "$CAT/$id/🔣️.json" "$id" 50 500 | sed 's/.*look/look/'
"$W/prove.sh" "$id"
