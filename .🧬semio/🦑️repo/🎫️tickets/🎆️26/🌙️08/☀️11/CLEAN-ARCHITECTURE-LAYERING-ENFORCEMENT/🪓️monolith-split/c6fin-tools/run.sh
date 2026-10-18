#!/bin/zsh
cd "$(dirname "$0")"
for t in "$@"; do
  python3 modsplit.py stage $t 2>&1 | tail -3
  python3 modsplit.py lossless $t 2>&1 | command grep -v "^  +" | head -30
  python3 modsplit.py paths $t 2>&1 | command grep -i "too long" 
  python3 vmod.py stage $t 2>&1 | tail -8
  python3 defscan.py $t 2>&1 | tail -4
  python3 ambig.py stage $t 2>&1 | tail -4
done
