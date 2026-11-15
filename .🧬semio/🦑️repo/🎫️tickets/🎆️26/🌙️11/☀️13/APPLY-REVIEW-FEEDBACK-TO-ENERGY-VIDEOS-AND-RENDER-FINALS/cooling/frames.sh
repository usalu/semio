#!/usr/bin/env bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/cooling-out"
FF="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
SCENE="$1"
STEP="${2:-3}"
mkdir -p "$OUT/frames"
rm -f "$OUT/frames/$SCENE"_*.png
"$FF" -v error -i "$OUT/direct/videos/🐍️/480p15/$SCENE.mp4" -vf "fps=1/$STEP,scale=640:-1,tile=2x3" -frames:v 20 "$OUT/frames/${SCENE}_%02d.png"
ls "$OUT/frames" | grep "$SCENE"
