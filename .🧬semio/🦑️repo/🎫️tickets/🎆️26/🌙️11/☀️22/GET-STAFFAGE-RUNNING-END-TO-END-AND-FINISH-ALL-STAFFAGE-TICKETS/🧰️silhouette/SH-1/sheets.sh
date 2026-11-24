#!/bin/zsh
# 🖼️ The 1:50 and 1:500 sheets of people before (the file of the session start) and after the repair, side by side per scale for looking at them: sheets.sh <slug>…  (output 🗑️generated/silhouette/SH-1/look/<slug>/{before,after}; a scale the person does not author is skipped by the preview)
source "${0:A:h}/env.sh"
for a in "$@"; do
  "$W/look.sh" "$a" "50 500" "$GEN/backup/$a.json" before > /dev/null 2>&1
  "$W/look.sh" "$a" "50 500" - after > /dev/null 2>&1
  echo "$a: $(ls "$GEN/look/$a/before" 2>/dev/null | wc -l | tr -d ' ') before, $(ls "$GEN/look/$a/after" 2>/dev/null | wc -l | tr -d ' ') after"
done
