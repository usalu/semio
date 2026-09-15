# 🕹️ What a hover costs — narrowing the interaction refresh scope (lane `interaction-scope-narrowing`, 2026-09-15)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`. Target: the React serve this lane owns,
`http://127.0.0.1:6024/?plugin=generation3d`, and the coordinator's wgpu serve `:6118`.
Closes the first open item of `📓️wgpu-selection-roundtrip-2026-09-15.md` §7 — *`interactionHover` now
costs a FULL shell refresh on both renderers, because generation3d declares no
`ArtifactApp::interaction_scope`*.

Repo MCP was down all session (`repo CONNECTION_CLOSED`, `semio CONNECTION_CLOSED`); no ticket was
opened, closed or reopened. No `git commit/stash/checkout/worktree`. Nothing under `🗑️generated` that
this lane did not create was deleted. No peer process was killed.

---

## 1. TL;DR

**The scope was never app knowledge in the first place — the app already declares it.**

`WindowKindDefinition.interactions` (`.window_kind_interactions(...)`) is the list that lets a renderer
dispatch an interaction verb from a pane at all. generation3d has declared it since the hover/selection
mechanism landed:

```rust
.window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,             vec![InteractionRef::new("graph")])
.window_kind_interactions(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW,         vec![InteractionRef::new("graph")])
.window_kind_interactions(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
```

So the framework can answer "what does a hover repaint" from the manifest, with no hand-written list per
window and no per-app hook: the windows that declare the touched domain, and nothing else. That
derivation is now the framework DEFAULT for every app —
`A::interaction_scope(...).or_else(|| registry.interaction_declared_refresh_scope(...)).unwrap_or(Full)`
— so generation3d needed **no new declaration at all**, and every other app that declares its window
interaction refs gets the same cut for free. `UiDirtyScope::Full` survives only where there is nothing
to derive from: a verb whose touched domain no window kind declares.

Measured consequence of one pointer move on the procedural 3d editor:

| | React `:6024` | wgpu `:6118` |
|---|---|---|
| scope the guest publishes for `interactionHover` | `full` → **`partial`** (3 window bodies, 0 panels, no sections, no catalogue) | `full` → **`partial`** (same 3) |
| bodies the guest renders per hover | **10 → 7** | — |
| surfaces the shell renders per hover (`render begin`) | — | **7 → 2** |
| wall per hover, pointer event → quiescence | **1 143 ms → 931 ms (−19 %)** | not timed (see §6) |

The select/clear half is narrowed too and is measured green, but its wall cost did **not** move,
because a host-side owed `refresh` pass still asks for `{kind:"full"}` on every pick (§5.3) — named
here as the next cut, not fixed.

---

## 2. The design

### 2.1 Why this is a framework derivation and not a table in the app

`puzzle3d`'s `puzzle3d_scope(Puzzle3dScopeClass::Interaction(verb))` is the pattern this lane was asked
to follow, and it works — but it is a hand-written body list per verb, held in sync with the window
declarations by a unit test. Every app that grows a window has to remember it twice. The
`window_kind_interactions` refs already say which panes paint a domain; the framework simply never read
them for this question.

### 2.2 The rule, stated once

`semio_framework::interaction_verb_surface_scope(verb, window_bodies, panel_bodies)`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`, new region `🔖️InteractionRefreshScope`):

| lane | answer | why |
|---|---|---|
| `window_bodies` | the body of every window kind declaring a touched domain | that ref is exactly "this pane paints this domain" |
| `panel_bodies` | every declared panel leaf body for `Select`/`ClearSelection`/`SelectAll`; **empty** for the other three | a `PanelTabDefinition` declares no interaction refs, so the framework cannot know WHICH panel paints a selection — keeping them all is the widest honest answer and can never under-refresh an inspector. The one lane narrowed without a declaration is `Hover`: hover is pointer-transient, owed to the surfaces the pointer is over, and a panel row that chased the pointer would flicker rather than inform. |
| `measures` | `true` for every verb but `Hover` | the Select chrome (active mode / granularity) is rail-bound persisted state; a hover moves none of it |
| `utilities` / `tools` / `engagements` / `labels` / catalogue | never | no interaction verb moves them; a `partial` scope never re-fetches the app-static catalogue (`uiDirtyScopeWantsCatalogue`) |

`interaction_declared_refresh_scope(verb, domains, windows_by_domain, panel_bodies)` returns `None` —
and with it the framework's `UiDirtyScope::Full` — whenever **no** declared window paints a touched
domain. That is the only remaining path to a whole-shell hover, and it is the honest one: there is no
declaration to derive from, so narrowing would be a guess.

### 2.3 Where it is read

`AppActionRegistry::from_definition` indexes the two inputs once per app
(`interaction_window_bodies_by_domain`, `panel_leaf_body_keys`) instead of walking the definition per
dispatched hover; both new fields join the registry's bounded-close discipline (`close_step`,
`terminal_is_empty`). `dispatch_interaction_action` then reads:

```rust
let scope = A::interaction_scope(verb, &declared)
    .or_else(|| self.registry.interaction_declared_refresh_scope(verb, &declared))
    .unwrap_or(UiDirtyScope::Full);
```

An app that DOES declare `interaction_scope` (puzzle3d) still wins — its table is checked first, so this
lane changes nothing for it.

---

## 3. Files

**Changed (the derivation):**
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — new `🔖️InteractionRefreshScope` region:
  `interaction_verb_surface_scope`, `interaction_window_bodies_by_domain`, `panel_leaf_body_keys`,
  `interaction_declared_refresh_scope`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `AppActionRegistry` gains
  `interaction_window_bodies` / `panel_body_keys` (built in `from_definition`, retired in `close_step`,
  asserted in `terminal_is_empty`) and `interaction_declared_refresh_scope`;
  `dispatch_interaction_action` falls through to it; `ArtifactApp::interaction_scope`'s contract doc
  now names the derived default.

**Changed (the laws):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` —
  `an_undeclared_interaction_verb_keeps_the_framework_full_scope` became
  `an_undeclared_interaction_verb_falls_through_to_the_declared_surface_scope`; new sibling
  `a_domain_no_window_declares_keeps_the_framework_full_scope`; `interaction_registry()` split so the
  synthetic `AppDefinition` is nameable (`interaction_app_definition()`).
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` —
  registers the new `🧪️InteractionScope` test module. **No declaration was added to the app**: the
  window refs it already carried are the whole input.

**New:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🕹️interaction-scope.json`
- `✏️s/…/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🦀️.rs` (Rust law, 5 tests)
- `✏️s/…/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🟦️.ts` (TS twin)

**Ticket:**
- `📓️interaction-scope-narrowing-2026-09-15.md` (this report), `🐍️hover-cost-probe.mjs` (new),
  `🗑️generated/scope/` (this lane's evidence root).

---

## 4. Laws

### 4.1 Rust — per reserved verb → expected surface set, for BOTH roles

Fixture `🧫️fixtures/🕹️interaction-scope.json` carries, per role, the app's declared window/panel
rosters, the bodies whose render consumes the interaction marks (`publishesHover`/`publishesSelection`),
the bodies that must stay quiet on a hover, and the exact expected scope of all six verbs. The law
builds the REAL `AppDefinition`s (`create_generation3d_app()`, `create_generation3d_viewer()`).

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib interaction_scope -- --nocapture
test result: ok. 5 passed; 0 failed
```

measured `[SCOPE]` rows:

| role | verb | window bodies | panel bodies | measures |
|---|---|---|---|---|
| editor | `interactionHover` | main, preview, generate-preview | — | false |
| editor | `interactionSelect` | main, preview, generate-preview | document, catalogue, inspection, framework.body.history, framework.body.toolRun | true |
| editor | `clearSelection` | main, preview, generate-preview | (same five) | true |
| editor | `selectAll` | main, preview, generate-preview | (same five) | true |
| editor | `setSelectionMode` | main, preview, generate-preview | — | true |
| editor | `setInteractionGranularity` | main, preview, generate-preview | — | true |
| viewer | `interactionHover` | procedural.view.preview | — | false |
| viewer | `interactionSelect` / `clearSelection` / `selectAll` | procedural.view.preview | framework.body.history, framework.body.toolRun | true |
| viewer | `setSelectionMode` / `setInteractionGranularity` | procedural.view.preview | — | true |

The five tests are: the table above verbatim; the fixture rosters ARE the definitions' own rosters; **a
verb never narrows below the surfaces that publish its lane** (and a hover never reaches
`generations` / `generate-form` / the Artifact and Catalogue panels); an undeclared domain — and a verb
that touched no domain — keeps `Full`; and the counter-model, that the `Full` this replaces wanted every
body of both roles.

The app half is load-bearing: `generation3d_render_body` hands the interaction marks to exactly
`flow_window`, `edit_preview`, `generate_preview` and `inspection_panel`, and those four are the
fixture's `publishes*` sets.

### 4.2 Rust — the framework default, failing-first

```
cargo test -p semio-framework-plugin --lib --features artifact-app-testing interaction
test result: FAILED. 50 passed; 7 failed
```

The two laws this lane owns pass. **Failing-first, measured**: with the `or_else(...)` fall-through
removed — the exact pre-lane shape — the same suite answers

```
✗ an_undeclared_interaction_verb_falls_through_to_the_declared_surface_scope
  left: Full   right: Partial { window_bodies: ["synthetic.main"], panel_bodies: [], measures: true, … }
test result: FAILED. 49 passed; 8 failed
```

and the other **7 failures are identical with and without this lane's change**, which is how they were
attributed: five are `interactive-job.missing-factory` faults on `TestApp`'s typed `setLabel` command,
one is `assertion failed: presence.is_empty()` in the live interaction-dispatch suite, one is
`set_selection_mode_and_set_interaction_granularity_persist_immediately` refusing an undeclared
granularity through the reserved-verb SpawnJob admission. None of them touches a refresh scope; the same
two files are being edited concurrently by the `worker-more-work-drive` and `wgpu-selection-roundtrip`
lanes.

### 4.3 TypeScript — the twin where the host reads the scope

`✏️…/✏️editor/🧪️tests/🕹️interaction-scope/🟦️.ts` re-derives the table from the app's declared surfaces
alone (a third-party implementation of the Rust rule, written from the fixture's vocabulary), then reads
each row back through the KERNEL predicates `buildUiRefreshRequest` filters a `refresh-ui` request with
(`uiDirtyScopeWantsWindowBody` / `WantsPanelBody` / `WantsSection` / `WantsCatalogue`), so both halves of
the round trip — what the guest publishes and what the host then fetches — are held against one table.

```
bun -e 'import {testGeneration3dInteractionScopeContract} from "./🟦️.ts"; testGeneration3dInteractionScopeContract()'
[SCOPE] editor hover 3 sections vs full 15 + catalogue
[SCOPE] viewer hover 1 sections vs full 8 + catalogue
TS interaction-scope contract: ok
```

i.e. one hover asks the guest for **3 of 15** fetchable sections in the editor and **1 of 8** in the
viewer, and for the catalogue in neither.

---

## 5. The React cost, measured on `:6024`

### 5.1 Method

`🐍️hover-cost-probe.mjs` (new). Nothing is inferred: the host prints
`[DEBUG] refreshUi sections {scope,asked,…}` per pass and the guest prints
`[DEBUG] gen3d render body=<key>` per rendered body, both armed by the probe writing the
`SEMIO_RUNTIME_DIAGNOSTICS` key into `localStorage` before the page boots. A gesture's cost is the delta
of those counters across it, and the scope is read out of the host's own line. Gesture points are swept
off the preview canvas until the pane publishes a `hoverTarget`, the way `🐍️interaction-matrix-probe.mjs`
finds geometry. Example: `box-shell-preview`, converged, 0 page errors.

### 5.2 Before → after, per hover

The before/after boundary was measured on the SAME page: the shard worker that served the run in
`🗑️generated/scope/before/` was replaced mid-run when a peer's restage put the new guest on disk
(two 12 s stalls at hover #4/#5 are that swap), so hovers 0–4 are the old guest and 6–7 the new one.
`🗑️generated/scope/after/` is a clean run on the restaged guest.

| | before (`scope/before`, hovers 0–3) | after (`scope/after`, 8 hovers) |
|---|---:|---:|
| scope the guest published | `full` | `partial` |
| window bodies named | *(all)* | main, preview, generate-preview |
| panel bodies named | *(all)* | — |
| sections + catalogue | all | none |
| refresh passes per hover | 1.00 | 1.00 |
| **guest body renders per hover** | **10** | **7** |
| **wall per hover (median)** | **1 143 ms** | **931 ms** |

Per-row, before: `1170 / 1143 / 1163 / 1140` ms, 10 renders each, scope `full`.
After: `935 / 937 / 933 / 983 / 923 / 923 / 929 / 921` ms, 7 renders each, scope `partial`.

The host's own line after the change:

```
[DEBUG] refreshUi sections {"scope":{"kind":"partial","windowBodies":["procedural.play.main","procedural.play.preview","procedural.play.generate-preview"],
  "measures":false,"utilities":false,"tools":false,"engagements":false,"labels":false},"asked":["procedural-main","procedural-preview"], …}
```

`asked` is two, not three, because the mounted-window fetch (`📓️react-hop-latency-2026-09-14.md` §3.1)
drops `generate-preview` while edit mode is on screen — the two cuts compose.

### 5.3 Select and clear — narrowed, and still not faster

| gesture | passes | guest body renders | wall (median) | scopes seen |
|---|---:|---:|---:|---|
| select | 2.25 | 22 | 1 799 ms | `partial`, **`full`** |
| clear (Escape) | 1.00 | 10 | 1 275 ms | `partial` |

The guest's own answer for a pick is now the narrow one, but a **host-side owed pass still asks for
`{kind:"full"}`** on every pick:

```
[DEBUG] refreshUi lane {"decision":"owed","scope":{"kind":"full"},"passes":40}
[DEBUG] refreshUi lane {"decision":"pass","scope":{"kind":"full"},"passes":41}
```

— four of them, one per select, and none during any hover. That owed full pass is what keeps the select
wall identical before and after, and it is the next cut on this path. It is NOT
`hostEffectRefreshScopeV1`'s `setPanel`/`loadDocument` widening (neither effect appears anywhere in the
run's console) and it is not one of the tutorial `refreshUi(session, {kind:"full"})` sites; this lane
did not find its owner and does not claim one.

---

## 6. The wgpu cost, measured on `:6118`

Guest restaged (`activate-generation3d-wgpu-dev`, exit 0). The derivation is live on this renderer too —
the wgpu shell prints the scope it dispatched with:

```
[DEBUG] wgpu-shell dispatch action=interactionHover  scope=partial windows=[procedural.play.main,procedural.play.preview,procedural.play.generate-preview] panels=[] … measures=false
[DEBUG] wgpu-shell render begin surface=procedural-main    body=procedural.play.main
[DEBUG] wgpu-shell render begin surface=procedural-preview body=procedural.play.preview
[DEBUG] wgpu-shell refresh scope=partial windows=[…] panels=[]
```

Counted over the whole `world3d-editor` row (`🗑️generated/wgpu-verify/world3d-editor/console.txt`):

| role | verb | scope dispatched | `render begin` per gesture |
|---|---|---|---:|
| editor | `interactionHover` (6) | `partial` | **2** (procedural-main + procedural-preview) |
| editor | `interactionSelect` (28) | `partial` | 7 (2 windows + 5 panel bodies) |
| viewer | `interactionHover` (10) | `partial` | **1** (procedural-view-preview) |
| viewer | `interactionSelect` (32) | `partial` | 3 (1 window + history + toolRun) |

The two roles answer exactly the two rows of the Rust fixture (§4.1), read back off the live shell.

The before-number for the same shell is its own full pass: the pre-lane console
(`🗑️generated/wgpu-oracle/world3d-editor/console.txt`, 2026-09-15 01:52) records **54 × `wgpu-shell
refresh scope=full rendered=7`** on this app — i.e. a whole-shell pass renders 7 surfaces. A hover now
renders **2**. (In that same pre-lane console every interaction verb answered `scope=none rendered=0`:
that is the dropped-answer defect `📓️wgpu-selection-roundtrip-2026-09-15.md` fixed, not a cost baseline,
which is why the "7" is taken from the full passes beside it rather than from the hover rows.)

Refresh passes by scope kind over the fresh editor row: `full 175, partial 42, none 7`. The 175 full
passes are `setCamera` (105 dispatches) and `setActiveExample` (8) — app actions this lane does not
touch.

---

## 7. No under-refresh — the rows

| row | where | result |
|---|---|---|
| `outline-selection` | React `:6024` | **10/10**, 0 page errors — including *Escape retires the mark* `[]` and *a traversal after Escape re-marks exactly one row*. G1-style stale marks do not return: `clearSelection` now rides the narrowed scope and the outline's presence mark still retires in the same turn. |
| `graph-keyboard` | React `:6024` | 13/13 steps, 0 page errors. The Inspection panel follows every traversal (`procedural-play-inspector.id=Id: height → sides → radius → profile`) and goes empty on `escape-clear` — whose `invoked` list is exactly `["clearSelection"]`, i.e. the panel re-rendered on the NARROWED clear scope. |
| `interact` | React `:6024` | **53/56**, 0 page errors. Three reds, neither an under-refresh: `Hexagonal Mushroom Column inspector` read an empty panel once and is **green on re-run**; `Rectangle Wire Preview select` + `inspector` fail because the pointer never lands on the 1-px wire — on the re-run its **hover** already fails after sweeping all 35 grid points, and `guestSelectedIds` is empty, so the guest never took a pick (upstream of any refresh). |
| `world3d-editor` | wgpu `:6118` | 62/115, 0 page errors — see §8, not this lane's and proven so. |
| `world3d-viewer` | wgpu `:6118` | 63/115, 0 page errors — identical failure shape, same proof. |

Battery run: `bun 🐍️wgpu-battery.mjs --only=world3d-editor,world3d-viewer` on `:6118`, 4 906 s,
`pageerrors=0`, started only after the standing probe gate cleared; no peer process was killed.

---

## 8. What is NOT claimed

- **The `world3d-editor` battery row is 62/115 on the current 6118 guest and the regression is not this
  lane's — proven, not asserted.** The reds are `h1 hovering the centre reports the example's own
  target {"hover":null}` on 7 of 8 examples and every selection assertion downstream of it. The wgpu
  world surface reports `objects=0 draws=0` at that moment, against an IDENTICAL camera
  (`[4,-4,3]->[0,0,0]/45deg`), pane (`459x814+978,54`) and centre (`1207.5,461`) to the pre-lane run of
  01:44, which reported `objects=1 draws=1` and `hover=Some("shell@solid")`. The decisive reading:
  **before that first hover, ZERO interaction verbs had been dispatched** — the passes preceding it are
  `setActiveExample scope=full` ×8, one `toolRunStart scope=partial` (a pre-existing tool-run scope) and
  one `none`. A narrowing that has not yet been applied cannot have emptied the scene. The React twin,
  on the same guest with the same narrowed scope, keeps hover+select green on 7 of 8 examples. The row
  scored 75/115 at 09:35 (`📓️wgpu-mesh-oracle-2026-09-14.md`); the viewer row (63/115) fails the same
  way, with the same reading — `objects=0 draws=0`, zero interaction verbs dispatched before it, and
  7 `full` passes plus one `toolRunStart partial` preceding it. The tree has since taken the
  selection-roundtrip bridge change, the `boot-camera-framing` lane's wgpu half (landed 09:40 with its
  wgpu reading explicitly **not taken**), and two in-flight reactor lanes. Handed on with this evidence;
  the owner is the world-scene delivery to the wgpu `procedural-preview` surface, not the refresh scope.
- **The host-side owed `{kind:"full"}` refresh on every React pick was not removed** (§5.3). It is why
  select is narrowed but no faster. Its owner was not found.
- **Panels are not domain-declarable.** `PanelTabDefinition` carries no `interactions` field, so the
  three selection verbs keep EVERY declared panel body — the Artifact and Catalogue panels included,
  which `generation3d_render_body` renders without ever reading the marks. Adding
  `panel_tab_interactions(...)` beside `window_kind_interactions(...)` would cut the editor's select
  scope from five panel bodies to one; it needs a `🛂️manifest` field, the generated TS manifest mirror
  (`bun nx run @semio-tech/framework:generate`) and a pass over the 19 other apps that declare an
  interaction domain, and this lane did none of it.
- **`measures: true` for the five non-hover verbs is the framework's opinion, not a declaration.** No
  generation3d measures rail binds a selection mode or granularity, so those passes fetch a section this
  app does not need; the rule is kept because a rail that DOES bind them (puzzle3d's) must never
  under-refresh, and puzzle3d's own `interaction_scope` still wins over the derivation anyway.
- **The wgpu hover was not timed.** Only `render begin` per gesture was counted; the wgpu probe harness
  measures gestures, not wall-to-quiescence, and this lane did not add that.
- **No app other than generation3d was measured**, although all of them now get the derived default. The
  framework law covers the shape (`a_domain_no_window_declares_keeps_the_framework_full_scope`), the
  generation3d law covers the content, and every app that declares no window interaction refs for a
  touched domain is unchanged.
- **The 7 pre-existing reds of `cargo test -p semio-framework-plugin --lib … interaction`** (§4.2) were
  not fixed; they fail identically with this lane's change reverted.
- **`(unchanged)` restages.** Both `activate-generation3d-*-dev` runs reported the components unchanged
  because a peer's restage had already compiled this lane's framework edit into the shared tree at
  10:42; the served wasm was confirmed by its behaviour (the scope kind in the host's and the wgpu
  shell's own lines), not by the nx message.
