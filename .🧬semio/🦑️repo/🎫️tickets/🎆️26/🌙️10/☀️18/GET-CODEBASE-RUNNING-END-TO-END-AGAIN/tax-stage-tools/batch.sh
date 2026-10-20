#!/bin/sh
# usage: batch.sh live|mirror|base <list-name> [seconds-per-file]   (base = pristine mirror /private/tmp/tsb, mirror = patched /private/tmp/tsm)   (list under 🗑️generated/tax-stage/<list-name>.txt; output <mode>-<list-name>.txt)
C="$(cd "$(dirname "$0")/.." && pwd)"; G="$C/🗑️generated/tax-stage"; 
case "$1" in base) S=/private/tmp/tsb; MODE=mirror;; mirror) S=/private/tmp/tsm; MODE=mirror;; *) S=/private/tmp/tsm; MODE=live;; esac
MAXFAIL=100 perl -e 'alarm 590; exec @ARGV' -- "$C/tax-stage-tools/run-tests.sh" "$MODE" "$S" "${3:-70}" $(cat "$G/$2.txt" | tr '\n' ' ') > "$G/$1-$2.txt" 2>&1
grep -E "^(live|mirror)" "$G/$1-$2.txt" | awk '{print $1,$2,$3,$4,$5}' | sort | uniq -c | sort -rn | head -3
