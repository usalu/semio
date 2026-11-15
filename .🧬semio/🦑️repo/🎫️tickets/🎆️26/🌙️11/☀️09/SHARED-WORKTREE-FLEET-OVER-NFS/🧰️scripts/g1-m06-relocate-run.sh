#!/bin/bash
export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
unset SEMIO_FLEET_CACHE_ENDPOINT
export PATH="/Users/ueli/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
echo "[wrapper] root listing check: $(ls /Users/ueli/Documents/semio 2>&1 | head -n 2 | tr '\n' ' ')"
bash "/private/tmp/claude-501/-Users-Shared-semio/ec8c2d19-c494-4cdf-b721-eb3a982c512c/scratchpad/semio-topology/🥾️provision/🐚️.sh" g1 g1-m06 --user ueli 2>&1 | tee "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🗑️generated/g1-m06-relocate-run.txt"
code=${PIPESTATUS[0]}
echo "provision exit $code" >> "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🗑️generated/g1-m06-relocate-run.txt"
chown ueli "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🗑️generated/g1-m06-relocate-run.txt"
exit 0
