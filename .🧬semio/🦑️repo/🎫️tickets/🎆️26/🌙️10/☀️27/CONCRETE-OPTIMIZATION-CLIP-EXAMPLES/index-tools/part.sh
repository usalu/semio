#!/bin/bash
# usage: part.sh <lang> <method[,method]> [keys...]  runs only some screens of the index scene
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES"
lang=$1; methods=$2; shift; shift
timeout 900 ".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe" "$T/index-tools/partial.py" ⚖️index Beat4_NormalizedIndex "$lang" "$methods" "$@" > "$T/🗑️generated/index/part-$lang.log" 2>&1
tr '\r' '\n' < "$T/🗑️generated/index/part-$lang.log" | grep -v "it/s\]" | grep -v "^$" | tail -${TAILN:-6}
