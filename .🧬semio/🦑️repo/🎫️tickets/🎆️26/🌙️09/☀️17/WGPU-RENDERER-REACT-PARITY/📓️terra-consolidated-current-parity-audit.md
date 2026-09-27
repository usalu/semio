# Consolidated Current WGPU Parity Audit

## Method and limits

Read-only source review on 2026-09-26. I used `rg` and direct reads. I did not run a build, native executable, browser, or test command, so this report does not claim runtime success.

The in-flight Table, DiffView, TextEditor, Dock/chrome accessibility, generic Toggle, and Marketplace work is excluded from implementation recommendations.

## New TextEditor Space Regression Review

The new regression is at:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1141-1187`

Its imports, fixture path, private-crate access, and `Waker::noop` polling form are sound. It now correctly asserts the complete `textSelect` descriptor and uses a unique window/surface id. Its focused and unfocused Space cases exercise the desired routing order.

The primary-pointer blur helper is also sound:

`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1585-1598`

It retains focus only for a primary press on the exact mounted TextEditor target, including window generation, node, and host id; secondary presses and pointer-up leave focus untouched.

### Lifecycle recheck

The direct `sync_engine_scene` Space law still has a test-only retirement wrapper, so it is not a production lifecycle proof. A separate phase-four law now fills the required narrower gap:

`🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:511-542`

It paints each editor through the retained-document production entry `render_ui_document_step` (`🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:325-349`), asks the interpreter to close the first window, and proves its token and slot are empty while the sibling remains bound. It then paints a successor in the closed window and proves that its generation and text do not alias either predecessor.

The fixture bridge is deliberately CPU-only, but it verifies the exact token captured in the close request, performs the registered engine retirement, verifies the slot is empty, and only then publishes terminal acknowledgement:

`🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:38-49`

No source fault was found in that law. Its boundary is precise: it proves retained-document admission and CPU `EngineCanvas` retirement; it does not replace a native `AppPresenter`/`ComponentSurfaceCloseOwner` GPU-retirement test. The production presenter ladder remains separately owned at `🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:375-414,694-718`.

## Current-source corrections to older reports

Several high-value reports in this ticket no longer describe the current source and should not be reopened as implementation work.

| Earlier concern | Current source finding |
| --- | --- |
| Per-document component engine hosts leaked on close | The token-bound component-close ladder described above now exists. |
| InkCanvas clipboard had no WGPU producer/parser | WGPU now has copy/paste construction, stream handling, stale-address rebase, native dispatch hooks, and shared fixture laws. See `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7227-7400` and `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1916-2124`. |
| EventFeed always formatted UTC | Current EventFeed state owns `EventFeedTemporalPresentation`, and the host-I/O test/contract route exists. The old UTC-only audit must not be used as current evidence. |
| Command-palette editable control suppressed keyboard semantics | Current browser input wire has the explicit editable-combobox key route and the mirror models the corresponding ARIA state. |
| Tutorial snapshots omitted selection/tree/dialog handling and scene-addressed gesture locations | Capture/apply now carries interaction selection, tree state, dialog state, and command-panel state. Gesture resolution now covers Scene, Canvas, Entity, Curve, and Domain at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23567-23721`. |
| Driver editor had only a selector | The current Shell has all seven axes, transient draft, save/delete routes, and localized controls at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1444-1451, 8317-8411, 10848-10875`. |
| Theme editor used page controls instead of React's expandable scrolling tree | The active builder is now documented as independently expandable and virtualized at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8312-8315`; orphaned page strings alone do not establish a behavior gap. |

## Remaining parity gates

The shared 15-kind contract remains at:

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`

Each kind has a production WGPU dispatch route and source-level regression location. That is not live acceptance. The current browser parity runner treats component scenes as visual leaves and only runs generic state/shell probes:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:146-205`
- structural leaf rule: `…/🏗️structure/🟦️.ts:163-185`
- probe scope: `…/🔬️probe/🟦️.ts:333-344`

The highest-value unresolved full-goal work is an app-backed `surface-behavior@1` acceptance fixture for every produced surface kind. It needs one deterministic app document, concrete input sequence, visible/app-state oracle, and action receipt for each row. Do not count a Storybook host, a Rust unit law, an old activation, or a component raster comparison as a completed renderer journey.

The minimum live rows are:

| Family | First journey |
| --- | --- |
| Canvas2d | Draw: modifier gesture, cancel, double click, and state consequence. |
| World3d | Puzzle3d/CAD: orbit, wheel, object pick, and context menu. |
| NodeGraph | DAG/Flow: wire creation or selection plus pan/wheel. |
| Paint2d, TiledMap, Board2d | One direct manipulation and one camera action each. |
| VFS, GraphTimeline, IconRender | Selection/scroll/context action appropriate to the seeded document. |
| BlockList | The existing Playbook builder: add, remove, and one drag/drop receipt. |
| InkCanvas | Note: edit, clipboard, pointer cancellation. |
| DiffView | Shell conflict detail: select an open conflict, inspect its preview, then accept or discard it. |
| EventFeed | First provide a real fixture producer; no static product construction was identified in this pass. |

## Current Virtual File System Recheck

The following current-source recheck supersedes the historical snapshot retained below for audit provenance. Do not reopen its hierarchy, activation, or descriptor-decoder claims without rereading the sources named here.

The two renderers now agree on the raw-node scene input. React's `VirtualFileSystemHost` derives the visible projection from raw rows, seeds expandable roots, toggles its local expansion set, and dispatches `navigateUri` double-click actions at `🧱️elements/🗣️Interpreter/🟦️.tsx:844-880`. WGPU derives the same projection and owns the same local expansion/selection behavior in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9513-9642,9755-9878`. The descriptor binding also deliberately preserves authored object order through `VfsFileNodeKinds`, matching React's first-binding walk (`⚙️VirtualFileSystem/🟦️.tsx:196-205`; WGPU `:9411-9463`).

The descriptor union and avatar path are now present in current source. WGPU accepts only matching text/time/avatar tags, uses the descriptor's `format`, requests accepted host-temporal labels, keeps ISO text until a matching reply is visible, and uses the exact image source as its rounded-raster cache key before falling back to the React-equivalent initials (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9624-9669,9755-9885`). The shared descriptor fixture has a native decoder test at `🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs:167-198`. This audit did not run it. The current test is a decoder oracle; a native/browser accepted-frame temporal reply and visual-label oracle remains the verification boundary.

The remaining concrete VFS gap is localized scene chrome and its future virtual accessibility projection. React resolves `ui.common.name`, `ui.common.noFileSystemNodes`, `ui.common.expand`, and `ui.common.collapse` at the component boundary (`⚙️VirtualFileSystem/🟦️.tsx:495-499,535-549`). WGPU still draws literal `"Name"` and a literal empty fallback and has no typed chevron name at `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9770,9811,9836-9846`.

### Minimal localization boundary

Keep these texts out of `VirtualFileSystemScene`: it is a shared document payload, while React correctly obtains its common chrome through `useLabel`. Also do not add a second scene-local WGPU dictionary. Add an immutable, presentation-time `SceneChromeLabels` value to `SceneEngineHosts`. The Shell owns construction because it owns the active locale; it resolves the four values from its existing chrome lookup, whose source notes point to React's `uiChromeTranslationBundles` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:29287-29315`; React strings at `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2853,2877,2893-2894,3774,3798,3814-3815`). `SceneEngineHosts` is created at Shell retained-document boundaries `:5067,20963,25406,25807,26023,27646`; fixtures must supply an explicit English or German label pack.

Thread that value only through `render_component_scene_step` into `render_vfs`, and use it for the header, absent-message fallback, and the expanded/collapsed chevron presentation record. That preserves locale ownership at the Shell, keeps it explicit in tests, avoids widening `WidgetContext`, and provides the real label authority a later accepted-frame VFS accessibility node needs.

Do not add BlockList's `Steps`, `No steps`, or `Palette` to this packet. React renders `steps`, `addStep`, and the generic absent-scene label but no palette heading or in-scene `No steps` copy (`🧩️BlockListHost/🟦️.tsx:199-201,221-257`). WGPU's three literals (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4499,4508,4519`) are non-parity decorations and should be removed or introduced into the React contract in a separate slice. Generic `render_placeholder("kind host")` text is likewise an absent-scene contract, not VFS chrome.

## Historical Virtual File System Snapshot (Superseded)

This pass found an independent, user-visible renderer defect in the registered `virtual-file-system` surface. The stored `VirtualFileSystemScene` has opaque `schema_json` and `rows_json` fields (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1908-1951`), but the two renderers assign incompatible meanings to `rows_json`.

React's `VirtualFileSystem` accepts an already-flattened list of visible `VirtualFileSystemRow` values. A row has a required `level` and optional `isExpanded`; `buildVirtualFileSystemVisibleRows` is a separate helper that a caller must use before passing `rows`. The component then renders `data={[...rows]}` directly. See `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🟦️.tsx:124-145,399-425,563-580`.

`VirtualFileSystemHost` parses `scene.rowsJson` and passes it unchanged. It supplies no `onToggleExpand` and no `onRowDoubleClick`: `🧱️elements/🗣️Interpreter/🟦️.tsx:820-866`. The DOM chevron can therefore invoke only an absent callback; a `navigateUri` row has no host double-click action. The React component does have the correct independent controls when an owner supplies them: its chevron is a native labelled button and calls the optional callback at `⚙️VirtualFileSystem/🟦️.tsx:535-549`.

WGPU instead parses the same JSON as a raw parent/child tree, derives a visible list, seeds root expansion, and owns its expanded-id state locally. Its paint, hit test and shift-range selection all use that derived list: `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9464-9526,9581-9608,9637-9716`. It also treats a repeated row press as `navigateUri` activation (`openInstance`, `exportMedia`, or `navigateVirtualFileSystemNode`), at `:9612-9634`. Current WGPU tests prove those native behaviors, while the React scene host has no corresponding realization test: `🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs:53-102`.

A raw `[{id:"folder",hasChildren:true},{id:"child",parentId:"folder"}]` payload consequently produces a collapsible nested child in WGPU but two direct rows in React; clicking the WGPU chevron hides the child, while clicking React's chevron cannot change the host. Conversely, treating the payload as a pre-flattened React row set gives WGPU a second hierarchy interpretation. This is a contract disagreement, not a paint-only difference.

The descriptor-cell decoder is also incompatible. React defines the values as the tagged union `{presentation:"text",text}`, `{presentation:"time",iso}`, and `{presentation:"avatar",name,icon?}`, and validates tag agreement before rendering text, a formatted ISO time, or an avatar: `⚙️VirtualFileSystem/🟦️.tsx:55,257-300`. WGPU reads only `presentation` from the descriptor schema, serializes every object cell to JSON, and for a time only attempts numeric parsing: `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9385-9415,9539-9560`. Thus the normal React text, ISO-time, and avatar values paint as JSON rather than their presentation. The current native VFS suite has pointer and glyph laws but no descriptor-union paint oracle.

The same scene also has a current localization mismatch. React resolves `ui.common.name`, `ui.common.noFileSystemNodes`, `ui.common.expand`, and `ui.common.collapse` through `useLabel` (`⚙️VirtualFileSystem/🟦️.tsx:487-491,535-549`). WGPU paints literal English `"Name"` and default `"No file system nodes"`, and gives its chevron no semantic name (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:9655-9697`). The VFS schema fixture should therefore carry English and German label expectations; do not duplicate a second scene-local dictionary.

### Required schema-first fix boundary

Replace the opaque, underspecified VFS presentation at the shared scene boundary with one bounded owned shape. It must say whether the transport contains raw nodes plus `expandedIds`, or already-visible rows; it must not leave one renderer inferring hierarchy. It must also own the tagged descriptor union and its `text`, ISO `time` (including `date`, `datetime`, and `relative` presentation), and `avatar` fields.

If raw nodes are canonical, React's scene host should own an ephemeral expanded-id set, derive rows with the existing helper, and dispatch the same activate/double-click verbs WGPU does. If visible rows are canonical, WGPU should consume the provided level/expanded projection and ask the document/action owner to change it. Pick one, use it in both renderers, and remove the other interpretation; this greenfield contract must not retain both shapes.

Time presentation requires a locale/timezone/now authority. Reusing a system UTC formatter would repeat the retired EventFeed defect. The existing accepted-frame EventFeed temporal request/reply design is useful evidence for ownership and stale-reply handling, but its timestamp-only schema is not automatically the VFS ISO/relative-time contract.

Add one neutral fixture with an expandable parent and child, text, valid ISO time, malformed ISO time, and avatar cells. Its dual-renderer acceptance must assert:

1. initial visible row ids, levels, and expansion state;
2. chevron activation changes only the specified accepted surface generation;
3. selection range follows the visible order after expansion;
4. the same `navigateUri` double-click receipt in both hosts, or its deliberate removal in both;
5. text, time, invalid-time fallback, and avatar accessible/visible labels; and
6. stale focus or late accessibility activation for a hidden child is refused.

The natural current test seams are the React VFS component suite (`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:9482-9600`), WGPU `wgpu-virtual-file-system`, and a new app-backed browser route rather than a Storybook-only assertion.

## Registered Surface Accessibility Census

`Component::Surface` itself is correctly modelled as a focusable `application` record by the shared projection (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:82-143`). That names and focuses the canvas-level surface when its owning document supplies a label. It does not expose the controls painted inside a component scene.

The current WGPU document publisher adds scene-specific virtual accessibility children only for Table steppers and TextEditor textboxes. `build_accessibility_dump` calls exactly `append_table_stepper_accessibility_nodes` and `append_text_editor_accessibility_nodes` (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:4091-4263`), and its addressed event dispatcher only has matching scene-specific handling before the generic retained route (`:979-1050`). This means VFS row/chevron controls, EventFeed entries, GraphTimeline checkpoints, BlockList add/remove/palette controls, and the other scene-internal controls have no virtual node in the browser accessibility mirror even where their WGPU paint code registers a hit target.

Do not solve that deficit by publishing all generic hit records as buttons. A hit carries neither the React role nor a localized name, selected/expanded semantics, nor a stable accepted-scene resolver. Start with the VFS fixture above: publish bounded accepted-frame virtual entries for its rows and chevrons, resolve activation against the current scene by stable row id, and retire focus when filtering/collapsing removes the entry. Then extract a shared scene-semantic-entry contract only when a second surface proves the same fields sufficient.

## Prioritized next slice

The Virtual File System presentation boundary is the strongest independent production slice. It has a concrete React/WGPU disagreement across hierarchy, expansion, activation and descriptor cell presentation; it does not overlap the in-flight Table, DiffView, TextEditor, Dock/chrome, Toggle, or Marketplace work. Start with its shared neutral fixture and one canonical presentation owner, then add VFS virtual accessibility entries as the same accepted-frame slice.

The component-engine lifecycle receipt remains the next verification slice after VFS. It is bounded, protects the 256-slot engine registry under normal document closing, and verifies the exact close authority rather than only a document queue.

After that receipt exists, the next substantive parity work should be a real dual-renderer surface journey, starting with Draw Canvas2d or World3d. The choice should follow availability of a fresh activation receipt; neither may be claimed complete from the existing source tests.

## TextEditor Correlated Action Receipt Boundary

### Verified receipt contract

React treats an editor emission as an ordered pair, not as two independently
forgettable descriptors. Its coalescing dispatcher sends `textEdit { surfaceId,
text }` first and sends `textSelect { surfaceId, start, end }` only after that
edit has resolved successfully (`🧱️elements/✏️TextEditor/🟦️.tsx:226-256`). A
refusal removes the exact pending text, may restore the last authoritative
value, and does not send that pair's selection. The echo reconciler similarly
matches pending text values in order (`:113-152`). It is therefore incorrect
to recover a completion by comparing `controllerId`, action, or arguments:
two editors and two sequential edits can have indistinguishable descriptors.

WGPU now writes the same descriptor shapes in
`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`'s
`emit_text_editor_actions`, but it has no identity left with which to settle an
accepted, refused, or cancelled emission. The input queue materializes every
`BoundedAction` to the bare descriptor at
`🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs:262-266`; renderer transfer then
stores only `ActionDescriptor` in `FrameActionOwners` and
`FrameDeferredWork::Action` (`🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10407-10629`).
The Shell's `dispatch_gesture_action` correctly receives and returns only a
domain action (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10776-11465`). The identity
must survive up to that call, then return along the same deferred path.

### Recommended narrow seam

Keep the public action schema unchanged. Add a non-serializable internal
envelope to the fixed WGPU queue:

```
ActionQueueReceipt {
  token: NonZeroU64,        // opaque renderer-local emission identity
  member: u8,               // emission member ordinal
  abort_correlation_on_error: bool,
}

FrameActionEnvelope {
  descriptor: ActionDescriptor,
  receipt: Option<ActionQueueReceipt>,
}
```

`BoundedAction` owns the optional receipt and converts to the envelope rather
than directly to `ActionDescriptor`. Add a tagged form of the existing bounded
batch reservation in `InputState`; all present `reserve_actions` callers keep
using the untagged method. Only `emit_text_editor_actions` reserves the tagged
one, using one opaque token per edit/select pair and member ordinals zero and
one. A selection-only update is a one-member token. The generic action queue
does not know that either member represents text editing.

Preserve the envelope in `FrameActionOwners` and
`FrameDeferredCursor`; unwrap `descriptor` only at
`Shell::dispatch_gesture_action`. Its `Ok` or `Err` settles the exact receipt.
On an error in a member marked `abort_correlation_on_error`, remove only
unstarted envelopes with the same token and settle them as cancelled. This
prevents a refused edit's selection from dispatching, without payload matching
or cancelling an identical edit from another editor. Existing direct
`FrameActionOwners::try_push(ActionDescriptor)` callers remain untagged.

The rendering engine owns a fixed-size `EditorReceiptLedger`, indexed by the
opaque token and recording the source `EngineSurfaceToken`. Its settlement
first verifies that token's engine surface identity is still current, so a
late result cannot change a successor that reused the registry slot. A source
close removes the ledger entry immediately; an already runtime-owned action
can still reach Shell, but its late local result becomes a no-op. This retains
the established user-commit law while preventing retired-editor resurrection.
`Ok` means dispatch acceptance only; it is not a document echo. The later
coalesced local-echo layer must remain responsible for reconciling an
authoritative scene update without regressing a newer local edit.

The token must be allocated only after bounded capacity can be reserved and
before the tagged batch is published. If any subsequent write or publication
step fails, retire the ledger entry and restore or resync the local editor
before returning the failure. This avoids an in-flight record for an action
that never entered the queue.

### Why not a skipped descriptor field

`ActionDescriptor` is the shared serialized domain schema
(`🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:25-34`), with serde and value
derives and generated manifest/TypeScript consumers. A search at this audit
point found 313 Rust struct literals across the framework (192 under the
renderer engine). A `#[serde(skip)]`/`#[value(skip)]` origin field still forces
those constructors to supply a renderer-transient field, changes generic
clone/equality/debug state, and offers no proof that a copied descriptor is
the original queue slot. Keeping the receipt in the queue envelope confines
the new data to the WGPU input and deferred-frame implementation, leaves the
wire payload untouched, and makes loss/cancellation paths observable.

### Required cancellation and retirement rules

1. A candidate supersession must retain tagged input actions exactly as it
   already retains untagged runtime-owned user commits; the eventual completed
   deferred frame dispatches each one once.
2. Queue-transfer failure, deferred-cursor close, or an explicitly discarded
   unstarted envelope must settle its receipt once as `Cancelled`. Do not leave
   the editor's receipt lane permanently in flight.
3. A Shell error settles only its envelope as refused and, when the receipt
   requests it, cancels only the remaining members with that same token.
4. Beginning `EngineSurfaceToken` close retires receipt lookup for that exact
   identity. A late completion must not mutate a same-slot successor, but it
   must not retroactively prevent a user action already committed to the
   runtime from dispatching.

This requires explicit receipt handling in the current renderer transfer
failure/retirement path, `FrameActionOwners` close/drain path, and deferred
action error branch. It does **not** require a frame candidate to own the
runtime action queue.

### Executable fixture and test matrix

Add a language-neutral `correlated-action-receipts` fixture, consumed by a
Rust queue/renderer law and an independent TypeScript oracle. The fixture
should use opaque token labels and intentionally duplicate descriptor payloads.

1. An accepted edit/select pair dispatches edit then selection and settles
   members `0`, `1` once.
2. A refused edit cancels its own unstarted selection; an identical descriptor
   under another token still dispatches. This proves correlation is not payload
   matching.
3. A superseded candidate retains a tagged pair for the successor completed
   frame and dispatches it exactly once.
4. Deferred-cursor close and input-transfer failure each produce one
   cancellation and no later select dispatch.
5. After source token retirement and slot reuse, a late result may complete
   Shell dispatch but cannot affect the successor ledger entry.
6. A failed tagged reservation/publish leaves no receipt and no local editor
   mutation.

The small storage test belongs beside the WGPU input action queue, the
correlation/cancellation test beside the renderer async/frame-action ledger,
and the stale-engine-token test beside the current EngineCanvas editor host
laws. This audit did not run those prospective tests.

## Host Temporal Presentation Audit

The browser host door is correctly centralized: the page and Worker route both
call `formatHostTemporalValuesV1` through `semioWgpuHostIo`
(`🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts:505-511`). That formatter uses the same
`Intl.DateTimeFormat` and `Intl.RelativeTimeFormat` primitives as the React
surface helpers (`🧬️contract/🕰️host-temporal-format/🟦️.ts:75-120`). Its
existing TypeScript door test drives the shared fixture through a separate
`Intl` oracle (`🧪️tests/🕰️wgpu-host-temporal-door/🟦️.ts:19-67`). This is good
WASM page-side ownership; the faults below are renderer request, native, and
late-reply boundaries.

### Confirmed EventFeed epoch defect

`EventFeedEntry.timestampMs` is an unconstrained number in the shared scene
shape (`🖱️ui/🎬️scene/🟦️.ts:1119-1126`). React always formats it as an epoch
date (`📡️EventFeedHost/🟦️.tsx:130-132`). WGPU instead filters out zero before
constructing its temporal request
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4903-4925`). A feed containing the valid
epoch instant `0` therefore passes an empty request to `host_temporal_reply`,
which rejects it, and `render_event_feed` draws no time for that entry
(`:4945-4947,5089-5091`). The current native parser test itself accepts a
missing timestamp as zero (`🧪️tests/🔬️wgpu-event-feed/🦀️.rs:4-13`), so zero
is not a declared absent-time sentinel.

The minimal fix is to retain every in-range timestamp, including zero. Add a
neutral EventFeed row with `timestampMs: 0` to the temporal fixture and assert
that the React/Intl and accepted-frame WGPU labels both expose the epoch time.
This has no effect on invalid or out-of-contract timestamps.

### Confirmed VFS relative-time drift

React VFS passes the current instant directly to the shared formatter
(`⚙️VirtualFileSystem/🟦️.tsx:235-240`), and its `relativeValue` rounds at the
second/minute/hour thresholds (`🧬️contract/🕰️host-temporal-format/🟦️.ts:53-72`).
WGPU rounds `app_now_ms` down to the start of a minute before requesting VFS
labels (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4895-4897,10023-10024`). Thus at
12:00:50, an item from 11:59:50 is `1 minute ago` in React but is formatted as
`10 seconds ago` by WGPU. The request/cache changes only on the minute, so the
wrong label can remain for up to 59 seconds.

Use the exact `app_now_ms` for VFS requests. The existing one-second request
throttle prevents repeated host calls; the request-match rule already treats a
changed `nowMs` as meaningful only when a relative value is present
(`:392-424`). Add a deterministic fixture case 50 seconds across the
minute/rounding boundary and assert its React reference label and the WGPU
accepted reply label are identical.

### Native formatter is not cross-platform Intl parity

The native installer is real (`🧊️renderer/🦀️.rs:17813-17816`), but
`SystemTemporalFormatter` is a hand-written template formatter rather than a
platform locale formatter. Its locale policy recognizes `de*` and exactly
`en-US`; every other locale receives ISO date order, and `dateTime` is joined
with a literal space (`🕰️native-temporal/🦀️.rs:237-283`). The browser/React
path delegates all locale ordering and separators to `Intl`
(`🧬️contract/🕰️host-temporal-format/🟦️.ts:83-103`). The shared fixture already
contains `en-GB` (`🧫️fixtures/🔣️.json:33-43`): native takes the ISO fallback
while the browser oracle requests `Intl("en-GB")`. Consequently native cannot
meet the same descriptor contract for that existing fixture, nor for the
other supported locales.

The native ISO reader is also narrower than React. React accepts whatever
`Date.parse` accepts (`⚙️VirtualFileSystem/🟦️.tsx:249-259`), including the
standard date-only `2026-03-08`; native requires a date, time, seconds, and
numeric or Z offset (`🕰️native-temporal/🦀️.rs:55-111`) and displays the raw
value on failure. Browser WGPU accepts the date through `new Date`, so this is
also a native/WASM divergence.

Finally, native reports `TZ` or the literal `"system"` as the profile time
zone and derives locale only from process environment
(`🕰️native-temporal/🦀️.rs:29-43`). `localtime_r`/`_localtime64_s` use the host
zone regardless of whether `TZ` is set, so the reported profile can be
`"system"` while labels use `Europe/Berlin`, and a system zone change without a
`TZ` environment change cannot advance the profile revision. On Windows, where
the POSIX locale variables are commonly absent, this likewise degrades to
`"und"` instead of the actual OS locale. The Unix FFI also assumes the
`time_t` parameter is `c_long`, a non-portable assumption outside the primary
64-bit Unix targets (`:115-171`).

Replace this native implementation behind the existing
`NativeHostTemporalFormatter` trait with OS formatter adapters that return the
actual resolved locale, IANA time-zone identifier, hour cycle, and labels.
Keep the shared request/reply schema and no external runtime dependency; the
platform calls belong in separate macOS, Windows, and Unix adapters. Do not
attempt to grow the current English/German template table: it cannot implement
the existing locale-neutral `Intl` contract. The native test must consume the
same fixture as the page door under controlled profile adapters, including
date-only ISO and `en-GB`; a system-smoke test alone cannot establish text
parity.

### WASM late-reply identity hole

The accepted-frame cache correctly separates candidate, queued candidate, and
accepted replies and seals them with the presentation epoch
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:381-476`; Shell seal/ack/discard at
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14453-14521`). A reply can still cross a
retirement boundary incorrectly. The WASM task captures only `host_id`, local
request generation, and request (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4953-4969`),
then publishes by `get_mut(host_id)` (`:4928-4941`). Retiring a scene removes
its map entry (`:1633-1641`); a later mount can reuse the same host id and
restart `HostTemporalPresentation` at generation one. If it issues the same
request, an old asynchronous answer has the same host id, generation, and
request and is accepted into the successor cache.

`AdmittedSurfaceToken { slot, epoch }` already rejects this exact class of
stale owner (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:180-246`). Capture the current
map token beside the request generation and change publish/refuse to use
`get_token_mut(token)`, not a host-id lookup. The async task should then be a
no-op after close or slot reuse. Add a paused host-I/O test: mount A and leave
its request unresolved, retire A, mount B with the same host id and identical
request, resolve A, and assert B has no candidate; resolve B and require that
only B's reply is sealable and then accepted. Repeat the same sequence across
a discarded candidate and a later accepted candidate to preserve the current
accepted-frame law.

This temporal audit was source-only. It did not run native, WASM, browser, or
fixture tests.

## Receipt Shutdown Re-Audit and Native Formatter FFI Review (2026-09-26)

This section supersedes the earlier hand-written-native-formatter diagnosis
above. It reviews the live system-adapter and receipt sources after the
coordinated temporal and action-receipt changes. It is source/API evidence
only; this audit did not compile or run a native, WASM, or browser target.

### Receipt ownership now covered at the ready-handback boundary

The renderer-local receipt remains beside `ActionDescriptor` in
`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10393-10430`, so the serialized domain
action is unchanged. `FrameActionReceiptOwner` settles the checked-out action
once and its `Drop` settles cancellation (`:10400-10417`). The deferred action
branch takes that owner before `dispatch_gesture_action().await`, settles
accepted or refused after the exact call, and cancels only the same opaque
correlation on an aborting refusal (`:11243-11264`). This is payload
independent.

`close_input_step` first removes a ready `ResumeFrameDeferred` or
`ResumeDispatch` completion and invokes its bounded
`close_returned_interaction_step`; only afterwards does it drain the resident
deferred cursor, staged actions, and InputState (`:12279-12315`). The existing
native law
`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:1284` stages ready handback,
resident cursor, staged action, and source receipt together. This establishes
the correct close order for a *ready* worker handback and avoids reopening a
discarded action.

### Remaining shutdown hole: a normal reserved future can be dropped in flight

`FrameMaintenanceExecutionRegistry` has a complete owner/recovery protocol:
its envelope and guard restore the exact `AppInteractionState` plus
`FrameDeferredCursor` on `Drop`, mark the registry abandoned, and enqueue an
empty `ResumeFrameDeferred` wake-up
(`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10797-10968,11985-12060`).

Normal deferred work does not use that protocol. Native
`spawn_frame_deferred_reserved` captures the future directly in an unowned
`spawn_app_task`, awaits it, and only then publishes the returned pair
(`:13414-13424`). If that scheduled future is dropped before its await resolves,
the current action's `FrameActionReceiptOwner` cancels itself, but the remaining
cursor and its remaining tagged action receipts are simply dropped. No ready
completion exists for `close_input_step` to drain; when runtime interaction is
already absent it returns terminal true (`:12311`). Waiting merely for an
in-flight completion would instead deadlock when the future was dropped.

The minimal correction is to generalize the existing maintenance registry and
envelope/guard to normal reserved `(AppInteractionState, FrameDeferredCursor)`
ownership. Guard drop must restore the whole pair to a mailbox-owned cell and
enqueue `ResumeFrameDeferred { interaction: None, cursor: None }`. Before the
`interaction == None` terminal branch, `close_input_step` must take that
recovered owner and reuse its existing one-step close cursor. It avoids an
unbounded destructor walk and gives every remaining receipt its existing
cancel path.

The missing law should block a normal deferred future behind a test gate, force
the worker future to drop before it returns, then repeatedly run input close.
It must observe each remaining tagged action cancelled exactly once, no
accepted/refused duplicate, pair recovery before terminal empty, and no
interaction resurrection. It complements rather than replaces the current
ready-handback test.

### Current native system-formatter fixes observed in source

Three high-impact issues identified during this review have already been
corrected in the live source snapshot:

1. `🌐️icu/🦀️.rs:69` now binds `ureldatefmt_formatNumeric`, matching React's
   `Intl.RelativeTimeFormat(..., { numeric: "always" })` in
   `🧬️contract/🕰️host-temporal-format/🟦️.ts:90-96`. ICU documents
   `ureldatefmt_format` as qualitative, while `formatNumeric` produces the
   required numeric form. The regression oracle still needs explicit plus/minus
   one-day English and German cases, because that is where the qualitative API
   differs (`yesterday`/`tomorrow`).
2. `🌐️icu/🦀️.rs:8,180-184` now uses a named `UDAT_PATTERN = -2` for both
   `udat_open` style parameters when it supplies the generated pattern. ICU
   requires that combination; `-1` is `UDAT_NONE`, which was not a valid way to
   request the supplied skeleton.
3. `🐧️unix/🦀️.rs:8-10,33` now uses named `RTLD_NOW | RTLD_LOCAL`. The former
   literal included Linux `RTLD_NOLOAD`, which only obtains an already-resident
   shared object and therefore prevented normal loading of the installed
   `libicui18n`.

Those source corrections have not been execution-verified by this audit.

### Remaining native FFI and availability work

**ICU 60--66 is advertised but cannot load.** Linux and Windows enumerate
major versions 60 through 99
(`🐧️unix/🦀️.rs:30`, `🪟️windows/🦀️.rs:36`), but `IcuApi::load` requires
`udatpg_getDefaultHourCycle` (`🌐️icu/🦀️.rs:60`). ICU marks that API stable
only since version 67. An installed ICU 60--66 is consequently found and then
permanently cached as unavailable. Preserve the stated range by deriving the
hour cycle from `udatpg_getBestPattern("j")` and scanning `K/h/H/k`, as the
macOS adapter already does, instead of mandating the version-67 symbol. A fake
loader without `udatpg_getDefaultHourCycle` should still format a fixture.

**macOS failure cleanup is incomplete.** The current macOS adapter now links
Foundation explicitly (`🍎️macos/🦀️.rs:41-42`), so
`NSRelativeDateTimeFormatter` is not only incidentally loaded. At
`:206-211`, however, a null pool, formatter, or components allocation returns
before releasing any of the non-null objects created by `alloc/init`. The
Foundation header makes `NSRelativeDateTimeFormatter` available from macOS
10.15, so the absence path is real on a lower-supported OS. Use a tiny local
release guard or explicitly release components, formatter, and pool in reverse
order before returning. Test the unavailable-class/allocation path or state a
product minimum of macOS 10.15.

**Windows currently assumes the combined library and ordinary DLL search.**
`🪟️windows/🦀️.rs:27-42` loads only `icu.dll` with bare `LoadLibraryW`.
Microsoft documents separate system `icuuc.dll` and `icuin.dll` in Windows 10
1703, with combined `icu.dll` introduced in 1903. Unless the product minimum
explicitly excludes the earlier native Windows releases, the adapter needs a
two-owned-handle fallback whose symbol lookup searches the common and i18n
libraries. `LoadLibraryW("icu.dll")` also follows the normal application DLL
search rather than proving it selected the OS ICU. A system-only adapter should
call `LoadLibraryExW` with `LOAD_LIBRARY_SEARCH_SYSTEM32`.

The current contract bounds reply locale/time-zone strings to 64 bytes and
labels to 256 bytes (`🧬️contract/🕰️host-temporal-format/🦀️.rs:100-115`), so
the native adapter must continue to reject rather than silently truncate a
platform string outside those bounds. Its current 256/512 UTF-16 work buffers
are not, by themselves, evidence of a contract violation.

Primary API references used for the FFI conclusions:

- [ICU relative-date formatter](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/ureldatefmt_8h.html)
- [ICU date formatter](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/udat_8h.html)
- [Linux dlopen](https://man7.org/linux/man-pages/man3/dlopen.3.html)
- [Windows system ICU](https://learn.microsoft.com/en-us/windows/win32/intl/international-components-for-unicode--icu-)
- [Windows DLL search order](https://learn.microsoft.com/en-gb/windows/win32/dlls/dynamic-link-library-search-order)

### Follow-up source check after the temporal packet

The later live source now also removes the version-67 hour-cycle dependency:
`🌐️icu/🦀️.rs:143-159` derives it from the localized best pattern for `j`.
The macOS relative path now owns pool, formatter, and components with reverse
drop cleanup (`🍎️macos/🦀️.rs:209-251`). The ICU overflow branches retry before
conversion (`🌐️icu/🦀️.rs:92-122,177-190,214-224,256-266`). These are coherent
with the bounded contract; their execution remains for the native receipt.

One concrete browser/native discrepancy remains in the source parser. The
shared TypeScript formatter accepts its ISO source through `new Date(iso)`
(`🧬️contract/🕰️host-temporal-format/🟦️.ts:60-61`). Native
`parse_iso_epoch_ms` requires a `:ss` field at byte 16 and parses seconds at
bytes 17--18 (`🕰️native-temporal/🦀️.rs:65-109`). Standard ISO minute-precision
values such as `2026-03-08T07:00Z` and
`2026-03-08T07:00+01:00` therefore format in React but become their raw source
text in native WGPU. Permit optional seconds with a default of zero; accept a
fraction only when seconds are present. Add both minute-precision forms to the
shared fixture and assert page Intl plus injected-native profile output. This
is an unresolved functional parity defect, not a claim about the pending
build.

The Unix and Windows dynamic `Library` wrappers also lack `dlclose`/
`FreeLibrary` cleanup. A successful system adapter must intentionally retain
its handle for the process lifetime, but a candidate that opens and then fails
the later `udat`/full-API symbol check currently leaks one reference while the
`OnceLock<Result<...>>` caches the failure
(`🐧️unix/🦀️.rs:28-48,64-73`; `🪟️windows/🦀️.rs:25-43,58-67`). A Drop wrapper
releases failure-path handles while the successful static tuple keeps its
library live. This is a bounded resource-lifetime correction, not a primary
user-visible parity blocker.

## Native157 Follow-up: Root Failures and Current Platform Adapter Review (2026-09-26)

`🗑️generated/astra-runtime/native157/run-2.log` records 71 passing and 11
failing focused native laws. It is not a green parity verdict. This section
separates that historical execution evidence from source changes visible after
the run. It does not claim a Native158 result.

### The three root-owned failures were not three equivalent production faults

1. `accepted_dock_names_match_painted_instance_titles_and_localized_react_actions`
   was a stale assertion. React `ModeDockTabBar` renders each tab's Focus or
   Unfocus button whenever its stack permits maximize; it does not suppress the
   stacked tab's button. WGPU likewise registers that painted control for each
   tab. The test's special assertion that a stacked Focus node must be absent
   therefore contradicted React. The correct shared-panel rule remains that
   every tab's `aria-controls` points at the one active stack panel, with its
   own selected state; it must not manufacture inactive panel IDs.
2. `chrome_controls_publish_accessible_names_and_shortcuts` used
   `nodes.len() == painted_hit_count`. The shell legitimately appends the
   non-interactive footer presence status after the hit-projected controls.
   That status should remain accessible. The law should assert the named hit
   nodes and their shortcut/role properties, not reject additional semantic
   status output.
3. `conflict_resolution_buttons_share_the_row_and_win_its_pointer_band` did
   expose a real horizontal-flow failure in the run: the reported action rects
   had the same x coordinate and successive y coordinates. The follow-up
   source diagnosis located `flow_for`'s early absolute placement for a
   `parentTreeRow`, which discarded the child stack's authored horizontal
   toolbar axis. Root changed that path to preserve the authored stack flow
   before applying the absolute box. This needs the targeted native law rerun;
   the audit does not mark it complete merely because the source changed.

### Native157 temporal failures are explained by source changes but remain unverified

The log's injected Foundation law reports, for the New York DST fixture,
`1:59:59` and a narrow no-break space where the shared Intl fixture expects
`01:59:59` and a normal space. The current macOS adapter obtains a
locale-specific skeleton, expands the unquoted hour token to at least two
characters, and normalizes NBSP and NNBSP only on the emitted absolute label
(`🕰️native-temporal/🍎️macos/🦀️.rs:102-126,170-190`). Those are direct,
minimal corrections for the recorded delta. The current injected-profile law
must run again on macOS before this becomes an execution result.

The log's relative-clock failure expected the accepted `first:relative` after
the next response was published. The current law instead distinguishes the
still-accepted cache from the candidate painted in the next sealed frame:
after the response it expects `visible() == next:relative` while
`accepted == first:relative`, then accepts the candidate only after the ACK
(`🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs:76-105`). That agrees with
`HostTemporalPresentation::visible`, which gives a candidate precedence only
for the candidate frame (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:435-474`). The
Native157 assertion predates this corrected acceptance model; rerun is still
required.

### Current Windows and Foundation adapter verdict

The prior warning about ordinary Windows DLL search is no longer current.
The Windows adapter caches `GetSystemDirectoryW`, constructs an absolute
System32 path, and supplies it to `LoadLibraryW`
(`🕰️native-temporal/🪟️windows/🦀️.rs:19-43`). It owns all loaded handles and
releases unsuccessful candidates in reverse order. It tries modern fused
`icu.dll`, unversioned split `icuin.dll` plus `icuuc.dll`, and versioned split
pairs before reporting an unavailable system ICU (`:46-131`). That is a sound
system-library boundary and fixes both the search-path and pre-fused-layout
findings in the older section above.

The macOS FFI now has the correct one-byte C `Boolean` representation for
`CFStringGetCString`, checks it against zero, and otherwise matches the
CoreFoundation date formatter signatures. It creates Foundation relative
objects inside an autorelease pool, copies the UTF-8 result before tearing down
the pool, formatter, and components in reverse order
(`🕰️native-temporal/🍎️macos/🦀️.rs:21-44,75-84,253-292`). The absolute
formatter sets the selected `CFTimeZone` after choosing its locale pattern,
which is the required order for the contract's explicit time-zone request.

Two product-support decisions remain, rather than defects in the present
loader: a Windows target without any system ICU still returns the explicit
unavailable error, and `NSRelativeDateTimeFormatter` is available only from
macOS 10.15. If the product supports either older platform, add a native system
adapter with the same contract and fixture; otherwise make those platform
minimums explicit. Neither is evidence for changing current supported-platform
loader behavior.

### Required next receipts

Run the targeted Native158 laws on their relevant hosts:

- conflict toolbar layout and the two corrected accessibility/dock laws;
- `macos_foundation_matches_the_shared_intl_fixture_for_injected_profiles` on
  macOS; and
- `relative_clock_refresh_preserves_the_accepted_label_until_the_exact_next_reply`.

The existing shared TypeScript fixture remains the language-agnostic oracle;
the native Foundation law must continue to consume it rather than introduce a
platform-specific expected string table.

## Browser26 Same-Viewport Screenshot Audit (2026-09-26)

This review compares the supplied same-document, 1280 × 720, DPR-1 captures:
[`react-current26.png`](🗑️generated/astra-runtime/browser26/react-current26.png)
and [`wgpu-current26.png`](🗑️generated/astra-runtime/browser26/wgpu-current26.png).
It is a static source-and-pixel review; it does not claim an interaction or a
new test execution result.

### P0 — The active Dock cap is state-gated, not absent from the painter

React visibly marks `Top` selected in red, whereas both WGPU caps are dark.
The WGPU painter already emits `theme.selected`, but only when both the tab is
the stack's active window and `DockState.active_stack` equals that stack path
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1683-1725`). Thus a change to colors or the
tab drawing primitive would not address the observed state. The boot/Focus
path must establish the concrete left-stack active instance before the
accepted frame is painted.

The existing Dock unit law covers an already-active tab, not the manifest boot
identity. The required executable receipt is a ConcreteForest shell boot law
that proves the active window is `puzzle3d-main-top`, its `active_stack` is the
left stack, and the matching cap has a `theme.selected` fill; switching to
Perspective must move that fill and deactivation must remove it. This belongs
with the current root-owned Focus/Close journey.

### P0 — WGPU drops the per-instance Dock icon contract

The leading WGPU `Top` glyph is the window-kind puzzle fallback, while React
uses the icon on the resolved window instance. React's Canvas maps the tab
through `windowsById.get(child.id)?.iconId` before falling back to
`"app-window"` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1160-1169`). Its actual
source of per-instance icon overrides is the Shell-local `windowIconsById`
record (`🐚️Shell/🟦️.tsx:567-570,983-985`): ShellHost resolves an extra window
as `windowIconsById[instance.id] ?? kind.iconId`
(`🏛️ShellHost/🟦️.tsx:12117-12127`). The WGPU `dock_chrome_maps` only copies
`kind.icon_id` for every instance and retains only instance *titles* from the
layout (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:25257-25287`). It has no counterpart
to that per-instance override record or its update path.

The smallest correction is a renderer-local `instance → icon` override
carrier, updated by the same shell-owned `SET_WINDOW_ICON` equivalent and
cleared with the owning instance/session; `dock_chrome_maps` should consult it
before the existing kind fallback. Do not put visual runtime state into the
persisted layout node. A fixture needs two instances of one kind with distinct
explicit overrides and must assert the actual dock textured quads use each
instance's resolved atlas cell, including that a retired-instance override
cannot appear in a successor accepted frame. The ConcreteForest capture then
becomes the browser acceptance check for the `Top` scene icon rather than an
image-only unit assertion.

### P1 — The source already creates the claimed missing controls

The screenshot does not establish missing interaction structure. WGPU builds
Focus/Unfocus, Close, and a `grip-vertical` action for each dock tab
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1236-1254`) and paints/registers them
(`:1742-1762`). Its footer independently creates the `s-sync-status`/hub item
and converts a signed-out hub to `framework.hub.signIn`
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:25133-25144,26737-26745`).

Both painter routes silently skip a glyph when `IconAtlas::icon_uv` has no
cell: Dock returns zero width (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1649-1656`),
and the shared Shell `chrome_icon` emits no textured quad (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17807-17811`). Their labels and hit targets can therefore exist while
the capture has no visible grip, remote, or sign-in symbol. Before changing
footer or tab structure, add an atlas/painter law for the exact live IDs
(`grip-vertical`, the resolved sync state icon, and the signed-out hub icon):
each must resolve and produce a textured quad in the accepted frame. If an ID
does not resolve, fix the canonical icon source/alias rather than adding a
second control.

### P1 — Camera and lighting need state evidence before a production change

The Top raster is offset right in WGPU and the Perspective capture has harder,
darker edges. Current source does *not* support the shortcut diagnosis that
WGPU has no viewport-aware projection framing. React derives its content
bounds and calls `frameWorldProjectionPose` with viewport dimensions
(`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:2381-2417,2524-2553`). WGPU computes the
same non-provisional-instance/visible-reference bounds and calls
`frame_projection_orbit_to_bounds` with the pane width and height
(`♾️infinite/🌍️world/🦀️.rs:11594-11615,11618-11648`; `🖱️ui/🎬️scene/📐️math/🦀️.rs:497-506`).

The next diagnostic must capture, for both panes and both renderers, the
accepted camera target/position/up/zoom/projection, bounds, reference
visibility, and actual content rect. Diff those values before retuning camera,
materials, lights, or antialiasing. Existing `world3d-camera-framing` and
`world3d-scene-shading` fixtures are the appropriate schema-first oracles;
the screenshot is a valuable acceptance signal but insufficient to identify a
lighting equation defect.

## Settings Section and Field Vertical-Geometry Audit (2026-09-26)

This is a read-only review of the supplied accepted captures
[`react-settings26.png`](🗑️generated/astra-runtime/browser26/react-settings26.png)
and [`wgpu-settings26.png`](🗑️generated/astra-runtime/browser26/wgpu-settings26.png).
They show the same Settings surface at 1280 × 720 DPR 1. React's panel begins
at approximately y=382 and is about 311 px tall; WGPU begins at approximately
y=485 and is about 203 px tall. The result follows the current shared
Section/Field contracts. It is not a Puzzle3D-specific geometry value and this
audit did not run a new renderer test.

### The React layout contract is composite chrome, not a 24 px header

`ContainerView` maps a `container/section` to `Section` and a
`container/field` to `Field`
(`🗣️Interpreter/🟦️.tsx:1201-1215`). `Section` is a normal block with
`mb-8`; when present, its title is an `h2.text-2xl.font-semibold.mb-4`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8980-8989`). The compact theme resolves
`text-2xl` to 1.35 rem and its line height to 1.9 rem
(`🖱️ui/🎨️styling/🖌️ui/🎨️.css:803-816`), so the captured two-line title has
two 30.4 px line boxes before its margin and body.

Each Field is a vertical flex container with `gap-single`. Its label is
`text-sm.font-medium` and `truncate`; optional description and error are
separate `text-xs` flex items, and the control is another direct flex item
(`📝️Field/🟦️.tsx:28-50`). Compact `text-sm` is 12.8 px with a 19.2 px line
box, while the shared native typography token states the same 12.8/19.2 pair
(`🖱️ui/🎨️styling/🔤️tokens/🦀️.rs:376-385`). Thus even the four one-line
Settings labels require a 19.2 px label line plus the single gap before each
control; descriptions and errors extend that flow rather than overpainting the
control.

### WGPU turns that composite layout into static bands and overlapping paint

At retained-layout admission, every Field becomes `LayoutNodeKind::Field {
top: theme.font_size_small + gap_standard }`, while every Section becomes
`LayoutNodeKind::Section { gap }`
(`🧊️wgpu/📌️mounted_layout/🦀️.rs:611-626`). `flow_for` ignores the authored
layout for both kinds: Field receives only its static top padding and Section
only a static `SECTION_HEADER_HEIGHT` top padding
(`🧊️wgpu/📐️flex/🦀️.rs:312-320`). That constant is 24 px
(`:444-447`), with no Section body-bottom margin. The existing flex laws
therefore explicitly assert the same static label band and static Section
header (`🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs:365-398`).

The values are not merely a paint-size difference:

- Field reserves `12.8 + 3.2 = 16` px before its child, although React
  reserves its 19.2 px line box plus that 3.2 px gap. It also has no
  description/error reservation after or before the child.
- Section reserves 24 px regardless of title text, whereas React must reserve
  the wrapped `h2` line boxes and its `mb-4`; it also omits React's `mb-8`
  after the body.
- The retained painter cannot repair that geometry. It paints the Field label,
  description, and error independently into the *same full Field bounds* with
  a regular small text run (`🧊️wgpu/🖌️paint/🦀️.rs:1312-1332`). It paints the
  Section title in regular `font_size_body` (14.4 px), not React's semibold
  21.6 px `text-2xl`, and also uses the full Section bounds
  (`:1334-1354`). The generic retained text path wraps at that whole box
  (`:400-414`), so a title can visibly acquire a second line without adding
  height or moving the first Field.

The native text subsystem already exposes the needed measured primitive:
`FontAtlas::measure_text` returns the CSS line-box height and
`measure_text_wrapped` returns `line_count × line_height`
(`🧊️wgpu/📝️text/🦀️.rs:896-906,954-962`). This is therefore not an atlas
limitation or a panel clipping issue.

### Hug layout is the direct cause of the short panel, not an independent truncation

The fixture's root Section and four Fields have `grow: false`; each NumberStepper
is a `hug` leaf (`🧫️fixtures/⚙️puzzle3d-settings-document/🔣️.json:20-107,
109-285`). Their retained Section consequently hugs the static 24 px header,
the static 16 px Field bands, control minimum heights, and the three standard
gaps. No later panel-height clamp has to truncate the result. The roughly 108
px capture delta is explained by the missing wrapped 2xl title and margins,
the four missing Field line-box deltas, and the absent Section bottom margin.
Changing a panel minimum height would conceal this particular capture while
leaving every narrow/localized Section and Field incorrect.

There is a separately documented retained grow discrepancy for Field/Section
children (`🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs:1805-1816`). It matters
when an ancestor supplies leftover height, but it is not required to explain
this hug-sized Settings panel; do not use it to defer the measured chrome
correction.

### Smallest generic correction and required receipts

The owner is the shared WGPU retained composite-layout and retained-paint
boundary, principally `📌️mounted_layout/🦀️.rs`, `📐️flex/🦀️.rs`, and
`🖌️paint/🦀️.rs`. Keep Puzzle3D's `ui::section`/`ui::field` producer unchanged.
Replace `Field { top }` and `Section { gap }`'s fixed bands with a bounded,
shared `SectionFieldChromeMetrics` calculation derived from the mounted node,
theme, and offered content width:

1. Section: measure the optional label at the React `text-2xl` line height
   and semibold weight, reserve its wrapped height plus the `mb-4` body
   separation, and preserve the Section trailing `mb-8` in the parent flow.
   Draw from the resulting title rect. The React Section has no disclosed
   interaction in this path, so the retained chevron must not determine title
   geometry or be treated as a substitute header contract.
2. Field: reserve the `text-sm` *line height* plus `gap-single` before its
   direct control; add each optional `text-xs` description/error line and its
   flex gap in React order. Draw label/description/error into those exact
   allocated rects with `font-medium` for the label and the existing colors.
   Keep the React `truncate` policy for labels instead of allowing the
   retained label painter to wrap into unreserved rows.
3. Feed the metrics into both flex padding/margins and the retained painter.
   The measurement must rerun when a Section label, Field label/description/
   error, width, locale-resolved label, or theme typography changes. The
   current invalidation is insufficient: although Field equality includes
   `error`, `layout_affecting_change` dirties layout only for its label or
   description (`🧊️wgpu/🔀️reconcile/🦀️.rs:1518-1519,1530-1540`). Include
   `error` in that layout path before its measured band becomes real.

Required test-first receipts:

- Replace the static-band assertions in
  `🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs` with a neutral narrow-width
  Section title that wraps twice and a Field with description and error. Assert
  the control's y coordinate follows the exact measured `text-sm` line box and
  flex gaps; assert the parent hug height includes the Section title/body/trailing
  spacing and grows by one complete Section line height when the offered width
  forces the second title line.
- Add a retained-paint law that receives the calculated text rects and proves
  label, description, control, and error have disjoint vertical bands; assert
  the Section title uses semibold `text-2xl` and no glyph paints outside the
  title rect. This prevents a layout-only fix that still paints the old 14.4 px
  title.
- Extend the existing schema-first
  `puzzle3d-settings-document` fixture/React test
  (`🧑‍🎨engine/🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx`) with the
  actual long Settings title at a constrained panel width. The React test is
  the language-agnostic DOM/token oracle; its WGPU counterpart must consume
  the same document and assert the accepted rect ordering and total hug height.
  Then refresh the same-size browser capture as final acceptance, rather than
  storing screenshot coordinates as a geometry oracle.

## Settings Presentation, Field Naming, and Dock Icon Audit (2026-09-26)

### Scope

This is a read-only comparison of the current generic Stepper, container
accessibility projection, and the new Dock icon mapping. It does not claim a
runtime check or a passing test.

### Stepper follow-up

The normal Settings values now have a proper shared twelve-significant-digit
presentation oracle. React uses Number.parseFloat(n.toPrecision(12)).toString()
in 🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx:335-344. The
fixture covers 0.30000000000000004 to 0.3, 1e-6 to 0.000001, and 1e21 to
1e+21; the WGPU implementation at
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🎯️targets/🧊️wgpu/🦀️.rs:20-38
uses only Rust standard-library formatting. No runtime dependency was added.

The normal-width centered input and square side controls agree. React's
Stepper has fixed h-medium, w-medium, shrink-0 buttons and a min-w-0 flexible
input inside an overflow-hidden group
(🧱️elements/🪜️Stepper/🟦️.tsx:189-275). WGPU shares its segment helper
between layout, paint, and events
(🧮️layout/🦀️.rs:337-340, 🖌️paint/🦀️.rs:1233-1269, ⚡️events/🦀️.rs:590-607).

Two generic boundaries do not match React:

1. At width less than twice control height, React retains both fixed-width
   buttons and clips the overflowing group; the input collapses. WGPU clamps
   both buttons to half the outer width, so the current 30 by 24 fixture gives
   15-pixel buttons. That changes both the visible affordance and the accepted
   pointer bands. Preserve logical side width equal to control height and
   derive visible/hittable rectangles from the outer clip. Replace the current
   narrow fixture case, which encodes WGPU rather than React behavior.
2. Under RTL, React's logical border-e/border-s and flex axis put minus on the
   physical right and plus on the left. WGPU has no direction argument and
   hard-codes decrement-left/increment-right; events consume that same order.
   Thread the existing generic FlowInline direction through Stepper geometry,
   paint, and press sign resolution rather than adding Puzzle-specific RTL
   handling.

The painter still uses body-size text glyphs for plus/minus, while React draws
size-tiny SVG icons and switches the desktop input to text-sm. This is visual
polish, independent of the value-formatting fix.

Accessibility remains inconsistent with the new visible text. Contract
accessibility_value publishes NumberStepperProps.value.to_string() at
🧬️contract/♿️accessibility/🦀️.rs:231, so an accepted spinbutton can announce
0.30000000000000004 while the React input and WGPU paint show 0.3. Move the
finite display formatter to the shared presentation contract and reuse it for
paint and accessibility; the contract must not depend on a WGPU module.

Required fixture rows: negative zero; values immediately on both sides of the
1e-6 and 1e21 transitions; a twelve-digit rounding carry; narrow physical
clipped hit rectangles; RTL rectangles plus left-increments/right-decrements;
and the spinbutton accepted value text. The React test should calculate each
formatter expectation through its actual formatNumber export.

### Field association and Container fallback

The current Puzzle3D Settings document obeys React's exact association
convention: each Field key such as settings.contact-tolerance has the direct
NumberStepper key settings.contact-tolerance.control. The producer asserts
this at
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:3512-3529.
React Field generates label htmlFor fieldId.control
(🧱️elements/📝️Field/🟦️.tsx:28-50) and Stepper applies the supplied id directly
to its input (🧱️elements/🪜️Stepper/🟦️.tsx:209-262). The current React
Settings test checks all four computed spinbutton names at
🧑‍🎨engine/🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx:77-104.

The WGPU inheritance in
🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:99-157 works for that direct Settings
shape and does not cross an intermediate wrapper. It is broader than React:
it labels any direct supported control under a Field, without requiring the
fieldKey.control identity. The generic schema permits arbitrary children and
does not constrain a Field's child count or control key. A direct button,
multiple controls, or a differently keyed control will therefore diverge.
The durable contract is the key association, not ancestry alone. Validate
exactly one field control named fieldKey.control, or require that equality in
the WGPU inheritance predicate.

The current general Container fallback is also not wholesale React parity:

- React renders Section and Group through Section with the component title
  (🗣️Interpreter/🟦️.tsx:1201-1207).
- React renders Field as a plain div plus native label; it does not create a
  labelled group (🗣️Interpreter/🟦️.tsx:1209-1215).
- Plain/Form/Toolbar spread explicit record accessibility ARIA but do not
  automatically apply component.label
  (🗣️Interpreter/🟦️.tsx:1217-1231).
- Section/Group/Field do not spread that explicit ARIA object today.

By contrast accessibility_projection_node chooses explicit accessibility label
then component label for every Container role and projects Field as a group
(🧬️contract/♿️accessibility/🦀️.rs:244-275). This correctly names the current
Settings Section from its component label, but needs a shared policy before it
can be called React parity. The narrow parity policy is Section component
label names the region; a conforming Field label names its .control input;
an explicit control label overrides that association. Decide separately
whether React should apply wrapper ARIA or WGPU should avoid component-label
fallback for Plain/Form/Toolbar/Field groups.

### Minimal Section and Field measurement packet

One reusable schema-first section-field-presentation fixture should be
consumed by a React DOM/token test and a WGPU retained-layout law:

| Case | Authored shape | Required oracle |
| --- | --- | --- |
| section-one-field | Section, direct Field, direct .control Stepper | heading line box and trailing margin; Field label-to-control gap; total hug height |
| section-wrapped-heading | same at a title-wrapping width | two heading lines; control below the whole heading block; no overlap |
| field-description-error | Field with description, required marker, control, error | description before control, error after; all bands contribute to height |
| field-key-miss | Field and direct but non-.control control | React name absent without own control label; WGPU must match or reject |
| explicit-wrapper-label | distinct component and accessibility labels on Section and Field | assert the chosen shared renderer policy |

Compact React tokens are spacing single 3.2 px, text-sm line height 19.2 px,
text-xs line height 14.4 px, and text-2xl line height 30.4 px
(🎨️styling/🖌️ui/🎨️.css:732-745,803-816). Store density and measured
relations/line count, not screenshot coordinates. The fixture should also
catch the existing WGPU invalidation omission: layout_affecting_change includes
Field label and description but not error
(🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1524-1540).

### Dock icon override audit

React owns a generic ephemeral windowIconsById map and SET_WINDOW_ICON
windowId/iconId action (🐚️Shell/🟦️.tsx:555-570,750-768,978-985). Its contract
test gives a base kind and an extra instance different arbitrary icons at
🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:2941-2951. ShellHost displays
windowIconsById[instance.id] before kind.iconId
(🏛️ShellHost/🟦️.tsx:12117-12131). World3D merely calls that generic callback
with a projection-derived icon
(🌐️World3dHost/🟦️.tsx:6648-6654).

WGPU currently stores only world_projection_template, then feeds it to
dock_tab_icon_id_v1(kind_icon_id, template_id)
(🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:19761-19765,25375-25403). Cleanup of that
projection map is correct at :7379-7384 and :7965-7979, but an arbitrary
per-instance icon cannot be represented and no generic host command exists.
The current test proves Top/projection icon selection only, not the React
contract.

The minimal boundary is a bounded ephemeral window_icon_overrides map beside
world_projection_template, written by a generic window-id/icon-id host command
and cleared with the same live/retirement paths. Dock resolution must be:
instance override, then projection-derived icon, then declared kind icon.
Projection selection can write through that generic path or remain the
lower-priority derivation; it must not be the storage type. Add one fixture
with two same-kind instances, distinct non-projection overrides, replacement
of one id, kind fallback, retirement, and reuse of the retired id. The
accepted Dock frame must contain only current-instance icons and no stale
override.

## Numeric Canonicalization and Retained Stepper Interaction Audit (2026-09-26)

### The half-up correction is correct, but it is only the first half of React's formatter

The React authority is now the shared `formatUiNumber`: it does
`Number.parseFloat(value.toPrecision(12)).toString()`
(`🧬️contract/🔢️number-format/🟦️.ts:1-4`, called by
`✏️Input/🟦️.tsx:335-344`). That has two separate operations: twelve-significant
decimal rounding and JavaScript's shortest decimal rendering of the re-parsed
binary number.

The new Rust tie predicate at `🧬️contract/🔢️number-format/🦀️.rs:26-35` is
sound for the first operation. With rounding quantum `10^k`, an exact binary
float is exactly halfway between decimal multiples precisely when its odd
significand has binary exponent `k - 1`; for `k > 0`, it must additionally
contain `5^k`. The `k <= 0` branch is sufficient because a binary-representable
half has already cancelled the corresponding denominator factor of five. The
`k <= 22` bound is safe: `5^23` exceeds the at-most-53-bit odd significand.

A direct temporary compilation of the actual Rust module produced the React
answers for the requested negative- and positive-power ties, including
`±1234567890125 → ±1234567890130`,
`±1000000000005 → ±1000000000010`,
`±12345678901250 → ±12345678901300`,
`±123456789015.5 → ±123456789016`, and
`±10000000000.25 → ±10000000000.3`. Carries such as
`9999999999995 → 10000000000000` also remain correct because the
ties-to-even formatter has already rounded upward and the correction is gated
on the parsed rounded magnitude being below the source magnitude.

The implementation audited before its planned canonicalization follow-up
still fails the second operation. The independent React oracle gives:

| exact f64 | React `formatUiNumber` | pre-canonical Rust output |
| --- | --- | --- |
| `f64::from_bits(1)` | `5e-324` | `4.94065645841e-324` |
| `f64::from_bits(2)` | `1e-323` | `9.88131291682e-324` |
| `f64::from_bits(100)` | `4.94e-322` | `4.94065645841e-322` |

Those values are finite and the current fixture contains none of them. The
missing boundary is the `parseFloat(...).toString()` round trip, not a tie
error. Reparse the corrected scientific string, get Rust's shortest f64
decimal, then normalize that shortest representation at React's `[-6, 21)`
fixed/scientific thresholds. Do not preserve the pre-parse twelve digits.
`Number.MAX_VALUE` is not an overflow exception: the actual React expression
returns the finite `1.79769313486e+308`. Its fixture row should assert that
value; an `Infinity` result would be a new parity defect.

The language-neutral fixture needs all signed counterparts where formatting
adds a sign, `k=-1`, `k=0`, `k=1`, and `k=2` ties, a carry, the three
subnormal rows above, transition neighbors at `1e-6` and `1e21`, negative zero,
and `Number.MAX_VALUE`. The existing Rust law in
`🧪️tests/🔬️targets-wgpu-layout-unit/🦀️.rs:23-30` and React's production
formatter should consume that same fixture. Value text must remain gated by
`uniform`: both shared accessibility projections now correctly produce no
`valueNow` or `valueText` for a mixed NumberStepper
(`🧬️contract/♿️accessibility/🟦️.ts:162`, `🦀️.rs:231`).

### Retained NumberStepper is focusable but not text-editable

React renders a native `input type="number"` inside the Stepper. It accepts
normal browser caret movement, selection, paste and IME; parsed numeric input
updates the local value and emits `onChange` (`🧱️elements/🪜️Stepper/🟦️.tsx:131-136,
209-262`). Arrow Up/Down update local state and dispatch the same step action;
Escape restores the last declarative value and blurs; Enter ends editing and
blurs (`:230-252`). A minus/plus pointer down changes the local value
immediately and starts repeat after 500 ms at 100 ms intervals (`:86-112,
146-178`).

WGPU puts NumberStepper in the retained Tab order
(`🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:323-325`) but only seeds `EditState` for
Input and IconSelect (`:386-399,502-511`). All text, clipboard, and IME routes
are consequently a no-op for a focused NumberStepper because they require that
buffer (`:1755-1876`). The only retained keyboard path is Arrow Up/Down
(`:2411-2435`), and it emits an intent without the React local display update
(`:639-648`). Pointer down has the same no-local-echo behavior and no repeat
timer (`:2122-2124`). This is a user-visible interaction and accessibility gap,
not a paint discrepancy.

The narrow next packet belongs at the generic retained-event boundary:

1. Give a focused uniform NumberStepper its own string edit buffer seeded from
   the same shared display formatter; never use raw `f64::to_string()`.
2. Admit TextInput, IME commit, selection/caret, Backspace/Delete, and clipboard
   through that buffer, publishing an absolute `Change` only when the buffer
   parses to a finite value. A mixed control begins empty with the React mixed
   placeholder and first valid edit ends mixed state.
3. Retain the declarative value separately. Escape and blur retire the buffer;
   Enter does likewise without a second value action. Arrow movement remains
   the existing delta/absolute-binding choice, but it must update the retained
   displayed value pending the accepted producer echo.
4. Add repeat only with an injected, cancellable clock and explicit pointer
   capture/loss-of-focus retirement; do not add a wall-clock loop to the frame
   walker.

A neutral `stepper-editing` fixture should cover focus seed, select-all replace,
invalid intermediate text with no action, IME commit, paste, Arrow Up/Down,
Escape, Enter/blur, mixed first edit, and retirement before an async echo. A
separate injected-clock row can assert one immediate pointer delta, no repeat
before 500 ms, 100 ms local-display repeat cadence, and cancellation on pointer
up/leave. React's timer changes `internalValue` only; it must not create extra
producer actions after the single immediate pointer-down delta.
The existing `🧪️tests/🎛️retained-control-commit/🦀️.rs` already owns retained
NumberStepper action routing and is the natural native law location; add a
React Stepper DOM/action oracle from the same fixture.

### Measured current Stepper border geometry

The paired browser measurement is more exact than the existing integer layout
fixture. React's 298 by 22.390625 px group begins at x=977.8125/y=482.328125;
the outer one-pixel border places the minus child at x=978.8125/y=483.328125
with a 22.390625 px box. The child intentionally retains the group height and
its lower edge clips under the parent's `overflow-hidden`. Its center begins at
x=1001.203125 and width 251.21875; plus begins x=1252.421875.

WGPU's shared segment helper currently begins child segments on the outer
bounds, while retained paint also paints outer and center borders from those
same uninset rectangles (`🖌️paint/🦀️.rs:1233-1245`). Its text `−`/`+` glyphs
also differ from React's `RemoveIcon`/`AddIcon` size-tiny SVGs. The minimal
generic geometry change is a theme-owned hairline inset passed to both segment
calculation and retained paint, retaining outer clipping and keeping hit
rectangles congruent with the displayed child bands. Add a fractional-height
fixture row that asserts group, child, and accepted hit rectangles, plus a
paint law for icon-atlas minus/plus rather than font glyphs.

### Formatter follow-up validation

The current formatter now reparses the corrected twelve-digit decimal, obtains
Rust's shortest f64 `Display`, accepts either fixed or `e` coefficient form,
and then applies React's display threshold
(`🧬️contract/🔢️number-format/🦀️.rs:13-30`). Rust Display is fixed-form for
the minimum subnormal and `f64::MAX`; the point/leading-digit decoder handles
that bounded form correctly and retains an explicit exponent branch where a
platform emits one.

I directly compiled the current Rust module and asserted the requested ties,
signed `k=-1`, `k=0`, `k=1`, and `k=2` ties, carry, negative zero, thresholds,
subnormal bits 1/2/100, and signed `f64::MAX`; every assertion passed. A
second 39-bit-pattern cross-check diffed current Rust output against Bun
executing React's actual `Number.parseFloat(x.toPrecision(12)).toString()`.
The diff was empty, including finite boundaries, signs, max, infinity, and
NaN. This is an isolated formatter validation, not a full WGPU runtime claim.

The permanent fixture has the requested `k=-1`/`k=1` ties and first two
subnormals. It should also retain signed `k=0` and `k=2` examples plus
subnormal bit 100 (`4.94e-322`) to cover the proof branches rather than rely
on the temporary cross-check.


### NumberStepper execution packet: schema, editing, local echo, and hold lifecycle

This corrects the earlier bounded packet with the exact retained React adapter
and with current source rather than inferred native-input behavior.

**Schema first.** NumberStepperProps has exactly value, step, and uniform
(🧬️contract/🧩️component/🦀️.rs:376-380), mirrored by the generated TypeScript
shape (🧬️contract/🧬️schema/🦀️.rs:485-491) and typed visitor
(🧬️contract/🧾️typed/🦀️.rs:23). Its WGPU projection also has only those
fields (🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2182-2195,
🔀️reconcile/🦀️.rs:853-865), so neither renderer currently receives a
contract range. Add nullable min and max there first, including limits, all
producers/fixtures, the WGPU projection/reconciler, both React adapter shapes,
and AccessibilityValue.min/max (today NumberStepper only exposes now/text at
🧬️contract/♿️accessibility/🦀️.rs:231). The contract admission law must reject
non-finite bounds and an inverted supplied pair.

Do not add readOnly to this step. The actual React StepperProps declares no
read-only or disabled property (🧱️elements/🪜️Stepper/🟦️.tsx:28-44), and neither
retained React adapter passes activity disabled into a Stepper
(🗣️Interpreter/🟦️.tsx:1163-1164,1415-1431). The WGPU focus and accessibility
router nevertheless suppresses controls whose presence is disabled
(⚡️events/🦀️.rs:324-332,1925-1931). That is a separate current React/WGPU
disabled discrepancy; it needs an explicit contract decision, rather than
inventing read-only semantics while implementing text editing.

**Current React behavior to preserve.** The center segment is a controlled
input type=number (Stepper:209-262). Focus marks editing and calls the
pointer-down callback but does not select all; caret placement, Shift
selection, Left/Right/Home/End, deletion, clipboard, and IME are native input
behavior. onChange calls parseFloat, ignores NaN, then locally clamps and emits
absolute onChange (Stepper:67-84,131-136). Enter only ends editing and blurs.
Escape restores value or defaultValue, ends editing, and blurs
(Stepper:230-252). The normal retained adapter supplies value only for a
uniform node and supplies onDelta only if the record actually binds Delta
(🗣️Interpreter/🟦️.tsx:1418-1431); a mixed field starts as local default zero
but renders an empty value with the mixed placeholder until first edit.

applyDelta clamps the local display but sends the raw delta when a Delta
binding exists; without Delta it sends the clamped absolute value
(Stepper:86-95). This makes the edge behavior important: side buttons are
disabled at an applicable range edge (Stepper:181-182,196-208,263-275), but an
Arrow Up/Down key still calls applyDelta (Stepper:230-240). Thus an arrow at
the edge leaves the display clamped, yet sends raw plus/minus step on Delta or
the clamped bound on Change.

Hold is intentionally local-only after its initial press. Mousedown applies
one delta and begins a 500 ms delay followed by 100 ms intervals
(Stepper:97-112,146-160); each interval only calls setInternalValue and never
calls onChange or onDelta. MouseUp clears it and invokes pointer-up; MouseLeave
clears it and invokes pointer-cancel; component unmount clears it; the
implementation attaches touch start/end equivalents
(Stepper:114-129,163-179,196-205,263-271). This is a bounded injected-clock
state in WGPU, not a frame-walker or repeated guest action.

**Current WGPU gap.** A retained NumberStepper is focusable, but only Input
and IconSelect can seed an EditState (⚡️events/🦀️.rs:359-404,505-514). A center
click can consequently focus the stepper but creates no editable buffer; text
input, paste, IME, navigation, selection, Backspace, and Delete all exit
through the missing buffer (⚡️events/🦀️.rs:1700-1888,2219-2254). The streaming
painter always uses the declarative value or mixed placeholder, never a local
edit/composition (🖌️paint/🦀️.rs:1256-1296). The current keyboard path only
handles Arrow Up/Down (⚡️events/🦀️.rs:2414-2439), and the pointer third path
only emits an action (⚡️events/🦀️.rs:581-651); neither updates local
presentation or schedules a hold. Accessibility Value parses and sends an
absolute action, but similarly retains no edit/presentation state
(⚡️events/🦀️.rs:1972-2000). Escape is currently reserved for overlays/search
and Enter falls through generic editing without a stepper buffer
(⚡️events/🦀️.rs:2219-2249).

The minimal owner is the retained EventRouter plus existing per-node
WidgetState.edit: allow a stepper edit buffer only while it holds focus, seed
its uniform display text at the end caret (mixed starts empty), have the
painter prefer it, and retire it on blur, Tab, node/surface retirement, Enter,
and Escape. Mutating input, paste, and IME commit must share one
NumberStepper-specific parse/clamp/action function; caret motion, selection,
IME start/update, and invalid intermediate text change presentation only.
Escape restores the last declarative value without an action; Enter merely
blurs and must not duplicate the per-change action. The root action receipt
outbox must associate local numeric edits with their opaque emission token so
an old accepted echo cannot roll back a newer local value; rejection retires
only the matching pending edit. Do not mutate the declaration on optimism.

**Neutral fixture and test seams.** Extend the existing language-neutral
🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json, native law
🖱️ui/🧪️tests/🎛️retained-control-commit/🦀️.rs, and React twin
📺️renderer/🧑‍🎨engine/🧪️tests/🎛️retained-control-commit/🟦️.ts; the focused
Delta-vs-Change binding law is already at native :400-440. A dedicated shared
stepper-editing fixture is preferable for ordered presentation snapshots and
controllable hold-clock inputs. It needs these rows:

- Uniform focus has an end caret and no implicit select-all; movement,
  selection, Backspace/Delete, paste, and valid type each preserve the
  expected local text and issue one absolute Change only when text mutates to
  a parsable number.
- IME start/update changes only composition presentation; commit applies the
  same numeric path once; invalid or empty text sends none.
- Enter/blur send no duplicate Change; Escape restores declared text and blurs
  with no action; mixed focus starts empty/placeholder and first valid value
  ends mixed presentation.
- Pointer and Arrow operations use Delta only when declared; Change fallback
  has value, Delta has delta; range edge pointer controls are inert, while
  edge arrow has raw Delta or clamped Change exactly as React does.
- Clock at 499 ms has only the immediate guest action, then every 100 ms tick
  changes local presentation with no guest action; pointer-up, leave,
  unmount/retirement, and focus loss stop it.
- Accessibility Value enters the same numeric local/action path, with field
  label/range projection and no special stale echo path.

Current shared fixture tests only a press and action address; React's static
contract markup test (📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:
4325-4336) and native presented-input fixture do not exercise this
interaction.

### Border and SVG follow-up

This paragraph is superseded by the current source review. The shared helper
now receives `border` and derives the measured React geometry
(`🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:337-346`): at the measured 298 by
22.390625 px group with a 1 px hairline it gives precisely the x+1/y+1,
22.390625 px side bands and the 251.21875 px center. Retained paint passes
`theme.stroke_hairline` (`🖌️paint/🦀️.rs:1252-1253`), and the event router
receives the same `border` before it calls that helper
(`⚡️events/🦀️.rs:581-600`). Paint and accepted pointer geometry therefore
remain one authority.

The icon half is also now source-aligned: React `RemoveIcon`/`AddIcon` resolve
to the canonical `minus`/`plus` IDs (`🧱️elements/🔣️Icons/🟦️.tsx:541,624`), and
the retained painter submits those IDs through `push_stepper_icon` at
size-tiny (`🖌️paint/🦀️.rs:1267-1285,2684-2687`). The root's new layout fixture
already passes the fractional border into the same helper
(`🖱️ui/🧪️tests/🔬️targets-wgpu-layout-unit/🦀️.rs:16-20`). I did not run that
law in this audit, so this is a source-congruence finding, not a runtime/pixel
sign-off. It needs the measured fractional row plus an icon-atlas paint
assertion before the screenshot gap can be closed.

### Dock tab keyboard and roving-tab-order gap

This is the next substantive generic window-system defect. React's
`ModeDockTabBar` implements a roving tab stop: only the selected tab has
`tabIndex={0}`; its siblings have `-1`; ArrowLeft/ArrowRight wrap within the
same stack; Home/End choose stack ends; and Enter/Space select the focused tab
without changing selection on focus alone
(`🧱️elements/🎨️Canvas/🟦️.tsx:974-1014,1046-1059`).

The current WGPU browser mirror creates a generic `div role=tab`, then maps
every `focusable` projection node to `tabIndex=0`
(`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:42-75`).
Shell chrome marks every published control focusable, including dock tabs
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:31883-31900`). Thus inactive dock tabs join
the browser Tab sequence. The mirror publishes only Focus, Blur, Activate, and
Value events (`♿️accessibility-mirror/🟦️.ts:117-130`); it has no tab-key
semantic.

That absence is observable behavior, not merely ARIA metadata. Browser input
prevents the native default for a focused non-button mirror tab and sends an
untargeted key to WGPU (`🎮️input-wire/🟦️.ts:80-90`). Shell's keyboard handler
has a global Tab active-window cycle (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16966-
17190`) but no branch that reads `accessibility_focused_control_id` to handle
tab arrows, Home/End, Enter, or Space. A generic role=tab div has no native
Enter/Space click behavior either. Consequently keyboard focus can reach
inactive tabs by Tab but Arrow/Home/End do nothing and Enter/Space cannot
activate that dock tab.

The clean cross-platform boundary is an accepted-addressed tab-key event, not
an inferred global key or a local DOM-only selection mutation. Extend the
shared accessibility event union with a finite tab operation
`previous|next|first|last|activate` and retain the exact
window/generation/node/key address already used by focus and activation. The
browser mirror owns DOM key interception and maps the five keys only for
`role=tab`; the Shell validates the accepted target, finds its exact dock stack,
updates focus for the four movement operations, and dispatches the existing
activation path only for activate. Add a `tabbable` projection fact so the
mirror exposes exactly the selected tab as Tab-reachable while allowing
programmatic focus on its `-1` siblings. Do not use `selected` as generic
focusability and do not change painted active/highlight state when focus moves.

Test at the existing browser mirror interaction law
(`📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`) and
the Shell input law
(`🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`), with a neutral fixture of three
tabs in one stack and one tab in another. Prove Tab enters only the selected
tab; previous/next wrap within one stack; Home/End remain in that stack;
movement leaves `selected` unchanged; Enter and Space select exactly once; and
a stale generation/node address changes neither focus nor selection. The React
component is the independent behavior oracle at the Canvas lines above.

### Window dock drag audit and a real maximize resynchronization mismatch

The tab-drag, split, join, root-split, and cancellation lanes are no longer a
useful duplicate implementation target. React delays a drag until movement
reaches 6 px, derives the lifted-out tree without mutating its committed
layout, commits only a non-null drop zone, and clears state on release or
Escape (`🧱️elements/🎨️Canvas/🟦️.tsx:1512-1601,1869-1909`). WGPU has the same
5 px squared threshold, committed-tree derivation, exact presented drop-zone
geometry, one-shot apply, outside-drop no-op, and pointer-owner cancellation
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7524-7547,13961-13964,14189-14223,
14361-14476`; `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:455-575,1025-1145`). Existing
native laws cover root split, corner merge, 50/50 split, outside-drop refusal,
Escape cancellation, and duplicate-release refusal
(`🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:1123-1145`; `🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1559-1585`).
No new drag/split implementation should be scheduled from this audit.

One current user-visible divergence remains at layout resynchronization.
React clears the ephemeral maximized stack on *any* changed `layout` or
`windows` prop (`🧱️elements/🎨️Canvas/🟦️.tsx:1366-1404`), even if the maximized
stack and its selected window remain present. WGPU instead deliberately
reanchors `maximized_stack` by window key in `DockState::apply_layout_diff`
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:663-680`), which Shell calls for a supplied
layout override during every dock synchronization
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7376-7404`). The native unit test currently
pins that opposite behavior by expecting a reordered incoming layout to retain
`maximized_stack` (`🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:524-558`).

Reproduction: use two stacks, focus/maximize the right stack, then deliver a
new layout with the same windows and active tab but a changed split ratio.
React restores the full dock; WGPU continues rendering only the maximized
stack. A close that changes the roster exposes the same difference whenever the
remaining maximized stack still has more than one window. This is independent
of the stale-path safety problem: the incoming layout may retain the exact same
stack identity.

The minimal parity change belongs at the Shell layout-provenance boundary, not
inside generic pointer/drop code: clear `dock.maximized_stack` before applying
a changed externally supplied layout or window roster, while retaining
key-based path re-resolution for ordinary, unchanged refreshes. The decision
must distinguish an unchanged self-persisted layout from an actual incoming
layout revision so a refresh loop cannot toggle view state. Update the neutral
layout fixture and the existing Dock unit law with (1) geometry-only incoming
layout clears maximize but preserves active id, (2) roster change clears
maximize and retires missing focus, (3) byte-identical resync keeps maximize,
and (4) a stale source/maximized path cannot act after its session changes.
The React `Mode` component is the independent behavior oracle; add its
rerender assertion beside that component's current layout-prop tests.

### Slider presentation is missing the default readout and filled range

This is a current shared primitive defect, distinct from the Slider editing
and local-echo work assigned elsewhere. React's owned Slider defaults
`showValue` to `true` (`🧱️elements/🎚️Slider/🟦️.tsx:137-152`) and consequently
lays out a `slider-row` grid with a track cell plus a fixed `w-large`,
right-aligned `slider-value` readout (`:500-538`). The readout uses
`formatNumber`, which delegates to the shared `formatUiNumber`
(`🧱️elements/✏️Input/🟦️.tsx:336-346`); it is present with or without a unit.
The range is a separate rounded filled rail, with the thumb centered over its
logical percentage (`🎚️Slider/🟦️.tsx:411-493`).

The base palette is also explicit: track `bg-muted`, range `bg-element`, and
thumb `bg-element`, becoming emphasized on hover and active-base while
dragging/focus-visible (`🎚️Slider/🟦️.tsx:24-43`). The WGPU base currently uses
`theme.separator` and `theme.accent`, so a geometry-only replacement that
retains those colors would still disagree. Resolve the retained rail from the
theme's muted token and range/thumb from its semantic element token; do not
hard-code the screenshot colors. The CSS scale is likewise tokenized:
medium, large, and small are respectively 7, 9, and 5 times UI spacing
(`🎨️styling/🖌️ui/🎨️.css:731-744`), which explains the captured fractional
22.390625, 28.796875, 16, and 3.1875 pixel measurements.

Both retained WGPU paint paths still use the whole accepted node box as an
unfilled 4 px track and draw a 12 px accent knob. The production retained
walker does that at `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1228-1244`; its only text
phase paints the unit string, not the numeric value. The test-only tree painter
has the same geometry at `:2663-2684`. That latter function's comment still
names an older interpreter-only unit sibling as the visual authority, but the
current shared Slider is the actual component that interpreter renders. The
contract interpreter passes no `showValue={false}` override
(`📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1158-1161`), so the
numeric readout is not optional for a retained Slider. A unit adds a separate
interpreter sibling; it cannot substitute for the Slider's own value.

The current local rendering evidence is precise. In the captured Window
Options row, its 104 by 22.390625 px outer grid resolves into a 75.203125 px
track cell and a 28.796875 px readout cell. The track is 3.1875 px high, the
default thumb is 16 by 16 px, vertically centered on that track, and the range
fills from the track's logical start through the thumb percentage
(`🗑️generated/astra-runtime/browser26/react-slider-rows27.json`, row 1). The
parent row's inherited flow is RTL, which puts the readout physically left of
the track, but the Slider root itself has its default `dir="ltr"` and its
range/pointer math remains low-to-high left-to-right. Thus a WGPU visual fix
must **not** feed inherited `FlowInline::Rtl` into slider value geometry; row
placement and track direction are independent facts.

The minimal visual slice needs no `SliderProps` schema change. Its existing
`value`, `min`, `max`, `step`, and optional `unit` are enough
(`🧬️contract/🧩️component/🦀️.rs:361-373`), and shared
`format_ui_number` already supplies the React display contract. At accepted
layout/paint time, derive a named slider presentation with a value cell and a
track cell, paint the muted rail, filled range, and 16 px thumb in the track
cell, then paint the formatted numeric readout in its own cell. Preserve the
separate unit presentation only where the interpreter actually supplies it.

The cell geometry also has one required narrow event consequence. Current
pointer commit calls `slider_value_at(bounds, x, ...)` on the whole node box
(`⚡️events/🦀️.rs:618-622`), whereas React attaches that pointer handler only to
`sliderElement` inside `slider-track-cell`; the numeric span is its sibling
(`🎚️Slider/🟦️.tsx:500-538`). Reusing one `slider_presentation`/`track_rect`
helper in paint and EventRouter therefore preserves the existing paint-hit
contract: a pointer maps only inside the physical track cell and a one-click
on the readout changes no Slider value. The helper may use outer inline flow
to place the two cells, but its track percentage remains LTR. This does not
include double-click editing, keyboard, IME, pointer draft, or receipt work.

The existing native assertion is insufficient: it merely proves that a unit
adds more glyphs than no unit (`🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs:
297-314`). Add a language-neutral `slider-presentation` fixture and matching
React DOM and native draw/layout laws. Its cases should cover min, a fractional
middle value, max, and the captured outer-RTL/inner-LTR row. Assert the two
cell widths, rail/range/16 px thumb rectangles, centered vertical geometry,
formatted numeric text without a unit, and logical LTR fill even when the
outer row is RTL. Add an event row proving a click in the value cell is inert
and equivalent normalized coordinates in the track cell emit the expected
value. The existing Slider component test matrix
(`🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx`) is the independent React
behavior oracle; it already proves its logical RTL mode separately, which the
current retained schema does not expose.

There is a separate current accessibility mismatch for unit-bearing Sliders.
The shared Rust and TypeScript accessibility projections generate
`valueText = "{value} {unit}"` for such a Slider
(`🧬️contract/♿️accessibility/🦀️.rs:228`,
`🧬️contract/♿️accessibility/🟦️.ts:162`), and the browser WGPU mirror turns that
into `aria-valuetext` (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:104`).
Actual React's declarative interpreter instead creates `<Slider>` without an
`aria-valuetext` prop, then appends the unit only as a visual sibling
(`📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1158-1161`); the Slider
thumb itself supplies min/max/now but no value-text attribute
(`🧱️elements/🎚️Slider/🟦️.tsx:458-493`). The existing shared projection tests
therefore agree with each other without testing the live React DOM.

The narrow resolution is to have the React interpreter forward the contract's
unit value text to the Slider thumb and add a unit-bearing React DOM assertion,
which brings it to the published/native contract. Alternatively, change the
contract and both native targets to omit it; leaving only the React omission
creates a cross-renderer spoken-value difference. This is distinct from the
static Slider geometry work.

### Maximize resync: the actual ShellHost inputs are effective layout and IDs

The isolated `Mode` component compares `layoutKey` and ordered-ID-only
`windowsKey` (`🧱️elements/🎨️Canvas/🟦️.tsx:1348-1390`), but its actual
`ShellHost` caller always passes a concrete effective layout:
`shellLayout ?? resolveFrameworkLayoutSeed(...).modeLayout`
(`🏛️ShellHost/🟦️.tsx:12203-12210,12360-12368`). Consequently a raw
default/`None` to an explicit but structurally equal layout is not a Mode prop
change in the live application. The fixture's `identical-default` and
`identical-override` cases correctly retain maximize; do not add a contrary
source-transition law.

The native parity signature should therefore be the effective
`WindowLayout`, the session owner, and the IDs in the same order as React's
`modeWindows`, not a window kind. `ShellHost` builds `modeWindows` as declared
base window kinds in manifest order followed by `extraWindowInstances`
(`🏛️ShellHost/🟦️.tsx:12011-12201`); its titles, icons, bodies, locale, and kind
metadata do not enter `windowsKey`. The closest existing WGPU derivation is
already `ShellState::session_window_instances`: it starts with
`session.app.window_kinds` and appends only non-base dock instances
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7450-7457`). Compare its resulting `id`
sequence only. `DockState::window_instances` is layout order
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:699-708`) and is useful to discover extras,
but its `(id, kind)` pairs must not themselves become the source signature.
An icon/body/window-kind descriptor update that leaves the Mode layout prop
unchanged must retain maximize; a layout-window title is separately part of
`layoutKey`, as the implementation review below details. Including kind in the
roster signature would over-clear.

The native session owner remains a valid additional lifetime boundary, since a
new session can reuse the same visible inputs. Finally, maximize itself is pure
ephemeral React state (`Canvas:1488-1490`): both native paths immediately
following `dock.toggle_maximize` currently persist an otherwise unchanged
layout (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15445-15447,17633-17634`). Removing
only those persistence calls is required; genuine drag, resize, close, and
new-window topology paths should continue publishing their changed layouts.

### Maximize implementation review: raw layout is not the live React layout key

The landed WGPU `sync_dock` work correctly avoids a default-layout teardown on
an unchanged stored input, clears maximization after a changed input has been
applied, records the post-apply roster for the next refresh, and makes a
session-owner change independently invalidating (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:
7376-7418`). It also correctly leaves the ordinary focus and close persistence
paths alone. The two maximize-only persistence calls have been removed.

Its identity is nevertheless not identical to the live host's `layoutKey`.
WGPU stores the raw framework `WindowLayout` chosen from `layout_override` or
`app.default_layout`. React passes a *mode-layout projection* to `Mode`.
That projection contains each resolved `title`
(`🛠️ShellHelpers/🟦️.tsx:1680-1810`), and live `ShellHost` rewrites
`shellLayout` with `retitleWindowLayoutNode` when locale/terminology changes
(`🏛️ShellHost/🟦️.tsx:5979-5990`). `Mode` compares
`JSON.stringify(layout)` (`🎨️Canvas/🟦️.tsx:1349-1390`), so that actual
title-bearing layout-prop change clears `maximizedStackPath`. A title change in
the `windows` descriptor alone does not, which is exactly what the current
`presentation-only` fixture proves; it is not a live `ShellHost` locale
rerender.

For exact host parity, the native identity needs the same effective mode-layout
projection, including the currently resolved layout-window titles and corners,
or the explicitly accepted narrower policy must be recorded as a divergence.
The discriminating shared fixture needs two separate cases: a descriptor-only
title change retaining maximization, and a layout-window-title change clearing
it. The React test must rerender the latter with a changed `WindowLayoutNode`
title, while the native law changes the matching framework-layout title through
the real sync boundary. This is more meaningful than adding a synthetic
default/override-source transition, which live ShellHost does not expose.

The reverse mismatch is also present. A framework layout leaf carries
`template_id`, but the conversion passed to `Mode` emits only
`kind`, `id`, resolved `title`, and `corner`; template installation is a
separate pending-projection effect (`🛠️ShellHelpers/🟦️.tsx:1680-1810`). Thus a
template-only raw layout revision currently clears WGPU maximization despite an
unchanged React `layoutKey` and `windowsKey`. A template-only fixture must
retain maximization. More generally, the identity should project the same
mode-layout facts React serializes, rather than comparing the richer transport
layout: it includes structure, size, resolved title, and corner; an authored
framework `active_window_kind_id` is absent from this projection. It excludes
`template_id` and any raw value obscured by the resolved label.

The remaining roster detail is lower priority. React creates `modeWindows` as
all declared kinds followed only by extra instances whose `windowKindId` exists
in the declaration (`🛠️ShellHelpers/🟦️.tsx:1740-1779,5741-5759`). WGPU's
`session_window_instances` currently retains every non-base dock tab, including
an unknown kind (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7450-7457`). Filtering the
identity roster alone is not a complete fix, because React also reconciles the
unknown layout leaf away while WGPU still renders it. Do not make an
identity-only filter claim that implies full unknown-kind parity; that is a
separate input-validation/rendering issue.

### Exact modeled maximize identity and template application boundary

The native sync needs two values with intentionally different scopes.

1. **Application input** is the exact accepted framework-layout transport
   (`layout_override` when present, otherwise `app.default_layout`) plus the
   session owner. A change here must still execute the existing
   `DockState::apply_layout_diff`/`DockState::from_app` path, then reconcile
   live window ids, `world_projection_template`, and icon overrides. This is
   the state transition that applies a template update.
2. **Maximize identity** is the React `Mode` input: its projected layout plus
   the ordered `modeWindows` ids, with session owner as a native lifetime
   guard. Only this identity clears `maximized_stack`.

For an authored framework layout, derive the latter with the Rust equivalent
of `convertFrameworkLayoutNodeToModeLayout`
(`🛠️ShellHelpers/🟦️.tsx:1673-1705`): an axis is `{kind: row|column, size?,
children}`; a stack is `{kind: stack, size?, children}`; and a leaf is
`{kind: window, id: instanceId ?? windowKindId, title, corner?}`. Omit both
`templateId` **and** `activeWindowKindId`; conversion writes no Mode
`activeId`. A later live `Mode.onLayoutChange` payload may contain `activeId`,
but `captureCurrentFrameworkLayout` drops it again when it serializes the
framework transport. Applying the authored active window to the native dock is
therefore necessary but must not affect this authored-input maximize identity.

With no framework root, React supplies `createEvenWindowLayout` over declared
window-kind ids, or its synthetic `main` id when none are declared
(`:1743-1754`). The modeled id list is React `modeWindows`: declared kind ids
in manifest order followed by declared extra instances in layout order. IDs,
not window-kind metadata, feed `windowsKey`.

Titles belong to the projected layout because `Mode` compares
`JSON.stringify(layout)`. Initially a bare kind selects its manifest localized
label (a baked base title is only an unknown-kind fallback); an extra selects
its baked title or kind id. Both then pass through the app `windowKind` label
overlay by instance id (`🛠️ShellHelpers/🟦️.tsx:1654-1704,3156-3179`). The
closest native authority is `ShellState::dock_chrome_maps`
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:25471-25503`):
`window_layout_instance_titles` provides extra titles and its fallback is
`kind.label.resolve(active_terminology, active_locale)`. It supplies the
manifest/native core but no React app-label overlay.

There is a live-host qualification. `ShellHost` usually owns a non-null
`shellLayout`; a terminology or locale change replaces every known layout
window title with the localized manifest label, including extras
(`🏛️ShellHost/🟦️.tsx:5963-5999`). That changes the effective Mode prop and
clears React maximize. A native signature that models live host state must use
this retitled value. Retaining a baked extra title through locale change is not
live React parity.

Template-only changes prove the signatures must remain distinct. React gathers
a known leaf's template into `pendingProjections` and installs it outside its
Mode prop (`🛠️ShellHelpers/🟦️.tsx:1743-1781`). Native sync currently obtains it
from the dock and refreshes `world_projection_template`
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7403-7413`). A template-only transport
revision must take the application path and update that map, while its equal
maximize identity retains maximize. React skips unknown-kind templates before
collection; native currently admits their leaves, which remains a separate
input-validation/rendering divergence.

Required discriminating laws: unchanged effective default/override retains;
an authored `activeWindowKindId` revision applies dock focus yet retains; a
template-only revision updates the projection map yet retains; a resolved
layout-title or locale-retitle revision clears; descriptor title/icon/body
updates with unchanged Mode layout retain; adding or removing a recognized
instance clears through projected layout and ordered roster. The native tests
should observe `world_projection_template` and `maximized_stack` after the
real `sync_dock` boundary, not only an isolated signature helper.

### Current modeled-identity review and the separate unknown-window gap

The current WGPU implementation follows the two-boundary design. Its
`DockInputIdentity.source` controls raw layout application, while its projected
`mode_layout` and ordered roster decide whether maximize clears
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3495-3532,7418-7464`). The projection clears
the raw stack-active and template fields, remaps an extra leaf to its instance
id, resolves bare-kind labels, and retains an extra's baked title until the
locale/terminology retitle transition. The source-only route retains the old
maximize path after applying its diff; this is sound for active/template-only
transport revisions because those do not alter the projected tree. The latter
template map remains intentionally sticky while the pane is live, matching
React's `pendingWorldProjectionByWindowId`, which is cleared only on pane
close (`🌐️World3dHost/🟦️.tsx:5271-5289`).

The revised native law now observes the retitled chrome label as well as the
maximize reset, and the browser test independently rerenders a known extra
through `retitleWindowLayoutNode`. This is the right observable pair. An
authored raw active-window value must not be asserted to switch focus: React's
framework conversion omits `activeId`, and WGPU `apply_layout_diff` likewise
keeps a surviving current active tab. Its separate active-window view state is
the focus authority.

Unknown layout leaves are an independent renderer-input problem, not a roster
shortcut. React converts a raw unknown leaf into the layout prop, so it still
changes `layoutKey`; it excludes that leaf from `extraInstances` because its
kind is undeclared, and `Mode.resolveModeLayout` then prunes it against the
`modeWindows` id set before it renders. WGPU both constructs a dock tab for
every raw leaf (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:780-849`) and appends every
non-base dock instance to its view roster (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:
7463-7485`). Filtering only that roster would leave the unrenderable dock tab
visible while falsely appearing to solve parity.

When this gap is scheduled, validate a native application-layout projection
against React's `resolveModeLayout`: allow declared base ids and only extras
whose declared `windowKindId` is known, then prune/collapse the rendered tree
before `DockState` receives it. Keep the raw Mode-identity projection separate
because an unknown raw leaf still affects React's prop key. A malformed leaf
whose unknown kind supplies an id equal to a declared base id is intentionally
not a simple filter-by-kind case: Mode reconciles by id, so its explicit
handling belongs in that dedicated normalizer.

### Commands category body: tree, direct execution, and header-action contract

The current React category body is not a generic settings stack.  It is a
`TreePanelConfig` whose rows are `TreeDataItem`s.  `buildCommandCategoryTree`
creates a zero-argument row with `onClick: () => onExecute(entry)` and no
argument object; an argument-carrying row instead toggles its sole
`expandedCommandId` key.  The one-argument-command category auto-expands
without writing that state.  When an argument command is expanded, its form
section precedes the list, each argument is a `TreeDataItem.control`, and
Execute/Reset are **section-header actions**.  The expanded command is absent
from the list.  See
`🛠️ShellHelpers/🟦️.tsx:5089-5169` and the direct React oracle at
`🧪️tests/🔬️engine-contract/🟦️.ts:10440-10517`.  The mounted Tree renders
these rows as `role="treeitem"`, with its normal single-select behavior and
`data-activatable`, not as buttons or free text
(`🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:2914-3010,3385-3478`).

The WGPU category builder presently emits a `Stack` of `Section`s.  Its form
uses `UiField`s and plain `UiButton`s; its list calls
`build_command_panel_row`, which yields a standalone `Select` for only a few
OS arguments and inert `Text` for every other entry, including every
zero-argument command.  This is therefore three connected production gaps:
zero-argument commands cannot run; argument rows do not use the tree's
expansion affordance; and the produced accessibility/layout vocabulary is not
the React tree.  The exact source is
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22259-22411`.  The intended native lowering
already has precedents in Settings and Display: `UiNode::Tree` with
`UiTreeSectionNode`/`UiTreeItemNode`
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5774-5786,8667-8673`).  The staged argument
helper must lower to a tree item's inline control rather than a `UiField`.

The direct route must take a fully qualified stable command key, re-resolve it
against the **current** `self.resolved_commands()`, and require both
`definition.in_palette` and no declared arguments.  Only then may it call
`apply_os_command(id, None)` for OS or `dispatch_command` with the resolved
non-OS address and an empty argument map.  It must neither accept an
unqualified command id nor deserialize a caller-provided owner/address.  The
key includes owner scope (`os`, `plugin`, `app`, or active `mode`) at
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22429-22437`; re-resolution therefore rejects
a delayed row from an old mode/session and an `inPalette: false` command
without sending either to a new owner.  Keep `executeStagedCommand` for the
argument form.  The existing `command_search_items` and keybinding dispatch
paths demonstrate direct current-owner execution; `runOsCommand` currently
only reaches OS and cannot be the generic route
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11113-11125,11642-11715,21879-21945`).

There is a shared protocol prerequisite for exact form parity.  React's
`TreeSectionAction` has a disabled state, and its Execute action is disabled
until all required arguments are staged
(`🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:498-513,686-704`).  Neither legacy
`UiTreeSectionNode` nor the retained native `TreeSection` carries header
actions (`🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2525-2541`;
`🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:221-229`).  Existing `RowAction` and
`UiTreeItemAction` lack disabled/presence, so they cannot represent this
Execute gate.  The domain-neutral shape is a TreeSection header-toolbar
relation, analogous to `TreeItemProps.inline_toolbar`, whose direct child is a
`ContainerRole::Toolbar` of ordinary Button records.  Those records already
own their action bindings, disabled presence, accessibility, and input
semantics.  Thread that relation through the contract, legacy WGPU mirror,
interpreter assembler, retained tree layout, paint, hit registration, and
accessibility.  A dedicated bounded header-action type with `id`, action,
label/icon, and presence is a viable alternative if the renderer cannot host
the toolbar; reusing `RowAction` or adding form-body tree rows would silently
lose the disabled/header contract.

The native law belongs beside the existing category-form law in
`🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:878-925`; the existing
palette registry law at `🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs:876-883`
is only a search-item oracle, not a category-body test.  The fixture should
assert: owner-qualified OS/plugin/app/mode zero-arg rows are treeitems and
execute their current owner; an ineligible command is absent; two argument
rows first disclose exactly one selected form and hide that row from the list;
a singleton argument category begins with the form; the required Execute
header button is disabled then enabled by staging; and a stale old-mode key
causes no program invocation.  React's independent mounted oracle should
invoke real `role="treeitem"` rows and verify the corresponding list/form
shape for both English and German.  No test was run for this audit.

### Slider readout typography is state-specific inside a compact measure tree

The Chromium observation is explained by the actual React cascade, rather
than a different compact driver.  `WindowMeasuresTree` unconditionally has
both `data-slot="window-measures-tree"` and `text-tiny`
(`🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4504-4553`).  The compact tokens are
`text-2xs = 9.6px`, `text-xs = 11.2px`, and `text-sm = 12.8px`
(`🎨️styling/🖌️ui/🎨️.css:803-810`; generated once in
`🎨️styling/🔤️tokens/🦀️.rs:376-378`).  The Window Measures selector then
explicitly applies `9.6px !important` to a static
`[data-slot="slider-value"]` (`🎨️styling/🖌️ui/🎨️.css:7481-7494`).  That is
why the saved browser capture's resting readout is 9.6px, even though
`Slider` itself asks for `text-xs`
(`🧱️elements/🎚️Slider/🟦️.tsx:39-40,511-535`).

Editing is deliberately different.  The Slider swaps that span for an
`Input` with a local `text-xs` class, but Input itself supplies
`text-base md:text-sm`; at the desktop capture width its responsive
`md:text-sm` rule wins and gives the native `<input>` 12.8px
(`🧱️elements/🎚️Slider/🟦️.tsx:518-530`,
`🧱️elements/✏️Input/🟦️.tsx:474-487`).  The measures CSS does not select
`[data-slot="input"]`.  It follows that the required state matrix is:

| owning tree | resting readout | edit input |
| --- | ---: | ---: |
| standard | 11.2px (`text-xs`) | 12.8px (`md:text-sm`) |
| compact Window Measures | 9.6px (`text-2xs`) | 12.8px (`md:text-sm`) |

The current retained WGPU paint has an important opposite error to the
reported `11.2/12.8` interpretation.  `retained_node_paint_step` detects any
compact owning tree and mutates **both** `font_size_small` and
`font_size_body` to `SIZE_TINY`; `SIZE_TINY` is 9.6px
(`🖌️paint/🦀️.rs:919-933`; `🖥️chrome/🦀️.rs:16-20`).  The Slider's paint arm
then chooses `font_size_small` at rest and `font_size_body` while its local
editor exists (`🖌️paint/🦀️.rs:1257-1276`).  Thus its current compact-tree
readout is 9.6px, but its edited text is incorrectly 9.6px too.  The global
default Theme values are correctly 11.2/12.8
(`🎨️theme/🦀️.rs:121-133,268-280`); this is not a justification to hard-code
either number in Slider paint.

The minimal safe Slider slice is to retain the unscoped base
`Theme::font_size_body` before the compact inherited paint scope, and use it
only for a `UiSliderNode` that owns an active text editor.  Keep the scoped
`font_size_small` for the resting readout.  This preserves caller theme
customization and produces all four matrix cells without adding a literal
pixel value or changing the already-correct compact resting glyph.  Do not
remove the compact scope wholesale in this slice: Select and several tree
labels presently rely on it for their direct Window Measures CSS equivalence.
Those components need their own component-specific audit before the inherited
scope can be redesigned.

The discriminating native law belongs with
`🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs:319-348` and must mount the
same Slider below both `TreePresentation::Standard` and `Compact`, then
repaint it with and without `slider_draft_value`/the retained edit state.  It
should assert font atlas glyph requests (or emitted glyph size) of
`11.2/12.8` for the standard pair and `9.6/12.8` for the compact pair.  Pair
it with the mounted React Window Measures oracle already rooted at
`🐚️Shell/🧪️tests/🎚️window-measure-controls/🟦️.tsx:60-68`; inspect the span
before edit and its `input[data-slot="input"]` after the double-click rather
than asserting utility-class strings.  No test was run for this audit.

### Graph Timeline drops co-authors and every supplied avatar image

This is a current, reachable scene-rendering defect, independent of the
in-flight Table, VFS, TextEditor, Slider, Commands, and Dock work.  The React
surface host passes `columnsJson` unchanged to `HistoryTable` and exposes its
row-selection action (`🌳️GraphTimelineHost/🟦️.tsx:70-82`).  For every history
row React maps **every** `column.authors` entry to a `TableAvatar` in authored
order, with `-space-x-2` overlap (`🕰️HistoryTable/🟦️.tsx:133-135`).
`TableAvatar` uses its `icon` string as an image source, keeps the initials
fallback until that exact image has loaded, and gives the fallback the author's
accessible image label (`📻️TableAvatar/🟦️.tsx:27-85`).  Thus a checkpoint with
two authors has two overlapping identity marks, and a supplied avatar is
visible once loaded.

The production WGPU parser discards both `HistoryColumnAuthor.id` and
`HistoryColumnAuthor.avatar`: `HistoryColumnAuthorJson` has only `name`
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5214-5217`).  Its painter then selects only
`column.authors.first()` and unconditionally paints one initials circle
(`:5387-5393`).  A second author and every image source therefore cannot
possibly reach either paint or the decoder.  The current native tests mask
this: their `column` helper always supplies `authors: Vec::new()`
(`🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs:3-5`), and the initials law
only exercises a scalar name (`:54-60`).  This is not an inference from old
reports: the live parser and painter are the source of the loss.

The narrow owner is `🎞️Scenes` WGPU GraphTimeline.  Extend its private author
decode shape to `id`, `name`, and optional non-empty `avatar`; no wire-schema
change is needed because the TypeScript `HistoryColumnAuthor` already owns
those fields (`🕰️HistoryTable/🟦️.tsx:17-21`).  Paint the supplied vector in
order as `20px` circles with an `8px` negative advance (React's
`size-small` is `5 * --ui-spacing`, and `-space-x-2` is two compact spacing
units; `🎨️styling/🖌️ui/🎨️.css:732-739,6719-6721`).  For each author use a
stable key derived from `host_id`, `checkpoint_id`, and author `id` (fall back
to that row index only if `id` is absent), then the existing
`interpreter::current_ui_image_key` / `push_rounded_raster_quad` behavior
already proven for VFS avatars
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:10136-10151`).  Its exact-source guard is
essential: it prevents an old decoded avatar from appearing under a changed
author image.  Retain initials only while that exact source has no pixels;
do not add another cache or a generic avatar protocol in this slice.

Add a domain-neutral GraphTimeline fixture with a single visible checkpoint
whose authors are `Ada Lovelace` and `Grace Hopper`, the first with a valid
data-image and the second without one.  The mounted React oracle should
assert two `[data-slot="avatar"]` elements in that order, overlap geometry,
and the first image's `alt`; it can live beside the current host interaction
test in `renderer/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts:86-103` or the
more direct host contract assertion at
`renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:7052-7072`.  The native
law belongs with `🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs`: assert two
rounded avatar records with the `12px` x-offset, fallback initials before the
image receipt, raster only after its own receipt, and no stale raster when
the source changes.  Pair this with the existing interpreter exact-source
law at `🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs:241-250`.
No test was run for this audit.

### Browser deadlines need a one-shot transport, not page rAF polling

The current browser path loses the distinction the new native scheduler is
about to make.  `BrowserRendererWorker::tick` emits `requestFrame` when
either runnable work *or merely a future deadline* exists
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:340-390`).
`runFrameTurn` forwards only that boolean through the Worker message
(`🎞️frame-worker/🟦️.ts:392-420`), and `BrowserFrameTransport.receive` turns it
straight back into `requestAnimationFrame` (`🚚️browser-frame-transport/🟦️.ts:582-623,953-1000`).
Consequently an idle 400 ms tooltip dwell is currently a 60 Hz page-to-worker
loop.  The Rust result already has `continue_frame`, but the frame-worker
uses it only as its private task-queue return value and drops it before the
page transport.

The minimal cross-realm protocol is a pair of scheduling facts on the existing
`kind: "frame"` result:

* `continueFrame: boolean` preserves the already-computed runnable-work fact.
* `nextDeadlineDelayMs?: number` is a finite, nonnegative **remaining
  duration**, present only for the earliest future deadline.  It must be
  calculated in `BrowserRendererWorker::tick` after the frame step from its
  scheduler's local clock, then clamped at zero.  Do not send an absolute
  `Deadline.due`: the page's `performance.now()` origin is not the worker
  `OsClock` origin.  `_timestamp_ms` is currently unused by the Rust tick and
  must not be repurposed as that authority.

Add those fields to `BrowserTickOutput` in
`🌐️browser-worker/🦀️.rs:48-58`, carry them in the parsed result and the `post`
call in `🎞️frame-worker/🟦️.ts:398-419`, and add them to the `frame` arm of
`BrowserFrameWorkerMessage` in `🚚️browser-frame-transport/🟦️.ts:343-358`.
There is no generated binding or second page callback to update: the
frame-worker's JSON parse is the sole Rust-to-TypeScript handoff.

`BrowserFrameTransportOptions` already injects `now`, `setTimer`,
`clearTimer`, `requestAnimationFrame`, and `cancelAnimationFrame`
(`🚚️browser-frame-transport/🟦️.ts:389-402`).  Give the transport one private
`deadlineTimer` alongside `rafHandle` (`:456-459`).  On a current-generation
frame result, its scheduling rule should be ordered as follows:

1. A queued input, stale-generation retry, or `continueFrame` cancels the
   deadline timer and calls the existing `requestFrame`; immediate work wins.
2. Otherwise a finite `nextDeadlineDelayMs` replaces the one timer.  The timer
   callback clears its handle and calls `requestFrame`; it must not call
   `flush`/`runtime.tick` directly, so batching, generation checks, and rAF
   pacing retain their existing owner.
3. With neither fact, cancel the timer.  `close`, `fail`, `quarantine`, and
   `clearQueues` must also cancel it.  A stale frame result may request the
   existing immediate retry but must not arm or replace a timer from its old
   generation.

Canceling the delayed timer at the beginning of `requestFrame` is also
necessary: a pointer event that arrives during a dwell subsumes the old wake;
the subsequent current frame result alone may re-arm its successor.  This is
bounded (one timer and one rAF), keeps the worker opaque to page timing, and
requires no runtime library.

Extend the existing neutral
`renderer/🧑‍🎨engine/🧫️fixtures/🧵️frame-turn-scheduling/{📐️schema,🔣️}.json`
rather than adding a test-only timing vocabulary.  The cases should cover a
future dwell with zero rAFs before its virtual timer fires; replacement by an
earlier deadline; an input/runnable turn canceling it; a late old-generation
frame unable to replace the current timer; and close/fault leaving no pending
timer.  `renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` already
has a virtual clock/timer harness at `:829-865`; add a fake rAF queue there.
The existing source law at
`renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts:438-462`
explicitly asserts that the Rust continuation source contains no
`next_deadline`; revise it to assert the distinct delay serialization instead.
No test was run for this audit.

### Tooltip Reveal needs a private presented canvas record

`TooltipStep::Reveal(NodeId)` now becomes reachable from
`EventRouter::advance_clock` (`🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:1521-1572`),
but it still does not open anything.  Reusing `Ui::open_overlay` would be
wrong: that API requires an authored content-root subtree, whereas a generic
tooltip has only the hovered anchor.  The retained overlay chrome path
explicitly excludes `Tooltip` because its own surface is supposed to be
painted by `paint_tooltip` (`🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:3020-3034`),
and `paint_tooltip` currently has no production caller
(`🖌️paint/🦀️.rs:3592-3603`).  The existing overlay stack therefore cannot
accommodate this consumer without manufacturing an invalid document overlay.

Keep ownership in the existing per-window presentation state.  Add one private
`presented_tooltip` record to `UiWindow` next to `presented_tree`,
`presented_router`, and `presented_interaction_epoch`
(`⚙️engine/🦀️.rs:70-122`).  It should contain the source `UiSurfaceToken`, the
presented document id (not a candidate arena `NodeId`), the resolved text, and
the presented accessibility generation/interaction epoch.  On
`advance_window_clock` receiving `Reveal(node)`, resolve the label through
`Ui::tooltip_label` while that exact presented tree is selected
(`⚙️engine/🦀️.rs:3081-3119`); a hidden or unlabeled node produces no record.
Store a document id rather than the raw node: acknowledgement swaps the arena
trees (`:1204-1226`), and a raw id can then address the retired revision.

On the candidate's next frame, map the stored document id into its candidate
tree, measure with `tooltip_surface_size`, and use
`resolve_overlay_placement_side(OverlayAnchor::Node(...), ..., Tooltip's
top/center placement, window flow)` before calling `paint_tooltip` in the
overlay draw route.  The record's text remains the accepted text; only a
document-id mapping may supply the current candidate geometry.  If that
mapping, surface token, generation, or window is no longer current, suppress
and clear it.  This mirrors the existing presentation/candidate split rather
than reading arbitrary candidate labels after a new ingress.

Clear it whenever the presented router no longer has the same revealed leaf,
on `TooltipStep::Dismissed`, surface retirement, and document replacement.
The router already resets `hover_revealed` on a different pointer target
(`⚡️events/🦀️.rs:1450-1500`), while both `dispatch_event` and
`dispatch_pointer_event` already advance the presented interaction after that
dispatch (`⚙️engine/🦀️.rs:2885-2953`).  A narrow internal
`revealed_tooltip_node()` accessor is sufficient for this comparison; do not
invent another shared overlay state or command.  React closes immediately on
pointer leave/cancel and blur (`💡️ChromeControlHint/🟦️.tsx:34-75,110-127`), so
the generic record does not need the router's overlay-only hover-out delay.

This completes canvas visual behavior only.  React also attaches the open
tooltip with `aria-describedby` and portals a real `role="tooltip"`
(`💡️ChromeControlHint/🟦️.tsx:78-109`); the current canvas accessibility mirror
has no tooltip projection.  Keep that as a later accessibility slice instead
of claiming this private painted record supplies DOM semantics.  The focused
native law should prove: idle time paints nothing; dwell reveals the accepted
label and shortcut above its accepted anchor; pointer move, replacement,
discarded candidate, and close remove it; and an old document's label cannot
appear after an acknowledgement.  No test was run for this audit.

### Dock ingress review: current-snapshot extras only

The new WGPU `DockState::reconcile_app_windows` has the right structural
shape for the four fixture cases: it filters before collapsing the tree,
repairs stack activity, re-resolves active/maximized paths by id, and clears a
template when it repairs a malformed kind from an allowed identity
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:205-235`).  This matches React's actual
pipeline.  `resolveFrameworkLayoutSeed` creates extras only from incoming
layout leaves whose **current** `windowKindId` exists, and only those leaves
contribute projections (`🛠️ShellHelpers/🟦️.tsx:1759-1779`).  `Mode` then
reconciles its raw layout against base kinds plus exactly that new extra array
(`🎨️Canvas/🟦️.tsx:683-689,285-301`), collapsing an all-pruned tree to an empty
stack.  The mounted fixture at
`renderer/🧪️tests/🪟️maximize-resync/🟦️.tsx:38-48` observes those tabs and
stack paths directly.

There is one important lifetime boundary for the pending “known extra id”
generalization.  A layout update replaces React's extra-instance state with
the seed derived from that update (`🛠️ShellHelpers/🟦️.tsx:3373-3378`), rather
than carrying extras from its prior dock.  Therefore an invalid leaf may be
repaired only from a valid mapping present in the **same incoming layout**;
it must not be admitted merely because a previous WGPU dock had that instance
id.  The present native implementation builds `instances` from the dock after
the new layout diff, which can express a same-snapshot valid extra
(`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:225-230`, called after
`apply_layout_diff` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7430-7442`).  Do not
move that source to a pre-diff roster.

The discriminating follow-up regression is sequential.  First accept a
layout containing valid `{ windowKindId: "b", instanceId: "b-2",
templateId: "good" }`.  Then replace the framework layout with an invalid
`{ windowKindId: "missing", instanceId: "b-2", templateId: "bad" }` and a
surviving `a` leaf, with no valid `b-2` leaf.  React produces no `b-2` extra,
so `Mode` removes the alias, its template, and its body.  Native must do the
same, retire the old projection, and never retain `good` or accept `bad`.
The converse same-snapshot alias case should establish a valid `b-2 → b`
mapping from a valid leaf in that *same* update before it repairs another
malformed `b-2` leaf.  No test was run for this audit.

### EventRouter clock: live UI engine has no deadline-to-host bridge

The prior conclusion that the renderer does not instantiate `Ui` was wrong. The
production Interpreter owns one process-wide, cross-worker
`UI_ENGINE: WorkerCell<ui_wgpu::wgpu::Ui>`
(`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:25-47,287`). Both normal and
presented pointer dispatch borrow that same engine, drain its commands, and
apply them before returning (`:947-978`). The Shell also borrows it for
document ingress, reconciliation, layout, and paint
(`:3179-3338`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:21313-21317`). The
correct defect is narrower: no production path calls `Ui::advance_clock`; its
current callers are package tests only.

The source-compatible clock authority is
`semio_framework_job::default_now_us()`, which delegates to
`semio_framework_trace::try_now_us()` (`🧵️job/🦀️.rs:74-94`).  On native it
uses one process-wide `OnceLock<Instant>`; host and worker threads share that
epoch (`⏱️trace/🦀️.rs:67-86`).  Bare WASM deliberately reports `None` until
the browser installs its `performance.now()` authority
(`⏳️async/⏱️clock/🦀️.rs:8-35`).  Therefore the worker may read this clock
directly; it must treat `None` as no-timer rather than synthesizing a
timestamp.  No new `FrameBuildInputs` field is necessary.  `app_now_ms()`
remains an epoch-millisecond input and must not enter this path.

Current `Ui::advance_clock`, router deadlines, and DrawList time are all
`f32` (`⚙️engine/🦀️.rs:3074`; `⚡️events/🦀️.rs:1040-1044,1110-1119`;
`🖍️draw/🏷️types/🦀️.rs:219,826-834`).  Widen interaction-clock and deadline
comparisons to `f64`; a draw-only `f32` phase conversion, if required by
paint, must never become the timing authority.  Before a document frame
seals, the worker must lock `UI_ENGINE`, drain already-pending UI commands,
advance the exact presented router, drain the new commands, release the lock,
and apply the combined commands through the existing Interpreter boundary.
That keeps timer mutations in the same worker-owned accepted interaction
authority as pointer and keyboard input.

The router needs an explicit bounded clock result instead of the current
`(TooltipStep, Vec<UiCommand>)`. Its repeat operation mutates a NumberStepper
draft but reports `TooltipStep::Idle`
(`⚡️events/🦀️.rs:1520-1544,1822-1841`). Consequently
`Ui::advance_clock` skips `advance_presented_interaction()`, because it
rebases only for a non-idle tooltip step (`⚙️engine/🦀️.rs:3074-3088`). A
private `RouterClockStep { tooltip, commands, changed, next_deadline }`
should mark repeat mutations and tooltip reveal/dismiss as `changed`; only a
changed presented router/tree rebases its candidate and interaction epoch.
Idle ticks must not cause candidate churn. `next_deadline` is the minimum of
an armed repeat's `next_at`, tooltip dismiss, and unrevealed hover dwell.
Select typeahead expiry is not a visible autonomous mutation and needs no
wake.

The absolute microsecond deadline now leaves the worker through
`RuntimeMailbox` as a replace-or-clear atomic.  The host subtracts its own
`default_now_us()` reading, then adds the nonnegative remaining duration to
`OsClock::now_seconds()`; this handles the intentionally different `OsClock`
epoch without comparing values from two epochs. A compare-min publication
cannot implement cancellation. The atomic is aggregated from exactly the
accepted Shell roster, one surface per `RetainedClock` boundary, rather than
from all `Ui` slots. A frame-owned publication must not use the generic host
waker, because that waker renumbers input generation; the completed frame's
existing `FrameReady` is the host notification.

The OS host publishes the converted due time to a replaceable
`FrameScheduler` key; native winit consumes it as `WaitUntil`
(`🪟️winit-app/🦀️.rs:1126-1130`). Browser tick now carries a relative
`nextDeadlineDelayMs`, beside immediate `requestFrame`, through Rust, the
frame Worker, and the page transport. The page replaces one timer and re-enters
through rAF when it fires, so a future deadline does not create a continuous
rAF loop. Pointer input, release/cancel, surface retirement, or a newer
deadline replaces or clears the key/timer.

Tooltip completion remains a separately visible gap. `TooltipStep::Reveal`
documents that its caller must reconcile/open an overlay, but no production
consumer or `UiCommand` variant exists for it
(`⚡️events/🦀️.rs:861-869,983+`). Advancing the clock alone therefore delivers
repeat state and dismissal timing but cannot make a generic document tooltip
appear. The Shell chrome tooltip is independent and also checks its system
clock only while a frame is already rendering
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:27643-27731`); it needs a scheduled deadline
or an explicitly shared clock path.

The renderer-level acceptance law belongs under
`renderer/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs` plus the standalone OS-host
test. Use a presented NumberStepper: pointer-down publishes its first
deadline; at 599ms no value changes; at 600ms the presented value changes
locally while no additional application action is emitted; release, cancel,
and retirement clear the deadline. Seal a successor or discard the candidate
before the old deadline fires and prove the old generation cannot mutate or
schedule another wake. Pair it with a language-neutral timer fixture and the
existing React hold oracle
(`renderer/🧪️tests/⚙️settings-general-layout/🟦️.ts:125-138`). The existing
router-only repeat test (`🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs:793-807`)
does not prove the worker-to-host deadline bridge. No test was run for this
audit.

### Retained clock integration review: native publication must not use the generic wake

`Shell` correctly snapshots staged retained ids into
`presented_input_geometry_staging` only at seal
(`🐚️Shell/🦀️.rs:14704-14736`) and swaps it only after the matching GPU
acknowledgement (`:14758-14777`). `FrameBuildPhase::RetainedClock` reads only
the published side (`🧊️renderer/🦀️.rs:16977-16988`), so a discarded candidate
cannot tick a newly staged surface. `Ui::advance_window_clock` also resolves
the current surface token and declines closing windows (`⚙️engine/🦀️.rs:3081-3099`).

The lower-level `Ui::seal_presented_input_candidate` originally violated that
same rule for hidden surfaces. While sealing candidate B, it called
`presented_router.suspend_clock()` for every current surface omitted by B
(`⚙️engine/🦀️.rs:1137-1148`). This clears an accepted NumberStepper repeat and
can advance its presented interaction before B's pixels are acknowledged;
discarding B then leaves accepted A spuriously cancelled. The candidate's
visibility must be captured under the exact seal witness and its current and
presented routers suspended only when that witness acknowledges. Discard and
late acknowledgement must clear that visibility receipt without mutating A.
SolTree is moving this boundary; the post-change audit must verify it uses the
seal's token-qualified visibility snapshot rather than the mutable staging
list at acknowledgement.

Pure time is correctly protected from stale presentation. A tick advances the
presented interaction epoch only when `RouterClockStep.changed` is true
(`⚙️engine/🦀️.rs:3090-3098`), which starts the normal candidate rebase
(`:192-225`). Seal refuses a visible presented surface with no ready candidate
(`:1137-1158`), and the production document route reconciles and paints before
Shell seals (`🗣️Interpreter/🦀️.rs:3444-3604`). Idle time therefore leaves the
candidate intact; a due repeat/reveal cannot accept pixels from before its
mutation. No stale-candidate defect was found in this sequence.

There was one native self-supersession route. `RetainedClock` publishes its
deadline from inside a live frame build (`🧊️renderer/🦀️.rs:16977-16988`).
`RuntimeMailbox::publish_retained_control_deadline` initially invoked the
shared runtime waker (`:13529-13535`), which sends `HostUserEvent::Wake`
(`🪟️winit-app/🦀️.rs:968-974`); that handler unconditionally increments
`frame_generation` (`:991-999`). A build could therefore invalidate its own
presentation witness before its ordinary `FrameReady`. The deadline update is
frame-owned and `FrameReady` already invalidates/presents before
`present_snapshot` reads the atomic (`:342-345`), so the publication must be
atomic-only, without the generic completion wake. Root is applying that narrow
correction.

The native law must start a live frame at generation `g`, publish/replacement/
clear its retained deadline during its clock phase, and prove no generic Wake
and no generation change occur. Its later FrameReady must install or clear
`RETAINED_CONTROL_CLOCK`. Pair it with an accepted stepper law: before-due
time keeps the interaction epoch; at-due time rejects the old witness; a
reconciled successor acknowledges; close removes the former roster member and
deadline. Add the hidden-surface cancellation law: seal B excluding a
repeating accepted A, verify A keeps repeating through B discard, then seal
and acknowledge B and verify the repeat stops exactly once. No test was run
for this audit.

### Browser delayed-wake integration review

The narrowed transport is coherent. Rust makes `request_frame` the immediate
`continue_frame` demand while transferring only a finite nonnegative relative
`next_deadline_delay_ms` (`🌐️browser-worker/🦀️.rs:379-390`). The frame Worker
keeps `continueFrame` private and forwards the delay (`🎞️frame-worker/🟦️.ts:400-422`).
`redraw_offscreen_worker` consumes a due scheduler key before its admitted
browser redraw (`🪟️winit-app/🦀️.rs:182-185`), avoiding a permanently overdue
caret/deadline key.

The transport clears a delayed timer before every immediate `requestFrame`,
replaces one timer only for a current-generation idle response, and returns
timer expiry through `requestFrame` and rAF rather than directly to `flush`
(`🚚️browser-frame-transport/🟦️.ts:570-615,985-1009`). Its close, fault,
quarantine, and queue-clear paths reach `clearDeadlineTimer`
(`:694-704,1011-1079`). A stale response takes the immediate-retry branch and
cannot replace the current timer. Existing virtual-timer coverage checks
no-rAF-before-due, input cancellation, stale reply rejection, and terminal
cleanup (`renderer/🧪️tests/📨️browser-frame-transport/🟦️.ts:76-188`).

One additional test is advisable: fire the delayed timer while its previous
batch is in flight, then acknowledge that batch. Assert that
`frameRequested` causes exactly one later rAF/batch (`🚚️browser-frame-transport/🟦️.ts:964-973`)
and a current reply replaces the timer. No production defect was found in the
browser delayed-wake ownership path. No test was run for this audit.

### Accepted hidden-clock receipt and deadline cancellation update

The hidden-clock correction is now present. `UiSealedVisibilityCandidate`
captures a fixed-slot `(UiSurfaceToken, visible)` roster under the presenter
witness when the candidate seals (`🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:846-849,1151-1178`).
It performs no router mutation at seal. A discard removes only the matching
receipt (`:1268-1289`). A matching acknowledgement swaps the candidate
revisions first, then reads and consumes that same receipt; it calls
`suspend_clock` only for its token-qualified hidden entries (`:1235-1265`).
Thus neither discarded B nor a late B acknowledgement can cancel accepted A,
and a closed/reused id cannot retarget the suspension to its successor.

The focused laws are appropriately discriminating:
`window_clock_keeps_a_noop_candidate_sealed_and_invalidates_it_once_when_hold_repeat_changes_state`
proves B-discard preserves A's armed repeat, B-ACK clears it, and an old held
press cannot catch up after hiding
(`🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs:275-321`).
`a_late_hidden_visibility_ack_cannot_suspend_a_reused_surface_id` retires the
old surface, re-admits the same id in the slot, arms a successor repeat, and
accepts the old witness without clearing the successor deadline (`:323-359`).
I did not run these tests. The sole non-blocking hardening opportunity is to
skip a token that is already in its close ladder before calling
`suspend_clock`; the current token check already prevents successor mutation,
and the presenter gate appears to make a close/ack overlap unreachable.

The accepted roster itself now also carries the exact token:
`staged_retained_clock_surfaces` snapshots `(id, token)` from the staged
visibility roster (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:5222-5228`),
`Shell` transfers it only at ACK (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14704-14713,14759-14774`),
and the clock sweep verifies the live token before it stamps time
(`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:953-963`). Direct pointer, key, and
accessibility delivery still stamps their individually addressed live surface
by id before dispatch (`:947-995,1152-1160`), which is necessary to establish
an initial interaction clock and is separate from the accepted-roster sweep.

One remaining deadline ownership gap is real. A frame's `RetainedClock` phase
publishes the old accepted minimum before a later B acknowledgement
(`📺️renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16973-16984`). ACK can then
suspend B-hidden owners, but the mailbox atomic is not refreshed until another
build, leaving a stale timer. The narrow fix is a read-only,
token-qualified `Shell::presented_retained_clock_deadline` after the geometry
swap: read each new presented router's `next_clock_deadline`, retain only an
equal live token, and take the minimum without advancing time. Carry that
value from `AppPresentPhase::Acknowledge` through `AppPresentStep::Complete`
to the shared winit completion branch, where that branch already owns the
mailbox and can replace/clear `retained_control_deadline_us`. This covers
native and browser without a global UI-to-host link or a generic wake.

The regression needs an accepted A with a held stepper and published deadline
`d`, followed by a B that hides it. After B completes, before any successor
build, assert the mailbox is clear and the host scheduler removes
`RETAINED_CONTROL_CLOCK`; discarding B must retain `d`. Repeat with an
old-token/same-id successor and prove the successor cannot contribute or lose
a deadline. No test was run for this audit.

There is a related tooltip cancellation requirement for the EventRouter and
new tooltip consumer. `suspend_clock` currently clears only `stepper_repeat`
(`🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:1541-1543`). It leaves
`hover_since`, `hover_revealed`, and `tooltip_dismiss_at` intact. Once the
presented tooltip record is painted, an acknowledged B that hides a revealed A
could re-show that old tooltip immediately when the surface returns, or use
the old dwell timestamp to reveal it immediately. React unmounts the hidden
`ChromeControlHint`, so the accepted hidden-token path must clear all three
hover-tooltip fields and clear the window's `presented_tooltip` record under
the same witness. The neutral follow-up law is: reveal A, accept hidden B,
then re-show it and prove no tooltip appears until a fresh 400ms hover; B
discard must preserve the still-visible A record. No test was run for this
audit.

### Current validation blocker

The parent reports that WASM attempt 8 stops in upstream XML/GLTF
`MutationLeaf` compilation after the SVG repair, before renderer assertions
run. This follows the earlier documented renderer ticket blocker in
`📓️astra-window-layout-resynchronization28.md:3`, which records the prior
XML `MutationLeaf` and SVG failures and their generated logs. The clock,
visibility, browser transport, and tooltip findings above are source reviews;
they are not a claim of a green WASM renderer validation.

### ACK-Time Deadline Refresh And Stale Completion Review

The former post-ACK deadline gap is closed in the current source. `Shell`
swaps accepted input geometry only after the interpreter has promoted the UI
router and consumed the matching hidden-visibility receipt
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14759-14792`; `🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1300-1338`).
`presented_retained_clock_deadline` then reads this accepted,
token-qualified roster (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14709-14710`), and
`Ui::surfaces_next_clock_deadline` ignores a changed or reused token
(`🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:3288-3297`). `AppPresentCursor`
captures that value after the successful shell ACK, merges the translated
chrome deadline, and carries it in `AppPresentStep::Complete`
(`📺️renderer/🎯️targets/🧊️wgpu/🦀️.rs:16603-16610,16646-16654`). The shared
winit completion path atomically publishes it and replaces the sole scheduler
key (`🪟️winit-app/🦀️.rs:292-297`), so an accepted hide immediately clears an
old retained deadline without waiting for another build.

The order before the generation comparison is intentional and correct. A
newer input C can advance `frame_generation` while B is still being presented;
B nevertheless becomes the actual painted and accepted interaction owner.
Publishing B's deadline first preserves the timer for the pixels on screen;
the subsequent `INPUT_STATE` invalidation starts C immediately. Moving the
publish below that comparison would instead erase B's live timer merely
because C is still candidate-only. `enqueue_host_event` and metrics both
invalidate the scheduler as they advance a free generation
(`🪟️winit-app/🦀️.rs:69-106`), and native `present_snapshot` refreshes the
same scheduler key from the mailbox on every present
(`:344-346`). No stale-completion ownership defect was found.

The browser uses the same precedence. Immediate `requestFrame` always clears
the one delayed timer before scheduling rAF
(`🚚️browser-frame-transport/🟦️.ts:584-595`); a worker response for an older
generation takes that immediate branch instead of replacing the current
timer (`:993-994`). A current idle response alone can install a delayed wake
(`:1002-1009`). The worker consumes a due scheduler key at the beginning of
its real offscreen redraw (`🪟️winit-app/🦀️.rs:181-185`), so the key does not
remain perpetually overdue. I did not run these paths.

The key regression to retain is B-then-C: accept B with deadline `d`, enqueue
C before B's `Complete`, then assert B's completion publishes `d` and has an
immediate input invalidation; discard C leaves `d`, while a later C ACK
replaces or clears it. Mirror the assertion in the browser virtual timer
fixture: C's input clears the B timer, the stale B response requests one rAF,
and only a current C response may replace the timer.

`EventRouter::suspend_clock` now also clears repeat, hover entry, revealed
tooltip, and dismiss deadline (`🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:1541-1547`),
and the accepted hidden-token path clears the presented tooltip record
(`⚙️engine/🦀️.rs:1327-1334`). This resolves the earlier tooltip resurrection
finding; the existing discard and token-reuse visibility laws remain the
right receipt coverage.

### Current Compile Receipt Status

`clock30/native-shell-2.log` retains only the three error headings, so it
cannot identify the current E0308/E0716 spans. Searches of the ticket's
generated receipts, Nx cache, Cargo temporary files, and local tool cache did
not recover a current detailed diagnostic. The preserved
`native158/shape.log:712` E0716 points to the older macOS temporal formatter,
but current `absolute_pattern` holds `OwnedCf` in a named local before
`cf_text` returns an owned `String`
(`🕰️native-temporal/🍎️macos/🦀️.rs:102-127`); it is not evidence of a current
failure. A fresh short-format Cargo/Nx run is required for those exact spans.
The current WASM-9 receipt records only the separate E0502 at
`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1933`, not an E0308 or E0716. No test
was run for this audit.

### Caret Cadence And Accepted Editable Ownership

The native host's `CaretBlink` is disconnected from the renderer's only live
blink implementation. `OsHost::present_snapshot` calls
`CaretBlink::sync(..., true)` for every presentation
(`🪟️winit-app/🦀️.rs:344-354`). `CaretBlink::visible` and `fire` are
test-only, and its unkeyed timer cannot cancel a previously appended wake
(`⏰️deadlines/🦀️.rs:66-104`). Its unit law manually drains the scheduler and
then manually calls the test-only `fire`
(`🧪️tests/🔬️wgpu-deadlines-unit/🦀️.rs:35-63`); no production caller does.
Thus an idle window pays one unnecessary 500 ms wake after presentation, but
that host object cannot toggle any production pixels or re-arm itself.

The renderer has a separate global blink path:
`FrameBuildPhase::Caret` toggles `AppInteractionState.caret_blink_visible`
from epoch `app_now_ms` on every build and calls
`node_graph_sync_caret_blink` (`📺️renderer/🦀️.rs:17021-17027`). That function
walks every `ENGINE_SURFACES` entry and affects only editing `NodeGraphEngine::Flow`
notes (`⚙️EngineCanvas/🦀️.rs:3373-3382`). It neither sees accepted
surface-token ownership nor covers TextEditor, Ink, or retained controls.

The retained focus model is more precise than `Ui::window_has_focus`:
`EventRouter` focus includes Button, Select, Toggle, Slider, NumberStepper,
Ring, and IconSelect as well as Input (`⚡️events/🦀️.rs:316-394`), whereas live
editable buffers are Input, IconSelect, and NumberStepper; Slider creates an
editor only after its value-cell gesture (`:501-507,1908-1945`). Current
production retained paint does display each live edit buffer but emits no
caret or selection primitive for Input, Slider, NumberStepper, or IconSelect
(`🖌️paint/🦀️.rs:1065-1094,1334-1419,1398-1419,1495-1505`). The elaborate
Input caret painter lower in that file is `#[cfg(test)]`
(`:2682-2733`) and cannot satisfy runtime parity.

React relies on the native browser input/textarea caret rather than an
application rAF; the test-only painter documentation correctly records that
there is no JSX caret geometry to copy (`🖌️paint/🦀️.rs:2691-2701`). The WGPU
contract therefore needs a renderer-owned, accepted-frame
`EditableCaretPresentation` rather than a generic focused-window boolean:
exact surface token, focused editable node/scene identity, visible bit, and
the bounded paint geometry necessary for that target. Candidate focus must
not arm or cancel it. The host should own one replaceable 500 ms deadline;
its due tick toggles the accepted presentation and the next accepted frame
uses it. Generic retained controls and EngineCanvas note editing are separate
paint consumers of that directive. TextEditor and Ink require their own
accepted scene identity before joining it; they cannot be inferred from a
window's focus.

The minimum laws are: no focused editable target schedules no caret wake;
accepted editable A schedules one; discarded B focus/blur preserves A;
acknowledged B changes or clears A exactly once; a reused surface id cannot
toggle a successor; a due tick toggles pixels and re-arms once. Add a
production retained-paint assertion for Input, NumberStepper, Slider readout,
and IconSelect before treating the timer as parity work. No test was run for
this audit.

### Tutorial Progress Has No Future Wake

React's actual `createTutorialClock` queues a new rAF from each playing tick,
updates time by elapsed wall time times rate, and pauses at duration
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6500-6557`). Its existing test explicitly
states that this self-ticking rAF is not exercised
(`🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:11790-11825`).

WGPU has the corresponding runtime state — `TutorialRuntime` tracks mode,
playhead, rate, `last_tick_wall_ms`, recorder state, and real-time camera
convergence (`🐚️Shell/🦀️.rs:23783-23827`) — and advances it only when a frame
build reaches `FrameBuildPhase::Tutorial`
(`📺️renderer/🦀️.rs:17102-17106`). `tutorial_tick` advances recording,
rate-scaled playback, auto-pauses at the exact end, and evaluates camera
convergence (`🐚️Shell/🦀️.rs:24379-24450`). Yet
`ChromeBuildState::next_deadline_ms` reports only tooltip and transient-notice
deadlines (`:22877-22894`), and `Shell::next_chrome_deadline` merely converts
that result to the monotonic clock (`:23045-23053`). The deadline module's
TutorialKeyframes region is empty. After the Play frame, the timeline,
recording clock, and camera tween depend on unrelated work to advance.

The narrow source boundary is
`ShellState::next_tutorial_deadline_ms(now_wall_ms) -> Option<f64>`, merged
with the existing Chrome deadline before its current monotonic adaptation.
It should return a 60 Hz next tick only for Playing, Recording, or a nonempty
convergence map, clamp playing to the exact end and convergence to
`TUTORIAL_CONVERGE_MS`, and return `None` for Paused, Deviated, or absent
runtime. The due build calls the existing `tutorial_tick`, which derives the
next deadline again. Existing native coverage is
`🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs`; the neutral tutorial bridge fixture
is `🧫️fixtures/🎥️tutorial-bridge/🔣️.json`. Extend them with play, rate,
record, convergence, auto-pause, and merge-with-earlier-tooltip cases.

This progress source shares the pending scalar-mailbox ownership issue below:
it mutates runtime during a build, while the final deadline transport is
acknowledgement-based. Its publish ownership must be resolved together with
retained UI clocks; it must not reintroduce an unconditional poll. No test was
run for this audit.

### Epoch `f32` GPU Time Freezes UI Motion

The prepared renderer writes `(app_now_ms() / 1000.0) as f32` into its GPU
input (`📺️renderer/🦀️.rs:17339-17341`). `app_now_ms` is `Date.now()` on WASM
and UNIX epoch milliseconds on native (`:10295-10302`). At current epoch
seconds, an `f32` has a 128-second unit in the last place. The value reaches
every prepared UI scalar (`🧊gpu/🦀️.rs:777,786,795`) through
`UiGlobals._pad[0]` (`🖍️draw/🦀️.rs:4066-4067`). The WGSL shader divides that
value directly into loading, waiting, and introduction pulse phases
(`🎨shaders/🦀️.rs:76-139`), so those animations remain fixed for roughly two
minutes and then jump.

This is unrelated to the retained `EventRouter` f64 monotonic clock. Use a
renderer-lifecycle-relative monotonic elapsed seconds value before the `f32`
GPU upload; preserve epoch wall time only for shell data that requires it. A
focused law must prove two sub-frame samples at a large epoch produce distinct
shader phase inputs, plus stable finite origin/restart behavior. Pair it with
the scheduler's existing animated-primitive rule only where a scene actually
advertises `has_animated_primitives`; do not make ordinary caret timing depend
on this uniform. No test was run for this audit.

### Partial Pre-Acknowledgement Deadline Publication Is Not Removable Alone

`RetainedClock` does not merely inspect a candidate. It advances the currently
presented router and tree, queues its commands, and advances the presented
interaction epoch when due (`⚙️engine/🦀️.rs:3242-3276`; interpreter wrapper
`🗣️Interpreter/🦀️.rs:953-964`). A host wake consumes the keyed scheduler
deadline before that build. If the ensuing candidate is discarded, retaining
the old scalar mailbox deadline is not a refresh: `present_snapshot` installs
that still-expired atomic value again (`🪟️winit-app/🦀️.rs:344-346`), while the
accepted router now owns a different next repeat/dismiss/reveal deadline.

Conversely, the old pre-ACK UI-only scalar can overwrite an earlier accepted
Chrome/tutorial deadline. A single replaceable minimum cannot safely be
updated by a partial source: it cannot distinguish raising the UI member of a
minimum from clearing another source. Removing the RetainedClock publication
is correct only if its presented-router mutation and command delivery move to
acknowledgement. Otherwise the bounded solution is separate mailbox deadline
owners for accepted retained UI and accepted Shell/Chrome/Tutorial, with the
host taking their minimum. The clock phase updates only its accepted UI owner;
ACK updates the Shell owner; a discard leaves both owners valid.

The discriminating regression is a due accepted stepper/tooltip clock that
advances during B, with an earlier Chrome deadline. Abort B, then prove the
new UI deadline still wakes and the Chrome deadline was not erased. Repeat
with a completed B and a fresh C to retain the existing stale-completion
ordering law. No test was run for this audit.

### Retained-Clock Publication Must Stay Ahead Of Candidate Acknowledgement

The current runtime already has the required separation: the retained-control
and Shell clocks occupy independent atomics and `control_deadline_us` takes
their minimum (`📺️renderer/🦀️.rs:12016-12017,13545-13561`). This fixes the
earlier cross-owner erasure. It does **not** make retained-control publication
safe to defer. `FrameBuildPhase::RetainedClock` advances each exact accepted
surface and drains its commands, accumulating the UI-only next deadline before
publishing that one owner (`📺️renderer/🦀️.rs:17042-17053`). The host's
`FrameScheduler::should_render` removes a keyed deadline as soon as it is due
(`🖌️render/⏱️schedule/🦀️.rs:147-155`).

Consequently, after the due build advances an accepted stepper repeat or
tooltip, removing that publication leaves the runtime atomic at the consumed,
past deadline. `present_snapshot` immediately reinstalls that old value into
the keyed scheduler on every admitted redraw (`🪟️winit-app/🦀️.rs:345-347`).
If the candidate is discarded, this produces immediate retry work until some
unrelated frame eventually acknowledges; it neither has the new accepted UI
deadline nor returns to idle. FrameReady/progress can help a live candidate
finish, but neither proves a fresh event after an abort. This violates the
same no-busy-wake contract the browser delayed timer was added to protect.

Keep the RetainedClock cursor's per-surface minimum and publish it to the
*retained UI atom* after the accepted-router advance. It is not an input
candidate value. Keep Shell/Chrome/Tutorial publication on acknowledgement,
because it comes from candidate-owned Shell rendering. The existing native
law `partial_control_deadline_publication_cannot_erase_another_owner_when_a_candidate_is_discarded`
(`📺️renderer/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs:5-22`) covers separation,
but needs one behavioural row: consume a due UI deadline, advance it to a
future value, discard the current candidate, then assert the scheduler has
that future UI deadline and the older Shell deadline remains its minimum. A
second assertion must show no immediate redraw before either deadline. No test
was run for this audit.

### Tutorial Ticks Have The Same Abort-Rearm Requirement

The new tutorial deadline query correctly derives cadence, exact playback end,
and convergence end from the live runtime (`🐚️Shell/🦀️.rs:24203-24225`) and
merges it with chrome before wall-to-monotonic adaptation
(`:23047-23060`). But the actual tick still runs in `FrameBuildPhase::Tutorial`
*before* Chrome seals its input candidate (`📺️renderer/🦀️.rs:17115-17118,
17128-17147`). It advances `playhead_ms`, recorder state, pending document
operations, convergence, and `last_tick_wall_ms` directly in `ShellState`
(`🐚️Shell/🦀️.rs:24418-24490`). The later candidate abort ladder only discards
its `PresentedInputCandidateWitness`; it does not roll back the tutorial
runtime (`📺️renderer/🦀️.rs:16247-16268`).

Therefore the future tutorial deadline belongs to the same side of the
candidate boundary as the tick. ACK-only Shell publication after a discarded
due tutorial tick repeats the consumed-past-deadline busy loop described
above. The narrow fix is to publish the **full current Shell minimum** (Chrome
plus Tutorial) immediately after this live tick, using the existing
`next_chrome_deadline(monotonic_now, wall_now)` query; it preserves an earlier
accepted chrome deadline while rearming the new tutorial one. The later ACK
may replace it with the full post-accepted candidate value. Moving the tick to
ACK is also coherent, but requires a distinct next-frame paint/intent contract
and should not be combined with this clock slice.

Add an abort law beside the renderer frame-job deadline laws: a playing
tutorial's due B tick advances its playhead and calculates a future cadence;
aborting B leaves that future Shell deadline armed, advances neither a second
time before it is due, and retains any earlier chrome deadline. Cover playback
end (the tick pauses and clears the tutorial member), recording, and
convergence. The current Shell tutorial tests contain no direct
`next_tutorial_deadline_ms` coverage. No test was run for this audit.

### Accepted Animated Primitives Need A Persistent Packet Receipt

The just-landed phase fix is correct. `ui_animation_seconds` reduces the
monotonic microsecond clock modulo 3.2 seconds before the `f32` upload
(`📺️renderer/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs:28-30`), which is the common
period of the shader's 1.6-second loading/introducing and 3.2-second waiting
branches (`🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:73-139`). The new neutral
fixture, native law, and Chromium Web Animations oracle cover phase precision.
They do not cause a new frame to be built, so a settled loading border still
freezes after the one accepted render.

There is a design oracle for the missing predicate, but it is not connected to
production WGPU. The generic renderer detects kinds 6, 7, and 9 once while it
finishes a packet and saves `RenderPacket::has_animated_primitives`
(`🖌️render/🎬️scene/🦀️.rs:742-754`). Its test-only frame engine then schedules
one `ANIMATION` deadline at 60 Hz (`🖌️render/🖼️frame/🦀️.rs:142-147,208-212`).
The actual WGPU `DrawList` emits the same three kinds only through
`push_loading_border`, `push_waiting_border`, and `push_introducing_border`
(`🖍️draw/🏷️types/🦀️.rs:1016-1044`), including overlay-routed draws. Its
prepared input and packet contain draw/overlay/time but no animation predicate
(`🎟️prepared/🦀️.rs:1592-1611,1869-1883`), and `AppPresentStep::Complete`
likewise carries only generation, directives, and clock deadlines
(`📺️renderer/🧊️renderer/🦀️.rs:15767-15772`). Thus no accepted WGPU packet
currently exposes “animation needed”. A scan at timer selection would be both
unbounded and too late.

The small production boundary is:

1. Add `has_animated_primitives: bool` to `DrawList`, false in both `Default`
   and `empty` (`🖍️draw/🏷️types/🦀️.rs:211-236,410-429`). Set it only *after*
   the retained-output claim succeeds in the three animated push helpers. Give
   `DrawList` an O(1) getter. This covers both normal and overlay buckets
   without walking layers. Finished kind 8 remains false. The predicate names
   rendered primitive kind, so `Celebrating` is included when it emits kind 9;
   it must not be inferred from a higher-level UI state.
2. At `PreparedRenderJob` publication, capture
   `draw.has_animated_primitives() || overlay.is_some_and(...)` in
   `PreparedRenderPacket`; expose a getter. That capture belongs beside the
   retained owners' transfer (`🎟️prepared/🦀️.rs:3178-3210`), so an admission
   refusal or abandoned input never publishes a predicate.
3. Read that immutable packet predicate during the existing present
   acknowledgement, store it in `AppPresentCursor`, and carry it in
   `AppPresentStep::Complete`. The packet is still available through
   `gate.pending_presented` in the Acknowledge phase
   (`📺️renderer/🧊️renderer/🦀️.rs:16585-16672`); it is no longer reachable by
   the later Directives phase. The value therefore means GPU-presented,
   accepted output, not “a candidate happened to paint an animated node”.
4. Add one `OsHost` boolean `accepted_animation_active` and one keyed deadline,
   distinct from `RETAINED_CONTROL_CLOCK`. On `Complete`, set the boolean and
   replace that keyed deadline with `now + 1.0 / 60.0` or `None`. Do this
   **before** the existing generation comparison in
   `build_and_publish_snapshot` (`🪟️winit-app/🦀️.rs:303-333`): an acknowledged
   packet is already visible even when its generation no longer matches the
   latest input, and an animated one must keep advancing. In
   `present_snapshot`, reapply the same keyed deadline from the persistent
   accepted boolean (`:345-367`). `FrameScheduler::should_render` consumes a
   due key (`🖌️render/⏱️schedule/🦀️.rs:137-155`); this per-redraw rearm keeps
   accepted A alive while candidate B is incomplete or discarded. An accepted
   static B clears the flag and removes only this key.

This stays within the current cross-platform scheduler transport. Native wakes
via `NativeHost::about_to_wait` and `WaitUntil`; the browser worker consumes a
due key in `redraw_offscreen_worker` before the same redraw path
(`🪟️winit-app/🦀️.rs:182-185,188-224`), and its existing one-timer transport
already sends `nextDeadlineDelayMs` and replaces the timeout
(`🌐️browser-worker/🦀️.rs:355-397`; `🚚️browser-frame-transport/🟦️.ts:993-1008`).
No requestAnimationFrame loop or external runtime is required.

The discriminating laws are: accepted animated A arms exactly one 60 Hz key;
its due wake is consumed then rearmed even if B is still pending; animated B
that is discarded cannot arm or clear A; accepted static B removes the key;
and an overlay-only kind 9 arms it. Reuse the production deadline/discard law
`partial_control_deadline_publication_cannot_erase_another_owner_when_a_candidate_is_discarded`
(`📺️renderer/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs:5-35`) for the accepted-A /
discarded-B ordering, plus the existing browser timeout transport suite
(`📺️renderer/🧪️tests/📨️browser-frame-transport/🟦️.ts`). Keep the already
landed phase schema/oracles as the independent time-value proof. No test was
run for this audit.


### Amendment: Measured Packet Receipt Is The Correct Animation Boundary

The preceding proposed `DrawList` push-time flag is superseded by the current
implementation. A `DrawList` is mutable public staging data; its buckets can
be moved or copied before a packet is sealed. The correct immutable source is
now the bounded `PreparedRenderJob` traversal. At every existing
`DrawMeasureCursor::LayerUi`, the job reads that exact normal or overlay
instance before advancing its cursor and ORs kinds
`KIND_LOADING_BORDER`, `KIND_WAITING_BORDER`, and
`KIND_INTRODUCING_BORDER` into its receipt
(`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3010-3027,3107-3112`). The result
is copied into `PreparedRenderPacket` at publication
(`:3209-3224`). It introduces no second traversal or counter and covers the
separate overlay draw list.

The neutral animation fixture now drives that actual bounded preparation path
for static, all three normal kinds, overlay-only introducing, and static
finished overlay, then asserts the packet receipt
(`🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:5-24`). This is the
required source-of-truth law; it verifies the classification survives packet
handoff rather than merely exercising a helper.

The acknowledgement handoff is also placed correctly. `AppPresentPhase::Acknowledge`
reads the immutable pending packet before the gate replacement and stores the
copy in its cursor (`📺️renderer/🧊️renderer/🦀️.rs:16658-16666`); the
Directives phase then carries it in `AppPresentStep::Complete`
(`:16698-16711`). `OsHost` accepts and schedules that receipt before its
existing generation mismatch return (`🪟️winit-app/🦀️.rs:292-299`). Therefore
a discarded candidate cannot alter the previous accepted animation demand,
while a GPU-acknowledged but superseded frame still owns its visible motion.
`present_snapshot` re-synchronizes the host-local accepted clock after every
admitted redraw (`:347-350`), retaining the current future deadline through
unrelated sub-frame work instead of continually pushing it out.

No visible-clip predicate should be added. The renderer-neutral oracle marks
all batched animated quads before clipping (`🖌️render/🎬️scene/🦀️.rs:742-754`).
Matching it by packet presence avoids an additional visibility traversal and
cannot suppress a partially clipped animated border. The only shader consumers
of packet `time_seconds` are still the loading, waiting, and introducing UI
branches at `🎨shaders/🦀️.rs:82,104,136`, with periods 1.6, 3.2, and 1.6
seconds; the 3.2-second modulo remains sufficient. CPU celebration colour
uses the separate retained `DrawList::clock_seconds` source and is outside
this packet-time receipt.

One narrow correction remains: `AcceptedAnimationClock::sync` currently
registers the keyed wake as `InvalidationReason::PAINT`
(`📺️renderer/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs:30`). The neutral renderer
contract registers the same accepted-primitive demand as
`InvalidationReason::ANIMATION` (`🖌️render/🖼️frame/🦀️.rs:208-212`) and its
scheduler laws identify it that way. The wake behavior is otherwise the same,
but use `ANIMATION` and assert the reason in the accepted-clock fixture to
preserve the shared scheduling taxonomy. No tests were run for this audit.


### Current Status: Animation Receipt Is Coherent (2026-09-27)

The preceding `PAINT` reason concern is resolved in current source: `AcceptedAnimationClock::sync` installs the keyed deadline with `InvalidationReason::ANIMATION`, and the deadline law asserts that exact reason at `🧑‍🎨engine/🧪️tests/🔬️wgpu-deadlines-unit/🦀️.rs:15-20`.

The production receipt has the correct ownership boundary. `PreparedRenderJob::measure_next` reads one `DrawMeasureCursor::LayerUi` item before advancing either the primary or overlay cursor and latches only kinds 6, 7, and 9 (`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3011-3027,3107-3112`). The boolean transfers to both the normal completion packet and the bounded abandonment packet (`:1848-1854,3211-3224`). It is not inferred from a mutable `DrawList` after submission.

`AppPresentCursor` copies the immutable packet result only after `Shell::acknowledge_presented_input` succeeds (`📺️renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16658-16664`) and carries it in `AppPresentStep::Complete` (`:16698-16711`). The host calls `AcceptedAnimationClock::accept` and keyed `sync` before the generation comparison (`🪟️winit-app/🦀️.rs:292-301`), so an already accepted visual frame remains animated if a newer input generation exists. A later accepted static frame removes only key 2; candidate discard has no API which can alter that accepted owner. `present_snapshot` reapplies the retained due only when it was consumed, avoiding extension from unrelated redraws (`:345-350`; `⏰️deadlines/🦀️.rs:16-31`). No source-level ownership or cancellation defect was found in this path.

The packet law uses the actual resumable job, all six primitive rows, direct `UiInstance.params[2]` mutation, both normal and overlay buckets, and packet retirement (`🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:5-24`). This audit did not run it. The retained scalar needs no special retirement phase: packet retirement does not create a second semantic owner.

### Current 15-Surface Acceptance Gate (2026-09-27)

All fifteen contract tags still have a production React host, a typed WGPU route, and a non-placeholder WGPU paint path. That establishes implementation reach, not end-to-end renderer parity. The current generic parity runner starts both renderers and performs a structural/pixel comparison, but it deliberately treats every `componentScene` as a raster leaf: it checks placement while omitting its text, colour, and internal semantics (`🧑‍💻dev/⚖️parity/🏗️structure/🟦️.ts:161-185`). Its only default behavioral suite is one generic `stateTransition`; the catalog explicitly has no scene interaction suite (`🧑‍💻dev/⚖️parity/🔬️probe/🟦️.ts:347-360`). Thus a passing generic parity run cannot accept an authored surface interaction.

The current production dispatch classification is useful for an acceptance matrix:

| Surface group | Current production route | First app-backed acceptance evidence still required |
| --- | --- | --- |
| Canvas2d, World3d, InkCanvas, IconRender | Direct retained scene paint; special render steps at `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3341-3344` | Draw gesture/cancel; 3-D orbit/pick/menu; ink edit/clipboard/cancel; icon asset fallback or raster identity. |
| NodeGraph, TiledMap, Board2d, Paint2d, TextEditor | Engine texture; bespoke pointer dispatch covers the graph/map/board subset at `:3723-3727`, with host event routing in `📺️renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:17646-17846` | One accepted direct manipulation and a camera or selection consequence per supplied app surface. TextEditor remains separately in flight and is excluded from a new gate. |
| Table, VFS, GraphTimeline, BlockList, DiffView, EventFeed | Retained scene paint at `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3524-3529`; list pointer routing begins at `:3028-3041` | A row/checkpoint/block/conflict/feed action with an application receipt, then an accepted-frame visible or accessible consequence. Table, VFS, GraphTimeline, BlockList, DiffView, and EventFeed have recent focused packets; do not reopen them solely because the generic runner cannot witness them. |

The next full-goal gate is therefore a schema-first `surface-behavior@1` fixture family, exercised in a mounted application rather than Storybook or a native-only painter law. Each row should bind: `(surface kind, app/document seed, exact input sequence, expected action receipt, expected visible or accepted accessibility consequence)`. Reuse the existing application specimens rather than creating product-only test scenes: Draw for Canvas2d, Flow/DAG for NodeGraph, Layout for TiledMap or Board2d, Note for InkCanvas, the existing Playbook scenes for Table/DiffView/EventFeed/TextEditor/BlockList, and an app that already emits VFS/GraphTimeline/IconRender.

Order the browser gate by the remaining broadest interaction families: (1) Canvas2d and InkCanvas direct input/cancellation, (2) NodeGraph plus one map/board manipulation, (3) retained list selection/scroll/activation rows. This is verification work, not evidence of a newly missing renderer path. No live journey or native suite was run in this audit.


## App-Backed Canvas2d and Ink Cancellation Acceptance Packet (2026-09-27)

### Current Runner and Activation Boundary

The paired browser executor is a real boot but not an app-behavior runner. 🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:146-205 prebuilds one plugin, starts React and WGPU, compares structure and pixels, then invokes only the state-transition probe. 🧑‍💻dev/⚖️parity/🔬️probe/🟦️.ts:348-360 defines that probe as one state transition. The catalog exports only state and shell. It has no fixture-selected document, scene-local pointer cancellation, action receipt, or accepted visible result.

Existing runnable paths are:

- bun nx run @semio-tech/framework-renderer-react:test runs the React renderer Vitest suite through the React package script at lines 31-35. No focused Canvas or Ink Nx target exists.
- bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit runs mounted Rust library laws. Its Nx target forwards Cargo filters and its router selects the library test target. See the WGPU project file lines 185-191 and script lines 102-113 and 525-527.
- .vscode/launch.json:3914-3958 supplies Draw React, WASM and native entries. Lines 3969-4013 supply Note entries. Browser entries run workspace:dev -- draw or note with app identities s.draw.drawing@1/*#editor and s.note.note@1/*#editor; native entries run @semio-tech/framework-renderer-wgpu:native -- draw or note.

The test browser-host activation artifacts cannot seed these apps. 🧑‍💻dev/♻️activation/🌐️browser-host/🟦️.ts:343-364 closes a fixed test host for gis and space, not Draw or Note. There is no current automated physical pointer-cancel artifact that seeds either app and exports its action receipt.

### Existing Coverage Versus Required Acceptance

| Surface | Existing proof | Missing acceptance |
| --- | --- | --- |
| Draw Canvas2d | Canvas2dHost input-contract and gesture-sample-lane tests prove host action shape. The generic scene-pointer-cancellation fixture proves source-owner plumbing. | No booted Draw app, action admission, artifact result, or WGPU comparison. |
| WGPU Canvas2d | Scenes WGPU lines 2157-2217 publishes a cancelled terminal pointer action and clears retained gesture state. | No admitted Draw action and accepted scene result. |
| Draw application | The canvas-pointer-down native app law at lines 227-250 proves a cancelled direct drag has no artifact mutation and clears preview. | It bypasses both renderers and declared application boot. |
| Ink host and WGPU | React emits begin, live and commit from InkCanvasHost lines 1087-1124; cancellation clears local draft and gesture state at lines 1360-1371. WGPU builds identical wire actions in Scenes lines 7614-7645 and 8536-8690 and clears transient retained state at 8020-8026. | No Note activation or receipt/result assertion. |

### Exact Production Policy

Draw cancellation has one required terminal receipt. React and WGPU emit canvasPointerUp with cancelled true. WGPU does so at Scenes lines 2157-2217. Draw handles that flag with cancel_gesture and emits no document mutation in its canvas-pointer-up command lines 12-40. A test must require the terminal receipt, unchanged document, absent preview, inert duplicate cancellation, and a succeeding fresh gesture.

Ink has a distinct accepted policy. React emits inkApplyEvents immediately for pencil begin and live. Cancellation emits no rollback or terminal action but clears the active preview. Note applies begin and live immediately; its native app law proves that a begin already adds the block. WGPU must preserve this outcome: cancellation terminates the active retained Ink gesture and clears preview, while accepted begin/live receipts remain. Post-cancel move, up, stale-owner completion and duplicate cancellation must publish no more actions.

The generic scene-pointer-cancellation fixture says Ink discard/no-publication. It is a source/schema law, so update it to state per-surface policy rather than use it as an application outcome oracle.

### Required Surface-Behavior Version One Fixture

Add a schema-first paired case to the existing parity executor with plugin and exact app identity, deterministic artifact/example seed, stable scene locator, normalized local input points, expected ordered action receipt fields, accepted artifact digest and visible projection digest.

Use shipped seeds. Draw exposes demo at its manifest line 6600 and later. Note exposes demo at manifest lines 6108-6124; its interactive composite canvas is InkCanvas in the composite mode file lines 9-12 and 86-95.

1. Draw: demo layer drag, pointer down, move beyond threshold, physical cancel. Require one terminal receipt action canvasPointerUp with cancelled true, pre/post artifact equality, no preview, duplicate cancel inert, then fresh down/up produces expected mutation and accepted visible update.
2. Draw stale/retire: start gesture, supersede the accepted surface or retire it, and prove cancellation belongs to exact surface ID, generation and pointer only. It cannot mutate successor; a fresh successor gesture is admitted.
3. Note: select pencil in interactive composite scene, down, move, physical cancel. Require accepted begin/live receipts emitted before cancellation, no commit or later move/up receipt after cancellation, no retained preview or marquee, and persisted Note digest induced by accepted begin/live. Then a fresh stroke is admitted and commits normally.
4. Note stale/retire: cancel after exact owner replacement. Old begin/live remain historical accepted receipts; no stale subsequent live or commit action can be admitted to successor.

Native laws must pass through document ingress, accepted-frame pointer routing, app action dispatch, and receipt/result projection. Direct calls to scene cancellation helpers do not meet this acceptance. The actual React oracle must mount real Draw/Note controller path, not only a component host. Only after this packet exists can paired browser parity add surface-behavior draw and note.

The launch entries make manual React, WASM and native confirmation possible. They do not make automated physical cancellation complete, and no existing command may be reported as satisfying this app-backed packet.

