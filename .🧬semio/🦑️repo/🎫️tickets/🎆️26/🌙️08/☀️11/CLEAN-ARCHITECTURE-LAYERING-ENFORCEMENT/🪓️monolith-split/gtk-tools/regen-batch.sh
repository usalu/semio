#!/bin/zsh
# usage: regen-batch.sh : regenerates the carve of several artifacts in sandboxes (logs F/🗑️generated/gtk-regen-<tag>.log)
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
ulimit -n 65536
run() { tag=$1; art=$2; bk=$3; shift 3; nice -n 10 bun "$F/gtk-tools/gtk-regen.ts" $tag "$art" "$G/gtk-regen-$tag.json" "$bk" "$@" > "$G/gtk-regen-$tag.log" 2>&1; echo $? > "$G/gtk-regen-$tag.exit"; }
S="✏️s/🔌️plugins"
run avi "$S/🗄️stdio/🗿️artifacts/📼️avi" "$G/f3-backup-avi"
run bmp "$S/🗄️stdio/🗿️artifacts/🪟️bmp" "$G/f3-backup-bmp2"
run epw "$S/🗄️stdio/🗿️artifacts/🌦️epw" "$G/f3-backup-epw2"
run gif "$S/🗄️stdio/🗿️artifacts/🎞️gif" "$G/f3-backup-gif3"
run mp3 "$S/🗄️stdio/🗿️artifacts/🎵️mp3" "$G/f3-backup-mp3"
run mp4 "$S/🗄️stdio/🗿️artifacts/🎥️mp4" "$G/f3-backup-mp4"
run png "$S/🗄️stdio/🗿️artifacts/📷️png" "$G/f3-backup-png"
run txt "$S/🗄️stdio/🗿️artifacts/🔤️txt" "$G/f3-backup-txt"
run wav "$S/🗄️stdio/🗿️artifacts/🔊️wav" "$G/f3-backup-wav"
run jpg "$S/🗄️stdio/🗿️artifacts/📸️jpg" "$G/r5a-backup-jpg"
run tiff "$S/🗄️stdio/🗿️artifacts/🖼️tiff" "$G/r5a-backup-tiff"
run json "$S/🗄️stdio/🗿️artifacts/🧾️json" "$G/r1-backup-json"
echo done > "$G/gtk-regen-batch.done"
