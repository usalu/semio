# WP2f — root script partition (rows 133, 138)

Partition: root `📜️script.ts`, root `📋️project.json`, root `package.json`, `.vscode/launch.json`, `nx.json`.
Successor to `📓️wp2e-root-script.md`. Every number below is a real run on this tree, pasted verbatim.

## 1. Per-row results

| Row | Result |
|---|---|
| 133 (schema `audit`/`check` tree walker skips git submodule paths) | **no root-side code exists**; the walker is `walkRepositoryTree` in `📚️library/🔍️discovery/🟦️.ts:2952`, reached from the root script only as `inventorySchemaScopes(this.root, loadCatalogTaxonomy())`. Exact patch for the library worker in §3.2; its effect measured end-to-end in §4.1. Today the `check` half of row 133 is **1 row**, not 8 (§3.1) |
| 138 (`policyMutationAggregateMembers` `leafNames` must accept two-segment leaves) | **already satisfied, verified, no change needed** — `policyStructuralMutationChildren` admits `<domain>/<verb>` candidates whenever `mutationDomainOwners[<root>]` declares the domain, and `mutationOwnerIdentity` resolves them, so architect's 266 two-segment leaves are all `direct-owner` and its 69 domain directories are all `domain-owner`. Measured: 266/266 aggregate branches resolve, 0 domain directories reported as leaves (§4.2) |

<!--RESULT-EXTRA-->

## 2. What changed

### `📜️script.ts`, `📋️project.json`, `package.json`, `nx.json`

**Untouched.** Neither row needs a root-script change: row 133's walker is library-owned (§3.2) and
row 138's depth handling is already correct (§3.3).

### `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`

Two executable commands existed in `📋️project.json` and `package.json` but in neither launch catalog, so
no dev could reach them (CLAUDE.md: every executable command is registered in `launch.json`). Added, in
both catalogs, following the existing `4_build` / `208.x` grouping of the schema block:

| name | command | group | order |
|---|---|---|---|
| `📦️test🧬️schema🧩️compile` | `bun nx run workspace:schema-compile` | `4_build` | `208.8` |
| `📦️generate🧬️schema📤️entries` | `bun nx run workspace:schema-entries` | `4_build` | `208.9` |

`schema compile` is the repo-wide gate (`cargo test -p semio-framework-schema --test schema-module-compile`),
`schema entries` regenerates the tracked registry dump and cross-checks it — the two commands W2e's row 117 /
row 118 work added to `📋️project.json`/`package.json` without a launch entry. Naming follows the block's own
verb prefixes (`📦️test…` for a gate, `📦️generate…` for a command that rewrites a tracked artefact).

<!--CATALOG-VERIFY-->

## 3. Findings

<!--FINDINGS-->

## 4. Verification (real output)

<!--VERIFICATION-->

## 5. Cross-partition requests

<!--REQUESTS-->

## 6. Open questions

<!--OPEN-->
