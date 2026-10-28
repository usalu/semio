#!/bin/bash
cd /c/git/semio
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES"
rm -f "$T/🗑️generated/cap_$1.txt"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/capture.py" 🔗️consistency Beat6_Consistency $1 > "$T/🗑️generated/cap_$1.txt" 2>&1
echo done >> "$T/🗑️generated/cap_$1.txt"
