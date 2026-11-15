#!/usr/bin/env bash
# Usage: render.sh <chapter-dir-name> <SceneClass>...   (direct manim preview render, German, into the private output root)
set -u
ROOT="/c/git/semio"
TK="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/heat-b-out/direct"
FF="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
MANIM="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/manim.exe"
export SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_LANGUAGE=de SEMIO_TEST_MANIM=1 PYTHONUTF8=1 SEMIO_TEACHING_OUTPUT_ROOT="$TK/🗑️generated/heat-b-out"
mkdir -p "$OUT"
CHAPTER="$1"
shift
for CLS in "$@"; do
  (cd "$OUT" && "$MANIM" -ql --disable_caching --media_dir "$OUT/media" "$ROOT/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/$CHAPTER/🐍️.py" "$CLS" > "$OUT/$CLS.log" 2>&1)
  rm -rf "$OUT/$CLS"
  mkdir -p "$OUT/$CLS"
  "$FF" -v error -i "$OUT/media/videos/🐍️/480p15/$CLS.mp4" -vf "fps=1/${FRAME_EVERY:-3}" "$OUT/$CLS/f_%03d.png" -y
  echo "done $CLS: $(ls "$OUT/$CLS" | wc -l) frames"
done
