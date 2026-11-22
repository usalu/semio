#!/bin/zsh
# 🩺️ Runs the staffage check CLI on a draft asset document: checkfile.sh <file.json> [samples]
source "${0:A:h}/env.sh"
cd "$S" && $B "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$1" --samples ${2:-9} 2>&1 | cut -c1-260
