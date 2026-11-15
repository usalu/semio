#!/usr/bin/env bash
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
OUT="$HERE/../🗑️generated/consistency-out/posters"
mkdir -p "$OUT"
cd "$ROOT" || exit 2
for video in "$@"; do
  echo "=== $video"
  repo tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" posters --language de --profile preview --audience speaker --video "$video" --jobs 2 --output-root "$OUT" 2>&1 | tail -40
done
echo done
