#!/bin/sh
# Derives the lock bun would write for one scope (offline, lockfile only) without leaving it in the tree. Usage: lockgen.sh <scope-dir|.>
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
S="$1"; N=$(printf '%s' "$S" | sed 's#/#_#g; s#^\.$#root#')
cd "/Users/ueli/Documents/semio/$S" || exit 2
cp bun.lock "$G/aje-lock-backup/$N.bun.lock.$(date +%H%M%S)"
cp bun.lock "$G/aje-lock-backup/$N.bun.lock.current"
perl -e 'alarm 900; exec @ARGV' -- bun install --lockfile-only --offline > "$G/aje-lock-gen/$N.log" 2>&1
echo "exit $?" >> "$G/aje-lock-gen/$N.log"
mv bun.lock "$G/aje-lock-gen/$N.bun.lock"
cp "$G/aje-lock-backup/$N.bun.lock.current" bun.lock
tail -n 3 "$G/aje-lock-gen/$N.log"
