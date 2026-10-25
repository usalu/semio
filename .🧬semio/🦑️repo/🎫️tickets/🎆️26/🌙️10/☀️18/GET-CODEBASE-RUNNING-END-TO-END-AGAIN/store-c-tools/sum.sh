#!/bin/sh
# usage: sum.sh <log> : compile errors, test results, failed test names
for f in "$@"; do
echo "== $f"
echo "compile errors: $(/usr/bin/grep -c ': error\[' "$f")  could-not-compile: $(/usr/bin/grep -c '^error: could not compile' "$f")"
/usr/bin/grep -E '^test result' "$f" | cut -c1-160
/usr/bin/grep -E '^test .* FAILED$' "$f" | cut -c1-200 | sort -u | head -${N:-40}
done
