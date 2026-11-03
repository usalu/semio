#!/bin/bash
# crop.sh <Scene> <scale> <out-name> <t> <box> [<t> <box>...]
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/sl"
s="$1"; sc="$2"; o="$3"; shift 3
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/🧰️sl/crop.py" "$G/mp4/$s.mp4" "$G/crops-$o.png" "$sc" "$@"
