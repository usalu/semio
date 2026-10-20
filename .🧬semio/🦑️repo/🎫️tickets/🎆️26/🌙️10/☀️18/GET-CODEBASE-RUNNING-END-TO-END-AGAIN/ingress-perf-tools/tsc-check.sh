#!/bin/sh
# 🔬 Type-checks the given overlay-relative files in an overlay copy and prints only diagnostics located in those files.
overlay="$1"; shift
cd "$overlay" || exit 2
files=""
for f in "$@"; do files="$files ./$f"; done
perl -e 'alarm 280; exec @ARGV' -- /Users/ueli/Documents/semio/node_modules/.bin/tsc --noEmit --strict --target esnext --module esnext --moduleResolution bundler --resolveJsonModule --allowImportingTsExtensions --skipLibCheck --types bun --lib esnext,dom $files 2>&1
