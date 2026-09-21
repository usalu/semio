# Checkpoint 18 Input Presentation Generation

## Finding

Checkpoint 18 exposes a real input/presentation generation mismatch. `dumpChrome` does not expose a speculative registry: it serializes the same resolved `InputState` hit registry that `Shell::handle_pointer_button_for` uses for live pointer routing. That registry becomes active when the chrome walk completes, before the prepared render packet reaches GPU presentation acknowledgement.

At step 12, the newly opened Settings NumberSteppers were therefore live pointer targets while the screenshot still showed the preceding full-width World frame. The current double buffer protects pointer routing from a half-built chrome walk, but it does not hold the completed chrome registry until the corresponding pixels are presented.

## Checkpoint receipt

`checkpoint-18-main/steps.json` establishes the publication transition:

| step | Chrome generation | published Settings body hits | screenshot |
| --- | ---: | --- | --- |
| 10 `settings-open` | 6 | none | old full-width World |
| 11 `settings-app` | 6 | none | old full-width World |
| 12 `settings-general` | 7 | Settings toggle and four NumberStepper controls | still old full-width World |
| 13 `settings-appearance-open` | 8 | Settings body remains published | still waiting for the new presentation |

Step 12 reports `generationBefore: 6`, `generationAfter: 7`, `firstPublicationAfterOperationMs: 264`, and settles in 2,657 ms. Its Chrome registry includes:

- `puzzle3d-play-settings` at `[1300, 768, 293.59998, 24]`;
- `puzzle3d-play-settings.contact-tolerance.control` at `[1300, 806.4, 293.59998, 22.400024]`, plus the proximity-radius, chunk-size, and grid-spacing NumberStepper controls below it, all owned by `puzzle3d.panel.settings`;
- the already-reserved perspective World at `[437.6, 60.8, 849.59985, 907.2]`.

The step 12 screenshot, `wgpu/12-settings-general.png`, still shows the preceding full-width World through that right-hand area. The prior presentation audit records the later frame timing and the first screenshot that visibly contains the Settings body in step 17.

The probe did not click one of the invisible NumberSteppers, so this run has no dispatched-action receipt for that exact gesture. The production source still establishes that the step 12 rows were active pointer authority, because diagnostics receives the exact slice used by `hit_at`.

## Production authority trace

### Registry promotion and diagnostics are the same operation

`InputState` carries a `HitRegistry` with `staging`, `resolved`, and a local publication counter. `register_hit` writes only to staging. `publish_hits` swaps staging and resolved; `hits` returns resolved; `hit_at` resolves from that same resolved vector (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:200-245,311-345`).

At the end of `ShellChromeFramePhase::PersistPreferences`, `render_chrome_step` calls `publish_retained_hit_registry` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23037-23042`). That function:

1. swaps the retained owner maps;
2. calls `input.publish_hits()`;
3. gives `input.hits()` to accessibility;
4. gives the same `input.hits()` slice to `note_chrome_hit_registry` (`Shell:13152-13166`).

The introspection implementation describes its rows as the pointer registry and maps that slice directly into `DumpHitTarget` rows. Its `ChromeLedger.generation` increments for each complete chrome walk (`Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:3486-3512,3699-3712`). There is no separate candidate registry behind the diagnostic result.

### Pointer dispatch reads the just-promoted registry

`Shell::handle_pointer_button_for` logs `input.hit_generation()` and `input.hit_at`, then uses `input.hit_at` for the actual press path (`Shell:12529-12537,12664-12683`). A retained body hit immediately routes through `route_retained_pointer_press`. No presented-frame generation or presenter witness is checked on this path.

### GPU presentation happens later and carries no input registry

`FrameBuildPhase::Chrome` mutates `AppInteractionState.input` in place by passing it to `render_chrome_step` (`renderer/🦀️.rs:15016-15025`). `AppFrameBuild` carries the prepared render input, packets, frame generation, cursor, theme, fullscreen request, and wake/progress state (`renderer:12346-12356`). `AppFramePresentation` carries the prepared render packet and the same frame metadata (`renderer:13448-13458`). Neither owns a staged hit registry or an input-publication witness.

The presenter later uploads, stages the prepared packet, performs GPU render work, and only then acknowledges its `PreparedPresenterWitness` (`renderer:14459-14585`). Completion is returned after presenter retirement/directives at `renderer:14603-14614`. Registry promotion already happened in the earlier Chrome build phase, so a blocked upload or render leaves new input geometry active over old pixels.

## Existing guarantee and missing guarantee

The existing registry double buffer correctly guarantees that a half-walked frame never becomes pointer authority. It also keeps each resolved hit vector and its retained owner map coherent.

It does not guarantee that resolved pointer authority belongs to the last GPU-presented frame. The `HitRegistry.generation` counts chrome publications only and is not tied to `AppFramePresentation.generation`, `PreparedPresenterWitness`, acknowledgement, or abort. A presentation delay exposes the exact mismatch observed in checkpoint 18; an aborted prepared packet would leave the same mismatch indefinitely.

## Focused regression law

The required native behavioral law is:

`candidate_chrome_hits_do_not_route_until_their_frame_is_presented`

The fixture must use two layouts with a target that exists only in the successor layout:

1. present layout A and verify A's target routes;
2. complete layout B's chrome walk while its prepared presentation remains pending;
3. press B's new-only target and verify it does not dispatch;
4. verify A's still-presented target remains the input authority;
5. acknowledge B's presentation;
6. press B's target and verify exactly one dispatch;
7. repeat the pending branch with an aborted B packet and verify B never becomes input authority.

This law belongs at the renderer transaction/presenter boundary. A Shell-only test would merely restate the current early `publish_hits` behavior and cannot prove correspondence with presented pixels. No registered test or production change was added while Native95 was running.

## Relation to the presentation audit

This finding does not change the earlier geometry conclusion: the completed Settings layout correctly reserves the right-hand width, and there is no evidence that the current World pass paints over the current Settings body. The defect is temporal authority: input advances to the completed logical layout before presentation advances to its pixels.

## Native 97 fail-first receipt

The first registered native law reproduced the production mismatch before any repair:

- filter: `shell::presented_input_authority_tests::a_completed_chrome_registry_is_not_pointer_authority_before_its_pixels_are_presented`;
- run: `990691ce-f96c-48af-9461-56306fc5c427`;
- result: 3 selected, 2 passed, 1 failed, 1,294 outside the filter, 63 ms test runtime, 4 min 15 s Nx runtime;
- failure: the completed candidate target routed while the last presented target was expected to remain authoritative.

The two other selected laws, the engine census and initial closed-window candidate, stayed green. This receipt releases the generation-bound production repair without attributing any unrelated renderer behavior to it.

## Implemented authority boundary awaiting Native 98

The repair keeps four candidate/presented families under one exact Shell-minted presentation witness:

1. `InputState` hit geometry;
2. retained hit owner and scene maps;
3. widget interaction bindings;
4. the Shell geometry used by pointer ownership, dock activation/resizing/drop zones, panel occlusion, context resolution, and utility cursor resolution.

A completed Chrome walk seals these as a candidate and mints a monotonically increasing `PresentedInputCandidateWitness`. The witness is independent of runtime input generation because initial frame zero and scene/resource-only successor frames may legitimately share that generation. It travels through `FrameBuildCursor`, `AppFrameAfterChrome`, `AppFrameBuild`, `AppFramePreparation`, and `AppFramePresentation`. The Chrome walk no longer calls `publish_hits`.

`AppPresenter::Acknowledge`, after Render and CloseGpu, acquires the runtime and interaction owner without blocking. If either is unavailable, the cursor remains in Acknowledge with its GPU and input witnesses intact. While holding that exact runtime lock it verifies the candidate witness, acknowledges the prepared GPU gate, and promotes the input families. A stale, superseded, duplicated, or aborted candidate cannot replace a newer presented authority. Initial witness one is accepted without relying on a nonzero runtime input generation, and successive presentations at the same runtime input generation still receive distinct candidates.

The Aborted and presenter-close phases acquire the same owner and discard only the matching candidate witness. They do not swap or clear the previous presented hits, bindings, owner maps, or geometry. Candidate hit entries remain in the staging buffer for the existing bounded next-frame retirement.

The expanded native laws exercise pending, accepted, stale-witness, repeated-input-generation, and aborted branches. The real retained Shell pointer law publishes two document revisions whose Button keeps its reconciliation key while moving and changing its binding, then publishes a different retained Button over the old pixels. Pointer state still runs through the live `EventRouter`, while the activation descriptor comes from the presented `HitTarget`. Before acceptance the old pixels route the old binding and the new pixels route nothing. After acceptance the overlap owns the old pixels and the moved same-key control owns only its new pixels.

This packet proves the presented hit/owner/geometry boundary and retained Button activation. It does not yet generation-qualify keyboard focus and Tab routing, accessibility event dispatch, or the value-bearing `Select`, `Input`, `Toggle`, `Slider`, `NumberStepper`, `Ring`, and `IconSelect` paths whose current-tree state may differ from the presented frame. Those remain explicit follow-up work; the Settings controls observed in checkpoint 18 are not claimed repaired by the Button-specific binding override alone.

The next fail-first packet is bounded to the checkpoint controls and their alternate input routes:

1. a presented `Select` moved by a candidate must open only from its displayed rectangle; the candidate-only rectangle cannot open it before acceptance;
2. a presented `NumberStepper` increment must use the displayed value, step, binding and rectangle rather than the reconciled candidate node;
3. a focused presented control must keep its exact key/binding for keyboard dispatch while the candidate is pending, and a candidate-only focus target must not receive Tab or text;
4. the accessibility dump and `Activate`/`Value` dispatch must expose the last accepted document generation and control semantics until the candidate witness is acknowledged; abort preserves that projection.

The current UI engine reconciles the candidate tree before GPU acceptance, so merely validating a `NodeId` against the live tree cannot satisfy these laws. The repair needs a bounded presented retained-control authority promoted by the same candidate witness as hits and geometry. It must preserve router gesture eligibility and local overlay/focus state without cloning an unbounded whole tree or routing events to candidate-only nodes. The exact representation remains under source audit and is not yet claimed implemented.

## Expanded neutral oracle receipt

The neutral fixture now includes the moved same-key and overlapping-successor row. Focused command:

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- '🧪️tests/🎯️presented-input-authority/🟦️.ts' -t 'presented input authority'`

Latest result: 1 file passed, 10 skipped; 5 tests passed, 136 skipped; Vitest 1.13 s, Nx 6.0 s, wall 9.9 s. Native production and laws still require Native 98 compilation and execution; this section does not claim that gate green.

## Presented Button gesture ownership review

The first retained Button implementation still had an invalid fallback: any `PointerUp` over a presented Button published its descriptor when the live candidate router produced no action. That admitted a release with no down, a post-cancel release, and a press on Button A followed by release over Button B. It also let a candidate-tree target receive focus or local overlay state before its pixels were accepted.

Three actual Shell laws are registered before the repair:

- `presented_button_release_without_down_does_not_activate`;
- `presented_button_cancelled_press_cannot_activate_on_later_release`;
- `presented_button_press_a_release_b_activates_neither`.

Each publishes real retained Button documents, promotes the initial hit registry, drives an explicit `PointerId`, and observes the framework action side effect. The required repair is a fixed-capacity presented pointer-press owner keyed by pointer id and exact presented target/witness. Only a matching primary press and release inside the same still-presented rectangle may activate. Cancel, foreign pointer traffic, owner replacement, and release over another target retire or preserve the corresponding slot without manufacturing an activation. These laws await the Native 99 fail-first receipt; the current fallback remains unchanged until that receipt.

## Native 99 Button fixture receipt

Native 99 did not execute the intended Button gesture behavior. All four retained Button laws stopped while decoding the test record because the fixture omitted the current required `Button.icon` field. This was a fixture defect, not a production behavior receipt, so it does not authorize changing the Button dispatch fallback.

The fixture now uses the complete current Button schema with `icon: "circle-dot"`. The production fallback remains unchanged for the targeted Native 100 fail-first run:

- `shell::presented_input_authority_tests::actual_retained_pointer_dispatch_keeps_the_presented_position_and_binding_until_acceptance`;
- `shell::presented_input_authority_tests::presented_button_release_without_down_does_not_activate`;
- `shell::presented_input_authority_tests::presented_button_cancelled_press_cannot_activate_on_later_release`;
- `shell::presented_input_authority_tests::presented_button_press_a_release_b_activates_neither`.

Native 99 also exposed existing Shell tests that perform a direct Chrome/render walk without the renderer presenter step. Their failures require causal triage: tests representing an accepted frame must explicitly seal and acknowledge that candidate; tests asserting behavior while the candidate is pending must keep the prior presented geometry. Production must not fall back from an empty presented registry to the candidate registry because that would recreate the checkpoint 18 invisible-control defect.

## Retained control neutral and React oracle

The neutral fixture now carries the exact old/new `Select`, `NumberStepper`, keyboard-focus, and accessibility semantics, plus the candidate's base presented-interaction epoch. The model proves that the old accepted revision supplies Select items/value/binding and NumberStepper value/step/binding until acceptance; a presented interaction invalidates a candidate prepared against the earlier epoch.

The same test mounts a real React DOM control through the repository testing-library adapter. Preparing the successor without committing it leaves only the predecessor DOM node and predecessor action binding mounted; rerendering commits the successor and then exposes its binding. The first attempt exposed that this focused browser-worker file previously used the node environment (`document is not defined`); the file now explicitly uses the canonical jsdom environment, rather than mocking a DOM.

Focused command:

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- '🧪️tests/🎯️presented-input-authority/🟦️.ts' -t 'presented input authority'`

Final result: one file passed and ten skipped; eight tests passed and 136 skipped; Vitest 4.92 s and Nx 11.4 s. This is a neutral/React oracle. It does not make the native candidate `UiWindow.tree/router` safe for input; the native dual-revision laws remain intentionally unregistered until the Button fail-first receipt is captured.

## Native 100 observation correction

Native 100 ran ten focused renderer tests: eight passed and two failed in 225 ms (Nx 3 min 8 s), run `6a0ea30b-a179-4e2c-b997-9d427649451f`. The complete 65-window retirement, closed-camera, reopen, and close-queue laws passed. One failure was the fixture's stale five-law count after the neutral fixture expanded to ten; the assertion now names all ten.

The positive two-revision Button law reached real pointer dispatch, and debug output showed stable presented hits and an admitted action, but it then read `ShellState.driver_save_label` without executing the queued application action. Its empty string therefore did not prove a production binding failure. The three negative laws used the same invalid observation and their passes were vacuous. All four now inspect `collect_fixture_actions` and the exact admitted `value` argument. Production remains unchanged for Native 101. The expected fail-first result is action publication from unmatched, cancelled, or cross-target releases through the unconditional `PointerUp` fallback; the positive law should separately show whether the fallback masks the candidate-tree mismatch.

Five native laws are now registered under the same module prefix for presented `Select`, `NumberStepper`, keyboard focus, accessibility, and stale candidate interaction epochs. They publish actual retained documents and leave successors pending under the Shell presentation witness. Their source parses; they have not yet received a native result.

## Native 101 valid fail-first receipt

Native 101 ran 13 focused tests: three passed and ten failed in 225 ms. Eight failures belong to this packet: unmatched Button release, cancelled Button release, A-press/B-release, presented Select semantics, presented NumberStepper semantics, presented keyboard/Tab ownership, presented accessibility ownership, and candidate interaction-epoch invalidation. The corrected positive two-revision Button binding law passed, confirming that the prior Native 100 failure was only its observation mistake. The two remaining failures belong to the independent scene-close packet.

This is the first valid native RED for the generic retained authority. It authorizes removing the Button descriptor override and building a single generic boundary rather than another widget-specific gesture ledger.

## Generic presented revision implementation

Each `UiWindow` now owns two bounded, move-owned `UiTree`/`EventRouter` revisions. The candidate remains the sole reconcile/layout/paint owner. The presented revision is the sole production pointer, keyboard, focus/capture, scene-query, and accessibility dispatch owner. The first accepted revision starts a bounded candidate baseline capped by `UI_DOCUMENT_NODES = 128` so the otherwise vacant candidate slot can repaint focus/press state before another document arrives. `UiNodeRecord` deliberately is not `Clone`: each opportunity duplicates exactly one record through `UiNodeRecord::credited_clone`, which separately acquires the component, action-binding, and menu-reference credits. A credit refusal retains the partial baseline and drains one owned record per later opportunity before returning `DocumentCredits`. Later acceptances swap the two owned slots and reconcile into the former presented slot. No input event clones a tree, and no generic `Clone` implementation was added to the credited contract records or their tables.

Candidate interaction transfer walks the fixed document identity ledger one record per reconcile opportunity and requires the same `UiNodeId`, key, widget discriminant, and component generation. `ComponentScene` additionally requires the same runtime `host_id`, so a sibling wire `surfaceId` or a replacement host cannot inherit interaction state across the two revisions. A presented input restarts this bounded rebase cursor instead of cloning a whole tree on the input call. After the record cursor finishes and no presented pointer capture remains, it transfers focus/Tab order, hover chain, overlays, drag state, tree-drag press, scrollbar-drag origin, typeahead, flow, focus-visible state, and the monotonic intent sequencer. Candidate paint keeps ownership of its own drag/drop/scroll registration maps, and no candidate can seal while the rebase cursor remains active.

The Shell witness now seals visible UI candidates with their base presented-interaction epoch. Every presented dispatch advances that epoch and refreshes the unsealed candidate state. A sealed frame whose base predates input no longer matches. The presenter verifies the combined Shell/UI witness before starting GPU submission, so stale pixels cannot be rendered and then rejected only after display. Exact acknowledgement swaps the UI revisions together with Shell hit/owner/widget/geometry families; abort clears only the matching sealed candidate.

A visible window that already has a presented revision now refuses Shell sealing while its candidate interaction rebase is incomplete. It cannot silently omit its UI witness and let a host-only candidate advance pixels around the older retained router.

The Button action substitution and unconditional `PointerUp` fallback are removed. Ordinary presented `EventRouter` commands now decide eligibility for every widget. Synchronous `UiCommand`s have one consumption owner in the returned command vector rather than also being cloned into `pending_commands`; asynchronous clipboard completion explicitly queues its commands once for the next interpreter drain.

The root surface-close ladder now also drains the presented router, presented document, and presented tree storage one bounded step at a time. Candidate and presented scene identities continue through the shared scene-retirement queue rather than being dropped with their arenas.

The two-revision pointer fixture now follows the same epoch rule as production: its clicks against the old presented pixels make the pre-click candidate witness stale, so the law first proves that acknowledgement is refused, discards that witness, drives the bounded interaction rebase through a fresh candidate paint, and acknowledges the new witness. The Select, NumberStepper, keyboard, accessibility, and explicit stale-epoch laws also discard their pending witness after the assertion instead of leaving a sealed test owner behind.

## Native 103 and 104 compile correction

Native 103 and Native 104 did not run behavior. Native 103 stopped because the first draft derived `Clone` for `UiDocumentTree` while `UiNodeTable` had no such implementation. Native 104 then proved that deriving `Clone` on the table would require `UiNodeRecord: Clone`, which is intentionally absent because a record embeds credited component values, action bindings, and menu references. Both generic derives and the cascading `UiTree`/arena derives have been removed.

The replacement uses the contract's existing `UiNodeRecord::credited_clone() -> Option<Self>` seam. A heap-owned `UiCandidateBaseline` owns an initially empty document header and advances one record per reconcile call, so the 64-slot fixed `UiWindow` table does not embed a third full document. It verifies that the presented source identity remains exact, keeps the candidate unsealable until the copied document has reconciled into its arena, and participates in document supersession and the bounded surface-close ladder. A failed partial copy is retained and drained before the terminal fault is reported. `rustfmt --check` parsed every touched Rust file; the command returned formatting differences from the repository's existing compact style, so it is not recorded as a formatting pass. Native behavior remains unverified until the next root-owned Cargo gate.
