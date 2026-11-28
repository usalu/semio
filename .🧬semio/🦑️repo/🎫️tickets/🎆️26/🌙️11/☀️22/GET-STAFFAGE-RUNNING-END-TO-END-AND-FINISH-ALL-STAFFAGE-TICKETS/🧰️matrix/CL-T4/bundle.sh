#!/bin/zsh
# 🧺️ Several tools of this folder one after the other inside ONE grant of the memory queue (the queue is long, a request per tool waits ten minutes each): bundle.sh <name> runs every line of `bundles/<name>.txt` (`<script> <args…>`) and prints its last line.
source "${0:A:h}/env.sh"
while IFS= read -r line; do [[ -z "$line" ]] && continue; echo "== $line"; (cd "$W" && zsh ${=line}) 2>&1 | tail -2 | cut -c1-240; done < "$W/bundles/$1.txt"
