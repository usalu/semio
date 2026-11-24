#!/bin/zsh
cd "$(dirname "$0")"
for n in 1 2 3 4 5 6 7 8; do
  ./run-cargo.sh r$n$1 "${@:2}" > "🗑️generated/run-r$n$1.exit" 2>&1
  grep -q "^exit 0" "🗑️generated/run-r$n$1.exit" && { echo done r$n$1 > 🗑️generated/retry-$1.done; exit 0; }
  grep -aq "^error\[E0616\]\|^error\[E0599\]\|^error\[E0308\]\|^error\[E0432\]\|^error: could not compile" "🗑️generated/run-r$n$1.txt" || { echo failed-test r$n$1 > 🗑️generated/retry-$1.done; exit 1; }
  sleep 240
done
echo gave-up > 🗑️generated/retry-$1.done
