#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
N="$R/✏️s/🔌️plugins/📕️norm"
for d in 🏋️en1991 🏛️en1992 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996 🌍️en1997 🫨️en1998 🪶️en1999 🧱️din4108 🌬️din16798 ⚡️din18599 📇️iso16757 🏭️vdi3805 ROOT; do
  w="$N/🗿️artifacts/$d"; [ "$d" = ROOT ] && w="$N"
  code=${d#*[^a-z0-9]}; code=$(echo "$d" | sed 's/[^a-z0-9]//g'); [ "$d" = ROOT ] && code=root
  cd "$w" && repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --all-targets --keep-going > "$T/🗑️generated/norm-rs-$code-check.log" 2>&1
  echo "$code exit $? errors $(grep -c '^error' "$T/🗑️generated/norm-rs-$code-check.log")"
done
