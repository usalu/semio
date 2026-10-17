#!/bin/zsh
# usage: rf.sh <dir>   rustfmt parse check (skip_children) of every 🦀️.rs below <dir>
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
n=0; bad=0
find "$1" -name "🦀️.rs" | while read f; do
  rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true "$f" > /dev/null 2> /Users/ueli/.cache/c4p34/rf.err || { echo "FAIL $f"; head -3 /Users/ueli/.cache/c4p34/rf.err; }
done
echo "rustfmt parse done: $(find "$1" -name '🦀️.rs' | wc -l | tr -d ' ') files"
