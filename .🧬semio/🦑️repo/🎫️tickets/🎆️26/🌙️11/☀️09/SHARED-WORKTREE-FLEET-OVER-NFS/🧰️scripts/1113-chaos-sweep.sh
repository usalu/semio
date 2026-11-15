#!/bin/sh
# usage: 1113-chaos-sweep.sh [sweep args]   e.g. --from 1 --count 200 --parallel 4  |  --profile hostile --machines 5 --steps 120  |  --shrink
# Failing seeds and traces are written to the ticket's 🗑️generated/chaos/ (delete when done).
set -eu
BUN="${BUN:-bun}"
HERE="$(cd "$(dirname "$0")/../../../../../../../.." && pwd)"
cd "$HERE/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests"
exec env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" run ./🌪️sweep.ts "$@"
