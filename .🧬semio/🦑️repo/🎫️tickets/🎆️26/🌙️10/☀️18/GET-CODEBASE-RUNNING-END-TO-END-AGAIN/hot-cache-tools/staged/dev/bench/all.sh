#!/bin/sh
# runs every prototype measurement against a running service; appends JSON lines to $OUT
# usage: all.sh WORKSPACE NATIVE_DIR OUT LANGUAGES TARGET   (for example: all.sh ws bin results.jsonl en,de p1499:build)
set -e
R=${1:?workspace}; BIN=${2:?native directory}; OUT=${3:?output file}; LANGUAGES=${4:?languages}; TARGET=${5:?project:target}
cd "$(dirname "$0")"
: > "$OUT"
echo "{\"mode\":\"env\",\"load\":\"$(sysctl -n vm.loadavg 2>/dev/null || cut -d' ' -f1-3 /proc/loadavg)\"}" >> "$OUT"
nice -n 10 bun bench.mjs direct "$R" --n 500 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
nice -n 10 bun bench.mjs noop "$R" --n 60 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
nice -n 10 bun bench.mjs edit "$R" --n 40 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
nice -n 10 bun bench.mjs edit "$R" --n 40 --delay 150 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
nice -n 10 bun bench.mjs watch "$R" --n 20 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
nice -n 10 bun bench.mjs concurrency "$R" --clients 16 --per 40 --languages "$LANGUAGES" --target "$TARGET" >> "$OUT"
for i in 1 2 3 4 5; do nice -n 10 bun stateless.mjs "$R" "$TARGET" "$BIN" >> "$OUT"; done
nice -n 10 bun events.mjs native 200 20000 "$BIN" >> "$OUT"
nice -n 10 bun events.mjs portable 200 20000 "$BIN" >> "$OUT"
nice -n 10 bun replay.mjs "$BIN" >> "$OUT"
echo done >> "$OUT"
