#!/usr/bin/env bash
# usage: frames.sh <Scene> <outname> [interval-seconds]
cd /c/git/semio || exit 1
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
FF=".🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
F=$(find "$TK/🗑️generated/demand-b-out/direct/media" -name "$1.mp4" | head -1)
D="$TK/🗑️generated/demand-b-out/frames/$2"
rm -rf "$D"; mkdir -p "$D"
"$FF" -v error -i "$F" -vf "fps=1/${3:-4},scale=854:-1" "$D/f%02d.png"
"$FF" -v error -framerate 1 -i "$D/f%02d.png" -vf "tile=2x2" "$D/sheet%d.png"
ls "$D" | grep -c sheet
