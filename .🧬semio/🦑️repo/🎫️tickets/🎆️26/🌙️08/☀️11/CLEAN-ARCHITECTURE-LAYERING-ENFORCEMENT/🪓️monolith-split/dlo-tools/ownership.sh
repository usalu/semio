#!/bin/sh
# 🏷️ Runs the s-dev composition source ownership law once. Usage: ownership.sh <label>
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
perl -e 'alarm 1500; exec @ARGV' -- bun -e 'const {sDevCompositionOwnershipV1}=await import("./✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts"); await sDevCompositionOwnershipV1("/Users/ueli/Documents/semio"); console.log("OWNERSHIP-OK");' > "$G/dlo-ownership-$1.txt" 2>&1
echo $? > "$G/dlo-ownership-$1.status"
