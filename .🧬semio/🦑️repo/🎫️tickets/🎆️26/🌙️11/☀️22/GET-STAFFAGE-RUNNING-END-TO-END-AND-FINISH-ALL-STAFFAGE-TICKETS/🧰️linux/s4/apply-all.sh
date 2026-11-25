#!/bin/sh
# 🖊️ Applies the asset scripts to the asset documents of a staffage catalog, each idempotent: `written`, `unchanged` or a refusal by name; nothing else of a document changes.
# This lane never ran it on the catalog of the worktree: whoever owns the catalog runs it when the files are quiet (used-harmonium and used-tumble-dryer belong to batch ML-1).
# usage: apply-all.sh <staffage root> [asset id ...]   (default: every asset of assets.txt)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$HERE/../../../../../../../../.." && pwd)
ROOT="$1"
shift
[ $# -gt 0 ] && IDS="$*" || IDS=$(cat "$HERE/assets.txt")
cd "$R"
for id in $IDS; do
  file=$(ls "$ROOT"/🗂️catalog/*/"$id"/🔣️.json)
  repo tools exec bun -- "$HERE/$id.ts" "$file" || echo "FAILED $id" >&2
done
