#!/usr/bin/env bash
# Batched twins generation and strict check for the schema-leftovers artifacts (serial, one heavy lease each).
T=$(cd "$(dirname "$0")/.." && pwd); H="$T/🗑️generated/build-runner/heavy.sh"; cd /home/deck/semio
for a in 🌍️en1997 🧩️en1994 🫨️en1998 🪶️en1999; do
  U="✏️s/🔌️plugins/📕️norm/🗿️artifacts/$a"
  echo "== generate $a"; "$H" nice -n 19 repo ./📜️script.ts schema twins generate --under "$U" 2>&1 | tail -5
  echo "== check $a"; "$H" nice -n 19 repo ./📜️script.ts schema twins check --strict --under "$U" 2>&1 | tail -8
  echo "exit-$a=${PIPESTATUS[0]}"
done
echo DONE
