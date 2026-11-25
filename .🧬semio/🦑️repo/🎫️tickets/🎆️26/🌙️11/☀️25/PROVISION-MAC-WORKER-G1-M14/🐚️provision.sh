#!/usr/bin/env bash
# 🍎 Provision this Mac as fleet worker g1-m14 with the verified kit in ~/semio-setup (see 1114-new-mac-worker-prompt.md). Pass --plan for the adoption dry run.
set -uo pipefail
export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
unset SEMIO_FLEET_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN
log=~/semio-setup/$([ "${1:-}" = --plan ] && echo plan || echo provision).log
bash ~/semio-setup/topology/🥾️provision/🐚️.sh g1 g1-m14 --user "$(id -un)" "$@" 2>&1 | tee "$log"
code=${PIPESTATUS[0]}
echo "PROVISION-EXIT=$code" | tee -a "$log"
