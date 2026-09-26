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
