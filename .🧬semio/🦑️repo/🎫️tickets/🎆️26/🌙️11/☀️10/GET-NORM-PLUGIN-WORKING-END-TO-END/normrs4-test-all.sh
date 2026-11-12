#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/home/deck/semio/✏️s/🔌️plugins/📕️norm"
R="repo cargo --store=shared --configuration=prototype --parallelism=4 --"
for d in 🪶️en1999 🧱️din4108 🌬️din16798 ⚡️din18599 📇️iso16757 🏭️vdi3805 🫨️en1998 ⚖️en1990 🏋️en1991 🏛️en1992 🔩️en1993 🧩️en1994 🪵️en1995 🌍️en1997 ROOT 🪨️en1996; do
  w="$N/🗿️artifacts/$d"; code=$(echo "$d" | sed 's/[^a-z0-9]//g'); [ "$d" = ROOT ] && w="$N" && code=root
  cd "$w" || continue
  $R check --all-targets --keep-going > "$G/normrs4-$code-check.log" 2>&1; echo "$code check exit $? errors $(grep -c '^error' "$G/normrs4-$code-check.log")"
  repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 > "$G/normrs4-$code-metadata.json" 2>/dev/null
  flags=$(python3 "$T/norm-rs-testable.py" "$G/normrs4-$code-metadata.json" "$G/normrs4-$code-check.log" 2>"$G/normrs4-$code-testable.txt")
  echo "$code testable $(cat "$G/normrs4-$code-testable.txt")"
  [ -n "$flags" ] && $R test $flags --no-fail-fast > "$G/normrs4-$code-test.log" 2>&1; echo "$code test exit $?"
  grep -E '^test result' "$G/normrs4-$code-test.log" | awk '{p+=$4;f+=$6} END{print "'$code' passed",p,"failed",f}'
done
