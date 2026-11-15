#!/bin/bash
cd /c/git/semio
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
FF="$(pwd)/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
V="${VIDEO:-$(pwd)/.🧬semio/🎓️teaching/⚡️cache/vc/final/de/speaker/rendered/Full_Cooling_Demand_speaker_de_1080p30.mp4}"
G="$(pwd)/$TK/🗑️generated/fix-cool/${OUT:-z}"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$TK/fix-cool/grab.py" "$FF" "$V" "$G" "$@"
