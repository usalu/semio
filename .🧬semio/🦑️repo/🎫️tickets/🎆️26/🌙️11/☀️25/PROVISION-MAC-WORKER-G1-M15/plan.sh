#!/bin/bash
set -euo pipefail

cd "/Users/ueli/Documents/semio"
plan_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/🗑️generated"
mkdir -p "$plan_dir"
if ! mkdir "$plan_dir/plan.running" 2>/dev/null; then
  printf '%s\n' 'The adoption plan is already running in another Terminal window.'
  exit 1
fi
trap 'rmdir "$plan_dir/plan.running"' EXIT
(
  plan_status=0
  sudo /usr/bin/env PATH=/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin \
    SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 \
    SEMIO_FLEET_NETWORK=192.168.178.0/24 /bin/bash \
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🐚️.sh" g1 g1-m14 --user ueli --plan </dev/tty || plan_status=$?
  printf '\n[DEBUG] provision plan exit: %s\n' "$plan_status"
  exit "$plan_status"
) 2>&1 | tee "$plan_dir/plan.log"
