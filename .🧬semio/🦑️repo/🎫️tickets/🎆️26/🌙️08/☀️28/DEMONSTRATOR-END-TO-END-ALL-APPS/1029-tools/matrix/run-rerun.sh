#!/bin/bash
# Second attempt (-r2) for batches that failed only in shared framework crates or in transient infrastructure (ENOSPC, jobserver fifo). Runs after run-extra.sh finishes so the tree is as current as possible.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
T="$C/1029-tools/matrix"
while [ ! -f "$G/matrix-extra.rc" ]; do sleep 15; done
cd "$ROOT" || exit 1
EXT="✏️s/🔌️plugins"
chk2(){
  name=$1-r2; ws=$2; shift 2
  rm -f "$G/matrix-$name.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$ws" --target wasm32-wasip2 --keep-going "$@" > "$G/matrix-$name.txt" 2>&1
  echo $? > "$G/matrix-$name.rc"
}
source "$T/rerun-lines.sh"
echo done > "$G/matrix-rerun.rc"
