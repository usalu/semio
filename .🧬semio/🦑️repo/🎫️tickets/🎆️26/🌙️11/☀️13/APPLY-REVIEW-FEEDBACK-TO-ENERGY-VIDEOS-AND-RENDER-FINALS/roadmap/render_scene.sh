#!/usr/bin/env bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/roadmap-out"
mkdir -p "$OUT"
export SEMIO_TEACHING_OUTPUT_ROOT="$OUT" SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_AUDIENCE=speaker PYTHONIOENCODING=utf-8 PYTHONUTF8=1
SCRIPT="$1"
SCENE="$2"
uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim --media_dir "$OUT/direct" --resolution 854,480 --fps 15 --disable_caching "🎓️teaching/🏛️architecture/⚡️energy/$SCRIPT" "$SCENE" > "$OUT/direct-$SCENE.log" 2>&1
echo "exit $?" >> "$OUT/direct-$SCENE.log"
tail -5 "$OUT/direct-$SCENE.log"
