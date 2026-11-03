#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
G="$T/🗑️generated/en1993-norm"
cd "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/🧭️spec-trace/🧰️codegen" && "$T/🗑️generated/build-runner/heavy.sh" /home/deck/semio/.venv/bin/python -I 🐍️.py > "$G/spec-trace.txt" 2>&1; echo "spec-trace exit $?" >> "$G/spec-trace.txt"
