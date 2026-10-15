#!/bin/zsh
# usage: oracle.sh <root> <label> [rustc args]  -> $OUT/<label>.short
root=$1; label=$2; shift 2
OUT=${OUT:-/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws/orc}
mkdir -p $OUT
cd $root/🌎️hub/📦️packages/🦀️rust
rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_hub --emit=metadata -o /dev/null --error-format=short -W unused-imports "$@" 🦀️.rs 2> $OUT/$label.raw
sed -E 's#^(\.\./)*##; s#^/Users/ueli/Documents/semio/##; s#^.*/sbx-[a-z]+/##; s#^([^:]+):[0-9]+:[0-9]+:#\1:#' $OUT/$label.raw | sort > $OUT/$label.norm
echo "$label: $(/usr/bin/grep -c ': error' $OUT/$label.raw) errors, $(/usr/bin/grep -c ': warning' $OUT/$label.raw) warnings"
