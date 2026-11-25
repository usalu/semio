# 🧭️ Paths of the multiview fixer ML-1 (source it from zsh): B the pinned bun, R the repo, S the staffage package, T the multiview suite, U the ticket, W this folder, M the MR tools, GEN the tool output, P the package folder; `asset <id>` prints the catalog document; `q <MB> <seconds> <cwd> <script> <args…>` runs a bun script from <cwd> through the machine-wide memory queue (`repo capacity run`, started at the repo root as the brief demands); `qs <MB> <seconds> '<shell commands>'` runs several processes one after the other under one grant. The queue gives a request up after about half an hour of waiting ("The wait ends here"); both functions then ask again until they are admitted, and print the output of the admitted run only.
export BATCH=ML-1
export B="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
export R=/Users/ueli/Documents/semio
export S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
export T="$S/🧪️tests/🔭️multiview-coverage"
export U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
export W="$U/🧰️matrix/ML-1"
export M="$U/🧰️matrix/MR"
export GEN="$U/🗑️generated/matrix/ML-1"
export P="$S/📦️packages/🟦️typescript"
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
waited() { local log="$1" code="$2"; [[ "$code" -ne 0 ]] && grep -q "The wait ends here" "$log"; }
q() { local mb="$1" seconds="$2" dir="$3" log code; shift 3; log="$(mktemp)"; while :; do (cd "$R" && repo capacity run --memory="$mb" -- timeout "$seconds" sh -c 'cd "$0" && exec "$@"' "$dir" "$B" "$@") > "$log" 2>&1; code=$?; waited "$log" "$code" || break; done; cat "$log"; rm -f "$log"; return $code; }
qs() { local mb="$1" seconds="$2" log code; shift 2; log="$(mktemp)"; while :; do (cd "$R" && repo capacity run --memory="$mb" -- timeout "$seconds" zsh -c "$1") > "$log" 2>&1; code=$?; waited "$log" "$code" || break; done; cat "$log"; rm -f "$log"; return $code; }
