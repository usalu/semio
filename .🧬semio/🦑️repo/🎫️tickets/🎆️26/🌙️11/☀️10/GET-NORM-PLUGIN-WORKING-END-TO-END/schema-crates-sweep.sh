#!/bin/sh
# usage: schema-crates-sweep.sh family-dir-name crate-family  -> log in generated
T="$(cd "$(dirname "$0")" && pwd)"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
for pair in "$@"; do
  d="${pair%%:*}"; f="${pair##*:}"
  cd "$N/$d" || exit 1
  timeout 2400 repo cargo --store=shared --configuration=prototype -- test -p "semio-s-artifact-norm-$f-schema" --offline > "$T/🗑️generated/sweep-$f.log" 2>&1
  echo "$f exit $?" >> "$T/🗑️generated/sweep-status.log"
done
echo ALLDONE >> "$T/🗑️generated/sweep-status.log"
