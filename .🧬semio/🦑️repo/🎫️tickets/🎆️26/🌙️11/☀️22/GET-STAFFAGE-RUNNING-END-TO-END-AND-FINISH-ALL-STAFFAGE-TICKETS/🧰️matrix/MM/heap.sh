#!/bin/sh
# ⚖️ Compares the young-generation size of the asset process inside ONE capacity grant: heap.sh <memory MB> <asset id> <suite dir or live> <MB> [<MB> …]
# Runs one.ts on the frozen asset once per size (0 = the runtime's default) with BUN_JSC_largeHeapSize set, under /usr/bin/time -l; logs in <generated>/heap-<asset>-<tag>-<MB>.log, where <tag> is the last folder of the suite dir's parent or `live`.
# The user seconds of the lines compare the processor time, the resident peak the memory.
set -u
R=/Users/ueli/Documents/semio
M="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MM"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🗑️generated/matrix/MM"
SP="${SP:-/private/tmp/claude-501/-Users-ueli-Documents-semio/77b365fc-dfc7-4b6f-9d54-4f0eb23231c9/scratchpad}"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
if [ "${1:-}" != "--inside" ]; then
  cd "$R" || exit 1
  memory=$1; shift
  exec repo capacity run --memory="$memory" -- sh "$M/heap.sh" --inside "$@"
fi
shift
asset=$1; suite=$2; shift 2
mkdir -p "$G"
cd "$R" || exit 1
if [ "$suite" = "live" ]; then tag=live; set -- "$@"; suiteflag=""; else tag=$(basename "$(dirname "$(dirname "$suite")")"); suiteflag="--suite $suite"; fi
for mb in "$@"; do
  log="$G/heap-$asset-$tag-$mb.log"
  if [ "$mb" = "0" ]; then
    /usr/bin/time -l timeout 3000 "$BUN" "$M/one.ts" --snapshot "$SP/snap/$asset.json" --out "$G/heap-$asset-$tag-$mb.cells.json" $suiteflag > "$log" 2>&1
  else
    BUN_JSC_largeHeapSize=$(( mb * 1048576 )) /usr/bin/time -l timeout 3000 "$BUN" "$M/one.ts" --snapshot "$SP/snap/$asset.json" --out "$G/heap-$asset-$tag-$mb.cells.json" $suiteflag > "$log" 2>&1
  fi
  echo "$asset $tag $mb MB: $(grep -E 'maximum resident| real ' "$log" | tr -s ' ' | tr '\n' ';')"
done
