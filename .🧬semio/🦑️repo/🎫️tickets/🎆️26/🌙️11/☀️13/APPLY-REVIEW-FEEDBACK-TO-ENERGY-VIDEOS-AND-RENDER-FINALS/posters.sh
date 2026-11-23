#!/usr/bin/env bash
# Usage: posters.sh <language>...; captures the stored chapter posters of each language in turn.
cd /c/git/semio || exit 1
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
TK="$(dirname "$0")"
G="$TK/🗑️generated/last"
mkdir -p "$G"
for language in "$@"; do
  "$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" posters --language "$language" --profile final > "$G/posters.$language.log" 2>&1
  echo "posters $language exit $? $(date +%H:%M)" >> "$G/status.txt"
done
