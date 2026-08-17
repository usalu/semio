#!/usr/bin/env bash
set -euo pipefail
ROOT="/Users/ueli/Documents/semio"
REPO="$ROOT/🧰️framework/🛍️products/🦑️repo"

replace_impl_paths() {
  local dir="$1"
  if [[ ! -d "$dir" ]]; then return 0; fi
  find "$dir" -type f \( -name '*.ts' -o -name '*.tsx' -o -name '*.rs' -o -name '*.go' -o -name '*.json' -o -name '*.mjs' -o -name '*.md' \) \
    ! -path '*/node_modules/*' ! -path '*/target/*' -print0 2>/dev/null | while IFS= read -r -d '' f; do
    if grep -q '⚡️implementations' "$f" 2>/dev/null; then
      sed -i '' 's|⚡️implementations/🟦️typescript|📦️packages/🟦️typescript|g' "$f"
      sed -i '' 's|⚡️implementations/🦀️rust|📦️packages/🦀️rust|g' "$f"
      sed -i '' 's|⚡️implementations/🐹️go|📦️packages/🐹️go|g' "$f"
      sed -i '' 's|/⚡️implementations/🐹️go|/🔨️modules/📚️lib|g' "$f" || true
    fi
  done
}

migrate_ts_owner() {
  local owner="$1"
  local old="$owner/⚡️implementations/🟦️typescript"
  local new="$owner/📦️packages/🟦️typescript"
  [[ -d "$old" ]] || return 0
  mkdir -p "$new"
  for f in package.json project.json script.ts tsconfig.json vitest.config.ts eslint.config.ts vite.config.ts vite.test.config.ts .vscode-test.mjs .vscodeignore LICENSE.md; do
    [[ -f "$old/$f" ]] && mv "$old/$f" "$new/$f"
  done
  for f in 📦️index.ts 📦️index.tsx 🧪️index.test.ts 🟦️extension.ts 🟦️extension.test.ts; do
    [[ -f "$old/$f" ]] && mv "$old/$f" "$new/$f"
  done
  for f in 🔣️*.svg 🖼️assets; do
    [[ -e "$old/$f" ]] && mv "$old/$f" "$new/"
  done
  if [[ -d "$old/.vscode" ]]; then
    mkdir -p "$new/.vscode"
    mv "$old/.vscode"/* "$new/.vscode/" 2>/dev/null || true
    rmdir "$old/.vscode" 2>/dev/null || true
  fi
  if [[ -d "$old/🟦️typescript" ]]; then
    for item in "$old/🟦️typescript"/*; do
      [[ -e "$item" ]] && mv "$item" "$new/"
    done
    rmdir "$old/🟦️typescript" 2>/dev/null || true
  fi
  for sql in "$old"/*.sql; do
    [[ -f "$sql" ]] && mv "$sql" "$owner/"
  done
  for doc in README.md AGENTS.md; do
    if [[ -f "$old/$doc" && ! -f "$owner/$doc" ]]; then
      mv "$old/$doc" "$owner/$doc"
    fi
  done
  rm -rf "$old/node_modules" 2>/dev/null || true
  rm -rf "$old" 2>/dev/null || true
}

migrate_go_owner() {
  local owner="$1"
  local old="$owner/⚡️implementations/🐹️go"
  local pkg="$owner/📦️packages/🐹️go"
  [[ -d "$old" ]] || return 0
  mkdir -p "$pkg"
  for f in project.json script.ts README.md AGENTS.md; do
    [[ -f "$old/$f" ]] && mv "$old/$f" "$pkg/$f"
  done
  if [[ -f "$old/go.mod" ]]; then
    mv "$old/go.mod" "$owner/go.mod"
    [[ -f "$old/go.sum" ]] && mv "$old/go.sum" "$owner/go.sum"
  fi
  if [[ -f "$old/main.go" ]]; then
    mkdir -p "$owner/🐹️main"
    mv "$old/main.go" "$owner/🐹️main/🐹️component.go"
  fi
  if [[ -f "$old/main_test.go" ]]; then
    mkdir -p "$owner/🧪️main"
    mv "$old/main_test.go" "$owner/🧪️main/🐹️component_test.go"
  fi
  rm -rf "$old" 2>/dev/null || true
}

migrate_lib_ts() {
  local owner="$REPO/🔨️modules/📚️lib"
  local old="$owner/⚡️implementations/🟦️typescript"
  local new="$owner/📦️packages/🟦️typescript"
  [[ -d "$old" ]] || return 0
  mkdir -p "$new" "$owner/🔍️discovery" "$owner/🗂️workspaces"
  mv "$old/🟦️discovery.ts" "$owner/🔍️discovery/🟦️component.ts"
  mv "$old/🗂️workspaces.ts" "$owner/🗂️workspaces/🟦️component.ts"
  mv "$old/🔣️taxonomy.json" "$owner/🔣️taxonomy.json"
  for f in package.json project.json script.ts tsconfig.json 📦️index.ts 🧪️index.test.ts README.md; do
    [[ -f "$old/$f" ]] && mv "$old/$f" "$new/$f"
  done
  rm -rf "$old/node_modules" 2>/dev/null || true
  rm -rf "$old" 2>/dev/null || true
  sed -i '' 's|from "./🟦️discovery.ts"|from "../../🔍️discovery/🟦️component.ts"|g' "$new/📦️index.ts"
  sed -i '' 's|from "./🗂️workspaces.ts"|from "../../🗂️workspaces/🟦️component.ts"|g' "$new/📦️index.ts"
  sed -i '' 's|join(__dirname, "🔣️taxonomy.json")|join(__dirname, "../../🔣️taxonomy.json")|g' "$owner/🔍️discovery/🟦️component.ts"
}

migrate_lib_go() {
  local owner="$REPO/🔨️modules/📚️lib"
  local old="$owner/⚡️implementations/🐹️go"
  local pkg="$owner/📦️packages/🐹️go"
  [[ -d "$old" ]] || return 0
  mkdir -p "$pkg" "$owner/🐹️repo"
  mv "$old/main.go" "$owner/🐹️repo/🐹️component.go"
  mv "$old/go.mod" "$owner/go.mod"
  [[ -f "$old/go.sum" ]] && mv "$old/go.sum" "$owner/go.sum"
  for f in project.json script.ts README.md AGENTS.md; do
    [[ -f "$old/$f" ]] && mv "$old/$f" "$pkg/$f"
  done
  rm -rf "$old" 2>/dev/null || true
}

migrate_rust_cli() {
  local owner="$REPO/🔨️modules/⌨️cli"
  local old="$owner/⚡️implementations/🦀️rust"
  local new="$owner/📦️packages/🦀️rust"
  [[ -d "$old" ]] || return 0
  mkdir -p "$new"
  mv "$old"/* "$new/"
  rmdir "$old" 2>/dev/null || true
  rmdir "$owner/⚡️implementations" 2>/dev/null || true
  sed -i '' 's|path = ".*📦️lib.rs"|path = "📦️lib.rs"|g' "$new/Cargo.toml"
  sed -i '' 's|path = ".*📦️main.rs"|path = "📦️main.rs"|g' "$new/Cargo.toml"
  if ! grep -q 'metadata.semio' "$new/Cargo.toml"; then
    sed -i '' '/^\[package\]/a\
\
[package.metadata.semio]\
role = "tool"
' "$new/Cargo.toml"
  fi
}

migrate_server_lib() {
  local owner="$REPO/🔨️modules/🖥️server/📚️lib"
  local old="$owner/⚡️implementations/🟦️typescript"
  local new="$owner/📦️packages/🟦️typescript"
  [[ -d "$old" ]] || return 0
  mkdir -p "$new" "$owner/👷worker"
  mv "$old/🟦️worker.ts" "$owner/👷worker/🟦️component.ts"
  mv "$old/📦️index.ts" "$new/📦️index.ts"
  rm -rf "$old" 2>/dev/null || true
  sed -i '' 's|../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts|../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️lib/📦️packages/🟦️typescript/📦️index.ts|g' "$owner/👷worker/🟦️component.ts"
}

# owners with TS only or TS+go
for rel in \
  "🔨️modules/💻️client/⌨️cli" \
  "🔨️modules/💻️client/🔌️mcp" \
  "🔨️modules/💻️client/🧩️vscode" \
  "🔨️modules/💻️client/🪶️sqlite" \
  "🔨️modules/🖥️server/🎛️coordinator"; do
  migrate_ts_owner "$REPO/$rel"
  migrate_go_owner "$REPO/$rel"
done

migrate_lib_ts
migrate_lib_go
migrate_rust_cli
migrate_server_lib

# fix go replace paths to lib module root
for gomod in $(find "$REPO" -name go.mod 2>/dev/null); do
  sed -i '' 's|🔨️modules/📚️lib/⚡️implementations/🐹️go|🔨️modules/📚️lib|g' "$gomod"
  sed -i '' 's|🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go|🔨️modules/💻️client/⌨️cli|g' "$gomod"
done

replace_impl_paths "$REPO"

# project.json cwd/sourceRoot updates
find "$REPO" -name '📋️project.json' -print0 2>/dev/null | while IFS= read -r -d '' f; do
  sed -i '' 's|⚡️implementations/🟦️typescript|📦️packages/🟦️typescript|g' "$f"
  sed -i '' 's|⚡️implementations/🦀️rust|📦️packages/🦀️rust|g' "$f"
  sed -i '' 's|⚡️implementations/🐹️go|📦️packages/🐹️go|g' "$f"
done

# package.json exports
for pkg in "$REPO"/🔨️modules/📚️lib/📦️packages/🟦️typescript/package.json; do
  [[ -f "$pkg" ]] && sed -i '' 's|"\./📦️index.ts"|"./📦️index.ts"|g' "$pkg"
done

# taxonomy area for repo product
TAX="$REPO/🔨️modules/📚️lib/🔣️taxonomy.json"
if [[ -f "$TAX" ]]; then
  python3 -c "
import json, pathlib
p = pathlib.Path('$TAX')
t = json.loads(p.read_text())
t.setdefault('areas', {})['🧰️framework/🛍️products/🦑️repo'] = 'clean'
p.write_text(json.dumps(t, indent=2, ensure_ascii=False) + '\n')
"
fi

echo "Migration complete."
