#!/usr/bin/env bash
# usage: run.sh <lang> <tag> <video-id>...   renders speaker then spectator, then delivers
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
SCRIPT="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp"
lang=$1; tag=$2; shift 2
args=(); for v in "$@"; do args+=(--video "$v"); done
for aud in speaker spectator; do
  bun "$SCRIPT" render "${args[@]}" --language "$lang" --audience "$aud" --quality h --fps 30 --no-play --log-output \
    > "$T/🗑️generated/render/render-$tag-$lang-$aud.log" 2>&1 || { echo "FAIL render $lang $aud"; exit 1; }
  "$PY" "$T/render/deliver.py" "$lang" "$aud" "$@" > "$T/🗑️generated/render/deliver-$tag-$lang-$aud.log" 2>&1 || { echo "FAIL deliver $lang $aud"; exit 1; }
done
echo DONE $lang $tag
