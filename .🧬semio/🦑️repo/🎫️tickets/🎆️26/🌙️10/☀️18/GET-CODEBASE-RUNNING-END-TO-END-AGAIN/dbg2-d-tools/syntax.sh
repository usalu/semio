#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
bad=0
while IFS= read -r f; do
  case "$f" in
    *.ts|*.tsx) nice -n 10 bun build --no-bundle "$f" --outfile "$D/🗑️generated/dbg2-d/syn.js" >"$D/🗑️generated/dbg2-d/syn.log" 2>&1 || { echo "ERR $f"; head -8 "$D/🗑️generated/dbg2-d/syn.log"; bad=1; } ;;
  esac
done < "$1"
echo "syntax-done bad=$bad"
