#!/bin/sh
# 🔭️ Scoped dependency-cruiser run (audit-2 section 1 config) over the gis plugin, the stdio docx mutations file and the SDK package. Usage: cruise.sh <label>
L="$1"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
CFG="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"
DOCX="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx"
perl -e 'alarm 1800; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins/🌍️gis "$DOCX" 🧰️framework/📦️packages/🟦️typescript --config "$CFG" --output-type err > "$G/dlo-cruise-$L.txt" 2> "$G/dlo-cruise-$L.stderr.txt"
echo $? > "$G/dlo-cruise-$L.status"
