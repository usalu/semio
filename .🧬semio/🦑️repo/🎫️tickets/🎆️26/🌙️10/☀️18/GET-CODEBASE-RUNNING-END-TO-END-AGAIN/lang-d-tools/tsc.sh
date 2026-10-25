#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 node_modules/.bin/tsc --noEmit --strict --skipLibCheck --allowImportingTsExtensions --resolveJsonModule --esModuleInterop --target ESNext --module ESNext --moduleResolution bundler --types node,bun "$@"
