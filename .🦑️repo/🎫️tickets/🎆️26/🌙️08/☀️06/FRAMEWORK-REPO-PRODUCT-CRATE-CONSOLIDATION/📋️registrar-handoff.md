# 📋️ Registrar Handoff — Framework Repo Product Crate Consolidation (W8d)

Exclusive migration completed under `🧰️framework/🛍️products/🦑️repo/**`. **Do not edit** those trees in this pass — only root workspace files and cross-repo import strings below.

## 1. Root `Cargo.toml` — `[workspace] members`

**Remove:**
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/⚡️implementations/🦀️rust",
```

**Add:**
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust",
```

(Crate name unchanged: `semio-framework-repo-cli`.)

## 2. Root `package.json` — `workspaces` array

**Remove** (5 lines):
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚡️implementations/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/⚡️implementations/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚡️implementations/🟦️typescript",
```

**Add** (5 lines):
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript",
```

Alternatively run (after merge): `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📜️script.ts workspaces --write`

## 3. Root `go.work` — `use` block

**Replace** the four repo-product module lines:

```diff
-	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go
-	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/⚡️implementations/🐹️go
-	./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🐹️go
-	./🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go
+	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli
+	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp
+	./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib
+	./🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator
```

(`go.mod` now lives at each **owner root** per Shape V2 Go contract; `📦️packages/🐹️go` holds only `📋️project.json` + `📜️script.ts`.)

## 4. Cross-cutting import substring (repo product only — **done**)

Within `🦑️repo/**`, all `⚡️implementations/<lang>` paths now read `📦️packages/<lang>` (or Go owner roots). **Outside** `🦑️repo/**`, run a targeted replace on import strings that still reference the old repo-lib path:

**Old substring:**
```
🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript
```

**New substring:**
```
🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript
```

High-impact files (non-exhaustive — grep the old substring repo-wide):

- Root `📜️script.ts` and `script.ts` (policy spawn exempt set line ~3122)
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`
- Every `**/📜️script.ts` that imports `BundleScript` from repo-lib (~40 plugin/framework packages)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts`

## 5. nx / launch.json

- Project **names** unchanged (`@semio-tech/repo-lib`, `repo-go-lib`, `@semio-tech/repo-cli-rs`, `@semio-tech/repo-coordinator`, etc.).
- Regenerate or bulk-update `sourceRoot` / `options.cwd` if nx cache still points at `⚡️implementations` (in-repo `📋️project.json` files already updated for rust cli + lib ts/go).

## 6. Post-registrar verification

```bash
bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📜️script.ts test quick
GOWORK=on go test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/...
cargo check -p semio-framework-repo-cli
bun ./📜️script.ts verify gate   # orchestrator
```

## 7. Taxonomy

`🔣️taxonomy.json` moved to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/🔣️taxonomy.json`; `areas["🧰️framework/🛍️products/🦑️repo"]` set to `"clean"`.
