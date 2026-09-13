# Exact Window-Config Provisioning Lifecycle Audit

## Finding

The current retained app and window-config APIs cannot initialize an exact window config through an explicit first-open lifecycle while preserving an existing or restored owner. This is a required shared framework and host follow-up for every registered exact window config. Generation3d makes the gap visible because its main owner needs a document-dependent camera seed, but default-seeded owners are subject to the same restore-order and eager-read problem. Generation3d production work must stop before the main renderer cutover until the shared seam is coordinated.

## Runtime Evidence

`Event::SurfaceVisible` enters `plugin_mount_surface`. That function decodes the host-owned `ViewModel`, narrows it to the concrete `window_id`, and stores only the body key and view context in `SurfaceContexts`. It does not call an app lifecycle hook or admit a window-config publication. The reactor then marks the surface dirty and later calls `plugin_render_surface`.

`plugin_render_surface` calls `PluginApp::render`. In `VcsArtifactApp::render`, the runtime calls `window_config_store.capture(Some(view_state))` before invoking `ArtifactApp::render_with_request_context`. The app therefore receives a `ConfigView` only after framework capture.

`WindowConfigOwnerRegistry::capture` resolves the exact `(window_kind_id, window_id)` and delegates to `TypedWindowConfigStoreOwner::capture`. Its `partition` method creates a missing partition immediately from `O::State::default()`. There is no non-materializing `peek`, `contains`, or absence result. By the time Generation3d app code can inspect `cfg.window`, an absent owner has become a default-valued existing owner.

`ArtifactApp` exposes registration, retained command, pending-effect, and render hooks, but no `window_open`, `surface_visible`, or pre-capture provisioning callback. `pending_effects` receives the document, application config, and an optional `ViewModel`; it receives neither the window-config registry nor an absence witness. Dispatching a synthetic command from render or a pending effect would occur after eager capture and would fail the required first-render and absence semantics.

## Actual Host Ordering

### React

`ShellHost.establishPrimarySession` creates the app, seeds the shell layout, and publishes the session. Its first `refreshUi` builds every requested window, panel, and reserved-section `surface-visible` event in `uiRefreshSurfaceEvents`; `PluginRuntime.refreshUi` submits that event batch immediately and then sends empty continuation turns until every previously missing retained surface publishes.

The React `PluginWasmHandle` exposes `readWindowConfigPacks` and `loadWindowConfigPack`, and the latter awaits the `LoadWindowConfig` command's `Done` frame. There is no production caller of either method. The only `loadWindowConfigPack` call in the repository outside the two bridge definitions is a PluginRuntime test. Consequently the current React order is:

1. `createApp` completes its retained instance-open lifecycle;
2. the shell publishes a session and layout;
3. the first `refreshUi` submits `SurfaceVisible`;
4. guest capture creates a default partition when it renders; and
5. no window-config restore occurs.

An external caller can manually establish `LoadWindowConfig` before calling `refreshUi`, because each awaited channel call settles before the next call begins, but the production shell neither performs nor enforces that sequence. The interface comment that says the load restores before first render describes a caller obligation, not current host behavior.

### WGPU

The TypeScript WGPU bridge follows the same order. `createApp` completes the retained instance-open lifecycle and installs its channel. `renderSurfaceSerialized` then submits one `surface-visible` event immediately and advances retained turns until that surface publishes. Its handle also exposes `readWindowConfigPacks` and `loadWindowConfigPack`, but there is no production caller.

The native WGPU shell is even narrower: `refresh_ui` walks live windows and panels and calls `ProgramBridgeEntry::render_with_document` for each. That method sends `Event::SurfaceVisible` first and then uses bounded `AdvanceRetained` requests until the document appears. The Rust `ProgramBridgeEntry` does not expose a window-config restore method. Its utility bar is derived locally from the manifest after window and panel rendering. Therefore WGPU also has no restore-before-visible guarantee.

This is a real capability gap. A reopen law that claims a host-restored exact owner cannot be accepted until both hosts have an owner for the saved packs and await their load before the first `SurfaceVisible`, or until the actor protocol carries an explicit per-instance restoration-complete barrier that the first visible event can depend on.

## Every Eager Capture Path

`WindowConfigOwnerRegistry::capture` resolves the exact roster entry and registered kind, then `TypedWindowConfigStoreOwner::capture` calls `partition`. `partition` creates `O::State::default()` when the exact id is absent. The following production paths can therefore materialize an absent owner before a body render:

| Path | Current capture point | Consequence for a pending seed |
| --- | --- | --- |
| Immediate emit | `VcsArtifactApp::dispatch_emit`, when an emit contains a window-config mutation | Captures and defaults the target immediately before dispatch. |
| Retained typed command | `VcsArtifactApp::start_typed_command_operation` | Captures before constructing the immutable command-job context, even if that command later emits nothing to the window lane. |
| Diagnostic/helper query | `VcsArtifactApp::window_config_generation` | A generation read currently creates a default partition. |
| Window body and app panel render | `VcsArtifactApp::render` | Captures before both snapshot-override and ordinary render branches. A panel uses its focused window through the registry's address rule. |
| Window engagements section | `VcsArtifactApp::window_engagements` | Iterates up to `UI_RESIDENT_SLOTS` and captures each window before calling the app hook. React can request this reserved section in the same event batch as first visibility. |
| Window measures section | `VcsArtifactApp::window_measures` | Iterates and captures each window before calling the app hook. This includes utility-tagged options. |
| Tool measures section | `VcsArtifactApp::tool_measures` | Captures the addressed or focused exact window before calling the app hook. |
| Context menu | `VcsArtifactApp::context_menu` | Captures the addressed or focused exact window before computing menu items. |

The utility bars themselves are not guest-rendered surfaces. React calls `resolveUtilityNodes` from the app and window-kind manifests, and WGPU calls `derive_utility_nodes` locally after window rendering. They do not touch `WindowConfigOwnerRegistry`. Dynamic utility options travel in `window_measures`, and utility interaction commands travel through one of the two dispatch paths, so both are covered by the barriers above. The legacy JSON `plugin_refresh_ui` also says `request.utilities` is intentionally unhandled; it renders windows and panels and calls the same engagement, measure, and tool accessors, all through the capture paths listed above.

The reserved engagement/measure/tool surfaces matter to ordering. React submits them alongside window bodies, and the reactor processes every `SurfaceVisible` before rendering dirty surfaces. A main-window provisioning candidate can therefore be scheduled before any section is rendered, but pending exact owners must make capture return a typed pending result instead of defaulting. A section may omit that one pending window and be dirtied again when the candidate reaches Ready; sibling windows and unrelated sections continue to publish.

## Required Capability

A coordinated framework change needs all of these properties:

1. An explicit app-instance restoration lifecycle admits zero or more saved packs and then records `RestoreComplete`. An exact `WindowOpened` lifecycle event runs after trusted window validation and independently of presentation visibility.
2. The window-config registry can query an exact `(kind, id)` without creating a default partition.
3. Every registered owner declares one initialization policy: a framework-default seed or a document-dependent seed. The window-open callback can return or begin one bounded exact `WindowConfigMutation` through the registered owner preparation/publication path.
4. A loaded/restored exact pack is observable as existing and suppresses provisioning. The lifecycle order must ensure pack restoration is admitted before first-open provisioning, or expose a restoration barrier that prevents a seed from racing a pending restored pack.
5. The callback is idempotent for repeated `WindowOpened` events and publishes only once for a genuinely absent exact owner. `SurfaceVisible` only attaches a presentation waiter to the already opened exact window.

The shared API must distinguish three capture outcomes: unregistered, existing authority, and provision-pending. Generation, body and panel render, sections, context menus, immediate dispatch, and retained-command capture must be non-materializing. An absent owner is initialized only by its explicit window lifecycle policy. There is no default-on-first-capture exception.

One viable schema-first shape is a declaration for every registered window kind plus one pure hook for document-dependent policies. A framework-default policy maps to the registered state default; a document-dependent policy maps the current immutable `ArtifactView` plus trusted narrowed `ViewModel` to at most one exact `WindowConfigMutation`. The hook only constructs the candidate. It does not dispatch a command, apply a mutation, run a publication loop, or render.

`InstanceOpened`, zero or more `LoadWindowConfig` events, and an explicit `RestoreComplete` establish authoritative absence for one app lifetime. `WindowOpened` is then the only scheduling event, keyed by `(app instance, window kind, window id)`. A window opened before `RestoreComplete` remains in a bounded waiting-for-restore state; the barrier either observes a restored owner or reserves absence and selects the declared seed. This cannot be overloaded onto `SurfaceVisible`: React can ask for commands and reserved engagement, measure, and tool sections before or in the same batch as body visibility, while a body can be hidden and shown repeatedly without closing its logical window.

After `plugin_mount_surface` validates and narrows a visible view, `SurfaceVisible` attaches the concrete surface binding to the existing owner, reservation, or waiting candidate. Concrete and leftover-alias surfaces for the same window share that candidate and retain their own surface-generation waiters. Repeated visibility with the same binding adds or refreshes a waiter; it never opens the window or constructs a seed. `WindowClosed` requests cancellation of an unfinished candidate, while `SurfaceHidden` only removes the presentation waiter.

The retained candidate should carry:

- the exact owner key and fixed-capacity waiting-surface set;
- the app lifetime and each surface binding generation;
- the captured document generation and content revision used to build the authored seed;
- an exact absence reservation in the window-config registry;
- the typed mutation and its `ErasedWindowConfigPublication` once begun;
- a cancellation lease and terminal state; and
- retained rejected/stale publication owners until their bounded close reports terminal emptiness.

The registry needs an atomic `reserve_exact_absent` operation. It reports Existing when a partition was loaded or already authored, Reserved when this candidate won absence, and Pending when another candidate owns the same exact absence. Beginning the publication may create the internal default partition, but the reservation prevents any renderer, section, helper, or command from observing that temporary default. The registered Snapshot mutation then publishes the authored seed through the existing `begin`/`advance` path.

The reactor already has a natural advancement point. It handles all inbound events first, then command ingress and intent dispatch, advances retained typed operations, and only afterward iterates dirty surfaces and calls `plugin_render_surface`. Add one bounded round-robin provisioning advance after the event batch and before command or intent capture, and collect readiness again immediately before dirty rendering if that slice completed. Each call spends a fixed one-item store grant and a fixed wall/fuel budget; it never calls `resolve_ready` to drive the complete lifecycle. `MoreWork` includes runnable provisioning and retained cancellation/cleanup work, so React's existing `settlePluginTurn` empty continuations and WGPU's existing `AdvanceRetained` loop naturally provide later slices.

Dirty rendering must check the exact candidate state. A pending window surface is retained as waiting and omitted from this turn's render set. When publication becomes Ready, the reactor re-dirties only its still-live surface bindings plus the reserved engagement, measure, or tool sections that omitted the pending window. Unrelated window and panel surfaces render in the original turn. A pending-focused panel, context-menu request, diagnostic generation query, or command receives the typed Pending outcome; section aggregation skips only that exact row. Command ingress aimed at it remains retained for a later turn rather than fabricating a default or returning a semantic rejection. Any other instance continues normally.

`WindowClosed`, app close, document generation or revision change, and a restored pack racing the candidate request cancellation. `SurfaceHidden` and surface rebind only remove the old presentation waiter; the logical window candidate keeps advancing, so a command or section request can remain pending and a later surface can bind to the same result. Cancellation does not drop the candidate or its rejected store publication. The state machine advances it through cancellation and bounded close, then releases its exact reservation. If the logical window remains open after a document revision change, a fresh candidate is scheduled from the new authored snapshot. If `LoadWindowConfig` arrives before commit, it wins: it cancels the seed, retains both candidates through cleanup, and publishes the restored envelope only through the retained exact-Pack load capability. If seed publication commits first, a later restored load revalidates its live-generation witness before an atomic replacement. The first dependent surface remains gated until the load's terminal receipt or restoration barrier has settled.

The host part is required. React persistence must load every saved exact pack, await all `Done` frames, and submit `RestoreComplete` before any `WindowOpened` for that app instance. A later `SurfaceVisible` can bind the body or section without changing owner state. WGPU needs an equivalent ProgramBridge restore and window-lifecycle API with the same ordering in session construction. Hosts with no saved packs must explicitly complete the barrier so absence is authoritative rather than merely “restore has not arrived yet.” Repeated restore-complete and window-open events are idempotent and scoped to the app lifetime.

The generic inner `WindowConfigPack` identity rejection remains a separate Store capability slice. The concrete open RED counterexample loads an outer target pack for window `target` with the same registered kind `identity-window` and schema `test.window.identity`, while its decoded inner partition id remains `window-config:identity-window:source`; the current loader admits it because it checks the schema but not that exact inner id. The delimiter counterexample similarly targets `scene:two` while the inner partition remains `window-config:identity-window:scene:one`. Component, schema, and version also remain closed envelope facets. Provisioning must reject both foreign inner partition ids, retain any rejected decoded candidate until bounded close, and must not weaken or synchronously drop the open RED law.

## Neutral Contract

The closed draft-2020-12 schema, fixture, and TypeScript oracle now live together under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config` in the `🧬️schema/🚪️provisioning-lifecycle`, `🧫️fixtures/🚪️provisioning-lifecycle`, and `🧪️tests/🚪️provisioning-lifecycle` taxonomy branches. The fixture has ten scenarios and models both declared seed policies, both exact-owner kinds with two concrete instances, all nine eager capture paths, repeated lifecycle and visibility events, pending and existing outcomes, stale document cancellation and rescheduling, restore races, a hidden but logically open command target, pending-section sibling progress, surface rebind, window close, instance close, a host-saved pack restoring a reused id in a new app lifetime, and exact foreign inner partition ids.

Ajv 2020 validates the closed fixture and rejects hostile event and expected-state fields. The retained reducer checks invariants after every event, while fixture-authored operations feed an independent `fast-json-patch` oracle for every final summary field. `bun nx run workspace:window-config-provisioning-lifecycle-oracle` passes through the registered root Nx route and reports `scenarios=10 capturePaths=9 policies=2 readMaterializations=0`; the durable ticket log is 281 lines and 9,649 bytes. The root and ticket Nx facades expose that route, and root and seed launch files register it at `311.240`. Order `311.241` remains reserved for the retained exact-Pack load native law described in `window-config-retained-exact-pack-load-capability-design.md`.

## Proposed Delivery Slices

1. Define a language-neutral provisioning lifecycle fixture and closed schema before framework code. The fixture covers restored-before-open, an explicit empty restore barrier, repeated open and concrete/alias visibility, framework-default and document-dependent policies, two same-kind windows, two window kinds, a document revision race, load racing a begun candidate, hidden and rebound waiters without logical cancellation, app close, every eager capture path, host-saved packs across app lifetimes, and exact foreign inner partition ids.
2. Implement an independent TypeScript oracle that models exact absence reservations, bounded advancement, surface gates, stale candidates, and terminal cleanup. Validate its state after every fixture event.
3. Add the non-materializing registry result and exact reservation tests. Every registered owner initializes through the explicit lifecycle; no capture path may expose or create an internal default.
4. Add `RestoreComplete`, `WindowOpened`, and `WindowClosed`, the per-kind policy declaration/hook, and the retained candidate owner with fixed capacities, one-item grants, cancellation leases, and close proofs. Add its reactor scheduling and `MoreWork` integration without a synthetic command or `resolve_ready` loop.
5. Make body, section, context-menu, helper, immediate-dispatch, and retained-command capture consume Existing/Pending explicitly. Test that only the exact pending owner waits.
6. Add React persistence/barrier ordering and WGPU ProgramBridge parity, then prove that the first surface event follows successful restore or explicit empty-restore completion.
7. Only after those framework and host laws pass, implement the three Generation3d exact camera owners and their approved seed semantics.

Acceptance requires the neutral oracle and native tests to agree on every step, two concrete instances for every owner kind, cross-kind isolation, stale and wrong-kind rejection, no first render before readiness, unrelated-surface progress, restored-owner priority, repeated-event idempotence, bounded cancellation and close, and stable document and application Pack+SPR bytes.

## Generation3d Seed Semantics

The seed source is the current authored `document.fixture.camera` at the moment an absent `procedural-main` instance opens. Existing and restored exact configs take priority.

`SetActiveExample` currently builds widget, synapse, layout, and schema artifact mutations through `generation3d_fixture_operations`. That function has no camera branch, and its `fixture_ops_ignore_camera` law fixes this behavior. The command separately copies `target.fixture.camera` into the application `Generation3dConfig.camera`, which the current main renderer does not read.

After removing the disconnected app camera field, `SetActiveExample` must continue to leave the authored document camera unchanged. A main window opened after an example switch therefore seeds from the unchanged document camera, not from the selected example's camera. An example camera becomes a seed only if another explicit document-level operation authors it. No such new operation belongs in the exact window-owner cutover.

## Bounded Follow-Up Test

Once the framework seam exists, the Generation3d native law should demonstrate this order directly:

- load an exact restored main pack, then deliver the first window-open event and observe no publication;
- open a truly absent main instance and observe one bounded WindowConfig publication before its first render;
- repeat visibility and observe no second publication;
- change the document camera and confirm the existing instance remains unchanged while a later absent instance seeds from the new document value; and
- run `SetActiveExample`, confirm document camera and all existing window packs stay byte-identical, then open another absent main and confirm it seeds from the unchanged authored document camera.

This audit made no Generation3d production or shared framework change.
