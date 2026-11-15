#!/usr/bin/env bash
# usage: wave-tests-ws.sh <log> <check|test> <family-dir-name> [extra cargo args]
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
L="$1"; M="$2"; D="$3"; shift 3
cd "$R/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"*"$D" || exit 1
if [ "$M" = check ]; then
  repo cargo --store=shared --configuration=prototype --parallelism=3 -- check --tests --keep-going --workspace "$@" > "$T/🗑️generated/$L.log" 2>&1
else
  repo cargo --store=shared --configuration=prototype --parallelism=3 -- test --no-fail-fast --workspace "$@" > "$T/🗑️generated/$L.log" 2>&1
fi
echo "exit $?" >> "$T/🗑️generated/$L.log"
