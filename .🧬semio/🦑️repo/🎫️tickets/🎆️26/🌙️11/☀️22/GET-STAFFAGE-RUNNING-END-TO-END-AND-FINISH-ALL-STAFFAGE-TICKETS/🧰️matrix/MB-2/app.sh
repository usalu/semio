#!/usr/bin/env bash
# 🧷️ Applies the repair ops of assets to the catalog from the package folder of the product: app.sh <id>…
source "$(dirname "$0")/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 300 "$BUN" "$M/apply.ts" "$@"
