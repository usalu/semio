# P4e Constructor, Spatial, Checkpoint, and Preview Implementation

## Outcome

Implemented the P4e packet in packet order without starting P5b. The mounted fill path now has one generation-owned cooperative preparation/reconfiguration path, the spatial owner and production broad phase are fixed and resumable with inner cursors, dormant whole-state checkpoint/clone/rebuild escapes are removed, and the bounded canonical diagnostic is transported and consumed independently of the candidate ghost. The accepted P4d R7/R8/R9-R11 registry, identity, admission, terminal, and retirement verifier predicates remain installed.

No Cargo, Nx, Wasm, browser, network, broad build, or runtime acceptance gate was run. This report does not claim runtime acceptance.

## Exact Source Files

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx`
- `📜️script.ts`

## Implementation

- Added `FillPreparationRoots`, `FillBuilder::begin_preparation`, and the ordered `PrepareFixture`, `PrepareCatalogs`, `PrepareMeshes`, `PrepareEntries`, `PrepareSpatial`, `PrepareLookup`, and `PrepareConfiguration` cooperative stages. Mounted rebuild, soft replan, and mesh refresh all enter `start_fill_preparation` with checked revision/generation identity. Brush target queue enumeration is cursorized.
- Replaced dynamic spatial buckets with `FixedOwnerMap<Cell, FixedOwnerSet<String>>`. Added fixed `CollisionCellSpan`, generation-bound `CollisionIndexOwner`, resumable replacement/removal/query cursors, mutation preflight, exact rejected-owner return, bounded page/truncation diagnostics, and inner containment/sampling part cursors. Production broad phase calls `begin_query`/`step_query`; accepted candidates and initial placement call `begin_replacement`/`step_replacement`.
- Removed `FillJobCheckpoint`, fixture fingerprint/restore/normalization paths, whole builder construction, direct configure/rebuild APIs, BTree conversion/clone helpers, rebuild-clear residuals, materialized covered-cell vectors, public direct spatial upsert/remove/query, accepted-prefix publication, and unbounded collision/broad-phase diagnostic vectors.
- Made `FillBuildPreview` the canonical bounded diagnostic with operation/revision/registry-generation/generation/sequence identity, stage/current pair/current target/current candidate, collision/sample state, an eight-entry candidate page, truncation, rejection, cursor/count fields, and optional ghost. Transport publishes the diagnostic before the optional ghost branch and rejects invalid/stale semantic identity.
- Added strict `WorldFillDiagnosticRecord` parsing, monotonic five-field identity rejection, ghost-independent `FillDiagnosticOverlay`, accessible diagnostic text/ARIA/data attributes, and conditional ghost rendering only when a valid diagnostic also owns a ghost.
- Extended the permanent interactivity verifier with faithful P4e source predicates and mutation self-tests while retaining the accepted P4d R7/R8/R9-R11 verifier sections.

## Fixtures

- `constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently`
- `stale_generation_stops_preparation_before_installing_any_entry`
- `spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population`
- `spatial_capacity_plus_one_refusal_preserves_exact_old_state`
- `spatial_stale_owner_cannot_finish_partial_replacement`
- `spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress`

## Permanent Verifier Mutations

- `whole-builder`
- `direct-configure`
- `direct-rebuild`
- `whole-checkpoint`
- `clone-helper`
- `dynamic-bucket`
- `materialized-coverage`
- `decorative-query`
- `direct-spatial-mutation`
- `producer-clone`
- `unbounded-diagnostic`
- `ghost-gated`
- `ignored-overlay`
- `stale-render`
- `ignored-truncation`
- `missing-constructor-cap-fixture`
- `missing-stale-preparation-fixture`
- `missing-sparse-query-fixture`
- `missing-capacity-fixture`
- `missing-stale-spatial-fixture`
- `missing-multicell-fixture`

## Scoped Gates

- `rustfmt --edition 2021 --check` on the six touched Rust files: clean.
- `git diff --check` on the seven touched source/verifier files: clean.
- Targeted forbidden-symbol census for whole checkpoint, fingerprint, BTree clone/conversion, rebuild-clear, covered-cell materialization, direct rebuild/configure, accepted-prefix, and unbounded diagnostic fields: no matches in the P4e Puzzle3d sources.
- `bun 📜️script.ts verify interactivity --self-test`: the Puzzle P4d/P4e baselines and all P4e mutations completed without a Puzzle failure. The overall audit then denied on the two unrelated DB findings below.

## Blockers and Limits

- Unrelated existing verifier findings in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`: DB input ownership lacks fixed-page construction/exact rejected-owner handback; DB I/O admission lacks the required caps.
- No Puzzle source/static blocker was found.
- Compile, runtime, Wasm, and browser acceptance remain intentionally unclaimed and must be performed by the designated acceptance lane.

## Independent-Audit B1/B2 Remediation

The independent P4e audit's B1/B2 findings were remediated after the initial implementation:

- Added typed `FixedOwnerVec<T>` page ownership with a fixed 32-slot/16-KiB backing contract, exact rejected-owner return, one-owner `pop`, and separate backing retirement.
- Replaced retained fixture object, attraction, and target-volume roots with `FixedFixtureOwner` fixed pages. Replaced all three retained catalog roots with `FixedCatalogOwner` fixed pages and replaced retained `kind_compatibility` with a fixed page. Meshes remain in `FixedOwnerMap<String, CollisionBody>`.
- Added `BrushCatalogView` and `BrushFixtureView` so search/preview functions consume the fixed roots directly without reconstructing ordinary unbounded fixture/catalog vectors. A `Fixture` snapshot is produced only at the existing explicit document projection boundary.
- Added one pre-mutation `preparation_capacity_refusal` covering fixture objects, fixture attractions, fixture target volumes, meshes, catalog objects, catalog vortices, catalog cables, kind compatibility, object weights, and vortex weights. The refusal retains its exact branch and omitted index; no preparation collection is mutated when it is present.
- Changed refusal ordering: the first current-owner grant publishes `preparation-capacity:<branch>` with operation, revision, registry generation, generation, and preview sequence while forcing `candidate_ghost = None`; only the following grant returns `fill-preparation-capacity`. Existing registry terminalization then makes `fill_progress().preview` absent, while the existing transport and renderer discard terminal/cancelled/replaced/stale state.
- Expanded `constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently` across all ten roots: fixture objects, attractions, target volumes, meshes, object/vortex/cable catalogs, compatibility, object weights, and vortex weights. Every cap case reaches search preparation by bounded turns; every plus-one case identifies the exact branch and omitted index, publishes before fault, and proves every destination remains empty. Weight refusals additionally assert the exact source key and `f64` owner (`object-weight-32`/`32.25` and `vortex-weight-32`/`32.5`).
- Added `capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_fault` and extended `all_fill_fixed_collections_store_max_entries_in_the_credited_page_and_return_plus_one` with fixed-vector page identity/exact-owner evidence.

### B1/B2 Verifier Mutations

- `missing-fixture-object-preflight`
- `missing-fixture-attraction-preflight`
- `missing-fixture-volume-preflight`
- `missing-mesh-preflight`
- `missing-catalog-object-preflight`
- `missing-catalog-vortex-preflight`
- `missing-catalog-cable-preflight`
- `missing-compatibility-preflight`
- `missing-object-weight-preflight`
- `missing-vortex-weight-preflight`
- `dynamic-fixture-object-owner`
- `dynamic-fixture-attraction-owner`
- `dynamic-fixture-volume-owner`
- `dynamic-catalog-object-owner`
- `dynamic-catalog-vortex-owner`
- `dynamic-catalog-cable-owner`
- `dynamic-compatibility-owner`
- `dynamic-mesh-owner`
- `missing-catalog-cap-acceptance`
- `fault-before-rejection-diagnostic`
- `omit-no-ghost-rejection`

The B1/B2 verifier set therefore contains 21 faithful mutations: ten missing-preflight mutations, eight dynamic-owner mutations, catalog-cap acceptance removal, diagnostic-before-fault inversion, and no-ghost transport omission.

### B1/B2 Scoped Gate Result

- Six-file Rust parse/format check: clean.
- Scoped source/verifier diff check: clean.
- Puzzle P4d/P4e baseline plus original and B1/B2 mutation self-tests: completed without a Puzzle failure.
- The overall interactivity verifier still denies only on the two unrelated P1q DB findings already listed above.
- No Cargo, Nx, Wasm, browser, network, broad build, or runtime gate was run; runtime acceptance remains unclaimed.
