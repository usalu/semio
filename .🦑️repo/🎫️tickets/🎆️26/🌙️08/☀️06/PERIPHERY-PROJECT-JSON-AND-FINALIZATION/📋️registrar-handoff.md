# Registrar Handoff — W10/W11 Finalization

Ticket `26/08/06/PERIPHERY-PROJECT-JSON-AND-FINALIZATION` · inventory `2026-08-06T12:59:25.597Z` · source `🧪️w9-inventory-snapshot.json`.

## Already applied by periphery (do not re-apply)

| Item | Status |
|------|--------|
| `.dependency-cruiser.cjs` taxonomy path → `📚️lib/🔣️taxonomy.json` | ✅ loads (`dependencyCruiserTaxonomyLoadOk: true`) |
| Root `📜️script.ts` `REPO_CLIENT_GO` / `REPO_MCP_GO` → module roots (`⌨️cli`, `🔌️mcp`) | ✅ matches on-disk `go.mod` |
| `.devcontainer/post-create.sh` MCP go build cwd → `…/🔌️mcp` | ✅ |
| `.gitignore` `**/📦️packages/**/Cargo.lock` | ✅ |
| Stale Shape V2 `📋️project.json` (dsl-derive cwd, os-host namedInputs) | ✅ `stalePackages: []` |
| `go.work` | ✅ already on module roots (0 impl lines) — **no registrar edit** |
| Root `📜️script.ts` repo-lib import | ✅ already `📦️packages/🟦️typescript` |
| Taxonomy norm: package entry = `📦️glue.rs` / `🟦️glue.ts` | ✅ `🔣️taxonomy.json` + policy/registry wording |

## Norm (package entry = glue)

From `PACKAGE-GLUE-ENTRY-RENAME-AND-FAT-EXTRACT/📌️important.md`:

- Rust package entry: `📦️packages/🦀️rust/📦️glue.rs` (`[lib] path = "📦️glue.rs"`)
- TS package entry: `🟦️glue.ts` (re-export only)
- **Forbidden:** package-root `lib.rs` / domain `component.*` inside `📦️packages/`

Live counts: **72** `📦️glue.rs`, **0** `📦️lib.rs` under packages, **0** `🟦️glue.ts`, **49** `📦️index.ts(x)` still under packages (TS glue rename still in flight — `PACKAGE-GLUE` / owning agents).

## Registrar-only patches still pending (do NOT apply until W8/W8d + cargo metadata green)

### 1. `🧪️vitest.config.ts` — drop dead `KNOWN_BROKEN` impl paths

Still listed:

```
- 🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/🧪️vitest.config.ts\n- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/🧪️vitest.config.ts
```

Remove entries whose paths no longer exist after OS/dev sandwiches die; re-run vitest workspace list.

### 2. Plugin registry path in root `📜️script.ts` (~line 1230)

Still references:

`…/🔌️plugin/⚡️implementations/🟦️typescript/📔️registry/…`

Repoint to Shape V2 packages path once OS plugin consolidation lands. **Not applied by periphery** (impl tree still present).

### 3. Root `package.json` / `nx.json` / `Cargo.toml` / `bun.lock`

No periphery single-line fixes required today. After `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION` + OS eradication:

- `bun nx run workspace:workspaces --check`
- apply any generated workspace diff
- `cargo metadata` green before merging registrar PR

### 4. Nested `Cargo.lock` under packages (git index)

Rule is in `.gitignore`. Registrar may `git rm --cached` when safe (4 locks were on disk earlier under framework packages — confirm with `git ls-files "**/📦️packages/**/Cargo.lock"`).

## W10 finalization tripwire flip (single registrar PR)

**Preconditions:**

- OS kernel / host+dev / repo product / singletons / compiler Shape V2 tickets registrar-complete
- `find` for `⚡️implementations` / `⚡️implementation` → **0** outside `compose` exempt
- Live count now: **92** impl dirs (all under `🧰️framework`) — owned by OS/compiler eradication agents; periphery must not delete

**Flip steps:**

1. `🔣️taxonomy.json` `areas` → `"clean"` for framework/hub/mit-bestand/s-modules/plugins when dual-layout gone
2. `.dependency-cruiser.cjs` `noImplSegmentRule()` severity `warn` → `error`
3. Root policy: promote taxonomy/impl breaches from warn to **high**; remove SECONDARY impl-sandwich walk when primary discovery is pure
4. `bun ./📜️script.ts verify gate` — forbiddenPathSegments on tracked paths (excl. compose) → exit 1
5. Vitest `KNOWN_BROKEN` cleanup (above)

## Out of scope (owning agents)

| Item | Owner |
|------|--------|
| Delete remaining **92** `⚡️implementations` trees | `OS-IMPLEMENTATIONS-FULL-ERADICATION` / kernel / compiler tickets |
| Rename remaining **49** TS package `index.ts(x)` → `🟦️glue.ts` | `PACKAGE-GLUE-ENTRY-RENAME-AND-FAT-EXTRACT` |
| ~87 `📋️project.json` still under impl paths | die with sandwiches |
| Physical OS/compiler impl deletion | **other agents — periphery must not delete** |
- Removed 4 nested package Cargo.lock files from disk (gitignore already covers **/📦️packages/**/Cargo.lock).
