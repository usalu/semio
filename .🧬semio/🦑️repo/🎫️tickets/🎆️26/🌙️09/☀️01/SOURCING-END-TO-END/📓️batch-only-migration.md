# 🧵️ Migrating the eight `BatchOnlyPendingRewrite` commands

## Why this is end-to-end work, not test cleanup
`validate_ui_dispatch_classification`
(🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11806) admits a UI dispatch only when the
command's registry classification is `InteractiveJobClassification::Migrated`; anything else is
`interactive-job.not-ui-safe`. `qualified_tool_proof` (same file, :18864) additionally refuses any typed
command that owns no tool proof — `interactive-job.missing-factory`.

Sourcing declared **six** migrated tools and listed **eight** more in
`SOURCING_CURATE_BATCH_ONLY_TOOL_IDS`. Those eight are the entire curation vocabulary
(`curateAdd`, `curateSetCount`, `curateRemove`, `dropOnPool`, `dropOnCurated`), the module filter
(`setFilterModule`), and both whole-document replacements (`setDocument`, `stockFromCatalogue`).
None of them could be invoked from the browser. Sourcing's core interaction — putting stock into the
curation and dragging between the two panes — was dead at runtime, not just in tests.

## What the migration required
### 1. A document-lane one-item preparation factory (the actual blocker)
`unsupported_publication_contracts` (:19019) rejects the `Artifact` lane outright unless the app
implements `build_artifact_store_one_item_preparation_factory`. Sourcing had only the config-lane one.

Added `SourcingCurateArtifactPreparationFactory` / `SourcingCurateArtifactPreparation`, modelled on
the existing `SourcingCurateConfigPreparationFactory` and on the same trait pair every peer with a
document lane implements (`🗒️note`, `📐️cad`, `🌊️flow`, `🌍️gis`). It:
- bounds the base by curated-row + `stock_extra` count and by each object id's length
  (`sourcing_curate_document_bytes`), rejecting rather than truncating;
- gives every mutation a one-row footprint (`sourcing_curate_mutation_footprint`) — all three
  semantic mutations address exactly one curated row;
- computes the post-state through the mutation's OWN `Mutation::diff` + `MutationDiff::apply` and its
  own `Mutation::inverse`, so the retained lane and the batch lane cannot diverge;
- treats an `Error`/`Fatal` outcome as a refusal. `MutationOutcome::error`/`fatal` force an EMPTY
  diff, so publishing anyway would write a no-op edit into history.

### 2. `SetFilterModules` on the config lane
`sourcing_curate_config_mutation_footprint` explicitly rejected `SetFilterModules` as "non-retained",
which is what kept `setFilterModule` off the lane. Given a footprint (one work item per module id,
bounded by the same item envelope as the typology path) and an inverse in
`prepare_sourcing_curate_config`, it is retained like every other filter.

### 3. `HostOnly` for the two whole-document replacements
`setDocument` and `stockFromCatalogue` emit no mutation at all — they return
`Emit { effects: vec![reset_document_effect(..)] }`, exactly like the already-migrated
`setActiveExample`. They publish on `HostOnly`, which needs no store lane.

### 4. Declarations
`SOURCING_CURATE_BOUNDED_TOOL_IDS` now lists all fourteen UI-reachable commands (`setLocale` stays
off it — it is `ForbiddenFromUi` and reaches the app only through the host-configuration route);
`bounded_first_step_tool_proofs!` carries a row per id; `PUBLICATION_CONTRACTS` names each id's lane;
every `.action_interactive_job(...)` is `Migrated`. `SOURCING_CURATE_BATCH_ONLY_TOOL_IDS` is deleted
rather than emptied — a "pending rewrite" list with nothing in it is exactly the kind of legacy
scaffold this repo forbids.
