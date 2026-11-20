# Puzzle 5D work scope and retirement

The retained command and example-selection paths exposed two framework issues.

- `OriginalWorkPerformedReceipt::performed(0)` marked an attempt even though the producer had performed no work. That prevented cancellation of the empty reservation and left its scope outstanding. The shared fix marks an attempt only for a nonzero delta. The partial-work fixture now claims a zero-work receipt before recording its funded segment.
- `FactoryBoxedValue` used the inherited retirement reply, which refused the original boxed window-config mutation. The shared cursor now answers through `close_into`. One funded step moves the payload into the captured owner and another releases its original box allocation. The captured owner handles the payload and issuer retirement through the same reply and recovery contract.

## Verification

- The filtered `owscope-8` run exited 101 before tests. Mesh reconstruction compiled against an engine build without `PolygonSurfaceKind` or `PolygonMeshSource.surface`. Both definitions are now present in the shared engine source.
- The first unfiltered run from the version crate stopped on an unresolved `Frame3` import in B-rep tessellation. The import now points to `semio_framework_3d_brep_vector::matrix::Frame3`, like its sibling consumers.
- The coordinated unfiltered retry stopped on four missing exports from the PDF filter dependency. All four exports were present in the shared source by the time Cargo reported the error.
- The unfiltered suite ran from isolated native inputs. Seven tests passed, including all five target-volume geometry and projection tests, `every_example_parses_into_its_fixture_counts`, and `every_registered_example_builds_its_document`. Eight target-volume command tests and the agent-lane boot test failed with `original lifecycle child receipt differs from its whole producer receipt`. The old boxed-owner refusal did not appear. The missing-origin command reproduced the new error in isolation with `RUST_MIN_STACK=67108864`.
- The shared-cache native fallback used the earlier invocation's pinned compiler, arguments, and build directory through the repo's capacity grant and Cargo lease.
- `original_work_partial_turns_invoice_once_and_keep_original_ceiling` passed in `semio-framework-value-control-contract` from the framework workspace. The run passed one test and filtered out 41.

## Target-volume receipt diagnosis

The isolated target-volume test hit the receipt error while subtracting reserved returns from the instance maintenance ladder's accumulated producer receipts. At the failing subtraction the accumulator contained 3 items and 55 copy bytes. The separate reserved-return ledger contained 1 item and 118 copy bytes. The linked testkit object identifies the caller as `VcsArtifactApp::maintenance_ladder_step`.

The ladder now measures its gross receipt from the initiating context before and after each unit. It excludes current-turn reserved returns only when calculating the remaining loan. The mounted caller still receives the full posted receipt. The registered settlement fixture now uses `MountedOwnerTurnV1::advance_lifecycle` for maintenance. Its previous wrapper could debit a reserved return a second time.

The mounted-owner reserved-return fixture now includes the 3-item, 55-byte fresh spend and 1-item, 118-byte reserved return. Both the focused Rust lifecycle test and the independent SQLite oracle passed both fixture cases.

The next full native run cleared the maintenance receipt mismatch and passed both example tests. The target-volume commands stalled in publication. At turn 4096 the isolated missing-origin command was in `Publishing`, with an unpinned caller, 65.5 billion fuel units, and more than 54 million copy bytes remaining. The fixture had never taken the outstanding preparation receipt, which blocks the publisher from replacing its retained receipt slot. The plain and traced 5D fixture drains now take and validate ownership receipts, finish stepped page projection, and close the receiver page before acknowledging it.

The next full run passed the same seven tests. The missing-origin command answered its expected notice and then failed during app teardown. Six other command tests reached a controlled-retirement panic, and the window-config command stalled later in mounted publication. The agent-lane fixture reached a new member-publication grant failure.

The 5D teardown now drives the shared registered fixture closer under the mounted close policy. The focused missing-origin rerun still reached its bound on the owned-store rung. With symbols, the document lane advanced through the original effect and the first all-zero quote selected stage six, WindowConfig, before its advance witness.

The isolated build predates the shared WindowConfig close-context repair. Its mounted close ladder entered the effect-only close path and the linked binary omitted `WindowConfigOwnerRegistry::close_step_context`. The false terminal condition at the zero quote resolved to the native owner registry's `WindowRegistry::terminal_is_empty`. Its preview and registry Construction checks had already passed. The shared work routes stage six through the original caller context. It also closes registry Construction first and preserves the posted receipts when a typed owner's Construction or StoreBirth advances.

The focused rerun now uses the shared repair from the mounted instance close ladder, WindowConfig owner registry, and typed WindowConfig store owners. It also includes the shared owned-store diagnostic. Snapshot-only testkit sampling was removed before that rerun.

The last completed full run still passed seven tests, including both example tests. Six mutating target-volume command tests and the agent-lane boot advanced to `member.publication: authoring work commit was refused`. The missing-origin teardown and voxel-dimensions publication still had their earlier outcomes in that build. Debugger inspection of the voxel path showed an 82,856-byte preparation quote against its 32,768-byte copy grant.

The focused missing-origin rerun compiled and passed the command assertions. Teardown retired the WindowConfig stage and reached `UnsupportedOwner` at the uncovered close frontier. Debugger inspection traced the first false terminal condition to `OriginalTypingDispatch`. Its original store birth remained active before the dispatch had bound a source. The instance close demand and advance paths now use the same rung selector and choose that typing owner by its terminal check. The selected close path already advances the partial birth through its original authority.

The next isolated full run includes that dispatch repair and the shared final-backing retirement rung. The final-backing repair depends on paged fixed-registry storage, child-registry backing retirement, retained operation alias retirement, and the cancellation issuer's paged backing close. The running build revealed the cancellation issuer dependency through a private-field compile error. Its shared production revision was copied to the isolated inputs for the next run.

The full rerun compiled after the remaining paged-registry consumers were synchronized. Eight of seventeen tests passed, including both example checks and the new WindowConfig preparation quote test. The missing-origin close reached the typed-operation rung and exposed unfinished command construction after its command custody was returned. A second debugger trace found each command-custody field empty and the construction still active. Close now pays that construction's remaining retirement through the existing original construction closer. A command that still holds custody continues to validate its source and receiving recipient.

The focused rerun then reached the three-item, 58-byte construction retirement and rejected its receipt against a second one-item grant. The typed close now follows the existing direct receipt path used by the other close rungs that post to the initiating context. The originating fixture or mounted phase retains the grant check.

The runtime and host-event typed close helper now validates command custody before removing an unpublished reservation. Its remaining close path lets each original owner authenticate its own retirement, including construction that survives the return of command custody.

The fifteenth native run passed both example parsing and codec laws, the boot backing law, and the missing-origin test's command assertions and application close. The agent-lane test completed boot and reached caller retirement. Both then exhausted the fixture decoder retirement loop. A debugger trace identified the decoder's remaining Source loan and a one-item post-refresh grant at its two-item close gate. The isolated build retained the old fixture caller that lent the raw two-item policy grant. Its shared source already quotes the decoder above the Source refresh and intersects the result with remaining caller stock. The sixteenth isolated run includes that shared revision.

The boot example's old 4,096-entry preallocation survived the move into publication as the same backing. At 384 bytes per entry the final release needed 1,572,864 bytes, beyond the mounted 1,048,576-byte release. The constructor now derives its exact output extent from the current typed document and requested example. The fifteenth native run passed the boot allocation law, and the agent-lane test cleared the former publication block.

Large example switches use separately admitted mutation pages in one document edit. The original order is preserved as the store prepares each mutation and creates its inverse, including both the removals from the previous document and the additions from the requested example. Page backing is quoted and released individually. The SQLite test ran through the repo tool and independently matched the row order, page sizes and release sequence in the shared fixture. Native pager and large-example integration are pending.
