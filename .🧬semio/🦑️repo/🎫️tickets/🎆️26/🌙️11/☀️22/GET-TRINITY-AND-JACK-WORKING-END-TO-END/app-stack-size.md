# Mounted app stack size: boxed owners

Status: design and code landed; verification state is listed at the end ("Verification").

## Problem

`VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, SemioMembers>` was 927,376 bytes inline. Every test future owning one was about 2.1 MB, so default 2 MiB test threads overflowed in debug, and the Trinity dev graph app-boot tests needed 64 MiB.

## Size breakdown (const-eval probes, `cargo check`, same type)

| Field | Before (bytes) | After |
|---|---|---|
| `tools` (`ToolOperations`) | 271,040 | boxed (8) |
| `pending` (`PendingReserved`) | 190,624 | boxed (8) |
| `time_travel` (`TimeTravelLedger`) | 167,504 | boxed (8) |
| `tool_machines` (`ToolMachineRuntime`) | 80,880 | boxed (8) |
| `stores` (`InstanceStores`) | 44,112 | boxed (8) |
| `host_events` (`HostEvents`) | 26,960 | boxed (8) |
| `child_composition` | 25,392 | boxed (8) |
| `constructor` (`ConstructorState`) | 24,096 | boxed (8) |
| `media` (`MediaTransfers`) | 23,072 | boxed (8) |
| `original_emission` | 21,984 | boxed (8) |
| `original_typing` | 14,048 | boxed (8) |
| `original_typed_command` | 11,440 | boxed (8) |
| `original_host_event_command` | 11,440 | boxed (8) |
| `roster`, `close`, `interaction`, `ingress`, `history`, `ingest`, `tool_runs`, `txn`, scopes, policy | about 15 KB together | inline |
| `A` (`EditorApp<TrinityJackPlayApp<..>>`) | 24 | inline |
| whole `VcsArtifactApp` | 927,376 | 64,704 after the first eight boxes, then about 14.9 KB after the last three (see Verification for the measured final value) |

`AgentLanePreview` (232 KB), `ConfigurationWorker` (128 KB) and `TimeTravelStoreState` (112 KB) are not separate owners: they are fields nested inside `ToolOperations`, `PendingReserved` and `TimeTravelLedger`, so boxing those owners removes them from the handle.

## Design

- `VcsArtifactApp` keeps its public fields; the thirteen owners above become `Box<..>` fields. Field access, method calls and disjoint field borrows are unchanged (a `Box` field auto-derefs and stays borrow-splittable), so no consumer needed an edit.
- `VcsArtifactApp::heap_bytes()` (const) is the inline handle plus every boxed owner; the bounded-retention accounting (`birth_demands` capacity, close `release_bytes`, original recipient frame demand) uses it instead of `size_of::<VcsArtifactApp>()`.
- In-place original constructor (`🧱️construction/🧺️sources`): the admission now carries `OwnerFrames`, one `Box<MaybeUninit<Owner>>` per boxed owner, allocated beside the mounted frame in the same paid allocation step (`🏭️factory`). Each staged phase (`constructor`, `stores`, typing, emission, typed commands, host events, child composition, pending, tools, media, time travel, tool machines) initializes its owner field by field through the unchanged `initialize_field_at` cursor, now pointed at the owner frame (`owners.x.as_mut_ptr()`) instead of `addr_of_mut!((*target).x)`. Phase 26 installs the finished boxes into the mounted frame (`OwnerFrames::install`) and then `assume_init`s the frame, so nothing is built on the stack and moved.
- Reference constructor (`with_registry_on_bus_boxed`, `📥️staging/🚪️ingress`): each owner is built by a non-inlined `boxed(..)` helper, so the owner lives in its own short stack frame and straight into its box, never in the constructor's frame or future.
- `OriginalTypedCommand<A>` is a type alias exported by the instance app crate so the construction crate does not need a new dependency.

## Regression law (language-agnostic)

- Fixture `🧪️tests/🧫️fixtures/📏️mounted-app-size/🔣️.json` (plugin tests): contract id, `stackCeilingBytes` 65536 and per-type `maxBytes` budgets (`composedParentVcsApp`, `composedParentApp`, `trinityJackEditorVcsApp`, `trinityJackEditorApp`, `trinityApps`). Any implementation language can read it.
- Rust laws asserting `size_of` against the fixture and each budget against the ceiling:
  - framework: `mounted_app_handle_stays_within_the_recorded_size_budget` (part-1-x fragment `📏️mounted-app-size`), also asserts `heap_bytes()` accounts the boxed owners;
  - Trinity dev graph: `mounted_jack_app_stays_within_the_recorded_size_budget` (`✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/📏️app-size`);
  - Trinity hub: `trinity_apps_enum_stays_within_the_recorded_size_budget` (the closed `TrinityApps` enum holds its largest variant inline).
- The laws run on the default test stack; no `RUST_MIN_STACK`.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs`
- `.../🧵️instance/🧱️construction/🧺️sources/🦀️.rs`, `🏭️factory/🦀️.rs`, `📦️original/🦀️.rs`
- `.../🧵️instance/📥️staging/🚪️ingress/🦀️.rs`
- `.../🔌️plugin/🧪️testing/🧪️artifact-app-laws/` (two construction docstrings)
- `.../🔌️plugin/🧪️tests/🧫️fixtures/📏️mounted-app-size/🔣️.json`, `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/📏️mounted-app-size/🦀️.rs` and its `🦀️.rs` include
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🦀️.rs` and `📏️app-size/🦀️.rs`
- `🌎️hub/🧩️compositions/🔱️trinity/🧪️tests/🔬️surface/🦀️.rs`

## Verification (dev graph test exe, `G:	\szb\debuguild\semio-s-dev-trinity-jackea526966c35b768\out\...exe`, debug, default stack, no `RUST_MIN_STACK`)

- Build: `cargo test --no-run` of `semio-s-dev-trinity-jack` compiled and linked with the 13 boxed owners (EXIT 0). The hub and `semio-plugin-tests-08` crates were not compiled (their new size laws are unverified); the first `cargo check` caught a missing-crate path in the construction crate, fixed with the `OriginalTypedCommand<A>` alias.
- Size law `mounted_jack_app_stays_within_the_recorded_size_budget`: passes (the handle is within the 16 KiB budget).
- Each of the 75 non-layout tests run alone on the default 2 MiB stack: 35 pass; 40 still overflow (all mounted-app tests, e.g. `native_composition_parameterizes_actual_registered_members`, `renders_jack_editor`, the history and rail-verb laws).
- Stack sweep for `native_composition_parameterizes_actual_registered_members`: overflows at 2, 4 and 8 MiB, passes at 16 and 32 MiB. Before the change it needed 64 MiB, so the requirement fell by 4x but the default stack is not reached.

## Open: remaining stack consumer

The app value is no longer the cause (it is now small). The remaining 8 to 16 MiB is in the construction call chain itself: debug builds do not overlap async state, so the nested `with_registry_on_bus_boxed` future (stores, envelopes, config/draft/interaction stores, tool registries held across awaits) and the sync temporaries inside `boxed(..)` helpers add up. Finding it needs `-Zprint-type-sizes` (largest `async fn` futures and frames) over the framework and dev graph crates, which is a full rebuild; it was not started because of the serialized-link policy. Next steps: run that, box or split the largest construction futures (`Box::pin` the isolated sub-awaits in `with_registry_on_bus_boxed`, build `InstanceStores` and `ToolOperations` through in-place initializers instead of by-value `new`), then rerun without `RUST_MIN_STACK`.
