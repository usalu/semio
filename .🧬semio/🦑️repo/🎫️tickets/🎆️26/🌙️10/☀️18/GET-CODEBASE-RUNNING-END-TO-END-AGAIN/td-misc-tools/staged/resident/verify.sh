#!/bin/sh
# 🔭 Laws of batch `resident`: sh verify.sh <repository or mirror root>  (bare, no SEMIO_REPO_* variables: state law green incl. the new trust row, watch 13/0 incl. the 8 native laws, service green except the 1 crash-safety describe skipped without SEMIO_REPO_NATIVE_DIR; base: service 3 red, watch 8 skipped)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "${1:?root}" && pwd -P); cd "$root"
python3 -I "$here/stage.py" check --root "$root"
T=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests
for f in "$T/🗃️state/🟦️.ts" "$T/🔭️watch/🟦️.ts" "$T/🏛️service/🟦️.ts" "$T/🕰️graph-hooks/🟦️.ts"; do env -u SEMIO_REPO_NATIVE_DIR -u SEMIO_REPO_WATCH_BINARY bun test "$f"; done
