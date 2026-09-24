#!/bin/zsh
# WP-T3: run `cargo test --lib` for each crate listed in $1, one at a time; summary to $2.
cd /Users/ueli/Documents/semio
: > "$2"
for crate in $(cat "$1"); do
  CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t3/target cargo test -p "$crate" --lib > ".tmp-ticket/wp-t3/generated/batch-$crate.txt" 2>&1
  echo "$crate exit=$? $(grep -a 'test result' .tmp-ticket/wp-t3/generated/batch-$crate.txt | tail -1) $(grep -ac '^error' .tmp-ticket/wp-t3/generated/batch-$crate.txt) errors" >> "$2"
done
echo DONE >> "$2"
