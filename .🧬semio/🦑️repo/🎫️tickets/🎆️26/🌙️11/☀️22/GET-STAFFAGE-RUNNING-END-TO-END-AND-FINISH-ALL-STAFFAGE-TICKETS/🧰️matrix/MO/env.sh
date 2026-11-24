# 🧭️ Paths of the multiview fixer MO (source it from zsh or bash): B the pinned bun, R the repo, S the staffage package, T the multiview suite, U the ticket, W this folder, M the folder of the MR tools, GEN the tool output, P the package folder; `asset <id>` prints the catalog document, `pb <script.ts> …` runs a script of this folder from the package folder.
export BATCH=MO
export B="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
export R=/Users/ueli/Documents/semio
export S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
export T="$S/🧪️tests/🔭️multiview-coverage"
export U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
export W="$U/🧰️matrix/MO"
export M="$U/🧰️matrix/MR"
export GEN="$U/🗑️generated/matrix/MO"
export P="$S/📦️packages/🟦️typescript"
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
pb() { (cd "$P" && timeout "${TO:-300}" "$B" "$W/$1" "${@:2}"); }
