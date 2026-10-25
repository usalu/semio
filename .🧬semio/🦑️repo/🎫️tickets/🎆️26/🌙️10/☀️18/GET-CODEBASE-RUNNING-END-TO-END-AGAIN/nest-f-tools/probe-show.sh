#!/bin/sh
# usage: probe-show.sh <log-prefix> <pinned|plain> <project>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
if [ "$2" = pinned ]; then
  PIN=$(nice -n 10 bun "$C/nest-f-tools/make-pin.ts" /Users/ueli/Documents/semio 1 $COVERED) || exit 2
  export SEMIO_BUILD_INVOCATION_PID=1 SEMIO_BUILD_GENERATION_PIN="$PIN"
fi
start=$(date +%s)
nice -n 10 bun repo show project "$3" > "$C/🗑️generated/nest-f/$1.out" 2> "$C/🗑️generated/nest-f/$1.err"
echo "rc=$? seconds=$(( $(date +%s) - start ))" > "$C/🗑️generated/nest-f/$1.rc"
