#!/bin/bash
# usage: norm-artifact-check.sh <artifact-dir-name>; typechecks the package source and suites like the CheckScript.
cd /home/deck/semio || exit 1
d="✏️s/🔌️plugins/📕️norm/🗿️artifacts/$1"; pkg="$d/📦️packages/🟦️typescript"
LIBS=$(python3 -c "import json;print(','.join(json.load(open('tsconfig.json'))['compilerOptions']['lib']))")
s=(); for x in $(grep -o 'suites:\[[^]]*\]' "$pkg/📜️script.ts" | grep -o '"[^"]*"' | tr -d '"'); do s+=("./$d/$x"); done
repo tools exec tsc -- "./$d/🟦️.ts" "${s[@]}" --noEmit --incremental false --allowImportingTsExtensions --module ESNext --moduleResolution Bundler --resolveJsonModule --allowSyntheticDefaultImports --strict --skipLibCheck --target ES2022 --lib "$LIBS" 2>&1 | grep -v "^\[capacity\]\|still working\|^\[tools\]"
