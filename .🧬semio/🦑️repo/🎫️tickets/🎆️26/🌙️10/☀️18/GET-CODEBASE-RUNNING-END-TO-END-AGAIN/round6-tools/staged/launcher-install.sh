#!/bin/sh
# live-only: builds the full `repo` (launcher + mcp) through the repository build, installs it, proves the script route against the pinned runtime; a no-op in the overlay simulation
case "${SEMIO_ROOT:-}" in /private/tmp/*) echo "launcher-install: skipped in the overlay simulation"; exit 0;; esac
set -e
cd "${SEMIO_ROOT:?}"
nice -n 10 bun repo run repo-mcp:build --output=stream
nice -n 10 bun repo run repo-mcp:install --output=stream
NEW="$(command -v repo)"
"$NEW" --version
TMP="$(mktemp -d)"
printf 'console.log(JSON.stringify({bun:Bun.version,args:process.argv.slice(2)}))\n' > "$TMP/hello.ts"
OUT="$("$NEW" "$TMP/hello.ts" a "b c")"
rm -rf "$TMP"
echo "$OUT"
case "$OUT" in *'"bun":"1.4.2"'*'"args":["a","b c"]'*) echo "launcher-install: pinned runtime 1.4.2 proven";; *) echo "launcher-install: script route did not run the pinned runtime"; exit 1;; esac
