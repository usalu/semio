# Note Empty Configuration Ownership

## Current result

The Note app now mounts framework `NoConfig` and `NoConfigMutation` directly. The local empty `NoteConfig` type, mutation, codec, fixture, tests, and five-language schema family are retired. The Note app schema descriptor moved to the editor owner and publishes an empty configuration facet while retaining the existing nonempty Note presence schema.

This correction does not move Note document state. Grid visibility, spacing, subdivisions, opacity, snapping, blocks, ink, and other document mutations remain document-owned because this slice found no producer/consumer evidence for a different owner. Exact composite-window camera configuration and engagement transient state remain exact-window state.

## Ownership boundaries

- App configuration: framework `NoConfig` / `NoConfigMutation`, bounded framework owners, and framework disposer.
- App presence: existing Note presence schema and mutation, with exact local-root, peer-root, and store retirement bound to the actual registry lifecycle.
- Composite-window configuration: existing `NoteCompositeWindowConfig`, including camera, keyed by the concrete window id, kind, and lease.
- Composite-window transient: existing `NoteCompositeWindowTransient`, including engagement state, keyed by the concrete window id, kind, and lease.
- Document: existing Note document and grid preferences, unchanged.

The app schema descriptor remains `s.note.note`. Its configuration facet is empty and its presence facet still embeds the real Note presence schema contract.

## Language-neutral validation

The fixture `✏️editor/🪟️window/🧫️fixtures/🔣️.json` contains an empty app config, a hostile foreign app-config field, document sentinels, and two same-kind composite windows. The ownership oracle `✏️editor/🪟️window/🧪️tests/🔬️ownership/🟦️.ts` independently checks the contract with:

- Ajv 2020 strict validation for the empty app config and the existing exact-window configuration and transient JSON schemas;
- Ajv rejection of the foreign app-config field;
- fast-json-patch application of a camera update and transient staging to the left window;
- byte-equivalent preservation of the right window, document, and app config;
- exact clearing of only the left-window transient;
- strict TypeScript checking of the oracle and existing window schema interfaces.

The isolated Nx facade completed successfully:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY='../🗑️generated/nx/note-empty-config-oracle-1' bun /Users/ueli/Documents/semio/node_modules/nx/dist/bin/nx.js run abstraction-ownership-validation:note-empty-config-ownership-oracle
```

Evidence: `🗑️generated/note-empty-config-ownership-oracle-1.log` and the post-constructor-correction rerun `🗑️generated/note-empty-config-ownership-oracle-2.log` (both exit 0). The current facade also evaluates the existing mounted-dispatch source oracle against the live helper and two hostile mutations: removing the initial drive and duplicating it. `🗑️generated/note-worker-job-one-turn-oracle-3.log` is green, so the shared carrier correction retains exactly one initial drive.

Static source checks also found no remaining `NoteConfig`, `NoteConfigMutation`, `editor::note::config`, or `s.note.note.config` reference under the Note plugin. The edited project JSON files parse, both launch catalogs contain orders `311.201` and `311.202`, framework casing remains at `311.200`, and the scoped diff passes `git diff --check`.

## Native validation

The first isolated native Nx run reached the focused crate and exited before runtime on local test harness compilation errors: the new laws lacked the `PluginApp` trait import, constructed pack fields with the wrong byte types, referenced the window module without importing it, and awaited the synchronous close step. Source review also bound bounded document retirement, framework NoDraft and NoTransient retirement, and exact domain retirement for Note's preserved nonempty presence. The second run compiled every production lifecycle addition and narrowed the remaining error to the test fixture's `ops` field, which is a string while `spr` is bytes. The third run compiled the focused crate cleanly, then the first selected camera law overflowed the native test thread before an assertion or debug witness. Heap-pinning each complete law future without changing the test thread's stack size did not alter the fourth run's failure.

The fourth run's matching macOS diagnostic identifies a production constructor fault rather than a remaining fixture-future fault. Every selected Note thread reaches `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<NoteSnapshot, NoteMutation, NoMembers>>::new`, whose `Vec::resize_with` path materialized each approximately 98 KiB registry owner through by-value `MaybeUninit` iterator frames. The bounded shared correction preserves fallible exact reservation, establishes the 64 uninitialized slots with `Vec::set_len`, and documents the invariant that the zero occupied mask prevents every read until insertion writes the slot. Existing exact insertion, removal, capacity, and terminal-empty disposal remain unchanged. A focused native registry law instantiates a larger 128 KiB owner type and verifies admitted 64-slot heap backing, a zero mask, capacity, emptiness, and absence of readable ids. The fifth run compiled and passed this registry law 1/1 under the existing stack limit. Its subsequent Note build stopped before runtime on 24 generic-bound errors in the concurrently added boxed document-store publication helper; the recursive owner repaired that separate shared compile surface after the run. The legitimate outer law heap pins remain in place. The newer recursive-suite 39 diagnostic is distinct: it has no `ArtifactFixedRegistry` or `resize_with` frame and triggers later in `ActiveArtifactStoreReplacement::drive_member_open` and `drive_initializer`, so it is not evidence for this constructor fault.

The sixth run compiled the Note crate, passed the registry-level NoConfig law with its exact foreign-pack rejection and terminal-empty retirement witness, and advanced the camera law beyond app construction. It then exposed a separate later stack boundary during the first typed camera dispatch: `MountedWorkerJobSession<ErasedToolJob>::pump_one` through `MountedTypedCommandFullOperation::drive_worker_step` and `VcsArtifactApp::start_typed_command_operation`. Its diagnostic has no fixed-registry, `resize_with`, or `MaybeUninit` constructor frame. The independent mounted-dispatch oracle requires exactly one initial worker pump inside the start helper, so removing or deferring that pump would violate the established one-turn contract.

The job module now fallibly reserves one heap slot before consuming the producer and initializes `WorkerJobAuthority` field by field at that stable address. A unique `WorkerJobAuthorityOwner` moves through the session, worker submission, rejection, outcome checkout, cancellation, and bounded retirement paths, preserving the existing state machine and one-initial-drive semantics while avoiding the large authority move exposed by Note6. Allocation refusal returns the exact job and parameters; payload pre-admission refusal additionally returns its page source. A focused native regression holds one concrete address from mounted admission through submit and outcome checkout, then resumes and performs bounded terminal retirement. Native run seven compiled and passed that regression 1/1 under the unchanged stack limit. The Note NoConfig law also passed, and both window laws advanced without a stack failure before reaching the fixture runaway guard.

The remaining run-seven fault was in the local host-turn fixture: it ACKed result pages and drained effect, event, and UI channels, but omitted the mandatory typed-operation completion witness. `has_pending_typed_operations` counts that outbox, so the otherwise completed camera and transient commands could never report quiescence. The helper now consumes completion in the host's actual event, completion, UI order. Run eight confirmed every command settles. Its camera assertion then read the fixture projection as plain text even though scene documents are encoded bytes; it now uses the existing typed `decode_fixture_scene::<InkCanvasScene>` path and compares each decoded `document_json.camera`. Run eight also exposed a final transient-case close failure while the concurrently executed focused laws and camera's reopened app all aliased global runtime instance `1`. Each focused app now binds a distinct runtime identity and dispatch metadata/result-page receiver uses that same identity, removing the cross-test owner alias before the next proof. Final Note runtime confirmation is source-ready and awaits the coordinated Cargo slot.

The target filters three focused Note laws:

- `note_empty_config_owner_registry_rejects_nonempty_pack_and_retires_terminal_empty`: the actual registry publishes an empty config pack, rejects a nonempty pack, preserves its empty state, and retires cleanly through the registered app close path.
- `note_empty_config_owner_exact_composite_window_cameras_isolate_and_reload_without_document_or_app_config_changes`: two same-kind windows update and reload independent camera configuration without changing document or app-config bytes.
- `note_empty_config_owner_exact_composite_window_transient_isolates_resets_and_cancels_with_registered_app`: one exact window stages and clears engagement state while the other window remains unchanged, then both registered apps close.

Compile, runtime, and crash evidence: `🗑️generated/note-empty-config-ownership-native-1.log` (session `20976`, exit 1 before runtime), `🗑️generated/note-empty-config-ownership-native-2.log` (session `13356`, exit 1 before runtime), `🗑️generated/note-empty-config-ownership-native-3.log` (session `81266`, compile green then SIGABRT stack overflow before any law completed), `🗑️generated/note-empty-config-ownership-native-4.log` (session `95166`, compile green and the same runtime overflow with the complete futures heap-pinned), `🗑️generated/note-empty-config-ownership-native-4.ips` (triggered-thread constructor frames), `🗑️generated/note-empty-config-ownership-native-5.log` (session `31311`, registry regression 1/1 green, subsequent Note build stopped on unrelated shared generic bounds before runtime), `🗑️generated/note-empty-config-ownership-native-6.log` (session `15406`, Note compile and NoConfig law green, then camera dispatch overflow), `🗑️generated/note-empty-config-ownership-native-6.ips` (the new worker-session submission frames), `🗑️generated/note-empty-config-ownership-native-7.log` (session `90205`, shared carrier regression and Note NoConfig law green, both window laws reach the now-corrected missing-completion fixture guard), and `🗑️generated/note-empty-config-ownership-native-8.log` (session `74376`, all commands settled; remaining projection decoding and parallel runtime-id fixture faults corrected afterward).

## Note9 source handoff

Note9 is source-ready and has not been run. Native run eight remains the latest Note runtime evidence, so this report does not claim the three-law Note proof is complete. The post-run-eight corrections are confined to the test and helper module in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`:

- `settle` consumes the typed completion witness and routes result-page reads through the caller's exact runtime instance id;
- `note_app_with_registry_id`, `action_meta`, and `dispatch_with_view_for_instance` bind app admission, action metadata, and settlement to the same concrete instance id;
- the three concurrently selected laws use distinct ids `71_100` through `71_103`;
- the camera render assertion decodes `InkCanvasScene.document_json` before comparing each exact camera.

The isolated invocation for root is:

```text
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY='../🗑️generated/nx/note-empty-config-native-9' bun /Users/ueli/Documents/semio/node_modules/nx/dist/bin/nx.js run abstraction-ownership-validation:note-empty-config-ownership-native
```

The permanent ticket facade runs the neutral ownership and one-turn source oracles first, then invokes Cargo with the exact filter:

```text
cargo test --manifest-path Cargo.toml -p semio-s-artifact-note-note --lib note_empty_config_owner_ -- --nocapture
```

The intended captured native evidence path is `🗑️generated/note-empty-config-ownership-native-9.log`. The scoped `git diff --check` is green after the handoff corrections. No Cargo command was run after native eight in this agent turn.

## Note9 result and Note10 preparation

Root ran native9 on the configured 2 MiB stack. NoConfig admission/terminal retirement and the complete two-window camera, rendering, persisted partition reload and final-close law passed. The transient law reached its final close and failed the terminal-empty guard. This is two passing laws and one failing law, not a passing Note suite. Evidence: `🗑️generated/note-empty-config-ownership-native-9.log`.

The transient fixture retained four `WindowTransientSnapshot` read leases across replacement and final app closure. The public snapshot owns an `Arc<ErasedSnapshotRead>`; retaining it prevents the exact retired root from becoming uniquely disposable. The fixture now releases each read lease immediately after its assertions, before replacement or app close. Production retirement remains strict. Note10 is queued and this correction is not yet runtime-verified.

Native10 subsequently exited 0 on 2 MiB: all three selected Note laws passed, alongside the neutral/Ajv/JSONPatch/strict TypeScript prelude. The transient debug witness confirms isolation, reset on document reload, cancellation of an unpublished retained mutation and final terminal-empty registered app closure. The other two laws confirm NoConfig foreign-byte rejection and exact two-camera render/reload isolation. Evidence: `🗑️generated/note-empty-config-ownership-native-10.log`. This completes the selected Note empty-config/window proof; broader Note editor tests were not run.

## Exact file ledger

Core owner and lifecycle changes:

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`

Native and language-neutral proofs:

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — focused laws and their inline `context` helper module; there is no separate helper file.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️ownership/🟦️.ts`

Shared fixed-registry constructor correction and focused native regression:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-artifact-fixed-registry/🦀️.rs`

Shared worker-authority heap ownership and focused native regression:

- `🧰️framework/🔨️modules/🧵️job/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs`

Command generic substitutions (`NoteConfig` / `NoteConfigMutation` to framework `NoConfig` / `NoConfigMutation`), each `🦀️.rs` under `✏️editor/🎮️commands`:

- `⏩️nudge-selection-right-fast`, `⏪️nudge-selection-left-fast`, `⏫️nudge-selection-up-fast`, `⏬️nudge-selection-down-fast`
- `✏️set-pencil-width`, `➕️add-block`, `➡️nudge-selection-right`, `⬅️nudge-selection-left`, `⬆️nudge-selection-up`, `⬇️nudge-selection-down`
- `🎥️set-camera`, `👁️set-grid-visible`, `💬️engagement-input`, `💾️save-download`, `📋️duplicate-block`
- `📏️set-grid-spacing`, `📐️set-snap-grid-spacing`, `📤️engagement-submit`, `📥️load-request`, `🔢️set-grid-subdivisions`
- `🔭️set-camera-zoom`, `🔲️set-grid-opacity`, `🕹️nudge-selection`, `🖊️ink-apply-events`, `🗃️set-active-example`
- `🗑️delete-block`, `🚚️move-block`, `🚫️delete-selection`, `🧫️set-fixture-json`, `🧭️navigator-engagement-input`
- `🧲️set-snap-enabled`, `🧽️set-eraser-radius`, `🩹️patch-blocks`, `🪞️duplicate-selection`
- `🗃️set-active-example/🧪️tests/🔬️unit/🦀️.rs`

Retired local empty configuration family under `✏️editor/🎚️config`:

- `🦀️.rs`
- `🧪️tests/🔬️contract-vectors/🦀️.rs`
- `🧪️tests/🔬️unit/🦀️.rs`
- `🧫️fixtures/🔁️mutation-contracts.json`
- `🧬️schema/🔗️.graphql`
- `🧬️schema/🔣️.json`
- `🧬️schema/🛰️.proto`
- `🧬️schema/🟦️.ts`
- `🧬️schema/🦀️.rs`

Focused launch and Nx routes:

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`

Documentation-only owner wording:

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/🦀️.rs`

The workspace contains concurrent Note edits outside this ledger. They are not evidence for this slice and were left intact.
