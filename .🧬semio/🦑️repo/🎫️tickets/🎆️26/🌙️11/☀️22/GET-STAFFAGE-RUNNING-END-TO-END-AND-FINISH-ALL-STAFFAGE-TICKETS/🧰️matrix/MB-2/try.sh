#!/usr/bin/env bash
# 🧫️ Runs try.ts from the package folder of the product: try.sh <id> [--no-ops] [--fast]
source "$(dirname "$0")/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 900 "$BUN" "$M/try.ts" "$@"
