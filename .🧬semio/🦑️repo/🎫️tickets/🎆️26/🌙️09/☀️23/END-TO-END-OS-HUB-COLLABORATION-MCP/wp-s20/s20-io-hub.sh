#!/bin/zsh
# 🚪️ S20 14c: `verify io` against a hub (default 7800) for one locale. Credentials are read from the hub recipe's
# provisioning list (user1) into the environment only — never argv, never logged.
# usage: zsh s20-io-hub.sh <en|de> <tag> [hub-url] [extra verify-io flags…]
locale="$1"; tag="$2"; hub="${3:-http://127.0.0.1:7800}"; shift 3 2>/dev/null
cd /Users/ueli/Documents/semio
entry=$(/usr/bin/grep -o '"user1@semio.dev|[^"]*"' .tmp-ticket/wp-w4/w4-restart-7800.sh | head -1 | tr -d '"')
export OS_HUB_PROBE_EMAIL="${entry%%|*}"
export OS_HUB_PROBE_PASSWORD="${entry##*|}"
export NX_DAEMON=false
cd '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript'
exec nice -n 5 bun ./📜️script.ts verify io --serve http://127.0.0.1:6640/ --hub "$hub" --locale "$locale" --tag "$tag" --out /Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-io "$@"
