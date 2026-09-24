#!/bin/zsh
# WP-T9: run `cargo test --lib` for each `crate[:features]` line of $1, one at a time; summary to $2.
cd /Users/ueli/Documents/semio
: > "$2"
for spec in $(cat "$1"); do
  crate="${spec%%:*}"
  features=""
  [[ "$spec" == *:* ]] && features="--features ${spec#*:}"
  CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t9/target cargo test -p "$crate" ${=features} --lib > ".tmp-ticket/wp-t9/generated/batch-$crate.txt" 2>&1
  echo "$spec exit=$? $(grep -a 'test result' .tmp-ticket/wp-t9/generated/batch-$crate.txt | tail -1) $(grep -ac '^error' .tmp-ticket/wp-t9/generated/batch-$crate.txt) errors" >> "$2"
done
echo DONE >> "$2"
