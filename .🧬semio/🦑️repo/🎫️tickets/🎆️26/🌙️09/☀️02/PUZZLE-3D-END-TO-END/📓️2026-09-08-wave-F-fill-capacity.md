# Wave F — Puzzle 3d Fill Planner Document Capacity (2026-09-09)

Fixes the headline finding of `📓️2026-09-08-performance-architecture-audit.md` §2(a) / fix #0: the bulk
fill planner was capacity-gated at **32 total objects** because `FIXED_OWNER_SLOTS = 32` — a
bookkeeping/retirement-batch constant — was also the default capacity of every document-scale
`FillBuilder` owner. Opening the Fill tool on the Nakagin capsule tower (180 objects) refused before
any placement and then faulted the background job.

No `cargo check`/`build`/`test` was run (machine at load ~50, swap full — the coordinator runs the
consolidated check). Everything below is verified by reading, `grep`, `rustfmt --edition 2021`
parse checks, and the `bun ./📜️script.ts verify interactivity` static scan.

## Files changed

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` | Separated bookkeeping from document capacities (new `DOCUMENT_*` constants + docstrings); `FixedOwnerMap`/`FixedOwnerSet` gained `capacity()`; the spatial index's `entries`/`cells`/`oversized` and `CollisionQueryCursor::candidates` now declare document capacities; the three mid-job fullness checks compare against the container's own capacity; restored the two audited literals a peer sweep had drifted (`page: Option<Box<[Option<(K, V)>; N]>>` — the `FixedOwnerMapPage` alias is inlined again — and `std::mem::size_of::<…>()`) |
| `…/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs` | Added `spatial_entries_admit_document_scale_beyond_one_cell_member_bucket` and `document_scale_capacities_are_derived_from_the_fill_ceiling_not_the_bookkeeping_batch` |
| `…/⏳️precompute/🪣️fill/🦀️.rs` | Every document-scale owner declares its capacity; `preparation_capacity_refusal` preflights per branch against that branch's capacity; new `PreparationCapacityRefusal::diagnostic()` publishes `preparation-capacity:<branch>:<limit>`; census page gates raised to `DOCUMENT_OWNER_PAGE_BYTES`; `candidate_cache` documented as deliberately bookkeeping-scale (nothing writes it) |
| `…/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` | Hostile-roots test drives each branch at its own `cap`/`cap + 1`; refusal test uses `DOCUMENT_OBJECT_SLOTS + 1`; added `document_capacities_match_the_language_neutral_capacity_law`, `document_scale_fixed_pages_are_admitted_by_the_fill_envelope_reservation`, `nakagin_scale_fill_is_not_refused_and_places_at_least_one_object` |
| `…/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json` | Added the `documentCapacities` law block (schema/limits/locales untouched, minimal 14-line insertion) |
| `📜️script.ts` | Audit expectations updated to pin the new declarations (strictly more: branch→capacity mapping, document constants, the `candidates` owner) |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts` | Mutation literals updated; three new mutations (`document-object-slots-are-the-bookkeeping-batch`, `document-page-ceiling-is-one-bookkeeping-page`, `document-entries-narrowed-to-bookkeeping`) |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts` | 13 mutation literals re-pointed at the new declarations (verified none is a dead no-op) |

Not touched (sibling wave W-T owns them): `✏️editor/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

## The two capacities, separated

`FIXED_OWNER_SLOTS = 32` is **kept, name and value**, and is now documented for what it actually
bounds: the default page width for owners the memory census walks and the retirement cursor releases
one entry per close grant — a spatial cell's member bucket, the oversized-span set, the retiring
hand-off slots. It is a per-close-step retirement batch, and `FixedOwnerVec/Map/Set`'s default `N`.

Renaming it was considered and rejected: `📜️script.ts:8859` and two source-mutation self-tests pin
the literal `FIXED_OWNER_SLOTS: usize = 32`, the task also requires keeping audited literals, and the
name is accurate once the document capacities exist beside it. The audit now pins **both** sides.

New document-scale constants (all in `📐️geometry/🦀️.rs`, all with derivations in their docstrings):

| Constant | Value | Derivation |
|---|---|---|
| `DOCUMENT_OBJECT_SLOTS` | 2048 | existing objects + `FILL_COUNT_MAX` (1000); Nakagin worst case 180 + 1000 = 1180 → next power of two. Bounds `base.objects`, `placed_lookup`, `seed_object_ids`, spatial `entries`, broad-phase `candidates` |
| `DOCUMENT_ATTRACTION_SLOTS` | `= DOCUMENT_OBJECT_SLOTS` (2048) | one attraction per placement plus the scene's own (Nakagin: 358) |
| `DOCUMENT_VORTEX_SLOTS` | `2 × objects` (4096) | measured ≈2 vortices per object on Nakagin (358/180); the "max 10 per object" figure is object-local, not a document bound. Bounds `blocked_vortex_ids` (2 ids per attraction) |
| `DOCUMENT_KIND_SLOTS` | 256 | Nakagin: 12 object kinds, 18 vortex kinds, 14 compatibility rows → >10× headroom. Bounds catalogs, `kind_compatibility`, both weight maps, `meshes`, `oversized`, `target_volumes` |
| `DOCUMENT_VOLUME_SLOTS` | `= DOCUMENT_KIND_SLOTS` | fill regions are hand-declared, not per-object |
| `DOCUMENT_CANDIDATE_SLOTS` | `4 × kinds` (1024) | one target's candidate set = kinds × vortex templates per kind (Nakagin ≈120); drained before the next target. Bounds `candidate_seen`/`candidate_cross`/`candidate_same` |
| `DOCUMENT_CELL_SLOTS` | `4 × objects` (8192) | an 8.0-unit grid: one object's AABB straddles up to ~8 cells; ≈1440 cells for 180 Nakagin capsules, ≈9.4k would need 1180 badly spread objects. Member buckets stay at `FIXED_OWNER_SLOTS` (per-cell occupancy is genuinely small) |
| `DOCUMENT_OWNER_PAGE_BYTES` | `64 × FIXED_OWNER_PAGE_BYTES` (1 MiB) | per-page ceiling asserted by every container constructor, expressed in bookkeeping pages so the 16 KiB page unit stays the single authority. Deliberately loose (widest real page ≈432 KiB) because a too-tight ceiling would be a runtime `assert!` panic in wasm and cargo was not run here |

`oversized` (objects whose span exceeds `MAX_CELLS_PER_ENTRY = 4096` cells, i.e. >128 world units per
axis) and per-cell member buckets stay bookkeeping-scale by design — documented in the docstrings.

## Memory / size analysis

The containers stay **fixed-capacity, heap-paged and resumable-safe**; no `Vec`/`HashMap` was
introduced. The page is already `Box`ed (`page: Option<Box<[Option<(K, V)>; N]>>`), so `size_of::<
FillBuilder>()` is unchanged by widening `N` (pointer + len per owner) — this matters because the
census credits `size_of::<FillBuilder>()` as one unit and the retirement cursor releases exactly one
page per close grant. Nothing is `Copy`, nothing is stack-allocated, so there is no stack risk at 2048
slots.

Checkpointing is unaffected: `fill_checkpoint` is the 56-byte `FILL_ENVELOPE_TOKEN_BYTES` token
indexing one of `FILL_ENVELOPE_MAX_OPERATIONS = 4` registry slots, and `restore_persisted_fill`
decodes that token and looks the slot up — the builder is never copied or serialized per step.

Estimated page bytes at admission (all pages are allocated by `begin_preparation`; struct sizes read
off the schema by hand, ±10%):

- `base.objects` 2048 × ≈216 B ≈ 432 KiB, `base.attractions` 2048 × ≈136 B ≈ 272 KiB, `target_volumes` 256 × ≈120 B ≈ 30 KiB
- catalogs 256 × (104 + 200 + 48) B ≈ 88 KiB, `kind_compatibility` 256 × ≈80 B = 20 KiB, `meshes` 256 × ≈72 B = 18 KiB, weights 2 × 8 KiB
- `placed_lookup` 64 KiB, `seed_object_ids` 48 KiB, `blocked_vortex_ids` 96 KiB, `candidate_seen` 24 KiB, `candidate_cross`/`candidate_same` 2 × 56 KiB
- spatial `entries` 96 KiB, `cells` 256 KiB, `oversized` 6 KiB

Total ≈ **1.6 MiB**, inside `FILL_ENVELOPE_MAX_BYTES` (4 MiB) and far inside
`FILL_ENVELOPE_MAX_ITEMS` (65 536 — ~20 pages are credited). The census walks the same number of
steps as before at admission (every collection is empty on a fresh builder), so widening the pages
does not lengthen admission. Peak *runtime* memory adds the lazily allocated cell buckets: worst case
8192 cells × 768 B ≈ 6 MiB, only reachable with ~2048 placed objects spread over the whole grid; the
new geometry test bounds that product at 8 MiB. `FILL_ENVELOPE_PROCESS_BYTES` (16 MiB, 4 slots) is
unchanged and remains the admission authority — growth after admission was already uncredited.

Two capacity gates upstream are unaffected: `FILL_WORKER_MAX_MESHES = 64` ≤ `DOCUMENT_KIND_SLOTS`
(so the mesh branch can no longer refuse anything the engine accepted), and
`PUZZLE_COMMAND_WORK_ITEMS = 4096` in the shared retained-command contract is untouched.

## Refusal path

Kept, and made honest instead of removed: `preparation_capacity_refusal` now checks each branch
against that branch's own capacity and the published preview reads
`preparation-capacity:<branch>:<limit>` (e.g. `preparation-capacity:fixture-objects:2048`). The
mid-job re-checks (`AcceptPhase::InstallLookup`, `CollisionSpatialIndex::step_replacement`) now test
`len() == capacity()` on the actual container, so a field's declared capacity can never drift from
the check that enforces it.

## Tests

Changed:
- `constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently` — per-branch `cap`/`cap + 1` (was `FIXED_OWNER_SLOTS`/+1), turn bound raised to `16 × DOCUMENT_OBJECT_SLOTS`, weight ids zero-padded to 4 digits so the `BTreeMap`'s sorted order is still the numeric order at index 256, expected diagnostic now carries `:{cap}`.
- `capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_fault` — refuses at `DOCUMENT_OBJECT_SLOTS + 1` objects.
- `retained_owner_census_advances_one_fixed_unit_…` / `retained_owner_census_credits_each_actual_fixed_slot_page_…` — per-grant page assertions now use `DOCUMENT_OWNER_PAGE_BYTES`.
- `spatial_capacity_plus_one_refusal_preserves_exact_old_state` — unchanged (name is pinned by the audit); it is in fact the per-cell member-bucket bound, since all its entries share one cell.
- The generic container boundary tests still exercise the **default** `N = 32` instantiation, which is what `FIXED_OWNER_SLOTS` now means.

Added:
- `nakagin_scale_fill_is_not_refused_and_places_at_least_one_object` (fill) — 180 objects × 3 vortices (540, two free + one attraction-connected), 360 attractions, 12 object kinds, 18 vortex kinds, 14 compatibility rows, one registered mesh; asserts no refusal at `begin_preparation`, no `Fault` and no `preparation-capacity` preview for any step, all 180 objects + 360 attractions + catalogs installed, and `sequence`/`appended_objects` reaching 1 placement (`placed_lookup` = 181). Synthetic rather than the real `📚️examples/🏗️nakagin-capsule-tower` DSL: the precompute tests have no DSL→`SceneConfig` helper (`grep nakagin` in the precompute tree returns nothing), and parsing the 129 KB DSL inside a unit test would couple the planner test to the snapshot text layer.
- `document_capacities_match_the_language_neutral_capacity_law` (fill) — binds every constant to the new `documentCapacities` block of the shared fixture JSON and re-derives the Nakagin headroom from it.
- `document_scale_fixed_pages_are_admitted_by_the_fill_envelope_reservation` (fill) — every declared page ≤ `DOCUMENT_OWNER_PAGE_BYTES`, and the whole census of a fresh builder completes (never `Rejected`) under the real `FILL_ENVELOPE_MAX_ITEMS`/`FILL_ENVELOPE_MAX_BYTES`. This is the test that would catch a wrong hand-computed struct size.
- `spatial_entries_admit_document_scale_beyond_one_cell_member_bucket` (geometry) — 128 entries in distinct cells are all admitted (32 would have refused before), with the page ceiling re-checked.
- `document_scale_capacities_are_derived_from_the_fill_ceiling_not_the_bookkeeping_batch` (geometry) — the derivations (objects ≥ `FILL_COUNT_MAX` + 1024, vortex/attraction/cell/candidate ratios), every concrete document page ≤ ceiling, and the fully occupied cells+buckets product ≤ 8 MiB.

The fixture JSON stays the shared oracle input: `schema`, `limits`, `boundaryLaws`, `preview`,
`color`, `locales` are byte-identical (14 inserted lines only), and
`interactivityPuzzleFillPreviewJsonSelfTests` still passes.

## Audit output — `bun ./📜️script.ts verify interactivity`

`interactivityPuzzleFillEnvelopeSelfTests`: **PASS**. It was **failing before this wave** (baseline
falsely rejected) because a peer sweep had extracted `type FixedOwnerMapPage` and unqualified
`size_of`, breaking two pinned literals; both are restored.

`interactivityPuzzleFillPreviewJsonSelfTests`: **FAIL, not mine** — mutation `puzzle3d-locale-default`
replaces `"None"` in `✏️editor/🗣️terminology/🦀️.rs`, which contains zero occurrences of `None`
(`grep -c None` → 0). No file I touched is involved.

`interactivityPuzzleFillP4eSelfTests`: **FAIL with exactly two findings, both pre-existing**, with
evidence:
1. *"P4e spatial owner is not fixed, resumable, generation-bound…"* — the check requires
   `pub(crate) struct CollisionIndexRemoval` in the **production** source, but it (and
   `begin_removal`/`step_removal`) are `#[cfg(test)]`-gated. `git show HEAD:…/📐️geometry/🦀️.rs`
   shows the same `#[cfg(test)]` attribute, so the clause failed before this wave. Un-gating it
   would add production API with no production caller (dead-code warnings) — a design call for the
   spatial owner, out of this wave's scope.
2. *"P4e preview publication is not the canonical bounded diagnostic page"* — the four `schema.*`
   clauses read `INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE =
   …/🧬️schema/🦀️component.rs`, which exists neither in the worktree nor in `HEAD`
   (`git show HEAD:…🦀️component.rs` → *does not exist*); the declarations it looks for live in
   `…/🧬️schema/🦀️.rs`. The audit path has always been blind here.

My own regression in that same function (the hostile-branch tuples gained a third element) was found
and fixed. Every mutation literal in both self-test files was machine-checked for deadness
(`🔍️probe-fill-audit-mutations.ts`, kept beside this report): only the pre-existing
`unbounded-diagnostic` mutation on the missing schema file is dead.

`rustfmt --edition 2021 --emit stdout` exits 0 on all four edited `.rs` files; the fixture JSON parses.

## NOT verified

- **`cargo check`/`build`/`test` were not run** (explicit wave constraint). Nothing here is
  compile-verified: const-generic argument counts, the `capacity()` const fns, the generic
  `fill_fixed_vec_backing_credit<T, const N: usize>`, the new test modules' imports
  (`FIXED_OWNER_SLOTS`/`FIXED_OWNER_PAGE_BYTES` moved to an explicit geometry import in the fill
  test, plus `FILL_ENVELOPE_MAX_ITEMS`/`FILL_ENVELOPE_MAX_BYTES` reached as ancestor-private consts)
  are read-verified only.
- **No test was executed**, so "places at least one object on a Nakagin-scale document" is proven by
  construction (the fixture mirrors the passing `adversarial_broad_phase_fill_…` geometry exactly:
  same tetrahedron body, same `port` vortex kind on host and template, same `overlap_budget = 0.0`)
  but not observed. Likewise the 400 000-turn bound and the ~1.6 MiB census total are estimates.
- **Struct sizes are hand-computed** from the schema (`FixtureObject` ≈216 B, `AttractionProps`
  ≈136 B, `DslValue` 32 B …). `DOCUMENT_OWNER_PAGE_BYTES` is deliberately 2–3× the widest estimate so
  a mis-estimate cannot turn into a constructor `assert!` panic; the added tests re-measure with
  `page_bytes()` at runtime.
- **No runtime/UI confirmation**: the Fill tool was not opened on the Nakagin example in a running
  app, and no console logs were captured.
- `verify interactivity` still exits 1 — for the three pre-existing findings above plus the many
  unrelated pre-existing failures the command reports (launch.json gate registrations, writer
  descriptor discovery, "19133 descriptors exceed fixed capacity 256"), none of which this wave
  touched.
- The `FixedOwnerMap` lookup path (`index_of`) is still a **linear scan** even though the page is
  kept sorted; at 2048–8192 slots a binary search would be O(log N). Deliberately not changed here:
  it is a behavioural change that relies on the sortedness invariant and cannot be validated without
  running the suite. Recommended as a follow-up together with fix #1 of the performance audit.
