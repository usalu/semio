# Flow Native7 Retirement Audit

## Scope and evidence

This is a source-only audit of the four failures reported for Flow Native7: three
registered-fixture close refusals with `document store close awaits a retained reader
or owner`, and one `OrderedMap` bare-drop during the move law. The generated
`flow-native7/run.log` is no longer present after ticket-generated-output cleanup, so
the failing test names, backtrace, and first fault site cannot be re-read. The runtime
categories above are therefore reported evidence, while the causal assignments below
are limited to current source.

No Cargo command or production change was made.

## Verified focused replay receipt

The subsequent focused Native7 replay supplies the production evidence that was unavailable when the source-only scope above was written. It ran with the canonical `RUST_MIN_STACK=134217728` and `RUST_BACKTRACE=1` and reported one Flow failure with 251 assertions outside that replay. The retained-map backtrace is `OrderedMap<WidgetLayout>::drop` from artifact `Recipe::advance`, then `Preparation::advance`, `ArtifactStore::advance_apply_batch`, and typed `VcsArtifactApp` publication. The runner recorded it in `🗑️generated/flow-native7-move-backtrace.log` before generated output cleanup.

This confirms a production retained-publication ownership failure on the move route. The fixture and temporary-observation violations below remain separately actionable, but they cannot explain away the focused move failure. The production repair must retain or retire the layout at the recipe/preparation publication transition; weakening close admission or changing fixture teardown alone is insufficient.

## Confirmed move owner path

The live NodeGraph move route is not an adapter path:

1. `node_graph_edit::apply` calls `fallible_host_operations` for `Move`.
   `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:119-147`
2. The host bridge projects the document, diffs `ChangeLayout`, and explicitly retires
   both detached `FlowHostSnapshot` roots.
   `.../✏️editor/🦀️.rs:2678-2695`
3. The plugin bridge maps framework `ChangeLayout` to `FlowMutation::MoveWidgets`.
   `.../🧬️schema/🧬️mutations/🦀️.rs:133-154`
4. `MoveWidgets::diff` reads a cloned working scene, moves its parts into a fresh
   content child, and leaves the temporary `FlowWorkingScene` empty.
   `.../🧬️schema/🧬️mutations/📍️move-widgets/🔺️diff/🦀️.rs:9-33`

`FlowSnapshot::to_host_snapshot` is a value projection that calls
`flow_working_scene(self).into_parts()`.
`.../🧬️schema/📸️snapshot/🦀️.rs:36-49`
The cloned `FlowWorkingScene` has a custom `Drop` that retires its layout root, while
`into_parts` empties that clone before its `Drop` runs.
`.../🗿️artifacts/🌊️flow/🦀️.rs:243-307`
This is source evidence *against* treating the working-scene cache itself as the
established bare-drop cause.

The real retained-store factory is `retained::artifact::preparation::PreparationFactory`,
not the older local `FlowStoreOneItemPreparation` type.
`.../✏️editor/🦀️.rs:2215-2217`
Its only document snapshot lease is `PreparationState.base`.
`.../🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:79-105`
The close ladder returns that exact lease at lines 455-458; it does not bare-drop it.
The same cursor closes its hash, canonical reader, recipe, post snapshot, scene, and
mutation frontiers first (lines 399-466). Its existing law drives a production
`MoveWidgets` publication through publish/retry/acknowledge or cancellation, then
closes the publication and document store under one-byte and production grants.
`.../📬️preparation/🧪️tests/🔬️unit/🦀️.rs:4-85`

`FlowPlayApp` is stateless and the instance-owned `FlowEvalSession` has a bounded close
ladder with an exact terminal witness.
`.../✏️editor/🦀️.rs:2131-2182`
It is not a supported owner assignment for the document-store reader refusal.

## Confirmed test-only violations

### Raw application fixtures

`flow_app()` and `flow_app_with_registry()` return a raw `VcsArtifactApp`.
`.../✏️editor/🧪️tests/🔬️unit/🦀️.rs:84-97`
The adjacent `FlowAppFixture` documentation states that a raw drop violates the
store's terminal-empty contract and that `close_registered_fixture_app` is the required
close path (`lines 10-50`). Multiple current Flow tests still use raw constructors,
including:

- NodeGraph delete/spotlight: `.../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:11-31`
- delete-selection: `.../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs:5-37`
- main scene and measures: `.../🎭️modes/✏️edit/🪟️windows/🌊️main/🧪️tests/🔬️unit/🦀️.rs:16-60`

The full source census finds raw use in more Flow test modules, so this is not bounded
to Native7's three rows. Replace test uses with `flow_app_closing()` or explicitly
drive `close_registered_fixture_app` before scope exit. This repair belongs to fixture
ownership. It does not by itself explain a failure that already came from a registered
fixture's close ladder.

### Bare host-snapshot observations

`FlowHostSnapshot` owns a fail-closed `OrderedMap<WidgetLayout>` and must be retired.
Current tests create and discard direct projections, for example:

- `.../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:15`
- `.../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs:11,17`
- `.../🧬️schema/🧬️mutations/📝️text/🧪️tests/🔬️unit/🦀️.rs:12,21,23,37,39,56,67-68`

Those expressions construct `to_host_snapshot()` solely to inspect a field, then
implicitly drop the host snapshot at expression end. This is an exact causal source for
an `ordered-map root must be explicitly retired before drop` failure. It is a fixture
observation defect, including if it is the Native7 move-test terminal failure.

Use the existing `with_live_host_snapshot` helper
(`.../✏️editor/🦀️.rs:2662-2669`) for field observations, or bind a local snapshot and
call `retire_cold()` after extracting scalar/test values. The existing true move law
already follows this discipline for both initial and live projections:
`.../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:80-96`.

## Close-block interpretation and focused reproduction

The framework maps a blocked owned-document close to the reported text only when the
store reports `SnapshotRetirementStep::Blocked`.
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15530-15546`
At that stage the cursor has no returned reader to retire while at least one snapshot
lease remains outstanding; retrying the closer cannot release that holder.
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1961-1990,16258-16276`

The exact fail-first native law should use the existing real NodeGraph `move` route,
not a synthetic `FlowWorkingScene`:

1. Construct `flow_app_closing()`.
2. Send `nodeGraphEdit` with the real `move(add, 284, 48)` row and settle its registered
   typed operation, as the existing move law does.
3. Verify the layout through a scoped/retired host snapshot.
4. Explicitly invoke `close_registered_fixture_app(&mut *app)` before scope exit and
   assert `close_terminal_is_empty()`.

The unmodified current source expectation is that this is green: the cursor has a
`base.return_to_registry()` close branch and its own store-level publish/cancel/close
law. If it reproduces the reported block, record the first active close rung and the
store's outstanding versus returned snapshot-read counts immediately before it. That
will distinguish a missed `Preparation` close from a different retained job/reader
owner. Without that witness, assigning the three close blocks to the Flow cache,
FlowPlayApp, or the fixture helper would be speculation.

## Repair ordering

1. Fix all raw Flow test constructors and all direct host-snapshot temporary
   observations. These are confirmed defects and remove independent false failures.
2. Run the focused real-move close law above with a close-rung/read-lease witness.
3. Only if that law remains red, trace the concrete holder that prevents the
   `Preparation.base` lease from reaching its lines 455-458 return branch. Repair that
   holder's ordinary completion and cancellation paths together; do not weaken the
   document-store refusal or bypass its reader-owner contract.

## Confidence

- **High:** raw-app fixture use and direct `to_host_snapshot()` temporaries are real,
  independently unsafe test ownership patterns.
- **High:** the current working-scene bridge and retained Preparation cursor contain
  explicit transfer/close paths; they are not source proof of the reported runtime
  defect.
- **Medium:** Native7's single OrderedMap failure is one of the direct observation
  sites above; the removed log prevents binding it to a specific test.
- **Low:** any claim that the three registered-fixture blocks are a Flow production
  leak. A focused run with the specified lease witness is required.

## Flow Native8: exact phase-22 map owner correction

Native8 retained the same production failure after the earlier phase-23 placeholder
work: its one red law was
`node_graph_move_wire_publishes_the_requested_widget_layout`, with the fail-closed
`OrderedMap<WidgetLayout>` drop. The recorded run executed six laws, five green and
one red. The Native8 receipt is
`🗑️generated/astra-runtime/flow-native8/run.log`. Parent-supplied focused-backtrace
evidence identifies `Recipe::advance`; the Native8 receipt itself only includes the
ordered-map panic, not the frames.

The direct cause is now source-proven. In the pre-correction phase 22, the recipe did
`let map = mem::take(&mut scene.layout)` and called `map.begin_set(...)` or
`map.begin_remove(...)`. Both APIs take `&self`, not `self`
([ordered map](../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:109)), so the cursor cloned the root while the original `map` local remained live.
The local then reached `OrderedMap::drop` at the end of `Recipe::advance`. A
phase-23 `replace`/placeholder retirement cannot reach that earlier drop.

The current repair transfers that exact original root to
`state.retirement` immediately after constructing the cursor:
[recipe phase 22](../../../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs:322).
This is the required transfer, rather than a capacity or lifecycle workaround.

### Owner accounting

1. The original pre-update map is moved into `Owner::Layouts` at phase 22. Its
   `FlowRetirement` route is `Owner::Layouts -> FlowOwner::Layouts`, which drains a
   bounded domain page ([adapter](../../../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:147)).
2. The `UpdateCursor` holds its own cloned base and transient root aliases. On
   completion it hands only its new result to `scene.layout`; its explicit phase-24
   close drains its base/current/path roots ([cursor contract](../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:278)).
3. The new result becomes the scene's sole live map at phase 23. The destination was
   made empty by `mem::take`, so this assignment does not overwrite another live map.
4. Normal completion takes the result scene into `Preparation`, then closes the
   recipe's accumulated original-map retirement before the recipe can become terminal
   ([recipe close](../../../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs:398)).
5. Cancellation after phase 22 calls `begin_close` on the cursor, drains
   `state.retirement` first, then explicitly closes the update cursor. Thus it cannot
   lose the original map or bare-drop the cursor clone.

The two roots are intentionally distinct retained aliases. Removing either the
phase-22 original-map owner or the cursor close would reintroduce an unretired root.

### Fail-first law

Keep the existing end-to-end `nodeGraphEdit` move law
([native fixture](../../../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:80)); it reaches phase 22 through typed `VcsArtifactApp` publication and asserts the requested durable layout. Add one direct recipe cancellation law that advances a populated `MoveWidgets` recipe through phase 22, calls `begin_close`, drives the existing one-item close helper to terminal emptiness, and asserts the source `Arc` expires. It isolates the only boundary the full route previously missed: original map ownership immediately after cursor construction.
