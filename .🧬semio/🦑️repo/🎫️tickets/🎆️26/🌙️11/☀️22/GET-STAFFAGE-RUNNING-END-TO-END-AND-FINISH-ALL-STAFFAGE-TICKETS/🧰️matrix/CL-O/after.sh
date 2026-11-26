#!/bin/zsh
# 🏁️ The write chain of lane CL-O inside one grant: the silhouette guard of the backups (`$GEN/backup/<id>.json`, the catalog files before this lane) into `$GEN/log/guard-before.txt`, then `prove.sh` for every asset named. after.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$M/measure.ts" "$GEN/backup/$a.json") 2>&1 | grep -E "silhouette:|validate:|sweep:|bbox" | cut -c1-300; done > "$GEN/log/guard-before.txt"
zsh "$W/prove.sh" "$@"
