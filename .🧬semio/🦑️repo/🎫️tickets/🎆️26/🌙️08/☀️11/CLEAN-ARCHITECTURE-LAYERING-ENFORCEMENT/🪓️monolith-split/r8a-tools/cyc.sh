#!/bin/zsh
# usage: cyc.sh <5d|2d> : plan with mutation families on, print crates/problems, blocker table and feedback arcs
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
case $1 in 5d) PKG='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust';; 2d) PKG='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust';; esac
export F3_SLICES=1 F3_GROUP_NAMES=40
zsh $F/r8a-tools/step.sh blk$1 bun $F/r8a-tools/r2-blockers.ts "$PKG" on
L=$F/🗑️generated/r8a-blk$1.log
/usr/bin/grep '^problem' $L | cut -c1-200
/usr/bin/grep -c '^crate ' $L
python3 $F/r8a-tools/ed.py $L ${2:-1} 8 | cut -c1-140
python3 $F/r8a-tools/nodesfas.py $L.nodes.json && python3 $F/r8a-tools/fas.py | cut -c1-170 | head -${3:-60}
