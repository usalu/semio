#!/bin/zsh
# 🧰️ Runs one tool of this folder for a plant with the pinned bun, in the foreground, log in 🗑️generated/silhouette/SV-2/run: t.sh <tool> <args…>
source "${0:A:h}/env.sh"
mkdir -p "$GEN/run"
tool="$1"; shift
cd "$S/📦️packages/🟦️typescript" && timeout ${TMO:-1200} "$B" "$W/$tool.ts" "$@" 2>&1 | tee -a "$GEN/run/$tool.log" | cut -c1-${WIDTH:-400}
