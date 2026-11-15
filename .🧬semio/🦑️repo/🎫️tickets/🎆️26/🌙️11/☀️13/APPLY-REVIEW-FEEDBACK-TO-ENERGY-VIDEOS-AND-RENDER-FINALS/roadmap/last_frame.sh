#!/usr/bin/env bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/roadmap-out"
FF="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
V="$(find "$OUT/direct/videos" -path "*$1*" -name "Beat1_Fahrplan.mp4" | head -1)"
mkdir -p "$OUT/frames"
"$FF" -v error -y -sseof -"${3:-1.0}" -i "$V" -frames:v 1 "$OUT/frames/$2.png"
