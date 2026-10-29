#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
SCRIPT="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp" DELIVER_PROFILE=480p15
lang=$1; tag=$2; shift 2
args=(); for v in "$@"; do args+=(--video "$v"); done
bun "$SCRIPT" render "${args[@]}" --language "$lang" --audience speaker --quality l --fps 15 --no-play --log-output \
  > "$T/🗑️generated/render/render-$tag-$lang.log" 2>&1 || { echo "FAIL render $lang"; exit 1; }
"$PY" "$T/render/deliver.py" "$lang" speaker "$@" > "$T/🗑️generated/render/deliver-$tag-$lang.log" 2>&1 || { echo "FAIL deliver $lang"; exit 1; }
echo DONE $lang $tag
