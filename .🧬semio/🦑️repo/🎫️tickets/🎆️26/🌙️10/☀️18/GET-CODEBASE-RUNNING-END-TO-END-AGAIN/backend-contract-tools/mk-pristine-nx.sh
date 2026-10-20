#!/bin/sh
# Assembles a pristine Nx (the repository's own install carries the envelope patch and refuses to run outside a repo CLI) from the installed dependencies and the unpatched package in Bun's cache. Output: /private/tmp/bkc/nxroot; then export SEMIO_TEST_NX_BIN=/private/tmp/bkc/nxroot/node_modules/nx/dist/bin/nx.js
set -e
N=$(ls -d "$(cd "$(dirname "$0")/../../../../../../.." && pwd)"/.cache/b/t/m/nx/*/ | head -1); P=$(ls -d ~/.bun/install/cache/nx@23.2.0@@@1); R=/private/tmp/bkc/nxroot
rm -rf "$R"; mkdir -p "$R"; cp -cR "$N/node_modules" "$R/node_modules"; chmod -R u+w "$R/node_modules/nx"; rm -rf "$R/node_modules/nx/dist"; cp -cR "$P/dist" "$R/node_modules/nx/dist"
for f in executors.json generators.json migrations.json package.json schemas; do rm -rf "$R/node_modules/nx/$f"; cp -cR "$P/$f" "$R/node_modules/nx/$f"; done
echo "$R/node_modules/nx/dist/bin/nx.js"
