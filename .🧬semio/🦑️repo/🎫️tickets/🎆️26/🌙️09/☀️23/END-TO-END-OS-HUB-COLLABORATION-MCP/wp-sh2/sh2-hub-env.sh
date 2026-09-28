#!/bin/zsh
# 🔑️ SH2: runs a command with the hub test credential of the W3 hub recipe in env only (never argv, never printed).
# usage: zsh sh2-hub-env.sh <command …>   e.g. zsh sh2-hub-env.sh bun sh2-probe-delete.ts <serve-url> <space-name>
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2 || exit 2
entry=$(/usr/bin/grep -o '"user1@semio.dev|[^"]*"' ../wp-w3/w3-restart-7800.sh | head -1 | tr -d '"')
export OS_HUB_PROBE_EMAIL="${entry%%|*}" OS_HUB_PROBE_PASSWORD="${entry##*|}" NX_DAEMON=false
exec "$@"
