#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
G="$T/🗑️generated/en1993-norm"
cd /home/deck/semio
nice -n 19 /home/deck/semio/.venv/bin/python -I "$T/🧰️scripts/en1993-norm/batch_apply.py" > "$G/apply.txt" 2>&1; echo "batch exit $?" >> "$G/apply.txt"
cd "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp" && "$T/🗑️generated/build-runner/heavy.sh" /home/deck/semio/.venv/bin/python -I "$T/🧰️ndp/codegen.py" 🔣️.json 🗃️table.rs > "$G/codegen.txt" 2>&1; echo "codegen exit $?" >> "$G/codegen.txt"
