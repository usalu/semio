# Wave B13 — Export / History / Locale / Engagement-Verb

Implementation pass, 2026-09-11. The four unowned FAILs left by clean battery #45b
(`🗑️generated/probe-2026-09-11T14-48-04.md`, `FAULTS=0`): `fill-history-entry entries=0`,
`export-only download=none`, `locale-control-present switched=false`,
`engagement-fill-verb #tool.fill aria-pressed=null`.

Every command tail and every browser reading quoted below is real output from this pass.

**Headline**: two of the four were real host regressions and are FIXED and re-measured green /
live-proven; the other two are **not** defects in the traced chain — `exportFixture` downloads a real
`.json` today (proven live, byte count and filename quoted), and the `fill` verb's guest half emits
`SetActiveTool{fill}` (new green law). Their probe steps still fail for reasons that are the probe's own
locators plus one systemic blocker, all named with the exact recipe below.

One systemic blocker that is **not** in this wave's remit but invalidates several battery steps is
documented in §5: the guest actor traps early with `plugin.reactor-close-authority` and everything
guest-provided (context menu, engagement dispatch, catalogue) is dead from that moment on.

---

## 1. §12 `fill-history-entry entries=0` — FIXED (host regression from B10) — **PASS after**

**Root cause** `🏛️ShellHost/🟦️.tsx:7873` `appTabsForBottomAnchor`, added by
`📓️2026-09-11-wave-B10-framework-chrome.md` §4. It fills `bottom-right` from **every**
`PanelGroup::Settings` tab in `session.app.panelTabs`. `framework.panel.history` is one of them: the
framework injects it into EVERY app unconditionally —

```rust
// 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5192-5199
if panel_tab_ids.insert(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()) {
    self.panel_tabs.push(PanelTabSpec::framework(
        PanelTabKind::App(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()),
        LocalizedLabel::native(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_LABEL, "Verlauf"),
        PanelGroup::Settings, Some(FRAMEWORK_HISTORY_BODY_KEY.to_string()), Vec::new(),
    ));
}
```

— so that the GUEST can render the history body, while `ShellHost` **also** builds that same tab
host-side (`frameworkUtilitiesHistoryTab`, `:8092-8166`, pushed at `:8501`). After B10 the anchor
carried both: two tab buttons, **one DOM id**. The probe's own tab dump proves it:

```
openPanel /settings/i tabs=[… {"id":"framework.settings"}, {"id":"framework.panel.history","text":"History"},
  {"id":"framework.marketplace"}, {"id":"framework.panel.history","text":"History"} …]
```

The guest-rendered twin won the id lookup, and a guest panel body is mounted through
`uiNodeToTreePanelConfig` (`🛠️ShellHelpers/🟦️.tsx:1896-1897`), which hosts it in a `UiDocumentStore`
whose surface id is `panel:${node.key}` — so every element id is rewritten to
`panel:<key>/<declared id>`. `[id^="framework.history.entry."]` stopped matching. The #45b dump shows
exactly that, with the entries still present under the mangled prefix:

```
entries:[] entryCount:0 sections:[]
tree:[… "#0=ACTIONS Undo Redo Redo Commit Checkpoint Create Alternative Filter All COMMANDS ",
      "panel:#0/framework.history.undo=Undo", … "panel:#0/framework.history.entry.12=Set Ac…"]
```

On #44 (`probe-2026-09-11T13-15-23.md:245`) the same query returned 29 bare
`framework.history.entry.N` rows — that is the before/after that dates the regression to B10.

**Fix** `🛠️ShellHelpers/🟦️.tsx` gains `SHELL_OWNED_PANEL_TAB_IDS` + `shellRendersPanelTabItself`, and
`appTabsForBottomAnchor` filters on it, so an anchor never mounts a panel tab the shell renders itself.
The host-built history tab stays (it is the one carrying `#s-checkin`, which
`💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2937-2938`'s e2e drives); B10's actual
target — puzzle3d's own `puzzle3d.panel.settings` reaching bottom-right — is untouched.

**Law** `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, *"never mounts an app-declared panel
tab the shell already renders itself, so one anchor never carries two nodes with one id"*:

```
 Test Files  1 passed | 22 skipped (23)
      Tests  1 passed | 871 skipped (872)
```

**Probe** before `fill-history-entry FAIL entries=0`; after, **PASS**, host-live, no rebuild
(`🗑️generated/wave-B13-probe-after1.txt:35`):

```
[94.7s] verdict fill-history-entry PASS
[94.7s] fill history after apply: {"entries":["framework.history.entry.1=Resize Window↶", … ],"entryCount":7, …}
```

and the tab list now carries `framework.panel.history` exactly **once**. Confirmed independently in the
browser: after a reload the bottom-right anchor lists one History tab.

### 1b. "a fill apply MUST produce one history row" — HOLDS, measured

`--only=tool-category,fill-tab,fill-wait-ready,fill-apply-max,fill-history`
(`🗑️generated/wave-B13-probe-after3.txt:35`) drives a real fill and the applied mutation is a row:

```
"framework.history.entry.25=create-object object { id=puzzle3d.brush.cd69933f1edb8514 object-kind=\"Hexagonal…"
"framework.history.entry.26={\"SetFillCount\":{\"count\":2}}↶"
…  entryCount:28   verdict fill-history-entry PASS
```

No guest change was needed: B10's `dispatch_emit` guard only suppresses the `record_command` row of a
`View`-kind dispatch whose **only** emission is the per-window config lane, and a fill is
`ActionKind::Mutation` carrying `artifact_mutations`, so its row comes from the mutation edit's own
`history_patch` (already pinned by the existing `import-distinct` law *"a distinct exported file must
record one history row"*, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:5111`).

**Two cosmetic residuals, flagged not fixed** (they are now visible again because the rows are
addressable): `entry.26` renders the raw serde debug `{"SetFillCount":{"count":2}}` and `entry.25` the
raw DSL `create-object object { … }` instead of a localized command label. Both are label resolution on
the history row builder, not this wave's lane.

**One false green this fix converts back into a real assertion — say so to the coordinator.**
`camera-emits-no-artifact-history` and `window-options-emit-no-history` score themselves with the SAME
`[id^="framework.history.entry."]` query, so on #45b they read `0` entries and passed **vacuously**
(`probe-2026-09-11T14-48-04.md:14,28`). They now measure real rows. B10's guest-side guard that keeps a
camera/window-option `View` dispatch out of the log **rides the next wasm component build**, so on
wasm #45 those two steps may now go legitimately red. That is a true reading replacing a blind one, not
a regression from this wave.

---

## 2. §24 `export-only download=none` — NO DEFECT IN THE CHAIN; export downloads a real file

Traced every hop and then drove it live on :6013.

- guest `📤️export-fixture/🦀️.rs:8-16` serializes the fixture and pushes
  `Effect::DownloadMediaExport{filename:"puzzle-3d.json", mime_type:"application/json", data, encoding:Some("utf-8")}`
- reactor lowers it (`🔌️plugin/⚛️reactor/🦀️.rs:1651`) onto the `download-media-export` wit variant
  (`🔌️plugin/🧬️schema/📜️.wit:429-434,587`)
- the LIVE decoder is `🔌️PluginRuntime/🟦️.tsx:867` `wireEffectToFriendly` (its `[DEBUG] … (this file's
  🔖️ActorAdapter doc)` wording is the one in the probe console, not `🎭️actor/…/🖼️wire-turn.ts`'s) and it
  **does** map `download-media-export` at `:900-901` — the older `🖼️wire-turn.ts` copy still does not,
  but nothing routes through it
- host `🏛️ShellHost/🟦️.tsx:4942-4953` → `downloadMediaExport` (`🛠️ShellHelpers/🟦️.tsx:577-592`), blob +
  `anchor.download` + `anchor.click()`

**Live proof** (browser pane on :6013, `#action.exportFixture` in the perspective window's Actions pane,
with `HTMLAnchorElement.prototype.click` and `URL.createObjectURL` hooked):

```
{"dl":[{"blobType":"application/json","size":7542},
       {"download":"puzzle-3d.json","href":"blob:http://localhost:6013/49b99fef-e7a9"}]}
```

A user pressing Export gets a real 7.5 KB `.json` download. The action row is present and labelled:

```
exportIds: ["action.exportFixture|DIV|Export"]
```

**Why the probe step fails.** Its only primary route is the canvas right-click context menu
(`activateWorkspaceMenuOrdinal("4")` → `[data-menu-action="exportFixture"]`), and that menu never opens —
`menu before 4: … "menus":[],"notices":["Agent disconnected"]`, `menu nodes exportFixture: []` in #45b,
in the #46 battery and in both of this wave's runs. The context menu is guest-provided and the guest
actor is already dead by then (§5); the same run scores `context-menu-opens FAIL rows=0`. Its fallback
then clicks the **last** button whose text is exactly `actions`, which *folds* an already-unfolded
Actions pane, so the follow-up count reads `action-pane export=0 textBtn=0`.

**Probe recipe** (not applied — the probe is not this wave's to edit):

```ts
// replace the fallback block in add("export-import", …)
const pane = page.locator('[id="framework.window.puzzle3dMainPerspective.engagement.toggle"]').first();
if (!(await page.locator('[id="action.exportFixture"]').count())) await pane.click({ force: true });
await page.waitForTimeout(1200);
const [download] = await Promise.all([
  page.waitForEvent("download", { timeout: 15000 }).catch(() => null),
  page.locator('[id="action.exportFixture"]').first().click({ force: true, timeout: 4000 }),
]);
```

**Residual, scoped, NOT fixed here** — the filename is the constant `"puzzle-3d.json"`, so exporting
Concrete Forest and Nakagin yields two identically named files. Naming it after the example is *not*
reachable from `export_fixture`: nothing retains the example identity. `set_active_example`
(`🎮️commands/🛍️set-active-example/🦀️.rs`) replaces `ctx.scene.fixture` wholesale and resets
`ctx.scene.runtime`; `Puzzle3dFixture` carries only `schema`/`domain` (both examples author
`schema=puzzle.3d.fixture domain=architecture`), and `ViewModel` (`🛂️manifest/🦀️.rs:4275+`) carries no
example/document name either. The clean fix is one new `active_example_id` on `Puzzle3dRuntime` set by
both `set_active_example` arms (the direct one and `Puzzle3dSetActiveExampleWork`), read by
`export_fixture` — that is a field in `🎚️config`, which **B12 owns this wave**, so it is handed over
rather than raced.

---

## 3. §25 `locale-control-present switched=false` — TWO findings: the control is reachable, and the switch leaked English everywhere. Leak FIXED.

### 3a The control — no reachability defect, the probe opens the wrong tab

`#framework.settings.language` is a real, correctly-labelled `Select`
(`📌️ChromePanels/🟦️.tsx:424-442`, omitted only under `host.locks.locale`, and the puzzle3d dev host
passes no `locks` at all, so it always renders). **One** click on the `framework.settings` branch
descends to its `order: 0` child and mounts it — measured live:

```
{"general":true,"lang":true,"langText":"Language\nEnglish",
 "tabs":[… "framework.settings.general","framework.settings.theme","framework.settings.keybindings" …]}
options: [{"t":"English","v":"en"},{"t":"Deutsch","v":"de"}]
```

Both options name themselves in their own language — no default-language leak in the control itself.

The probe's `setLanguage` does `openPanel(/settings/i)`, which matches on `id` OR `text` and takes the
FIRST hit. After B10 put the app's tabs ahead of the shell chrome, that first hit is
`puzzle3d.panel.settings` (its id contains "settings" and its label IS "Settings"), which has no
language row, so `switched=false`. Reordering the anchor is not a fix: `settings-steppers-present` uses
the same `/settings/i` and would break instead. The honest read is that the anchor carries **two tabs
both labelled "Settings"** — a real disambiguation wart for a user too, but the app tab's label is
guest-declared (`📌️panels/⚙️settings/🦀️.rs:21`, `LocalizedLabel::native("Settings","Einstellungen")`)
and renaming it is B12's settings lane and rides #46.

**Probe recipe** (not applied): address the shell branch by exact id instead of the fuzzy label.

```ts
const opened = await openPanel(/^framework\.settings$/);          // NOT /settings/i
await page.locator('[data-slot="panel-tab-button"][id="framework.settings.general"]').first()
  .click({ force: true, timeout: 3000 }).catch(() => {});          // no-op when already descended
const trigger = page.locator('[id="framework.settings.language"][role="combobox"]').last();
```

### 3b The real defect — switching language relabelled only half the shell — FIXED

Driving the switch live exposed a genuine no-default-language violation. After picking **Deutsch** the
shell rendered two languages at once:

```
German : Einstellungen | Vollbild | Einklappen | Beispiel | Betonwald | Standard-Apps | Remote: getrennt
English: Artifact | Catalogue | Inspection | Settings | Marketplace | History | General | Theme |
         Hotkeys | Conflicts | Language | Terminology | Driver | Layout | Appearance | Reset panels
```

**Root cause** `🏛️ShellHost/🟦️.tsx:6709-6711`: the locale effect moved **only** the shell's own
`ShellScope` i18next instance —

```tsx
useEffect(() => {
  void scope.i18n.changeLanguage(uiLocale);
  …
}, [uiLocale, uiTheme, scope]);
```

A shell has two i18n ports. `scope.i18n` (`🐚️ShellScope/🟦️.tsx:90`, `createShellI18nInstance`) is what
`useUiTranslation`/`useLabel` resolve through — that is why "Vollbild"/"Einklappen" flipped. Every
**tree builder** runs outside hook context and resolves through `shellLabel`
(`🛠️ShellHelpers/🟦️.tsx:1966-1968`), which reads the shared module singleton
`uiI18n` (`🖱️ui/…/⚛️react/🟦️.tsx:4117`) — and nothing ever called `changeLanguage` on it, so every
builder-produced name stayed pinned at the language the module booted in: the panel tab names, the
Settings branch and all its children, the Display/Tool/Command category names. (`Standard-Apps` and
`Remote: getrennt` flipped only because their builders take an explicit `locale` argument —
`defaultAppsSettingsTabLabel(locale)` / `syncPillText(state, uiLocale)` — which is what isolated the
singleton as the culprit.)

**Fix** a named `syncShellLabelLocale(locale)` beside `shellLabel` in `🛠️ShellHelpers/🟦️.tsx` (so the
coupling is stated where the reader of `shellLabel` will find it), called from the same effect:

```tsx
void scope.i18n.changeLanguage(uiLocale);
syncShellLabelLocale(uiLocale);
```

**Law** `🔬️engine-contract/🟦️.ts`, *"moves every out-of-hook chrome label when the in-app language
switch runs, and restores them on the way back"*:

```
 Test Files  1 passed | 23 skipped (24)
      Tests  1 passed | 882 skipped (883)
```

**Live proof after the fix** (reload on :6013, host-live, no rebuild) — the whole chrome moves, and the
EN⇄DE round trip restores:

```
de: framework.panel.artifact=Dokument | framework.panel.catalogue=Katalog | framework.panel.inspection=Inspektion
    framework.category.display=Anzeige | framework.panel.history=Verlauf | framework.category.tool=Werkzeug
    framework.category.command=Befehl | framework.settings=Einstellungen
    framework.settings.general=Allgemein | .theme=Thema | .keybindings=Tastenkürzel | .conflicts=Konflikte
    framework.settings.language → "Sprache / Deutsch"
en (after picking English again): Artifact | Catalogue | Inspection | … | Example | Concrete Forest
```

**Residual** `framework.marketplace` still reads "Marketplace" under `de` — either a genuinely identical
DE term or a missing bundle cell; one key, not chased here.

---

## 4. §14 `engagement-fill-verb #tool.fill aria-pressed=null` — guest chain PROVEN GREEN; one real a11y defect found and FIXED

### 4a The verb chain is correct

`🎮️commands/📨️engagement-submit/🦀️.rs:16-21` — `fill` takes the `strip_engagement_prefix` arm, sets
`ctx.scene.active_utility = "fill"` and pushes `set_fill_count::request(count)`; `dispatch_step`
(`✏️editor/🦀️.rs:3356-3360`) then emits `Effect::SetActiveTool{ tool_id: "fill" }` because fill is a
mode-level **tool**, not a window utility. The host mirrors it at `🏛️ShellHost/🟦️.tsx:4892-4903`
(`SET_ACTIVE_TOOL` + `clearAllWindowUtilities`). There was **no law** on any of it.

**Law added** `✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
`the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count` — pins both lanes (`fill` →
`SetActiveTool` and no `SetActiveUtility`; `brush` → `SetActiveUtility` and no `SetActiveTool`) plus the
`setFillCount` hand-off, because the two are one `match` apart and the browser symptom of getting it
wrong is indistinguishable from a dead input:

```
running 1 test
test editor::puzzle3d::component::tests::the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 694 filtered out; finished in 0.31s
```

### 4b The real defect the verdict was reaching for — panel tabs had no ARIA state — FIXED

`#tool.fill` is a `PanelTabBar` tab button, and live inspection showed it carries **no ARIA state at
all** while active:

```
attrs: data-slot=panel-tab-button, data-tab-id=tool.fill, data-active=true, data-state=on, aria-label=Fill
role=null  aria-selected=null  aria-pressed=null
```

`data-active`/`data-state` are styling hooks; assistive technology reads neither, so every tab in a bar
announced identically and "Fill is armed" / "the Document panel is open" never reached a screen reader.
That is a straight accessibility violation, and it is also exactly why the verdict could never go green.

**Fix** `🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx` — the tab button now carries `aria-pressed={isActive}`,
mirroring the `data-state="on"` its active colour already keys off (it *is* a toggle: pressing the open
tab folds it). The component docstring records the reason and links the APG button pattern.

**Law** `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`, *"Panel announces which tab is open
through aria-pressed, not only through its styling data attributes"* — asserts the pressed tab flips
across a rerender and the other reads `"false"`:

```
 Test Files  4 failed | 18 passed (22)
      Tests  3 failed | 705 passed (708)
```

(the 3 + the storybook loader failure are pre-existing / peer churn — see Verification.)

**Live proof after the fix** (reload on :6013, host-live):

```
tabs: ["framework.category.tool|true","framework.category.command|false","tool.fill|true"]
toolFill: {"active":"true","label":"Fill","pressed":"true"}
otherTabs: ["framework.panel.artifact|false","framework.panel.catalogue|false", …]
```

### 4c Why the probe step still cannot pass — three independent probe-side blockers

1. **It never reaches the command line.** The step's locator union ends with
   `[id^="framework.window.puzzle3dMainPerspective.engagement"] input`, and the Engagement pane also
   hosts the Action Pane, whose expanded Patch Inspector form contributes `position.x` **earlier in DOM
   order**. `.first()` therefore picks a number input. The battery's own log proves both the wrong pick
   and that the real input is right there:
   ```
   engagement toggle=1 input=1 placeholder=x
   fields:[… "position.x|number|x","position.y|number|y","position.z|number|z",
           "puzzle3d-engagement|input|brush, fill <n>, zoom, clear, pick, rectangle, lasso"]
   ```
   This also makes `engagement-brush-verb FAIL activeUtility=select` and the `engagement-placeholder-
   has-no-dead-verbs` **PASS** (it read `placeholder="x"`) equally meaningless.
2. **`#tool.fill` is only in the DOM while the Tool category branch is the active root.** Measured:
   with the branch collapsed, `document.getElementById("tool.fill")` is `null` and `[id^="tool."]` is
   empty; one click on `#framework.category.tool` mounts it. The battery's `fill-tab` step relies on the
   earlier `tool-category` step for exactly this, but by `engagement-bar` (t≈766 s) the panel has moved
   on. `getElementById(...)?.getAttribute(...) ?? null` cannot distinguish "absent" from "unpressed".
3. **The guest actor is dead by then** (§5), so nothing dispatches at all.

**Probe recipe** (not applied):

```ts
const input = page.locator('[id="puzzle3d-engagement"]').first();   // never the union+.first()
…
await page.locator('[data-slot="panel-tab-button"][id="framework.category.tool"]').first().click({ force: true });
await page.waitForTimeout(1500);
const fillPressed = await page.evaluate(() => {
  const el = document.getElementById("tool.fill");
  return el ? el.getAttribute("aria-pressed") : "absent";   // distinguish absent from unpressed
});
```

**One real cosmetic mismatch, flagged not fixed**: the placeholder advertises `fill <n>` with a space,
but the field physically cannot hold one — `normalizeEngagementActionText`
(`🖱️ui/…/⚛️react/🟦️.tsx:9725-9737`) PascalCases and strips separators, so typing `fill 5` shows
`Fill5` (measured). The guest accepts that form (`strip_engagement_prefix("Fill20","fill") == Some("20")`,
`🔌️plugin/🧪️tests/🔬️engagement-unit/🦀️.rs:6`), so the verb works — but the advertised syntax is a
lie. Fixing it means restating `PUZZLE3D_ENGAGEMENT_VERBS`
(`🎮️commands/📨️engagement-submit/🦀️.rs:12`) in the normalized form and is a guest change riding #46.

---

## 5. BLOCKER for whoever owns actor stability — the guest actor traps within ~30 s of boot

Every battery and every browser session this pass shows `notices:["Agent disconnected"]` from roughly
the third step onward, and the console names the terminal fault (decoded from the byte array in
`shard 0 worker fault [handler/turn] actor=puzzle#1`):

```json
{"code":"plugin.reactor-close-authority","message":"native close terminal unavailable",
 "origin":"framework","retryable":false,"scope":{},"severity":"error"}
```

preceded by a `registerBrushMesh` retry storm all failing `actor-activation.revoked`, and followed by
`plugin-handle.closed` on every subsequent call (`history snapshot failed`, `render failed`,
`readConflicts failed`, `local interaction observation failed`). The page then shows the
`Plugin Recovery / This program crashed.` card.

This is pre-existing — it is in #45b and in the peer's #46 battery
(`🗑️generated/battery-2026-09-11-46-6013.txt`), both of which predate this wave's first edit — and it is
what kills the context menu (`context-menu-opens FAIL rows=0`), the export primary route, the catalogue
and any late engagement dispatch. **Any battery step after ≈t+40 s is measuring a dead guest**, which is
worth saying loudly before more waves are scoped off late-battery FAILs.

---

## Verification runs

| Gate | Result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 88 warnings` / `Finished dev profile … in 0.39s` — **0 errors** |
| `RUST_MIN_STACK=… cargo test … --lib the_engagement_fill_verb_arms_the_fill_tool -- --test-threads=1` | `test result: ok. 1 passed; 0 failed; … 694 filtered out` |
| `RUST_MIN_STACK=… cargo test … --lib engagement -- --test-threads=1` | `4 passed; 1 failed` — the failure is `every_advertised_engagement_verb_is_implemented` on its `clear` assertion (`typing clear must empty the framework-owned selection, left: 1 right: 0`). **Not this wave**: it reproduces standalone, this wave's diff to that file is purely additive (one new law; the file's other 654 added lines are concurrent peers'), and `clear` routes through `ctx.clear_selection()` which this wave never touched. |
| renderer-react vitest, `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 21 passed (24)` / `Tests 9 failed \| 874 passed (883)` — the **same 9** B4 and B10 both baselined: 6 in `🧪️tests/🧩️package-integration/🟦️.ts`, 1 engine-contract `buildNoteShellCommandAction`, 2 `🔌️PluginRuntime`. **No new failure**; both of this wave's engine-contract laws pass. Full log `🗑️generated/wave-B13-renderer-react-vitest.txt`. |
| ui-react vitest, `bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/…/⚛️react/vitest.config.ts` | `Test Files 4 failed \| 18 passed (22)` / `Tests 3 failed \| 705 passed (708)` — `package entry and self-alias…`, `UIDialog … nested owned kind picker`, `UIIntroduction … glass boxes` are all three in B10's pre-existing list; the 4th failing FILE is `.storybook/🧪️tests/🧭️scope-resolution/🟦️.ts` failing to **resolve** (peer churn — `.storybook/main.ts` is modified in the working tree). The file this wave edited passes. Full log `🗑️generated/wave-B13-ui-react-vitest.txt`. |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | Filtered to the files this wave touched, every reported error predates it: `⚛️react/🟦️.tsx(10868\|10924)` `ImportMeta.dir`, `engine-contract(4900\|4901\|8057)` (peer's `interactionId`/`gumballActive` shapes, far from the new laws at ~7655), `ShellHost(1905\|7774\|7775)` — B10 reported the same three at `1898\|7759\|7760` before peer line drift. **No new error.** |
| `bun nx run @semio-tech/ui-react:typecheck` | No error in `🧭️PanelTabBar`; the `🧪️owned-locale-detector-retirement` errors are pre-existing implicit-`any`s at lines 206–1790, nowhere near the new law at ~3611. |
| probe `--only=fill-tab,fill-wait-ready,fill-apply-max,fill-history,engagement-bar,locale-switch,export-import --port=6013` | `🗑️generated/wave-B13-probe-after1.txt` — `fill-history-entry` **PASS** (was FAIL) |
| probe `--only=tool-category,fill-tab,fill-wait-ready,fill-apply-max,fill-history --port=6013` | `🗑️generated/wave-B13-probe-after3.txt` — `fill-history-entry` **PASS** with a real `create-object` + `SetFillCount` row |
| probe `--only=engagement-bar,locale-switch,export-import --port=6013` | `🗑️generated/wave-B13-probe-after2.txt` — the three still FAIL for the locator/actor reasons in §2/§3a/§4c, recipes given |

All probe runs were taken only with `pgrep -f "browser-probe|lane-probe"` empty; one run was lost to
`:6013` going unresponsive for ~1 min (`curl … 000 time=30.0`) and was re-run once it answered `200`.

**Host-live vs #46.** Every fix in this wave is in TypeScript host code and is live on a reload — all
four were re-measured on the running :6013 without a wasm rebuild. Nothing here rides #46. What rides
#46 is only the handed-over work: B10's camera/window-option history guard (§1b), the export filename
(§2), the app settings tab label (§3a), and the engagement placeholder wording (§4c).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `SHELL_OWNED_PANEL_TAB_IDS`, `shellRendersPanelTabItself`, `syncShellLabelLocale`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — the anchor filter, the shared-port locale sync, two imports
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — re-exports for the three new symbols
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — two new laws
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx` — `aria-pressed` on the tab button, docstring
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — one new law
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — one new law
