# Wave B41 — the duplicate per-window DOM id, and the two verdicts that scored the wrong thing

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · host-side only; guest halves ride wasm #57.
Written incrementally while the wave ran. Every command quoted below ran in the FOREGROUND with its tail.

Inputs read first: `📓️2026-09-12-wave-B40-locale-actor-revocation.md` §2, §5 ("Remaining reds") and §6.2
("One handover found while tracing, NOT fixed here"); `📓️2026-09-12-wave-B38-probe-recipes-export-segments.md` §1.

---

## 1 One id for every world surface — FIXED

### 1.1 Root cause, `file:line`

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:4551`
(pre-fix): `<Pane id="framework.worldOrbit.projection" …>` — a **hardcoded literal** on a component
rendered from `:6450` inside the world canvas overlay, i.e. once per mounted world surface. A world
surface is mounted once per OPEN WINDOW INSTANCE of its kind (`WindowInstanceIdContext`, `:4568`,
whose own docstring says so: *"every window kind's `UiNode` is shared verbatim across all of its open
instances"*), and the puzzle3d default layout opens two — `puzzle3d-main-top` and
`puzzle3d-main-perspective`. B38's candidate dump shows both (`inPerspective: false` /
`inPerspective: true`).

The blast radius is larger than the one literal, because `Pane` derives control ids from its `id`
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:9510`, `:9519`):

| what | id | copies in a two-pane layout |
|---|---|---|
| pane root | `framework.worldOrbit.projection` | 2 |
| chrome fold toggle | `…projection.pane.fold` | 2 |
| unfolded close control | `…projection.pane.foldControl` | 2 |
| every projection-switch row | `parallel`, `orthographic`, `axonometric-dimetric`, `perspective`, `one-point`, … | 2 each |
| the switch's tree section | `projection-modes` | 2 |

The switch rows are DOM ids too: `worldProjectionSwitchTreeItems`
(`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:1965` pre-fix) emitted
`id: template.id`, and `Tree` renders an item/section id straight onto the row
(`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:2965`, `:3055` → `TreeItem`/`TreeSection`
`id={id}` at `:1798`, `:1608`). So an unfolded pane contributes **eleven** ids, all of them duplicated.

Duplicate ids are invalid HTML, they make the pane unaddressable through `<label for>`,
`aria-labelledby` and any automation, and `document.getElementById` silently answers with the FIRST —
which is the Top pane, not the one a user is looking at.

**The measures rail is the same defect** and B40 §6.2 read it correctly: `world3d_projection_measures`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35323`) takes a **kind-level** `id_prefix`,
and puzzle3d passes the literal `"puzzle3d"`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs:17`),
while `ArtifactApp::window_measures` publishes the tree **per instance** (`✏️editor/🦀️.rs:8263`
`window_measures_body`, keyed by `view_state.window_id`). Every authored measure id therefore stands in
the document once per unfolded rail — `puzzle3d-measure-projection-orthographic-view`,
`puzzle3d-play-grid-visible`, `puzzle3d-voxel-w`, all of them. They escaped B38/B40 only because
`unfoldMeasures` unfolds the PERSPECTIVE rail alone
(`🔍️browser-probe.ts`: `[id="framework.window.puzzle3dMainPerspective.measures.unfold"]`); a user who
unfolds both gets the collision. A second, quieter consequence: `WindowMeasureTreeGroup` keys
`useTreeOpenState` on the group id (`🌳️Tree/🟦️.tsx:4186`), so folding "Parallel" in one pane folded it
in the other.

### 1.2 The fix — the id scheme, in one packet with the probe's locators

**(a) World3dHost owns a per-instance pane id.** New exported
`world3dProjectionPaneElementId(windowElementSegment)` =
`childElementId("framework.worldOrbit.projection", segment)`, so the two panes are
`framework.worldOrbit.projection.puzzle3dMainTop` and `…​.puzzle3dMainPerspective` — the same camelCase
`childElementId` convention the window element ids already use (`framework.window.puzzle3dMainPerspective`,
which `🔍️browser-probe.ts`'s own `inPerspective` predicate addresses). `WorldOrbitProjectionSwitchPane`
takes `windowElementSegment` and the render site passes `windowInstanceId ?? node.surfaceId` — the
identical fallback the neighbouring `world3d-frame-instances-${windowInstanceId ?? node.surfaceId}`
button (`:6461`) already used. The pane's two derived control ids follow for free.

**(b) The projection switch is qualified by its pane.** `WorldProjectionKindSwitchProps` /
`WorldOrbitProjectionSwitchProps` gained a required `id`, and `worldProjectionSwitchTreeItems(id, …)`
now emits `childElementId(id, template.id)` for every row (recursively) with the section id and
`selectedIds` derived the same way, so selection still matches.

**(c) The host namespaces measure DOM ids, not the guest.** New in
`🛠️ShellHelpers/🟦️.tsx`, beside `windowMeasuresChrome` (which already holds the owning `windowId` and
already stamps it onto every measure ACTION):

```ts
export function windowMeasureDomId(windowId: string, measureId: string): string {
  return `${windowId}/${measureId}`;
}
export function qualifyWindowMeasureIds(measures: readonly WindowMeasure[], windowId: string): WindowMeasure[]
```

`windowMeasuresChrome` runs both partitions through `qualifyWindowMeasureIds` before rendering. This is
the HOST's job, not the guest's, and it is the rule the codebase already wrote down for exactly this
case — `uiNodeDomId` (`🗣️Interpreter/🟦️.tsx:1244`): *"the surface prefix is what namespaces it per
window, since two windows of one app can render the same authored key"*. Same separator, same reason.
Consequences, all deliberate:

- **no plugin `🦀️.rs` change** — the authored id stays the program's one integration key, so
  `windowMeasureTreeContainsId` (which decides whose window an introduction anchor belongs to,
  `🏛️ShellHost/🟦️.tsx:9273`), the `activeUtilityId` routing and the `reveal` group ids all keep
  speaking it, and no peer-owned guest file is touched. The qualifier is a pure projection and a law
  asserts it does not mutate its input.
- the fold-state bleed is fixed with it, because the group id is what `useTreeOpenState` keys on.
- an UNLABELLED control's accessible name (`WindowMeasureSelect`'s
  `aria-label={uiDataLabel(measure.label ?? measure.id)}`) now carries the pane, which is what
  distinguishes the Top pane's projection select from the Perspective pane's for assistive tech.
  (The real fix for that name is a `label` on the guest's selects — `world3d_projection_measures`
  passes `label: None` — handover, §5.)

**(d) CAD's pane, in passing.** `✏️s/🔌️plugins/📐️cad/…/📺️renderer/🟦️.tsx:4777` carried
`id="cad-orbit-projection"`, which is not even valid element-id grammar (`Pane` → `assertElementId`
logs `console.error` on it in dev, and `childElementId` then derived two more malformed ids). It is now
the module-level `CAD_ORBIT_PROJECTION_PANE_ID = "cad.orbitProjection"`, passed to the switch as well.
CAD mounts one world per editor shell and has no window-instance context, so it is not
instance-qualified; noted in §5.

### 1.3 Files touched

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | new exported `world3dProjectionPaneElementId`; `WorldOrbitProjectionSwitchPane` exported and takes `windowElementSegment`; pane + switch share the qualified id; render site passes `windowInstanceId ?? node.surfaceId` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` | `worldProjectionSwitchTreeItems(id, …)`; required `id` on `WorldProjectionKindSwitchProps` / `WorldOrbitProjectionSwitchProps`; section id and `selectedIds` qualified |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | new `windowMeasureDomId` / `qualifyWindowMeasureIds`; `windowMeasuresChrome` qualifies both partitions; `WindowMeasureSlider` stamps `data-published-value` (§2.5) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | re-exports the four new symbols |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` | `CAD_ORBIT_PROJECTION_PANE_ID` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | new `per-window element ids` region — 4 laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx` | signature update + in-source disjointness law (see §4.2 — this project's vitest config does not load) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts` | one `../` too many on both tooling imports (§4.2) |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | `TreeCheckbox` takes `publishedValue` → `data-published-value` (§2.5) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx` | select + toggle stamp the program's published value (§2.5) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎚️window-measure-controls/🟦️.tsx` | new published-value law (§2.5) |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` | authored-tail measure locators, `published` reading, §2's two scoring fixes, one scoping repair |

### 1.4 The laws, run in the FOREGROUND

`🔬️engine-contract/🟦️.ts`, new region `per-window element ids`. The first law is the one asked for —
it MOUNTS two real `WorldOrbitProjectionSwitchPane`s in jsdom, clicks each pane's own fold toggle so the
whole unfolded id surface is live, and then reads `document.querySelectorAll("[id]")`:

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts --reporter=verbose --silent=false -t "per-window element ids"
 ✓ …/🔬️engine-contract/🟦️.ts > per-window element ids > renders two world surfaces with no duplicate DOM id 113ms
 ✓ …/🔬️engine-contract/🟦️.ts > per-window element ids > qualifies every projection switch row id by its owning pane 1ms
 ✓ …/🔬️engine-contract/🟦️.ts > per-window element ids > renders two windows' measures rails with no duplicate DOM id and keeps the authored id as the tail 14ms
 ✓ …/🔬️engine-contract/🟦️.ts > per-window element ids > qualifies ids without mutating the authored measure tree or any other field 0ms
 Test Files  1 passed | 29 skipped (30)
      Tests  4 passed | 1000 skipped (1004)
```

What each one pins:

1. **two world surfaces, zero duplicate ids** — each unfolded pane contributes `> 8` ids, the two panes
   contribute the same count, the union has NO duplicate, both `world3dProjectionPaneElementId(...)`
   values are present, and every id satisfies `isElementId` (the dot-separated camelCase grammar, which
   the old `cad-orbit-projection` spelling violated).
2. **the pure id builder** — every `worldProjectionSwitchTreeItems` row is a `childElementId` of its
   pane and the two panes' row sets are disjoint, so a regression is reported by the builder and not
   only by a mounted pane.
3. **two measures rails, zero duplicate ids**, plus `endsWith("/" + authored)` resolving to exactly ONE
   element per pane — which is precisely the invariant the probe's new locators rely on.
4. **the qualifier is a projection** — the authored tree is byte-identical afterwards,
   `windowMeasureTreeContainsId` still finds the authored id in it, and labels / `onChange` / every
   other field survive verbatim.

No guest Rust was touched, so no cargo law was owed; `cargo check` is in §4.

---

## 2 The two verdicts that scored the wrong thing — FIXED in `🔍️browser-probe.ts`

Both are B40's §2.2/§5 handovers, discharged. Every step name and every verdict name is unchanged.

### 2.1 `projection-repaints-camera` and `projection-control-flips`

`projection-control-flips` scored `JSON.stringify(before) !== JSON.stringify(after)` over the WHOLE
reading, whose `text` field is the trigger's rendered text — and the rail holds an optimistic draft
(`🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx:28` `useWindowMeasureDraft`, whose docstring measures the
round trip at 0.7 s idle and seconds on a busy app) that moves the text FIRST. B38's own run is the
proof: `after={"value":"","text":"Plan"}` — the verdict passed on the draft while the program had
answered nothing. It now scores the PROGRAM's published value with a settle of its own. (B40's literal
advice was "score on `value`"; run 1 then showed that a select trigger has no value of its own at all,
which is what §2.5 is about — read it as the continuation of this paragraph, not as a separate item.)

`projection-repaints-camera` read `cameraOf(...)` ~100 ms after the click with no settle (B38
`[22.4s]`→`[22.5s]`), i.e. it sampled the pre-dispatch pose; the `camera-gestures` lane in the same
probe already polls `cameraSettled` for every one of its three verdicts. It now waits with
`cameraSettled("puzzle3d-main-perspective", cameraBefore)` and reports `waitedMs`.

The guest half needs no change and is already lawed green — B40 §2.2's
`flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone`.

### 2.2 `locale-de-document-section-label`

It demanded `/Baukomponenten/i`, which is the **`reuse`** terminology's DE cell. The shell runs
`native` throughout this lane — the probe switches the LANGUAGE only — and `native_de` is `"Objekte"`
(`✏️editor/🗣️terminology/🦀️.rs:11`). So the verdict was scoring an axis nobody had selected, and per
CLAUDE.md an unauthored axis must never fall back.

It now reads the ACTIVE terminology off the page — the persisted chrome preference
`ui.chrome.terminology` (`UI_CHROME_TERMINOLOGY_STORAGE_KEY`, default `native`), cross-checked against
the `framework.settings.terminology` control's rendered text, both logged — and asserts that
terminology's DE cell out of a probe-local mirror of the guest's authored table:

```ts
const OBJECTS_SECTION_LABEL_DE = { native: "Objekte", reuse: "Baukomponenten" } as const;
```

So the verdict is now falsifiable in both terminologies instead of red by construction in one.

### 2.3 The locators, in the same packet as §1(c)

A measure's DOM id is now `${windowInstanceId}/${authoredId}`, so every §3/§4 locator matches on the
AUTHORED TAIL — the same rule §19's settings locators already followed ("every §19 locator matches on
the AUTHORED suffix, never on a bare `^=` prefix"). One new resolver carries it, and `readMeasure` /
`nudgeMeasure` keep their authored-id call sites verbatim:

```ts
const resolveMeasureId = async (authored: string) => …  // el.id === target || el.id.endsWith(`/${target}`), preferring the Perspective pane
```

It is idempotent on an id that is already live, so `projection-options` can feed its own candidate dump
straight back in. `unfoldMeasures`'s dump and `projection-options`'s candidate query moved from `^=` to
`*=`. `WINDOW_OPTION_IDS` and `puzzle3d-voxel-w` are untouched — they are authored ids and stay that
way.

### 2.4 One scoping repair found by the typecheck

`locked-refusal` read a `c` that belongs to the `marquee-click` step's own block
(`🔍️browser-probe.ts:1484` vs `:1523`), so the step threw `ReferenceError: c is not defined` before it
ever dragged the gumball. It now declares the same `page.locator("canvas").last()` every sibling
gesture step uses. Compile-level repair only — `locked-refusal` is not in this wave's re-run set, so it
is NOT claimed as live-verified.

---

## 2.5 What run 1 then measured: a select trigger has NO value, so the rail now publishes one

Scoring on the reading's `value` (B40's literal advice) turned `projection-control-flips` red, and the
red was the PROBE again — for a reason worth writing down. `readMeasure`'s `value` falls back to
`(el as HTMLInputElement).value`, and the target is a `<button data-slot="select-trigger">`:
`HTMLButtonElement.value` is the button's own `value` attribute, which the trigger never sets. So the
field is **structurally `""` for every select**, before and after any flip:

```
[18.0s] verdict projection-control-flips FAIL … beforeValue="" afterValue=""
        before={…,"value":"","text":""} after={…,"value":"","text":"Plan"}
```

Reading the trigger's DOM settles it: `🔽️Select/🟦️.tsx:360`–`:375` stamps `role`, `aria-expanded`,
`data-state`, `data-placeholder` — and no value. The chosen option's `data-value` exists only on a
`select-item`, which is mounted only while the popup is open. And the trigger's rendered text is the
DRAFT's (it is `<Select value={draftOrPublished}>` that decides what `SelectValue` renders). **The
published value of a draft-bearing rail control was not observable from outside React at all.**

That is a product gap, not a probe gap, so the product now closes it. Every draft-bearing rail control
stamps the AUTHORITY's value beside the value it renders:

| control | where | attribute |
|---|---|---|
| `WindowMeasureSelect` | `🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx` | `data-published-value={measure.value}` on the trigger |
| `WindowMeasureToggle` | same file, through a new `publishedValue` prop on `TreeCheckbox` (`🌳️Tree/🟦️.tsx:608`, which destructures explicitly and spreads nothing) | `data-published-value={String(measure.pressed)}` |
| `WindowMeasureSlider` | `🛠️ShellHelpers/🟦️.tsx` | `data-published-value={String(measure.value)}` |

Without it nothing outside React — assistive technology, a tutorial, an end-to-end probe — can tell a
pending optimistic draft from a landed change, for the whole 0.7 s-to-seconds round trip the draft
exists to cover. `readMeasure` now reports it as `published`, and that is what
`projection-control-flips` scores, with its own settle.

New law in `🧪️tests/🎚️window-measure-controls/🟦️.tsx` — the file whose whole subject is the draft:

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts -t "exposes the value the program published"
 ✓ …/🎚️window-measure-controls/🟦️.tsx > 🎚️ window measure controls while their dispatch is in flight > exposes the value the program published beside the draft it renders 71ms
 Test Files  1 passed | 29 skipped (30)
      Tests  1 passed | 1004 skipped (1005)
```

It pins both halves for both control kinds: mid-flight the select SHOWS "Always" while
`data-published-value` still reads `"selected"`, and the checkbox shows unchecked while
`data-published-value` still reads `"true"`; once the program publishes, each attribute follows.

---

## 3 Live verdicts on `:6013`

Run after the coordinator's full battery finished (`🗑️generated/battery-2026-09-12-56-full-6013.txt`,
`[1147.5s] done`), with `pgrep -f 'bun .*browser-pro[b]e'` empty. Smoke first:

```
$ bun 🔍️browser-probe.ts --only=boot --port=6013
[5.2s] boot booted: windows=[{"id":"puzzle3d-main-top",…},{"id":"puzzle3d-main-perspective",…}] canvases=2 treeItems=0
[10.6s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=6
```

Then the three lanes (`🗑️generated/b41-three-lanes-3.txt`):

```
$ bun 🔍️browser-probe.ts --only=projection-options,locale-switch,window-content --port=6013
[67.3s] battery verdict boot                            PASS
[67.3s] battery verdict window-both-present             PASS
[67.3s] battery verdict window-one-canvas-each          PASS
[67.3s] battery verdict window-same-document-both-views PASS
[67.3s] battery verdict window-distinct-camera          PASS
[67.3s] battery verdict projection-measures-present     PASS
[67.3s] battery verdict projection-control-flips        PASS
[67.3s] battery verdict projection-repaints-camera      PASS
[67.3s] battery verdict locale-control-present          PASS
[67.3s] battery verdict locale-flips-document-labels    PASS
[67.3s] battery verdict locale-de-document-section-label PASS
[67.3s] battery verdict locale-no-english-leak          PASS
[67.3s] battery verdict locale-switch-back-en           PASS
[67.3s] battery verdict guest-alive-read                PASS
[67.3s] battery verdict battery-hard-faults             PASS
[67.3s] battery verdict battery-faults                  PASS
[67.3s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=32
```

**16 of 16 verdicts PASS, zero faults, zero guest-death faults.** Against B40's run of the same two
lanes that is `projection-repaints-camera` FAIL → **PASS** and `locale-de-document-section-label`
FAIL → **PASS**, with the third lane (`window-content`) green throughout — the proof that qualifying
the world-surface ids broke neither surface.

### 3.1 The id fix, read off the live shell

`projection-options`' own candidate dump, which used to show two elements sharing
`framework.worldOrbit.projection`, now shows four DISTINCT pane ids and the measure ids carrying their
window instance:

```
[17.0s] projection measure candidates=[
  {"id":"framework.worldOrbit.projection.puzzle3dMainTop","slot":"pane","inPerspective":false,…},
  {"id":"framework.worldOrbit.projection.puzzle3dMainTop.pane.fold","slot":"window-pane-chrome-toggle","inPerspective":false,…},
  {"id":"framework.worldOrbit.projection.puzzle3dMainPerspective","slot":"pane","inPerspective":true,…},
  {"id":"framework.worldOrbit.projection.puzzle3dMainPerspective.pane.fold","slot":"window-pane-chrome-toggle","inPerspective":true,…},
  {"id":"puzzle3d-main-perspective/puzzle3d-measure-projection-orthographic-view","slot":"select-trigger","role":"combobox","isControl":true,"inPerspective":true,…},
  {"id":"puzzle3d-main-perspective/puzzle3d-measure-projection-perspective-kind","slot":"select-trigger","role":"combobox","isControl":true,"inPerspective":true,"text":"3-Point"},
  {"id":"puzzle3d-main-perspective/puzzle3d-measure-projection-fov","slot":"slider","isControl":true,"inPerspective":true,…}]
```

### 3.2 The projection hop, in three timestamps

```
[43.4s] projection nudge puzzle3d-main-perspective/puzzle3d-measure-projection-orthographic-view … {"before":{…,"value":"","published":"","text":""},"after":{…,"text":"Plan"},"waitedMs":35}
[44.7s] projection published="plan" moved=true waitedMs=1217
[44.7s] camera settle puzzle3d-main-perspective moved=true waitedMs=11
```

`nudgeMeasure` returns after **35 ms** — the draft — while the program's own answer lands after
**1 217 ms** and the camera follows. That 35 ms is exactly the window B38 and B40 were reading in, and
it is why the old `projection-control-flips` PASSed on nothing and `projection-repaints-camera` FAILed
on a pose that had not been published yet. Both verdicts now measure the program.

### 3.3 The terminology, read from the page

```
[30.3s] locale active terminology={"terminology":"native","stored":null,"controlText":"Terminologie Nativ"} expectedSectionLabel=Objekte
[30.3s] verdict locale-de-document-section-label PASS
```

`framework.settings.terminology` renders "Terminologie Nativ" and nothing is persisted, so the shell is
on `native` and the DE cell owed is "Objekte" — which the outliner publishes. B40 §5 called this exactly.

---

## 4 Verification, all FOREGROUND

| command | result |
|---|---|
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts -t "per-window element ids"` | `Test Files 1 passed \| 29 skipped (30)` / `Tests 4 passed \| 1000 skipped (1004)` — §1.4 |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts` (full lane, before §2.5) | `Test Files 3 failed \| 27 passed (30)` / `Tests 15 failed \| 989 passed (1004)`. **No NEW failures** — byte-identical failure set to B40's (6 in `extension invocation completion ownership` + `noteShellCommand`, 6 in `🧩️package-integration` wgpu generated-worker bytes, 2 in `🔌️PluginRuntime`), all peer-owned. Passed moved `985 → 989`: exactly this wave's four laws, and total cases `1000 → 1004`. |
| the same full lane again, after §2.5 | `Tests 16 failed \| 989 passed (1005)` — the 15 peer-owned reds plus **one load-induced timeout**, `leftover brush guest hover retain > keeps the leftover vortex id on an armed brush window`, whose whole body is three `expect`s over a pure function behind `await import("…/🌐️World3dHost/🟦️.tsx")`. It is a cold dynamic import of the three.js/r3f graph against `load averages: 51.79 46.90 33.88` (peers' builds), and the same run reports `transform 152.44s / import 221.57s` against the earlier run's `32.05s / 55.94s`. Proven to be the clock and not the logic: |

```
$ … vitest run --config …/⚛️react/vitest.config.ts --testTimeout=30000 -t "keeps the leftover vortex id on an armed brush window"
 ✓ …/🔌️PluginRuntime/🟦️.tsx > leftover brush guest hover retain > keeps the leftover vortex id on an armed brush window 21705ms
 Test Files  1 passed | 29 skipped (30)
      Tests  1 passed | 1004 skipped (1005)
```

21.7 s of a 5 s budget spent inside that one import. Nothing in §2.5 touches `World3dHost`, `PluginRuntime`
or `leftoverBrushRetainGuestHoverV1`; what it does touch is `🌳️Tree/🟦️.tsx`, which invalidates the whole
`ui-react` transform graph and so makes every downstream cold import slower while the box is saturated.

| command | result |
|---|---|
| `SEMIO_TEST_LEVEL=long … -t "exposes the value the program published"` | `Tests 1 passed \| 1004 skipped (1005)` — §2.5 |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json` | **861 errors — the identical count B40 measured**, and **zero** of them fall in any line this wave touched (checked per region: `🛠️ShellHelpers` 34xx–35xx, `🌐️World3dHost` 455x, `🎨️r3f` 196x–202x, `🧪️chunkkey` 28x–29x, `🔬️engine-contract` 96xx–97xx, the CAD renderer — no hits). This package's typecheck is deeply red independently of this wave (`🧪️docklayoutstore` fast-check generics, `ImportMeta.dir`). |
| `bun x tsc --noEmit --skipLibCheck --strict 🔍️browser-probe.ts` | **2 errors, both pre-existing and neither in this wave's edits**: `ImportMeta.dir` (bun-only) and a Playwright `modifiers` option typing at `:1698`. The third, `:1523 Cannot find name 'c'`, was a real scoping bug and is fixed (§2.4). |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile [unoptimized] target(s) in 15.45s` — **0 errors**, 88 warnings (B40's baseline exactly). No guest Rust was touched, so no cargo test was owed. |
| `bun 🔍️browser-probe.ts --only=boot --port=6013` | `verdicts=6`, all PASS, `faults=0` — §3 |
| `bun 🔍️browser-probe.ts --only=projection-options,locale-switch,window-content --port=6013` | **16/16 PASS**, `faults=0 hard=0 collateral=0 guest-death-faults=0` — §3 |

### 4.1 The one law this wave could NOT run, and why

The in-source disjointness law added to `🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx` (`qualifies every switch row
id by the owning pane so two world surfaces share none`) is **written but NOT run** — that project's
own Vitest host does not load, for two pre-existing reasons found while trying:

1. both tooling imports in `♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts` carried
   **one `../` too many** (nine ups from the config's own directory lands on the repository root, but
   `🔨️modules/🖱️ui/…` lives under `🧰️framework`, which is eight). Fixed in this wave — verified by
   `node -e resolve(...)`, which printed `/Users/ueli/Documents/semio/🔨️modules/🖱️ui/…` for the old
   spelling, a path that does not exist:
   ```
   $ bun x vitest run --config …/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts …
   ERROR: Could not resolve "../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️.ts"
   ```
2. with the paths repaired the config gets further and then hits the known strip-only blocker — a
   config that really IMPORTS framework TypeScript at runtime cannot be loaded by Node's strip-only TS:
   ```
   $ bun x vitest run --config vitest.config.ts -t "switch tree"
   ⎯⎯⎯⎯⎯⎯⎯ Startup Error ⎯⎯⎯⎯⎯⎯⎯⎯
   SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
   ```
   The engine's own `…/⚛️react/vitest.config.ts` avoids this by importing nothing from the UI build
   tooling. Handover in §5 — out of this wave's remit.

Because of that, the SAME invariant is also asserted where it does run: `per-window element ids >
qualifies every projection switch row id by its owning pane` in `🔬️engine-contract`, quoted green in
§1.4. Nothing in this report claims the `🧪️chunkkey` law passed.

---

## 5 Handovers found while tracing, NOT fixed here

1. **A hyphenated measure id can never be an introduction anchor.** `ShellHost`'s
   `introductionMeasureWindowId` (`:9269`) exists to force-unfold a rail "so targets like
   `puzzle3d-play-vortex-show` can mount for the tour", but the app builder REJECTS a non-camelCase
   `introduce`/`show` id outright — `🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs:465`
   `build_definition_rejects_introduction_step_targeting_malformed_element_id` asserts exactly that for
   `"not-camel-case"`. So no authored measure id in the repo can reach that path today (a grep of every
   `.introduce(` finds only window/panel/utility element ids), and this wave's qualification changes
   nothing that is reachable. If a measure ever IS made an introduction target, the qualified DOM id
   needs the authored id stamped as `data-element-alias`, which `elementIdSelector` already resolves —
   `SelectTrigger` and `Slider` spread rest props and would take it for free; `TreeCheckbox`
   (`🌳️Tree/🟦️.tsx:628`) destructures explicitly and would need one prop.
2. **`world3d_projection_measures` authors its selects with `label: None`**
   (`🔌️plugin/🦀️.rs:35325`), so the rail's only accessible name for them is the id itself
   (`WindowMeasureSelect`'s `aria-label={uiDataLabel(measure.label ?? measure.id)}`). An id is never an
   accessible name; those selects want real localized labels from the guest.
3. **CAD's projection pane is not instance-qualified.** `cad.orbitProjection` is now valid element-id
   grammar (§1.2d) but still one literal; a second CAD editor shell in one document would duplicate it
   again. CAD's renderer has no `WindowInstanceIdContext` equivalent to qualify with.
4. **The `🎨️r3f` package's Vitest host is unloadable** (§4.1 item 2) — every in-source
   `import.meta.vitest` law in `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` and its `🧪️tests/🧪️chunkkey` suite is
   currently unrun. Same class as the `Vite Config Node Strip-Only TS` incident: the config must import
   the UI build tooling with `import type`, or inline what it needs.

