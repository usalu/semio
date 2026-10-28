#!/bin/sh
# 🧬 Laws of batch `clonejournal` after apply: sh verify.sh <root>   (repository or mirror root; builds both watchers with rustc into a temp dir outside the root)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
R=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident
python3 -I "$here/stage.py" check "$root"
out=$(mktemp -d "${TMPDIR:-/tmp}/hpcj.XXXXXX"); mkdir -p "$out/new" "$out/base"
rustc --crate-name semio_repo_watch --edition 2021 -O "$R/🔭️watch/🦀️rust/🦀️.rs" -o "$out/new/semio-repo-watch"
rustc --crate-name semio_repo_watch --edition 2021 --test "$R/🔭️watch/🦀️rust/🦀️.rs" -o "$out/unit" && "$out/unit"
rustc --crate-name semio_repo_watch --edition 2021 -O "$here/base/00.rs" -o "$out/base/semio-repo-watch"
for cfg in semio_check_linux semio_check_windows; do rustc --crate-name semio_repo_watch --edition 2021 --cfg "$cfg" --emit=metadata -o "$out/m.rmeta" "$R/🔭️watch/🦀️rust/🦀️.rs"; done
SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" bun test "$R/📜️journal/🧪️tests/🟦️.ts" "$R/🧪️tests/🔭️watch/🟦️.ts" "$R/🧪️tests/🏗️provision/🟦️.ts"
echo "negative control: the base watcher must fail the clone law"
if SEMIO_REPO_WATCH_BINARY="$out/base/semio-repo-watch" bun test "$R/📜️journal/🧪️tests/🟦️.ts" -t "copy-on-write clone" >"$out/neg.txt" 2>&1; then echo "NEGATIVE CONTROL DID NOT BITE"; exit 1; else grep -E "Expected|Received|fail" "$out/neg.txt" | head -8; fi
rm -rf "$out"
echo "clonejournal laws: GREEN"
