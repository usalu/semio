#!/bin/zsh
# usage: run-diff.sh <label> <groups...>   scoped patched rustc differential (oracle.py) per group, lib, --test and wasm32-wasip2; C4P34_LIVE=1 compares the live tree
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/c4p34-tools" || exit 2
label=$1; shift
for g in "$@"; do
  perl -e 'alarm 2400; exec @ARGV' -- python3 oracle.py $g lib,test,p2 > "$F/🗑️generated/c4p34-diff-$label-$g.log" 2>&1
  echo "$g exit=$? $(tail -1 "$F/🗑️generated/c4p34-diff-$label-$g.log")" >> "$F/🗑️generated/c4p34-diff-$label.exit"
done
