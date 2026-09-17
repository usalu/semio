# 📓️ Wave 2F — puzzle ◻️2d target regions + area brush (the flat twin of 3d's target volumes)

Slice 2F of the 2d/5d parity fleet. Ported from `🧊️3d`'s `target-volume` family and its
`🧊️volume-brush` utility; everything below is the flat analogue, never a second mechanism.

**Vocabulary.** A **target region** is an axis-aligned board rectangle `{x, y, width, height}` that
constrains where FILL may place, exactly as a 3d target volume constrains `world_volumes_contain_aabb`.
An **area brush** paints one, grid-snapped, on alt+click — the 2d twin of the volume brush.

---

## 1. What landed

### 1.1 Document record and schema (schema-first, all twins hand-updated)

| file | what |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs:464-524` | `Puzzle2dTargetRegion { id, x, y, width, height, label: Option<String>, hidden, locked }` + `bounds()` (normalizes a negative extent) + `contains_bounds()`; and the free function `puzzle2d_regions_contain_bounds` (:513) — *the* fill rule, flattened from 3d's `world_volumes_contain_aabb`. An empty VISIBLE set is unconstrained. |
| `…/✳️any/🧬️schema/📸️snapshot/🦀️.rs:40-46` | `Puzzle2dSnapshot.target_regions: Vec<Puzzle2dTargetRegion>`, `#[dsl(table)]`, **`skip_serializing_if = "Vec::is_empty"`** |
| `…/✳️any/🧬️schema/🦀️.rs:21,37,42,51,217` | `Puzzle2dArtifact.target_regions` + the three snapshot conversions + the re-export |
| `…/✳️any/🧬️schema/🔺️diff/🦀️.rs:24,88-110` | `Puzzle2dDiff.target_regions: Option<Puzzle2dTargetRegionsDelta>` + `…Delta` / `…PatchEntry` / `…Patch` |
| `…/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:35,107,137,199` | `apply_target_regions_delta` wired into **both** `apply_to_artifact` and `MutationDiff<Puzzle2dSnapshot>::apply`, plus the `absorb` arm |

> 🎯️ **The one deliberate asymmetry, and why.** `target_regions` is the only collection on the 2d
> snapshot that is OMITTED when empty (`nodes`/`edges` always write `[]`). Without it, adding the
> field would have rewritten all 150 committed snapshot leaves of the mutation corpus and every
> `.op`/`.pack` twin derived from them. With it, a board that never had a region and a board whose
> last region was deleted have the identical wire form, and the whole pre-existing corpus stayed
> byte-identical. The Python second implementation states the same rule (`OPTIONAL_MEMBERS`) and
> `validate` refuses an empty `targetRegions` written as `[]`.

**Twins updated by hand** (12 files): `🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`,
`📸️snapshot/{…}`, `🔺️diff/{…}`. The diff's JSON schema gained
`Puzzle2dTargetRegionsDelta`/`…PatchEntry`/`…Patch` `$defs`.

**The 53 committed `🔺️diff/🔣️.json` fixtures gained `"targetRegions": null`** between `"edges"` and
`"meta"` — `Puzzle2dDiff` serializes every field including nulls, in declaration order, so this was
forced. Their sha256 rows in `🧫️fixtures/🧬️mutations/🔣️.json` were refreshed (53 digests).

### 1.2 The seven mutations — `…/✳️any/🧬️schema/🧬️mutations/`

| dir | variant | kind | shape | 3d source |
|---|---|---|---|---|
| `🌍create-target-region` | `CreateTargetRegion` | `create-target-region` | `{targetRegion, index}` | `🌍create-target-volume` |
| `🪦delete-target-region` | `DeleteTargetRegion` | `delete-target-region` | `{id}` | `🪦delete-target-volume` |
| `🚀move-target-region` | `MoveTargetRegion` | `move-target-region` | `{id, newX, newY}` | `🚀move-target-volume` |
| `📐resize-target-region` | `ResizeTargetRegion` | `resize-target-region` | `{id, newWidth, newHeight}` | `📐scale-target-volume` |
| `🖋️edit-target-region-label` | `EditTargetRegionLabel` | `edit-target-region-label` | `{id, newLabel}` | *(replaces 3d's `🌀rotate-target-volume`, which an axis-aligned rectangle cannot answer)* |
| `🙈change-target-region-hidden` | `ChangeTargetRegionHidden` | `change-target-region-hidden` | `{id, newHidden}` | `🙈change-target-volume-hidden` |
| `🔏change-target-region-locked` | `ChangeTargetRegionLocked` | `change-target-region-locked` | `{id, newLocked}` | `🔐change-target-volume-locked` (🔐 was already taken in 2d by `change-edge-locked`) |

Each leaf carries the full 2d quintet: `🔣️.json` descriptor, `🧬️schema/🔣️.json` draft-07 branch with
the `"mutation"` const discriminator and `additionalProperties: false`, `🦀️.rs` payload +
`MutationKind` impl, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, and `🧪️tests/<scenario>/🦀️.rs`.

Registries extended: the `Puzzle2dMutation` enum (`🧬️mutations/🦀️.rs:63-69`), `KINDS` (:98-104,
**26 → 33**), the seven `pub use` re-exports, `puzzle2d_snapshot_mutations`'s region delta block
(:280-306 — the semantic differ now emits region mutations), the 33-branch `oneOf` union
(`🧬️mutations/🔣️.json`), the `🟦️.ts` union + seven payload interfaces, and the three hand-written
grammars (`📝️text/{📖️.grammar.semio,🅰️.g4,🔤️.ebnf}`) with a new `region-block` production.
`💾️binary/🦀️.rs` and `📝️text/🦀️.rs` are generic over `DslVariants` and needed no edit.

Module wiring: `◻️2d/🦀️.rs:1295-1420`, seven `#[path]` blocks with their 15 `#[cfg(test)]` scenarios.

### 1.3 Fixture corpus — 15 vectors, `…/✳️any/🧫️fixtures/🧬️mutations/`

| kind | applied | rejection |
|---|---|---|
| create | `🌍️appends-region-2` (alpha board) **+ `🌍️paints-a-tower-footprint` (real-world: the shipped `concrete-forest` seed)** | `🚫️rejects-a-region-id-the-board-already-holds` (`mutation.duplicate-id`, Fatal) |
| delete | `🪦️removes-region-1` | `🚫️rejects-deleting-a-region-the-board-never-held` |
| move | `🚀️slides-region-1` | `🚫️rejects-moving-…` |
| resize | `📐️widens-region-1` | `🚫️rejects-resizing-…` |
| label | `🖋️renames-region-1` | `🚫️rejects-renaming-…` |
| hidden | `🙈️hides-region-1` | `🚫️rejects-hiding-…` |
| locked | `🔏️locks-region-1` | `🚫️rejects-locking-…` |

Every rejection commits the contract-D6 empty `🔺️diff/🚫️.absent` sentinel, never an invented empty
patch. All 75 new leaf files are catalogued with their sha256 in `🧫️fixtures/🧬️mutations/🔣️.json`
(375 → 450 entries).

> 🔢️ **Fixture numbers are non-integral by construction** (`-12.5`, `80.5`, `120.5`). `dsl::json`
> spells a whole `f64` `20` while `serde` spells it `20.0`, and the leaf tests compare through both;
> a half-integer is the one form on which the two agree. Measured, not assumed — see §4.4.

### 1.4 Commands — `…/✳️any/✏️editor/🎮️commands/`

| dir | verb | kind | lanes | work |
|---|---|---|---|---|
| `🎯️add-target-region` | `addTargetRegion {origin, size?}` | Mutation | `Artifact` | generic |
| `🪦️delete-target-region` | `deleteTargetRegion {id}` | Mutation | `Artifact` | generic |
| `🚚️relocate-target-region` | `relocateTargetRegion {regionId, after:{position,size}}` | Mutation | `Artifact` | generic |
| `🚩️set-target-region-flag` | `setTargetRegionFlag {id?, flag, value}` | Mutation | `Artifact` | generic |
| `🖍️set-area-brush-size` | `setAreaBrushSize {axis, value}` | View | `WindowConfig` | generic |

All five are `InteractiveJobClassification::Migrated`. Registries touched in
`…/✳️any/✏️editor/🦀️.rs`: `puzzle2d_command_variants!` (:1678-1682), `PUZZLE2D_RETAINED_TOOL_IDS`
(:1950-1954), `PUZZLE2D_GENERIC_TOOL_IDS` (:1999-2003), `PUBLICATION_CONTRACTS` (:2126-2130),
`puzzle2d_dispatch_emit`'s match (:2748-2752), `bounded_first_step_tool_proofs!` tools (:4696-4700),
`.mutation`/`.action_with` (:5226-5233), `.action_interactive_job` (:5318-5322),
`puzzle2d_generic_extent` (:2831-2839 — `setTargetRegionFlag` is selection-sized, or 1 when it
addresses an explicit `id`). `command_from_action` and `build_tool_job` are generic and needed no
per-verb row. Fixtures: `🧫️fixtures/🗄️retained-jobs/🔣️.json` `toolIds` (49 → 54) and
`🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` (artifact group +4, window-config group +1).

**Selection verbs now act on regions too**, as 3d's do on volumes:
`📏️scale-selection/🦀️.rs` (extent scales with the gesture — a region has no kind catalogue to take a
footprint from), `🚀️translate-selection/🦀️.rs`, `🗑️delete-selection/🦀️.rs`. A rotation is
deliberately refused: a target region is axis-aligned by construction
(`puzzle2d_transform_target_regions`, editor root :1113).

Editor-root helpers, all in one `//#region 🎯️TargetRegions` (`…/✏️editor/🦀️.rs:1005-1144`):
`fixture_target_regions`, `puzzle2d_selected_target_region_ids`, `puzzle2d_region_bounds`,
`puzzle2d_fixture_regions_admit`, `puzzle2d_paint_target_region`, `puzzle2d_relocate_target_region`,
`apply_target_region_flag`, `delete_target_regions_from_fixture`, `puzzle2d_transform_target_regions`,
`puzzle2d_target_region_centroid`. New granularity `PUZZLE2D_GRANULARITY_TARGET_REGION = "targetRegion"`
(:81).

### 1.5 Area brush utility + window config

- `…/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖍️area-brush/🦀️.rs` — `UTILITY_ID = "areaBrush"`,
  icon `square-dashed`, and the W/H extent sliders (1…64 grid cells) as its Utility Options group.
  Direct port of EDITOR3's `🧊️volume-brush` with the voxel W/D/H trio collapsed to W/H.
- Registered: `create_puzzle2d_app` `.utility(...)` (`✏️editor/🦀️.rs:5341`), the overview window's
  `utilities` vec and its `window_measures` (`…/👁️overview/🦀️.rs:33,46`).
- Window config gained `area_brush_width`/`area_brush_height`
  (`🪟️window/🦀️.rs:24-25,42-43,464-465,495-496`), the runtime gained the same two
  (`🎚️config/🦀️.rs:104-109`, default `PUZZLE2D_DEFAULT_AREA_BRUSH_EXTENT = 1.0`), and all four
  window-schema twins were updated by hand (`🪟️window/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`).
- **Alt+click paint** — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`:
  `board2dAreaBrushCommits(activeUtility, altKey, button)` (:368, exported for the engine-contract
  tests) and the `onPointerDown` gate (:1114-1123) that converts the screen point with the existing
  `puzzle2dScreenToWorld` and dispatches `addTargetRegion {origin:[x,y]}` **instead of** letting the
  gesture reach the engine. This is the exact shape of 3d's `World3dHost` alt+click
  (`world3dVolumeBrushCommits` / `dispatch("addTargetVolume", { origin })`).

### 1.6 Fill constraint — `…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`

- `FillRunReason::OutsideTargetRegion` (:144), `ALL` is now 12 (:148), id `"outside-target-region"`
  (:184), label `fill_reason_outside_target_region` (:211). Appended, so every existing reason's
  `code()` is unchanged and committed traces stay readable.
- `fill_visible_region_bounds` / `fill_regions_admit` (:333-351) and
  `FILL_RUN_TARGET_REGION_SLOTS = 256` (:26).
- `Puzzle2dFillRunJob.target_regions` is read ONCE at construction from the run's base document, and
  `accept()` rejects a placement whose own `fill_node_bounds` footprint is not admitted: the
  candidate is decided `warning:outside-target-region`, `rejected` is incremented and the search
  continues. Empty visible set ⇒ unconstrained, exactly 3d's rule.
- EN + DE labels for the new reason and for `area_brush` / `target_region` / `target_regions` /
  `target_region_origin_required` (`🗣️terminology/🦀️.rs:85-88,120`).

### 1.7 Panels (coordinate with slice 2C — see §5)

Both additions are inside their own clearly-fenced regions so 2C's work never anchors on the same
lines:
- `📌️panels/🗿️artifact/🦀️.rs` — `//#region 🎯️TargetRegionRows` with `region_flag_row_action` +
  `target_region_row`, and `TARGET_REGIONS_SECTION` as a third collapsed outliner section.
- `📌️panels/🔍️inspection/🦀️.rs` — `//#region 🎯️TargetRegionFields` with `region_flag_row` +
  `target_region_fields`, resolved after nodes and edges in `selected_section`.

Row/field actions dispatch `setTargetRegionFlag`, never `setSelectionFlag`: the two verbs address
different collections and a shared action would flag a node that happened to share the id.

### 1.8 Tests and oracles

- **15 leaf fixture tests** (`🧬️mutations/<kind>/🧪️tests/<scenario>/🦀️.rs`), 8 applied × 7 laws and
  7 rejections × 5 laws: applies-to-after, inverse-restores-before, JSON canonicality, declared
  outcome, produces-committed-diff, diff canonicality, diff-applies-to-after; the rejections add the
  D6 sentinel + exact code/level/target. Snapshot canonicality goes through `dsl::json` (§4.4).
- **`fill_run_job_places_only_inside_visible_target_regions`**
  (`⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:240-299`) — four halves in one law: every placement
  inside the region, at least one `outside-target-region` refusal, the same run WITHOUT the region
  provably leaves it (so the vector can tell a constraint from a coincidence), and a HIDDEN region
  constrains nothing. Driven by a new `targetRegion` vector in
  `⏳️precompute/🪣️fill/🧫️fixtures/🎞️fill-run.json`, stated as a `halfSpan` around the seed node's own
  centre rather than absolute coordinates, so it is language-neutral and survives re-authoring.
- **Python second implementation** — `🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py`: `OPTIONAL_MEMBERS`,
  `REGION_MEMBERS`, `REGION_FLAGS`, `REGION_GEOMETRY`, `region_at`, seven forward branches, seven
  inverse branches, `validate` widened, and `equals_committed`/`restores` widened to compare the
  optional collection through `.get` (a member one side omits and the other writes empty is now a
  reported difference, not a skipped one). `KINDS` 26 → 33, `SPEC_VECTORS` 49 → 57.
- **Third-party shapely oracle** — `🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py` gained a
  `//#region 🎯️Regions` with `region_vectors` / `region_box` / `regions_of` / `region_containment`,
  registered as the `region-containment` scenario, plus its Gherkin scenario in that case's
  `🥒️.feature`. It adjudicates by containment and intersection: area == |w·h|, move == shapely
  `translate`, resize keeps the minimum corner, label/flag verbs move no geometry, **a painted region
  never straddles a node** (contains its whole footprint or does not touch it), and a refused vector
  moves nothing.
- Oracle registrations: `🔮️oracles/🔣️.json` — `mutationCatalogs[0].vectors` (+7), `.kinds` (+7,
  26 → 33) and `mutationManifests[0].mutations` (+7). The Gherkin `🥒️.feature` of
  `◻️mutate-puzzle-2d-1` gained 7 rows in each exhaustive table and 8 in the spec-vector table;
  `◻️mutate-puzzle-2d-1/🦀️.rs`'s `KINDS`/`SPEC_VECTORS` mirror the Python's.

---

## 2. Commands run — verdicts

| # | command | verdict |
|---|---|---|
| 1 | `.venv/bin/python3 'TICKET/🐍️2F-region-oracle-probe.py'` | ✅ **exit 0** — 15/15 vectors agree with the Python second implementation (forward + committed after + full inverse law + D6 sentinel); `KINDS = 33`, `SPEC_VECTORS = 57`. Captured: `🗑️generated/2F/oracle-probe.txt` |
| 2 | same probe, half two (shapely) | ✅ `region-containment`: **33 checks over 15 vectors, zero disagreements** |
| 3 | `bun ./📜️script.ts publication-authority-audit Puzzle2dPlayApp` | ❌ exit 1 — **pre-existing, not 2F** (§3) |
| 4 | `bun 'TICKET/🟦️2F-publication-diff.ts' Puzzle2dPlayApp` | 46 ✅ / 1 ❌ — every route, lane, classification, retained-id, proof and contract clause green; the single ❌ is the pre-existing one. Captured: `🗑️generated/2F/publication-audit-clauses.txt` |
| 5 | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | ⚠️ **inconclusive** — `🗑️generated/2F/check-native-2.txt` stopped at 24 lines (`Checking semio-framework-ui-contract`), **0 errors so far**, still queued behind the fleet on the shared target-dir lock after the addendum's 5-minute poll budget. The crate never reached its own compilation unit (§4.1) |
| 6 | wasm32-wasip2 check, filtered `cargo test` | 🚧 **owed to integration** — per the 19:55 addendum these run only after a green native check |

Two reusable probes were left in the ticket folder, not in the scratchpad (which is wiped on every
process restart): `🐍️2F-region-oracle-probe.py` and `🟦️2F-publication-diff.ts`.

---

## 3. Not mine — findings handed to the coordinator

### 3.1 🚨 The build gate in the 15:15 addendum deadlocks the whole fleet

```
pgrep -f 'cargo (check|test).*semio-s-artifact-puzzle-[2]d'
```
also matches **the waiting shells themselves**: each gated command's own `zsh -c` argv contains both
the pgrep pattern and the literal `cargo check -p semio-s-artifact-puzzle-2d`. The `[2]` bracket only
stops `pgrep` from matching its own process, not the wrapper. Measured at 15:40: **six "builds"
counted, zero real cargo binaries** — every agent was waiting for every other agent. A gate that
counts only real cargo processes:

```sh
until [ "$(ps -eo comm,args | awk '$1 ~ /cargo$/' | grep -c 'semio-s-artifact-puzzle-2d')" -lt 3 ]; do sleep 20; done
```

### 3.2 `publication-authority-audit Puzzle2dPlayApp` is red at HEAD, before this slice

The audit requires the literal
`(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))` in
`ownerOracle`'s 2d branch (`📦️packages/🟦️typescript/📜️script.ts:288`). A sibling slice rewrote
`puzzle2d_generic_extent`'s tail to the `if addressed > … { return None } … saturating_add(connects)`
shape to carry `PUZZLE2D_PROXIMITY_GESTURE_MAX`. Verified pre-existing:
`git show HEAD:…/✏️editor/🦀️.rs | grep 'addressed > PUZZLE2D_SELECTION_BATCH_LIMIT'` hits.
**Owner: the slice that added `proximityConnect`** — either restore the shape or update the audit's
literal. `Puzzle3dPlayApp` and `Puzzle5dPlayApp` both pass.

### 3.3 Both third-party adapters discover zero vectors

`🕸️third-party-puzzle-2d-1/🐍️.py:78` and `🌐️third-party-puzzle-2d-1/🟦️.ts:42` still resolve scenarios
at `<vocabulary>/<leaf>/🧪️tests/<scenario>`, the pre-relocation layout; the quintets now live at
`<vocabulary>/<leaf>/<scenario>`, so `vectors()` raises "no committed vector was discovered". My
`region_containment` deliberately does **not** route through `vectors()` — it reads the real layout
itself, which is why it runs green today. Repairing the shared discovery would turn four currently
erroring handlers into four *failing* ones (the jsonschema handler already documents 24 payload
rejections), so it is a decision for the coordinator, not a side effect of this slice.

---

## 4. What is NOT verified

### 4.1 The crate was never compiled

`cargo check -p semio-s-artifact-puzzle-2d` was attempted five times across three sessions. It never
reached `semio-s-artifact-puzzle-2d`'s own compilation unit: two runs were killed by the usage-limit
outages, one by the gate deadlock of §3.1, one got as far as `Checking semio-s-artifact-stdio-binary`
with **0 errors in 109 lines** before the second outage, and the last (the addendum's single gated
run, `check-native-2.txt`) was still at `Checking semio-framework-ui-contract` — **0 errors, 24
lines** — when the 5-minute poll budget ran out. Between 13 and 18 concurrent checks of this one
crate were live throughout; the binding constraint is the shared cargo target-dir lock, not this
slice's code.

Both captured runs are clean as far as they got, which means the dependency graph beneath this crate
compiles with the schema changes in it — the snapshot/diff/mutation-union edits are already exercised
by `semio-framework-schema`'s and `semio-framework-os-kernel`'s own builds. What is unproven is
`semio-s-artifact-puzzle-2d`'s own 4,000-line editor and the seven mutation leaves.

**Every Rust file this slice wrote is therefore unproven by the compiler.** The Python second
implementation, the shapely oracle and the whole fixture corpus ARE proven (§2, rows 1–2) — they
carry the mutation semantics, so a compile error here will be a signature or import slip, not a
behavioural one. The most likely spots, in order: the two panel files (`UiFixedList`/`TreeItemBuilder`
builder signatures), `fill/🦀️.rs`'s `accept()` borrow of `self.target_regions` while `self.live` is
taken, and `Board2dHost/🟦️.tsx`'s `dispatch`/`readContainerSize` capture in the pointer `useEffect`
(both were added to its dependency array).

### 4.2 The board engine paints and hit-tests nothing

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` was
**not** touched. Regions are in the document, in the outliner, in the inspector, in fill and in the
brush's alt+click — but the Rust canvas does not draw them, does not hit-test them, and emits no
`regionCreate`/`regionMove`/`regionResize` events, so a region cannot yet be selected or dragged ON
the board. Owed:
1. `parse_fixture_json` ingests a `targetRegions` array into a fixed-capacity owner;
2. the draw path paints them beneath nodes;
3. the pick path hit-tests them **after** nodes/handles/edges;
4. `BoardEventKind` gains `RegionCreate`/`RegionMove`/`RegionResize`, and
   `🎮️commands/🎲️apply-board-events` folds them onto `relocateTargetRegion`/`addTargetRegion`;
5. `Board2dHost` forwards a `targetRegions` scene lane.

Steps 4–5 are small; 1–3 are inside the 12,600-line engine file that slices 2B and 2E also edit, and
the exploration agent mapping it was killed by the 16:45 outage. **This is the one deliverable of the
slice brief that is not done.** Until it lands, `relocateTargetRegion` has no caller — it is
reachable only by an explicit dispatch — and region selection depends on the outliner row.

### 4.3 The fill fixture states a law, not a verdict prefix

`🎞️fill-run.json`'s `targetRegion` vector declares four booleans and a reason id, not an exact
`verdictPrefix` like the five `cases` do. An exact prefix has to be read off a real run, and the
crate never compiled. The law it does state is strictly stronger than a prefix in one respect (it
compares a constrained run against its unconstrained twin), and weaker in another (it does not pin
the candidate order).

### 4.4 Measured, not assumed: the float canonical form

`serde_json`'s `Number::eq` compares `N::PosInt(20)` and `N::Float(20.0)` as **unequal**
(`serde_json-1.0.149/src/number.rs:37-46`, and `to_value(20.0f64)` yields `20.0` — measured with a
standalone probe). `dsl::json` spells a whole `f64` as `20`. The leaf tests compare snapshots through
`dsl::json` and payloads through `serde`, so this slice's fixtures use half-integers throughout,
where both agree. **Side finding:** the pre-existing 2d rejection leaf tests (e.g.
`🌱create-node/🧪️tests/🚫️rejects-a-capsule-id-the-tower-already-holds`) assert
`serde_json::to_value(snapshot) == committed` against boards carrying `"radius": 20`, `"x": 0`,
`"gap": 0`. By the above that assertion cannot hold. It is not in this slice's files and was not
touched — flagged for whoever takes the 2d test suite to green.

---

## 5. Hand-offs

- **Slice 2C (panels)** — I added two self-contained regions, `//#region 🎯️TargetRegionRows`
  (`📌️panels/🗿️artifact/🦀️.rs`) and `//#region 🎯️TargetRegionFields`
  (`📌️panels/🔍️inspection/🦀️.rs`), plus exactly two call-site lines: one
  `.window_section_or_placeholder(... TARGET_REGIONS_SECTION ...)` in `render` and one
  `if let Some(region) = …` arm in `selected_section`. Nothing else in either file was moved.
- **Slices 2B / 2E (board engine, `Board2dHost`)** — §4.2 is yours to finish or to hand back. My only
  edit to `Board2dHost/🟦️.tsx` is the exported `board2dAreaBrushCommits` predicate and the
  `onPointerDown` gate; the engine file is untouched.
- **Coordinator** — §3.1 (fleet deadlock), §3.2 (pre-existing audit red and its owner), §3.3 (stale
  third-party discovery), §4.4 (pre-existing rejection-test assertion).

### Battery ids (slice 2G)

| what | id |
|---|---|
| utility | `areaBrush` (overview pane utility bar) |
| verbs | `addTargetRegion`, `deleteTargetRegion`, `relocateTargetRegion`, `setTargetRegionFlag`, `setAreaBrushSize` |
| palette label | **Add Target Region** / **Zielbereich hinzufügen** |
| utility-option sliders | `puzzle2d-area-brush-w`, `puzzle2d-area-brush-h` (group `puzzle2d-play-utility-options-area-brush`) |
| outliner section | `puzzle2d-play-document.target-regions`, label **Target Regions** / **Zielbereiche** |
| inspector rows | `puzzle2d-play-inspector.target-region.{id,label,x,y,width,height,hidden,locked}` |
| interaction granularity | `targetRegion` (domain `vortex`) |
| document key | `targetRegions` (absent when empty) |
| fill trace reason | `outside-target-region`, verdict `warning` |
| gesture | arm `areaBrush`, then **alt + primary click** on the board |
