#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t4/target CARGO_INCREMENTAL=0
out=.tmp-ticket/wp-t4/generated/check-crates-$1.txt
: > $out
shift
for spec in "$@"; do
  echo "=== $spec" >> $out
  eval "cargo check $spec --message-format short" >> $out 2>&1
  echo "=== exit $? $spec" >> $out
done
echo ALLDONE >> $out
