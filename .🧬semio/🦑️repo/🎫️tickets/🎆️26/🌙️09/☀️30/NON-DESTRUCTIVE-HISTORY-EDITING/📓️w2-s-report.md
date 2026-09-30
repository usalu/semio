# 📓️ W2-S — Schema ↔ payload parity (lint, repo-wide aggregate rule, D-scope fixes)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-S, 2026-09-30. Contract: `📋️design.md` §6 (history editing edits
`payload_value()` at the input pointer the leaf schema declares, validates against the leaf schema, `with_payload_value()`).
Scope after the coordinator split: **D** = the lint and its corpus test, the repo-wide aggregate/tag-const rule, and the fixes for
energy, architect, remodel, fem, gis, lowpoly, trinity, animate, playbook, vcs, writer, space, sourcing, demonstrator, wfc,
block, reasoning and the framework os aggregates. A (norm), B1/B2 (stdio), C (design) and E (layout + tail invalids) are other
executors.

## 1. Outcome

- **Lint `schema-mutation-payload-parity` — DONE and VERIFIED.** It runs through `bun`, through nx, and through the
  language-agnostic corpus test with a third-party oracle.
- **Repo-wide aggregate rule — DONE** for every aggregate outside A/B/C. This includes E's plugins and every framework os
  aggregate. The rule was approved by the coordinator.
- **D scope — 0 findings.** The one exception is 8 wfc findings, all caused by the stale catalog: `os/store/child/schema.json`
  has no catalog row yet. With the full document set those 4 fixtures come out clean (§6). The findings clear with the final
  `schema generate`, which is gated on the coordinator.
- **Repo-wide census.**

  | | clean fixtures | findings |
  |---|---|---|
  | baseline | 650/2771 | 2516 |
  | now | 2657/2757 | 206 |

  All 206 remaining findings are in A, B and C, which are parked or in progress (§4).

## 2. The lint

**Code.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`, region `⚖️MutationPayloadParity`.
It sits next to `🎛️MutationInputUi`. Both lints now share `📚️SchemaDocuments` (`catalogSchemaDocuments`, `readJsonObject`).

**Registration.** The diagnostic code is registered in `SCHEMA_DIAGNOSTIC_CODE_TABLE` (`🧪️test/🟦️.ts`) and in the protocol enum
(`🧪️test/🧬️schema/🔣️.json`). The code-table invariants suite passes, 5/5.

**Commands**, run in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

| Command | nx target (`@semio-tech/repo-test-domain`) |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads [--census] [--under <path>] [--json]` | `test-schema-mutation-payloads`; with `--census`, `test-schema-mutation-payloads-census` |
| `bun ./📜️script.ts test mutation-payload-parity` (the corpus test) | `test-mutation-payload-parity` |

Strict mode exits 1 on any finding. `--census` always exits 0.

**What it does.**

1. **Walks** `✏️s` and `🧰️framework` and collects:
   - every leaf descriptor, including nested leaves such as architect `🧬️mutations/<entity>/<verb>` and the test aggregates
     under `🧫️fixtures`;
   - every `#[derive(Mutations)]` enum in a `🧬️mutations/🦀️.rs`;
   - every committed `…fixtures/…/🦠️mutation/🔣️.json` or `🧪️tests/…/🦠️mutation/🔣️.json`.
2. **Reads the aggregate wire layout from Rust.** `rustMutationAggregates` is a small lexer that drops comments and keeps
   literals whole. It reads the enum's `#[value(tag, content, rename_all)]` and each variant's `#[value(rename)]`.
   `mutationVariantWireName` is the TS twin of value_derive `variant_wire_name`.
3. **Maps each fixture to its leaf.** It first tries the leaf directory the fixture path names; emoji are ignored and windows
   of path segments are tried. It then falls back to the variant the payload names in a nearby aggregate. Finally it picks
   the aggregate that declares the leaf's `aggregateVariant` and shares the longest path prefix with it.
4. **Cuts out the leaf payload exactly as `payload_value()` does** (`mutationLeafPayload`):
   - external: the one-key wrapper;
   - adjacent: the `content` member;
   - internal: the members beside the tag.

   The stdio exhaustive case records `{kind, mutation, before, after}` are unwrapped (`mutationFixtureWire`). The root `const`s
   are spliced back with the manifest reader `mutationInputInstance`, the TS twin of `mutation_input_instance`.
5. **Validates** with npm `jsonschema`. The `$ref` universe is every `$id` document under the catalogued scope directories, the
   same universe the input-UI lint uses.

**Finding classes** (`MutationPayloadFindingClass`):

| Class | Meaning |
|---|---|
| `invalid` | The payload fails its leaf schema. |
| `undescribed` | A payload member that no `properties`, `patternProperties` or `additionalProperties` describes. The check walks through `$ref` and `allOf`, and through the `oneOf`/`anyOf` branch that matches. There is one finding per member. |
| `opaque` | Static check over every leaf: an object node without members, reached through every reachable subschema, including foreign documents. |
| `layout` | The fixture is not that variant in the aggregate's wire layout. |
| `aggregate` | The aggregate's own `🧬️mutations/🔣️.json` rejects its fixture's wire, or does not resolve. |
| `unmapped` | The fixture belongs to no leaf. |
| `unresolved` | A `$ref` or the leaf document is missing. |

The checker is exported as `mutationPayloadChecker(documents)` with `payload`, `aggregate`, `opaque` and `fixture`.

**Known limit.** The fixture is the witness. A drift that no committed fixture exercises cannot be seen: for example, an
`Option` field emitted as `null` whose fixtures always carry a value. I fixed the one I found by reading the Rust code
(architect presence `adjacencyKindFilter`, §5).

## 3. Language-agnostic corpus and third-party oracle

- **Corpus.** `🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json` has 11 cases, each with Rust source, variant, leaf schema,
  aggregate schema, fixture, the expected layout, wire name and payload, and the expected `class@pointer` findings. The cases
  cover:
  - the internal, external and adjacent layouts, including a variant `#[value(rename)]`;
  - a Rust comment that must not be read as an attribute;
  - snake-case drift;
  - a wrong layout;
  - an object without members;
  - an open object;
  - a null Option, before and after the fix;
  - leaves that lack the tag const;
  - the stdio case records.
- **Test.** `🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts` has 22 tests. A third-party oracle runs alongside the lint's own npm
  `jsonschema` validation:
  - Ajv (draft-07) must agree with every `invalid` verdict;
  - Ajv 2019 with `unevaluatedProperties: false` injected must report exactly the lint's `undescribed` pointers.
- **Negative check.** One expectation was deliberately falsified. Both the lint test and the oracle test failed, so neither
  suite passes vacuously.

## 4. Census

Baseline (`🗑️generated/w2s/census-before.tsv`, taken once the lint had the aggregate class): 650/2771 fixtures clean,
2516 findings. Without the aggregate class the baseline was 2350/2771 clean and 582 findings. Current state is in
`🗑️generated/w2s/census-after.tsv` and `after.json`: 2657/2757 fixtures clean, 206 findings.

| owner | before (aggregate / other) | now | owner of the rest |
|---|---|---|---|
| energy | 578 / 0 | **0** | — |
| architect | 266 / 0 | **0** | — |
| remodel | 136 / 0 | **0** | — |
| fem | 118 / 0 | **0** | — |
| block | 104 / 2 | **0** | — |
| gis | 22 / 0 | **0** | — |
| lowpoly | 17 / 0 | **0** | — |
| trinity | 19 / 2 | **0** | — |
| animate, playbook, vcs, writer, space, sourcing, demonstrator, reasoning | 50 / 0 | **0** | — |
| wfc | 39 / 7 | 8 (4 `unresolved`, 4 `aggregate`: all the stale-catalog `os/store/child/schema.json`) | final `schema generate` |
| math, process, imperative, sequence, flow, dag, layout, os | 72 aggregate / 84 other | **0** | aggregates by D, other items by E |
| norm | 233 / 369 | 64 | A (norm executor, running) |
| stdio | 49 / 70 | 11 | B2 (parked) |
| shooting, puzzle, procedural, cad, raster, note, forms | 173 / 70 | 123 | C (parked) |

## 5. What was changed

### 5.1 Repo-wide aggregate rule (coordinator-approved)

The leaf root is the `payload_value` shape exactly as Rust emits it. Beyond that, each layout gets:

| Layout | Leaf schema | Aggregate branch |
|---|---|---|
| Internally tagged | The root also declares `<tag>: {const: <wire name>}` first in `properties` and first in `required`, and in `propertyNames` when the leaf has it. There is no Payload/Wire `$defs` split. | `oneOf` of the leaf `$ref`s, in Rust variant order |
| Externally tagged | Payload only | `{type: object, additionalProperties: false, required: [W], properties: {W: {$ref leaf}}}` |
| Adjacently tagged | Payload only | `{required: [tag, content], properties: {tag: {const}, content: {$ref leaf}}}` |

`Option` fields that value_derive emits as `null` get `[T, "null"]`, or a `oneOf` with `null` when the field is an enum.

**Tools**, kept in the ticket root:

- `🧪️w2-s-aggregate-map.ts` reads the Rust layouts with the lint's own lexer and writes `aggregate-map.json`.
- `🧪️w2-s-aggregate-rule.py` applies the rule.
  - It is idempotent, and it re-reads each file right before writing it.
  - Canonical files are re-rendered. Files in any other format get a byte-preserving text insertion, verified semantically.
  - Payload/Wire splits are folded into the root.
  - Anything it cannot handle is listed for a hand edit.

**Applied** to about 940 leaf and aggregate schemas:

- energy, architect, remodel, fem, gis, lowpoly, trinity, animate, playbook, vcs, writer, space, sourcing, demonstrator, wfc,
  block and reasoning;
- dag, flow, sequence, mathematical, process and imperative (E was told);
- framework os: flow-vcs, dag-vcs, store, workflow, workflow-run and os config.

Every `x-semio-ui` was kept.

**Payload/Wire flattening.** 21 leaves had their Payload/Wire split folded into the root: vcs (6), sequence (8), the gis map
window config (6) and the gisterrain camera (1). The framework flow and dag VCS leaves were flattened too.

**`propertyNames` fix.** 29 framework VCS and run leaves carried `propertyNames.enum`, which would have excluded the new tag.
The tag was added to the enum.

### 5.2 D-scope source fixes

**trinity**

- **jack `ReplaceQueryResult`.** The Rust struct was the outlier, because it had no `rename_all`. It gained
  `#[value(rename_all = "camelCase")]`, and the fixture now carries `executionId`. The TS twin was already camelCase.
- **The leaf schema** gained an `$id` in the catalogue scheme and the `kind` const. `result` now references a new
  `$defs/QueryResult` in `s/trinity/jack/artifact.json`:
  - `kind` table|graph, with option labels;
  - `columns`;
  - `rows` as `DslValue` rows (hidden);
  - `graphFixture` as a jack snapshot (hidden).
- **The jack results-window transient snapshot** now references the same definition. The aggregate gained an `$id` and a `oneOf`.
- **Window leaf ids.** The 4 window leaves with `https://semio.dev/...` ids (rewriting window and jack graph window:
  `set-camera`, `set-lod-mode`) now use `app/trinity/<artifact>/<window>/config/mutation/<kind>/schema.json`. Their dialect is
  draft-07, and their aggregates gained an `$id`.

**wfc**

- **3d `PinSlot`.** The Rust struct gained `#[value(rename_all = "camelCase")]`. The fixture, the TS twin `🟦️.ts` and the Python
  second implementation now use `tileId`.
- **grid3d.** The snapshot gained `$defs/Grid3dTileMedia`, and its `meshChild.child` is now the store child handle. The
  `change-tile-media` and `create-tile` leaves reference the new definition, which carries labels for the mesh, colour and
  child fields.
- **grid2d.** `media.child` now references the store child schema (hidden) in both the leaves and the snapshot.
- **Leaf ids.** 5 leaf `$id`s moved to the catalogue scheme: the wfc2d config `change-active-tile`, `change-camera` and
  `replace-config`, and the wfc2d/wfc3d transient `set-solve`.

**block**

- The `update-presentation` (2d) and `update-part2d` (5d) Option fields are now `[T, "null"]` and required. The Rust code emits
  `null`.

**architect**

- **`DeliveryPhase`.** `constructionArtifacts` → `constructionDocuments` in the snapshot schema, the combined schema and the TS
  twin. The Rust variant is `ConstructionDocuments`.
- **Presence `adjacencyKindFilter`.** It is now nullable and required, because the Rust `Option` is emitted as `null`.

**space**

- The home `apply-directory-page` `$id` was renamed to the kebab kind.

**os config**

- **`change-merge-policy`.** `policy` described the db index `MergePolicy` (`maxRunsBeforeMerge`). It is now the replication
  string enum `LaissezFaire|Normal|Vigilant`, with a segmented widget, en/de option labels and a description that states the
  real severity floors.

**remodel formats** (strict Ajv oracle)

- The formats `uint32`, `uint64`, `int32`, `int64`, `float`, `double` and `base64` stay, because the repo's facet-parity scalar
  mapping reads them (`field-discovery`).
- Both format lists are pinned in the shared vector corpus `🧬️schema/🧫️fixtures/✅️draft07-validation-vectors.json`:
  `assertedFormats` (the 7 asserted formats) and `annotationFormats` (those 7 spellings). The annotation vector case covers
  all of them.
- The strict oracle `semioSchemaAjvV1` registers:
  - the asserted formats through `ajv-formats`;
  - the annotation formats as always-valid, which is the owned validator's semantics.

  An unknown format still fails in strict mode.
- A new Rust law, `owned_format_policy_matches_the_shared_format_lists`, pins `ASSERTED_STRING_FORMATS` and the annotation
  semantics against the corpus.
- Result: 304/304 architect+remodel leaves compile in the strict oracle, where 30 of the 36 remodel leaves failed before. All 654
  leaves of the other D plugins compile.

## 6. Verification (all run, foreground, gated)

| Check | Result |
|---|---|
| `bun ./📜️script.ts test mutation-payload-parity` and nx `test-mutation-payload-parity` (`NX_DAEMON=false`; the daemon's graph was stale) | 22 pass |
| nx `test-schema-mutation-payloads-census` | runs; exit 0 |
| `schema mutation-payloads --under <p>` for energy, architect, block, trinity, os config and `🧰️framework` | 0 findings each |
| wfc media fixtures through the exported checker, with store and io documents added (what the catalog refresh provides) | 4/4 clean |
| `schema mutation-inputs` (W1-D lint) over the D scope | 0 findings, except wfc 2 `refUnresolved` (the same stale catalog) |
| `bun test` W1-D `🧪️mutation-inputs/🟦️.ts` (uses the strict oracle) | 78 pass |
| code-table invariants (`🧬️schema-invariants -t code`) | 5 pass |
| `bunx tsc` over the orchestration file, the corpus test, `📜️script.ts` and the oracle (files confirmed in `--listFilesOnly`) | 0 errors |
| `verify taxonomy report` on the two new directories | clean |
| `🧪️w2-s-strict-compile.ts` (strict oracle compile) | architect+remodel 304/304; the 15 other D plugins 654/654 |
| `cargo check -p semio-s-artifact-wfc-3d` | ok (2 pre-existing warnings) |
| `cargo test -p semio-s-artifact-wfc-3d --lib` | 186 pass; the pin-slot fixture laws, including `committed_json_is_canonical`, pass |
| wfc3d Python second implementation (`🐍️.py --fixtures`) | 15 vectors, 0 problems |
| `cargo check -p semio-s-artifact-trinity-jack` | ok |
| `cargo test -p semio-s-artifact-trinity-jack --lib` | 174 pass |
| `cargo test -p semio-framework-schema --lib owned_` | 7 pass, including the new format-policy law |
| schema vitest (`bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts`) | the draft07 oracle passes in full. 1 failure in `📤️schema-export-entries`, which expects a `semio.tech` id; its files are untouched in the working tree, so the failure is not mine. The other 2 files fail to load (no suite; `bun:test` under vitest), which is also pre-existing. |
| **WRITTEN BUT UNVERIFIED** | The repo-test case `mutate-trinity-jack-1-any-editor-edit-results-transient` could not run. The runner's contract phase aborts on 1406 pre-existing layout and contract breaches repo-wide. The fixture and the Rust are consistent (the lib compiles; the TS twin was already camelCase). |

## 7. Open items and hand-offs

1. **Catalog refresh.** The final `schema generate` is gated on the coordinator. After it:
   - `os/store/child/schema.json` and `os/store/link` are catalogued, which clears the wfc 8 and part of the norm and stdio
     `unresolved` findings;
   - the new `$id`s from §5.2 are catalogued, which clears W1-D's `leafUncatalogued`;
   - the 3,500+ stale hashes are refreshed.
2. **launch.json.** The new nx targets need to appear in the generated `.vscode/launch.json`, via W3-G's regeneration.
3. **Gate wiring.** Wire `schema mutation-payloads` (strict) into `test schema`/CI once A, B and C reach 0 findings.
4. **Not in my power, or left by decision.**
   - **Missing aggregate schemas.** About 15 editor-lane aggregates have no `🧬️mutations/🔣️.json`: energy, architect, animate, wfc
     and playbook config/presence/transient lanes; the os config, store and workflow aggregates. The lint does not flag an
     absent aggregate schema.
   - **grid3d aggregate `$id`** is `…/mutations/schema.json`, while every other aggregate uses `…/mutations.json`. This belongs
     to the `schema check` aggregate-id rule.
   - **bitmap transient `set-solve` `$id`** has no `/mutation/` segment.
   - **`MergePolicy` wire strings are PascalCase.** They come from a hand-written ToValue, and changing them is a framework wire
     change.
   - **Built wasm.** The committed `dist` wasm still contains `constructionArtifacts`; the next build fixes it.
   - **Plugin descriptors and roster inputs** need the central `describe` regeneration. This is W1-D open item 1.
5. **For A, B and C.** Rerun `🧪️w2-s-aggregate-map.ts`, then `🧪️w2-s-aggregate-rule.py --apply <plugin prefix>`, to get the
   tag consts and aggregate wrappers in one pass. Then fix the remaining `invalid`, `opaque` and `unresolved` findings per the
   lint.

## 8. Files

**Created.**

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts`
- Ticket inputs: `🧪️w2-s-aggregate-map.ts`, `🧪️w2-s-aggregate-rule.py`, `🧪️w2-s-strict-compile.ts`

**Modified: lint and oracle.**

- `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
- `🧪️test/🟦️.ts`
- `🧪️test/🧬️schema/🔣️.json`
- `🧪️test/📜️script.ts`
- `🧪️test/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts`
- `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/✅️draft07-validation-vectors.json`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`

**Modified: Rust.**

- trinity jack `…/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs`
- wfc 3d `…/🧬️schema/🧬️mutations/📌️pin-slot/🦀️.rs`

**Modified: TS twins, oracles and fixtures.**

- wfc 3d `🧬️mutations/🟦️.ts`, `🧪️tests/🧩️mutate-wfc3d-1/🐍️.py` and the `📌️pin-slot` fixture
- architect `🧬️schema/🟦️.ts`
- the jack `replace-query` fixture

**Modified: schemas.**

- The leaf and aggregate schemas the rule touched: about 940 under the D-scope plugins and framework os, plus E's six plugins.
- trinity:
  - jack `artifact.json`;
  - jack results transient snapshot, leaf and aggregate;
  - trinity window leaves and aggregates.
- wfc:
  - grid3d snapshot plus 2 leaves;
  - grid2d snapshot plus 2 leaves;
  - 5 wfc2d/3d config/transient leaves.
- block: 2 leaves.
- architect:
  - `🧬️schema/🔣️.json` and `📸️snapshot/🔣️.json`;
  - presence leaf.
- space: home transient leaf.
- os config: `change-merge-policy` leaf.

**Scratch** in `🗑️generated/w2s/`:

- census files: `census-before*.tsv`, `census-after.tsv`, `before-v*.json`, `after.json`;
- `aggregate-map.json`, `aggregate-rootcauses.txt`, `rule-*.txt`, the surveys, `tsconfig.lint.json`, `tsc.txt`.

## Follow-up (coordinator, 2026-09-30): every `#[derive(Mutations)]` enum has a schema-first contract; both lints gate `test schema`

### F1. Lint: static aggregate contract

`mutationPayloadParityReport` now runs a static pass over every aggregate enum, test fixture aggregates included, reporting
class `aggregate` on the Rust file. For each enum the pass checks:

1. The aggregate document `🧬️mutations/🔣️.json` exists and declares an `$id`. A file that holds several enums (os config holds
   5) carries one `$defs/<Enum>` per enum, and the lint addresses that enum at `<id>#/$defs/<Enum>`.
2. Every variant has exactly one nearest mutation leaf, and that leaf declares a unique `$id`.
3. An internally tagged enum's leaves declare the tag `const` equal to the variant's wire name.
4. The `oneOf` equals the branches the rule derives, in Rust variant order. Key order inside a branch does not matter.

**New export.** The rule's branch shape is exported as `mutationAggregateBranch(layout, wireName, leafId)`. It is the TS twin
of the Python rule script.

**Leaf discovery fix.** A `🧬️mutations` directory that holds an aggregate `🦀️.rs` directly under `🧫️fixtures` is now a leaf root.
Before, the store presence-retirement fixture aggregate saw 3 ambiguous `SetValue` leaves.

**Corpus.** Every case that has an aggregate schema now asserts that its branch is `mutationAggregateBranch(...)`. The test
passes: 22 tests, 84 expect() calls.

**Before the fixes**, the static pass reported:

| Finding | Count |
|---|---|
| Aggregate schema absent | 48 (38 production, 10 test fixtures) |
| Leaf missing the tag const | 20 (test fixtures) |
| `oneOf` differs from the derived union | 9 (glTF and puzzle 2d: order only; 7 test fixtures: direct-object form, a Payload/Wire `allOf`, or order) |
| Ambiguous leaves | 1 |

### F2. Aggregate schemas written

`🧪️w2-s-aggregate-rule.py` now does three more things:

- **Creates missing aggregate documents.** The `$id` sits beside the leaves: `<base>/mutation/<kind>/schema.json` becomes
  `<base>/mutations.json`. When the leaves share no base, the `$id` falls back to the lane module's directory. A file shared by
  several enums gets a `$defs` entry per enum plus a root `oneOf` of those.
- **Rewrites non-`oneOf` aggregate documents into the `oneOf` form.** `$schema`, `$id`, `title` and `description` are kept.
- **Adds the tag to a leaf's `propertyNames` enum** when the leaf has one.

`🧪️w2-s-aggregate-map.ts` now includes the leaves of fixture aggregates.

The script was applied repo-wide (`✏️s`, `🧰️framework`) and wrote **77 files**.

| What | Where |
|---|---|
| 38 new production aggregate documents | trinity jack editor transient; block3d world transient; dag config/presence; reasoning canvas transient; animate config/presence; space home transient; procedural generation3d viewer config/presence/transient and editor config/transient; generation2d transient; wfc 3d config/transient, bitmap transient, 2d config/transient; imperative config; note presence; forms config; architect config/presence; shooting config/presence; fem 3d results transient; playbook config and the procedural extension; energy config; store `SpaceHistoryMutation`; workflow; os config (5 enums in one document) |
| 10 new test-fixture aggregate documents | plugin surface/transaction/dummy/test-app config and document; spr command registry; store lossy/validated/severity/timestamped/demo/presence retirement |
| Tag consts added | 20 test-fixture leaves |
| Aggregates rewritten to the rule form | 9: glTF and puzzle 2d reordered; the plugin job-test, dependency-contribution, contributed-mutation-wire, publication presence/transient and declaration-channels fixtures |
| Payload/Wire leaves folded | the dependency-contribution and contributed-mutation-wire `add-value` leaves |

Every variant already had a leaf schema with a `$id`, so no new leaf schema was needed. W1-D's lint keeps the `x-semio-ui`
coverage of those leaves; the tag consts are skipped as discriminators.

One more `$id` was fixed: wfc bitmap transient `set-solve` → `…/transient/mutation/set-solve/schema.json`.

The script is idempotent. A second run reports: would write 0, 2089 already conform, 0 manual.

**Strict Ajv oracle** (`🧪️w2-s-strict-compile.ts --aggregates`): 200 of 205 aggregate documents compile with their leaves loaded.
All the new documents compile. The 5 that do not are all leaf-level problems in other WPs:

| Aggregate | Cause | Owner |
|---|---|---|
| sequence step, imperative | Leaf `$defs/NeuralValue` uses a multi-type `type: [null, boolean, number, string]`, which strict Ajv refuses without `allowUnionTypes`. Use an `anyOf` of single types. | E |
| raster | A leaf declares the draft 2020-12 `$schema`. | C |
| pdf, dwg | Unresolved leaf refs. | B2 |

### F3. `test schema` gate

- **Wiring.** `SchemaScript.run` (`test schema`) now appends the diagnostics of `mutationInputUiReport` and
  `mutationPayloadParityReport` to the invariant diagnostics, for the same `--under` scope. A finding from either lint now fails
  the gate. Verified on the wfc subtree: `schema-mutation-payload-parity` and `schema-mutation-input-ui` rows appeared, and they
  vanished after the peer catalog refresh.
- **nx cache.** The nx `test-schema` target now declares its inputs. Before, it had none, so its cache only saw the test
  module. The inputs are now:
  - `default`;
  - `schemaSources`, which covers leaf schemas, aggregates, descriptors and the Rust aggregate files under `🧬️schema`;
  - `{workspaceRoot}/**/*fixtures/**/🦠️mutation/🔣️.json`;
  - the manifest reader `🛂️manifest/🟦️.ts` and the glossary;
  - `📚️library/🔣️schema-catalog.json`.

  Verified by running the target through nx (`NX_DAEMON=false`).
- **What I did not run.** I did not run `schema generate`. Someone else refreshed the catalog at 06:57. That refresh catalogued
  `os/store/child/schema.json`, and the wfc stale-catalog findings cleared with it.

### F4. Census after the follow-up

| Lint | Clean | Findings | Where the findings are |
|---|---|---|---|
| `schema mutation-payloads` | 2753/2757 fixtures | 8 | all norm (A, running): 4 `invalid`, 4 `aggregate` |
| `schema mutation-inputs` | 5201/5360 inputs | 1258 | norm 1257 (A), stdio 1 (B2) |

Every aggregate enum now has a conforming aggregate schema: the static pass reports 0 findings. Files:
`🗑️generated/w2s/census-followup.tsv`, `followup-before.json` and `rule-apply-2.txt`.

### F5. Verification

| Check | Result |
|---|---|
| `bun test` parity corpus | 22 pass |
| code-table invariants | 5 pass |
| `bunx tsc` over the lint, test, script and oracle | 0 errors |
| `schema mutation-payloads --under 🧰️framework` | 0 findings |
| `test schema --under ✏️s/🔌️plugins/🀄️wfc`, directly and through nx | runs, and includes the new codes |

No Rust changed in this follow-up. The aggregate documents are embedded as text by the artifact schema registries
(`json_schema: include_str!`) and are stored uncompiled, so the next plugin builds only pick up new content.

**Files added in the follow-up:**

- the 48 new `🧬️mutations/🔣️.json` documents listed in F2;
- the 29 rewritten aggregate and leaf schemas;
- `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`;
- `🧪️test/📋️project.json`;
- `🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts`;
- `🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json`;
- ticket inputs `🧪️w2-s-aggregate-rule.py`, `🧪️w2-s-aggregate-map.ts` and `🧪️w2-s-strict-compile.ts`.

## Follow-up 2 (audit `📓️audit-inputs-leaves.md` §2.1(a)): no evidence is never reported as clean

### F6. Where the stdio json, svg and xml "0/0" came from

These artifacts have no wire-form fixture at all.

- **svg.** The fixtures `🧫️fixtures/<kind>-applied/{⬅️before,➡️after}.svg` are documents only. The mutations live in the `🥒️.feature`
  Scenario Outline tables as `{"kind": "<id>", "params": <params>}`. That form appears in 190 features repo-wide.
- **Why `params` is not a witness.** The adapters map `params` by hand into typed ops, not through `FromValue`. For example, svg
  `set-doctype` passes the inner doctype string, not the `XmlDoctype` wire. So `params` is not the Rust wire, and treating it as a
  witness would re-create the false green in a different form.
- **Result.** The lint therefore does not read `params`. Instead it now reports the missing evidence (F7). svg now shows
  `0/29 leaves witnessed; 32 findings` instead of `0/0 … 0 findings`.

### F7. Lint changes

1. **New class `unwitnessed`.**
   - **Leaf:** fires for every leaf that no committed wire fixture witnesses. A witness is a fixture that maps to the leaf, is
     that variant in the aggregate layout, and, for a wrapped leaf, is its editable variant.
   - **Aggregate:** fires on the Rust file for every `#[derive(Mutations)]` enum none of whose variants is witnessed.
   - **Census:** a new `witnessed` column. The summary prints `W/L leaves witnessed`.
   - **Gate:** strict mode and `test schema` fail on these findings.
2. **Wrapped leaves**, coordinated with W1-D (their reply: an explicit marker, not the variant name):
   - `rustValueEnums` reads every enum's `#[value(...)]`, its tuple payload types and `#[mutation_leaf(payload = <Variant>)]`.
     `rustMutationAggregates` now returns the ones that derive `Mutations`.
   - The walker collects the marked enums in each leaf's `🦀️.rs` and `🦠️mutation/🦀️.rs`.
   - `mutationInputPayload` mirrors W1-D's `payload_value()`: it cuts the aggregate wrapper, then takes the content of the
     `payload` variant.
   - Any other variant (`Restore`) is inert. It is not editable, so its leaf-schema check is skipped (the aggregate check still
     runs) and it is not a witness.
   - `checker.fixture(…, wrapper)` takes the wrapper. The new type is `MutationLeafWrapper`.
3. **Corpus.** Two new cases carry a `leafRust` with the marker: an `apply` witness validated against the flat payload, and an
   inert `restore`. The test passes: 25 tests, 109 expect() calls, including an Ajv oracle case for the apply witness. tsc reports 0
   errors.

### F8. Census with evidence (`🗑️generated/w2s/census-witness.tsv`, `witness.json`)

**Repo-wide:** 1972/3025 leaves witnessed, 2633/2757 fixtures clean, 1251 findings. The findings are 1127 `unwitnessed`, 124
`invalid` and 4 `aggregate`.

**The `invalid` findings:**

- **glTF (120)** is the real drift W1-D's marker now exposes. The 120 glTF leaf schemas are still the `oneOf(phase)` form, and they
  must go back to the flat Apply payload. That is W2-S-B1's job.
- **norm (4)** is A's.

**`unwitnessed` per owner:**

| Owner | Leaves | Aggregates | Routed to |
|---|---|---|---|
| stdio | 591 | 35 | B2, and B1 for glTF (1 aggregate) |
| norm | 336 | 5 | A |
| layout | 17 | — | E |
| cad | 6 | — | C |
| raster | 2 | — | C |
| framework os: workflow | 23 | 2 | D (mine) |
| framework os: dag-vcs | 14 | 1 | D |
| framework os: flow-vcs | 10 | 1 | D |
| framework os: store | 6 | 1 | D |
| framework os: plugin | 1 | — | D |
| framework os: test-fixture aggregates | 43 | 22 | D |
| trinity rewriting window config | 2 | 1 | D |
| gisterrain camera | 1 | 1 | D |
| playbook procedural extension | 1 | 1 | D |

The D-scope evidence gap has two possible fixes, both outside this lint change:

- committed wire fixtures per leaf;
- W1-D's fixture-independent derive law (`with_payload_value(payload_value()) == op` plus schema validation over
  `demo_mutation_cases`). This is the natural witness for the framework os and test-fixture aggregates. Adding it as a lint input
  needs a Rust-side report the lint can read.

**Files:** `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`, `🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts`,
`🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json`.

## Follow-up 3 (coordinator decision): committed wire witnesses — features as wire, conversion recipe

### F9. Lint: wire-form feature rows are witnesses

- **What counts as a row.**
  - `mutationFeatureRows(source)` reads a `🥒️.feature` through the harness's own `parseFeature`. That is the same plan every
    native host receives, so outline expansion and cell handling are identical to what the adapters see.
  - Every expanded step docstring that is a JSON object `{"kind": <leaf kind>, "params": <payload>, …}` is a row. The Scenario
    Outline template `{"kind": "<id>", "params": <params>}` is the common form: 190 features use it.
  - Any other template (for example `"vector"`) and data tables are not rows.
- **Mapping and validation.**
  - A row maps to the leaf whose descriptor `semanticKind` equals `kind` and that lives under the same `🏅️standards/<std>` as the
    feature's owner.
  - `params` gets the root consts spliced with `mutationInputInstance` and is validated against the leaf schema with the same
    checks as a fixture: `invalid` and `undescribed`.
  - Every mapped row is a witness. A row that is not valid still counts as a witness, but it is reported.
  - A row whose kind has no leaf is reported `unmapped`. This covers the `no-mutation` sentinel (102 rows), `identity-round-trip`
    and similar.
- **`unwitnessed`** stays strict.
- **Census.** A new `rows` column. The summary prints `clean/fixtures+rows … W/L leaves witnessed`.
- **Corpus.** A `features` case pins the reader: outline rows are read; a data table and the `vector` template are not. The test
  passes, 26 tests.
- **Current state:** stdio 1506 rows, of which 843 `invalid` + 90 `undescribed` (the hand-mapped params) and 106 `unmapped`. The
  lint is loud exactly where the conversion below is needed. Non-stdio features using the template:

  | Owner | Rows | Findings | Where |
  |---|---|---|---|
  | animate | 19 | 18 `invalid` | hand-mapped rows |
  | dag | 28 | 2 `invalid` | |
  | flow | 21 | 1 | `no-mutation` |
  | sequence | 33 | 1 | `no-mutation` |
  | shooting | 63 | 1 | `no-mutation` |
  | imperative | 8 | clean | |
  | reasoning | 20 | clean | |

### F10. Conversion recipe (every executor follows it exactly)

**Rule.** A feature row's `params` IS the leaf wire payload: the leaf's `payload_value()`, byte-equal to what
`pack::to_json_string(&payload)` emits for the leaf payload struct. It is written:

- camelCase, as the Rust `#[value(rename_all)]` emits it;
- with `Option` fields as the Rust code emits them: `null`, or absent when the field has `skip_serializing_if`;
- with every nested record in its Rust wire form, never a shorthand.

For a `#[mutation_leaf(payload = Apply)]` leaf (the 140 stdio phase/value leaves) it is the flat `Apply` content, with no
`phase`/`value`. The aggregate tag is not part of `params`. The leaf schema may declare it as a `const`; the lint splices it in.
The `kind` cell is the leaf descriptor's `semanticKind`.

**Steps per artifact:**

1. **Rewrite every Examples row.**
   - Rewrite `params` to the wire payload. The exact bytes are the Rust `ToValue` of the payload the old adapter built from the
     old row. Recipe: temporarily print `pack::to_json_string(&payload)` inside the old adapter for each row. Or write the value
     by hand from the leaf schema and check it with the lint.
   - Do not write `|` inside a cell: the harness splits table rows on every `|`.
2. **Remove `no-mutation` sentinel rows** and their scenarios. `no-mutation` is no mutation leaf; the stdio aggregates dropped
   `NoMutation`. The identity or baseline check becomes its own non-outline scenario without a `{"kind","params"}` docstring.
   Rows of kinds that have no leaf (`identity-round-trip` …) go the same way.
3. **Replace the adapter's hand-mapped `params`→op code** (the per-kind `match` in `mutation_from_spec`) with the generic
   decoder:

   ```rust
   let op = SvgMutation::from_payload_value(spec.str("kind").as_str(), DslValue::from(spec.get("params").unwrap_or(&Json::Null)))
       .map_err(|error| error.to_string())?;
   ```

   `from_payload_value(kind, value)` is the derive-generated static constructor. I requested it from W1-D; it is the owner of
   `#[derive(Mutations)]`. It matches `kind` against the descriptor `semanticKind` and decodes the leaf, or the payload variant of
   a wrapped leaf, through `FromValue`, with the law `from_payload_value(kind, op.payload_value()) == op`.
   Delete every helper that only served the hand mapping: `str_field`, `path_field`, `usize_field`, `json_to_xml_node`, … as
   they become unused. No compatibility path.
4. **Oracles.**
   - If the second implementation (Python/TS) consumed the old shorthand, it now consumes the wire payload: the schema is the
     contract.
   - If it built the op from shorthand, it builds it from the wire payload with the same field names.
5. **Verify.**
   - `bun ./📜️script.ts schema mutation-payloads --under <artifact>` in the repo test module must report 0 findings, and every
     leaf must be witnessed.
   - Then run the artifact's repo case (`bun ./📜️script.ts run --case <case>`) or its cargo test.

**Worked example: svg 1.1 base, `set-declaration`.**

- The leaf is `SetDeclarationMutation { Apply(SetDeclarationPayload), Restore(SvgDiff) }`, marked `payload = Apply`. The payload
  is `SetDeclarationPayload { declaration: Option<XmlDeclaration> }`, and `XmlDeclaration` is camelCase with
  `encoding`/`standalone` skipped when absent and `quote` skipped when `double`. The leaf schema (`$id
  …/s/stdio/svg/1.1/base/mutation/set-declaration/schema.json`) describes exactly that.
- **Before** (hand-mapped; the lint reports `invalid`: `version`, `encoding` and `standalone` are not allowed, because the leaf has
  only `declaration`):

  ```gherkin
  | set-declaration    | {"version": "1.1", "encoding": "UTF-8", "standalone": true} |
  ```

  ```rust
  "set-declaration" => Ok(SvgMutation::SetDeclaration(SetDeclarationMutation::Apply(SetDeclarationPayload {
      declaration: str_field(&params, "version").map(|version| XmlDeclaration::new(version, str_field(&params, "encoding"), match params.get("standalone") { Some(Json::Bool(b)) => Some(*b), _ => None })),
  }))),
  ```

- **After** (wire):

  ```gherkin
  | set-declaration    | {"declaration": {"version": "1.1", "encoding": "UTF-8", "standalone": true}} |
  ```

  ```rust
  fn mutation_from_spec(spec: &Json, _base: &SvgSnapshot) -> Result<SvgMutation, String> {
      SvgMutation::from_payload_value(spec.str("kind").as_str(), DslValue::from(spec.get("params").unwrap_or(&Json::Null))).map_err(|error| error.to_string())
  }
  ```

- **The same file's other rows**, following the lint's messages:

  | Row | Change |
  |---|---|
  | `set-doctype` | `{"doctype": <XmlDoctype wire>}`, not the inner string. The adapter used to prepend `<!DOCTYPE …>`. |
  | `set-view-box` | Carries the payload's own field names. The lint says `viewBox` is not a member. |
  | `set-transform` | Every transform item in the `SvgTransform` wire union. |
  | `no-mutation` | Deleted. |

### F11. D-scope witnesses (done)

Every D-scope leaf now has a committed wire witness. Each witness is asserted by its Rust owner: decoding through
`FromValue` and re-encoding through `ToValue` must give exactly the committed JSON under `serde_json` (key order free, number
spelling exact). The shared helper for this is `store::os_store::test_support::assert_wire_witness::<Op>(witness) -> Op`, in
`🏪️store/🦀️.rs` next to `assert_op_line_round_trip`.

**Single source.** Where a law fixture already carried the wire inline, the inline copy is removed and the rows point at the
witness. This applies to the spr counter laws, the trinity window cases, the gis `mutations.valid` row and the playbook cases.

| Owner | Witnesses | Where | Owner test (all run, gated, private target) |
|---|---|---|---|
| flow-vcs, dag-vcs | 10, 14 | `<artifact>/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` | already in F-earlier: `all_ten`, dag 59/59 |
| store test aggregates (demo, timestamped, severity, validated, lossy) + presence retirement | 15 + 1 | leaf-local `<leaf>/🧫️fixtures/🧾️wire-witness/…` | `semio-framework-os-kernel`: `committed_wire_witness*` |
| store `SpaceHistoryMutation` (6 leaves, adjacent `operation`/`payload`) | 6 | `🏪️store/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/…` | same test; 9/9 os-kernel tests pass |
| spr `testkit.counter` laws, `command.test.counter` laws, registry `mini.doc` | 5 + 5 + 1 | leaf-local | `direct_fixture_contract` ×5, `counter_fixture_codecs_and_descriptors`, `derive_mutations_wires_complete_leaf_and_atomic_registration` |
| plugin fixture aggregates (13 enums) | 17 | leaf-local | `semio-framework-plugin`: 7 `committed_wire_witnesses_are_the_canonical_wire` pass |
| plugin interaction `set-interaction-state` | 1 | leaf-local | `local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned`, derived law `semio_payload_law_interaction_config_mutation`; 73 interaction tests pass |
| workflow, run | 18, 5 | `<artifact>/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/…` | `semio-framework-artifact-workflow-{workflow,run}` pass |
| trinity rewriting window config | 2 | `🎚️config/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/…`; cases name `"witness"` | 5/5 `rewriting_window_config*` with `--features component-app-assembly`; TS twin passes |
| gisterrain camera | 1 | `🎚️config/🧫️fixtures/🧬️mutations/🎥️set-camera/…` | `strict_snapshot_and_aggregate_json_vectors` (`--features component-app-assembly`) |
| playbook procedural `set-payload` | 2 | `🧫️fixtures/🧬️mutations/📦️set-payload/{🎚️sets-params,🫙️clears-params}/🦠️mutation/🔣️.json` | `procedural_payload_vectors_match_the_json_oracle` |
| animate presentation (feature rows) | 18 rows | `🧭️mutate-presentation-1/🥒️.feature`, converted by the F10 recipe | repo case: Python oracle 18/18 mutate+inverse; Rust subject 19/19 |

**Source drifts the witnesses exposed and fixed:**

- **`commit-space-checkpoint`.** The leaf schema said `timestamp.physicalMs`, but `HybridLogicalTimestamp`'s Rust `ToValue`
  emits `physical_ms`. The `transaction-ref` schema already pins `physical_ms`. The schema is fixed.
- **`InteractionConfigMutation`.** This was the only D-scope aggregate the lint could not see. It had hand-written
  `ToValue`/`FromValue`/`Mutation` impls inside `🔌️plugin/🦀️.rs`. It now lives at `🕹️interaction/🧬️mutations/🦀️.rs` as
  `#[derive(dsl::Mutations, ToValue, FromValue)] #[value(rename_all = "camelCase", deny_unknown_fields)]`.
  - `SetInteractionState` implements `MutationKind`.
  - The variant is renamed `SetState` → `SetInteractionState`, because the derive asserts kind == kebab(variant). The descriptor
    `aggregateVariant` is updated to match.
  - The wire is `{"setInteractionState": <InteractionState>}`. No TypeScript or fixture used the old key.
  - The aggregate schema `🕹️interaction/🧬️mutations/🔣️.json` is written.
  - The hand-written aggregate code is deleted. `Default`, `MutationDiff`, `OpText` and `OpBinary` stay.
  - `cargo check -p semio-framework-plugin --lib` is clean.
- **Jack results-window fixture.** `result.graphFixture: null` is not canonical: `skip_serializing_if` omits it. Removed.
- **Animate `replace-source`.** `pdfPage: null` is not canonical: `skip_serializing_if` omits it. Removed.
- **Animate adapters.**
  - `decode_presentation_mutation_json(kind, text)` now decodes the payload through the derive-generated
    `from_payload_value`, with no per-kind mapping.
  - The Python oracle reads `ctx.doc_json()["params"]` and no longer carries a transcribed `PARAMS` copy. The `unwrap` helper is
    deleted.

### F12. Lint changes in this round

1. **Step vocabulary (coordinator item 1).**
   - `mutationFeatureStep(step, kind)` defines a row: a `When` step whose text names the kind and applies, attempts or replays it
     (`… is applied …`, `… is attempted …`, `… is replayed …`).
   - The `identity-round-trip` `Then` docstrings (5 features) and every expectation or codec step are neither witnesses nor
     `unmapped`.
   - The vocabulary comes from a survey of all 520 features: 23 step shapes, all `When … <kind> … applied|attempted|replayed`.
   - A new corpus case pins it: a round-trip `Then`, a `When`+`Then` pair with the same docstring, and an `attempted` error step.
     The corpus test passes, 30/30. `tsc` reports 0 errors.
2. **Uncatalogued `$ref` documents.**
   - `resolveReferencedSchemaDocuments` loads every document a loaded schema references by an absolute `$id` that the catalog
     lacks. It finds them among the `🧬️schema` JSON files of the mutation tree, transitively, and reads them only when a
     reference is missing.
   - This removed 36 false `unresolved`/`aggregate` findings: fixture-aggregate component schemas such as
     `os/store/fixtures/schema.json#/$defs/I32` and the declaration-channel components.
3. **No exemptions (coordinator item 2).** The lint has no exemption input, and `unwitnessed` stays strict. Every former
   exemption candidate in D scope is witnessed; the hand-written interaction aggregate was converted instead of exempted.
4. **Aggregate-rule script now knows the wrapped-leaf branch (coordinator item).**
   - `🧪️w2-s-aggregate-map.ts` records, per variant, the leaf's wrapper enum. It finds it exactly as the lint's `wrappersOf`
     does: `#[mutation_leaf(payload = …)]` in `<leaf>/🦀️.rs` or `<leaf>/🦠️mutation/🦀️.rs`, where the aggregate's payload type
     is that enum.
   - `🧪️w2-s-aggregate-rule.py` points such branches at `<leaf $id>#/$defs/<wrapper>`, like `mutationAggregateBranch`.
   - Repo-wide dry run (`rule-dryrun-3.txt`): glTF has **0** writes, and 2086 files already conform.
   - Three files would still be written: the svg, xml and json `🧱️base` aggregates. They are wrapped-leaf aggregates still in the
     plain-root form, and the lint agrees (`aggregate` finding on svg). They are B2's lane, so they were not applied.

### F13. TS twins broken by the repo-wide rule, repaired

**Cause 1: aggregate is now a `oneOf` of leaf `$ref`s.** The aggregate documents became a `oneOf` of leaf `$ref`s (F-earlier
§5.1), and twins that compiled an aggregate alone failed with `can't resolve reference`.

- New shared helper `addSemioMutationLeafSchemasV1(ajv, mutationsUrl)` in `💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts` adds every
  `<leaf>/🧬️schema/🔣️.json` of a collection.
- Twins now built on `semioSchemaAjvV1` + that helper, all passing:
  - trinity jack graph-window, which also needs the jack artifact/snapshot/value/io/child documents;
  - trinity rewriting window;
  - writer window-state and partial-construction;
  - equation graph-window.

**Cause 2: leaf gained its tag `const`.**

- `store/🧪️tests/🪪️artifact-addressing` now validates the wire form `{mutation: "setDefaultApp", …}`, and the tagless payload
  as rejected.
- reasoning wires:
  - the ten leaf TS interfaces declare their tag literal (`mutation: "moveNode"` …);
  - the `WiresMutation` union is now the plain union of leaves;
  - the twin reads fixtures from `🧫️fixtures/🧬️mutations` (it scanned `🧬️schema/🧬️mutations` and found none).

**Cause 3: `x-semio-ui` in strict Ajv, from W1's annotations.** Switched to `semioSchemaAjvV1`:

- architect program document contract (its `expectedFields` was also stale since 09-21: `knowledgePayload`/`benchmarksPayload`);
- jack document contract;
- replication local-interaction source contract. This one also re-added its own schema three times; it now reuses the added one.

Verified by `🧪️w2-s-ts-twin-sweep.ts`, which runs every exported zero-argument `test*` of a file list in its own process.
`tsc` over the edited twins shows only the pre-existing `@webassemblyjs/leb128` TS7016.

### F14. Census after F11–F13 (`🗑️generated/w2s/census-f12.txt`)

- **Repo:**
  - 4236/4532 fixtures and rows clean. Before: 3476/4465.
  - 2582/3025 leaves witnessed. Before: 2464.
  - 758 findings. Before: 1687. B2's stdio conversion landed in parallel.
- **Framework (`🧰️framework`):** 124/124 clean, 114/114 witnessed, **0 findings**.
- **D scope:** 0 findings in every D owner: architect, block, energy, fem, gis, lowpoly, trinity, animate, playbook,
  procedural, vcs, writer, space, sourcing, demonstrator, wfc, reasoning, remodel, os.
- **Remaining, other lanes:**

  | Owner | Findings | Nature | Lane |
  |---|---|---|---|
  | stdio | 224 `invalid`, 56 `unmapped`, 90 `unwitnessed` | feature-table conversion in flight | B2 |
  | norm | 341 `unwitnessed`, 4 `invalid`, 4 `aggregate` | | W2-S-A |
  | puzzle 2d | 10 `invalid` + 10 `aggregate` | guard fixtures (`🧱️zero-width-node`, `negative-scale` …) violate `exclusiveMinimum`; the wire decodes, `diff` refuses | W1-F / W2-D; decide whether guards stay schema-invalid |
  | layout | 17 `unwitnessed` | | not assigned to me |
  | s-plugin dag `🌳️mutate-dag-1` | 2 `invalid` rows | `replace-node-kind.newKind` matches no branch | not assigned to me |

- **TS twins still failing, not caused by this lane:**
  - semio object/kit document contracts: strict `x-semio-ui`; use `semioSchemaAjvV1` (B2).
  - norm document contract: snapshot lacks `projectId`/`structureKind` (W2-S-A).
  - flow editor source-contract: the demo `.dsl.semio` is no longer JSON after its header.
  - stdio `office-schema-contract`: missing `🧩️composition/🏗️build/🟦️.ts`.
  - draw ×4 and raster ×3 `bun test` twins (W2-S-C): they validate tagless payloads against leaf schemas that now declare the
    tag, and use strict Ajv without `x-semio-ui`. The fix is the F13 recipe.
- **Unrelated failure seen:** `animate … video::program::tests::a_demo_scene_is_one_still_scene_for_every_captured_frame`
  (frame sampling; 336 other tests pass).
- **`schema generate`:** not run; waiting for the coordinator's go.

### F15. Files (this round)

- **Lint and gate:**
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`: `mutationFeatureStep`,
    `resolveReferencedSchemaDocuments`, `schemas` in the tree.
  - `…/🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json`: new feature case.
  - `💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts`: `addSemioMutationLeafSchemasV1`.
- **Kernel and helpers:** `🏪️store/🦀️.rs` (`test_support::assert_wire_witness`).
- **Store:**
  - `🏪️store/🧪️tests/🔬️unit/🦀️.rs`;
  - `🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs`;
  - `🏪️store/🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🧬️schema/🔣️.json`;
  - witnesses under `🏪️store/🧫️fixtures/🧬️mutations/**`.
- **spr:**
  - `📡️spr/🧪️tests/🧬️mutation-laws/🦀️.rs`, `📡️spr/🧪️tests/🧬️mutation-laws-mutations-*/🦀️.rs` (5);
  - `📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🦀️.rs`, `📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`;
  - both `🧫️fixtures/🧬️mutation-laws/🔣️.json`;
  - 11 witnesses.
- **Plugin:**
  - `🔌️plugin/🦀️.rs`;
  - `🕹️interaction/🦀️.rs`, `🕹️interaction/🧬️mutations/{🦀️.rs,🔣️.json}`;
  - `🔁️set-state/{🦀️.rs,🔣️.json,🧪️tests/🔁️set-state/🦀️.rs}`, `🕹️interaction/♻️retirement/🦀️.rs`;
  - 7 fixture test files;
  - 18 witnesses.
- **Workflow:** `🔁️workflow/🧪️tests/🔁️workflow/🦀️.rs`, `🏃️run/🧪️tests/🏃️run/🦀️.rs`, 23 witnesses.
- **Plugins:**
  - trinity rewriting `🎚️config` test (Rust + TS), fixture, 2 witnesses; trinity jack graph-window twin and fixture;
  - gisterrain `🎚️config` test, fixture, 1 witness;
  - playbook procedural unit test, fixture, 2 witnesses;
  - animate `🧬️mutations/🦀️.rs` and `🧭️mutate-presentation-1/{🥒️.feature,🦀️.rs,🐍️.py}`;
  - reasoning wires 10 leaf `🟦️.ts`, `🧬️mutations/🟦️.ts`, document-contract twin;
  - writer ×2 twins, equation twin, architect twin, jack document-contract twin;
  - replication local-interaction twin;
  - store artifact-addressing twin.
- **Ticket scripts:**
  - `🧪️w2-s-aggregate-map.ts`, `🧪️w2-s-aggregate-rule.py` (wrapped-leaf branch);
  - `🧪️w2-s-ts-twin-sweep.ts` (new).

## Follow-up 4 (coordinator decision): invariant refusals are negative witnesses

### F16. Lint rule, corpus, census

**Rule.**

- `mutationFixtureOutcome(outcome)` reads the committed `<case>/🎯️outcome/🔣️.json`. The fixture is **negative** exactly when the
  outcome is `{"status": "rejected", "code": "mutation.invariant"}`.
- `checker.fixture(…, outcome)` and the payload-only (composite) branch send a negative's leaf-payload findings through
  `mutationNegativeVerdict`:
  - clean when the leaf schema rejects it (`invalid`);
  - otherwise one `negative` finding: the schema accepts what the domain refuses.
  - A negative is never reported `invalid` or `aggregate`.
- Positive fixtures are unchanged: failing the schema is `invalid`.
- Negatives are counted in a new census column `negatives`, and the summary line prints them.
- They never witness a leaf: `witnessed` counts positive fixtures and wire rows only, so `unwitnessed` stays strict.
- Every other refusal code (`target-missing`, `duplicate-id`, …) keeps the fixture positive.

**Corpus.** Three new cases (puzzle-2d `scale-selection` shape), all passing with the third-party Ajv oracle (36/36):

- factor 0 with `exclusiveMinimum` → clean negative;
- factor 2 refused as invariant → `negative@`;
- a `target-missing` refusal stays positive → `invalid@` + `aggregate@`.

`tsc` over the lint and corpus is clean. The only error is in a peer's `📚️library/🟦️.ts:2071` (`RUST_TEST_NOCAPTURE`).

**Census** (`🗑️generated/w2s/census-f16.txt`):

- 4110/4535 clean, 189 negative witnesses, 2588/3025 witnessed, 873 findings: 445 unwitnessed, 180 invalid, 179 negative,
  50 unmapped, 2 aggregate.
- **Puzzle 2d:** its 10 guard fixtures are now clean negatives, with 0 findings.
- **179 `negative` findings.** These are invariant refusals the leaf schema still accepts:

  | Owner | `negative` |
  |---|---|
  | energy | 149 |
  | fem | 16 |
  | remodel | 7 |
  | trinity jack | 1 |
  | animate | 1 |
  | flow | 1 |
  | sequence | 1 |
  | forms | 1 |
  | dag | 1 |
  | norm | 1 |

  They fall in three kinds:
  1. **Value ranges the schema should state.** `minimum`/`exclusiveMinimum`/`minLength`/`exclusiveMaximum`, for example energy
     "refuses-a-negative/zero/blank-name/rank-zero", fem zero area/modulus/thickness, ν = 0.5, animate zero-width crop,
     din18599 negative irradiance. This is the bulk; the fix is in the leaf schemas.
  2. **State-dependent refusals coded as `mutation.invariant`.** Jack `create-edge` "endpoints absent", flow `duplicate-widget`
     "onto a taken id", forms `create-block` "step does not exist". No payload schema can reject these. Their outcome code should
     be `mutation.target-missing` / `mutation.duplicate-id`; they then become positive fixtures.
  3. **Cross-field or ordering invariants draft-07 cannot state.** Sequence "step connected to itself", energy "jumbled list",
     "unpaired nodes", fem "sliver outline"/"loose hole". This needs a decision: recode them, or accept a documented
     non-schema invariant class.

  Kinds 2 and 3 are why I did not bulk-edit schemas in this round.
- **New `unwitnessed` from the no-witnessing rule:** trinity jack `create-edge` and forms `create-block` (their only fixtures
  were invariant refusals, kind 2), plus 15 in norm.

**Files:**

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`: `MutationFixtureOutcome`,
  `mutationFixtureOutcome`, `mutationNegativeVerdict`, class `negative`, census `negatives`;
- `…/🧪️test/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json` (3 cases);
- `…/🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts`.

`schema generate` has not been run.

## Follow-up 5 (coordinator decision): declared invariants JSON Schema cannot state

### F17. `x-semio-invariant`, the lint rule, corpus and census

**Decision.** The coordinator settled each kind of schema-valid negative separately:

1. **Value ranges.** The owners fix them in the leaf schemas. The fix is routed to them.
2. **State-dependent refusals.** The owners recode them to `target-missing` or `duplicate-id`. This is also routed to them.
3. **Payload-intrinsic cross-field and ordering rules.** They stay `mutation.invariant`.
   - The leaf schema declares them, schema-first, under `x-semio-invariant`.
   - The refusal's `🎯️outcome` names one of them with `"invariant": "<id>"`.

**Meta-schema (schema-first).**

- **Manifest schema** (`🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json`) gains two `$defs`:
  - `SchemaInvariant`: `{id, description}`. The `id` is kebab-case (`^[a-z0-9]+(?:-[a-z0-9]+)*$`). The `description` is `InputLabelLocales`, so `en` and `de` are required and non-empty.
  - `SchemaInvariants`: a non-empty array of `SchemaInvariant`.
  - Both declare the `🔣️jsonschema` and `🟦️typescript` formats.
- **TypeScript twin:** `SchemaInvariant` and `SchemaInvariants`, plus `parseSchemaInvariants`. The parser also enforces unique ids within one annotation.
- **Vocabulary** (`💻️os/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json`): `x-semio-invariant` now refers to `…/framework/manifest/schema.json#/$defs/SchemaInvariants`. It was `{"type":"string"}`.
  - The strict oracle `semioSchemaAjvV1` now refuses a string value ("data must be array") and accepts the declared form.
- **Migration.** The only previous users were 14 free-text strings in the fem 2d artifact and snapshot schemas. They are rewritten as declared invariants, for example `node-coordinate-finite`, `material-poisson-ratio-admissible` and `analysis-modal-mode-count`, with en and de descriptions. No compatibility form remains.
  - Both documents compile under the strict oracle.

**Lint rule.**

- `mutationFixtureOutcome` now returns `{direction, invariant}`. It reads the outcome's `invariant` for a `mutation.invariant` refusal.
- `mutationSchemaInvariants(document)` collects the ids the leaf document declares in `x-semio-invariant` on any node. `checker.invariants(leafId)` exposes them.
- `mutationNegativeVerdict(payload, outcome, declared)` decides a negative witness:
  - **Clean** when the leaf schema rejects it (`invalid`).
  - **Clean** when it is schema-valid and its outcome names a declared invariant.
  - **`negative`** otherwise. The detail distinguishes "names no invariant" from "names `<id>`, which the leaf neither enforces nor declares (declared: …)".
- **Census.** A new column, `invariants`, counts the negatives cleared by a declared invariant. The summary line prints it too.

**Corpus.** Three new cases on a `connect-steps` leaf with `from == to`: declared, undeclared, and id mismatch. The corpus test also checks `checker.invariants`. The Ajv oracle test accounts for declared clears. 42/42 pass. `tsc` over the lint, corpus, oracle and manifest twin is clean, apart from the peer's `📚️library/🟦️.ts:2071`.

**Census** (`🗑️generated/w2s/census-f17.txt`):

- 4297/4600 fixtures and rows are clean.
- 183 negatives, 1 of them cleared by a declared invariant (an owner has already started).
- 2670/3025 leaves witnessed.
- 667 findings in total. The `negative` findings are energy 149 plus the routed remainder.

**Seen, not caused here.** The fem window-config twin fails on `routes` `maxItems` 18, in the config routes. It is unrelated to invariants.

**Files:**

- `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` and `🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json`
- the fem 2d `🧬️schema/🔣️.json` and `🧬️schema/📸️snapshot/🔣️.json`
- the lint `…/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
- the corpus `…/🧫️mutation-payload-parity/🔣️.json` and its test `…/🧪️tests/🧪️mutation-payload-parity/🟦️.ts`

`schema generate` has not been run.
