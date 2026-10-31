#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"; G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/transport"
cd /home/deck/semio
WRITE=1 "$T/🧰️scripts/transport/run-oracles.sh" 🌀️parcel-jet ☁️condensation > "$G/oracles5.txt" 2>&1
repo tools exec bun -- x vitest run --config "$T/🧰️scripts/transport/transport.vitest.config.mjs" > "$G/vitest6.txt" 2>&1
echo finished >> "$G/vitest6.txt"
repo tools exec bun -- "$T/🧰️scripts/transport/render-evidence.ts" "$G/evidence" > "$G/render-log.txt" 2>&1
echo finished >> "$G/render-log.txt"
