#!/bin/zsh
# pre.sh <owner-crate> : oracle of the working tree against HEAD for the owner alone (NEW non-stub rows = in-flight breakage)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
o="$1"; n="pre-${o##*-}"
cat > "$F/hdr3-tools/pos/families/hdr3-$n.json" <<EOT
{"name": "hdr3-$n", "crates": ["$o"], "base": {"rev": "7880cc36f78", "crates": ["$o"]}, "note": "pre-check"}
EOT
cd /Users/ueli/Documents/semio
python3 "$F/hdr3-tools/pos/pos.py" run "$F/hdr3-tools/pos/families/hdr3-$n.json" --cfgs lib,test --label p > "$G/hdr3-pre-$n.txt" 2>&1
/usr/bin/grep -E '^-- cfg|privacy:|resolve:|type:|other:' "$G/hdr3-pre-$n.txt"
