# Registrar Handoff — W9 Periphery + W10 Finalization Patch Plan

Periphery ticket `26/08/06/PERIPHERY-PROJECT-JSON-AND-FINALIZATION` — **do not apply** until owning W8/W8d agents close and `cargo metadata` is green. This file is the exact patch plan for registrar-only root edits.

Inventory source: `📋️w9-inventory.md` / `🧪️w9-inventory-snapshot.json`.

---

## A. Root `📜️script.ts` — repo-lib import + Go constants

### A1. Repo-lib barrel import (line ~51)

**Replace:**

```ts
} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
```

**With:**

```ts
} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
```

### A2. Go client paths (lines ~75–76)

After `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION` lands, repoint:

| Symbol | Current | Target (Shape V2) |
|--------|---------|-------------------|
| `REPO_CLIENT_GO` | `…/⌨️cli/⚡️implementations/🐹️go` | `…/💻️client/⌨️cli/📦️packages/🐹️go` (confirm on disk) |
| `REPO_MCP_GO` | `…/🔌️mcp/⚡️implementations/🐹️go` | `…/💻️client/🔌️mcp/📦️packages/🐹️go` |

Grep `⚡️implementations/🐹️go` in `📜️script.ts` after repo ticket closes — expect **zero** hits.

---

## B. `.dependency-cruiser.cjs` — taxonomy path (blocking today)

**Replace** `readFileSync` path (line ~19):

```js
"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🔣️taxonomy.json"
```

**With:**

```js
"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/🔣️taxonomy.json"
```

**Verify:** `node -e "require('./.dependency-cruiser.cjs')"` exits 0.

**Registry / plugin scripts:** `rg '📚️lib/⚡️implementations/🟦️typescript/🔣️taxonomy' 🧰️framework` — repoint any hits to `📚️lib/🔣️taxonomy.json` in the same registrar pass.

---

## C. `go.work` (draft — apply with repo product cutover)

**Current** (4 impl module roots + compose):

```go
use (
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/⚡️implementations/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go
	./compose/client/lib/go
)
```

**Draft target** (paths must match post–Shape-V2 `go.mod` locations from `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION`):

```go
use (
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🐹️go
	./🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🐹️go
	./compose/client/lib/go
)
```

If repo-lib consolidates to a single Go module root at `📚️lib/go.mod`, collapse to that one `use` line instead — **read disk**, do not guess.

---

## D. `.devcontainer/post-create.sh`

**Replace** line ~79:

```sh
go build -o '…/client' './…/🔌️mcp/⚡️implementations/🐹️go'
```

**With** the same MCP package path as `go.work` / `REPO_MCP_GO` after cutover.

---

## E. Root `package.json` / `nx.json`

No stale `print` or `assets` impl workspace entries remain (print already at `📓️print/📦️packages/🟦️typescript`).

**After repo product migration:** run `bun nx run workspace:workspaces --check` (M6 generator) and apply any diff registrar gets from `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION/📋️registrar-handoff.md` when that ticket adds one.

---

## F. Draft `.gitignore` addition (nested workspace locks)

Add under root `.gitignore` (or confirm existing rule covers):

```gitignore
# Shape V2 standalone verification — never commit nested workspace locks
**/📦️packages/**/Cargo.lock
```

Then `git rm --cached` the DELETE-READY locks listed in `📋️w9-inventory.md` §5.

---

## G. W10 finalization tripwire policy (registrar + verify gate)

**Preconditions (all required before flip):**

- `FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION`, `FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION`, `FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION`, `FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH`, compiler human ticket (or explicit exempt) — registrar-complete.
- `find . -type d \\( -name '⚡️implementations' -o -name '⚡️implementation' \\)` → **0** hits outside `compose` exempt areas.

**Flip steps (single registrar PR):**

1. **`🔣️taxonomy.json`** — set `areas["🧰️framework"]`, `areas["🌎️hub"]`, `areas["♻️mit-bestand"]`, `areas["✏️s/🔨️modules"]` to `"clean"`; set `areas["✏️s/🔌️plugins"]` to `"clean"` when no dual-layout crates remain.
2. **`.dependency-cruiser.cjs`** — in `noImplSegmentRule()`, change `severity: "warn"` → `severity: "error"`. Update comment to cite W10.
3. **Root `📜️script.ts`** — promote taxonomy policy breaches from warn/low to **high** (see `policyTaxonomyDirsBreaches` / related regions: remove `policyNewSurfacePriority` downgrade; delete SECONDARY impl-sandwich walk in `policyDiscoverCrateDirs` per comment at ~1701).
4. **`bun ./📜️script.ts verify gate`** — add or tighten check: any `forbiddenPathSegments` match in tracked source paths (excluding `compose`) → **exit 1**.
5. **`🧪️vitest.config.ts`** — remove `KNOWN_BROKEN` entries whose paths no longer exist; re-run vitest workspace list.

**Post-flip invariant (no-impl-segment error):**

> No import graph edge and no on-disk path under a semio-owned area may contain a path segment equal to `⚡️implementations` or `⚡️implementation`. Violations are **errors** in dependency-cruiser, root policy, and verify gate — not warnings.

---

## H. Out of scope for registrar (owning agents)

| Item | Owner |
|------|--------|
| 89× `📋️project.json` under OS/compiler impl | OS/kernel/compiler tickets (bulk-repoint `cwd` to `📦️packages` or delete projects when sandwiches removed) |
| 2× stale Shape V2 `📋️project.json` | `FRAMEWORK-OS-HOST-AND-DEV`, DSL derive agent |
| Plugin registry under `…/plugin/⚡️implementations/🟦️typescript` | OS plugin consolidation |
| `.cursor/plans` stale links | Optional docs hygiene |
