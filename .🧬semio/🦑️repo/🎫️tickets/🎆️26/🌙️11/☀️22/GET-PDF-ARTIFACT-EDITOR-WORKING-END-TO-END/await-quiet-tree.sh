#!/bin/sh
# usage: await-quiet-tree.sh <minutes>  (returns once no Rust source or manifest under the PDF artifact or its hub composition changed for <minutes>; other sessions refactor these live, and a build started mid-refactor fails on a half-written tree)
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
quiet=${1:-15}
while :; do
  recent=$(find "$root/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf" "$root/🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf" \( -name '*.rs' -o -name 'Cargo.toml' \) -mmin "-$quiet" 2>/dev/null | head -1)
  [ -z "$recent" ] && exit 0
  sleep 60
done
