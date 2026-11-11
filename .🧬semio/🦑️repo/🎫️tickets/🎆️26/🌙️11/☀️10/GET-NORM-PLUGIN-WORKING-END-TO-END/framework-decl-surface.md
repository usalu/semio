# decl-surface, snapshot-owners and composition-emission migration

Verified: `repo cargo ... check -p semio-plugin-composition-emission -p semio-plugin-decl-surface -p semio-framework-test-original-snapshot-owners --all-targets` from the repo root: exit 0, 0 errors (log `🗑️generated/decl-surface-root-4.log`). The en1990 workspace only checks libs meaningfully for these crates: they are non-member path deps there, so their dev-dependencies are not resolved and lib-test targets fail with unresolved dev-dep crates. Verify tests from the root.

## semio-plugin-decl-surface
- `🌱️original`: `EditorApp`/`ViewerApp` use `RetireOwned` with `RetirementCursor` assoc type and `retirement_seed` (String cursor); `retired` helper is effect-shaped (`close_original_retained_effect_step`); controller close uses `effect.grant()`.
- `✏️artifact-editor-trait`: `DraftMutation` also requires `OpBinaryReceiving`.
- `📥️natural-file` (mounted): `close_owned_turn` takes the effect; owners admitted via `admit_owned_retirement(&mut Option<T>, effect)` through `admit_retirement`; ticket close via `close_original_effect_step` + `close_factory_ticket(slot, body, effect)`.
- `🏗️surface-builders`: `SnapshotBuilder` bounds relaxed to the cold requirements it uses (`S: Default+Clone+PartialEq+ArtifactDsl+ArtifactPack`, `M: ColdMutation<S>`); the cold builder test fixture has no native mutation facet.

## semio-framework-test-original-snapshot-owners
Closes recipients through an `EffectBank` (`bank_run`/`driven`), new dependency `semio-framework-value-testing-admission`; progress is recorded into the snapshot owner wallets.

## semio-plugin-composition-emission
- `🪪️metadata`: `PrivateMetadataStringIssuer` split into `advance` (grant, copy) and `advance_leased` (effect, shared-actor clone via `SharedUtf8::admit_clone`); `PrivatePublicationMetadataIssuer::advance` takes the effect; child issuer stays grant-shaped.
- Group owner (`📬️publication/🤝️group/📦️owner`): `advance_inputs` takes the effect instead of the grant. Callers elsewhere (e.g. test `🧬️mutation-fixtures-surface/🧵️retained-work-and-child-emission-laws`) need `effect` and were not verified.
- Unit tests migrated with a `funded_advance` helper (EffectBank) and the 3-arg `SharedUtf8::admit`; new dev-dependency `semio-framework-value-testing-admission`.

## Leftover: surface-fixture test (crate `semio-plugin-surface-fixture`)
- Migrated `🧪️tests/🧬️mutation-fixtures-surface/🧵️retained-work-and-child-emission-laws/🦀️.rs`: both `group.advance_inputs(.., grant)` calls now run through a local `funded_group_input` helper (EffectBank, observed heap) and pass the effect; added dependency `semio-framework-value-testing-admission` to `🧪️tests/🪟️surface-fixture/📦️packages/🦀️rust/Cargo.toml`.
- NOT verified: `check -p semio-plugin-surface-fixture --all-targets` (root, log `🗑️generated/decl-surface-fixture-1.log`) stops at its dependency `semio-plugin-document-media` (lib, 61 errors, same API migration: `close_step(grant)` to effect, `artifact_retirement_box_close_step` 3-arg, `NativeSnapshotFailure::into_value_error`, `.result`/`.work` on polls in `📨️envelope-decode-and-ingress`, `🧰️store-replacement-initializer-and-retirement`, `🗄️document-archive-load`). Outside my crates; the fixture test cannot compile or run until it is migrated. No fixture-crate errors were reported beyond that dependency.
