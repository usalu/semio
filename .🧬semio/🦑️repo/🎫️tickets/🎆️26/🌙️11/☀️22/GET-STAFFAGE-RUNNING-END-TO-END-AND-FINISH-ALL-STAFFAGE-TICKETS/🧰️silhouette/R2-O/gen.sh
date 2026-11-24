#!/bin/zsh
# 🏭️ The reproducible repair of an object that has a generator in a ticket: hooks the ops of batch R2-O as the last step of the generator (once), runs the generator, copies the document it wrote to the catalog (read right before) and proves it: gen.sh <id> <generator.py>
source "${0:A:h}/env.sh"
python3 "$W/hook.py" "$2" "$1"
(cd "${2:h}" && python3 "${2:t}" | grep "silhouette repair") || exit 1
cmp -s "${2:h}/🔣️.json" "$CAT/$1/🔣️.json" || cp "${2:h}/🔣️.json" "$CAT/$1/🔣️.json"
NOAPPLY=1 "$W/prove.sh" "$1"
