#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/home/deck/semio/✏️s/🔌️plugins/📕️norm"
R="repo cargo --store=shared --configuration=prototype --parallelism=4 --"
for d in ⚖️en1990 🏋️en1991 🏛️en1992 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996 🌍️en1997 🫨️en1998 🪶️en1999 🧱️din4108 🌬️din16798 ⚡️din18599 📇️iso16757 🏭️vdi3805 ROOT; do
  w="$N/🗿️artifacts/$d"; code=$(echo "$d" | sed 's/[^a-z0-9]//g'); [ "$d" = ROOT ] && w="$N" && code=root
  cd "$w" || continue
  [ -s "$G/normrs2-$code-check.log" ] && [ "$code" = en1990 ] || $R check --all-targets --keep-going > "$G/normrs2-$code-check.log" 2>&1; echo "$code check exit $? errors $(grep -c '^error' "$G/normrs2-$code-check.log")"
  repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 > "$G/normrs2-$code-metadata.json" 2>/dev/null
  flags=$(python3 "$T/norm-rs-testable.py" "$G/normrs2-$code-metadata.json" "$G/normrs2-$code-check.log" 2>"$G/normrs2-$code-testable.txt")
  echo "$code testable $(cat "$G/normrs2-$code-testable.txt")"
  [ -n "$flags" ] && $R test $flags --no-fail-fast > "$G/normrs2-$code-test.log" 2>&1; echo "$code test exit $?"
  grep -E '^test result' "$G/normrs2-$code-test.log" | awk '{p+=$4;f+=$6} END{print "'$code' passed",p,"failed",f}'
done
