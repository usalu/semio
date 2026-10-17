#!/bin/zsh
# usage: checkapplied-one.sh <label> <name>   one stdio artifact: `new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813`; retries while a peer's infra edit makes the script fail to load
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/Users/ueli/.cache/c4p34
label=$1; n=$2
typeset -A P
P=(png "📷️png" jpg "📸️jpg" avi "📼️avi" pptx "📽️pptx" dxf "🖋️dxf" deflate "🗜️deflate")
cd /Users/ueli/Documents/semio || exit 2
log="$F/🗑️generated/c4p34-checkapplied-$label-$n.log"
for attempt in 1 2 3 4 5 6 7 8 9 10; do
  rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
  ( ulimit -n 65536; perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/${P[$n]}/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" > "$log" 2>&1; echo "exit=$?" >> "$log" )
  if /usr/bin/grep -q "Cannot find module\|Export named\|Original selected owner registry changed\|ENFILE\|EMFILE" "$log"; then echo "$n attempt $attempt: infra/ENFILE retry" >> "$F/🗑️generated/c4p34-checkapplied-$label.attempts"; sleep 120; continue; fi
  break
done
echo "$n exit=$(tail -1 $log)" >> "$F/🗑️generated/c4p34-checkapplied-$label.exit"
