#!/bin/sh
# usage: sh regress.sh MIRROR OUTFILE SUITE...   runs existing suites (paths relative to the library) one process at a time and appends "suite<TAB>test<TAB>result" rows
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
M=$1; OUT=$2; shift 2
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
mkdir -p /private/tmp/pm-seam/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/mint.ts")
cd "$M"
for suite in "$@"; do
  while [ "$(sysctl -n vm.loadavg | awk '{print int($2)}')" -ge 30 ]; do sleep 20; done
  SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/pm-seam/art perl -e 'alarm 540; exec @ARGV' -- nice -n 10 bun test --timeout 240000 "./$L/$suite" 2>&1 | sed 's/ \[[0-9.]*ms\]//' | command grep -E '^(\((pass|fail)\)| *[0-9]+ (pass|fail)$)' | sed "s|^|$suite\t|" >> "$OUT"
done
