#!/bin/zsh
# 🔧️ Runs one tool script of this folder with the pinned bun inside the TypeScript package (so the test tools resolve), in the foreground with a timeout: t.sh <script.ts> <args>…
source "${0:A:h}/env.sh"
s="$1"; shift
cd "$S/📦️packages/🟦️typescript" && timeout ${T:-900} "$B" "$W/$s" "$@"
