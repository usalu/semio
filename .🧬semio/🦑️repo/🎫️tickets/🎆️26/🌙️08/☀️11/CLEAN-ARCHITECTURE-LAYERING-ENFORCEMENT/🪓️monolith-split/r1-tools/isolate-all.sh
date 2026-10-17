#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SB=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/r1
ulimit -n 65536
i=0
for a in 🗜️deflate 📰️xml 🧾️json 🎒️zip; do
  i=$((i+1))
  nice -n 10 bun "$F/r1-tools/cfgtest-isolate.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/$a" "$SB/keep-$i-combined" > "$F/🗑️generated/r1-isolate-$i.log" 2>&1
  echo "$a $?" >> "$F/🗑️generated/r1-isolate-all.status"
done
echo done >> "$F/🗑️generated/r1-isolate-all.status"
