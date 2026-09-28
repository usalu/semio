#!/bin/zsh
# 🚀️ C13 window-3 live proof: rows 3.4 (viewer, en + de) and 3.11 (cross-undo) on one hub through serve 6670, sequentially.
# usage: zsh run-window3.sh <tagPrefix> <hubUrl> <adminCapabilityFile>
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-c13 || exit 1
KINDS="2d.block,s.note.note,text.document,2d.drawing"
zsh run-two-human.sh "$1-viewer-en" viewer en "$2" http://127.0.0.1:6670/ http://127.0.0.1:6670/ "$3" "$KINDS" 300000
zsh run-two-human.sh "$1-viewer-de" viewer de "$2" http://127.0.0.1:6670/ http://127.0.0.1:6670/ "$3" "$KINDS" 300000
zsh run-two-human.sh "$1-undo-en" cross-undo en "$2" http://127.0.0.1:6670/ http://127.0.0.1:6670/ "$3" "s.note.note,text.document,2d.drawing" 300000
echo "WINDOW3 DONE $(date '+%F %T')"
