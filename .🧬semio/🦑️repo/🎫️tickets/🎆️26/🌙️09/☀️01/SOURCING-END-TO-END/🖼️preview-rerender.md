# Preview does not re-render after a row pick (defect A)

## Root cause
- A row click dispatches `interactionSelect`; `apply_interaction_verb` closes on `A::interaction_scope(..)` or else `interaction_declared_refresh_scope`, which derives the dirty window bodies from `WindowKindDefinition.interactions` (the schema-first declaration of which window kind reads which interaction domain).
- The sourcing manifest declared `rows` for pool, curated and grid only. The Preview window kind (`sourcing-preview`, body `sourcing.preview`) reads `interaction.selection("rows")` in `render_with_request_context` but declared nothing (`interactions: Vec::new()`), so the pick scope was `Partial { window_bodies: [pool, curated, grid] }`. The reactor was correct: it only renders what the scope names. Preview stayed on its last render until an unrelated edit dirtied it.
- Not a reactor (`dirty-poll-surfaces`, `dirty-surface-render`) defect: those render exactly the surfaces the scope names.

## Fix
- `✏️editor/📋️manifest/🦀️.rs`: `.window_kind_interactions(preview::SOURCING_CURATION_WINDOW_PREVIEW, vec![InteractionRef::new("rows")])`.
- The framework derivation is already generic (every window that declares the domain is dirtied); the declaration is the contract, so no reactor change.

## Law
`a_rows_pick_dirties_every_window_that_reads_the_rows_selection` in the curation `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: the declared refresh scope of `Select` on `rows` contains every window body of the app.

## Also
`🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` line 53 had one extra `)` (a peer's in-flight edit, broke the testing-laws crate); removed it.

Needs a wasm rebuild.
