#!/bin/zsh
# usage: plan.sh <tag>  runs the blocker dump with the standard bim lift in the background, result in F/🗑️generated/r9a-<tag>.json
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
rm -f "$G/r9a-$1.exit"
(F3_SLICES=1 F3_GROUP_NAMES=60 nohup zsh "$F/r9a-tools/step.sh" "$1" bun "$F/r9a-tools/blockers.ts" "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust" "$G/r9a-$1.json" on 'standards::v1::subsets::any::schema::*,standards::v1::subsets::any::io::*,editor::*,render::*' >/dev/null 2>&1 &)
