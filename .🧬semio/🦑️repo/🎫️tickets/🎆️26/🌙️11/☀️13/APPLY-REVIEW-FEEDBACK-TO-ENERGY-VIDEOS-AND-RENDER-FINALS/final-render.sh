#!/usr/bin/env bash
# Usage: final-render.sh <video-id> <root-name>; renders the German final, retrying when inputs change mid-render.
cd /c/git/semio || exit 1
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
TK="$(dirname "$0")"
G="$TK/🗑️generated/last"
mkdir -p "$G"
for i in 1 2 3 4 5; do
  "$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video "$1" --profile final --jobs 3 --no-play --output-root "C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/$2" > "$G/$1.$i.log" 2>&1
  code=$?
  echo "$1 try $i exit $code $(date +%H:%M)" >> "$G/status.txt"
  [ $code -eq 0 ] && exit 0
  tr '\r' '\n' < "$G/$1.$i.log" | grep -a -q "inputs changed" || exit $code
done
exit 1
