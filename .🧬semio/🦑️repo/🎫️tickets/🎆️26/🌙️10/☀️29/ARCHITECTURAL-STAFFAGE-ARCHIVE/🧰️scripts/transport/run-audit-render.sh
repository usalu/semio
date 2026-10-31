#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
G="$T/🗑️generated/transport"
cd /home/deck/semio
repo tools exec bun -- "$T/🧰️scripts/transport/audit-assets.ts" "$G/audit.json" > "$G/audit-log.txt" 2>&1
echo finished >> "$G/audit-log.txt"
repo tools exec bun -- "$T/🧰️scripts/transport/render-evidence.ts" "$G" > "$G/render-log.txt" 2>&1
echo finished >> "$G/render-log.txt"
