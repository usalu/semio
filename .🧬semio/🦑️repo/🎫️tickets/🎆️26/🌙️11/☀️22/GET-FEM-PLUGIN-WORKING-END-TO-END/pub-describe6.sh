#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="/c/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"
until [ -f "$G/🧾️registry-reads/🟨️.mjs" ] || ! grep -q "registry-reads" "$G/📥️load/🟦️.ts"; do sleep 30; done
sleep 120
cd /c/semio || exit 2
repo run workspace:components -- describe fem --profile prototype > "$T/🗑️generated/pub-describe6.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/pub-describe6.log"
