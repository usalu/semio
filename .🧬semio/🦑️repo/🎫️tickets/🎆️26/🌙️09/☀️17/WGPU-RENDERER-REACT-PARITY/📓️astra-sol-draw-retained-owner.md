# Draw Retained Owner Repetition

## Law

The regression uses the registered `DrawingInstanceOperationOwner` through `context::drawing_app()` and `settled()`. It mounts the production `drawing-composite` window, arms the per-window `shapeRect` utility, and sends Draw10's exact idle Move/Down/Move/Up coordinates for rectangle A. After its publication it constructs a fresh `ViewModel`, rearms `shapeRect`, renders the post-publication composite, and sends the distinct non-overlapping sequence for rectangle B.

The law requires one layer after A, two layers after B, two Direct Select reset effects, rectangle geometry in both added layers, distinct layer identities, and the exact utility, window, session, completion operation, revision, lanes, and completion count in every causal failure message. A third fresh-view gesture then repeats B's exact geometry. It must publish one more distinct layer and reset again, proving that creation identity does not collapse two deliberate user creations merely because their visual content is equal.

## Fail-first receipts

`draw-native-1.log` selected the new law and failed before gesture admission with `interactive-job.live-instance`: the test had constructed a real app but had not bound `ActionMeta.instance_id`. The harness now calls `bind_instance_id` before its initial production render. This was a setup RED and did not justify a product change.

`draw-native-2.log` then reached the intended defect: 0 passed, 1 failed, 276 filtered. Rectangle A completed as operation 256 at revision 2258456641583784106 with lanes `Artifact/Effect/Ui/Terminal`; rectangle B completed as operation 512 at revision 2892813045267397835 with the same lanes. Both observations carried `shapeRect`, window `drawing-composite`, session `draw-repeat-owner`, and one completion. The final document contained two total layers where the fixture's one base layer plus two rectangles required three.

The first panic occurred at the final +2 layer assertion. A secondary Drop assertion named one still-live operation owner because the old assertion preceded fixture closure; the law now captures its observations, closes the registered app, and then asserts.

## Causal repair

`commit_shape_drag` created every Rectangle from `default_layer_base("Rectangle")`. That constructor hashes only the display name, so both non-overlapping drags emitted the same layer id. `CreateLayer::diff` correctly rejects the second node with `mutation.duplicate-id`, explaining why the second operation could complete and publish a new revision while the projection retained only the first rectangle.

Both structure feature files say that re-creating an existing node with the same id is a real collision; they do not say that two user creation events at equal geometry are one node. The accepting duplicate-layer design likewise changes identity to create a copy. Framework `AppOperationContext` is documented as the durable authority captured once at public command admission and copied into every continuation. A geometry-only hash would therefore replace the proven name collision with an identical-geometry collision.

The repair derives the shape id from a fixed kind discriminator, the complete durable `AppOperationContext` (`app_instance_id`, length-framed `parent_document_id`, `operation_id`, `generation`, and `canonical_base_revision`), the document-local root-layer ordinal, and the exact four persisted geometry lanes used by Rectangle, Ellipse, or Line. Integer lanes and floating-point bit patterns use fixed big-endian bytes and feed the existing content-addressed `create_drawing_id("shape", ...)`. Retrying the same admitted operation against the same document state derives the same id. A different app instance cannot collide merely because its runtime-local operation number and geometry agree, and a second creation within one document cannot collide at identical geometry. No independent counter, runtime entropy, legacy action, or fake input argument is introduced.

The pure identity law pins all three properties: identical full context plus ordinal is stable, changing only `app_instance_id` changes the id, and changing only the document-local ordinal changes the id for identical geometry.

## Validation

The root-owned full Draw run `draw-native-4.log` executed 278 tests: 215 passed, 63 failed, and none skipped. Both packet laws — the real repeated-shape retained-owner scenario and `shape_identity_is_replay_stable_and_scoped_to_the_durable_app_operation` — are absent from the failure list and are therefore among the 215 passes.

The 63 failures are separated from this repair by their assertions:

- 25 component tests use unbound app fixtures (`interactive-job.live-instance`) or drop stores without their terminal-empty witness. The one-shape law is in this group; the new real-owner law binds the mounted instance and closes it.
- 3 binary/text store tests and 8 retained mutation-authority tests fail on retained aggregate-capacity or exact-owner retirement paths.
- 1 kinds-catalog test fails because a catalog `payloadSchema` path no longer ends with the descriptor path.
- 26 committed-diff fixture laws fail in 13 pairs because their expected JSON still carries removed camera, locale, hover, selection, and engagement fields.

None of those assertions concern shape creation identity, repeated equal geometry, the current mounted instance used by this packet, or its two utility-reset publications. They remain outside this bounded repair.

## Read-only fixture repair boundary after Native 4

The next fixture-only packet can repair four ownership mismatches without touching the retained mutation aggregate budget:

1. Replace `context::drawing_app() -> DrawingApp` with a self-closing `DrawingAppFixture` that dereferences to `DrawingApp`, binds `meta("local").instance_id` at construction, and drives `close_registered_fixture_app` from `Drop` unless the app is already terminal-empty or the thread is unwinding. Flow's current `FlowAppFixture` is the repository pattern. This one construction repairs the common live-instance setup and terminal closure for all 26 `drawing_app().await` call sites. Generic helper calls such as `settle_registered_typed_operation` must receive `&mut *app`, as the wrapper does not itself implement `PluginApp`.
2. Encode the retained-envelope edit with the production Drawing operation codec. `vcs.edits[*].forwards` remains the canonical list, but its member is the hex scalar returned from `crate::spr::encode_op(&mutation)`, matching the Process3d/Generation examples. The current fixture inserts a serde object as the member, which explains `drawing-envelope.mutation-pack-must-be-scalar`. The live-envelope tests then exercise Ready/ACK, cancellation, stale handles, and hostile edit ids under the same self-closing app owner.
3. In the three Drawing binary/text store laws, construct with `ArtifactStore::new`, immediately install `crate::spr::drawing_document_store_owners()` through `install_document_store_owners_exact`, and drain `close_owned_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)` to `close_owned_terminal_is_empty()` on every terminal path. The generic `plain_test_store` is insufficient authority for this domain because these laws are meant to exercise Drawing's actual snapshot and mutation retirement factories.
4. Update the two manifest claims independently: require `setActiveUtility` in the application action ledger with the canvas utility/window relationship, and keep the document artifact preparation factory while requiring the canvas window config owner's factory instead of a nonexistent `NoConfig` document-config factory. Neither change adds a production route or weakens retirement.

The catalog path normalization, 13 committed-diff fixture pairs, and the retained mutation aggregate admission defect are separate packets. The first two need their own canonical-oracle generation evidence; the aggregate defect needs the mutation-specific owner-budget law recorded in `📓️terra-draw-native4-failure-authority-audit.md`. They should not be mixed into the app-fixture/wire packet.

No Draw source was modified during this audit and no Cargo command was run. The proposed focused receipt should select the live envelope law, representative pointer and patch dispatch laws, utility/route manifest laws, and all three store round-trip laws; the full Draw suite must follow because the fixture constructor is shared across 26 tests.

## Fixture packet implementation

The authorized fixture packet changes only three Draw test files:

- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `DrawingAppFixture` binds the registered `meta("local")` instance and closes through the framework lifecycle from `Drop`; generic law helpers receive the dereferenced app. The retained envelope now stores the production Drawing operation codec's hex scalar inside the canonical `forwards` list. The utility law reads `setActiveUtility` from the app ledger and resolves it through the canvas window; the retained-route law requires no document `NoConfig` preparation factory and registers the concrete canvas `WindowConfigOwner` instead.
- `🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: both mutable store laws install `drawing_document_store_owners()` before Apply and drive owned close after their oracle.
- `🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: the command-envelope store law uses the same exact owners and owned close.

Against Native 4's disjoint classification, this packet is expected to remove 28 failures: 13 live-instance component cases, 9 component terminal-owner cases, 1 live-envelope wire case, 1 retained-route disposition case, 1 utility ledger-location case, and 3 store round-trip owner cases. The supported remaining bucket is 35: 1 catalog path oracle, 26 committed-diff oracle pairs, and the 1 aggregate-budget defect plus its 7 cascading retained-authority cases. This is an expected boundary pending root-owned focused/full native receipts; no Cargo command was run locally.

## Read-only recommendation for the catalog and 13 committed diffs

The catalog failure should be repaired at the declared payload authority, not by making the current suffix predicate more permissive. All fourteen derived `MutationLeafDescriptor` values read their adjacent leaf descriptor and declare `payload_schema: "🧬️schema/🔣️.json"`; that owner-relative file exists for every Drawing mutation and is the taxonomy's sole `mutationPayloadSchemaAuthority`. The four subset oracle documents still mix eleven removed `🔣️.schema.json` paths with three Rust source fragments. The eleven JSON paths resolve to no file. The repository's current mutation authoring path also emits `🧬️schema/🔣️.json` for a JSON-schema leaf.

The exact catalog repair is therefore:

1. In the `style`, `metadata`, `transform`, and `structure` `🔮️oracles/🔣️.json` documents, replace every Drawing mutation manifest's `payloadSchema` with the leaf-declared `🧬️schema/🔣️.json`. This is fourteen entries, including the three old `#ReplaceLayerFill`, `#ReplaceLayerStroke`, and `#CreateLayer` Rust references.
2. Change `kinds_match_the_enum_and_the_catalog` from `ends_with` to exact equality between the manifest entry and `descriptor.payload_schema`. In the same loop, join `descriptor.owner` with `descriptor.payload_schema`, require that exact regular file to exist, parse it as JSON, and require the declared draft-07 dialect. Keep the existing semantic id, aggregate variant, text opcode, binary tag, count, and ordering checks.
3. Do not add a basename comparison, accept both old forms, infer a sibling type, or copy the nonexistent `.schema.json` names. Those choices would create a second payload authority and contradict the current descriptor-linked owner contract.

The diff repair is a data-only rebaseline of these exact thirteen cases: `set-layer-blend-mode/normal-to-multiply`, `set-layer-opacity/dims-shape-a-to-half`, `replace-layer-fill/solid-to-linear-gradient`, `replace-layer-stroke/adds-a-dashed-stroke`, `rename-layer/renames-shape-a-without-touching-its-id`, `set-layer-visible/hides-shape-a`, `set-layer-locked/locks-shape-a`, `set-layer-boolean-operation/union-to-subtract`, `update-layer-transform/translates-and-scales-shape-a`, `update-layer-trace-params/sharpens-the-trace`, `create-layer/appends-shape-b-at-the-root`, `reorder-layer/moves-shape-a-above-shape-b`, and `delete-layer/removes-group-a-with-its-child`.

Every one of the thirteen committed `🔺️diff/🔣️.json` files contains the same seven stale null members: `cameraX`, `cameraY`, `cameraZoom`, `engagementInput`, `hoveredId`, `locale`, and `selectedIds`. `DrawingDiff` now has exactly seven artifact lanes (`artifact`, `schema`, `id`, `title`, `layers`, `assets`, and `artboard`), so decoding then re-encoding already proves that those view/runtime members are outside the persistent diff. Rebaseline each fixture from `serde_json::to_value(<DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(...).diff())`; the supported byte-level edit is removal of those seven trailing null members while preserving the produced artifact delta. Run all three laws per case: produced equals committed, committed decodes and re-encodes canonically, and committed applies to the declared after snapshot. This accounts for the paired 26 failures without touching production diff semantics.

This is a recommendation from current source and committed fixture inspection. No Draw fixture, catalog, or production source was mutated, and no Cargo command was run while the root-owned native lane was active.

## Native 12–13 completion census

Draw Native12 ran 281 laws: 277 passed and four failed. The retained-mutation cancellation, duplicate collision, and late-cancel laws passed. After the bounded camera, selection, and initializer repairs, Draw Native13 ran all 281 laws: 280 passed and only `set_selected_opacity_reads_the_framework_interaction_selection` failed.

That final failure occurred after the artifact-lane assertion passed and after the document opacity was mutated. The shared `assert_one_artifact_publication` helper required one terminal completion for every command, but this framework-selection consumer is a two-component retained operation and reported two terminal witnesses. The helper now has an exact `assert_artifact_publication_units(receipt, expected_completions)` form. It still requires exactly one Artifact result page, and additionally requires the exact expected completion count, the same number of operation identities and revisions, and distinct operation identities. Ordinary commands continue through the one-completion wrapper; the selection-consuming opacity law alone requires two, then retains its exact `0.25` document assertion. This accounts for component operation structure without weakening the artifact or mutation outcome.

No Cargo command was run locally. The expected Draw14 acceptance is 281/281.

## Catalog and committed-diff implementation

After the Native 4 fail-first receipt was accepted for this bounded packet, all fourteen Drawing mutation entries across the four subset oracle documents were changed to the exact descriptor authority, `🧬️schema/🔣️.json`. The catalog law now compares `payloadSchema` to `descriptor.payload_schema` exactly and pins the Drawing payload surface to that canonical owner-local path. It retains the semantic id, aggregate variant, text opcode, binary tag, count, and declaration-order checks.

The thirteen committed diff fixtures listed above were handcrafted by removing only the seven obsolete null view/runtime members. Their artifact deltas were not regenerated, reordered, or weakened. JSON parsing covered all 122 Drawing `🔣️.json` documents; no stale member remained under a committed `🔺️diff/🔣️.json`, and the four manifests exposed exactly fourteen canonical payload references.

Third-party validation used Ajv in draft-07 strict mode with the Drawing artifact schema registered for the diff schema, the repository's `x-semio-state` keyword, and the declared `uint32`/ `double` formats. All 13 diff fixtures validated and all 14 owner-local payload schemas compiled. The Nx-owned Draw JavaScript publication audit also passed: 2 tests, 0 failures, strict Ajv schema accepted, owned oracle accepted, and 5 hostile mutations refused. A first generic `nx exec workspace` attempt was blocked before the validator ran by the concurrent workspace project-graph cycle `framework-os-kernel -> value-derive -> framework-os-kernel`; the direct `@semio-tech/draw-js:publication-authority-audit` target completed successfully. No Cargo command was run locally; the root-owned full Draw census remains the native acceptance receipt.
