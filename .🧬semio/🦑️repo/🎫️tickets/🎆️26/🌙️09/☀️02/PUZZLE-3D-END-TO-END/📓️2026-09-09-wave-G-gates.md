# ⚖️ Wave G — Publication-Authority Audit, Retained-Jobs Fixtures, Interactivity/Repo Gates

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-G. 2026-09-09, 00:20–01:40 CEST. No Rust file was
edited (waves W-S/W-F own `✏️editor/**`, `🎮️commands/**`, `🔌️plugin/🦀️.rs`); every change below is
TypeScript audit code, a JSON fixture, or a JSON schema. No git state-modifying command was run; the
repo's own auto-commit `5dae35ae71` (00:35) captured the first half of this wave's edits.

## 1️⃣ `publication-authority-audit` — red for all three owners → green for all three

`bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit [owner]`

| owner | before | after | admitted ids |
|---|---|---|---|
| `Puzzle3dPlayApp` | ❌️ `publication authority diverged from the fixture` | ✅️ exit 0 | **62** |
| `Puzzle2dPlayApp` | ❌️ same | ✅️ exit 0 | 38 |
| `Puzzle5dPlayApp` | ❌️ same | ✅️ exit 0 | 13 (its 41 batch-only routes stay unadmitted by design) |
| no argument (all three) | ❌️ | ✅️ exit 0 | 113 |

Four independent defects, all in the audit (`📦️packages/🟦️typescript/📜️script.ts`), found with
`🔍️diagnose-publication-authority.ts` + `🔍️trace-publication-authority.ts` (both added to this ticket
folder; the second copies the audit out, wraps `production` in a logging proxy and numbers every
`return false`, which is how the opaque "diverged" message was localised):

1. **Dead `variant` helper — lane comparison (all three owners).** `exactContracts` compared the
   fixture's kebab lanes (`"host-only"`, `"artifact"`) against the Rust variant names the source scan
   produces (`HostOnly`, `Artifact`). The `variant()` helper existed (added with the kebab-ification in
   `025ec86a42`, 2026-09-08 20:58, when the fixture's lanes went `"HostOnly"` → `"host-only"`) but was
   never called — `grep -n variant` returned only its own declaration. Sibling plugin `🧱️block`'s audit
   shows the intended form (`exact(contracts, app.routes.map(({ id, lanes }) => …lanes.map(variant)…))`).
   Fixed: `group.lanes.map(variant)`.
2. **Same bug for the disposition.** `expectedPairs` compared fixture `"migrated"` against source
   `Migrated` — clause #5 of `ownerOracle` (line 122), the structural clause the trace named for all
   three owners. Fixed: `variant(group.status)`. (1) and (2) together are why 5d failed even though
   every one of its literal source anchors was present.
3. **Puzzle3d's two locale anchors (peer's OS-wide locale move, `de617a7c17`).**
   `matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")` and
   `matches!(value.as_str(), "native" | "reuse")` no longer exist anywhere in the 3d editor
   (`grep -c "Locale\|Terminology"` → **0**; 2d keeps only a *read* of the projected
   `puzzle2d_config_locale(view_state)`, 5d has none). They were replaced by the invariant that still
   exists in the same Config preparation, i.e. the exact Config mutation envelope:
   `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 32_768`,
   `Puzzle3dConfigMutation::Snapshot { config } => puzzle3d_config_store_bounded_bytes(config).ok(),`
   and `return Err("Puzzle3d Config preparation rejected its exact mutation envelope".into());`.
   The OS-owned design is now asserted **positively** for every owner by a new top-level
   `localeIsOsOwned(production)` clause: no puzzle owner may carry `"setLocale"`, `"setTerminology"`,
   `ConfigMutation::SetLocale` or `ConfigMutation::SetTerminology`. That is strictly stronger than the
   two anchors it replaces — re-introducing an artifact-local locale now fails the gate.
   The matching hostile mutation `widenedTerminology` (which had become a silent no-op, and would have
   made the audit throw "accepted a widened envelope" once the rest was green) was replaced by
   `widened Config mutation envelope`: collapsing
   `Puzzle3dConfigMutation::Snapshot {…} => …ok(), _ => None,` to `_ => Some(0),`. All five Puzzle3d
   hostiles moved into a named, documented `puzzle3dHostileSources()` map and each one is now asserted
   to **actually apply** (`hostile === source` → `Puzzle3d hostile mutation for <invariant> did not
   apply`), so this class of rot cannot recur silently.
4. **Puzzle2d's two dispatch-emit call shapes.** The audit anchored
   `puzzle2d_dispatch_emit(command, snapshot.0.clone(), …)` / `(command, before, …)`; the source (HEAD
   *and* working tree, lines 1678/1717/3581) passes borrowed roots since the peer changed the helper to
   `before: &Value`: `(command, &snapshot.0, config, &selection, None)` and
   `(command, &doc.snapshot.0, config, interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned())`.
   Fixed to the live spellings — the invariant (both retained entry points route through the *single*
   `puzzle2d_dispatch_emit` pipeline) is unchanged.

Nothing was relaxed: the four hostile-fixture mutations, the Ajv schema validation, `fixtureOracle`,
the manifest/retained/proof/contract bijections, the 5d reserved-route guard order and the
`missing publication contract` mutation all still run and still pass. A peer concurrently widened the
lane vocabulary (`"interaction"` in the TS union, `ArtifactToolPublicationLane::Interaction` in the
framework and `Interaction` in `publicationContracts`' regex) while this wave was running; the audit
stays green with it and puzzle's fixture uses no interaction lane yet.

## 2️⃣ Retained-jobs fixtures — stale `toolIds` in 3d **and** 5d

The consuming oracle is `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs`
(`language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`, line 224 →
`assert_fixture` line 168: `assert_eq!(actual.tool_ids, expected.tool_ids)`, where `expected` is
`PUZZLE{2,3,5}D_RETAINED_TOOL_IDS`) plus
`…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:53`
(`assert_eq!(fixture.get("toolIds"), Some(&Value::Array(PUZZLE3D_RETAINED_TOOL_IDS…)))`).
So `toolIds` must be the Rust constant **as an ordered list**, not a subset. `evidenceToolIds` is a
different field (the ids the `vectors` may reference) and is deliberately *not* set-equal to `toolIds`
— puzzle2d ships 13 evidence ids for 37 routes — so it was left untouched; no field was invented.

| fixture | before | after |
|---|---|---|
| `🗿️artifacts/🧊️3d/…/🗄️retained-jobs/🔣️.json` | 2 ids (`openAddObjectDialog`, `worldPointerDown`) | **62**, in `PUZZLE3D_RETAINED_TOOL_IDS` order |
| `🗿️artifacts/🖐️5d/…/🗄️retained-jobs/🔣️.json` | 2 ids (`canvasPointerDown`, `worldPointerDown`) | **9**, in `PUZZLE5D_RETAINED_TOOL_IDS` order |
| `🗿️artifacts/◻️2d/…/🗄️retained-jobs/🔣️.json` | already exact (37) | untouched |

Verified without cargo by `🔍️verify-retained-jobs-fixtures.ts` (added to this ticket folder), which
replicates every assertion of the Rust oracle from source: ordered `toolIds` ≡ the Rust constant,
`owner`/`documentSchema`/`payloadSchema`, all seven `capacities` against
`PUZZLE_COMMAND_{RAW_BYTES,DECODED_ITEMS,WORK_ITEMS,OUTPUT_BYTES,STEP_MICROS,CHECKPOINT_BYTES}`
(8192/512/4096/262144/7500/112), `locales == {de,en}` with a `cancel` label each, the 16 base vector
ids as a prefix, all 6 checkpoint vector ids with their exact `checkpointBytes`/`expected`, unique
vector ids, the base fingerprints (`8193:0:0:0:0:0`, `1:0:0:0:0:0`, `1:1:1:0:0:0`, `0:0:0:0:0:0`) and
every `toolId`/`toolIds` in `vectors` ⊆ `evidenceToolIds`.

```
bun .🧬semio/…/PUZZLE-3D-END-TO-END/🔍️verify-retained-jobs-fixtures.ts   → exit 0
  puzzle2d (rust 37 ids, fixture 37) · puzzle3d (rust 62, fixture 62) · puzzle5d (rust 9, fixture 9)
```

The two Rust tests W-T listed as failing (60-vs-2, then 62-vs-2) are therefore satisfied on the
fixture side; **the cargo run itself was not performed in this wave** (no cargo per the wave brief).

## 3️⃣ `verify interactivity tool-jobs` — 7 blockers cleared, 1 peer-owned residual

`bun ./📜️script.ts verify interactivity tool-jobs` aborted on its very first check. Each fix below
un-masked the next failure, so this is a chain, all of it outside puzzle:

| # | failure (before) | verdict | fix |
|---|---|---|---|
| 1 | `📜️script.ts:6080 [shared-action-fixture] recordTutorial arms before the accepted retained route` | **stale audit**, source correct | The wgpu Shell (`…🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) now passes the freshly projected `&live_view_state` (line 4208), so the audit's `&session.view_state` anchor latched onto the unrelated `openSpace` dispatch at line 5259 and found no arming after it. Re-anchored on the arming flag declaration (4089) → the `dispatch_action` acceptance (4208) → the exact arming block (4213), and kept the banned early-arm pattern. Ordering is now checked flag→accept→arm, i.e. stricter than before. |
| 2 | `ENOENT …🏗️fem/🗿️artifacts/◻️2d/…/🪆️subsets/✳️any/✏️editor/🦀️.rs` | **stale fixture** (fem renamed `✳️any` → `🌐️any` on 2026-09-05, `fe7c8a8f8b`) | repointed both occurrences in `🧵️retained-command/🧫️fixtures/🎚️scalar-config-cohort.json` |
| 3 | `bounded reducer proof catalogs contain no exact owner-local rows` + 37 × `Migrated declaration has 0 exact bounded reducer proofs` (fem2d, generation2d, layout) | **stale audit** (import-style brittleness) | `toolJobProofs` required `contract: semio_framework::ToolExecutionContract::…`; the repo is now mixed (20 qualified / 18 unqualified after the peer un-qualify sweep — puzzle**5d**'s proofs were invisible to this gate too). Both proof regexes now accept `(?:[A-Za-z_]\w*::)*ToolExecutionContract::`. |
| 4 | 4 × `forged bounded reducer factory or compiler witness … LayoutExportJobFactory` | **stale audit** (artifact-crate split) | `toolJobFactoryModuleFile` only walked crate roots whose *own* path contains `/📦️packages/`; since the per-artifact crate split the `#[path]` module tree lives in `🗿️artifacts/<artifact>/🦀️.rs` (declared by that artifact's `📦️packages/🦀️rust/Cargo.toml` `[lib] path = "../../🦀️.rs"`), so `crate::editor::layout::engine::export` could not resolve. Added `toolJobArtifactCrateRoot()` and accept that root too. |
| 5 | `scalar Config route bijection fem2d` / `route disposition fem2d/addNode` | **stale fixture** | every `setLocale` route row (fem2d, generation2d, layout) dropped — none of the three editors contains `Locale`/`Terminology` any more; fem2d's 16 routes regenerated from source evidence (all `Migrated`, lanes `["Artifact"]`/`["Config"]`, empty blocker) with `🔨️repair-scalar-config-cohort-fixture.ts` (added to this ticket folder, drives the gate's own exported `toolJobOwnerSourceEvidence`). |
| 6 | `scalar Config strict schema accepted a hostile fixture` (2 of 4 hostiles admitted) | **stale schema** | `🧵️retained-command/🧬️schema/🔣️.json`: `textMaximumBytes`/`grantBytes` were `minimum/maximum` ranges, so the test's `129`/`4097` hostiles validated. Pinned `const 128` / `const 4096` (the values every owner's Rust carries verbatim). The same schema's `ScalarConfigRoute.lanes` enum admitted only `Config|HostOnly`, which cannot describe fem2d's Artifact-lane migration — widened to the full `ArtifactToolPublicationLane` vocabulary (`HostOnly, Artifact, Config, Draft, Presence, Transient, Child, Interaction`); every other clause (lanes ≡ source lanes, Migrated ⇒ non-empty lanes + empty blocker) is unchanged. |
| 7 | `borrowed map schema accepted hostile lifetime shape`, `strict canonical reader schema accepted hostile input`, `canonical error-progress schema admitted forged credit or completion` | **stale schema** (same class) | `🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json`: pinned `longKeyBytes const 4407`, `lifetime.iteratorDropsBeforeRoot const true`, `CanonicalReader.sourceFixture const "canonical-borrowed-map"`, `CanonicalReader.grants const [0,1,7,4096]`, `CanonicalErrorProgress.grants const [0,1,7,256,4096]`, `expectedBytes const 11`, `expectedComplete const false`, `expectedRootRetirements const 1` — exactly the fields the tests' hostile mutations attack. |
| 8 | `canonical error-progress live encoder/reader/sealer linkage is missing` | **stale audit** (rustfmt) | 4 clauses anchored single-line statement pairs that `rustfmt` has since spread over lines (`Err(error) => { self.failed = true; error.written_bytes }`, `self.cancelled = true; encoding_error = Some(error.reason); error.written_bytes`, and the two orderings against `if let Some(error) = encoding_error { return Err(error); }`). `🔬️canonical-error-progress/🟦️.ts` now flattens the four extracted method bodies and its two brittle hostile mutations use a new `looseStatements()` whitespace-tolerant pattern. Semantics of every clause unchanged (same tokens, same order). |
| 9 | `latest-wins production admission/publication authority is incomplete` | **stale audit** (test extracted) | The only failing clause of 25 obligations + 6 extras was `source.includes("retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close")`; that test now lives in `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1844`, not inline in `🔌️plugin/🦀️.rs`. The clause now reads that file and requires the exact `async fn …()` signature. |

**Residual (peer-owned, not fixed):**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:115`
  `retained ChildEmit close and rejected completion handback are incomplete`. Three clauses fail, and
  unlike #9 they are **not** a relocation: `self.emit = Some(rejected.emit)` and the message
  `typed child output lacks a retained nested producer` do not exist anywhere in the framework Rust any
  more (`grep -rn … --include='*.rs' 🧰️framework` → 0 hits), i.e. the rejected-completion handback in
  `🔌️plugin/🦀️.rs` was rewritten. Owner: the plugin-runtime peer (`🔌️plugin/🦀️.rs` is W-S/W-F +
  `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` territory; the file changed 5 times today).
  Whoever owns that rewrite must either restore the named handback expressions or re-express the three
  clauses; deciding that from the outside would weaken the invariant, so it was left alone.
  (`retained close_child_one`, `self.emit = rejected.emit.ok()` and `self.ephemeral = Some(rejected.ephemeral)`
  in `🧵️retained-command/🦀️.rs` are all still present — only the `🔌️plugin/🦀️.rs` half is gone.)

## 4️⃣ Other interactivity gates

### `verify interactivity apps` — ❌️ 782 failures, **0 puzzle-owned**

Identical before and after this wave (`--actions` gives the same 782). Full `--json` census:

| count | failure class | owner |
|---|---|---|
| 759 | `descriptorVersion must be 1` / `manifest is missing` / `role must be plugin` — 253 relocated `🧬️schema/🔣️.json` files being read as plugin descriptors | `SCOPE-OWNED-SCHEMA-CONTRACTS` (schema relocation) |
| 12 | `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`: `expected one exact ⚖️gate…registration, found 0` (interactivity, tool-jobs, apps, apps-actions, dependencies, dependencies0️⃣) | launch-seed owner (seed is uncommitted in `git status`) |
| 3 | fixed-capacity overflows: launch.json 2383 > 512, seed 1306 > 512, 19129 descriptors > 256 | same two owners |
| 8 | `has no owner-qualified React + WGPU Wasm + WGPU native launch variant` (writer, …) | those plugins |

`grep puzzle` over all 782 → **0**.

### `verify interactivity` (full sweep) — ❌️ 1 puzzle-owned residual (was 2, one fixed)

| before | after |
|---|---|
| `Puzzle fill envelope baseline was falsely rejected: Puzzle fill admission does not advance one fixed nested allocation/entry …` | fixed (audit) |
| `Puzzle fill P4e baseline was falsely rejected: P4e spatial owner is not fixed …; P4e preview publication is not the canonical bounded diagnostic page` | second half fixed (audit); first half **remains** |

Fixed here, both audit-side and puzzle-scoped:

- `📜️script.ts:8870` required `fixedOwners.includes("std::mem::size_of::<[Option<(K, V)>; N]>()")`, but
  `…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:164` now writes the unqualified `size_of::<…>()` (peer
  un-qualify sweep; the file was touched 10 min before the run). Clause is now the regex
  `/(?:std::mem::)?size_of::<\[Option<\(K, V\)>; N\]>\(\)/`.
- `INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE` still pointed at
  `…/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, **deleted on 2026-09-02** (`21fbcd3538`); the fill
  preview/diagnostic schema (`pub candidate_page: [Option<String>; 8]`, `registry_generation`,
  `truncated`, `collision_count`) lives in `…/🧬️schema/🦀️.rs:747/765`. Constant repointed — which
  also re-arms the P4e self-test's `unbounded-diagnostic` hostile that had silently become a no-op.

**Residual — puzzle-owned, needs W-F (Rust):**
`📜️script.ts:9005` `P4e spatial owner is not fixed, resumable, generation-bound, and used by the
production broad phase`. Exactly one clause fails, isolated by feeding the real audit function
prefix-injected sources: `geometry.includes("pub(crate) struct CollisionIndexRemoval")`. In
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs`
the resumable removal owner is now test-only — `#[cfg(test)]` at **line 921** (`struct
CollisionIndexRemoval`), **1166** (`fn begin_removal`) and **1172** (`fn step_removal`) — committed
this way at HEAD (looks like a dead-code sweep: nothing in production calls it; `step_replacement`
*is* production, used at `🪣️fill/🦀️.rs:3378` and `:3971`). Two honest resolutions, both requiring a
decision in W-F's file: (a) drop the three `#[cfg(test)]` attributes and use the removal cursor from
the production broad phase (what the invariant says), or (b) consciously narrow the audit's
production-anchor list. Relaxing the audit unilaterally would weaken a P4e invariant, so nothing was
changed.

## 5️⃣ Repo gates

| gate | result | owner of failure |
|---|---|---|
| `bun ./📜️script.ts verify dependencies literal-external` | ❌️ `target=0, current=193, oracle-conflicts=23, toolchain-owner-conflicts=2` (rust 80, js 79, python 34; 105 production-reachable) | repo-wide, pre-existing; puzzle is not among the declaring packages (the single `puzzle` grep hit is a `✒️writer` oracle-conflict line) |
| `bun nx run repo:policy-check` | ❌️ 2 `CACHE-06` violations: `…📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json:131 @semio-tech/framework-renderer-wgpu:wasm` and `…🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:52 @semio-tech/framework-os-dev:build` — "Target still needs an explicit output contract" | framework renderer/dev owners |
| `bun nx run repo:graph-check` | ✅️ exit 0 — `projects=698 edges=6188` | — |
| `bun nx run repo:artifact-check` | ❌️ the same 2 `CACHE-06` violations (same `PolicyScript`) | same |
| `bun ./📜️script.ts verify taxonomy` | ❌️ needs a mode: `expected report or enforce, got undefined` | — |
| `bun ./📜️script.ts verify taxonomy enforce` (and `report`) | ❌️ `Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/🔎️recherche` | that top-level directory's owner (last touched by peer commit `de617a7c17`); 0 puzzle mentions |

Nx did not time out; every gate above finished well inside 10 minutes (graph-check 32 s,
policy-check 18 s), so the direct-`📜️script.ts` fallback was not needed.

## 📁️ Files changed

| path | change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` | `variant` wired into `exactContracts` + `expectedPairs`; new `localeIsOsOwned` + `puzzle3dHostileSources`; Puzzle3d locale anchors → Config-envelope anchors; Puzzle2d dispatch-emit anchors → borrowed spellings; hostiles now assert they apply |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🗄️retained-jobs/🔣️.json` | `toolIds` 2 → 62 (Rust order) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/🗄️retained-jobs/🔣️.json` | `toolIds` 2 → 9 (Rust order) |
| `📜️script.ts` (root) | recordTutorial ordering re-anchored; both `ToolExecutionContract` proof regexes qualifier-tolerant; `toolJobArtifactCrateRoot()` + artifact-crate roots in `toolJobFactoryModuleFile`; `size_of` clause regex; `INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE` → `🧬️schema/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧫️fixtures/🎚️scalar-config-cohort.json` | fem path `✳️any`→`🌐️any`; 3 stale `setLocale` routes removed; fem2d's 16 routes regenerated from source (Migrated + real lanes) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json` | `textMaximumBytes`/`grantBytes` pinned; `ScalarConfigRoute.lanes` widened to the full lane vocabulary |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json` | 8 missing hostile-refusing pins added |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` | `looseStatements()` + flattened method bodies (rustfmt tolerance) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` | named latest-wins test read from its extracted `🧪️tests` file |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️diagnose-publication-authority.ts` | **added** — clause-by-clause mirror of the puzzle audit |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️trace-publication-authority.ts` | **added** — runs the real audit with `production` proxied and `return false` numbered |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️verify-retained-jobs-fixtures.ts` | **added** — cargo-free replication of the Rust retained-jobs fixture oracle (exit 0) |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔨️repair-scalar-config-cohort-fixture.ts` | **added** — regenerates the cohort fixture's route rows from source evidence |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/📓️2026-09-09-wave-G-gates.md` | **added** — this report |

## ⚠️ Not verified / deliberately not done

- **No cargo run.** The two Rust tests that consume the retained-jobs fixtures were verified only
  through the TypeScript replication above; they were not compiled or executed in this wave.
- **`mutations` laws in `🎚️scalar-config-cohort.json` still carry `fem2d/SetLocale` and
  `layout/SetLocale`.** They are pure Immer/JSON algebra laws (never cross-checked against source), and
  the schema pins `mutations.minItems: 11` with exactly 11 present, so removing the two stale ones
  needs that floor lowered — a call for the framework schema owner, not this wave.
- **`.vscode/🧩️launch.seed.jsonc` / `launch.json`** were not touched (the 12 missing `⚖️gate…`
  registrations and the 512/256 capacity overflows are the launch-seed owner's).
- No dev boot, no runtime/console confirmation of any puzzle3d behaviour.
