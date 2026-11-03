#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
G="$T/🗑️generated/sm"; mkdir -p "$G"
rm -f "$G/results.jsonl"
while IFS= read -r f; do
  case "$f" in *💊️capsule*) echo "{\"scene\":\"<file>\",\"status\":\"skipped\",\"file\":\"$f\",\"head\":\"grasshopper capsule known multi-hour\"}" >> "$G/results.jsonl"; continue;; esac
  echo "$(date +%T) $f" >> "$G/progress.log"
  "$T/🧰️sm/run.sh" "$f" >> "$G/progress.log" 2>&1
done < "$T/🧰️sm/files.txt"
echo ALLDONE >> "$G/progress.log"
