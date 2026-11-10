#!/usr/bin/env bash
cd /home/deck/semio
A="✏️s/🔌️plugins/📕️norm/🗿️artifacts"
for d in "$A"/*/; do
  s="$d📦️packages/🟦️typescript/📜️script.ts"
  name=$(basename "$d")
  list=$(grep -o 'suites:\[[^]]*\]' "$s" | grep -o '"[^"]*"' | tr -d '"')
  files=()
  for f in $list; do files+=("./$d$f"); done
  for f in "${files[@]}"; do :; done
  echo "=== $name (${#files[@]} suites)"
  base=$(grep -o 'suites:\[[^]]*\]' "$s" | head -1)
  echo "$list" | while read -r rel; do :; done
  # suites are relative to package roots; resolve by search
  pkgroot=$(grep -o 'runArtifactTypeScriptPackageMain([^)]*' "$s" | head -1)
  for rel in $list; do
    p=$(find "$d" -path "*$rel" -not -path '*/node_modules/*' | head -1)
    [ -z "$p" ] && { echo "MISSING $rel"; continue; }
    timeout 900 repo --service=off tools exec --light bun -- test --timeout 120000 "./$p" 2>&1 | grep -E '^ *[0-9]+ (pass|fail)|error:' | tr '\n' ' '; echo " <- $rel"
  done
done
