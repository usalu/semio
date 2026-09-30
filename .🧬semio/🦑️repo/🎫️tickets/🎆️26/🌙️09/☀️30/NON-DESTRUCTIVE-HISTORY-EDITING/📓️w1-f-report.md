# 📓️ W1-F — Puzzle 2d schema: selection leaves and `x-semio-ui` inputs

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W1-F, 2026-09-30. Contract: `📋️design.md` §6 and §8.
Alias `PZ2D` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`, `MUT` = `PZ2D/🧬️schema/🧬️mutations`,
`FIX` = `PZ2D/🧫️fixtures/🧬️mutations`.

## 1. Outcome

DONE and VERIFIED. All of it compiles, every named test ran green, and the editor still compiles.

- Three new parametric leaves exist with the full leaf anatomy: `drag-selection` (binary tag 33), `rotate-selection` (tag 34)
  and `scale-selection` (tag 35).
- `x-semio-ui` is on every input of all 36 leaf payload schemas, including every field of every `$defs` record. That is 255
  annotations in total.
- There are 15 new fixture quintets.
- The language-agnostic cases were extended in Rust, Python and TypeScript.

## 2. Leaf semantics (decisions)

| Topic | Decision |
|---|---|
| Payloads | `drag-selection {targets, dx, dy}`, `rotate-selection {targets, pivotX, pivotY, angle}` (radians, counter-clockwise, `#[dsl(angle = "rad")]`), `scale-selection {targets, pivotX, pivotY, factor}` |
| Classification | Each id is classified by document membership against the BASE. A node id is a node, a target-region id is a region, and anything else (unknown ids, handles, edges) is missing. A duplicated id moves once. |
| Diff | Every surviving record is patched whole from the base, in document order, so a replay on any base re-derives positions. Only records that actually change are patched. |
| Rotate | Same formula as the editor's `Puzzle2dTransform::Rotate`: `cx + (x-cx)cos − (y-cy)sin`, `cy + (x-cx)sin + (y-cy)cos`, and every handle angle gets `+angle`. The pivot is recorded, not derived. Target regions are axis-aligned by construction, so a region in `targets` is skipped as `mutation.partial` (reason "axis-aligned target regions do not rotate"). This matches the editor and the engine ring (`update_transform_drag`), which never move regions. |
| Scale | Same rule as the editor's `Puzzle2dTransform::Scale`. Node positions spread about the pivot and the node `scale` field is NOT touched ("a node kind's footprint is the kind's"). A region scales its corner AND its extent. One recorded pivot is used for nodes and regions; the editor today uses two separate centroids. |
| Partial | Skipped ids produce one Warning `mutation.partial` per reason, in the fixed order missing → locked → rotation-skipped region, with ids in payload order. Survivors still apply. |
| Target-missing | When nothing survives, the outcome is Error `mutation.target-missing` with the default diff. The message targets are the payload's `targets`. |
| No-op | Identity parameters (`dx = dy = 0`, `angle = 0`, `factor = 1`) short-circuit to Warning `mutation.no-op` with the default diff. The short-circuit exists because `(x−c)+c` is not bit-exact. The same outcome applies when no survivor moves. |
| Invariant | A non-finite parameter or a `factor ≤ 0` is Fatal `mutation.invariant` with the default diff. This is the frozen verb-family rule "move/drag/rotate/scale ⇒ Fatal non-finite or non-positive". |
| Inverse | Base-derived and EXACT, using absolute setters: `move-node` per moved node, `replace-node-handle` per turned handle, `move-target-region` / `resize-target-region` per moved region. A negated parameter is never used, because it would drift in floating point. Nothing moved means an empty inverse. |
| Labels | en/de, for example `Drag 2 items by (5, -2.5)` / `2 Elemente um (5, -2.5) ziehen`, `Rotate 1 item by 90°` / `1 Element um 90° drehen`, `Scale 1 item by a factor of 0.5` / `1 Element um den Faktor 0.5 skalieren`. |

`MUT/🦀️.rs` also holds W1-D's own change, which must be kept when the file is edited. It is a forwarding block that passes
`INPUT_SCHEMAS`, `input_schema`, `payload_value` and `with_payload_value` through the `Value` and `Puzzle2dPlaySnapshot` bridges.
The `MutationLeaf` derive now `include_str!`s each leaf's `🧬️schema/🔣️.json`.

The shared logic lives once, in the aggregate `MUT/🦀️.rs` region `🔖️SelectionTransform`. It has four public helpers, which W2-D
can reuse: `puzzle2d_selection_diff`, `puzzle2d_selection_inverse`, `puzzle2d_selection_number` and `puzzle2d_selection_items`.
Each leaf's `🔺️diff` only validates its parameters and passes its per-record rule to the shared helper.

## 3. Files

### 3.1 New

- `MUT/{✋️drag-selection,🔄️rotate-selection,🔍️scale-selection}/`. Each contains:
  - `🦀️.rs`, `🔣️.json` (descriptor), `🧬️schema/🔣️.json`, `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`;
  - `🧪️tests/<case>/🦀️.rs` for 5 cases.
- `FIX/<leaf>/<case>/{📸️snapshot/⬅️before,➡️after,🦠️mutation,🔺️diff (🚫️.absent for refusals),🎯️outcome}`. The 15 vectors are
  listed in §3.3.
- Ticket authoring tools, kept at the ticket root:
  - `🧪️w1-f-annotate-puzzle2d-inputs.py` writes the `x-semio-ui` tables and the new leaf schemas.
  - `🧪️w1-f-author-selection-vectors.py` writes the fixtures, the Rust leaf tests and the fixture-catalog hashes. It mirrors
    the Rust arithmetic operation by operation.
  - `🧪️w1-f-check-puzzle2d-inputs.ts` runs every schema through W1-D's `mutationInputDefs`.

### 3.2 Changed

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs`: mount tree for 3 leaves and 15 test modules.
- `MUT/🦀️.rs`: enum variants, `KINDS`, re-exports and the `🔖️SelectionTransform` region.
- `MUT/🧪️tests/🔬️unit/🦀️.rs`: kind count changed from 33 to 36, missing-target laws for the new kinds, and 7 new selection
  laws. The laws cover: exact inverse on non-dyadic floats; Fatal invariant; partial ordering and dedupe; rotate region skip and
  regions-only target-missing; identity no-ops; replay on a moved base; en/de labels.
- `MUT/🟦️.ts`: TS twin interfaces and union arms.
- `MUT/🔣️.json`: aggregate `oneOf` grew from 33 to 36 `$ref`s.
- `MUT/📝️text/{📖️.grammar.semio,🔤️.ebnf,🅰️.g4}`: three productions plus `id-list`.
- `MUT/💾️binary/📡️.protocol.semio`: tags 33, 34 and 35.
- `MUT/💾️binary/🧪️tests/🔬️wire-format-guard/🦀️.rs`: round trip of the new kinds, plus an opcode/tag law.
- `MUT/*/🧬️schema/🔣️.json` for the other 33 leaves: `x-semio-ui` annotations.
- `FIX/🔣️.json`: the fixture catalog, now 525 files with hashes recomputed from disk.
- `PZ2D/🔮️oracles/🔣️.json`: catalog vectors, `kinds` and manifest entries, and the native second implementation `vectors`
  count changed from 75 to 105.
- `PZ2D/🧪️tests/◻️mutate-puzzle-2d-1/{🥒️.feature,🦀️.rs,🐍️.py}`: 3 kinds in the mutate and inverse tables and 12 spec-vector
  rows (`<kind>-mixed|partial|refused|unchanged`). The Python second implementation of the three kinds includes the exact
  inverse and an in-role message check.
- `PZ2D/🧪️tests/🕸️third-party-puzzle-2d-1/{🥒️.feature,🐍️.py}`:
  - new shapely geometry checks for the selection kinds, using `affinity.translate/rotate/scale` per survivor; a negative
    control was confirmed;
  - `targetRegions` added to the jsonpatch diff reproduction.
- `PZ2D/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts`: `targetRegions` added to the fast-json-patch diff reproduction. Record
  equality now uses the library's `compare` instead of `JSON.stringify`, because key order is not significant.
- `PZ2D/✏️editor/🦀️.rs`: editor preflight only, see §6.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`: regenerated with
  `bun ./📜️script.ts schema generate` (3551 scopes). It gained the 3 new leaf scopes plus the new hashes of the annotated leaves.
  It also picked up peers' on-disk scope changes: `framework.time-travel`, `framework.tool-machine`, `framework.manifest` and
  others were added or changed, and `framework.os.flow.brep-invoke` was removed.

### 3.3 Fixture vectors

The fixture vectors were renamed to stay at or under the 240-byte taxonomy path limit; the longest is now 238 bytes. They all use
one synthetic selection board: node-a, node-b, a LOCKED node-c, region-1, and a LOCKED region-2.

| Leaf | Applied | Mixed nodes + regions | Partial | Target-missing | No-op |
|---|---|---|---|---|---|
| `drag-selection` | `✋️drags-two-nodes` | `🎯️drags-node-and-region` | `⚠️skips-locked-ghost` | `🚫️rejects-ghosts` | `⏸️keeps-a-zero-offset` |
| `rotate-selection` | `🔄️turns-two-nodes` (π/2 about (20, 10), handles turned) | `🎯️skips-the-region` (node turns, region partial) | `⚠️skips-locked-ghost` | `🚫️rejects-ghosts` | `⏸️keeps-a-zero-angle` |
| `scale-selection` | `🔍️doubles-two-nodes` | `🎯️halves-node-region` | `⚠️skips-locked-ghost` | `🚫️rejects-ghosts` | `⏸️keeps-a-unit-factor` |

Outcome files use the outcome-class vocabulary:
- applied: `{"status":"applied"}`, with `messages` present when the vector is partial;
- no-op: `{"status":"no-op","messages":[…]}`;
- refused: `{"status":"rejected","code","path"}`.

Messages carry `level`, `code` and `target`. Rotation pivots and angles were chosen so that the `cos(π/2) ≈ 6e-17` terms vanish
below half an ulp. As a result, the Rust, Python and shapely answers are bit-identical.

## 4. `x-semio-ui` (design §6)

W1-D confirmed this annotation shape by message, and the checks below use W1-D's landed meta-schema and reader.

- **Reference ids.** Every id input is `{widget: reference, role: target, ref: {kind, domain: "vortex", granularity}}`.
  - `kind` is one of node, edge, handle or targetRegion.
  - Target regions use granularity `node`, because that is how the vortex domain selects them today.
  - `targets` is `kind: ["node","targetRegion"]` for drag and scale, and `["node"]` for rotate.
  - `many`, `minItems: 1` and `uniqueItems` come from the array type.
- **Positions and offsets.** dx, dy, x, y, pivots, extents and edge x/y use `stepper`, `step 1`, `precision 2` and
  `snapSource {config: "gridFactor"}`.
- **Angles.** `angle` (and handle or template angles) use `dial`, `unit rad`, `displayUnit deg` and
  `displayFactor 57.29577951308232`. The step is 1° in radians, the soft range is ±π, and the snaps are −π, −π/2, 0, π/2 and π.
- **Factors.** `factor` (and node or handle `scale`) use `slider`, `scale log`, `softMin 0.1`, `softMax 10`, snaps
  `[0.25, 0.5, 1, 2, 4]`, `step 0.01` and `precision 2`.
- **Booleans** use `toggle`.
- **Enums** use `select` or `segmented` with localized `options`: shape, anchor (Fixed/Fest, Derived/Abgeleitet) and specificity.
- **Indices** use `stepper` with `step 1`, `precision 0` and the existing `minimum: 0`.
- **Free text** uses `text`, or `multiline` for descriptions.
- **Discriminator.** The `mutation` const is `{widget: hidden, role: discriminator}`.
- **Groups and order** are set on every input.
- **Hard bounds** were added in standard keywords:
  - `exclusiveMinimum: 0` on `factor`, `newScale`, `newRadius`, `newWidth` and `newHeight`, and on node radius, width, height
    and scale and handle radius and scale;
  - `minimum 0 / maximum 1` on template `t`, and `minimum 0` on `order` and `rank`;
  - `minItems 1` and `uniqueItems` on `targets`;
  - `newShape` became `anyOf [{enum circle|rectangle}, {type null}]`. The reader refuses a labelled enum that contains `null`,
    and null must stay valid.
- **Factor bound.** The factor's hard bound is `exclusiveMinimum: 0`, not 0.1..10. A gesture may legitimately exceed the slider
  range, so 0.1..10 is the soft slider range.
- **Region extents.** Target-region width and height have no lower bound, because a brush stroke can leave a negative extent.

## 5. Verification (exact commands and counts)

All of these were run in the foreground, with the rustc gate for cargo and a private uplift dir
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-w1f`.

| Check | Result |
|---|---|
| `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --lib --message-format=short` (final run, after the renames) | **708 passed, 0 failed**. About 100 of these are selection tests: 7 per applied vector, 6 per no-op, 4 per refusal, 7 laws, 2 wire guards. There are 0 warnings in puzzle 2d files. |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | **Finished**, exit 0. No warning comes from the edited preflight; the listed editor warnings are pre-existing. The first attempt failed only in peers' in-flight `semio-framework-plugin` / dsl derive; the retry was green. |
| `bun ./📜️script.ts oracle exhaustive --owner PZ2D --case ◻️mutate-puzzle-2d-1` (test domain) | **142/142** (Python second implementation) |
| `… subject exhaustive … ◻️mutate-puzzle-2d-1` | **142/142** (Rust adapter) |
| `… parity exhaustive … ◻️mutate-puzzle-2d-1` | **284/284, parity 142/142** |
| `… oracle long … 🕸️third-party-puzzle-2d-1` | **6/6**: networkx, shapely (new selection geometry), jsonschema, jsonpatch plus deepdiff (now with regions), region containment |
| `… oracle long … 🌐️third-party-puzzle-2d-1` | **4/4**: graphology, npm jsonschema, fast-json-patch (now with regions) |
| `bun nx run '…◻️mutate-puzzle-2d-1:test-contract'` | Red repo-wide: 2435 high-priority breaches in other owners. The only puzzle 2d breach is the pre-existing `runtime-inventory-missing`. There is no breach for the new catalog, vectors, feature rows or manifest. |
| W1-D reader `mutationInputDefs` (TS) over all 36 schemas (`🧪️w1-f-check-puzzle2d-inputs.ts`) | **103 top-level inputs, 0 failures**, including nested record fields |
| Python `jsonschema` Draft7 of every `x-semio-ui` against manifest `$defs/InputUi` | **255 annotations, 0 errors** |
| W1-D's own gate, run and reported by W1-D: `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"` | 103/103 inputs of 36 leaves declared, 0 findings |
| All 90 pre-existing committed payloads against the annotated leaf schemas (Python jsonschema) | 0 errors |
| `bun ./📜️script.ts schema check` | Identical puzzle 2d diagnostics before and after: 77 lines, none new |
| `verify taxonomy report --scope` on the 6 new dirs | Only pre-existing finding classes remain; see §7.1. The path-too-long finding was eliminated by the renames. |

## 6. Editor (W2-D, please read)

Handling added in `PZ2D/✏️editor/🦀️.rs` `Puzzle2dArtifactStorePreparationFactory::preflight`:
- `DragSelection` declares `1 + targets` work items.
- `ScaleSelection` declares `1 + 2·targets`.
- `RotateSelection` declares `min(1 + targets·(1+64), MAX)`, using the new const `PUZZLE2D_ROTATE_SELECTION_INVERSE_ROWS`.

These are needed because the exact inverse has one row per moved record and per turned handle; the default declaration of 2
would fail-close ("batched item candidate failed its exact fixed fold contract"). No other editor code enumerates the variants.

For W2-D:
1. Yield only NODE ids for `rotate-selection`. A region id there is reported as `mutation.partial`, by design.
2. Choose and record the pivot. The editor's per-collection centroids are gone at the leaf level.
3. The leaf diff does not refuse the whole gesture on a locked member, unlike `refuse_when_locked`. Decide whether the tool still
   pre-refuses.
4. `puzzle2d_selection_*` helpers are public for the reducers.
5. Derived-anchor nodes move their stored x/y like `move-node` does. The inference may still pin them.

## 7. Findings outside this WP

### 7.1 Tooling limitations

- **Taxonomy report findings.** Every new dir reports `directory-kind-unresolved` and fixture `mutation-fixture-unpaired`.
  FE0F-named fixture leaves also report `mutation-payload-schema-authority-invalid`. Existing leaves report the same classes
  (`🚀move-target-region`, `🪢️connect-handles`, shooting `↔️drag-assets`). The scoped verifier cannot load the oracle catalog
  projection: `projection-catalog-invalid … catalog is missing at PZ2D/🔮️oracles/🔣️.json`, although the file exists and the
  contract phase reads it.
- **Whole-scope taxonomy crash.** `verify taxonomy report --scope PZ2D` crashed before my edits with
  `frozen-coordinate-evidence-invalid` on `📚️library/🧫️fixtures/📐️cad-draw-path-projection`, which is a peer file.
- **Fixtures lint.** `puzzle-plugin fixtures-lint` was not run. Its `CORE_CASE_FILES` still expects quintets under
  `<leaf>/🧪️tests/<case>/`, so it predates the `🧫️fixtures/🧬️mutations` layout.
- **Runtime inventory.** The runtime inventory (`test inventory`, production bridge) was not produced, so the contract breach
  `runtime-inventory-missing` for `s.puzzle.2d@1/any` persists. It was there before.

### 7.2 Pre-existing issues fixed in owned files

- **Catalog hash.** `FIX/🔣️.json` had a stale sha256 for `🪢️connect-handles/⏸️keeps-an-edge…/🎯️outcome` (status changed to
  `no-op` on 09-25). The regenerated catalog fixes it.
- **Mutate case no-op branch.** The `◻️mutate-puzzle-2d-1` Rust `noop` spec-vector branch required status `applied`, while the
  committed no-op vector says `no-op`. It now requires `no-op`, and `connect-handles-duplicate` passes in the subject phase.

### 7.3 Peer work still open

- **Unfinished renames.** The `WINDOWS-CLONE-PATH-BUDGET` peer renamed two puzzle 2d reject fixture dirs:
  `🚫️rejects-withdrawing-pair-relation-never` and `🚫️rejects-locking-region-board-never-held`. The oracle manifest scenarios
  and the Rust test dir names still use the old long names. I did not touch them. Other existing fixture paths still exceed the
  240-byte taxonomy limit.

## 8. Not done / follow-ups

- The text grammar mirrors (`📖️.grammar.semio`, `.ebnf`, `.g4`) are hand-written documentation in the same positional style as
  their 33 siblings. The real codec is the dsl record printer, and it round-trips in the wire guard test.
- The generated `🔗️.graphql` and `🛰️.proto` facets still describe the stale snapshot shape, as they did before, and list no
  kinds.

## Follow-up — schema hard bounds and Rust invariants agree

Audit item: `📓️audit-inputs-leaves.md` §2.7 ("puzzle 2d schema hard bounds are stricter than the Rust diffs"), routed by the
coordinator. I stayed in `🧬️schema`, `🧫️fixtures`, `🧪️tests` and `🔮️oracles`. The only other file touched is the test-module
mount block of the artifact root `◻️2d/🦀️.rs` (10 `#[cfg(test)] mod` lines, needed to run the new tests). The editor was not
touched, and the aggregate `oneOf` order from W2-S was kept.

### F.1 Rule (schema-first)

Every value a puzzle 2d leaf payload schema forbids through a hard bound is refused by that leaf's `🔺️diff` as a Fatal
`mutation.invariant` with the default diff. The check runs before the base is read, so the answer is the same on any board.

The shared checks live in the new aggregate region `MUT/🦀️.rs` `🔖️Invariants`:

| Check | Rule |
|---|---|
| `puzzle2d_finite` | named numbers are finite |
| `puzzle2d_positive` | present numbers are finite and greater than 0 |
| `puzzle2d_shape` | a shape is `circle` or `rectangle` |
| `puzzle2d_targets_invariant` | the target set is non-empty and names no id twice |
| `puzzle2d_handle_invariant` | finite angle; radius and scale greater than 0 when present |
| `puzzle2d_node_invariant` | finite x/y, a known shape, radius/width/height/scale greater than 0, valid handles |
| `puzzle2d_region_invariant` | finite corner and extent (a zero or negative extent is a brush stroke and is admitted by the schema) |
| `puzzle2d_catalogs_invariant` | template angle finite, template radius greater than 0, template `t` within 0..=1, author `rank` ≥ 0, handle-kind `order` ≥ 0 |

These are the 15 leaves that now emit the invariant:
- `move-node`, `move-target-region`, `resize-target-region`, `connect-handles`, `replace-edge-geometry`: finite numbers.
- `replace-node-geometry`: shape, plus radius/width/height greater than 0.
- `scale-node`: `newScale` greater than 0.
- `create-node`: node record.
- `add-node-handle`, `replace-node-handle`: handle record.
- `create-target-region`: region record.
- `replace-kind-catalogs`: catalogue.
- `drag-selection`, `rotate-selection`, `scale-selection`: parameters, plus the target set.

The other 21 leaves carry only strings, booleans, enums and `usize` indices. Their schema bounds (`enum`, `minimum: 0` on
`index`) are enforced by the Rust type at decode, so no diff-level value is forbidden.

**Behaviour change.** A repeated target in `targets` was silently de-duplicated. It is now the invariant the schema's
`uniqueItems` states, and the de-duplication code is deleted from both implementations. An empty `targets` was
`mutation.target-missing` and is now the invariant (`minItems: 1`).

**For W2-D.** The select tool must yield non-empty, unique target lists and factors greater than 0; anything else is refused as
Fatal.

**German labels.** Also closed, from the audit §3.3 first bullet: `puzzle2d_selection_number` now answers `(en, de)` with a
decimal comma in German. The German drag label uses `;` between coordinates, e.g. `(5; -2,5)`.

### F.2 Invariant fixtures

There are 10 new quintets, all `rejected` with code `mutation.invariant`, the `🔺️diff/🚫️.absent` sentinel, and before equal to
after. The board is the same synthetic selection board. The longest path is 238 bytes.

| Leaf | Case | Forbidden value |
|---|---|---|
| `drag-selection` | `🧱️no-targets` | `targets: []` (`minItems`) |
| `rotate-selection` | `🧱️repeated-targets` | `node-a` twice (`uniqueItems`) |
| `scale-selection` | `🧱️zero-factor`, `⛔️negative-factor` | `factor` 0 and −1 |
| `scale-node` | `🧱️zero-scale` | `newScale: 0` |
| `replace-node-geometry` | `🧱️negative-radius` | `newRadius: -4` |
| `create-node` | `🧱️zero-width-node` | node `width: 0` |
| `add-node-handle` | `🧱️zero-radius-handle` | handle `radius: 0` |
| `replace-node-handle` | `🧱️negative-scale` | handle `scale: -1` |
| `replace-kind-catalogs` | `🧱️off-rim-template` | template `t: 1.5` |

**Where they are registered.** The vectors are in the oracle manifest (native `vectors` count is now 115) and in the fixture
catalog (575 files; hashes recomputed from disk). They are also 10 `refused` spec-vector rows in `◻️mutate-puzzle-2d-1`
(`<kind>-invariant`).

**Rust leaf tests.** There are 3 per case:
- the board stays at the committed after;
- there is exactly one Fatal `mutation.invariant` at the declared path, with the default diff;
- the refusal is independent of the base (the empty board gives the same result).

Values that JSON cannot carry (NaN, ±∞) are covered by the new unit law `every_bounded_leaf_refuses_what_its_schema_forbids`.
It checks 30 forbidden payloads across all 15 leaves, and 4 admitted edge values (a negative region extent, `newScale: null`,
`t = 1`, and a valid rectangle rebuild). The selection invariant law gained empty and repeated target sets.

### F.3 Second implementation and third-party cases

**Python second implementation.** `◻️mutate-puzzle-2d-1/🐍️.py` now carries its own `invariant_violation`, written from the
schemas:
- finite-number arguments and positive arguments per kind;
- the node, handle, region and catalogue records;
- index ≥ 0, the specificity enum, and non-empty and unique targets.

`apply_mutation` refuses such a payload before it looks at the board. The refused spec-vector handler requires the committed
code to be `mutation.invariant` exactly when this check fires. A local cross-check over all 115 committed vectors found 0
disagreements.

**Third-party payload-schema checks.** `🕸️third-party-puzzle-2d-1` (Python `jsonschema`) and `🌐️third-party-puzzle-2d-1` (npm
`jsonschema`) now require the validator to REJECT every committed payload whose outcome is `mutation.invariant`, and to accept
every other payload. So a third-party validator states that the schema forbids exactly what the subject refuses. Both feature
files state the new step.

### F.4 Verification

| Check | Result |
|---|---|
| `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --lib` (private uplift dir) | **750 passed, 0 failed**, 0 warnings in puzzle 2d files. The first attempt was SIGKILLed (exit 137) under load 45 and 6.9 GB of swap; the retry was green. |
| `bun ./📜️script.ts oracle exhaustive --owner PZ2D --case ◻️mutate-puzzle-2d-1` | **152/152** (Python, including the 10 invariant rows) |
| `… oracle long … 🕸️third-party-puzzle-2d-1` | **6/6** |
| `… oracle long … 🌐️third-party-puzzle-2d-1` | **4/4** |
| `… subject exhaustive … ◻️mutate-puzzle-2d-1` | **152/152** (Rust adapter) |
| `… parity exhaustive … ◻️mutate-puzzle-2d-1` | **304/304, parity 152/152** |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly` | **Finished**, exit 0. This includes W2-D's in-flight editor state. |

### F.5 Files

- **Changed:**
  - `MUT/🦀️.rs`: the `🔖️Invariants` region. `puzzle2d_selection_diff` now checks targets first and no longer de-duplicates. `puzzle2d_selection_number` returns `(en, de)`.
  - The `🔺️diff/🦀️.rs` of the 12 existing bounded leaves listed in F.1.
  - The drag, rotate and scale leaf `🦀️.rs` labels.
  - `MUT/🧪️tests/🔬️unit/🦀️.rs`.
  - `◻️2d/🦀️.rs` (mounts only).
  - `FIX/🔣️.json`.
  - `PZ2D/🔮️oracles/🔣️.json`.
  - `◻️mutate-puzzle-2d-1/{🥒️.feature,🦀️.rs,🐍️.py}`.
  - `🕸️third-party-puzzle-2d-1/{🥒️.feature,🐍️.py}` and `🌐️third-party-puzzle-2d-1/{🥒️.feature,🟦️.ts}`.
  - The ticket-root tool `🧪️w1-f-author-selection-vectors.py`, which now also writes the invariant vectors.
- **New:** 10 fixture quintets under `FIX/<leaf>/<case>/` and 10 test files under `MUT/<leaf>/🧪️tests/<case>/🦀️.rs`.
- **Not done:** the audit's "fail closed when the validator is `None`" belongs to `⏪️time-travel` (W1-B/W2-A) and is outside this WP. I did not change it.
