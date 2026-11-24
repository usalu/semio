#!/bin/zsh
# 🧼️ The document of an object with a generator as it is before the repair of batch R2-O: runs the generator with an empty ops file of this batch (the ops of batch SO stay in force), copies what it wrote to 🗑️generated/silhouette/R2-O/pristine/<id>.json, puts the ops file back and prints the path: pristine.sh <id> <generator.py>
source "${0:A:h}/env.sh"
mkdir -p "$GEN/pristine"
O="$W/ops/$1.json"; [[ -f "$O" ]] && mv "$O" "$O.keep"
echo "[]" > "$O"
(cd "${2:h}" && python3 "${2:t}" > /dev/null) || { echo "generator failed"; exit 1 }
cp "${2:h}/🔣️.json" "$GEN/pristine/$1.json"
rm "$O"; [[ -f "$O.keep" ]] && mv "$O.keep" "$O"
echo "$GEN/pristine/$1.json"
