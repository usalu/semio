#!/bin/sh
# [farmid] Runs the harness snapshot like the executor seam does (bun --eval <stream loader> -- <digest>, snapshot as descriptor 3, a regular file) in the foreground, niced; store in the session scratch dir; json/rows/log in 🗑️generated/farmid/. usage: run.sh <label> <image dir> [VAR=value ...]. Not a library file.
T=$(cd "$(dirname "$0")" && pwd); G="$T/../🗑️generated/farmid"; label=$1; img=$2; shift 2
BUN=$(ls -d /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
STORE_BASE=${STORE_BASE:-/private/tmp/claude-501/-Users-ueli-Documents-semio/018e7755-a677-4898-8d08-6b4d960f0145/scratchpad/store}
mkdir -p "$G" "$STORE_BASE"; ulimit -n 8192
R=$(cat "$T/../buildmem-tools/refresh-roots.txt"); DATA_ROOT=${DATA_ROOT:-$(cd /var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/${FI_MIRROR:-fiC} && pwd -P)}
start=$(python3 -c 'import time;print(time.time())')
env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_SCRIPT_STALL_MS GRAM_ROOT="$DATA_ROOT" LIST_ROOT=/Users/ueli/Documents/semio STORE="$STORE_BASE/$label" OUT="$G/$label.json" ROWS_OUT="$G/$label.rows.json" "ONLY_ROOTS=$R" SEMIO_LANGUAGE=en SEMIO_GRAPH_PHASES=1 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 BUN_JSC_forceRAMSize=2147483648 "$@" \
  nice -n 10 "$BUN" --eval "$(cat "$img/loader.js")" -- "$(cat "$img/digest")" harness 3<"$img/snapshot.json" > "$G/$label.log" 2>&1
rc=$?
echo "rc $rc wall $(python3 -c "import time;print(round(time.time()-$start,1))")"
/usr/bin/grep -E "graph:|HARNESS|refused|identity|rror" "$G/$label.log" | cut -c1-600 | tail -25
rm -rf "$STORE_BASE/$label"
