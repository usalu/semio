#!/bin/zsh
# 🌐️ WG9 ticket serve: the wasm32 wgpu release shell of one variant on <port>, its plugin module from a catalog-exact durable root,
# its agent-bridge rendezvous private to this port (`S_AGENT_BRIDGE_DIR`). Detached through w2-detach (own session); prints the pid.
# usage: zsh wg9-serve.sh <port> <variant> <pluginId> <moduleRootName>
set -u
R=/Users/ueli/Documents/semio
H=$R/.🧬semio/🌐hub
PORT=$1; export WG9_VARIANT=$2 WG9_PLUGIN=$3 WG9_MODULE_ROOT=$4
export S_AGENT_BRIDGE_DIR=$H/s13-wg9-bridge-$PORT
mkdir -p $S_AGENT_BRIDGE_DIR && chmod 700 $S_AGENT_BRIDGE_DIR
mkdir -p "$H/s13-wg9-runtime-$2/activation" "$H/s13-wg9-runtime-$2/extensions"
[ -f "$H/s13-wg9-runtime-$2/activation/🔣️receipt.json" ] || printf '{}' > "$H/s13-wg9-runtime-$2/activation/🔣️receipt.json"
lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null && { echo "port $PORT bound"; exit 1; }
python3 $R/.tmp-ticket/wp-w2/w2-detach.py $H/s13-wg9-logs/serve-$PORT.log bun $R/.tmp-ticket/wp-wg9/serve/wg9-serve.ts $PORT
