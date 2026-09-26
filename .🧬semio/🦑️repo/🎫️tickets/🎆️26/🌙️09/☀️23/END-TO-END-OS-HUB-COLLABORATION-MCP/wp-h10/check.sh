#!/bin/zsh
# ✅️ H10: `cargo check -p <crate> [args]` at nice 15, printing only errors, warnings inside the given path filter, and the final line.
# usage: check.sh <crate> <path-filter-regex> [cargo args…]
CRATE=$1; FILTER=$2; shift 2
cd /Users/ueli/Documents/semio
OUT=$(CARGO_INCREMENTAL=0 nice -n 15 cargo check -p "$CRATE" "$@" --message-format short 2>&1)
rc=$?
echo "$OUT" | /usr/bin/grep -E "^error|: error" | head -40
echo "$OUT" | /usr/bin/grep -E "warning" | /usr/bin/grep -E "$FILTER" | head -30
echo "$OUT" | /usr/bin/grep -E "Finished|could not compile" | tail -3
echo "EXIT $rc"
