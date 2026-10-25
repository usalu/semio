#!/bin/sh
# usage: chain.sh <crates-file> [verdicts-file]  (lines: <workspace Cargo.toml><TAB><package>); one crate at a time.
# per crate: `--lib sqlite_snapshot`, then `--tests sqlite_snapshot` when the lib ran no matching test and compiled clean.
# verdict row: crate TAB target TAB passed TAB failed TAB errorLines ; raw logs in C/🗑️generated/plugc-d/crate-<pkg><target>.raw
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plugc-d"
V="${2:-$G/verdicts.tsv}"
TAB=$(printf '\t')
export SEMIO_BUILD_TERMINAL=false
while IFS="$TAB" read -r manifest crate; do
  [ -n "$crate" ] || continue
  grep -q "^$crate$TAB" "$V" 2>/dev/null && continue
  mp="--manifest-path $manifest"
  for target in --lib --tests; do
    log="$G/crate-$crate$target.raw"
    # shellcheck disable=SC2086
    env -u SEMIO_SCRIPT_STALL_MS nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test $mp -p "$crate" $target sqlite_snapshot --no-fail-fast --message-format=short > "$log" 2>&1
    passed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\1/' | awk '{s+=$1} END{print s+0}')
    failed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\2/' | awk '{s+=$1} END{print s+0}')
    built=$(grep -c ': error\(\[\|:\)' "$log")
    if [ "$passed" != 0 ] || [ "$failed" != 0 ] || [ "$built" != 0 ]; then break; fi
  done
  printf '%s\t%s\tpassed=%s\tfailed=%s\terrorLines=%s\n' "$crate" "$target" "$passed" "$failed" "$built" >> "$V"
done < "$1"
echo chain-done >> "$V"
