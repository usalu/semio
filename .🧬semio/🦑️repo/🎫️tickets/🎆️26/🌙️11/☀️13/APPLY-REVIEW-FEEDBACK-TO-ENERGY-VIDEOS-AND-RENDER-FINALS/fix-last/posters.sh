#!/usr/bin/env bash
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
OUT="$HERE/../🗑️generated/fix-last/posters-out"
mkdir -p "$OUT"
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
cd "$ROOT" || exit 2
for video in "$@"; do
  echo "=== $video"
  "$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" posters --language de --profile preview --audience speaker --video "$video" --jobs 2 --output-root "$OUT" 2>&1 > "$OUT/$video.log" 2>&1; tail -15 "$OUT/$video.log"
done
echo done
