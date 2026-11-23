#!/bin/bash
cd /c/git/semio
F=$(realpath .🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe)
V="$1"
O="$2"
mkdir -p "$O"
"$F" -hide_banner -loglevel error -y -i "$V" -vf "fps=1/5,scale=1280:720" "$O/f_%04d.png"
