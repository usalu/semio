#!/bin/sh
# 🔗 Author 09: full validation and publication of each named asset in turn, then the fast connected and feature rules of the remaining ones. Usage: sh rework-09-chain2.sh
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-09"
for a in vintage-car-1929 delivery-van; do sh rework-09-chain.sh "$a"; done
for a in electric-kick-scooter motor-scooter city-bus tram; do "$BUN" rework-09-connected.ts "vehicles/$a" > "$G/$a.connected.log" 2>&1; done
echo CHAIN2DONE > "$G/chain2.done"
