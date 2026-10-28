#!/bin/sh
# bench3-g staged batch `observer`: the Bazel process-summary pattern accepts the singular ("INFO: 1 process: 1 internal." = a hot no-op, 0 executed) + 2 fixture rows.
# usage: sh apply.sh check|apply|revert   (check = base hashes still equal; apply copies new/, revert copies base/; both refuse on drift)
H=$(cd "$(dirname "$0")" && pwd)
R=/Users/ueli/Documents/semio
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench"
FILES="🧬️schema/🔣️.json 🧪️tests/🧫️fixtures/🔣️.json"
same() { cmp -s "$1" "$2"; }
state=base
for f in $FILES; do
  if same "$R/$B/$f" "$H/base/$f"; then :; elif same "$R/$B/$f" "$H/new/$f"; then state=new; else echo "DRIFT $f"; exit 2; fi
done
case $1 in
  check) echo "state: $state"; for f in $FILES; do same "$R/$B/$f" "$H/base/$f" && echo "base $f" || echo "new  $f"; done;;
  apply) [ $state = new ] && { echo "already applied"; exit 0; }; for f in $FILES; do cp "$H/new/$f" "$R/$B/$f"; done; echo applied;;
  revert) [ $state = base ] && { echo "already base"; exit 0; }; for f in $FILES; do cp "$H/base/$f" "$R/$B/$f"; done; echo reverted;;
  *) echo "usage: sh apply.sh check|apply|revert"; exit 1;;
esac
