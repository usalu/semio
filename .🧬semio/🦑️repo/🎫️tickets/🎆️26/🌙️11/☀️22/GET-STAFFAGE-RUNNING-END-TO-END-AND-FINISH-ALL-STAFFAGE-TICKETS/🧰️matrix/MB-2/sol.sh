#!/usr/bin/env bash
# 📦️ Runs solids.ts from the package folder of the product: sol.sh <id> <scale> [id-substring]
source "$(dirname "$0")/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 300 "$BUN" "$M/solids.ts" "$@"
