#!/usr/bin/env bash
# [scratch] assembles, checks and previews the dairy cow: cow-view.sh <extra preview args...>
T=/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE; A=$T/🧰️scripts/animals; OUT=$T/🗑️generated/animals
mkdir -p $OUT/dairy-cow; bash $A/category.sh $OUT/dairy-cow.json dairy-cow
cd /home/deck/semio && repo tools exec bun -- $A/preview.ts --file $OUT/dairy-cow.json --asset dairy-cow "$@" 2>&1 | grep -v "^\[" 
