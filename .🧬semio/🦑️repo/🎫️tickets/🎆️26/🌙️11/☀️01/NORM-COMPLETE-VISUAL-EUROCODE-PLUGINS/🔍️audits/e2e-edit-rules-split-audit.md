# Edit Rules Package Split Audit

Read-only current source review. No build/test launched; parent session3873 owns execution.

Base: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`.

## Ownership And Manifest Wiring

`✏️edit-rules/🦀️.rs` owns NormEdit, NormEditRules, selector/payload resolution, can_set_field and move intents. Its package depends on kernel, value and foundation; no framework/plugin command/UI dependency is declared. Foundation has no reverse live-core/edit-rules dependency.

`✏️edit-rules/🎮️commands/🦀️.rs` owns command/remedy/coercion/typed staging and imports its live-core parent. `🧲️live-core/🦀️.rs` mounts this file as edit_commands and publicly exports it; live-core depends on the new edit-rules package and publicly exports that package too.

Both manifests resolve workspace="../../../../../.." to existing norm/Cargo.toml. The workspace lists the new edit-rules member and workspace dependency path. The root app-surface publicly reexports live-core; NormEditRules::can_set_field therefore remains available through the existing surface.

A source definition census beneath app-surface found exactly one definition each of NormEdit, NormEditRules, can_set_field, resolve_norm_edits and resolve_set_fields. No duplicate implementation was found.

## Test Includes

Pure crate owns two neutral tests: inspector_editability_follows_the_declared_typed_boundaries and nested_set_fields_use_the_nearest_typed_boundary.

Commands owns sibling_writes_replay_against_the_preceding_typed_parent and typed_bundle_refuses_errors_and_fatals_but_preserves_warning_operations, with local 🧫️model module.

Both pure include_str paths and the relocated command include_str path resolve to existing norm/🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json. The relocated commands path uses one additional parent traversal, matching its deeper owner directory.

## Typed Staging Retirement

resolve_norm_edits owns document.clone() in ColdReplayProjection. Every resolver/index/admission error passes through its result closure; the outer Err branch retires the accumulated Vec<M> via RetireOwned before returning. Previously accepted mutation owners and later unexecuted mutations in the same returned vector are included.

Each fold_operation_into result's message vector is explicitly retired before any refusal return. Error/Fatal admission or apply failure returns false; warnings do not refuse. The scratch guard retires its current projection on success and on every early error via Drop. No into_inner transfers its retirement.

Empty edits return before cloning/owning scratch. validate_writes errors occur before scratch/mutations exist. Resolver-internal temporary ownership before it returns Err remains the resolver's own responsibility, outside this staging function.

Current leaf coercion materializes the staged current P for each write. It still falls back to the supplied value if get_value_at_path fails; the subsequent canonical resolver remains authoritative and can refuse missing paths. This behavior is unchanged by the package split and is not claimed as independent missing-path validation.

No concrete source ownership, include relocation, export or staging-retirement defect was established by this bounded audit. Compilation and execution remain unverified here.

## Pure Binding Lookup Implementation

Authored neutral fixture/schema/feature and Bun schema oracle at norm/🧪️tests/🧭️edit-bindings before appending the native binding_lookups_preserve_structural_capabilities_and_concrete_boundaries test and implementing public lookups.

Only pure edit-rules source and its tests were changed. Added SetterBinding with borrowed rule plus owned concrete boundary/relative strings, insertion_binding, can_insert_item, can_remove_item and setter_binding. Parsing and segment serialization reuse owned path helpers. Selector normalization uses segment variants, not string stripping. Removal strips only one terminal row selector and checks exact own removal or explicit canonical identity-removal paths. Setter selection scans parsed prefixes longest first, excluding clear-only rules. can_set_field remains unchanged.

The12 committed vectors cover exact/nearest setter precedence, original id and index selectors, nested relative selectors, explicit identity and positional removal, independent insertion, unsupported structural operations under a valid setter, malformed paths, clear-only setters and textual-prefix collisions. Existing facade glob exports expose the new public type/methods without a new dependency.

No tests/builds launched by this worker; parent owns actual red/green evidence. The two EN1998 insertion item strings remain untouched pending actual red. New retained paths remain below256 characters.
