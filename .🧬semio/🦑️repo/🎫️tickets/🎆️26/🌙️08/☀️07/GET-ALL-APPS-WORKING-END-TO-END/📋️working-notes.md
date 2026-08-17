# Working Notes — Get All Apps Working End to End

## Root Causes Found & Fixed

### 1. Missing workspace symlink — @semio-tech/framework
- Problem: node_modules/@semio-tech/framework symlink pointed to wrong path
- Fix: Recreated symlink -> ../../🧰️framework/📦️packages/🟦️typescript

### 2. Missing workspace entries — 4 CAD extension packages  
- Problem: package.json workspaces missing 4 CAD extension packages
- Fix: Added aec-building, aec-building-energy, aec-building-structure, spatial-shape

### 3. @semio-tech/flow-core (WASM not built)
- Problem: flow/core/pkg WASM build output doesn't exist
- Fix: Removed from renderer package.json static deps (dynamically imported at runtime)

### 4. Stale repo-lib symlink
- Problem: node_modules/@semio-tech/repo-lib -> old 📚️lib path (renamed to 📚️library)
- Fix: Recreated symlink to correct path

### 5. repo/lib/js/index.ts imports in compose scripts
- Problem: 23 files used relative repo/lib/js/index.ts imports (old path)
- Fix: Bulk replaced all with @semio-tech/repo-lib

### 6. 109 broken symlinks in node_modules/@semio-tech/
- Problem: Old symlinks from plural emoji dirs restructuring
- Fix: Removed all broken symlinks

### 7. Truncated compose/client/lib/rs/lib.rs
- Problem: File committed at exactly 256KB - truncated mid-function
- Fix: Restored full content from git commit 3b1d1c984d

## Verification Status
- bun install: PASSES
- MCP server: STARTS OK (JSON-RPC responding)
- Framework OS dev: STARTS (http://127.0.0.1:6066/)
- s playground: STARTS with extensions
- compose-js build: PASSES
- compose/graphql build: IN PROGRESS (lib.rs restored)
- JS tests: RUNNING
