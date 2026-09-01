# 🌳️ Kind-Only Leaves — Repo-Wide Design and Census

Goal: the whole monorepo follows the domain-driven multi-implementation tree taxonomy where
**folders carry semantics and files are kind-only leaves** (`🦀️.rs`, `🟦️.ts`, `🔣️.json`, …),
regardless of whether the leaf is a component, test, schema, fixture or oracle.

This is the SSOT position already: `🔣️taxonomy.json` declares
`physicalLeafRendering.filename = "file-kind-emoji-and-extension-chain"` and
`_treePurityComment` = *"Files use registered kind-only basenames; semantic concerns live in
registered emoji-plus-slug directories."* The tree has not caught up with the contract.

## 1. Census (authoritative, computed from `git ls-files` + `🔣️taxonomy.json`)

Scripts: `📜️goal-leafname-census.py`, `📜️goal-collision-census.py` (this ticket root).

| class | files | meaning |
|---|---:|---|
| `RENAME` | 43 449 | emoji leaf carrying a stem; kind-only target is free in its directory |
| `SPLIT` | 21 519 | kind-only target collides with a sibling — needs a semantic child directory |
| `EXEMPT` | 4 265 | matched by `fixedFilenameContracts` / `scopedFileKinds` / `pathExclusions` |
| `OK` | 3 025 | already kind-only |
| `UNKNOWN` | 254 | extension not registered in `fileKinds` |

Production-only (excluding `.🧬semio` ticket evidence): **≈41 500 RENAME + ≈6 500 SPLIT**.

Dominant stems to erase: `🦀️component` (12 065), `🔣️component` (9 322), `🟦️component` (7 561),
`📌️empty` (4 346), `🔗️component` (1 488), `🛰️component` (1 487), `📖️component` (491),
`📡️component` (449), and 448 each of `🌶️ 🔠️ 🥋️ 🅰️ 🔤️ component`.

## 2. The engine already implements the target

`🧹️normalization/🟦️.ts`:

- `GENERIC_SEMANTIC_STEMS` (line 865) = `asset assets component components glue test tests implementation impl index`.
  `canonicalFile()` (line 3121) erases these to `<parentCanonical>/<kindEmoji><ext>`.
- A stem that is **not** generic and **not** equal to its parent slug resolves against
  `semanticDirectoryKinds`; on a hit the canonical path becomes
  `<parentCanonical>/<kindEmoji><stem>/<kindOnly>` (line 3132) — semantics pushed into a
  **directory**, exactly as the goal states. On a miss it emits `semantic-stem-unresolved`.
- `planMoves()` (≈7256) stages every entry whose `sourcePath !== normalizedPath` with
  `rationaleRule: "semantic-stem-resolution"`, and `applyTaxonomyPlan()` rewrites references
  (rust `mod`/`use`/`#[path]`, TS imports, Cargo.toml, package.json, `📋️project.json`,
  JSON `$ref`, markdown links).

Verified on `🧰️framework/🔨️modules/🧬️schema`, the engine plans exactly:

```
📦️packages/🦀️rust/📦️glue.rs -> 📦️packages/🦀️rust/🦀️.rs
🟦️component.ts              -> 🟦️.ts
🦀️component.rs              -> 🦀️.rs
```

**No new rename mechanism is needed.** The work is to clear what blocks the engine.

## 3. Blockers

### 3.1 Repository boundary (submodule) — blocks every UNSCOPED run

`♻️mit-bestand/recherche` is a real submodule (gitlink, mode 160000).
`inventoryTaxonomyWithSourceParentPruning` (line 6140) throws
*"Normalization requires an explicit repository-boundary decision before authored classification"*.
This guard is deliberate and a peer is mid-implementation on the designed contract
(`retained-gitlink-boundary`, `authoredSource:false`, `traversal:"terminal"` — see
`SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-*`). **Do not add an escape-hatch flag.**

Workaround that needs no change: every `plan` / `verify` / `inventory` run passes `--scope`
with a production path, which keeps the submodule out of the admitted set. All scoped runs work.

### 3.2 The pipeline poisons its own reference closure — MEASURED

`referenceClosure.candidateSource = "git-tracked-and-untracked-plus-explicit-ticket"`, so even
untracked files are admitted. The pipeline writes `📊️taxonomy-plan/🔣️.json` and
`📊️taxonomy-verification/🔣️.json` **into the ticket folder**; those artifacts contain repository
paths as JSON string values, which the next run admits as unrewritable references.

Same scope, same baseline, real output:

| tree state | result |
|---|---|
| previous run's artifacts present | `moves=3 edits=970 unresolved=504` |
| artifacts moved out of the tree | `moves=3 edits=192 unresolved=124` |

The tool's own output inflates its next run's blocking decisions ~4× and can never converge.
Until this is fixed structurally, **generated plan/verification JSON must be moved out of the
repository before every run**.

### 3.3 Remaining 124 unresolved decisions (scope `🧰️framework/🔨️modules/🧬️schema`)

| code | n | where | disposition |
|---|---:|---|---|
| `reference-syntax-unsupported` | 85 | closed-ticket reports, `.cursor/plans` | freeze as historical evidence |
| `reference-syntax-unsupported` | 28 | plugin `🧬️mutations/🦀️component.rs` | make Rust path joins provable |
| `reference-syntax-unsupported` | 10 | `🗣️dsl/✨️derive`, `🗄️stdio` | make Rust path joins provable |
| `semantic-stem-unresolved` | 3 | `build.rs`, `🔣️entity-kinds.json`, `🦀️validator.rs` | vocabulary decision |
| `frozen-coordinate-evidence-unowned` | 2 | repo library fixture | re-anchor frozen coordinates |
| `collision-*` | 5 codes / 1 case | `🧬️schema/✨️derive` | semantic directory, never a stem |

`reference-syntax-unsupported` in ticket reports must **not** be rewritten — closed-ticket text
records what was true then; rewriting falsifies history. The designed mechanism is
`frozenMarkdownCoordinateEvidenceContracts` / `frozenCoordinateEvidenceContracts`
(`referenceClosure.frozenSourceCoordinates = "exact-digest-and-token-authority"`).

## 4. Direction guardrail (dev instruction, 2026-08-28)

Domain-neutral per-mutation folders — `↩️inverse`, `🔺️diff`, `📸️snapshot`, `🧬️mutations`,
`💡️inferences`, `📝️text`, `💾️binary` — **stay**. Removing them makes the tree less semantic.
Every disambiguation is a **directory**, never a file stem. `SPLIT` cases are resolved by
introducing the right implementation-neutral directory, never by keeping a name on the leaf and
never by merging two semantically distinct files.

Existing neutral vocabulary already covers the goal's list: `🧬️schema`, `🧫️fixtures`,
`🧪️tests`, `🧪️oracle` (`testContributionDirName`), `📚️examples`, `⚙️engine`, `🤖️generated`,
`📇️registry`, `🏗️builder`, `🪓️decomposer`, `👁️viewer`, `✏️editor`.

## 5. Execution order

1. Clear the 124 unresolved decisions (four independent slices, in flight).
2. Fix 3.2 structurally so runs converge without manual parking.
3. Per-scope `plan → apply → verify`, smallest blast radius first, building after each wave.
   Applies must be **serialized** — a plan carries `expectedAffectedPreStateDigest` and
   `sourceTreeDigest`, so concurrent applies fail their digest checks.
4. `♻️mit-bestand` waits on the peer's gitlink boundary contract.

## 6. File placement convention (dev instruction, 2026-08-28)

- Hand-written inputs (scripts, configs) and durable reports: **ticket root**.
- Tool-generated output (command logs, captured JSON, dumps): **`🗑️temp/` at the ticket root**,
  deleted when the ticket closes.

## 7. Findings from the first wave (2026-08-28)

### 7.1 `✨️derive` collision was a TRUE POSITIVE — duplicated implementation

The `collision-*` rows on `🧬️schema/✨️derive/🦀️.rs` were the gate correctly detecting that two
files hold the same source:

```
🧬️schema/✨️derive/🦀️component.rs                 288 lines
🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs   283 lines   # same source, rustfmt wrapping only
```

Same for `💻️os/🔨️modules/🗣️dsl/✨️derive`. Both normalize onto one kind-only destination because
both classify as `implementation`.

The repo's own pattern for a proc-macro crate is glue-as-wiring, implementation at the owner root:

```rust
//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️component.rs`.
#[path = "../../🦀️component.rs"]
mod component;
```

(`🔄️machine/✨️derive` 33 lines, `⏳️async/✨️macros` 15 lines). The two `✨️derive` crates simply do
not follow it. **Resolution is de-duplication, not a taxonomy change** — the collision disappears
once one source of truth remains. Suppressing the hoist in
`packageImplementationDestination` / `inventoryTaxonomyWithSourceParentPruning` would hide real
duplication repo-wide and must be reverted.

Lesson: a normalization collision is evidence about the tree, not noise to silence.

### 7.2 Structural fix for the self-poisoning closure

`explicitTicketRows` now skips a reserved `🗑️temp` child of the ticket root
(`TICKET_GENERATED_OUTPUT_DIRECTORY`), so a pipeline's own generated artifacts can never re-enter
its reference closure. This matches the dev's convention: tool output under `🗑️temp`, deleted
when the ticket closes; hand-written inputs and reports at the ticket root.

### 7.3 Measurements (scope `🧰️framework/🔨️modules/🧬️schema`, artifacts parked)

| state | result |
|---|---|
| baseline | `moves=3 edits=192 unresolved=124` (5 collision rows) |
| with hoist suppressed | `moves=5 edits=192 unresolved=201` (0 collision rows) |

The planned moves are exactly the goal's shape:

```
📦️packages/🦀️rust/📦️glue.rs        -> 📦️packages/🦀️rust/🦀️.rs
✨️derive/📦️packages/🦀️rust/📦️glue.rs -> ✨️derive/📦️packages/🦀️rust/🦀️.rs
✨️derive/🦀️component.rs             -> ✨️derive/🦀️.rs
🟦️component.ts                      -> 🟦️.ts
🦀️component.rs                      -> 🦀️.rs
```

### 7.4 Vocabulary decisions for the three `semantic-stem-unresolved` rows

| file | decision |
|---|---|
| `📦️packages/🦀️rust/build.rs` | `fixedFilenameContracts["cargo-build-script"]`, authority cargo, package-root 🦀️rust — Cargo mandates the basename (7 in repo) |
| `🦀️validator.rs` | new `semanticDirectoryKinds["validator"] = {emoji "✅️"}` → `🧬️schema/✅️validator/🦀️.rs` |
| `🔣️entity-kinds.json` | new `semanticDirectoryKinds["entity-kinds"] = {emoji "🏷️"}`, mirroring `property-kinds`/`attribute-definitions` → `🧬️schema/🏷️entity-kinds/🔣️.json`, and the same stem carries the generated sibling to `🤖️generated/🏷️entity-kinds/🟦️.ts` — one language-neutral directory serving JSON and TypeScript |

### 7.5 Apply is globally serialized

The CLI throws when `plan.unresolved.length > 0`, and that list spans the whole repository's
reference closure regardless of `--scope`. So **every** blocker must clear before **any** scope can
apply. Plans also carry `expectedAffectedPreStateDigest` / `sourceTreeDigest`, so concurrent
applies fail their digest checks — applies must run one at a time.

## 8. Second axis: semantic-collection completeness

`bun ./📜️script.ts generate taxonomy census --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`
runs unscoped (it does **not** hit the submodule guard) and reports the *other* half of
"folders carry semantics": 4 756 components, 22 317 consumer edges, **12 362 problems**,
179 duplicate-evidence clusters.

| code | n |
|---|---:|
| `manifest-child-missing` | 4 756 |
| `member-component-leaf-missing` | 3 533 |
| `collection-manifest-missing` | 1 884 |
| `collection-empty` | 945 |
| `collection-authored-behavior` | 501 |
| `package-implementation` | 226 |
| `packaging-violation` | 192 |
| `module-production-consumer-minimum` | 81 |
| `manifest-without-marker` | 72 |
| others (10 codes) | 172 |

Two notes:

- `package-implementation` (226) is the same rule whose suppression §7.1 rejects. It fires
  repo-wide on real authored implementation sitting inside a package boundary, so it is a live
  purity gate, not an artefact of the two `✨️derive` crates.
- This census writes `📊️semantic-census.json` / `📓️semantic-census.md` to the ticket ROOT. Like
  the plan/verification artifacts they are generated output and must live under `🗑️temp`
  (already gitignored, `.gitignore:21`, and now structurally excluded from the reference closure).

This axis is a separate workstream from kind-only leaves and is not attempted here; it is recorded
so the remaining distance to the goal is not understated.
