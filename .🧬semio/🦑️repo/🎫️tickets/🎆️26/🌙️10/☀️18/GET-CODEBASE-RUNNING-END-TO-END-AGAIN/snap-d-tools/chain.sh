#!/bin/sh
# usage: chain.sh <crates-file>  (lines: <workspace Cargo.toml or -><TAB><package>); one crate at a time; rows -> C/🗑️generated/snap-d/verdicts.tsv
# per crate: `--lib sqlite_snapshot`, then `--tests sqlite_snapshot` when the lib has no matching test and no error
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/snap-d"
TAB=$(printf '\t')
while IFS="$TAB" read -r manifest crate; do
  [ -n "$crate" ] || continue
  grep -q "^$crate$TAB" "$G/verdicts.tsv" 2>/dev/null && continue
  if [ "$manifest" = "-" ]; then mp=""; else mp="--manifest-path $manifest"; fi
  for target in --lib --tests; do
    log="$G/crate-$crate$target.raw"
    # shellcheck disable=SC2086
    sh "$C/snap-d-tools/cargo.sh" "$crate" $mp -p "$crate" $target sqlite_snapshot --no-fail-fast > "$log" 2>&1
    passed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\1/' | awk '{s+=$1} END{print s+0}')
    failed=$(grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\2/' | awk '{s+=$1} END{print s+0}')
    built=$(grep -c '^error\(\[\|:\)' "$log")
    if [ "$passed" != 0 ] || [ "$failed" != 0 ] || [ "$built" != 0 ]; then break; fi
  done
  printf '%s\t%s\tpassed=%s\tfailed=%s\terrorLines=%s\n' "$crate" "$target" "$passed" "$failed" "$built" >> "$G/verdicts.tsv"
done < "$1"
echo chain-done >> "$G/verdicts.tsv"
