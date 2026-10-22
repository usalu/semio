#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/small-fixes/tests"
mkdir -p "$G"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
files=(
"$L/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"
"$L/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts"
"$L/🏗️build/🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts"
"$L/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts"
"$L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts"
"$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts"
"$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts"
"$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts"
"🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🎟️writer/🟦️.ts"
)
i=0
for f in "${files[@]}"; do
  i=$((i+1))
  start=$(date +%s)
  nice -n 10 bun test "./$f" > "$G/$i.log" 2>&1
  rc=$?
  end=$(date +%s)
  echo "$i rc=$rc secs=$((end-start)) $(basename "$f")" >> "$G/summary.txt"
done
echo DONE >> "$G/summary.txt"
