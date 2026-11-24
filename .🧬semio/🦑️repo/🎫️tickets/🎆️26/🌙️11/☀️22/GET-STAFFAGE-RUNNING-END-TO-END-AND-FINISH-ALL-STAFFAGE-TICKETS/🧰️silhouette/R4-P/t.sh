#!/bin/zsh
# 🔧️ Runs one tool of this folder with the pinned bun in the package folder, one process, with a timeout: t.sh <tool.ts> <args>…   (T_TIMEOUT seconds, default 1800)
source "${0:A:h}/env.sh"
tool="$1"; shift
cd "$S/📦️packages/🟦️typescript" && timeout ${T_TIMEOUT:-1800} "$B" "$W/$tool" "$@"
