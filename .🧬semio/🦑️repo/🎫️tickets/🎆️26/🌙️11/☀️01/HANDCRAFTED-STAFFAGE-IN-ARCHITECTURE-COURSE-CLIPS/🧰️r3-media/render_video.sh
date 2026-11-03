#!/usr/bin/env bash
# render_video.sh <en|de> <video-id>... : canonical 480p15 speaker render of manifest videos, one at a time.
# Segments whose state is unchanged (same sources and staffage cast keys) are reused; a source edit during the run aborts the video
# (guard) and the video is retried, which only redoes the segments that did not finish. One run per language at a time (lock folder).
W=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
while [ ! -d "$W/🧰️framework" ] && [ "$W" != "/" ]; do W=$(dirname "$W"); done
CACHE="${SEMIO_TEACHING_CACHE_ROOT:-$W/.🧬semio/🎓️teaching/⚡️cache}"
LANGUAGE="$1"; shift
AUDIENCE="${AUDIENCE:-speaker}"
JOBS="${JOBS:-1}"
ATTEMPTS="${ATTEMPTS:-4}"
LOGS="${MEDIA_LOGS:-$CACHE/media-logs}"
LOCK="$CACHE/videos/$LANGUAGE/.media-lock"
mkdir -p "$LOGS" "$CACHE/videos/$LANGUAGE"
mkdir "$LOCK" 2>/dev/null || { echo "another render of language $LANGUAGE holds $LOCK"; exit 3; }
trap 'rmdir "$LOCK"' EXIT
export TMP="$LOGS/tmp" TEMP="$LOGS/tmp"; mkdir -p "$TMP"
cd "$W"
for v in "$@"; do
  for attempt in $(seq 1 "$ATTEMPTS"); do
    if bun "$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --video "$v" --language "$LANGUAGE" --audience "$AUDIENCE" \
        --quality l --fps 15 --no-play --log-output --jobs "$JOBS" > "$LOGS/$v-$LANGUAGE-a$attempt.log" 2>&1; then
      echo "[DEBUG] $v ok attempt=$attempt $(date +%T)"; break
    fi
    echo "[DEBUG] $v failed attempt=$attempt $(date +%T): $(grep -m1 -i 'changed while\|Error' "$LOGS/$v-$LANGUAGE-a$attempt.log")"
  done
done
