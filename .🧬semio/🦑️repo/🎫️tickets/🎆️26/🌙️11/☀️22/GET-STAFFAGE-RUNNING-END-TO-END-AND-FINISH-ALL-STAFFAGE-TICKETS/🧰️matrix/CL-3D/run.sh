#!/bin/zsh
# 🏃️ Runs one TypeScript tool of this folder with the pinned bun from the TypeScript package of staffage (so its dependencies resolve): run.sh <tool.ts> <args…>
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && "$B" "$W/$1" "${@:2}"
