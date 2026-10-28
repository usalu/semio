#!/bin/sh
# [mem3] Runs the instrumented harness snapshot like the executor seam (bun --eval <marked stream loader> -- <digest>, snapshot as descriptor 3, a regular file), foreground, niced; json/log in 🗑️generated/mem3/. usage: run.sh <label> <image dir> [VAR=value ...] (defaults: in-process, SEMIO_GRAPH_WORKERS=0; JSC RAM 2 GiB as the graph child). Not a library file.
T=$(cd "$(dirname "$0")" && pwd); G="$T/../🗑️generated/mem3"; label=$1; img=$2; shift 2
BUN=$(ls -d /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
STORE_BASE=${STORE_BASE:-/private/tmp/claude-501/-Users-ueli-Documents-semio/018e7755-a677-4898-8d08-6b4d960f0145/scratchpad/m3store}
mkdir -p "$G" "$STORE_BASE"; ulimit -n 8192
R=${M2_ROOTS:-$(cat "$T/../buildmem-tools/refresh-roots.txt")}; DATA_ROOT=${DATA_ROOT:-$(cd /var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/${M2_MIRROR:-m2A} && pwd -P)}
start=$(python3 -c 'import time;print(time.time())')
env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_SCRIPT_STALL_MS GRAM_ROOT="$DATA_ROOT" LIST_ROOT=/Users/ueli/Documents/semio STORE="$STORE_BASE/$label" OUT="$G/$label.json" ROWS_OUT="$G/$label.rows.json" "ONLY_ROOTS=$R" SEMIO_LANGUAGE=en SEMIO_GRAPH_PHASES=1 SEMIO_GRAPH_WORKERS=0 SEMIO_HOST_CAPACITY_PLAN='{"jobs":8,"memoryMegabytes":8190}' BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 BUN_JSC_forceRAMSize=2147483648 "$@" \
  nice -n 10 "$BUN" --eval "$(cat "$img/loader.marked.js")" -- "$(cat "$img/digest")" harness 3<"$img/snapshot.json" > "$G/$label.log" 2>&1
rc=$?
echo "rc $rc wall $(python3 -c "import time;print(round(time.time()-$start,1))")"
/usr/bin/grep -E "graph:|HARNESS|refused|identity|rror|M2 " "$G/$label.log" | cut -c1-400 | tail -40
rm -rf "$STORE_BASE/$label"
