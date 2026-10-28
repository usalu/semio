#!/bin/sh
# 📦 Applies / reverts / checks all germanqa batches in landing order: sh apply_all.sh <root> apply|revert|check   (land as ONE group: the pinned tests change with the catalogs)
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=$1; verb=$2
for b in process libbuild libcache libother mcp modules docs; do
  [ -d "$here/staged/$b" ] || continue
  echo "== $b"; python3 -I "$here/staged/$b/stage.py" "$verb" --root "$root"
done
