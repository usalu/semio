#!/bin/zsh
# DB1: run ONE db lib law in place directly from the last built test binary (no cargo), optional `sample` of the storm.
# usage: db-run.sh <exact law path> [--sample <capture>] ; binary = last `Executable` line of s13 build logs.
cd /Users/ueli/Documents/semio
export RUST_MIN_STACK=268435456
law="$1"; shift
sample_to=""
if [[ "$1" == "--sample" ]]; then sample_to="$2"; shift 2; fi
bin=$(/usr/bin/grep -h "Executable" .🧬semio/🌐hub/s13-db1-logs/build-*.txt | tail -1 | sed -E 's/.*\((.*)\)/\1/')
export SEMIO_DB_ISOLATED_LAW="$law"
echo "=== law $law start $(date +%T) bin $bin"
nice -n 10 "$bin" --exact "$law" --nocapture --include-ignored --test-threads=1 "$@" > "$TMPDIR/db1-law-$$.txt" 2>&1 &
pid=$!
if [[ -n "$sample_to" ]]; then
  until /usr/bin/grep -q "storm of .* documents begins" "$TMPDIR/db1-law-$$.txt" 2>/dev/null || ! kill -0 $pid 2>/dev/null; do sleep 0.2; done
  /usr/bin/sample $pid 1 1 -file "$sample_to" > /dev/null 2>&1
fi
wait $pid; rc=$?
/usr/bin/grep -E "throughput\[|test result|panicked|greeting storm|^\[" "$TMPDIR/db1-law-$$.txt" | head -80
/usr/bin/grep -A2 "panicked" "$TMPDIR/db1-law-$$.txt" | head -12
rm -f "$TMPDIR/db1-law-$$.txt"
echo "LAW EXIT $rc $(date +%T)"
