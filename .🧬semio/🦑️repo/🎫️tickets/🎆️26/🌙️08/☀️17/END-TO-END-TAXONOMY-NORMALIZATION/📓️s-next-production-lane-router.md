# Next Production Lane Router

## Decision

The next production work should not restart with a full inventory. Four exact authorities are already ready to consume, while package purity and the remaining semantic leaves still need owner-specific destination authority. The safe order is:

1. transact the exact CAD/Draw projection;
2. integrate and transact the exact README/LICENSE projection;
3. freeze package-purity destinations in parallel with integrating the WGPU/JCO package projection, then co-plan those package changes;
4. close the small exact fixed/evidence owner cohorts;
5. use scoped inventory pushdown to partition the remaining semantic-leaf population by owner/generator before the final full census.

All live moves remain gated by transaction-v2 signoff. Actual `compose/**` and `temp/compose/**` were not enumerated, traversed, read, or modified. No Git state, production file, taxonomy, discovery, normalization, or transaction file was changed by this census.

## Current bounded observation

Observation date: `2026-08-27`.

The bounded Git-admitted population consists only of repository roots and explicit non-Compose production/tool roots: `.cargo`, `.codex`, `.devcontainer`, `.github`, `.kiro`, `.storybook`, `.vscode`, `✏️s`, `🧰️framework`, `♻️mit-bestand`, `🌎️hub`, and exact repository-root fixed/configuration leaves.

| Datum                                                       |                                                      Current value |
| ----------------------------------------------------------- | -----------------------------------------------------------------: |
| Admitted bounded files                                      |                                                             45,463 |
| NUL-delimited Git-ledger SHA-256                            | `c2403b2876d59ca45cd5321aebbd361334735c50d54525c174555f3c5bb95fdb` |
| Taxonomy schema                                             |                                                                  7 |
| Taxonomy SHA-256                                            | `7bf866f53921e22ae0f514db3ba7bc19d83c2ab5e56991ccb6d5c468ee15e975` |
| Fixed/configurable winners in the bounded resolver census   |                                                                666 |
| Already canonical physical leaves                           |                                                                 10 |
| Noncanonical renameable leaves under scope-aware resolution |                                                             44,786 |
| Leaves with no global kind or fixed/configurable authority  |                                       1: `♻️mit-bestand/recherche` |

The semantic-leaf count is not additive with the exact projection and package counts below: those cohorts are subsets of the same physical population. A more conservative basename-authority lower bound, which generously excludes every fixed/configurable basename even when its scope does not match, is 44,747. Its leading extension chains are `.rs` 12,513, `.json` 10,546, `.ts` 8,038, `.md` 4,359, `.graphql` 1,531, and `.proto` 1,529. Exact token hits include `component` 36,135, `test` 346, `glue` 131, and `index` 72.

## Reconciled completed authority

These items are not new production lanes:

| Completed authority               | Current evidence                                                                                                                                                                                        |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| External fixed filenames          | Canonical `CNAME`, `Caddyfile`, and `Dockerfile` are present; all three prefixed source paths are absent.                                                                                               |
| Nx-owned package/config manifests | `nx-owned-node-package-manifest` currently wins 77 paths and `nx-owned-typescript-config` wins 15. This subsumes the former 13 `package.json` plus two `tsconfig.json` scope misses.                    |
| Cargo cache target authority      | 44 direct ticket target markers, 10 `wasm32-unknown-unknown` markers, and 11 `wasm32-wasip2` markers now resolve. Six exact evidence markers remain intentionally unresolved.                           |
| VS Code publisher identity        | VSCE/Bun/Nx identity and packaging are closed. Its fixed package `LICENSE.md` remains part of the README/LICENSE taxonomy-integration lane, and its package source roles remain part of package purity. |

## Ranked actionable lanes

### 1. CAD and Draw exact projection

| Measure                               | CAD | Draw | Total |
| ------------------------------------- | --: | ---: | ----: |
| Frozen file mappings                  | 209 |   11 |   220 |
| Current source mappings present       | 209 |   11 |   220 |
| Current destination mappings present  |   0 |    0 |     0 |
| Current live path-bearing occurrences |  77 |   23 |   100 |

- Historical source roots: [frozen CAD `/projections/0/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/0/sourceRoot) and [frozen Draw `/projections/1/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/sourceRoot).
- Owner contracts: `artifact-example-model-catalog-v1` and `artifact-editor-command-bundle-v1`.
- Reference authority: the engine owns 76 audited CAD occurrences plus the adjacent Rust `Path::join` occurrence, and 23 Draw occurrences across JSON, TOML, Rust, TypeScript, Nx, and the root policy router.
- Golden: `🧪️cad-draw-path-projection/🔣️.json`, SHA-256 `1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0`.
- Blocker: no mapping/authority blocker remains. Live apply waits only for transaction-v2 signoff and must be followed by the package-purity reclassification because some Draw descendants remain package leaves.

### 2. README and LICENSE exact owner projection

| Disposition                                 | Count |
| ------------------------------------------- | ----: |
| Exact owner documentation projection        |    28 |
| Exact third-party attribution projection    |     4 |
| Exact configurable owner-license projection |     1 |
| Exact ticket evidence/scratch projection    |     3 |
| Preserve exact publishable package basename |     4 |
| Total                                       |    40 |

All 40 current sources remain present. The 36 configurable destinations are absent; the four fixed cases naturally have identical source/destination identity.

- Source roots: the exact golden spans `.devcontainer`, three ticket evidence paths, 11 `✏️s` owner roots, and framework/repo/package/asset owners. No wildcard root is authoritative.
- Owner contracts to add: exact `📃️readme/📝️.md` and `⚖️license/📝️.md` projections plus only the four exact package publisher contracts.
- Reference owners: repo CLI Go dev-doc discovery, the CommonMark scratch Rust reader, the Markdown relative-reference adapter, Bun package publication, VS Code package selection, and exact asset-distribution owners.
- Generator owner: `@semio-tech/assets:build` must change its registered output from `🧰️framework/🔨️modules/🖼️assets/README.md` to `🧰️framework/🔨️modules/🖼️assets/📃️readme/📝️.md` before regeneration.
- Golden: `🧪️readme-license-owner-authority/🔣️.json`, SHA-256 `051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4`.
- Blocker: taxonomy currently has neither the two exact projection contracts nor the four exact publisher contracts. Add them without restoring a repository-wide nested README/LICENSE blanket.

### 3. Package-purity owner destinations

The current bounded package corpus is 1,028 admitted files and 605 source leaves.

| Source role    | Count |
| -------------- | ----: |
| Implementation |   269 |
| Unresolved     |    23 |
| Tool metadata  |   171 |
| Declaration    |   119 |
| Registration   |    23 |

| Owner family                    | Implementation | Unresolved |
| ------------------------------- | -------------: | ---------: |
| `🧰️framework/🔨️modules/🖱️ui`    |            124 |          4 |
| `🧰️framework/🛍️products/💻️os`   |             48 |          5 |
| `🧰️framework/🛍️products/🦑️repo` |             32 |          2 |
| `✏️s` plugins/modules           |             24 |         10 |
| Other `🧰️framework`             |             29 |          2 |
| `🌎️hub`                         |              9 |          0 |
| `♻️mit-bestand`                 |              3 |          0 |

- Existing contracts: all six language boundary rules are present, including JavaScript and the native C/C++ profile; 172 package `📜️script.ts` leaves have explicit source dispositions, of which 171 currently validate as tool metadata.
- Highest exact roots: UI WGPU 28 implementation plus one unresolved, renderer WGPU 24 plus two, coordinator TypeScript 18, UI contract Rust 14, and UI render Rust 12.
- Required owner work: freeze an exact semantic destination beside each package boundary, preserve only configuration/declaration/registration/thin adapter leaves below `📦️packages`, and carry source/import/project/Cargo/Nx references through the existing adapter registry.
- Blocker: destination authority is not frozen for the 269 implementations or 23 unresolved leaves. The package scanner must consume the same Git-admitted population as inventory; ignored Cargo outputs must not inflate acceptance counts.

### 4. WGPU and JCO structural package projection

| Package       | Frozen mappings | Current source | Current destination | Exact live references |
| ------------- | --------------: | -------------: | ------------------: | --------------------: |
| Renderer WGPU |              32 |             32 |                   0 |       191 in 18 files |
| JCO guest     |               4 |              4 |                   0 |          3 in 3 files |

- Source roots: `…/renderer/engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` and `…/fixtures/🔌️jcoprobe/👽️guest`.
- Destination roots: `…/renderer/engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust` and `…/fixtures/🔌️jcoprobe/👽️guest/📦️packages/🦀️rust`.
- Reference owners: 106 authored plus 85 generated WGPU occurrences; two authored plus one generated JCO occurrence. Generated owners are the launch seed/renderer, root `package.json` for `bun.lock`, and root `📜️script.ts` for `🔒️dependencies.json`.
- Golden: `🧪️nested-cargo-package-authority/🔣️.json`, SHA-256 `88619870710263fe4e968714a903dace8c2dc79c619fdc5df87950cf21653972`.
- Blocker: taxonomy does not yet contain either exact structural projection. This projection must be co-planned with package purity: the current renderer root alone contains 24 implementation and two unresolved source leaves, and moving the JCO component beneath a package boundary must not become a new purity exception. The known staged/worktree golden discrepancy remains read-only evidence and must not be repaired with Git.

### 5. Small fixed and retained-evidence owner cohorts

| Cohort                         | Current exact count | Owner decision                                                                                                                                   |
| ------------------------------ | ------------------: | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Nested ticket manifests        |       19/19 present | Repo-MCP embedded-ticket relocation authority                                                                                                    |
| Ticket scratch Go module files |         2/2 present | Exact retained-evidence normalization                                                                                                            |
| Ticket progress documents      |         2/2 present | Exact semantic evidence relocation                                                                                                               |
| Unowned Cargo cache markers    |                   6 | Three historical unprefixed target roots plus three leaked transaction-fixture repositories; retain/remove only through exact evidence authority |
| Micro-commit Bun pin           |                   1 | Register one repository-governance text destination and update the TS writer/test plus Go reader atomically                                      |
| Ticket `_tmp/package.json`     |                   1 | Exact generated/evidence disposition; do not add an Nx/package wildcard                                                                          |

The pin mismatch is still live: TypeScript owns `compose-micro-commit-bun`, while Go reads `🐹️compose-micro-commit-bun`. This is a repository metadata filename only; it provides no authority to restore or access the intentionally deleted Compose tree.

- Blocker: each cohort lacks one exact destination/retention decision. The existing rejection contracts correctly keep all 23 ticket paths fail-closed and must not be weakened.

### 6. Remaining semantic-leaf partition

After the exact lanes above are accounted for, do not emit a single 44,786-row blind rename plan. Partition by owner/generator using scoped inventory pushdown.

| Bounded root                 | Current noncanonical leaves under scope-aware resolution |
| ---------------------------- | -------------------------------------------------------: |
| `✏️s`                        |                                                   41,388 |
| `🧰️framework`                |                                                    3,058 |
| `♻️mit-bestand`              |                                                      208 |
| `.storybook`                 |                                                       71 |
| `🌎️hub`                      |                                                       27 |
| Remaining bounded tool roots |                                                       34 |

- Owner contracts: artifact/plugin manifests, mutation/example catalogs, schema/asset registries, Storybook and other generator preview contracts, then exact external-tool contracts.
- Blocker: this count intentionally overlaps CAD/Draw, README/LICENSE, WGPU/JCO, and package leaves. A scoped post-integration census must subtract converged exact authorities before assigning new owner work. The retained pre-transaction inventory must not be reused as current acceptance evidence.

## Evidence commands

Representative read-only commands:

```text
git ls-files --cached --others --exclude-standard -z -- \
  .cargo .codex .devcontainer .github .kiro .storybook .vscode \
  '✏️s' '🧰️framework' '♻️mit-bestand' '🌎️hub' \
  AGENTS.md Cargo.toml Cargo.lock LICENSE.md README.md package.json \
  pyproject.toml tsconfig.json go.work go.work.sum nx.json \
  '📋️project.json' '📜️script.ts'

bun -e '<load taxonomy; classify only Git-admitted files below literal 📦️packages/<language> boundaries>'

bun -e '<resolve fixed winners with exact repository-root, package-root, adjacent Nx-project, parent-kind, and fixed-parent contexts>'

bun -e '<load the three exact goldens; count source/destination existence and exact reference-token preimages>'

git ls-files --cached --others --exclude-standard -z -- \
  ':(glob).🧬semio/🦑️repo/🎫️tickets/**/CACHEDIR.TAG'

rg -o -F '<exact CAD/Draw source marker>' <exact owner/reference files only>
```

Every path-derived content read followed lexical admission through the explicit bounded roots or one exact golden/rejection identity. No full inventory was run.
