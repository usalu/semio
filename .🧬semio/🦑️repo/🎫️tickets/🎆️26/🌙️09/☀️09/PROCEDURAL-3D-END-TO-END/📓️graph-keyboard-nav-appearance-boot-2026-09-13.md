# Graph Keyboard Navigation and the Appearance Boot Key — generation3d (2026-09-13)

Lane: `graph-keyboard-nav-appearance-boot`. Closes the two things
`📓️react-i18n-a11y-customization-2026-09-13.md` explicitly did **not** do: §4.1 (arrow-key node-by-node
traversal inside the Flow canvas is not implemented) and §3.2 (the pre-paint boot scripts read a dead
storage key, so a dark session flashes light on every reload).

All runtime evidence is from **6018** (`http://127.0.0.1:6018/?plugin=generation3d`), the coordinator's
React dev serve, after a successful `activate-generation3d-react-dev` restage (16m 45s, exit 0,
11 components, "changed"). No git-modifying command was run; `📓️status.md` and `🎫️ticket.json` were
not touched.

---

## 0. TL;DR

| # | Item | Before this lane | Now |
|---|---|---|---|
| 1 | **Keyboard traversal of the node graph** | Not implemented. The canvas was focusable and announced `role="application"` — which *promises* it handles its own arrow keys — and then handled none. The honest keyboard path was the sibling outline tree | Five new guest verbs, schema-first, en+de, arg-free, **window-owned by the Flow window**, bound to the four bare arrows plus the canvas's own `Trigger::Activate`. Traversal is graph topology: wires for upstream/downstream, layout reading order for next/previous. Selection moves through the SAME framework selection lane a pointer pick uses |
| 2 | **The law** | — | One language-neutral fixture over the bundled hex-column graph, answered twice: Rust (the shipped reducer against the shipped example) and a TypeScript twin that re-implements the traversal from the fixture alone. **8 Rust laws + 47 TS checks**, both proven to bite by a negative control |
| 3 | **Appearance boot key** | `PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT` read `localStorage["ui.chrome.appearance"]`, a key the OS shell has not written since it moved to the event-sourced `semio.os.config` document. Measured consequence: a dark session boots **light** | Both boot scripts replay the `semio.os.config` UI-preference event log. Fixture + Rust law + a TS law that **executes the shipped script string** in a DOM-less sandbox. Runtime A/B on 6018: `rgb(247,243,227)` → `rgb(0,17,23)` |

**The headline finding behind item 1:** the app could not express "move the selection" at all until
this lane went looking — but the framework already could. `Emit.interaction_writes` (the lane
`26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM` built, and which `🧩️puzzle`'s
`selectSameKindSelection` already uses) applies an app-initiated selection through
`protocol::next_selection`, the very same single-writer machine the reserved `interactionSelect` verb
uses. So an arrow key and a click are literally the same gesture to everything downstream — which is
why the Inspection panel follows an arrow key in §4 without one line of panel code changing.

**Honest scope limit:** §6 lists what is NOT claimed — most importantly that in-canvas **camera**
focus is still impossible from the guest, why, and what it would take.

---

## 1. Keyboard navigation — design

### 1.1 The traversal, and where it lives

Traversal is a pure derivation from the flow document, so it lives with the document, in the
**framework** flow artifact, not in generation3d — every flow-backed app (flow, generation2d,
generation3d, workflow) gets it:

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:1120-1199`

- `FlowGraphStep` — `Next` / `Previous` / `Upstream` / `Downstream`, the whole vocabulary.
- `keyboard_order(&FlowFixture) -> Vec<&str>` — **reading order**: ascending `layout.x`, then ascending
  `layout.y`, then the document's own widget order. A node the layout map never names sits at the
  origin, *exactly as `FlowFixture::to_artifact` places it* — the two must agree or the keyboard would
  visit a node where the canvas does not paint it.
- `keyboard_step(&FlowFixture, anchor, step) -> Option<&str>` — `Next`/`Previous` walk the reading order
  and **wrap** (so one key reaches every node of a disconnected graph); `Upstream`/`Downstream` follow a
  **wire** and deliberately **do not** wrap (a source node has nothing upstream, and pretending
  otherwise would teleport the selection across the canvas). With no anchor at all a forward step
  enters at the first node and a backward step at the last, so the very first keypress always selects
  something.
- A wire step with more than one candidate takes the first in **reading order**, never in synapse
  order. This is not cosmetic: on the hex column, `upstream(profile)` is `{radius, sides}` in wire
  order but `sides` in reading order, and `upstream(extrude)` is `{profile, extrusion-axis}` in wire
  order but `extrusion-axis` in reading order. Both are fixture rows, and the runtime probe hit the
  first one (§4).

### 1.2 The five verbs

`✏️editor/🎮️commands/🧭️navigate-graph/🦀️.rs` — ONE leaf, five verbs, because they are one traversal
seen from five directions: the same anchor reduction, the same `keyboard_step`, the same selection
write, differing only in which `FlowGraphStep` they name. A `graph_keyboard_command!` macro emits the
four step modules so the body exists once.

| verb | en / de | step |
|---|---|---|
| `selectNextNode` | Select Next Node / Nächsten Knoten auswählen | next in reading order, wraps |
| `selectPreviousNode` | Select Previous Node / Vorherigen Knoten auswählen | previous in reading order, wraps |
| `selectUpstreamNode` | Select Upstream Node / Vorgelagerten Knoten auswählen | follows a wire IN |
| `selectDownstreamNode` | Select Downstream Node / Nachgelagerten Knoten auswählen | follows a wire OUT |
| `activateSelection` | Open Node Ports / Knotenanschlüsse öffnen | replaces the node selection with that node's port handles |

`anchor_node` (`:38`) is the one rule that makes the family composable, and it states three things the
fixture also states: a **port handle** (`{node}@{port}`) steps from the node that owns it, so arrowing
after opening a node closes it again; a **multi-selection** steps from its first member in reading
order, never from whichever id the set happened to hold first; a **stale id** is no anchor at all, so
the step re-enters the graph rather than refusing.

### 1.3 Selection moves through the framework's lane, not an app cursor

`step_emit` / `activate_emit` return an `Emit` carrying exactly one `InteractionWrite` and **nothing
else** — no artifact mutation, no config mutation. The publication contract says so out loud:

`✏️editor/🦀️.rs:781-789` — five `ArtifactToolPublicationContract` rows with
`lanes: &[ArtifactToolPublicationLane::Interaction]`. That lane is `HistoryLane::Interaction`:
persisted-local and **excluded from undo by declaration**, which is the correct semantics for moving a
highlight. `dispatch_emit` faults a tool that carries interaction writes without declaring the lane
(`🔌️plugin/🦀️.rs:25298`), so the declaration is load-bearing, not decoration.

Because the write goes through `apply_interaction_writes` → `protocol::next_selection`
(`🔌️plugin/🦀️.rs:23508`), it obeys every declared `SelectionSpec` rule — mode clamping, pruning,
topology validation — and everything that reads the `graph` domain follows for free: the outline
tree's highlight, the Inspection panel, the preview gumball. A step that has nowhere to go emits
**nothing** rather than an idempotent `Replace` that would cost a publication to change nothing.

### 1.4 The chords, and why they are bare arrows

`✏️editor/🦀️.rs:2456-2470`:

```
.keybinding("arrowdown",  "selectNextNode")
.keybinding("arrowup",    "selectPreviousNode")
.keybinding("arrowleft",  "selectUpstreamNode")
.keybinding("arrowright", "selectDownstreamNode")
```

Bare, with no modifier, because `role="application"` — which `accessibility_role` already implies for
every `Component::Surface` and which the React shell now paints on the canvas — *is* the ARIA contract
for "this widget handles its own arrow keys". A modifier would break the promise the canvas makes to a
screen reader.

They are safe as bare chords for two reasons, both read off `ShellHost` rather than assumed:

1. **Window ownership is the scope.** All five verbs are listed in
   `window_kind_action_refs(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, …)` (`✏️editor/🦀️.rs:2404-2422`),
   and `handleAppKeydown` builds its action lookup from the **focused window kind's** actions alone
   (`🏛️ShellHost/🟦️.tsx:8353`, `if (!definition) continue;`). A chord for a window-owned verb is live
   exactly while that window has focus — the rule `📓️editor-verbs-cancel-undo-2026-09-13.md` §6.1
   measured for `cancelPreviewEval`, used here on purpose.
2. **The handler yields.** It returns early on `event.defaultPrevented` and on any editable target
   (`🏛️ShellHost/🟦️.tsx:8339,8352`), and `useShellKeydown` listens on `document` in the bubble phase
   (`🐚️ShellScope/🟦️.tsx:222`), so a tree row or a text field that handles its own arrows keeps them.

The key token is the DOM `event.key` lowercased (`arrowdown`), never a shorthand:
`keyboardEventMatchesChord` compares the last `+` segment against `event.key` verbatim
(`🛠️ShellHelpers/🟦️.tsx:3732`) — the same rule that made `mod+period` a silently dead chord.

`activateSelection` is deliberately **not** an app chord. It is the canvas's own
`Trigger::Activate` binding (`🕸️flow/🦀️.rs:296`, via the new
`activatable_scene_surface`): `SurfaceAccessibilityShell` fires an activate trigger only for the
canvas that actually has focus and calls `preventDefault` (`🗣️Interpreter/🟦️.tsx:1533-1541`), whereas an
`enter` chord in the keybinding table would also fire on every focused button in the shell. The
binding carries `AccessibilitySpec.shortcut = "Enter"`, which reaches a reader as
`aria-keyshortcuts` — the only way a user who cannot see the canvas learns the chord exists.

`escape` → `clearSelection` and `delete,backspace` → `deleteSelection` already existed (the former
minted by `build_definition`'s interaction domain, the latter declared by the app); this lane did not
add them, it **proved** them (§4).

---

## 2. Files (keyboard navigation)

**Framework**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:1120` —
  `FlowGraphStep`, `keyboard_order`, `keyboard_step`, `FlowFixture::keyboard_*`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6530` — the **test-only** fixture-tree projection
  (`artifact_app_laws::project_and_retire_fixture_tree`) now carries each node's `accessibility`
  alongside its `bindings`, for the same reason the bindings comment already gives: a surface paints
  into a canvas with no accessible children, so the record's own name/description/liveness/shortcut is
  all an assistive technology ever learns about it, and a projection that omitted them made every
  "this canvas is named / advertises its chord" assertion read against a tree carrying no
  accessibility at all. Production wire codecs are untouched.

**generation3d**
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:81` — `activatable_scene_surface`
- `…/🧊️generation3d/🦀️.rs:733` — `navigate_graph` module mount
- `…/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🦀️.rs` — **new**, the five verbs
- `…/🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs` — **new**, 8 Rust laws
- `…/🧭️navigate-graph/🧪️tests/🔬️unit/🟦️.ts` — **new**, the TypeScript twin
- `…/✳️any/🧫️fixtures/🧭️graph-keyboard-navigation.json` — **new**, the statement
- `…/✳️any/🧫️fixtures/⌨️keyboard-reachability.json` — 4 new `bindings` rows + a new `surfaceBindings`
  section for the canvas's activate trigger
- `…/✳️any/✏️editor/🦀️.rs` — command enum rows, `GENERATION3D_RETAINED_TOOL_IDS`,
  `PUBLICATION_CONTRACTS`, `bounded_first_step_tool_proofs!`, `command_from_action`,
  `generation3d_retained_reduce`, `Generation3dPlayApp::handle`, five `ActionDefinition`s +
  `action_interactive_job`s, `window_kind_action_refs`, four `keybinding`s
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:290` — the canvas's activate binding
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs` — the activate-binding law
- `…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `every_command()`, wire-keyword roster, route counts
  (25→30 retained tools, 25→30 publication contracts, 34→39 bounded proofs)
- `…/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts` — registers the new twin
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `🧭️navigate-graph` member name

---

## 3. The law (keyboard navigation)

Test-driven: the fixture was written first, then both halves.

**Fixture — `🧫️fixtures/🧭️graph-keyboard-navigation.json`.** A language-neutral restatement of the
bundled `hexagonal-mushroom-column` graph (7 nodes with their layout coordinates, 6 wires), the
expected `keyboardOrder`, five `anchors` rows, and six `walks` of arrow sequences with the selection
each step must leave behind (a step with nowhere to go is written as the repeated row).

**Rust half — `🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs`, 8 laws, all green:**

| law | what it pins |
|---|---|
| `the_shared_fixture_restates_the_bundled_hex_column_graph` | the fixture's graph **equals** the example's widgets/layout/synapses — the hinge that stops both halves agreeing about an invented graph |
| `the_keyboard_order_is_the_reading_order_the_fixture_declares` | `keyboard_order` |
| `every_anchor_rule_the_fixture_states_holds` | port → owner, multi-selection → first in reading order, stale id → none |
| `every_arrow_walk_lands_where_the_fixture_says` | **21 steps** across six walks |
| `a_keyboard_step_authors_no_document_and_no_config_operation` | the Interaction-only emit, plus activate's open/already-open/no-port cases |
| `every_step_the_fixture_names_is_a_declared_arg_free_verb` | each verb is declared on a window and takes no required argument |
| `an_arrow_verb_moves_the_framework_owned_graph_selection` | **end to end**: `select_graph(height)` → `SelectDownstreamNode` → `interaction_state().selection["graph"] == ["extrusion-axis"]` → `SelectUpstreamNode` → `["height"]` |
| `activate_opens_the_selected_nodes_ports_and_an_arrow_closes_them` | **end to end**: activate on `extrude` selects `["extrude@wire","extrude@vector","extrude@solid"]`; the next arrow steps from `extrude` and lands on `column-preview` |

Plus, in the flow window's own suite,
`the_node_graph_canvas_declares_the_activate_binding_the_keyboard_fixture_states`, and in the editor's,
the pre-existing `the_editor_binds_every_keyboard_verb_the_fixture_names` (which now covers the four
arrow rows for free, because it asserts the fixture is the WHOLE app keyboard surface).

**TypeScript twin — `🧭️navigate-graph/🧪️tests/🔬️unit/🟦️.ts`, 47 checks.** A second implementation:
it re-derives reading order, anchor reduction and wire following from the fixture alone. It also
carries a property the fixed-length walks cannot state — *repeating one key reaches every node* — in
both directions. Registered on the artifact package's `twins` hook, so
`@semio-tech/…:test` prints `[DEBUG] generation3d-graph-keyboard-twin checks=47`.

**Counts**

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- \
  navigate_graph flow::tests:: the_editor_binds_every_keyboard retained_route_dispositions every_printed_op_line
→ 19 passed; 0 failed
generation3d-graph-keyboard-twin checks=47
```

**Negative control (both halves bite).** Changing one fixture row —
`a fork upstream takes the first candidate in reading order` from `sides` to `radius` — turned the TS
twin red (`AssertionError: … after upstream`) and the Rust law red
(`left: ["sides"] right: ["radius"]`). Fixture restored; both green again (47 checks, 19 tests).

---

## 4. Runtime proof on 6018 (keyboard only)

`🐍️graph-keyboard-nav-probe.mjs`, output in `🗑️generated/graph-keyboard/keys-4/`. The probe uses the
pointer exactly once — one click on the canvas, on no verb, to make the Flow window active and focus
the `role="application"` shell — and then presses keys. Every step records the `performInvocation`
console lines (so a chord that reached nothing shows as zero lines) and the **Inspection panel's own
rendered rows**, which are fed by `marks.graph_selection_ids()`.

| step | key | verb invoked | Inspection panel |
|---|---|---|---|
| baseline | — | — | `INSPECTION Schema: flow.fixture Widgets: 7` |
| 1 | ArrowDown | `selectNextNode` | **`WIDGET Id: height Range: 0..10`** |
| 2 | ArrowDown | `selectNextNode` | **`WIDGET Id: sides Range: 3..12`** |
| 3 | ArrowDown | `selectNextNode` | **`WIDGET Id: radius Range: 0.1..2`** |
| 4 | ArrowRight | `selectDownstreamNode` | **`WIDGET Id: profile Id: brep.curve.polygon`** |
| 5 | ArrowLeft | `selectUpstreamNode` | **`WIDGET Id: sides Range: 3..12`** |
| 6 | ArrowUp | `selectPreviousNode` | **`WIDGET Id: height Range: 0..10`** |
| 7 | Enter | `activateSelection` | `WIDGET Id: height …` (ports of `height` selected) |
| 8 | ArrowRight | `selectDownstreamNode` | **`WIDGET Id: extrusion-axis Id: math.vector`** |
| 9 | Escape | `clearSelection` | `INSPECTION Schema: flow.fixture Widgets: 7` |
| 10 | ArrowDown | `selectNextNode` | `WIDGET Id: height …` |
| 11 | Delete | `deleteSelection` | `INSPECTION No selection` |
| 12 | Ctrl+Z | `undo` | `WIDGET Id: height Range: 0..10` |

Every row matches the fixture, including the two that discriminate the design:

- **Step 5** is the fork. `upstream(profile)` has two candidates; wire order says `radius`, reading
  order says `sides`. The runtime chose **`sides`** — the layout rule, live.
- **Step 8** is the port anchor. After Enter the selection is `height`'s *handles*; ArrowRight still
  stepped from `height` and landed on `extrusion-axis` (wire `e1`), i.e. the node closed itself.

Steps 9–12 prove the three verbs this lane did not add but is responsible for reporting:
`escape` reaches the framework's `clearSelection`, `delete` reaches `deleteSelection` (and the graph
re-evaluated — the outline shows `Vector Computing`), and `mod+z` reaches `undo`, which brought
`height` back. **Zero page errors** across the whole run.

---

## 5. The appearance boot key

### 5.1 The defect, measured

`📓️react-i18n-a11y-customization-2026-09-13.md` §3.2 characterised it and deliberately did not fix it.
Confirmed here on the live serve before touching anything: with
`localStorage["semio.os.config"]` holding `{"mutation":"setAppearance","appearance":"dark"}` and the OS
preferring light, the first painted frame is

```
htmlDarkClass: false, dataUiAppearance: "light", bodyBackground: rgb(247,243,227)
headScriptReads: ["ui.chrome.appearance", "ui.chrome.theme.snapshot"]
```

Nothing has written either of those keys since the shell moved to the event-sourced config lane:
`writeStoredUiChromeAppearance` / `writeStoredUiChromeThemeSnapshot`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1908,1985`) have **zero** callers outside stories and the
demonstrator, both of which only read.

### 5.2 The fix, at the owning layer

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:405-431` — both boot scripts now read the ONE
durable document the shell writes, `localStorage["semio.os.config"]`, take
`preferences["os.config.ui-preferences"]`, and **replay** its append-only event log, because that lane
is event-sourced (`commitUiPreferencesConfigMutation`, `🧑‍🎨engine/🎚️UiPreferences/🟦️.ts:101`) and the
last `setAppearance` is the only appearance there is. The theme script folds `setTheme` and
`setCustomTheme` the same way.

The scripts stay hand-written literals and not imports — an inline `<script>` in a head runs before any
module is fetched, so it can share no code with the engine. What keeps them honest is the fixture in
§5.3 and a law that asserts the hand-authored dev-distribution head
(`💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🌐️.html:10-11`, updated in the same edit) carries the
**byte-identical** scripts.

One deliberate consequence, stated in the docstring: a **built-in** theme is not in storage at all
(only its id is), so it has nothing to pre-apply and boots on the head's inline defaults — as it did
before this fix in every session where the snapshot key was absent, which is all of them.

### 5.3 The law

**Fixture** — `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/🌓️appearance-boot.json`: the storage key,
the preference key, a `deadKeys` roster (the two retired keys the bootstrap must **not** read), eight
`cases` (each an event-log replay plus the system preference, with the resolved appearance and the
body background it must paint) and one `deadKeyCases` row.

**TypeScript half** — `🎨️styling/🧪️tests/🧩️suite/🟦️.ts` (the bun suite `@semio-tech/ui-styling:test`
actually runs): **executes the shipped script string** in a `node:vm` sandbox with the smallest DOM it
touches, and reads the resolved class / dataset / background straight back off it. A head script is
not importable code; running that exact string is the only honest test of it.

**Rust half** — `💻️os/🎚️config/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`: replays the same `events` through
`decode_ui_preferences_config_mutation_json` + `apply_ui_preferences_config_mutation`, the projection
that owns the vocabulary, and asserts the same appearance; plus a law that the fixture's retired keys
are not part of this facet's vocabulary.

```
bun nx run @semio-tech/ui-styling:test            → 58 pass, 0 fail, 1479 expect() calls
cargo test -p semio-framework-os-config --lib      → 52 passed, 0 failed
```

**Negative control:** flipping the `a dark session on a light system boots dark` row to `light`
produced `Expected: "light" / Received: "dark"`; fixture restored, green again.

### 5.4 Runtime proof on 6018

`🐍️appearance-boot-flash-probe.mjs`, output in `🗑️generated/graph-keyboard/appearance-boot/`. Same live
page, same dark `semio.os.config`, OS preferring light, measured at `domcontentloaded` — after the head
ran, before React mounted anything, i.e. exactly the frame a user sees flash. The `after` run rewrites
the served HTML over the wire, substituting the two scripts **read out of the source file at probe
time** (never retyped), because a Vite plugin's module is loaded once at server start and the serve
belongs to another lane.

| run | head reads | `.dark` | `data-ui-appearance` | `color-scheme` | first painted `<body>` background |
|---|---|---|---|---|---|
| `1-served-head-as-is` | `ui.chrome.appearance` | **false** | **light** | light | **`rgb(247, 243, 227)`** |
| `2-shipped-head` | `semio.os.config` | **true** | **dark** | dark | **`rgb(0, 17, 23)`** |

That is the flash, and its absence.

---

## 6. What is NOT claimed

1. **In-canvas camera focus is still impossible from the guest, and this lane did not add it.** An
   "activate = centre the graph on the node" verb cannot be written today, for two independent
   reasons, both read off the host: the node-graph camera is host-owned and the React host
   deliberately **never** copies `scene.viewport` on a resync (`🕸️NodeGraph/🟦️.tsx:1913-1914` says so in
   as many words — live pan/zoom lives in the `FlowWasmSession`), and the app's own
   `nodeGraphViewport` writes `config.camera` while the window renders `fixture.camera`
   (`🕸️flow/🦀️.rs:275`), with `fixture_ops_ignore_camera` making the camera a Config-lane fact by law.
   Giving the guest a camera would need a new keyed one-shot request lane on `NodeGraphScene` plus its
   wgpu twin — the wgpu node graph is owned by a live peer lane, so it was not started here.
   `activateSelection` therefore opens the node's **ports**, which is expressible, useful to a
   keyboard user, and proven.
2. **"Open the Inspection panel" is also not reachable from a guest verb today**, and was rejected for
   the same reason rather than shipped as a no-op: `setActivePanelTab` is intercepted only in
   `hostMode` (`🏛️ShellHost/🟦️.tsx:6407`), and the generation3d playground has no `hostConfig`, so
   `hostMode` is false there; `Effect::SetPanel` writes only the space `panelJson`, not the panel
   path/visibility. Fixing that gate is a real framework improvement and is left flagged, not done.
3. **No screen reader was run.** `aria-keyshortcuts="Enter"` and the activate binding are verified
   present in the projected tree by law; "a reader announces the chord" is a structural argument.
4. **The wgpu renderer (6118) was not exercised.** The four chords are app-level and the wgpu shell has
   its own keybinding loop; the verbs and their laws are renderer-neutral, but no 6118 run was made —
   the wgpu lanes own that port.
5. **The `🧪️playgroundflowwasmdevstubplugin` in-source assertions were updated but are dead code.**
   `🏗️builder/🌐️vite/🟦️.ts`'s `import.meta.vitest` block is named by no vitest config's
   `includeSource`, so those two boot-script expectations have not been executed by any target. They
   were corrected in place (they asserted the retired keys) and the **live** law is the one added to
   `🧩️suite/🟦️.ts`. Enabling that config is somebody's separate cleanup.
6. **`semio-framework-artifact-flow-flow`'s own test target does not compile, and it is a peer's.**
   `cargo check -p semio-framework-artifact-flow-flow` (non-test) is clean, including this lane's new
   code; `--lib --tests` fails in `🧵️retained/🧪️tests/🧵️retained/🦀️.rs` with
   `no method named reserve_push_allocation / next_push_allocation_bytes found for FlowRetirement` —
   `🧵️retained/🦀️.rs` is modified in the working tree by the `flow-eval-session-retirement` lane whose
   test file has not caught up. Not touched.
7. **Three generation3d `component::unit_tests` reds are pre-existing and not this lane's.**
   `component::unit_tests` → **60 passed, 3 failed**:
   `generation_preview_is_one_app_transient_shared_by_two_generation_windows` (`transient=0`),
   `two_instances_converge_disjoint_widget_moves` (`module.vcs` fail-closed merge) and
   `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`
   (`generation3d-publication.contended`). All three are already recorded as pre-existing or peer-owned —
   `📓️close-out-fixes-2026-09-12.md` row 6, `📓️hotpath-optimization-2026-09-10.md` and `📓️status.md`'s
   own attribution line — and none of the three messages mentions any verb, lane or fixture this lane
   added.
8. **The full generation3d `--lib` suite was not run to completion here.** It carries the ~115
   close-ladder reds `📓️editor-verbs-cancel-undo-2026-09-13.md` §7.1 characterised; this lane ran the
   two modules it changes (`navigate_graph`, `flow::tests`) plus the whole `component::unit_tests`
   manifest/route module.

---

## 7. Commands run

- `cargo check -p semio-framework-artifact-flow-flow` — clean
- `cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --tests` — exit 0
- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- navigate_graph flow::tests:: the_editor_binds_every_keyboard retained_route_dispositions every_printed_op_line` — **19 passed, 0 failed**
- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- editor::generation3d::component::unit_tests` — 60 passed, 3 failed (§6.7)
- `cargo test -p semio-framework-os-config --lib` — **52 passed, 0 failed**
- `bun nx run @semio-tech/ui-styling:test` — **58 pass, 0 fail**
- generation3d graph-keyboard TypeScript twin — **47 checks**
- `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` — exit 0, 16m 45s, 11 components (changed)
- `bun 🐍️graph-keyboard-nav-probe.mjs`, `bun 🐍️appearance-boot-flash-probe.mjs` — §4, §5.4

**Probes (ticket root, kept)** — `🐍️graph-keyboard-nav-probe.mjs`, `🐍️appearance-boot-flash-probe.mjs`.
**Probe output (`🗑️generated/graph-keyboard/`, deletable)** — `keys-4/` (13 screenshots + `results.json`),
`appearance-boot/` (2 screenshots + `results.json`).
