# W9 Periphery Inventory

Ticket `26/08/06/PERIPHERY-PROJECT-JSON-AND-FINALIZATION` · goal `🎯aioptimizedrepo` · snapshot `🧪️w9-inventory-snapshot.json` (regenerate: `bun ./📜️script.ts inventory`).

## 1. `⚡️implementations` / `⚡️implementation` (repo-wide)

| Top area | Count |
|----------|------:|
| `🧰️framework` | **99** |
| `✏️s`, `🌎️hub`, `♻️mit-bestand`, `compose` | **0** |

Plugins and hub sandwiches are **gone** from disk. All remaining impl segments live under framework.

### Framework breakdown (OS kernel families)

| Bucket | Dirs |
|--------|-----:|
| `os/🛢️db` | 24 |
| `os/🗣️dsl` | 16 |
| `os/📡️protocol` | 13 |
| `os/♾️infinite` | 12 |
| `os/🎒️pack` | 10 |
| `os/🏪️store` | 3 |
| `os/🌊️flow` | 3 |
| `os/🧠️neural` | 2 |
| `os/🔌️plugin` | 2 |
| `modules/📚️compiler` | 6 |
| other OS singletons (`🧑️‍💻️dev`, `🌿️vcs`, `🪐️space`, …) | 1 each |
| `products/💻️os` (product-level `⚡️implementations`) | 1 |

Plus **1** singular `⚡️implementation` dir: `…/🌊️flow/🫀️core/pkg/⚡️implementation` (wasm-pack layout).

**Note:** `🦑️repo` product Go/TS sandwiches were **8** dirs in an earlier `find` pass; they are **not** in the current 99-count (likely removed mid `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION`). Re-run inventory after that ticket lands.

**Ownership:** OS/kernel/repo/compiler consolidation tickets own deletion of these trees — not periphery.

## 2. `📋️project.json` still tied to impl paths

| Metric | Count |
|--------|------:|
| Files referencing `⚡️implementations` / `⚡️implementation` | **91** |
| `📋️project.json` **under** an impl sandwich (expected until cutover) | **89** |
| Shape V2 `📦️packages/…/📋️project.json` with **stale** impl references | **2** |

### Stale Shape V2 project.json (fix with owning OS agents, not registrar)

1. **stale `cwd`** — `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📋️project.json`  
   All test targets still `cwd` → deleted `…/✨️derive/⚡️implementations/🦀️rust`.  
   **Fix:** repoint `cwd` to `…/✨️derive/📦️packages/🦀️rust` (or `{projectRoot}`).

2. **stale `namedInputs`** — `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📋️project.json`  
   Still globs `{workspaceRoot}/🧰️framework/🛍️products/💻️os/⚡️implementations/**/*.rs`.  
   **Fix:** drop impl glob; use host component tree + `{projectRoot}` only (`FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION`).

### Noise (not nx projects — ignore for W9)

Six `📋️project.json` files under `♻️mit-bestand/🧺️demonstrator/dist/asset/**` match the impl grep via bundled asset metadata — not migration blockers.

## 3. `go.work`, devcontainer, vitest, taxonomy consumers

| Surface | Status |
|---------|--------|
| **`go.work`** | **4** `use` lines still under `🦑️repo/…/⚡️implementations/🐹️go` (cli, mcp, lib, coordinator). Draft replacement in `📋️registrar-handoff.md`. |
| **`.devcontainer/post-create.sh`** | `go build` still targets `…/🔌️mcp/⚡️implementations/🐹️go`. |
| **`🧪️vitest.config.ts` `KNOWN_BROKEN_IN_AGGREGATOR`** | **4** entries; **2** paths still under impl sandwiches (OS product TS + OS dev TS). Two others are package-resolution bugs (`@semio-tech/assets`, `@semio-tech/animate-present-core`). Revisit after OS host/dev consolidation — paths may move or configs delete. |
| **`🔣️taxonomy.json`** | Canonical file: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/🔣️taxonomy.json`. Old `…/📚️lib/⚡️implementations/🟦️typescript/🔣️taxonomy.json` **missing**. |
| **`.dependency-cruiser.cjs`** | **Broken:** still `readFileSync` old taxonomy path → `ENOENT` on load (`dependencyCruiserTaxonomyLoadOk: false`). |
| **Root `📜️script.ts`** | Imports repo-lib from deleted `…/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts`; live entry is `…/📚️lib/📦️packages/🟦️typescript/📦️index.ts`. Constants `REPO_CLIENT_GO` / `REPO_MCP_GO` still use impl Go paths. |

## 4. Stale markdown links

| Area | `.md` files mentioning `⚡️implementations` |
|------|---------------------------------------------|
| `.cursor/plans` | **36** |
| `🧰️framework`, `✏️s`, tickets (excl. `.🦑️repo` in script) | **0** |

Plans are historical; no user-docs drift in framework/plugins. Optional cleanup wave: `.cursor/plans` only.

## 5. Safe deletions — DELETE-READY (closed family tickets)

**Old sandwich directories** called out in closed W8 handoffs are **already absent** on disk (verified 2026-08-06):

- `🖼️assets`, `📓️print`, `🧰️framework/⚡️implementations/🦀️rust`, `#⃣hash` / `🧬️schema` / `✍️editor` impl rust, `🌎️hub/⚡️implementations`, `🏗️fem` plugin impl.

Nothing left to `rm -rf` for those paths.

### DELETE-READY: git-tracked nested `Cargo.lock` (verification-artifact class)

Safe once owning crate is workspace-registered and overlays removed (registrar housekeeping):

| Path | Source ticket |
|------|----------------|
| `🧰️framework/📦️packages/🦀️rust/Cargo.lock` | `FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH` |
| `🧰️framework/🔨️modules/#⃣hash/📦️packages/🦀️rust/Cargo.lock` | same |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.lock` | same |
| `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.lock` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust/Cargo.lock` | OS host consolidation |

**Not DELETE-READY:** any path still listed as a root `Cargo.toml` workspace member until registrar cutover + green `cargo metadata`.

## 6. W10 finalization tripwire (draft policy for registrar)

See full registrar wording in `📋️registrar-handoff.md` § W10. Summary:

1. **Disk:** `find` count for `⚡️implementations` and `⚡️implementation` = **0** (exempt `compose` only if explicitly still `exempt` in taxonomy).
2. **Lint:** `.dependency-cruiser.cjs` `no-impl-segment` **warn → error** after taxonomy path fix.
3. **Policy:** root `📜️script.ts` taxonomy rules **warn → high**; remove SECONDARY impl-sandwich burn-down loop in `policyDiscoverCrateDirs`.
4. **Areas:** `🔣️taxonomy.json` `areas` for `🧰️framework`, `🌎️hub`, `♻️mit-bestand`, `✏️s/🔨️modules` → `clean` (plugins already `mixed` → `clean` when dual-layout gone).
5. **Verify gate:** `bun ./📜️script.ts verify gate` must fail if any forbidden segment remains in a **dependency-resolved** path or on-disk owner tree (not merely warn).

## 7. Top remaining blockers (ordered)

1. **In-flight OS / repo / compiler / singletons registrar cutovers** — 99 impl dirs are expected until those tickets finish.
2. **Broken repo-lib import path** on root `📜️script.ts` (and any consumer still pointing at `…/lib/⚡️implementations/…`).
3. **Broken `.dependency-cruiser.cjs`** taxonomy load (blocks W10 promotion).
4. **`go.work` + devcontainer** still building from impl Go module roots.
5. **2 stale `📋️project.json`** under OS Shape V2 packages (derive + host).
6. **Human-gated compiler ticket** (`26/08/05/COMPILER-MODULE-CALL-SITE-SWAP-AND-TYPST-EVICTION`) — 6 compiler impl dirs remain.
