#!/usr/bin/env bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/roadmap-out"
FF="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
NAME="$1"
VIDEO="$(find "$OUT/direct/videos" -name "$NAME.mp4" | head -1)"
mkdir -p "$OUT/frames"
rm -f "$OUT/frames/${NAME}_"*.png
"$FF" -v error -ss "${2:-0}" -i "$VIDEO" -vf "fps=1/${3:-4},scale=640:-1,tile=2x3" -frames:v 3 "$OUT/frames/${NAME}_%02d.png"
ls "$OUT/frames" | grep "$NAME"
