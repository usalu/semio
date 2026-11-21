# Mounted Puzzle 5D rewrite tests

The earlier native binary stalled in `prepare_reserved_media_header` on the registered editor’s first import. That stage needs the current reserved projection. The fixture’s snapshot assertion reads the document directly and leaves the projection cache cold. The repeated batch fixture import could therefore keep returning pending without reaching a cancellation or fuel limit.

After a producer mounts, `advance_typed_operation_publication` waits for the host to collect the preparation and retirement ownership receipts. The direct fixture now collects those receipts, renews the fixture grant for each import, publication and maintenance turn, and bounds the loops with the fixture caller’s authored turn limit. The original import continues to own its result acknowledgements.

The tests exercise an intrinsic Puzzle 5D rewrite, a foreign schema refusal, cancellation of the exact pending rewrite, and intrinsic artifact ingestion followed by an undo of the single document edit.

## Mounted envelope retirement

An in-process Mach sampling probe of the cold-cache rebuild captured the producer reaching `OriginalEmissionAdmission::take_mounted_into` after native wire admission, the Puzzle 5D job builder, and child-content capture. The stalled turns returned through `OriginalEmissionAdmission::matches` in the reserved mount dispatcher.

Closing the mounted envelope clears its source binding. The dispatcher checked that cleared binding before revisiting the phase that releases the empty envelope, so it could never return the mounted operation to its original media command. The exact mounted receipt remains available as an authority witness for releasing the terminal envelope.

## Catalog imports and native archives

The mounted Puzzle snapshot delegated child visitation to its typed snapshot but inherited an empty child-slot declaration. A catalog edit then exposed a real child reference with no matching slot. Delegating the declaration fixed the same producer’s five existing kit cases, including the exact media ceiling and one-byte refusal. The rebuilt binary also passed the three mounted rewrite cases.

A catalog import also created a child handle without opening a composable member. Native archive export emits only live members, and native restore checks that every projected child has its exact member. Puzzle’s editor and viewer now share a one-kind roster backed by stdio’s existing Semio kit producer. They derive the kit from the persisted catalog and verify its canonical handle before encoding the member. A fixture checks that updating this catalog replaces its single archive member and that native archive ingress restores the catalog-bearing snapshot.

The native snapshot codecs confirmed that the shipped Nakagin and Capsule Dream DSL handles no longer matched the catalog rows persisted alongside them. The corrected DSL changes only the two occurrences of each identifier. Re-parsing gives the same typed snapshot with its canonical kit handle. Capsule Dream’s package asset digest was updated and verified on the final bytes. The checked-in Puzzle 5D SPR example assets are placeholders and contain no authored mutation records. The example Pack markers likewise carry no canonical kit handles. The raw DSL assets are the catalog reference owners.

## Catalog edit history wire

The fresh archive initially reached the forward-mutation decoder and rejected the operation bytes while parsing a Pack value header. The original publisher had already emitted the protocol-declared record frame, which contains a format byte, the mutation tag, and a record body. Puzzle 5D now uses this tagged form for both the synchronous codec and the retained native receiver. The existing wire round-trip pins the format byte and declared tags. The receiver derives original typed receiving for the mutation records and their actual nested fields.

The native rebuild passed the five existing kit cases and three mounted rewrite cases. The byte guard passed in a native harness linked to the production mutation aggregate. Catalog updates generated exactly one canonical kit archive member and replaced that member after the second delivery.

Fresh archive restore passed forward-operation hydration and reached the replacement's member frontier. The archive runner checks for its exact replacement operation and generation before independent genesis. The rebuild still reported that the roster sealed early. The diagnostic also covers a replacement that entered rejection before admission, and does not report that replacement's typed refusal. Tracing the initializer confirmed that the candidate rejected replay because the Puzzle 5D store owner catalog had no native semantic replay producer. The editor and viewer now fund the existing cold replay authority and install the matching tagged binary wire. The editor retains its specialized mounted publication factory. The rewrite regression also reopens the exported document in a fresh app and checks the rewritten projection before undoing the original edit.

Once replay was enabled, both history-bearing archives exercised a common Store initializer panic. Receiving a retained history input requires one turn to admit the exact field alias and a later turn to publish its projection. The initializer was testing alias occupancy as its readiness check. It now waits for the published projection, matching the existing preview and report replay drivers. Both archives then reached semantic replay retirement. The initializer dropped the original Source identity from the retirement effect, and the retained source owner refused it. The initializer now lends the original Source-bound effect to its existing controlled retirement callback.

In-process tracing of the next typed Work refusal found it in forward replay after operation preparation. The selected cleanup was trying to refill its original failure-storage pool while the decoder still held the replay effect's Work reservation. The critical stack was `OriginalWork::prepare_for_action` → `NativeRetirementDecodeControl::prepare_failure_storage_work` → `NativeSemanticPreparation::close_step` → `ArtifactDerivedOperation::step`.

The Store initializer now funds the failure-storage pool through the decoder before a controlled retirement action enters the replay effect. The same decoder recipient carries that storage into the selected cleanup. The shared Store changes are in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️native/🦀️.rs`.

The final native editor run passed all six `kit_in` and all three `rewrite_in` tests. The kit archive contained a single canonical member after each delivery, replaced that member by content identity, and restored the final snapshot and member in a fresh app. A separate intrinsic Rewrite input changed one projection field, preserved the rewritten projection across native archive restore, and undid the original edit in one Undo. The other rewrite tests passed the pending publication and cancellation assertions. The tagged-operation byte guard and both shipped catalog-handle checks passed through native probes.

### Files for the tagged codec and its fixture

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️wire-format-guard/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
