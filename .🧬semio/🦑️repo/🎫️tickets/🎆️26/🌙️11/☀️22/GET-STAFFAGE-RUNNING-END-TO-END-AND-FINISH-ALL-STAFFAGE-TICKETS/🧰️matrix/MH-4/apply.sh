#!/bin/zsh
# ✍️ Writes the ops of one asset into its catalog file and proves the chain is stable: apply.sh <id>   runs fit.ts with --write on the catalog file (refused when the file changed on disk meanwhile), then once more without (must report "0 changed something"), and compares the result with the dry run of try.sh.
source "${0:A:h}/env.sh"
source "$W/lock.sh"
a="$1"; f="$S/🗂️catalog/👤️humans/$a/🔣️.json"
(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --write) | tail -2 | cut -c1-200
(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json") | tail -1
cmp -s "$f" "$GEN/try/$a.json" && echo "catalog file equals the proven dry run" || echo "catalog file DIFFERS from the dry run"
