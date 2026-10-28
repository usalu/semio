#!/bin/sh
# 🪟 Laws of batch `winframe` after apply: sh verify.sh <mirror-root>   (builds the watcher with rustc into a temp dir outside the root; checks the real Windows and Linux targets with the installed std)
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); XPLAT5=$(cd "$HERE/../.." && pwd); . "$XPLAT5/verify-common.sh"
B='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; RS="./$B/🏡️resident/🔭️watch/🦀️rust/🦀️.rs"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
OUT=$(mktemp -d "${TMPDIR:-/tmp}/xp5w.XXXXXX")
law "watcher unit tests (verbatim paths, clone kinds, filter, merge)" sh -c "rustc --crate-name semio_repo_watch --edition 2021 --test '$RS' -o '$OUT/unit' && '$OUT/unit'"
for target in x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin x86_64-apple-darwin; do
  if rustup target list --installed 2>/dev/null | grep -qx "$target"; then law "watcher type-checks for $target" rustc --crate-name semio_repo_watch --edition 2021 --target "$target" --emit=metadata -o "$OUT/$target.rmeta" "$RS"; else echo "SKIP  watcher type-check for $target (std not installed)"; fi
done
rustc --crate-name semio_repo_watch --edition 2021 -O "$RS" -o "$OUT/semio-repo-watch" || FAILED=1
SEMIO_REPO_WATCH_BINARY="$OUT/semio-repo-watch" law "native frontier contract" bun test "./$B/🕸️graph/🕰️generations/🧪️tests/🪟️native-frontier/🟦️.ts"
SEMIO_REPO_WATCH_BINARY="$OUT/semio-repo-watch" law "watcher, scan and provision laws" bun test "./$B/🏡️resident/🧪️tests/🔭️watch/🟦️.ts" "./$B/🏡️resident/🧪️tests/🏗️provision/🟦️.ts"
law "graph memo (unchanged)" bun test $(find "$B/🧭️invocation/🕸️graph/🧠️memo" -path '*🧪️tests*' -name '🟦️.ts' | sed 's#^#./#')
negative "native frontier" "$HERE" "$ROOT" bun test "./$B/🕸️graph/🕰️generations/🧪️tests/🪟️native-frontier/🟦️.ts"
rm -rf "$OUT"
exit $FAILED
