#!/bin/sh
# runs every prototype measurement against a running service; appends JSON lines to $OUT
R=/private/tmp/hot-cache/ws1; OUT=${1:-/private/tmp/hot-cache/results.jsonl}; : > $OUT
cd /private/tmp/hot-cache/ov
echo "{\"mode\":\"env\",\"load\":\"$(sysctl -n vm.loadavg)\",\"cores\":$(sysctl -n hw.ncpu)}" >> $OUT
nice -n 10 bun bench/bench.mjs direct $R --n 500 >> $OUT
nice -n 10 bun bench/bench.mjs noop $R --n 60 >> $OUT
nice -n 10 bun bench/bench.mjs noop $R --n 100 --client rust >> $OUT
nice -n 10 bun bench/bench.mjs edit $R --n 40 --client rust >> $OUT
nice -n 10 bun bench/bench.mjs edit $R --n 40 --client rust --delay 150 >> $OUT
nice -n 10 bun bench/bench.mjs edit $R --n 20 >> $OUT
nice -n 10 bun bench/bench.mjs watch $R --n 20 >> $OUT
nice -n 10 bun bench/bench.mjs concurrency $R --clients 16 --per 40 >> $OUT
for i in 1 2 3 4 5; do nice -n 10 bun bench/legacy.mjs $R p1499:build >> $OUT; done
nice -n 10 bun bench/events.mjs native 200 20000 >> $OUT
nice -n 10 bun bench/events.mjs portable 200 20000 >> $OUT
nice -n 10 bun bench/replay.mjs >> $OUT
echo done >> $OUT
