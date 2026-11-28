#!/bin/zsh
# 🏃️ Runs one TypeScript tool of this folder with the pinned bun inside the TypeScript package of staffage: run.sh <tool.ts> <args…>
source "${0:A:h}/env.sh"
t="$1"; [[ "$t" == /* ]] || t="$W/$t"
cd "$S/📦️packages/🟦️typescript" && "$B" "$t" "${@:2}"
