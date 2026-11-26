#!/bin/sh
# usage: check-census.sh <tag>  (prints, per component crate of a phase-1 run, the failing crates and the distinct compiler errors, message only)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tag=${1:?tag}
for log in "$here/🗑️generated/$tag"/*.log; do
  echo "== $(basename "$log" .log)"
  grep -a 'could not compile' "$log" | sed 's/^error: could not compile `\([^`]*\)`.*/  crate \1/' | sort -u
  grep -a -E ': error(\[E[0-9]+\])?: ' "$log" | sed -E 's/^.*: (error(\[E[0-9]+\])?: .*)$/  \1/' | cut -c1-200 | sort | uniq -c | sort -rn | head -8
done
