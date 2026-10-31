# 1029 Audit: hookreview (`admit_document_store_publication_authority`)

Scope: read-only review of the no-op hook added by executor hostinit (see `1029-result-hostinit.md`). No cargo, no edits.

## Verdict
**Shim-like, not dead, not a clean extension point.** The hook is load-bearing today (it is the only production producer of the publication lease that the three `REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY` editors need), but it exists only to shuttle a lease into a process-global one-slot table that duplicates a check the framework already performs. It also has a silent-error path and a replay-producer gap. Recommended clean replacement is below.

## Definitions and callers
- Trait definition, no-op default: `🧰️framework/.../📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs:255-256`.
- Only caller: the same trait's default `build_document_store_initialization_job`, `…artifact-editor-trait/🦀️.rs:251`, which runs unconditionally before `bounded_document_store_initialization_job` (line 252).
- Consumer of the lease: `validate_document_store_publication` default at `…artifact-editor-trait/🦀️.rs:258-259` (Err when the const is true and no override exists).
- Framework call site of the gate: `🧰️framework/.../🔌️plugin/🪜️maintenance-ladder/🦀️.rs:484` (`A::validate_document_store_publication`), after the generation guard at `:468` (`active.generation != live_generation`) and the complete-candidate check at `:464-467`.
- Const flag: `…artifact-editor-trait/🦀️.rs:33` (default false); process3d true at `✏️s/🔌️plugins/🏭️process/.../✏️editor/🪟️editor-impl/🦀️.rs:120`; gen2d true at `…generation2d/.../✏️editor/🕹️play-app-editor/🦀️.rs:65`; gen3d true at `…generation3d/.../✏️editor/🪟️artifact-editor-impl/🦀️.rs:40`.

## Implementors that override it
| Domain | Override (admit) | Validate/release | Host admit fn |
|---|---|---|---|
| process3d | `…process3d/.../✏️editor/🪟️editor-impl/🦀️.rs:127-129` (`let _ = …`) | `:134-137` | `…process3d/.../📬️publication-authority/🦀️.rs:143-170` |
| gen2d | `…generation2d/.../🕹️play-app-editor/🦀️.rs:76-79` (`let _ = …`) | `:81-85` | `…generation2d/.../🪪️publication-leases/🦀️.rs:125` |
| gen3d | `…generation3d/.../🪟️artifact-editor-impl/🦀️.rs:59-62` (`let _ = …`) | `:64-68` | `…generation3d/.../🏠️host/🦀️.rs:228` |

What they do: each admit writes one `Some((key, lease))` into a process-global `Mutex<Option<…>>` (process3d `…publication-authority/🦀️.rs:116-119` and `:143-170`), with every field set to the generation (base = parent = live = generation). Validate then checks the lease fields against the same generation values and releases the slot. Nothing else in production calls admit or release (rg over the repo: only these three overrides and the trait call site).

## Is the default initializer already providing this authority?
No, and the framework already covers most of what the lease checks:
- The framework default initializer (`…♻️retirement/🦀️.rs:7-10`) is only a replay wrapper; it issues no publication token.
- The ladder already refuses on generation drift (`maintenance-ladder/🦀️.rs:468`) and on an incomplete candidate (`:464-467`) before it reaches validate (`:484`).
- The host lease comparisons (`process3d …publication-authority/🦀️.rs:270`) are tautological in production: admit sets `base_revision = parent_revision = live_revision = generation`, so the only non-trivial case is a `refresh` (`:247-262`), which nothing calls in production.
- So the only thing the lease adds in production is a fail-closed "slot must still be present" check, and that is exactly what the silent-error path weakens (below).

Conclusion: the hook is not dead (REQUIRES=true domains fail closed without it), but it is redundant with framework checks and only serves as an out-of-band token store.

## Defects found while reviewing
1. Silent error: all three overrides discard admit errors with `let _ =` (`editor-impl/🦀️.rs:128`, `play-app-editor/🦀️.rs:78`, `artifact-editor-impl/🦀️.rs:61`). On contention or a duplicate, the initializer proceeds and the failure surfaces only at publication as `…publication-authority.absent`/refusal, after the whole replay has run.
2. Single-slot app lease: `process3d_app_publication_lease()` holds one `Option` (`publication-authority/🦀️.rs:116-119`). Admit checks duplicates only for the same operation (`:146-151`); a second operation silently overwrites a first one that was admitted but never validated (an initialization cancelled before publication leaks its lease; no release path exists for cancellation). gen2d and gen3d use the same shape.
3. Lease lifetime: admit runs when the job is created (`artifact-editor-trait/🦀️.rs:251`), not when publication is possible, so cancellation or refusal leaks the lease.
4. Stale doc: `…process3d/.../🌉️wasm/🧪️tests/🔬️unit/🦀️.rs:29` refers to `Process3dPlayApp::build_document_store_owners`, which does not exist (rg finds no override in process3d).

## Replay producer risk (hostinit "replay producer missing")
Confirmed, and wider than the hostinit note says.
- Native replay of a history-bearing document requires `owners.replay_preparation`; without it it refuses with `UnsupportedOwner "initialization requires its original installed native semantic producer"` at `🧰️framework/.../🏪️store/🏗️initialization/🔁️replay/🧬️native/🦀️.rs:66`.
- Default catalog: `ArtifactEditor::build_document_store_owners` (`artifact-editor-trait/🦀️.rs:185`) calls `bounded_artifact_store_owners` (`…🏪️store/🏗️initialization/🏠️owners/🦀️.rs:716-724`), which uses `admit_source_constructor` and installs `one_item_preparation` and `replay_preparation` as `None` (`owners/🦀️.rs:303`).
- process3d, gen2d and gen3d editors do not override `build_document_store_owners` (rg), so they inherit that catalog: no replay producer.
- Raster editor overrides the catalog with `raster_document_store_owners_admission` (`…raster/.../🚀️store-initialization/🦀️.rs:9-10`), which also uses `admit_source_constructor` and installs no replay. The raster demo-archive law (`a_demo_edit_archive_loads_back_through_the_document_archive_door`) is therefore expected to fail.
- The only production catalogs that install a replay producer are `bounded_artifact_store_owners_with_semantic_preparations` / `…_with_shared_semantic_preparation` (`owners/🦀️.rs:736-765`), used by the window config owner contract (`🪟️window/🎚️config/📐️owner-contract-and-mutation/🦀️.rs:44`) and the IDML issuer (`✏️s/🔌️plugins/🗄️stdio/.../📚️idml/🏭️issuer/🦀️.rs:11`).

## Recommended clean replacement
Delete the hook and the per-domain lease plumbing for default-initializer domains:
1. Let the framework mint the publication token in `bounded_document_store_initialization_job` (or in the replay initializer's completion), keyed by `(operation, generation)`, and pass it through the ladder to `validate_document_store_publication`. Remove `admit_document_store_publication_authority` from the trait and from the three overrides.
2. Delete the three `admit`/`release` host functions and their process-global lease tables, unless the peer path that `REQUIRES_…` guards against is shown to need them (not verified here).
3. Give process3d, gen2d, gen3d (and raster) a real replay producer in their owner catalog (`with_semantic_preparations`-style), or fail closed at catalog admission rather than at first history replay.

Not verified: the exact "unmanaged peer path" that the REQUIRES flag guards; no cargo run; no test run.
