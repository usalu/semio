#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
E="$T/🧰️scripts/schema-evolve"
for s in sheet-profiles fire-insulations; do
  .venv/bin/python -I "$E/evolve.py" add-collection en1999 "$E/specs/en1999-$s.json" --apply > "$T/🗑️generated/en1999-fleet/apply-$s.txt" 2>&1
  echo "rc $?" >> "$T/🗑️generated/en1999-fleet/apply-$s.txt"
done
