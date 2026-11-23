#!/bin/bash
cd /c/git/semio
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
F=$(realpath .🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe)
V=${VID:-.🧬semio/🎓️teaching/⚡️cache/vb/final/de/speaker/rendered/Full_Heating_Demand_speaker_de_1080p30.mp4}
O="$TK/🗑️generated/fix-eb2/${OUT:-one}"
mkdir -p "$O"
for t in "$@"; do "$F" -hide_banner -loglevel error -y -ss "$t" -i "$V" -frames:v 1 -vf scale=1280:720 "$O/t_${t}.png"; done
