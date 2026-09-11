# Wave B27 — the catalogue row was never covered, and the admitted-operation refresh already follows the completion

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B27, 2026-09-12 00:00–01:0x CEST. Assignment: (1) find and
fix the layout/z-order cause of "catalogue rows are covered by the navbar"; (2) fix the architecture of
"admitted typed operations never refresh the panels".

No git write, ticket not opened/closed, `🗑️generated` written to and never deleted, no wasm build run,
every command foreground, no `[DEBUG] b27` tap left anywhere (`rg -n "DEBUG\] b27"` over `🧰️framework`
and `✏️s` → no match).

**Headline — both defects were mis-attributed, and this wave has the live readings that say so.**

| # | what the ticket believed | what the live shell says | where it actually is |
| --- | --- | --- | --- |
| 1 | `nav#ui.navbar` covers the catalogue rows (B26 §5) | the navbar's computed `z-index` is **0**, BELOW `z-panel`; `elementsFromPoint` over the open panel returns the PANEL at every height inside the navbar band. The catalogue's four sections are **folded**, so every kind row is `display:none`, its rect is `{0,0,0,0}` and the probe's hit-test degenerates to `elementFromPoint(0, 0)` — the viewport origin, which IS the navbar | `📌️panels/🛍️catalogue/🦀️.rs:153-156` — every section declared `default_open: false` |
| 2 | a `mutationCount: 0` admission yields `{kind:"none"}`, so the panels are never re-taken | the completion lane already carries the mutation's own scope and re-takes the panel: `setSelectionFlag` → one completion with `panelBodies:["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history","puzzle.3d.play.kinds"]`, row flips `Hide`→`Show` at **1.86 s** on a throttled tab | not reproducible on wasm #49. The real residual is the OPPOSITE — a refresh **storm**: 20 completions per idle 10 s, half of them empty, each paying a full host-effect pass |

Everything quoted below is real output: `cargo`/`vitest` tails from this pass, and DOM/console readings
taken live off the `:6013` React serve running wasm **#49** (the coordinator's deploy finished
`00:07:16`; this page was loaded after it).

---

## 1 §18 `catalogue-add-object-kind` — the row is FOLDED, not covered

### 1.1 The measurement that ends three waves of guessing

B26 §5's evidence was `document.elementFromPoint(box.x + box.width/2, box.y + box.height/2)` over the
first kind row's `[data-slot="tree-label"]`, answering a child of `nav#ui.navbar`. Reading the SAME
element live, with the Catalogue open:

```
labelRect: { "bottom": 0, "height": 0, "left": 0, "right": 0, "top": 0, "width": 0, "x": 0, "y": 0 }
labelText: "Hexagonal Cut Concrete Forest Left Hexagonal Cut Concrete Forest Left"
top:       "DIV#-[-]{p-single flex gap-single items-center min-w-0 h-fu}"
```

The label's box is **all zeros**, so the hit-test asks what is at `(0, 0)` — the viewport origin, which is
inside the navbar at any viewport size. `click({force:true})` lands there too. "Covered by the navbar" was
the probe reporting the coordinate origin.

Walking up from that row finds the reason (live):

```
DIV [tree-item-row]         h=0   id=panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left
DIV [tree-section-content]  h=0
DIV [collapsible-content]   h=0   display:none  hidden:true   id=semio-collapsible-_r_1q_-content
DIV [collapsible]           h=24
…
sections: OBJECTS aria-expanded=false · VORTICES false · CABLES false · ATTRACTIONS false
panel text: "Artifact | Catalogue | Collapse | OBJECTS | VORTICES | CABLES | ATTRACTIONS"
```

Opening the Catalogue shows **four empty headers**. A folded row still answers `querySelectorAll`, which
is exactly why `catalogue-kind-rows-present` and B26's `data-activatable` dump were green on rows that
were not laid out at all.

### 1.2 Proof that the fold is the whole defect (live, host, no rebuild)

Expanding OBJECTS in the live page and re-measuring the same label:

```
exp: "true"
labRect: { "top": 38.59, "bottom": 62.59, "left": 190.97, "right": 283.80, "height": 24 }
covered: false      activatable: "true"     rowOfTop: "panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left"
```

then a real hit-tested press (`elementFromPoint` → `pointerdown/mousedown/pointerup/mouseup/click` on
what the hit-test returned):

```
before: ["seed-left-001", …74 brushes]
after:  ["seed-left-001", "puzzle3d.object.9acf0e0e279673de", …74 brushes]
```

**The row's own add gesture works.** The only thing between the user and it was the fold.

### 1.3 And the navbar genuinely does not cover the dock

Same page, Catalogue open, `1512×900`:

```
navRect   { top: -12,   bottom: 16.80, height: 28.80 }          ← layout is h-screen inside an 885px host, hence the −12
panel     { top: -8.80, bottom: 111.59, width: 300, class: "… z-panel flex-col" }
cap       { top: -8.80, bottom: 13.59, height: 22.39 }
body      { top: 13.59, bottom: 111.59 }                        ← 3.20px of it inside the navbar's box
elementsFromPoint(153, 14)  → DIV[tree-gutter] → DIV[tree-row-layout] → DIV#panel:…kinds.objects[tree-section-row]
elementsFromPoint(153, 0)   → BUTTON#framework.panel.catalogue[panel-tab-button] → DIV[panel-tabs] …
navPE: "auto"   navZ: "0"
```

`navZ: "0"` is the point: `🎨️ui.css:50-57` forces
`[data-slot="navbar"], [data-slot="footer"] { z-index: var(--z-base) !important }`, so the chrome has
always painted and hit-tested BELOW `z-panel` (30). Both the panel's cap AND the 3.2 px of body inside the
navbar's box answer to the panel. The `z-navbar` class the two components carried was overridden on every
page they ever rendered — a source that said the opposite of the browser, and half of why this red read as
a stacking bug.

### 1.4 Fixes

**Guest (rides the next wasm, #50)** — `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:153`:
the `objects` section is `default_open: true`, exactly like the outliner's own primary section
(`📌️panels/🗿️artifact/🦀️.rs:316` has always been `true, false, false, false`). The three template
catalogs (vortices/cables/attractions) stay folded, so the first `[role="treeitem"]` under the panel — the
element the battery presses — is an object kind. The `render` docstring now carries the whole story.

**Host (vite-live now)** — `🔝️Navbar/🟦️.tsx:75` and `🔚️Footer/🟦️.tsx:52` declare
`getLevelZClass("base")` instead of the dead `z-navbar`, with the invariant written down on `Navbar`: a
chrome-hosted `Panel` unfolds INTO this band, so the band must never stack above `z-panel`.

**Law (ui-react, host-live)** `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
→ *"keeps shell chrome at the base stacking level and lands an open chrome-hosted panel's cap exactly on
the chrome control row"*. It states the geometry from the same `STYLING_METRICS.chrome` tokens the CSS is
generated from — band `9u`, control row `7u`, padding `1u`, `chromeHostedOpenPanelPositionStyle`'s
overhang `(9+7)/2 = 8u` — and asserts the cap row `[band−overhang, band−overhang+control] = [1u, 8u]`
IS the chrome's own control row `[pad, band−pad]`, so the panel's body begins at or below that row's end
and no dock row can ever sit under a chrome control. Then it renders `Navbar`, `Footer` and an open
chrome-hosted `Panel` and pins `z-base` / `z-base` / `z-panel` with `LEVELS.indexOf("panel") >
LEVELS.indexOf("base")`.

```
✓ |@semio-tech/ui-react| 🟦️.tsx > Shell components > keeps shell chrome at the base stacking level and lands an open chrome-hosted panel's cap exactly on the chrome control row 67ms
 Test Files  1 failed | 1 passed | 20 skipped (22)
      Tests  1 passed | 713 skipped (714)
```

Red with the old class restored (real coverage, not a tautology):

```
AssertionError: expected 'relative h-large z-navbar ui-surface' to contain 'z-base'
      Tests  1 failed | 713 skipped (714)
```

**Law (guest)** `📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` →
`the_catalogue_opens_its_object_kinds_and_folds_the_template_catalogs`.

```
test editor::puzzle3d::panels::catalogue::tests::every_object_kind_row_binds_activate_to_add_object_kind_with_its_own_kind_id ... ok
test editor::puzzle3d::panels::catalogue::tests::kinds_tree_object_drag_data_carries_object_kind_and_mesh_url ... ok
test editor::puzzle3d::panels::catalogue::tests::the_catalogue_opens_its_object_kinds_and_folds_the_template_catalogs ... ok
test editor::puzzle3d::panels::catalogue::tests::the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 715 filtered out; finished in 0.09s
```

Red at HEAD (with `false` restored):

```
assertion `left == right` failed: the placeable object kinds must be laid out when the panel opens
  left: Some(false)
 right: Some(true)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 719 filtered out; finished in 0.08s
```

### 1.5 Verdict before / after

`catalogue-add-object-kind` — **still red until wasm #50 ships**, and honestly so: the fold is declared by
the guest, and this wave ran no wasm build (the coordinator owns the deploy). The host half is proven
green live (§1.2: a real press on the unfolded row added `puzzle3d.object.9acf0e0e279673de`), and the
stacking half is proven green live and pinned by the ui-react law. **First lane to re-run on #50:**
`bun 🔍️browser-probe.ts --only=catalogue-panel --port=6013`.

---

## 2 §17 the admitted-operation refresh — the lane works; the storm is the defect

### 2.1 The stated defect does not reproduce on #49

Live, diagnostics armed (`localStorage SEMIO_RUNTIME_DIAGNOSTICS=1`), the Artifact panel open, pressing
the outliner's `Hide` on `seed-left-001` and sampling the row every 100 ms:

```
[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setSelectionFlag"}
[DEBUG] command ingress lane {"instanceId":1,"actionId":"setSelectionFlag","seq":190,"lane":"Interactive"}
[DEBUG] completion apply {"operation":11264,"scope":{"engagements":false,"kind":"partial","labels":false,
  "measures":true,"panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document",
  "framework.body.history","puzzle.3d.play.kinds"],"tools":false,…}}
[DEBUG] applyHostEffects refresh {"scope":{… same partial …}}
[DEBUG] performInvocation settled {…"actionId":"setSelectionFlag","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyUpserts":0,"effects":0}

samples: [{"ms":906,"t":"…|Show|Lock"},{"ms":1907,"t":"…|Hide|Lock"}]      ← toggled back and forth, both directions land
```

So on wasm #49: the mutation's **completion** carries a partial scope naming `puzzle.3d.play.document`,
`applyHostEffects` re-takes it, and the row's label flips within ~1.9 s on a hidden (timer-throttled) tab.
Two further corrections to B26 §6's reading while we are here:

- `setSelectionFlag` does **not** travel the browser-actor route at all — the console shows
  `performInvocation` + `command ingress lane`, i.e. the plugin-channel route, so
  `browserActorDispatchUiScopeV1` (B9 §4) never runs for it and cannot be its cause;
- the architecture the assignment asks for — *"the refresh must follow the operation's COMPLETION, dirty
  scope derived from the completion frame"* — is **already what `ShellHost` does**:
  `subscribeOperationCompletions` → `applyHostEffects(completion.requestedEffects, target,
  <completion scope>, owner)` (`🏛️ShellHost/🟦️.tsx:5333-5347`), fed by the guest's own
  `TypedOperationCompletionWitness { operation, ui_scope }` (`🔌️plugin/🦀️.rs:24205`).

### 2.2 What IS wrong: every completion pays, including the empty ones

Same page, **zero interaction**, 10 s window, counting the two diagnostics:

```
windowMs: 10178   completions: 18   refreshes: 36   scopes: { none: 12, partial: 6 }
sample:  completion apply {"operation":19712,"scope":{"kind":"none"},"historyPatch":false,…}
         completion apply {"operation":19648,"scope":{"engagements":false,"kind":"partial",…,"tools":true,
                           "windowBodies":["puzzle3d.play.composite"]},…}
```

A retained operation completes on **every drain poll** (operation ids step by 64, the typed-operation slot
count), so an idle shell finishes ~2 completions per second — and each one ran a full `applyHostEffects`
pass: a `SET_SESSION` dispatch plus a `refreshUi` call, for a completion that dirtied nothing, patched no
history and requested no effect. That is the "refresh storm on View-kind completions" this wave was told
to keep out.

### 2.3 Fix — one rule, stated once, at the completion

New pure `typedOperationCompletionRefreshV1` in `🛠️ShellHelpers/🟦️.tsx` (beside
`browserActorDispatchUiScopeV1`, which it is deliberately the counterpart of):

```ts
export function typedOperationCompletionRefreshV1(completion: {
  readonly uiScope: UiDirtyScope | undefined;
  readonly historyPatch: unknown;
  readonly requestedEffects: readonly unknown[];
}): UiDirtyScope | null {
  const scope = resolveUiDirtyScope(completion.uiScope);
  if (scope.kind !== "none") return scope;
  if (completion.historyPatch !== undefined) return { kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] };
  return completion.requestedEffects.length > 0 ? scope : null;
}
```

- the mutation's own scope, verbatim — never a guess keyed on a verb name;
- an absent scope still means `full` (an older program, or a response built without one);
- a history patch with an EMPTY scope still owes a refresh, and owes it to exactly one surface: the
  reserved history body (wave B21 put the patch on the terminal completion, so this is the case the
  assignment's law names);
- everything else is bookkeeping and owes **no pass at all** (`null`).

`🏛️ShellHost/🟦️.tsx:5339-5346` applies it (`if (refresh === null) return;`) and logs the decision in the
existing `[DEBUG] completion apply` line as `"refresh"`. New TS mirror
`FRAMEWORK_HISTORY_BODY_KEY = "framework.body.history"` in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:332`,
next to the three `FRAMEWORK_PANEL_TAB_HISTORY_*` mirrors it belongs with (the Rust original is
`🖱️ui/…/🧊️wgpu/🧩️component.rs:724`).

### 2.4 Law

`🧪️tests/🔬️engine-contract/🟦️.ts` → *"refreshes a typed operation on its completion's own scope and pays
nothing for a completion that dirtied nothing"*: an admission reply with `mutationCount: 0` answers
`{kind:"none"}`; the completion that follows with the mutation's partial scope yields exactly that scope;
a completion with `{kind:"none"}` and a history patch yields the single history body; a `{kind:"none"}`
completion with no patch and no effects yields `null`; one with an effect yields `{kind:"none"}` so the
effect pass still runs and nothing is re-taken.

```
✓ |@semio-tech/framework-renderer-react| ../../../../🧪️tests/🔬️engine-contract/🟦️.ts > shell option locks (SEMIO_LOCKED_*) > refreshes a typed operation on its completion's own scope and pays nothing for a completion that dirtied nothing 2ms
 Test Files  1 passed | 23 skipped (24)
      Tests  1 passed | 900 skipped (901)
```

Red with the rule reduced to the old `return resolveUiDirtyScope(completion.uiScope)`:

```
AssertionError: expected { kind: 'none' } to deeply equal { kind: 'partial', …(1) }
      Tests  1 failed | 900 skipped (901)
```

### 2.5 Host-live before / after, same page, same idle window

| | completions / 10 s idle | of which empty | `applyHostEffects` passes they caused |
| --- | --- | --- | --- |
| before | 18 | 12 | 2 per completion |
| after | 20 | 10 (`"refresh":null`) | 1 per completion, **0 for the empty ones** |

```
after:  { completions: 20, noneCompletions: 10, refreshes: 30, refreshesNoneScope: 20 }
        completion apply {"operation":2688,"scope":{"kind":"none"},"refresh":null,"historyPatch":false,…}
        completion apply {"operation":2624,"scope":{…"kind":"partial",…"tools":true,"windowBodies":["puzzle3d.play.composite"]},"refresh":{…same…}}
```

and the mutation lane is unregressed on the same build:

```
samples: [{"ms":866,"t":"…|Hide|Lock"},{"ms":1860,"t":"…|Show|Lock"}]
mutationCompletion: completion apply {"operation":12480,"scope":{…"measures":true,
  "panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history","puzzle.3d.play.kinds"],…}}
```

The 20 remaining `{"kind":"none"}` refresh lines per idle 10 s are **not** this path — one still follows
each completion from the drain poll's own invocation response (`🏛️ShellHost/🟦️.tsx:5203`,
`resolveUiDirtyScope(response.uiScope)`); `refreshUi` returns on its first line for a `none` scope
(`:4288`), so each costs a log plus an identity-preserving `SET_SESSION`. Named, measured, not touched:
that is the generic action-response path, and moving it needs its own wave.

### 2.6 The band-aid this does NOT retire, and what it would take

`WINDOW_CONFIG_RAIL_ACTION_IDS` (`🛠️ShellHelpers/🟦️.tsx:4383-4407`) is a hard-coded list of 23
app-specific action ids inside the domain-neutral renderer, used by
`browserActorWindowConfigDispatchUiScopeV1` to fake `{kind:"full"}` when a browser-actor admission reports
`mutationCount: 0`. It is exactly the "guess keyed on a verb name" the rule above replaces — but it cannot
be deleted yet: the browser-actor route publishes **no** `OperationCompleted` frame at all (B9 §4), so on
that route there is no completion to read a scope from. Retiring it needs the browser-actor handoff to
carry either the guest's `UiDirtyScope` or a completion of its own; until then, deleting the list would
silently re-break B12's window-option rail. Flagged here rather than half-done.

---

## 3 Files

Product (host, vite-live now):

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🟦️.tsx`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

Product (guest, rides wasm #50):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`

Laws:

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`

Probe: **not touched** (`🔍️browser-probe.ts` is off-limits to this wave).

## 4 Verification (all foreground)

| command | result |
| --- | --- |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile … in 5.46s`, 0 errors |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- catalogue --test-threads=1` | `ok. 5 passed; 0 failed; … 715 filtered out` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/…/⚛️react/vitest.config.ts` | `Test Files 4 failed \| 18 passed (22)`, `Tests 3 failed \| 711 passed (714)` — **none new**: `bun:sqlite` cannot be bundled (`.storybook/🧭️scope-resolution`), the React package self-alias path, the `UIDialog` nested kind-picker `vi.fn()` + 5 s timeout, and a `[data-slot="dialog-box"]` selector assertion. None mentions navbar/footer/chrome geometry; this wave's new law is among the 711. |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/📺️renderer/…/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 21 passed (24)`, `Tests 9 failed \| 892 passed (901)` — **the same nine B4 and B26 recorded**, byte for byte: `buildNoteShellCommandAction`, the six `🧩️package-integration` wgpu-worker rows, `uiRefreshSurfaceEvents` ("binds two instances of one body…") and `readAppDocumentPack()`'s extra `"ops"` field. Both of this wave's new laws are among the 892. |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json` | 1086 pre-existing errors in peer-owned files (a `Command`/`ConformanceStep` generics refactor in `🧪️docklayoutstore`, `ImportMeta.dir`, `InteractionState`, …); **zero at any line this wave touched** — no error in `🛠️ShellHelpers`, none in `🔝️Navbar`/`🔚️Footer`, none at `🏛️ShellHost` 446-449 / 5319-5352, none at `🔬️engine-contract` 8313-8336 or `🛂️manifest:332`. |
| probe `--only=catalogue-panel --port=6013` | **not run as a verdict**: `catalogue-add-object-kind` needs the guest fold fix, which rides wasm #50, and this wave runs no wasm build. §1.2 is the live substitute and is stronger (a real hit-tested press adding a real object). |
| probe `--only=outliner-rows --port=6013` | see §5 |

## 5 Probe lane

Both lanes run on a free port (`pgrep -f 'bun .*browser-pro[b]e'` → 0) against `:6013` / wasm #49 with
this wave's host changes vite-live. Outputs in `🗑️generated/b27-outliner-rows.txt` and
`🗑️generated/b27-catalogue-panel.txt`.

### 5.1 `--only=outliner-rows` — `outliner-hide-applies` **PASS** (was FAIL in B26)

```
[13.6s] verdict outliner-hide-control-present PASS
[22.8s] outliner hide clicked={"tag":"button","slot":"action","text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"}
        worldHiddenBefore=[{"surface":"window:puzzle3d-main-top","hidden":[]},{"surface":"window:puzzle3d-main-perspective","hidden":[]}]
        worldHiddenAfter =[{"surface":"window:puzzle3d-main-top","hidden":["seed-left-001"]},{"surface":"window:puzzle3d-main-perspective","hidden":["seed-left-001"]}]
[22.8s] verdict outliner-hide-applies PASS
[26.0s] verdict outliner-show-restores FAIL restored=false afterShowHead=[…"Hexagonal Cut Concrete Forest Left Show Lock"…]
[30.3s] battery PASS=7 FAIL=1 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

The panel body is re-taken: the row's label reads `Show` after the press, which is the `hide_lock_actions`
flip B26 could only see natively. **B26 §6's "the artifact panel body is never re-taken" is closed.**

### 5.2 `outliner-show-restores` is a probe-timing artifact, not a product defect

The step presses `Show` and samples ONCE after a fixed `page.waitForTimeout(3000)`
(`🔍️browser-probe.ts:2610-2611`), while the Hide step right above it polls 8 × 1 s
(`:2587-2590`) — and in this very run the Hide needed **9.2 s** (clicked at 13.6 s, settled at 22.8 s)
under the machine's load. Three seconds is not enough for the same round trip.

Measured live on the same build, pressing the same row's `Show` with a 100 ms sampler, recording BOTH the
label and the world scale:

```
clicked: "Show"   reachable: true
[ { "ms": 864, "k": "Hexagonal Cut Concrete Forest Left|Hide|Lock [[1,1,1]]" } ]
```

**864 ms: the label is back to `Hide` and the world scale is back to `[1,1,1]`** — the restore lands, in
the panel and in the world, in both directions. The guest's own declaration was never the suspect the
step's note guesses at: `hide_lock_actions` emits `flag_args(entity, id, "hidden", !hidden)`
(`📌️panels/🗿️artifact/🦀️.rs:140`), i.e. the inverse of the row's CURRENT state, not a hardcoded `true`,
and `📌️panels/🗿️artifact/🦀️.rs:127-131`'s own docstring says so. **This wave changed nothing for it and
recommends the step poll its restore the way its own Hide half already does** — the probe is not this
wave's to edit.

### 5.3 `--only=catalogue-panel` — still red, exactly as predicted, and the reading proves §1.3

```
[9.4s]  verdict catalogue-kind-rows-present PASS
[9.4s]  catalogue add hit-test={"label":"span|tree-label","top":"div|?|?","covered":true,"activatableRow":null,
         "topChain":["div#-[-]{p-single flex gap-single items-center min-w-0 h-full}",
                     "nav#ui.navbar[navbar]{relative h-large z-base bg-transparent}", …]}
[17.6s] verdict catalogue-add-object-kind FAIL before=1 after=1 ids=["seed-left-001"]
[21.2s] verdict catalogue-drag-drop PASS
[25.4s] battery PASS=7 FAIL=2 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

Note the chain now reads `z-base` — this wave's host change IS live in the probe's own page — and the
hit-test is **byte-identical** to B26's. That is the cleanest possible proof that the navbar's stacking
was never the cause: the reading did not move when the stacking claim was corrected, because the reading
is `elementFromPoint(0, 0)` over a folded row. It goes green with the guest fold fix on wasm #50, and
`catalogue-drag-drop` passing beside it is the same command reached by the route that bypasses layout.

## 6 Handover

1. **Re-run `--only=catalogue-panel --port=6013` on wasm #50** — `catalogue-add-object-kind` and
   `catalogue-add-selects-new-object` are the two verdicts the guest fold fix turns green; the host and
   dispatch halves are already proven (§1.2, §1.3, B26 §5's three green measurements).
2. **`outliner-show-restores` needs a probe change, not a product change** (§5.2): poll the restore like
   the Hide half. Its current note ("flag_args hardcodes value:true") is false and should go with it.
3. **`WINDOW_CONFIG_RAIL_ACTION_IDS` is still a domain-specific name list in domain-neutral framework
   code** (§2.6). Retiring it needs the browser-actor handoff to carry a scope or publish a completion.
4. **The drain poll's own invocation response still runs one `{kind:"none"}` host-effect pass per
   completion** (§2.5) — 20 per idle 10 s, each a log plus an identity-preserving `SET_SESSION`.
