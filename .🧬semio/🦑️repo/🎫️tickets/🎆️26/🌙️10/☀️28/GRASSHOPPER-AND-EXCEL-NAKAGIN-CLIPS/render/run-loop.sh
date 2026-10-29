#!/usr/bin/env bash
# usage: run-loop.sh <lang> <tag> <video-id>...  one video at a time, retried while sources keep changing
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
SCRIPT="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp" DELIVER_PROFILE=480p15
lang=$1; tag=$2; shift 2
for v in "$@"; do
  ok=0
  for try in 1 2 3 4 5 6; do
    if bun "$SCRIPT" render --video "$v" --language "$lang" --audience speaker --quality l --fps 15 --no-play --log-output \
        > "$T/🗑️generated/render/render-$tag-$lang-$v.log" 2>&1; then
      "$PY" "$T/render/deliver.py" "$lang" speaker "$v" > "$T/🗑️generated/render/deliver-$tag-$lang-$v.log" 2>&1 && ok=1
      break
    fi
    echo "retry $v $lang $try"
  done
  echo "RESULT $v $lang ok=$ok $(date +%T)"
done
echo DONE $lang $tag
