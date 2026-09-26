#!/bin/zsh
# 🌐️ WG7 session-12 ticket serve on 6552: the product wgpu release config with the durable catalog-note module root
# (`serve/wg7-serve.ts`), its agent-bridge rendezvous PRIVATE to this slice (`S_AGENT_BRIDGE_DIR`), so no other session's
# semio-MCP gateway ever sees this shell and this shell never dials theirs. Detaches itself; prints the serve pid.
# usage: zsh s12-serve-6552.sh [port] [moduleRootName]   (defaults 6552, s11-wg7-catalog-modules)
set -u
setopt no_bg_nice
R=/Users/ueli/Documents/semio
H=$R/.🧬semio/🌐hub
PORT=${1:-6552}
export WG7_MODULE_ROOT=${2:-s11-wg7-catalog-modules}
export S_AGENT_BRIDGE_DIR=$H/s12-wg7-bridge-$PORT
mkdir -p $S_AGENT_BRIDGE_DIR && chmod 700 $S_AGENT_BRIDGE_DIR
lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null && { echo "port $PORT bound"; exit 1; }
cd $R/.tmp-ticket/wp-wg7 || exit 1
nohup bun serve/wg7-serve.ts $PORT > $H/s12-wg7-logs/serve-$PORT.log 2>&1 < /dev/null &
echo $! > $H/s12-wg7-logs/serve-$PORT.pid
disown
cat $H/s12-wg7-logs/serve-$PORT.pid
