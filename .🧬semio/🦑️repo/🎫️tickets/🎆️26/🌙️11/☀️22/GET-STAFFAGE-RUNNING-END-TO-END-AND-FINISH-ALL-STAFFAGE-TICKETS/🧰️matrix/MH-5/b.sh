#!/bin/zsh
# 🪶️ The pinned bun for the small tools of this fixer (fit.ts, mgate.ts, scan): b.sh <script> <args…> runs it in the directory b.sh was called from, only through the machine-wide memory queue (`repo capacity run --memory=${MEM:-1000}` from the repo root, memory rule of 2026-11-24) and under `timeout ${LIMIT:-900}`. It takes no batch lock, so inside q.sh it is only called with MH5_GRANT=1 (the chain already holds one grant; a grant inside a grant waits for itself), where it runs bun directly.
bun="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
[[ -n "$MH5_GRANT" ]] && exec timeout "${LIMIT:-900}" "$bun" "$@"
here="$PWD"
cd /Users/ueli/Documents/semio
exec repo capacity run --memory="${MEM:-800}" -- /bin/zsh -c 'cd "$1" && shift && exec "$@"' b "$here" timeout "${LIMIT:-900}" "$bun" "$@" 2> >(grep -v '^\[capacity\]' >&2)
