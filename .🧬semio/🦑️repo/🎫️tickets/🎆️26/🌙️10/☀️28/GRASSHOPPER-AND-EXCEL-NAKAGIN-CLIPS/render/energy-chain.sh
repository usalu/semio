#!/usr/bin/env bash
# usage: energy-chain.sh <jobs> <video-id>...   renders the energy course videos one after another through the energy script
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
SCRIPT="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp"
jobs=$1; shift
for v in "$@"; do
  for attempt in 1 2 3; do
    bun "$SCRIPT" render --video "$v" --language en --audience speaker --quality l --fps 15 --no-play --log-output --jobs "$jobs" \
      > "$T/🗑️generated/render/energy-$v-a$attempt.log" 2>&1 && { echo "RESULT $v ok attempt=$attempt $(date +%T)"; break; }
    echo "RESULT $v fail attempt=$attempt $(date +%T)"
  done
done
echo DONE
