#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997" || exit 1
timeout 2700 repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1997 --offline > "$T/🗑️generated/en1997-family.log" 2>&1
echo "exit $?" > "$T/🗑️generated/en1997-family.status"
