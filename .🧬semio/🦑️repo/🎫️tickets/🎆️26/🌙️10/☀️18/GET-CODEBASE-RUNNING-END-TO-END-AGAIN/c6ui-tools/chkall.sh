#!/bin/zsh
cd "$(dirname "$0")"
G=/usr/bin/grep
for t in "$@"; do
  echo "##### $t"
  python3 modsplit.py stage $t | tail -2
  python3 modsplit.py lossless $t | $G -E "UNEXPLAINED|^missing|LOSSLESS|regions"
  python3 vmod.py stage $t | tail -2
  python3 defscan.py $t | tail -2
  python3 ambig.py stage $t | tail -2
  python3 modclash.py $t
  python3 modsplit.py paths $t | $G -c "TOO LONG"
done
