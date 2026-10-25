#!/bin/sh
# Ticket input script (dl-c): type-checks the given TypeScript files (and their import closure) with the repo's pinned tsc, strict, no emit.
#   sh tc.sh <file>...
cd /Users/ueli/Documents/semio || exit 2
exec nice -n 10 bun node_modules/typescript/bin/tsc --noEmit --strict --skipLibCheck --target ES2022 --module ESNext --moduleResolution bundler --allowImportingTsExtensions --resolveJsonModule --types bun "$@"
