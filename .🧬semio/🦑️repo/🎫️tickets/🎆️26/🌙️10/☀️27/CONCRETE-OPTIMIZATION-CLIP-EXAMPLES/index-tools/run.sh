#!/bin/bash
# usage: run.sh <lang> [keys...]  captures the index scene and prints the last error lines
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES"
lang=$1; shift
timeout 900 ".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe" "$T/capture.py" ⚖️index Beat4_NormalizedIndex "$lang" "$@" > "$T/🗑️generated/index/run-$lang.log" 2>&1
tr '\r' '\n' < "$T/🗑️generated/index/run-$lang.log" | grep -v "it/s\]" | grep -v "^$" | tail -${TAILN:-6}
