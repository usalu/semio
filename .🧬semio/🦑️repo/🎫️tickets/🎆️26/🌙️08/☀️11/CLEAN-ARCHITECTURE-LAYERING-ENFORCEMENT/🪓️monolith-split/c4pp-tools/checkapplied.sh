#!/bin/zsh
# usage: checkapplied.sh <name> <subset dir name>   new crates <subset> --check-applied for one stdio semio subset (sandbox under /Users/ueli/.cache/c4pp/ca-<name>)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets'
n=$1; d=$2
cd /Users/ueli/Documents/semio || exit 2
rm -rf "/Users/ueli/.cache/c4pp/ca-$n"; mkdir -p "/Users/ueli/.cache/c4pp/ca-$n"
ulimit -n 65536
nice -n 10 perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "$S/$d/📦️packages/🦀️rust" --check-applied --sandbox="/Users/ueli/.cache/c4pp/ca-$n" > "$F/🗑️generated/c4pp-checkapplied-$n.log" 2>&1
echo "$n exit=$?" >> "$F/🗑️generated/c4pp-checkapplied.exit"
