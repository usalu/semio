#!/bin/zsh
# usage: checkapplied.sh <tag> <artifact package dir relative to repo root>
tag=$1; pkg=$2
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pfa/chk
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd /Users/ueli/Documents/semio
rm -rf "$S/$tag-sbx" "$S/$tag-sbx-base" "$S/$tag-sbx-combined"
perl -e 'alarm 2400; exec @ARGV' -- bun ./📜️script.ts new crates "$pkg" --check-applied --sandbox="$S/$tag-sbx" > "$F/🗑️generated/pfa-checkapplied-$tag.log" 2>&1
echo "exit=$?" >> "$F/🗑️generated/pfa-checkapplied-$tag.log"
