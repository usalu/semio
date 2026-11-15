#!/usr/bin/env bash
# usage: sheet.sh <video.mp4> <out.png> <seconds-per-frame> <cols> <rows> [start-offset-seconds]
FF="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe"
"$FF" -y -loglevel error -ss "${6:-0}" -i "$1" -vf "fps=1/$3,scale=480:-1,tile=$4x$5" -frames:v 1 "$2"
