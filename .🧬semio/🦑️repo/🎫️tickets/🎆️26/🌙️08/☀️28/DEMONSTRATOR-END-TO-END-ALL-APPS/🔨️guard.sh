#!/usr/bin/env bash
# 🛡️ Keeps the tree buildable while the non-idempotent rename-plan applier is active.
# Reaps ONLY `bun -e` children of the ChatGPT/Codex app-server (never the app itself),
# and restores emoji path segments that applier doubles (`📦️📦️packages` → `📦️packages`).
# Doubling is always a defect of that applier, never an intended name; every action is logged.
set -uo pipefail
cd /Users/ueli/Documents/semio || exit 1
PARENT="${CODEX_PARENT:-66250}"
while true; do
  for p in $(ps -eo pid,ppid,args 2>/dev/null | awk -v pp="$PARENT" '$2==pp' | grep 'bun -e' | grep -v grep | awk '{print $1}'); do
    kill "$p" 2>/dev/null && echo "[guard $(date '+%H:%M:%S')] reaped applier $p"
  done
  while IFS= read -r src; do
    [ -z "$src" ] && continue
    base="$(basename "$src")"
    single="$(printf '%s' "$base" | perl -CSD -pe 's/(\X)\1+/$1/g')"
    dst="$(dirname "$src")/$single"
    [ "$src" = "$dst" ] && continue
    [ -e "$dst" ] || { mv "$src" "$dst" 2>/dev/null && echo "[guard $(date '+%H:%M:%S')] restored $dst"; }
  done < <(find . -type d \( -name '*📦️📦️*' -o -name '*📇️📇️*' -o -name '*🎫️🎫️*' -o -name '*🦑️🦑️*' \) -not -path './node_modules/*' -not -path './target/*' 2>/dev/null)
  sleep 2
done
