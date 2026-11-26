#!/bin/zsh
# 🥇️ First stage of closing lane CL-A inside ONE grant: for each asset the pins measured on the catalog file (`$GEN/pins/<id>-before.txt`), the backup with its digest (taken once: a second trial starts from the same backup), the trial copy `$GEN/try/<id>.json` made by the repair tool (and a second run on the copy that must change nothing), the pins on the copy (`<id>-try.txt`) and the light proof of the copy (`light.sh`). Nothing is written to the catalog. stage1.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/pins" "$GEN/try" "$GEN/backup" "$GEN/log"
for a in "$@"; do
  f="$(ls "$C"/*/"$a"/🔣️.json)"
  [[ -f "$GEN/backup/$a.json" ]] || { cp "$f" "$GEN/backup/$a.json"; echo "$a $(shasum -a 256 < "$f" | cut -c1-16) $(wc -c < "$f" | tr -d ' ') bytes $(date +%F\ %H:%M)" >> "$W/backup-sha256.txt"; }
  (cd "$S/📦️packages/🟦️typescript" && "$B" "$W/pins.ts" "$a" "$GEN/backup/$a.json") > "$GEN/pins/$a-before.txt" 2>&1
  zsh "$W/tool.sh" "$a" "$GEN/backup/$a.json" "$GEN/try/$a.json" > "$GEN/log/$a-tool.txt" 2>&1
  zsh "$W/tool.sh" "$a" "$GEN/try/$a.json" "$GEN/try/$a.second.json" > "$GEN/log/$a-tool2.txt" 2>&1
  cmp -s "$GEN/try/$a.json" "$GEN/try/$a.second.json" && echo "second run: unchanged" >> "$GEN/log/$a-tool.txt" || echo "second run: DIFFERS" >> "$GEN/log/$a-tool.txt"; rm -f "$GEN/try/$a.second.json"
  (cd "$S/📦️packages/🟦️typescript" && "$B" "$W/pins.ts" "$a" "$GEN/try/$a.json") > "$GEN/pins/$a-try.txt" 2>&1
  zsh "$W/light.sh" "$a"
  echo "== $a"; cat "$GEN/log/$a-tool.txt"; tail -12 "$GEN/pins/$a-try.txt"; cat "$GEN/light/$a.txt"
done
