# 🧭️ Paths of the multiview fixer MC (source it): B the pinned bun, R the repo, S the staffage package, T the multiview suite, U the ticket, W this folder, M the folder of the MR tools, GEN the tool output, P the package folder, IDS the 20 ids of the batch; `asset <id>` prints the catalog document and `q <MB> <timeout s> <dir> <command…>` runs a command through the memory queue.
export BATCH=MC
export B="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
export R=/Users/ueli/Documents/semio
export S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
export T="$S/🧪️tests/🔭️multiview-coverage"
export U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
export W="$U/🧰️matrix/MC"
export M="$U/🧰️matrix/MR"
export GEN="$U/🗑️generated/matrix/MC"
export P="$S/📦️packages/🟦️typescript"
IDS=($(cut -f1 "$U/🧰️matrix/batches/MC.tsv"))
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
q() { "$W/q.sh" "$@"; }
