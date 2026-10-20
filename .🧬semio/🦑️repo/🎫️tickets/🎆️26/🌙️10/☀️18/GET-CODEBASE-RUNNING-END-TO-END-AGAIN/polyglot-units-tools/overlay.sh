#!/bin/sh
# usage: overlay.sh   -> builds /private/tmp/polyglot-units/ov : a mirror tree where the staged module sits at its real depth
# (L/🏗️build/🧩️units) next to symlinks to the real validator, the live TypeScript units engine and node_modules. Re-run after every edit.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"; R=/Users/ueli/Documents/semio; O=/private/tmp/polyglot-units/ov
F="🧰️framework"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"
mkdir -p "$O/$F/🛍️products/🦑️repo/🔨️modules" "$O/$L/🏗️build"
ln -sfn "$R/node_modules" "$O/node_modules"
ln -sfn "$R/$F/🔨️modules" "$O/$F/🔨️modules"
ln -sfn "$R/$L/🏗️build/🟦️typescript" "$O/$L/🏗️build/🟦️typescript"
ln -sfn "$R/$L/⚡️caching" "$O/$L/⚡️caching"
rm -rf "$O/$L/🏗️build/🧩️units"
mkdir -p "$O/$L/🏗️build/🧩️units"
cp -R "$HERE/staged/🧩️units/." "$O/$L/🏗️build/🧩️units/"
cat > "$O/tsconfig.json" <<J
{"compilerOptions":{"target":"ES2022","module":"ESNext","moduleResolution":"bundler","strict":true,"noEmit":true,"allowImportingTsExtensions":true,"resolveJsonModule":true,"skipLibCheck":true,"types":["bun","node"],"noUncheckedIndexedAccess":false,"lib":["ES2023","DOM"],"typeRoots":["$R/node_modules/@types","$R/node_modules"]},"include":["$L/🏗️build/🧩️units/**/*.ts"]}
J
echo "$O/$L/🏗️build/🧩️units"
