#!/bin/zsh
# 🧾️ Builds U/multiview-MH-4.md from report-head.md, the measured section of every asset (report.py, with the note of notes/<id>.md) and report-tail.md: make-report.sh   (needs $GEN; run it before $GEN is deleted)
source "${0:A:h}/env.sh"
out="$U/multiview-MH-4.md"
{
  cat "$W/report-head.md"
  for a in $IDS; do
    python3 "$W/report.py" "$GEN" "$W/ops" "$a" | sed '$d'
    echo "- $(cat "$W/notes/$a.md")"
    echo
  done
  cat "$W/report-tail.md"
} > "$out"
wc -c "$out"
