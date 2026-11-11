#!/bin/bash
# Equivalent of `repo run @semio-tech/norm-<code>:build|check|test` while the graph service is blocked.
# Runs bun build (bundle), tsc --noEmit on source and suites, and bun test on suites, all via the tool registry.
cd /home/deck/semio || exit 1
G="$1"; : > "$G/artifacts.txt"
LIBS=$(python3 -c "import json;print(','.join(json.load(open('tsconfig.json'))['compilerOptions']['lib']))")
for d in ${ONLY:-✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/}; do
  d=${d%/}; pkg="$d/📦️packages/🟦️typescript"; n=$(basename "$d"); L="$G/a-$(echo "$n" | md5sum | cut -c1-6)"
  suites=$(grep -o 'suites:\[[^]]*\]' "$pkg/📜️script.ts" | grep -o '"[^"]*"' | tr -d '"')
  src="./$d/🟦️.ts"
  repo tools exec bun -- build "$src" --target node --outdir "$G/out-$(basename "$L")" > "$L.build" 2>&1; b=$?
  s=(); for x in $suites; do s+=("./$d/$x"); done
  repo tools exec tsc -- "$src" "${s[@]}" --noEmit --incremental false --allowImportingTsExtensions --module ESNext --moduleResolution Bundler --resolveJsonModule --allowSyntheticDefaultImports --strict --skipLibCheck --target ES2022 --lib "$LIBS" > "$L.check" 2>&1; c=$?
  repo tools exec bun -- test "${s[@]}" > "$L.test" 2>&1; t=$?
  echo "$n build=$b check=$c test=$t $(grep -E '^ *[0-9]+ (pass|fail)$' "$L.test" | tr -s ' \n' ' ') log=$L" >> "$G/artifacts.txt"
done
echo DONE >> "$G/artifacts.txt"
