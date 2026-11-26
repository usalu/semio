#!/bin/bash
set -euo pipefail

cd "/Users/ueli/Documents/semio"
origin=g1-m15
origins=/Volumes/semio-store/v1/origins
if [[ ! -r "$origins" || ( -e "$origins/$origin" && ! -f .cache/f/machine.json ) ]]; then
  printf '%s\n' "Cannot confirm that $origin is available in the mounted fleet store."
  exit 1
fi

log_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/🗑️generated"
mkdir -p "$log_dir"
(
  join_status=0
  sudo /usr/bin/env PATH=/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin \
    SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 \
    SEMIO_FLEET_NETWORK=192.168.178.0/24 /bin/bash \
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🐚️.sh" g1 "$origin" --user ueli </dev/tty || join_status=$?
  printf '\n[DEBUG] provision join exit: %s\n' "$join_status"
  exit "$join_status"
) 2>&1 | tee "$log_dir/join.log"
