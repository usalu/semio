#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/home/deck/semio/✏️s/🔌️plugins/📕️norm"
R="repo cargo --store=shared --configuration=prototype --parallelism=4 --"
for d in "$@"; do
  w="$N/🗿️artifacts/$d"; code=$(echo "$d" | sed 's/[^a-z0-9]//g'); [ "$d" = ROOT ] && w="$N" && code=root
  cd "$w" || continue
  $R check --all-targets --keep-going > "$G/normrs3r-$code-check.log" 2>&1; echo "$code check exit $? errors $(grep -c '^error' "$G/normrs3r-$code-check.log")"
  repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 > "$G/normrs3r-$code-metadata.json" 2>/dev/null
  flags=$(python3 "$T/norm-rs-testable.py" "$G/normrs3r-$code-metadata.json" "$G/normrs3r-$code-check.log" 2>"$G/normrs3r-$code-testable.txt")
  echo "$code testable $(cat "$G/normrs3r-$code-testable.txt")"
  [ -n "$flags" ] && $R test $flags --no-fail-fast > "$G/normrs3r-$code-test.log" 2>&1; echo "$code test exit $?"
  grep -E '^test result' "$G/normrs3r-$code-test.log" | awk '{p+=$4;f+=$6} END{print "'$code' passed",p,"failed",f}'
done
