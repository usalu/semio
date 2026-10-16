#!/bin/zsh
# usage: run_oracle.sh <modes...>   (build + widen + oracle; logs to SP/oracle.log)
SPD=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osdb
STAGEDB="$SPD/stage/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osdb-tools"
{
python3 build.py 2>&1 | tail -6
python3 widen.py "$STAGEDB" "$SPD/crates.json" 2>&1 | tail -3 | cut -c1-300
python3 testkit.py "$STAGEDB" "$SPD/crates.json" 2>&1 | head -30 | cut -c1-300
python3 srcfix.py apply 2>&1 | tail -4 | cut -c1-300
python3 finalize.py 2>&1 | tail -3 | cut -c1-200
python3 oracle2.py "$STAGEDB" "$SPD/crates.json" "$@" 2>&1 | head -80 | cut -c1-240
} > $SPD/oracle.log 2>&1
echo done >> $SPD/oracle.log
