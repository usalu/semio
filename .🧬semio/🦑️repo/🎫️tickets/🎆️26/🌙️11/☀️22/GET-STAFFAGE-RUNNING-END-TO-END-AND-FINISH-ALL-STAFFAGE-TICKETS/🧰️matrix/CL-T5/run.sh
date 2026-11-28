#!/bin/zsh
# 🏃️ Runs one TypeScript tool (of this folder, or an absolute path) with the pinned bun from the TypeScript package of staffage (so its dependencies resolve): run.sh <tool.ts> <args…>
source "${0:A:h}/env.sh"
t="$1"; [[ "$t" == /* ]] || t="$W/$t"
cd "$S/📦️packages/🟦️typescript" && "$B" "$t" "${@:2}"
