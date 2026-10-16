#!/bin/sh
# Usage: rulecount.sh <err.txt>  -> lines "count rule" sorted
sed -n 's/^  error \([^:]*\):.*/\1/p' "$1" | sort | uniq -c | sort -rn
