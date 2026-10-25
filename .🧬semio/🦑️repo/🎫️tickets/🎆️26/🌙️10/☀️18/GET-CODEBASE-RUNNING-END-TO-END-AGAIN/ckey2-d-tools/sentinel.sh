#!/bin/sh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated"
find "$G" -maxdepth 3 -type f \( -name '*.log' -o -name '*.raw' -o -name '*.txt' \) -mmin -${1:-15} 2>/dev/null | while read f; do
  /usr/bin/grep -l -E "Uncaptured implementation module refused|Undeclared compiler-captured physical|Captured implementation Source image required|implementation scope required|Uncaptured require namespace" "$f" 2>/dev/null
done
