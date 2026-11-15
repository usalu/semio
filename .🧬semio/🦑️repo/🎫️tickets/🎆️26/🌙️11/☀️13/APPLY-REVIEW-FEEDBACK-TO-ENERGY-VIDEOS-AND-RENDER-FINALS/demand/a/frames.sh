#!/usr/bin/env bash
# usage: frames.sh <SceneClass> <step-seconds> [cols] [rows]   -> contact sheets in frames/<SceneClass>-NN.png
cd /c/git/semio || exit 1
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/demand-a-out"
FF="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
scene="$1"; step="$2"; cols="${3:-2}"; rows="${4:-3}"
mkdir -p "$OUT/frames"
rm -f "$OUT/frames/$scene"-*.png
"$FF" -v error -y -i "$OUT/direct/media/videos/🐍️/480p15/$scene.mp4" -vf "fps=1/$step,scale=854:480,tile=${cols}x${rows}" "$OUT/frames/$scene-%02d.png"
ls "$OUT/frames" | grep "$scene"
