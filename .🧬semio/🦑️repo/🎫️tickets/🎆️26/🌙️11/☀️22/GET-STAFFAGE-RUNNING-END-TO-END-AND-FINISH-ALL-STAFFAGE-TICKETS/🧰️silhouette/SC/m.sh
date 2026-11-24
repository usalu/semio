#!/bin/zsh
# 📐️ The gate measurement of components (measure.ts), one process: m.sh <id>… [--scales 50,100]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$W/measure.ts" "$@" 2>&1 | cut -c1-400
