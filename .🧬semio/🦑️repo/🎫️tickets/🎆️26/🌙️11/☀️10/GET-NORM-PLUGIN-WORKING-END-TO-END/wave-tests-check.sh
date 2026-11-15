#!/usr/bin/env bash
# usage: wave-tests-check.sh <log> <check|test> <family...>
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R/🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust" || exit 1
L="$1"; M="$2"; shift 2
P=()
for f in "$@"; do while read -r n; do P+=(-p "$n"); done < "$T/🗑️generated/wave-tests-$f.pkgs"; done
if [ "$M" = check ]; then
  repo cargo --store=shared --configuration=prototype -- check --tests --keep-going "${P[@]}" > "$T/🗑️generated/$L.log" 2>&1
else
  repo cargo --store=shared --configuration=prototype -- test --no-fail-fast "${P[@]}" > "$T/🗑️generated/$L.log" 2>&1
fi
echo "exit $?" >> "$T/🗑️generated/$L.log"
