#!/bin/sh
# usage: chain2.sh <rows-file> [verdicts-file]  rows: manifest TAB package TAB kind(lib|test) TAB testTargetName TAB count
# One `cargo test -p pkg (--lib|--test name) sqlite_snapshot` per row, sequential; verdict row: pkg TAB kind TAB name TAB passed TAB failed TAB errorLines
# raw logs: C/🗑️generated/plugc-d/t-<pkg>-<kind><name>.raw
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plugc-d"
V="${2:-$G/verdicts2.tsv}"
TAB=$(printf '\t')
export SEMIO_BUILD_TERMINAL=false
while IFS="$TAB" read -r manifest pkg kind name count; do
  [ -n "$pkg" ] || continue
  grep -q "^$pkg$TAB$kind$TAB$name$TAB" "$V" 2>/dev/null && continue
  if [ "$kind" = lib ]; then tsel="--lib"; else tsel="--test $name"; fi
  log="$G/t-$pkg-$kind$name.raw"
  # shellcheck disable=SC2086
  env -u SEMIO_SCRIPT_STALL_MS nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$manifest" -p "$pkg" $tsel sqlite_snapshot --no-fail-fast --message-format=short > "$log" 2>&1
  passed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\1/' | awk '{s+=$1} END{print s+0}')
  failed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\2/' | awk '{s+=$1} END{print s+0}')
  built=$(grep -c ': error\(\[\|:\)' "$log")
  printf '%s\t%s\t%s\tpassed=%s\tfailed=%s\terrorLines=%s\tof=%s\n' "$pkg" "$kind" "$name" "$passed" "$failed" "$built" "$count" >> "$V"
done < "$1"
echo chain-done >> "$V"
