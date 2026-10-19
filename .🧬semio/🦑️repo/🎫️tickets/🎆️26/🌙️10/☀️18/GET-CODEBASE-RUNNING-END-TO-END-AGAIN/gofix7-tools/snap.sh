#!/usr/bin/env bash
# usage: snap.sh <label> -- records tracked .🧬semio status (no untracked) and recently modified goal/ticket files outside the coordinator ticket
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gofix7-logs"; mkdir -p "$G"
git status --short -uno .🧬semio | sort > "$G/status-$1.txt"
echo "tracked-status-lines=$(wc -l < "$G/status-$1.txt") sha=$(shasum "$G/status-$1.txt" | cut -c1-12)"
[ -e "$G/marker" ] || touch "$G/marker"
find .🧬semio/🦑️repo/🎯️goals .🧬semio/🦑️repo/🎫️tickets -type f -newer "$G/marker" -not -path "$C/*" 2>/dev/null | head -20 > "$G/newer-$1.txt"
echo "files-newer-than-marker-outside-C=$(wc -l < "$G/newer-$1.txt")"
