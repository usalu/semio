#!/bin/zsh
# 🀄️ Serves one wfc playground variant on its react port (detached; poll with curl). Usage: 📜️serve-wfc-react.sh <variant> <port> <log>
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$2"
exec bun ./📜️script.ts serve "$1" react dev > "$3" 2>&1
