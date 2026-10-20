#!/bin/zsh
# usage: sheet.sh BeatClass fps  -> contact sheets (2x3 frames each) straight from the mp4
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/ch1"
B=$1; FPS=${2:-0.7}
F="$T/media/videos/scene_1/480p15/$B.mp4"
setopt nonomatch
rm -f "$T"/frames/${B}_*
ffmpeg -v error -i "$F" -vf "fps=$FPS,tile=2x3:padding=4" "$T/frames/${B}_sheet_%02d.png"
ls "$T/frames" | grep "${B}_sheet"
