#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
SB=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/r3-sbx
$F/r3-tools/step.sh fin-kit bun ./📜️script.ts new crates "$P/🧰️kit/📦️packages/🦀️rust" --check-applied --sandbox=$SB/kit-applied
$F/r3-tools/step.sh fin-image bun ./📜️script.ts new crates "$P/🖼️image/📦️packages/🦀️rust" --check-applied --sandbox=$SB/image-applied
$F/r3-tools/step.sh fin-document bun ./📜️script.ts new crates "$P/📑️document/📦️packages/🦀️rust" --check-applied --sandbox=$SB/document-applied
$F/r3-tools/step.sh fin-flow bun ./📜️script.ts new crates "$P/🌊️flow/📦️packages/🦀️rust" --check-applied --sandbox=$SB/flow-applied
$F/r3-tools/step.sh fin-object bun ./📜️script.ts new crates "$P/📦️object/📦️packages/🦀️rust" --check-applied --sandbox=$SB/object-applied
echo finished > $F/🗑️generated/r3-fin-all.done
