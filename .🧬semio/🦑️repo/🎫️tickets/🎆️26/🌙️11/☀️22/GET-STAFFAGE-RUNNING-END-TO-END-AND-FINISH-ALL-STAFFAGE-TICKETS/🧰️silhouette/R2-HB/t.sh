#!/bin/zsh
# 🔧️ Runs one tool of this folder with the pinned bun in the package folder, one process, with a timeout; an argument that names a file of this folder (ops/…, trial/…) is made absolute: t.sh <tool.ts> <args>…
source "${0:A:h}/env.sh"
tool="$1"; shift
args=(); for x in "$@"; do [[ "$x" != /* && -e "$W/$x" && "$x" == */* ]] && args+=("$W/$x") || args+=("$x"); done
cd "$S/📦️packages/🟦️typescript" && timeout ${T_TIMEOUT:-900} "$B" "$W/$tool" "${args[@]}"
