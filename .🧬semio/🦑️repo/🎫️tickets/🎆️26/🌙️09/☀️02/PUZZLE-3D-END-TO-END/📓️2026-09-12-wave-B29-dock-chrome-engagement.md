# Wave B29 — the dock tab bar stopped covering the window chrome, and the typed engagement line reaches the guest

Implementation pass, 2026-09-12 01:0x–04:5x CEST, against the live `:6013` React serve (wasm **#50**,
vite-live host). Every reading below is real output from this pass. No git write, ticket not opened or
closed, `🗑️generated` written to and never deleted, no `[DEBUG] b29` tap left anywhere
(`rg -n "DEBUG\] b29"` over `🧰️framework`, `✏️s`, `.🧬semio` → no match), every command foreground except
one lane run the harness backgrounded at its 600 s cap (§4.3). Nothing was killed by this wave.

**Headline** — both product defects are root-caused to a named `file:line` and fixed, and both are proven
live on the probe's own page:

| verdict | #49 (B28) | #50 (this pass) |
| --- | --- | --- |
| pane toggle hit-test | `covered:true`, top = `BUTTON#mode-dock-tab-1-puzzle3d-main-perspective` | **`covered:false`**, top = the toggle's own `SPAN[inline-label]` |
| `export-only` | FAIL `download=none` (and only ever green through a synthetic `element.click()`) | **PASS** — real pointer press, synthetic fallback deleted |
| `engagement-input-present` | PASS only via the synthetic fallback | **PASS** by a real pointer press, `waitedMs=1` |
| `engagement-brush-verb` | FAIL `activeUtility=select` | **PASS** `activeUtility=brush waitedMs=21264` |
| `engagement-fill-verb` | FAIL `#tool.fill aria-pressed=null` | **PASS** `#tool.fill aria-pressed=true waitedMs=2479` |
| `engagement-placeholder-has-no-dead-verbs` | FAIL (predicate was inverted) | **PASS** `dead=[]` |
| `focus-selection` | would have been FAIL at B28's 8 s budget | **PASS**, camera moved at **waitedMs=20172** |

`:6013` wedged mid-pass and all seven named mutation lanes were then measured after it recovered on its
own (§4.4/§4.5) — including B28's unrun gumball axis scan, which is **green**: `gumball-handle-enter PASS`,
`gumball grabbed tried=["1@540,439=false","0.8@527,433=false","0.6@514,426=true"]`. Several reds are newly
VISIBLE rather than newly broken and are named down to their cause (§2.5, §3.4, §4.5).

---

## 1 Defect 1 — the mode dock's tab bar covered every window pane toggle

### 1.1 Root cause

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css:7014-7020` (at HEAD):

```css
.window-silhouette-content-plane:has(
  [data-window-content-layout="edgeless"],
  [data-slot="window-dead-line-scroll"]
) { padding-block-start: 0; padding-block-end: 0; }
```

`.window-silhouette-content-plane` (`:7007-7012`) pulls the plane UP by
`--window-silhouette-top-clearance` and pushes its in-flow content back DOWN by the same padding — the
margin keeps the outer intrinsic size, the padding keeps content below the chrome. The rule above zeroes
**only the padding**, and its `:has()` is unscoped, so it matches through any descendant at any depth.

The mode dock's stack body IS such a plane, and the window inside it carries a
`ChromeAwareWindowScrollSurface` (`data-slot="window-dead-line-scroll"`, react-package `:7962`) four
levels down. Live, before the fix (my own browser, 1440×900, same geometry the probe measures):

```
mode-dock-stack (path 1)   y=32  h=836
mode-dock-tabbar           y=32  h=22.39        ← the dock's own tab bar band
mode-dock-stack-body       y=32  h=836   margin-block-start:-22.3906px  padding-block-start:0px
  └ mode-dock-panel-1      y=32
    └ #puzzle3d-main-perspective[window] y=32   ← hoisted INTO the tab bar band
      └ …engagement.toggle y=35  75x22
deadline=1  edgeless=0  hasMatch=true
depthPath: mode-dock-panel-1 > window > window-body > pane-host-root > #framework.window.puzzle3dMainPerspective[window-dead-line-scroll]
elementsFromPoint(toggle centre) → ["SPAN#-[inline-label]","DIV#-[-]","BUTTON#mode-dock-tab-1-puzzle3d-main-perspective[-]","DIV#-[mode-dock-tab]", …]
```

So the negative margin stood, the compensating padding was gone, and the whole `Window` — chrome rows and
all — was lifted 22.39 px into the dock's cap band. The cap row is `z-[2]` and the body `z-[1]`
(`⚛️react/🟦️.tsx` `WindowChrome`), so the dock tab won both the paint and the hit-test. This is B27's band
arithmetic in a second place: not the navbar's stacking (which B27 correctly cleared), but the plane's own
clearance being dropped underneath it.

**Second, independent obstruction, found only once the first was fixed.** With the padding restored the
toggle moved to `y=58` — out of the tab bar band — and the hit-test then answered
`BUTTON#world3d-frame-instances-puzzle3d-main-perspective`. That is
`🌐️World3dHost/🟦️.tsx:6269` (at HEAD): `<div className="pointer-events-auto absolute left-3 top-3 z-40" data-slot="world-frame-instances">`
wrapping a button whose label was the hardcoded English string `Frame`. It sat at `left-3 top-3` — inside
the window's own chrome control row — at `z-40`, above the pane overlay's `z-20`/`z-30`, so it swallowed
the toggle's pointer events outright. Its sibling `computing` status (`:6281`, `right-3 top-3`) painted
over the top-right **window options** toggle the same way (that one is `pointer-events-none`, so it
covered the control visually without eating the press).

### 1.2 Fix

- **`🖌️ui.css:7014-7025`** — the opt-out is scoped: a plane that hosts a whole window keeps its chrome
  clearance, because the chrome that clearance protects belongs to that window, not to this plane's own
  content. The nested window's own planes still opt out for their own content.

  ```css
  .window-silhouette-content-plane:has(
      [data-window-content-layout="edgeless"],
      [data-slot="window-dead-line-scroll"]
    ):not(:has([data-slot="window"])) { padding-block-start: 0; padding-block-end: 0; }
  ```

- **`⚛️react/🟦️.tsx`** (react package, after the `🚧️WindowContentDeadLine` re-exports) — one exported rule
  for every top-corner offset a window's CONTENT needs, so it is stated once instead of guessed per call
  site:

  ```ts
  export const windowChromeClearedTopOffset = `calc(var(${windowChromeScrollClearanceVar}, calc(var(--size-medium) + var(--spacing-single))) + var(--spacing-single))`;
  ```

  `--window-chrome-scroll-clearance` is the live clearance `Window` already publishes on `window-body`
  from the mounted engagement/search/measures overlays (`measureWindowChromeScrollClearancePx`); live it
  reads `26px`.

- **`🌐️World3dHost/🟦️.tsx:6269-6289`** — the two top-corner scene overlays became one top-**right** rail
  at `windowChromeClearedTopOffset`, holding the frame button and the computing status; the top-left band
  is left to the folded engagement's own quick-action rail (`Add Object…`, live at `y=83`), which the
  button would otherwise collide with. Raw `px-2 py-1`/`gap-2` became `px-single py-half`/`gap-single`.
- **`🌐️World3dHost/🟦️.tsx`** — the label is localized: `shellLabel("ui.host.frameVisible")`, with
  `frameVisible` authored in BOTH locales (`⚛️react/🟦️.tsx` `ui.host`, EN `Frame visible` / DE
  `Sichtbares einpassen`) and declared on `UiTranslationSchema` (`🧱️elements/📚️I18n/🟦️.tsx:419`). It was
  a hardcoded English literal, which the no-default-language rule forbids outright.
- **`🪟️Window/🟦️.tsx:366`** — the engagement quick-action rail now consumes the same exported offset
  instead of its own `calc(var(--size-medium) + (var(--spacing-single) * 2))` literal.
- **`🔍️browser-probe.ts`** — B28's synthetic `element.click()` fallback in `unfoldWindowPane` is
  **deleted**. A real pointer press only; the returned record still carries `paneToggleObstruction`'s
  chain, so a covered toggle leaves the pane folded and the downstream verdict fails naming whatever
  covers it. A green can no longer be reached through an unreachable control.

### 1.3 Law + output

**`🖌️ui.css` contract** — `🎨️styling/🧪️tests/🧩️suite/🟦️.ts`, the existing
*"expands the clipped content plane without changing document or auto-size clearances"* extended, plus a
new *"never lets a content plane that hosts a whole window drop its chrome clearance"* which asserts the
guard is present, ordered after the dead-line clause, and that the block still zeroes both paddings.

```
bun test "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts" -t "content plane"
 2 pass / 40 filtered out / 0 fail / 5 expect() calls
```

Red with HEAD's selector restored (real coverage, not a tautology):

```
error: expect(received).toContain(expected)
Expected to contain: ":not(:has([data-slot=\"window\"]))"
Received: ".window-silhouette-content-plane:has( [data-window-content-layout=\"edgeless\"], [data-slot=\"window-dead-line-scroll\"] )"
 0 pass / 2 fail
```

**B27's layout law extended** — `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`, new sibling
*"keeps the mode dock tab bar band clear of the hosted window's own chrome control row"*. It states the
band arithmetic from the same `STYLING_METRICS.chrome` tokens B27 used — dock cap band `[0, control]`,
plane clearance `= control`, window chrome row cleared `[clearance+pad, clearance+pad+control]` vs hoisted
`[pad, pad+control]` — and asserts the cleared row starts at or below the band's end while the hoisted one
starts inside it. Then it renders a two-stack `Mode` and pins the exact DOM shape the CSS guard addresses:
the dock's tab bar is in `mode-dock-tabbar`, the body carries `window-silhouette-content-plane`, and that
plane HOSTS a `[data-slot="window"]` with a mounted `window-search-overlay`.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts -t "keeps the mode dock tab bar band clear"
 Test Files  1 failed | 1 passed | 20 skipped (22)
      Tests  1 passed | 715 skipped (716)
```

(the failed FILE is the pre-existing `bun:sqlite` storybook-scope bundling error B27 recorded.)

### 1.4 Live proof — the hit-test the assignment asked for

Same page, after both fixes, `1440×900`:

```
plane  { marginBlockStart: "-22.3906px", paddingBlockStart: "22.3906px", rect: {x:483,y:32,w:954,h:836} }
rail   [{rect:{x:398,y:84,w:75,h:16}, text:"Frame visible"}, {rect:{x:1355,y:84,w:75,h:16}, text:"Frame visible"}]
puzzle3dMainTop         rect {x:10 ,y:58,w:75,h:22}  covered:false  top "SPAN#-[inline-label]"
puzzle3dMainPerspective rect {x:490,y:58,w:75,h:22}  covered:false  top "SPAN#-[inline-label]"
  chain ["SPAN#-[inline-label]","BUTTON#framework.window.…engagement.toggle[window-pane-chrome-toggle]",
         "DIV#-[window-chrome-chip-cap]","DIV#-[window-engagement-zone]","DIV#…engagement[window-engagement-overlay]"]
```

and through the probe's own helper (`--only=export-import,engagement-bar`, no synthetic fallback left):

```
engagement unfold={"unfolded":true,"obstruction":{"present":true,"covered":false,
  "chain":["SPAN#-[inline-label]","BUTTON#framework.window.puzzle3dMainPerspective.engagement.toggle[…]", …]},"waitedMs":1}
engagement pane={"rootPresent":true,"searchPresent":true,
  "rootText":"Actions | Collapse | 1 Objects · 0 Attractions | AddObject | ACTIONS | Set Active Example | Import | Translate Selection | …",
  "fields":["?|file|","puzzle3d-engagement|input|brush, fill <n>, zoom, clear, pick, rectangle, lasso"]}
verdict engagement-input-present PASS
verdict export-only PASS  download=puzzle-3d.json dest=…/🗑️generated/probe-2026-09-12T01-22-20-export.json
verdict import-same-file-idempotent PASS before=2 after=2
```

`export-only` was `download=none` on #49 and is green here **by a real pointer press**, which is the
verdict B28 could not restore. Mobile: the fix is size-independent (the CSS guard and the offset are both
token-derived, and the `Pane`/rail code paths already branch on `useUiMobile()` for width only), so no
desktop-only escape hatch was introduced.

---

## 2 Defect 2 — the typed engagement line and the Fill tool

### 2.1 Root cause A — the controlled field could not hold a keystroke

`⚛️react/🟦️.tsx` `Search` (`:10253-10255` at HEAD):

```ts
const isControlledInput = !!input?.onChange;
const draft = normalizeEngagementActionText(isControlledInput ? (input?.value ?? "") : uncontrolledDraft);
```

The line shown is the program's published `input.value`, nothing else. The puzzle3d guest stores the text
(`🎮️commands/⌨️engagement-input/🦀️.rs` writes `ctx.scene.runtime.engagement_input`) but the host's
`value` prop stayed `""` across the whole exchange, so every keystroke dispatched `engagementInput` and
the field snapped straight back to `""` — and `engagementSubmit` therefore carried an **empty** line.
That is why `engagement_submit`'s `match` fell through to `_ => {}` and `activeUtility` stayed `select`.

### 2.2 Root cause B — a program-armed tool mounts no leaf tab

`🏛️ShellHost/🟦️.tsx:9370-9377` (at HEAD): the single owner of "the selected `tool.<id>` leaf IS the
active tool" starts with

```ts
const path = mobile ? mobilePanelPath : panelActivePaths[toolAnchor];
if (branchPath.some((segment, index) => path[index] !== segment)) return;   // ← skipped wholesale
```

Typing `fill 3` DOES arm the tool guest-side — `engagement_submit` sets `scene.active_utility = "fill"`
and `✏️editor/🦀️.rs:3441` emits `Effect::SetActiveTool { fill }` — but with the footer Tool category not
the active root the reconciliation returned before selecting anything, so `#tool.fill` never mounted, the
tool's options were unreachable, and nothing on screen said it had armed. B28's reading `tools=[]` was
that, not a dead verb.

### 2.3 Fix

- **`⚛️react/🟦️.tsx`** — new pure rule beside the other search helpers:

  ```ts
  export interface SearchLineEdit { readonly text: string; readonly base: string }
  export function searchControlledLineV1(published: string, edit: SearchLineEdit | null): string {
    if (!edit) return published;
    return published === edit.base ? edit.text : published;
  }
  ```

  `Search` keeps one local edit (`{ text, base: publishedLine-at-edit-start }`); the edit LEADS while the
  published value stands still, and the published value wins back the moment it moves away from the base —
  which is exactly the case where the program authored the line itself. Confirm and abort **release** the
  edit (`releaseDraft`) rather than writing `""` locally: a program that keeps a standing value in that
  field keeps it, and puzzle3d's own `engagement_submit` (which sets `engagement_input = String::new()`)
  clears it. Releasing rather than clearing is what keeps the existing law
  *"Search input PascalCases action text and space confirms like enter"* green — the first version cleared
  locally and turned that law red, which is how the distinction was found.
- **`🛠️ShellHelpers/🟦️.tsx:4342-4355`** — new pure rule `programArmedToolRevealV1(previousToolId,
  activeToolId, toolCategoryActive)`: a MOVE to a real tool while the category is not the active root must
  reveal its own leaf; an idle category, a disarm, and a re-render on the same tool owe nothing.
- **`🏛️ShellHost/🟦️.tsx:9371-9388`** — applies it before the old early return, selecting the tool's leaf
  path and making that anchor visible (desktop and mobile branches both), and keeps `revealedToolIdRef` in
  sync on the in-category path so browsing away and back does not re-reveal. The decision the assignment
  asked for is therefore **the verb opens the category**, stated generically: any program-armed tool
  reveals itself, not just `fill`.

The guest needed no change: `engagement_submit`'s `fill` arm already emits `SetActiveTool` and its `brush`
arm already claims the window utility, and `engagementSubmit` already publishes
`WindowConfig + WindowTransient + Interaction` (`✏️editor/🦀️.rs:7140`).

### 2.4 Laws + output

**vitest, the draft** — `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`,
*"a controlled search line keeps the typed draft while the program's published value stands still"*: the
pure rule's four cases, then a rendered `Search` whose program never republishes (`value: ""` on every
rerender, exactly puzzle3d), typing `brush`, asserting the field holds `Brush` and `onChange` saw
`["Brush"]`, that Enter submits `"Brush"` (not `""`), that the confirm releases the line back to the
program's `""`, and that a later published `"fill 3"` wins the field back as `Fill3`.

```
 Test Files  1 failed | 1 passed | 20 skipped (22)
      Tests  1 passed | 715 skipped (716)
```

> Worth recording: `normalizeEngagementActionText` PascalCases and strips separators, so the line the user
> types as `fill 5` reaches the guest as `Fill5`. The guest's `strip_engagement_prefix`
> (`🔌️plugin/🦀️.rs:35565`) compares alphanumerics case-insensitively and keeps `5` as the remainder, so
> that is correct end to end — and it is what the live log now shows (`engagement typed="Fill5" for="fill 5"`).

**vitest, the reveal** — `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`,
*"a tool the program arms reveals its own leaf tab, and a tool the user browsed away from does not"*.

```
 Test Files  1 passed | 25 skipped (26)
      Tests  1 passed | 917 skipped (918)
```

Red with the rule reduced to `return false`:

```
+ false
 ❯ …/🔬️engine-contract/🟦️.ts:7922
      Tests  1 failed | 917 skipped (918)
```

**cargo, the verb → tool** — the guest law already exists and is green on current source, which is what
makes the host half the whole remaining defect:

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib engagement -- --test-threads=1
test editor::puzzle3d::component::tests::engagement_exposes_no_utility_switch_options ... ok
test editor::puzzle3d::component::tests::every_advertised_engagement_verb_is_implemented ... FAILED
test editor::puzzle3d::component::tests::fill_and_brush_params_are_tagged_utility_options_not_engagement_controls ... ok
test editor::puzzle3d::component::tests::the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count ... ok
test editor::puzzle3d::component::tests::transform_engagement_does_not_block_background_deselect ... ok
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 715 filtered out; finished in 0.37s
```

`every_advertised_engagement_verb_is_implemented` fails at
`✏️editor/🧪️tests/🔬️unit/🦀️.rs:4825`, `assertion left == right failed: typing clear must empty the
framework-owned selection / left: 1 / right: 0`. **Not this wave's**: this wave changed zero `.rs` files
(`git status --porcelain | rg "🧩️puzzle.*\.rs"` → empty), so the red is committed HEAD — the `clear`
arm's `ctx.clear_selection()` / `Emit.interaction_writes` path, touched by tonight's commits
`de93f84300`/`989582baab`. Named here because a reader of this report will run the same command.

### 2.5 Live proof

`--only=engagement-bar --port=6013`, wasm #50, two consecutive runs (`probe-2026-09-12T01-22-20.ndjson`
and `…01-24-55.ndjson`), identical but for the abort step:

```
engagement typed="Brush" for="brush"     → PASS engagement-brush-verb  activeUtility=brush  waitedMs=21264
engagement typed="Clear" for="clear"     → PASS engagement-clear-is-a-noop before/after count=1 waitedMs=10398
engagement typed="Fill5" for="fill 5"    → PASS engagement-fill-verb   #tool.fill aria-pressed=true waitedMs=2479
PASS engagement-input-present            toggle=1 input=1 placeholder=brush, fill <n>, zoom, clear, pick, rectangle, lasso
PASS engagement-placeholder-has-no-dead-verbs  advertised=["brush","fill <n>","zoom","clear","pick","rectangle","lasso"] dead=[]
FAIL engagement-abort                    rearmed=true activeUtility=brush waitedMs=15511
```

`engagement typed=` is the field's own value read **before** the Enter — the direct proof that the draft
now survives, since B28 could only read `value=""` after the fact.

**`engagement-abort` is newly visible, not newly broken.** With brush re-armed immediately before the
Escape (`rearmed=true activeUtility=brush waitedMs=38`), the guest tap at abort time reads

```
[DEBUG] puzzle3d.utility.publish action=engagementAbort window=Some("puzzle3d-main-perspective") utility=fill map_hit=false
```

`engagement_abort` (`🎮️commands/🛑️engagement-abort/🦀️.rs:9-11`) **returns untouched** while
`puzzle3d_fill_tool_active(ctx.config)` — by design, "leaving fill is exclusively a host
`setActiveTool \"\"`" — and `config.active_tool_id` is still `fill` from the `fill 5` above because the
host's utility/tool mutual exclusion has not round-tripped into the guest's runtime copy yet. So Escape
after a fill can never disarm. The hop was unreachable until the typed line started arriving, so this is
the first pass that can see it; it belongs to the utility/tool mutual-exclusion lane and rides a wasm
build, so this wave names it and does not touch the guest.

---

## 3 Item 3 — probe mutation budgets

### 3.1 Fix

New shared helper in `🔍️browser-probe.ts` (beside `countSafe`):

```ts
const settleFor = async <T>(read: () => Promise<T>, settled: (value: T) => boolean, budgetMs = 30000, stepMs = 500)
  : Promise<{ value: T; waitedMs: number; ok: boolean }>
```

Every mutation/census verdict in the named lanes now polls through it up to 30 s and carries `waitedMs` in
its note; **no verdict or step name changed.** Converted:

| lane | verdict | was | now |
| --- | --- | --- | --- |
| selection-keybindings | `duplicate-selection`, `delete-selection` | 3 s + 3 s retry | 15 s + 15 s retry, census predicate |
| selection-keybindings | `focus-selection` | `cameraSettled(…, 8000)` | 30 s |
| gumball-drag | `gumball-scene-delta` | fixed 1.5 s | 30 s pose predicate |
| relocate | `relocate-pose-delta` | fixed 3.5 s | 30 s pose predicate |
| volume-brush | `volume-brush-add-target-volume`, `…-voxel-dims` | fixed 3 s / 1.8 s | 30 s census / measure predicate |
| outliner-rows | `outliner-hide-applies` | 8 × 1 s | 30 s |
| outliner-rows | `outliner-show-restores` | fixed 3 s | 30 s (B27 §5.2's handover item; its false `flag_args hardcodes value:true` note is gone) |
| catalogue-panel | `catalogue-add-object-kind`, `…-selects-new-object`, `catalogue-drag-drop` | 8 × 1 s / none / 3.5 s | 30 s / 15 s / 30 s |
| locked-refusal | `locked-refusal-notice` | fixed 1.5 s | 30 s notice predicate |
| context-menu-rows | `context-menu-zoom-moves-camera` | fixed 3 s | 30 s camera predicate |
| engagement-bar | `engagement-brush-verb`, `…-fill-verb`, `…-abort` | fixed 3 s / 2 s | 30 s / 30 s / 15 s |
| engagement-bar | `engagement-clear-is-a-noop` | fixed 3 s | 10 s spent proving the census did NOT move |

`cameraSettled` is expressed in terms of `settleFor`, defaults to 30 s and returns
`{ camera, moved, waitedMs }`; its four callers were updated. `nudgeMeasure` returns `waitedMs` too.
B28's rewritten gumball axis scan (fractions `1, 0.8, 0.6, 0.4` of origin→tip, every read strictly after
`mouse.up()`) is present and unchanged — it did not get to run, see §4.4.

`engagement-placeholder-has-no-dead-verbs` had an **inverted predicate** (`!/clear|rectangle|lasso/`). The
cargo law `every_advertised_engagement_verb_is_implemented` drives all three verbs end to end and the
placeholder is DERIVED from `PUZZLE3D_ENGAGEMENT_VERBS`, so an advertised verb with no arm cannot exist.
Name kept; the predicate now catches the drift it can actually catch — a placeholder advertising a verb
the guest's list does not carry (`dead=[]` live).

### 3.2 What the budgets bought, measured

```
PASS engagement-brush-verb  activeUtility=brush  waitedMs=21264   (21029 in the prior run)
PASS focus-selection        camera settle puzzle3d-main-perspective moved=true waitedMs=20172
PASS duplicate-selection    before=1 after=2 waitedMs=6937
PASS engagement-fill-verb   #tool.fill aria-pressed=true waitedMs=2479
FAIL delete-selection       before=2 after=2 waitedMs=30299
```

Three of those greens are **longer than the budget they replaced** (21 s and 20 s against 3 s and 8 s),
so they were unmeasurable before, and `delete-selection` is now a red proven at 30 s rather than at 3.

### 3.3 Verdicts run

```
--only=boot --port=6013
[22.7s] verdict boot PASS
[22.7s] verdict battery-hard-faults PASS
[22.7s] verdict battery-faults PASS
[22.7s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=6

--only=export-import,engagement-bar --port=6013   (probe-2026-09-12T01-22-20)
PASS boot · PASS engagement-input-present · PASS engagement-placeholder-has-no-dead-verbs
PASS engagement-brush-verb waitedMs=21029 · PASS engagement-clear-is-a-noop waitedMs=10056
PASS engagement-fill-verb waitedMs=2467 · FAIL engagement-abort waitedMs=10055
PASS guest-alive-mutate · PASS export-only · FAIL export-names-the-example (example=UNRESOLVED)
PASS import-same-file-idempotent · FAIL import-distinct · FAIL import-distinct-records-history
PASS guest-alive-replace · PASS battery-hard-faults · PASS battery-faults

--only=engagement-bar --port=6013                 (probe-2026-09-12T01-24-55, after the abort step's re-arm)
… identical, plus: engagement re-armed brush=true activeUtility=brush waitedMs=38
FAIL engagement-abort rearmed=true activeUtility=brush waitedMs=15511

--only=selection-keybindings --port=6013          (probe-2026-09-12T01-26-57)
PASS boot
PASS duplicate-selection        before=1 after=2 waitedMs=6937
PASS duplicate-reselects-clone
PASS focus-selection            camera moved after waitedMs=20172
FAIL delete-selection           before=2 after=2 waitedMs=30299
PASS guest-alive-mutate · PASS battery-hard-faults (hard=0 collateral=0 distinct=0) · PASS battery-faults (raw=0)
```

Reds that are not this wave's, with what the run says about each:

- `export-names-the-example` — `example=UNRESOLVED`: `#playground.navbar.fixture` named no example, so the
  step could not form an expectation. A probe precondition, not the export.
- `import-distinct` / `import-distinct-records-history` — guest-side, and the taps say where:
  `[DEBUG] puzzle3d.import.ingress args=true payload_len=9944` → `puzzle3d.import.parsed objects=2 before=2`
  → **`puzzle3d.import.apply ops=0 after_objects=2`**, then a second ingress `objects=3 before=2`. The
  payload arrives and parses; the apply emits zero operations.
- `delete-selection` — `Delete` and `Backspace` both left the census at 2 after a full 30 s.
- `engagement-abort` — §2.5.

### 3.4 The selection census was reading mode-dock tabs — fixed

`duplicate-reselects-clone` passed, and `catalogue-add-selects-new-object` failed, on this note:

```
selected=["mode-dock-tab-0-puzzle3d-main-top=Top","mode-dock-tab-1-puzzle3d-main-perspective=Perspective"]
```

`selectionState()` (`🔍️browser-probe.ts:509` at HEAD) swept
`[aria-selected="true"], [data-selected="true"], [data-state="selected"]` document-wide. Every tablist
carries that state — the mode dock's window tabs and the panel tab strips — so for a document with a
freshly added object selected in it the answer was two dock tabs: one verdict green on the wrong
elements, and one that could only ever read false.

**Fix** tab strips are excluded (`[role="tablist"]`, `mode-dock-tabs`, `mode-dock-tab`, `panel-tabs`,
`panel-tab-button`) and the world's own selection is folded in from `data-interaction-json` — the guest's
view plus the host's leftover overlay, which is the selection truth these verdicts ask about. Live after
the fix: `focus-selection … selected=["seed-left-001","seed-left-001"]` (real object ids), and
`catalogue-add-selects-new-object FAIL added=1 selected=[]` — an honest red where the dock tabs used to
stand in, i.e. `addObjectKind` adds its object and does **not** select it.

---

## 4 Verification

| command | result |
| --- | --- |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/…/⚛️react/vitest.config.ts` | `Test Files 5 failed \| 17 passed (22)`, `Tests 4 failed \| 712 passed (716)` — see §4.1 |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/📺️renderer/…/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 23 passed (26)`, `Tests 9 failed \| 909 passed (918)` — **the same nine** B4/B26/B27 recorded, byte for byte |
| `bun x tsc --noEmit -p …/🖱️ui/…/⚛️react/tsconfig.json` | 338 pre-existing errors, **zero in any file this wave owns** (§4.2) |
| `bun x tsc --noEmit -p …/📺️renderer/…/⚛️react/tsconfig.json` | 862 errors — B28's exact count — **zero at any line this wave touched** (§4.2) |
| `bun x tsc --noEmit … 🔍️browser-probe.ts` | the two pre-existing errors only: `import.meta.dir` and `mouse.click({ modifiers })`; no new ones |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib engagement -- --test-threads=1` | `4 passed; 1 failed` — the failure is committed HEAD's `clear` arm, §2.4 |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile … in 40.18s`, **0 errors**, 88 warnings (pre-existing; their presence proves expansion completed) |
| `bun 🔍️browser-probe.ts --only=boot --port=6013` | §3.3 |
| `--only=export-import,engagement-bar`, `--only=engagement-bar`, `--only=selection-keybindings` | §3.3 |
| `--only=gumball-drag`, `relocate`, `volume-brush`, `outliner-rows`, `catalogue-panel`, `locked-refusal` | all run after `:6013` recovered — §4.5 |

### 4.1 The four ui-react test failures

`package entry and self-alias resolve to the canonical React source`, the `.storybook/🧭️scope-resolution`
`bun:sqlite` bundling error, `UIDialog … nested owned kind picker`, `UIIntroduction … glass boxes` — all
four recorded by B10/B27 before this wave. The fifth failing FILE this run,
`owned Diagram force > cursorizes live controlled 20,000-node and edge setup before cooperative
projection`, is `Error: Test timed out in 5000ms` on a 20,000-node force-simulation perf test under
`load average 13–18`; it did not fail on the first full run of this pass and nothing this wave touched is
on the Diagram path. Both of this wave's new ui-react laws are among the 712.

One ui-react law DID go red from this wave and was resolved rather than worked around:
`Search input PascalCases action text and space confirms like enter` broke when the confirm cleared the
line locally (`["SetHeight", ""]`), which is what drove the release-not-clear design in §2.3. It is green
in the final state.

### 4.2 Typecheck scoping

ui-react: 338 `error TS` across 12 files, the largest being the pre-existing `TS7006`/`TS2304`/`TS2347`
families in `owned-locale-detector-retirement/🟦️.tsx` (152), `🧪️docklayoutstore/🟦️.ts` (84) and
`backbone-envelope-io/🟦️.ts` (44). **Zero at any line of `⚛️react/🟦️.tsx`, `🪟️Window/🟦️.tsx` or
`📚️I18n/🟦️.tsx`**, and zero inside either of this wave's added test ranges (3830-3900, 6155-6195 —
checked by extracting every error line number and intersecting). The four errors this wave DID introduce
(`'frameVisible' does not exist in type …`, twice per locale) were real and are fixed by declaring the key
on `UiTranslationSchema`.

renderer: 862, matching B28 exactly; intersecting every error line against this wave's edited ranges in
`🌐️World3dHost` (55-75, 6260-6300), `🛠️ShellHelpers` (4340-4360), `🏛️ShellHost` (490-500, 9365-9395) and
`🔬️engine-contract` (1880-1895, 7905-7930) returns empty.

### 4.3 One lane run was backgrounded, not detached

`--only=selection-keybindings` and `--only=gumball-drag` were launched as one foreground command; the
harness moved it to the background at its 600 s cap. It was then polled to completion rather than
abandoned, and no process was killed.

### 4.4 `:6013` wedged mid-pass, then recovered on its own

`selection-keybindings` finished at `223.9s`. The `gumball-drag` lane that followed never got past
`navigating`:

```
[0.4s] navigating
[60.4s] boot goto: TimeoutError: goto: Timeout 60000ms exceeded.
  - navigating to "http://127.0.0.1:6013/?plugin=puzzle3d", waiting until "domcontentloaded"
```

and the serve stopped answering:

```
curl --max-time 10 http://127.0.0.1:6013/?plugin=puzzle3d  →  http=000 t=10.009589
lsof -nP -iTCP:6013 -sTCP:LISTEN → bun 91282
ps: 91282 at 96.1 % CPU, 30:39 CPU time over 01:05:00 elapsed
load averages: 13.47 15.85 18.03  (peers' rustc ×2 and node ×2 resident)
```

This is exactly the condition B28 §7.1 and B10 recorded. **Per this wave's rules the serve was NOT
restarted and probing stopped**, and the wedged `gumball-drag` process was left alive rather than killed.

It then cleared on its own: the harness terminated that backgrounded command (`exit 144`, its browser
closing mid-`snapshot()` with `evaluate: Target page, context or browser has been closed`), the gate went
free, and `:6013` answered `http=200 t=0.006514` again. Nothing was killed by this wave. §4.5 is the lane
pass that followed.

### 4.5 All seven named mutation lanes, measured

Each run in a fresh page, one lane per invocation, `pgrep` gate checked before the first.

```
--only=gumball-drag                 (probe-2026-09-12T02-37-53)
PASS gumball-handle-enter    handle={"kind":"moveX","sx":540,"sy":439,"ndcZ":0.996} entered=true moveEntered=true taps=1
     gumball handle … origin={"kind":"origin","sx":474,"sy":407,"ndcZ":0.996} onscreen=4
     gumball grabbed tried=["1@540,439=false","0.8@527,433=false","0.6@514,426=true"]
FAIL gumball-scene-delta     sceneDelta=false poseLen=266 waitedMs=30246
PASS guest-alive-mutate · PASS battery-hard-faults · PASS battery-faults      battery PASS=5 FAIL=1 FAULTS=0

--only=relocate                     (probe-2026-09-12T02-39-29)
PASS relocate-arm            found=true activeUtility=worldRelocate
FAIL relocate-pose-delta     beforeLen=266 afterLen=266 instances=1 waitedMs=30007
PASS relocate-no-hard-fault  newHardFaults=0                                  battery PASS=6 FAIL=1 FAULTS=0

--only=volume-brush                 (probe-2026-09-12T02-40-41)
PASS volume-brush-arm · PASS volume-brush-target-volume-attribute (count=0)
PASS volume-brush-add-target-volume  before=0 after=1 waitedMs=6153
PASS volume-brush-voxel-dims         slider 1 → moved                         battery PASS=8 FAIL=0 FAULTS=0

--only=outliner-rows                (probe-2026-09-12T02-41-12)
PASS outliner-panel-opens · PASS outliner-hide-control-present
FAIL outliner-hide-applies   waitedMs=30501
     clicked={"tag":"button","slot":"action","text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"}
     worldHiddenBefore=[…"hidden":[]…]  worldHiddenAfter=[{"window:puzzle3d-main-top","hidden":["seed-left-001"]},
                                                          {"window:puzzle3d-main-perspective","hidden":["seed-left-001"]}]
FAIL outliner-show-restores  not reachable — the Hide row action itself never changed the row
                                                                              battery PASS=6 FAIL=2 FAULTS=0

--only=catalogue-panel              (probe-2026-09-12T02-42-18, re-run 02-44-52 after §3.4's census fix)
PASS catalogue-panel-opens · PASS catalogue-kind-rows-present (rows=1, activatable="true", rowKind="group")
PASS catalogue-add-object-kind  before=1 after=2 waitedMs=6226 (14230 on the re-run)
     ids=["seed-left-001","puzzle3d.object.9acf0e0e279673de"]  hitTest covered:false activatableRow:"true"
FAIL catalogue-add-selects-new-object  added=1 selected=[] waitedMs=15383
PASS catalogue-drag-drop        before=2 after=3                              battery PASS=8 FAIL=1 FAULTS=0

--only=locked-refusal               (probe-2026-09-12T02-43-07)
FAIL locked-flag-row         lockChrome=false
     gumball grabbed tried=["1@540,439=false","0.8@527,433=false","0.6@514,426=true"]
FAIL locked-refusal-notice   notices=[] waitedMs=30456                        battery PASS=4 FAIL=2 FAULTS=0

--only=selection-keybindings        (probe-2026-09-12T02-45-52, re-run after §3.4's census fix)
FAIL duplicate-selection     before=1 after=1 waitedMs=30546
FAIL duplicate-reselects-clone  selected=[]
PASS focus-selection         selected=["seed-left-001","seed-left-001"], camera moved (orbit precondition 17209 ms, focus 6281 ms)
PASS delete-selection        before=3 after=2 waitedMs=23695                  battery PASS=6 FAIL=2 FAULTS=0
```

Zero faults, zero hard faults, zero guest deaths across all seven.

**What the lanes say, item by item.**

- **B28's item 4 is CLOSED.** `gumball-handle-enter` is green and the diagnostic names why the old
  single-point press could never work: the projected arrow **tip** (`fraction 1`) and `0.8` both miss the
  mesh; `0.6` of the origin→tip segment grabs it. The `{kind:"origin"}` stamp B28 added to
  `WorldGumballHitStamp` is what makes that addressable.
- **B28's item 5 is gated one hop EARLIER than B28 thought.** The drag is no longer the blocker —
  `locked-refusal` shows the same `gumball grabbed … 0.6 … true` — but `locked-flag-row FAIL
  lockChrome=false`: the inspector renders no lock control, so nothing is ever locked and no refusal can
  exist. `locked-refusal-notice` is downstream of that, not of the drag.
- **`gumball-scene-delta` / `relocate-pose-delta` are a publication defect, not a dispatch one.** The
  gumball lane's own taps prove the whole chain fired:
  `[DEBUG] gumball drag entered {kind: moveX, ids: Array(1)}` →
  `[DEBUG] gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}` → history row
  `framework.history.entry.11=move-object id=seed-left-001 new-origin=13.099411116785944,0,0`. The move
  committed with a correct new origin and an undoable row, yet `data-instances-json` was byte-identical
  after 30.2 s. `chromeState().notices` carried `"Agent disconnected"` in the same reading, so the most
  likely reason the world payload never republished is that the actor dropped — but that is a hypothesis,
  not a measurement, and the two are distinguishable by arming runtime diagnostics on a re-run.
- **B27's catalogue fold fix is live and `catalogue-add-object-kind` is GREEN** — its handover item 1,
  closed: `covered:false activatableRow:"true"`, a real press on the laid-out row adds
  `puzzle3d.object.9acf0e0e279673de`. `catalogue-add-selects-new-object` is the honest red behind it
  (§3.4).
- **`outliner-hide-applies` regressed against B27 §5.1.** On #49 B27 measured the row flipping
  `Hide`→`Show` at 22.8 s. On #50 the **world lane applies** (`worldHiddenAfter` carries
  `seed-left-001` on both surfaces) and the **panel body is never re-taken** in 30.5 s, so the row still
  reads `Hide`. B27's own `typedOperationCompletionRefreshV1` is the first place to look — it returns
  `null` (no refresh pass at all) for a completion with `{kind:"none"}`, no history patch and no requested
  effects — but the probe does not arm `SEMIO_RUNTIME_DIAGNOSTICS`, so this pass did not capture the
  completion's scope and that remains a hypothesis. `outliner-show-restores` is downstream.
- **`duplicate-selection` / `delete-selection` are flaky across runs**, in both directions: 01-26-57 had
  `duplicate-selection PASS before=1 after=2 waitedMs=6937` with `delete-selection FAIL waitedMs=30299`;
  02-45-52 had the exact opposite (`duplicate FAIL waitedMs=30546`, `delete PASS before=3 after=2
  waitedMs=23695`). The lane's `select()` precondition has no verdict of its own, so "nothing was
  selected" and "the keybinding did nothing" still read the same. That is the next probe change this lane
  needs, and it is NOT a budget problem — both reds burned the full 30 s.

### 4.6 The last two re-runs overlapped the coordinator's #51 battery

`🗑️generated/battery-2026-09-12-51-6013.txt` starts at 04:44 local, and a second probe's stream
(`probe-2026-09-12T02-44-30.ndjson`, a `--battery --reload-between-groups` run carrying `reboot-mutate`
and the whole read group) interleaves with my `catalogue-panel` re-run (02-44-52) and
`selection-keybindings` re-run (02-45-52). So **two probes shared the one-tab lease for those two runs**,
and that is the likeliest source of §4.5's `duplicate`/`delete` variance. Stated rather than hidden. The
five lanes before them (02-37-53 … 02-43-07) ran alone. Probing stopped here — `pgrep` now returns two
live probes (55059, 57469), which is the coordinator's #51 battery, and it runs this wave's own
`🔍️browser-probe.ts`, so it re-measures all of the above on #51.

---

## 5 Honest gaps

1. **Two of the seven lane re-runs shared the tab with the coordinator's #51 battery** (§4.6). The five
   that ran alone are clean; the two that did not are the two whose verdicts moved between runs.
2. **Two of §4.5's root causes are hypotheses, not measurements** — the `Agent disconnected` explanation
   for `gumball-scene-delta`, and `typedOperationCompletionRefreshV1` for `outliner-hide-applies`. Both
   need a re-run with `SEMIO_RUNTIME_DIAGNOSTICS=1` armed, which the probe does not do.
3. **`engagement-abort` and the guest's `clear` arm are real reds this wave does not fix** (§2.4, §2.5).
   Both are guest-side and ride a wasm build; both are named down to the `file:line` and the log line that
   proves them.
4. **`duplicate-selection`/`delete-selection` need a selection precondition verdict**, not a bigger budget
   (§4.5) — both reds burned the full 30 s.
5. **The layout law cannot go red from the CSS alone.** jsdom performs no layout, so the extended
   ui-react law pins the band arithmetic and the DOM shape the CSS guard addresses, while the CSS text
   contract (which IS provably red at HEAD, §1.3) lives in the styling suite. The live hit-test in §1.4 is
   the third leg and the only one that measures real geometry.
6. **`import-distinct` narrowed, not fixed**: the guest's own taps put it at `import.apply ops=0` with a
   parsed payload in hand. That is a guest command lane, not this wave's.

## 6 Files

Product (host, vite-live now):

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

Laws:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

Probe (this wave owns it):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`

Guest: **unchanged** (zero `.rs` edits).

## 7 Handover

1. **`outliner-hide-applies` regressed on #50 and the world half is fine** (§4.5): the hide reaches both
   world surfaces and the panel body is never re-taken in 30.5 s. Start at
   `typedOperationCompletionRefreshV1` (`🛠️ShellHelpers/🟦️.tsx`) with runtime diagnostics armed — a
   `{kind:"none"}` completion with no history patch and no effects now gets no refresh pass at all.
2. **`gumball-scene-delta` / `relocate-pose-delta`: the commit lands, the world payload does not
   republish** (§4.5) — `translateSelection` dispatched and history carries
   `move-object … new-origin=13.0994,0,0`, yet `data-instances-json` is byte-identical after 30 s, with an
   `Agent disconnected` banner in the same reading.
3. **`locked-flag-row FAIL lockChrome=false`** (§4.5) — the inspector renders no lock control, so
   `locked-refusal-notice` is unreachable. B28's §5 chain is gated here, not at the drag.
4. **`catalogue-add-selects-new-object`: `addObjectKind` does not select the object it adds** (§3.4) — an
   honest red now that the probe's selection census no longer answers with mode-dock tabs.
5. **`engagement_abort` cannot leave the fill tool** (§2.5). Either the guest treats a fresh utility claim
   as leaving fill, or the host disarms the tool before forwarding the abort. Guest change rides a wasm
   build.
6. **`every_advertised_engagement_verb_is_implemented`'s `clear` arm is red at HEAD** (§2.4) — the
   `ctx.clear_selection()` / `Emit.interaction_writes` path, from tonight's `de93f84300`/`989582baab`.
7. **`import.apply` emits `ops=0` on a parsed distinct payload** (§3.3, §5 item 6).
8. **`selection-keybindings` needs a "something is selected" verdict** before `duplicate`/`delete`
   (§4.5) — without it, a missed canvas pick and a dead keybinding read the same.
