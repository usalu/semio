# Explore 2 — the quiz's DOM, layout and interaction code, for UI mischief by pets

Read-only exploration for the coordinator of `QUIZ-PETS`. Nothing was run (no build, test, server, browser): every number is
**read** from code, **derived** from classes and tokens (marked *derived*), or **not verified** at runtime (marked *unverified*).
Reads happened 2026-10-02 22:42–22:58 WEST. The working tree is edited concurrently; the root `🟦️.tsx`, `🎨️.css`,
`🎛️preferences` and `🌐️i18n` were last written 22:41–22:42 (before my reads), the ticket `QUIZ-ADAPTIVE-LAYOUT` (opened
today, no files yet) will **change the item layouts** ("a classification item with its dropdown beside it" on one line where it
fits): every item size below is a snapshot and will shrink in height on desktop.

Path aliases: `Q` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `Q/🔨️modules` (every module file is `<module>/🟦️.tsx`),
`UI` = `🧰️framework/🔨️modules/🖱️ui/🧱️elements`, `PR` = `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react`,
`SITE` = `🎓️teaching/🏛️architecture/❓️quiz`, `FIX` = `SITE/🎭️e2e/🚶️learner/🟦️.ts`, `SPEC` = `SITE/🧪️tests`,
`SP` = `🎓️teaching/🏛️architecture/🐾️pets`, `EN` = `🎓️teaching/🏛️architecture/⚡️energy`.

## 0. The ten facts that decide the design

1. **There is no "question stack" on the run screen that is safe to touch.** A "question" is a *task* (2–3 per quiz). Its stack is a
   read-only `<ol>` of `<li>` only on the **opened quiz page** (`M/📖️quiz-page:169-177`, no handlers, no data attributes); on the run
   screen the same tasks are the task-stepper buttons (`M/▶️run:134-160`), which are controls. Items (`[data-quiz-item]`) exist only inside a run.
2. **Pets are already `quiet` (resting, frozen) on the whole run screen**: `M/🐾️pets:192` `const quiet = state.step.screen === "run";`.
   The pet-walk spec asserts "pets stay where they are while a run is on screen" (`SPEC/🐕️pet-walk:501`). Every e2e step that acts on items happens in a run.
3. **Home cards must not move**: they are click targets (`M/🪟️chrome:82-87`), they already use the individual `translate` property for the hover lift
   (`M/🪟️chrome:73`, Tailwind 4), their cells scroll (`🎨️.css:232-240` `overflow-y: auto`) and `SPEC/🥞️layered-home:113` asserts all card boxes `toEqual` after pointer moves.
4. **Displacement is clipped by the card, but not out of the scroll area.** The card body carries `clip-path: polygon(silhouette)`
   (`UI/🗂️WindowChrome:459-463,568-569`), so painting *and hit-testing* outside the card outline vanish; scrollable overflow of the page scroller
   (`M/🟦️:290` `overflow-auto`) is **not** reduced by `clip-path` (*unverified*). `SPEC/📱️phone:15-24` fails on any horizontal overflow.
5. **A transformed/translated/rotated element gets its own stacking context and paints above its non-positioned siblings**, i.e. it covers
   neighbours' buttons and takes their clicks (Playwright "intercepts pointer events", 20 s `actionTimeout`, `SITE/🎭️e2e/🎚️config:41`).
   Item gaps are only 3.2 px (desktop) / 4.4 px (touch).
6. **Card padding is 6.4 px (desktop) / 8.8 px (tablet, phone)** (`p-double`, `--spacing-compact 0.2rem`, `.touch --spacing-touch 0.275rem`): no pet fits in it.
   Free standing room beside cards exists only on the non-home screens and only wide: **144 px per side at 1440** (run/results/intro/identity),
   **272 px** on non-wide opened pages (quiz page); **none at 768 and 375**. Home has 12.8 px between cards.
7. **Pets are 28–58 × 40–56 px**, 0.8 scale below 768 px, at most 6 / 4 / 2 pets at ≥1024 / ≥768 / <768 (`PR/🔨️modules/🫧️layer:54-60`).
8. **Every pointer path is on `window`/`document` and passive**; pets never take pointer events (`PR/🎨️.css:5-11`, `PR/🔨️modules/🫧️layer:347-349`,
   asserted by `SPEC/🐕️pet-walk:469`). A pet that can be grabbed needs a deliberate break of that contract (and of pet-walk).
9. **Writing `style`/`data-*` on elements is invisible to the existing observers** (pets: `SURVEY_ATTRIBUTES = class, hidden, inert, open`,
   `PR/🔨️modules/📡️survey:225`; WindowChrome: `data-dock, data-silhouette-remeasure, data-slot, data-window-silhouette-chip`,
   `UI/🗂️WindowChrome:213`; its border observer watches `class`, `data-celebrated`, `data-introduced`, `:326`). **Writing `class` is not safe** (re-renders silhouette, and React rewrites `className`).
10. **Specs can idle for tens of seconds without any input** (the numbers are `expect` timeouts, real durations not measured): `🔌️connection-shortage` (up to 30 s + 90 s on a run screen), `📴️proctor-away` (same, on the **overview**),
    `🏆️live-leaderboard` (up to 40 s on the opened board) and, per device, `👥️shared-presence` (device A idles while B is driven). A pure "N seconds idle" rule does not protect them; the **screen/width/mode gates do**.

---

## 1. What a "question" and a "question stack" are on screen

### 1.0 The frame every screen shares

`Q/🟦️.tsx:454-481` (verbatim essentials):

```tsx
<div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale} data-icon-motion={icons ? "on" : "off"} data-pets={pets}>
  <header className="shrink-0"> <Navbar …/> </header>
  <main id="quiz-main" ref={main} tabIndex={-1} className="flex min-h-0 flex-1 flex-col outline-none"> … <Screen …/> </main>
  <LegalFooter …><PetsSwitch …/></LegalFooter>
  <PresenceOverlay …/>   {/* fixed inset-0 z-40 pointer-events-none overflow-hidden, only with peers */}
  <QuizPets />           {/* .pet-layer: position fixed; inset 0; contain strict; z-index 35 inline; pointer-events none */}
</div>
```

The **window never scrolls** (`.quiz-app` is `overflow-hidden`, height `var(--ui-available-height, 100dvh)`). Scrollers: `Page` for run/results/intro/identity
(`M/🟦️:290-291`):

```tsx
<div className="relative min-h-0 flex-1 overflow-auto p-double">
  <div className={cn("quiz-page mx-auto w-full", props.aside === undefined ? "max-w-6xl" : "quiz-pair max-w-6xl")}>
```

and `PageFrame` for the pages behind the overview (`M/🪟️chrome:107-108`):

```tsx
<div data-page={props.page} className="quiz-page-frame relative h-full min-h-0 w-full overflow-auto bg-background p-double text-foreground">
  <div className={cn("relative mx-auto flex w-full flex-col gap-double", props.wide ? "max-w-6xl" : "max-w-4xl")}>
```

Tokens (read): `--spacing-compact: 0.2rem`, `--spacing-touch: 0.275rem` (`UI/…/🖌️ui/🎨️.css:134-135`), `.touch` is toggled on `<html>` for every device ≠ desktop
(`…/🌓️appearance/🟦️.ts:274`), desktop = ≥1024 px (`📱️device:26-31`). So **single / double = 3.2 / 6.4 px at ≥1024; 4.4 / 8.8 px below**. All of it scales with the
learner's text size (`🎨️.css:7-9` root `font-size: calc(100% * var(--quiz-text-scale))`, normal 1 … largest 1.5): `max-w-6xl` = 72rem = 1152 px at scale 1.
`.quiz-target` = `min-height: max(1.5rem, 7·spacing)` = 24 px desktop, 30.8 px touch (`🎨️.css:178-180`).

**Card** (`M/🪟️chrome:52-78` → `UI/🃏️OverviewCard:95-100`):
`section[data-card="<card>"][data-presence-anchor][data-overview-card][data-revealed?].group.min-h-0.min-w-0` →
`div[data-slot="quiz-card-stack"][data-window-silhouette][data-overview-card-stack].relative.flex.flex-col.overflow-visible` →
`svg[data-window-silhouette-border]` (absolute, `pointer-events-none`, z 40) + `div[data-slot="window-chrome-cap"]` (z 2; holds `[data-slot="window-chrome-chip-cap"]` title chip, 26 px tall) +
`div[data-slot="window-chrome-body-surface"]` (glass, absolute, `pointer-events-none`) +
`div[data-slot="window-chrome-body"].window-silhouette-content-plane.relative.p-double` (**`clip-path: polygon(…)`**, z 1) > `div[data-slot="quiz-card-content"].flex.h-full.min-h-0.flex-col.gap-double` +
`div[data-slot="window-chrome-footer"]` (z 2; footer chips with the actions).

### 1.1 Home overview (≥768 px "strip", <768 px "list")

`M/🏠️home:232-264`, `UI/🥞️LayeredOverview:778-799`:

```
div.quiz-home > h1.sr-only … > div.relative.min-h-0.flex-1 >
  div[data-layered-overview][data-mode=strip][data-pan=pointer].relative.h-full.w-full.overflow-clip
    div[data-layered-strip].grid.will-change-transform   (inline transform written per frame; contain: layout paint)
      div[data-layered-pane=<id>][aria-hidden][inert].relative.h-full.w-full.overflow-hidden  (contain: layout paint) > PageFrame …   ×9
    div[data-layered-veil][data-level=dialog].ui-veil.pointer-events-none.absolute.inset-0.z-30   (clip-path hole over the revealed page)
    div[data-layered-overlay][role=group].quiz-home-grid.pointer-events-none.absolute.inset-0.z-[31]
      div[data-layered-card=<id>][data-revealed?].contents       (onPointerEnter/Leave mouse only, onFocus/onBlur)
        div.quiz-home-cell                                       (grid cell; flex; padding: double; overflow-y: auto)
          div.contents                                           (QuizCard onClick wrapper, only with onOpen)
            section[data-card="quiz:heating"][data-presence-anchor="home:quiz:heating"][data-overview-card].pointer-events-auto.cursor-pointer.transition-transform.duration-200 (+ -translate-y-0.5 while revealed)
```

* `data-layered-card` / `data-layered-pane` ids in reading order: `learner`, quiz 1, `intro`, quiz 2, `board`, quiz 3, `badges`, quiz 4, `prefs` (`HOME_PAGES`, `M/🧭️session:80`).
  The quiz id is **in the DOM** (`data-layered-card="heating"`, `data-card="quiz:heating"`, anchor `home:quiz:heating`).
* There are **no tasks and no items on a home card**: the quiz card is title chip (glyph + `h2` link `#heating`), `<p class="m-0 text-xs leading-normal text-foreground">`
  description, `Facts` = `ul[role=list].flex.flex-wrap` of `li` (text-xs: "2 tasks", best score, open run, "n learning now"), optional `Earned` `p`, footer chips (`last result`, primary start/resume).
  "A question on home" = the whole card ("n tasks").
* Desktop grid (`🎨️.css:220-244`, `M/🏠️home:75`): columns `1fr 1.5fr 1fr`, rows `1fr 1.4fr 1fr`, **no gaps, spacing lies inside the cells**; each card is `width: 100%`, centred
  (`align-items: safe center`) in its cell. Layout is the list below 768 px, or when the viewport is lower than `homeGridMinHeight` (desktop 600 px, tablet 856 px, both ×text scale).
* Sizes *derived* at **1440×900**: header 56 (`HOME_CHROME_HEIGHT_PX`), footer ≈ 25 → overview 1440×≈819 → columns 411/617/411, rows ≈241/337/241; cards 399/604/399 wide (cell − 2×6.4). The
  previous ticket measured the heating quiz card at 399×122 (cap row 26, visible body 73; `📓️report-wp-j.md:183-185`) → ≈ 60 px free above and below it in its 241 px cell.
  **768×1024** (tablet grid, 2 columns × 5 rows, cell ≈ 384×187, card 366 wide, touch spacing); **375×812** list: one `section[data-layered-section]` per page as tall as the list (≈ 724 px),
  card centred, `px-double` (8.8) and `pb-[88px]` (`UI/🥞️LayeredOverview:767`), card 357 wide.
* Pages behind the glass are `inert`, mounted and live (`LayeredPaneView:368-381`); the pets' survey skips `[inert]` subtrees (`PR/…/📡️survey:42,112-121`).

### 1.2 Quiz page (the page of a quiz, opened as `#heating`, or revealed clear behind a hovered card — inert then)

`M/📖️quiz-page:160-181`. `PageFrame page="quiz:<id>"` (non-wide: **`max-w-4xl` = 896 px**) with three cards (`card="quiz"` anchor `quiz:<id>`; `card="quiz-tasks"` anchor `quiz:<id>:tasks`; `card="quiz-crowd"`).
**The only list of questions that is read-only:**

```tsx
<ol role="list" className="m-0 flex list-none flex-col gap-single p-0">
  {quiz.tasks.map((task, index) => (
    <li key={task.id} className="flex min-w-0 items-center gap-double border border-normal px-double py-single text-sm">
      <span className="quiz-nowrap w-[2ch] shrink-0 font-semibold tabular-nums text-muted-foreground">{index + 1}</span>
      <TaskGlyph task={task} />
      <span className="min-w-0 flex-1 font-medium">{localized(task.title, locale)}</span>
      <span className="text-xs text-muted-foreground">{text(TASK_KIND_LABELS[task.kind])}</span>
    </li>
```

* 2–3 rows per quiz (cooling 2, demand 2, heating 2, physics 3), in **catalog order**; no `data-*`, no id in the DOM (the task id is only the React key). No handler on the row (not a control: not in `PET_CONTROLS`, but a `li` → default pet keep-out `PET_TEXTS`).
* Row size *derived*: height ≈ 2 border + 2·py (6.4 / 8.8) + 20 line = **28.4 px desktop / 30.8 px touch**; width = card body = 896 − 12.8 − (card padding) ≈ **870–883 px** at ≥1024, ≈ 330–340 px at 375.
* Surroundings at 1440: pane is screen sized, content 896 centred → **272 px free left and right** (`(1440−12.8−896)/2+6.4`); wide pages (leaderboard, badges, learner) 6xl → 144 px.

### 1.3 Run screen (`M/▶️run:113-247`)

```
main > Page(div.relative.min-h-0.flex-1.overflow-auto.p-double) > div.quiz-page.mx-auto.w-full.max-w-6xl >
  div.quiz-run.flex.flex-col.gap-double >
    section[data-card=run][data-presence-anchor=run]            title, description, progress, nav>ol>li>button, hint
    section[data-card=task][data-presence-anchor="task:<taskId>"]   prompt, TaskView, CrowdDoor, (TaskFigures)
```

* **Task stepper** (the other "question stack"): `<nav aria-label="Tasks"><ol role="list" className="m-0 flex list-none flex-wrap gap-single p-0">` / `<li key={entry.id} className="min-w-0">` / `<button type="button" aria-current={…} onClick={() => go(position)} className="quiz-target flex min-w-0 cursor-pointer flex-col items-start border border-normal px-double py-single text-left text-sm transition-colors …">` —
  ≈ 44 px tall (two lines), 150–330 px wide, wraps; **order is the sheet's** (first task fixed, rest shuffled, `🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts:53-56`); **no task id in the DOM** (only the active task's id in the card anchor `task:<id>`). The **quiz id is nowhere in the run DOM** (only `scene` handed to the pet layer, `M/🐾️pets:191`).
* `sheet` items per task are a random subset when the catalog sets `draw` (`🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts:20-22`): cooling/air-change-rates 10 of 15, cooling-load-and-demand 7 of 9, demand/final-energy 7 of 9, heating/u-values 10 of 17, heating-load-and-demand 8 of 11, physics/power-or-energy 12 of 16, powers 10 of 14, energies 9 of 12; demand/standard-profiles (6 items, 6 categories) draws all. → **an item ground may have no element on screen.**

**Classification** (`M/🗂️classification:76-108`). Pool and bins are drop zones; chips are rows:

```tsx
<li key={item.id} data-quiz-drag="" data-quiz-item={item.id} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)} className="grid grid-cols-[auto_minmax(0,1fr)] items-center gap-single border border-normal bg-background p-single">
  <DragGrip …/> <span className="text-sm font-semibold"><IconLabel …>{label}</IconLabel></span>
  <select id=… aria-label=… className="quiz-target border border-normal bg-background px-single text-sm text-foreground col-start-2 w-full">
```

`section[data-quiz-drop="pool"]` (`DROP_ZONE_CLASS = "flex min-w-0 flex-col gap-single border border-dashed border-normal p-double"`) > `ul.m-0.flex.list-none.flex-col.gap-single.p-0` > chips; bins: `div.grid.grid-cols-[repeat(auto-fill,minmax(min(100%,18em),1fr))].gap-double` > `section[data-quiz-drop="category:<id>"][data-presence-anchor="category:<id>"]` (+ optional `RadarChart`) > `ul` of chips.
Chip size *derived*: ≈ 2+2·3.2+24 (row 1)+3.2+24 (select) = **≈ 60 px tall, full width of the list** (≈1120 px at 1440, ≈330 at 375); bins ≥ 288 px (18em), 3 columns at 1152. Quiz id and task id **not on the chip** (task id on the card anchor; item id `data-quiz-item`).

**Sorting** (`M/↕️sorting:186-235`):

```tsx
<ol id={listId} role="list" tabIndex={-1} className="m-0 flex list-none flex-col gap-single p-0 outline-none" aria-label=…>
  <li key={id} className="grid grid-cols-[auto_2em_minmax(0,1fr)_auto_auto] items-center gap-single border border-normal bg-background px-single py-single"
      data-quiz-drag="" data-quiz-item={id} data-presence-anchor={PRESENCE_ANCHORS.item(id)} data-quiz-drop={`${ITEM_ZONE}${id}`}>
```

Row = grip, position number, label, ↑ button, ↓ button, plus a second row with the numeric guess `<input type="text" class="quiz-input w-40 …">`. ≈ **60 px tall × list width**; every row is also a drop zone `item:<id>`.

**Matching** (`M/🃏️matching:51-153`): per dimension a **pool of value cards** (`ul.flex.flex-wrap.gap-single` of `li[data-quiz-drag][data-used?]`, `inline-grid`, ≈ 50 px tall × 110–200 px wide, **no item id**) and a **table**
`<table className="w-full table-fixed border-collapse text-sm">` with `<col class="w-[38%]"/>`, `<col/>`, `<col class="w-[3.25em]"/>`; rows:
`<tr key={item.id} data-quiz-drop={`${SLOT_ZONE}${dimension.id}:${item.id}`} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)}>` → `th[scope=row]` label, `td > select`, `td > button ✕`. Row ≈ 32 px (desktop) / 40 px (touch); border-bottom is a collapsed cell border.
Example values: `data-quiz-drop="slot:u-value:wall-geg"`, `data-presence-anchor="item:wall-geg"`.

Crowd figures (only when asked) add `figure[data-crowd-figure=answers][data-task=<taskId>]` > `tr[data-crowd-item=<itemId>]` (`M/🗳️crowd:313,346`) — task and item ids, no quiz id.

### 1.4 Results (`M/🏁️results:225-268`)

`div.quiz-results.flex.flex-col.gap-double` > `section[data-card=results][data-presence-anchor=results]` (score, `ScoreFigure`, badges `ul > li[data-earned].flex.gap-double.border.border-normal.p-double`) and
per task `section[data-card=task-result][data-presence-anchor="result:<taskId>"]` with `div.relative.max-w-full.overflow-x-auto > table.w-full.border-collapse.text-sm` (`ResultTable`, line 55): rows `<tr key={item.item}>` **without any data attribute** (77, 107, 152), and for sorting also
`<ol className="m-0 flex flex-col gap-single ps-double text-sm">` of `<li key={item.item}>` plain text lines ("true order", line 124-131: read-only, no table).

### 1.5 First-visit screens
Introduction / identity / language: `Page` + `QuizCard card=introduction|identity|language|preferences`, no task/item lists. Scene is `home`.

---

## 2. Topic keys

### 2.1 The `grounds` of the twenty species (all 124 verified to exist in the four quiz files)
5 quiz-level, 13 task-level, 106 item-level. Notation `quiz/task/item`; `(task)` = task-level, `(quiz)` = quiz-level.

| Species | Grounds |
|---|---|
| sunny | physics/powers/{sun, sunlight-on-earth, sunlight-square-metre}; physics/energies/world-primary-energy; physics/power-or-energy/pv-module-peak; (task) cooling/cooling-load-and-demand |
| cloudy | (quiz) physics, cooling; physics/powers/sunlight-square-metre; physics/power-or-energy/pv-annual-yield; (task) cooling/cooling-load-and-demand |
| radiatory | heating/u-values/window-passive-house; physics/power-or-energy/heating-load; physics/powers/heating-load-old-house; (task) heating/heating-load-and-demand, demand/final-energy |
| chilly | (quiz) cooling; cooling/cooling-load-and-demand/{passive-house-home, school-new-build, hospital, office-1970s, data-centre} |
| pumpy | demand/final-energy/{kfw-40-heat-pump, deep-retrofit-heat-pump, old-house-heat-pump}; demand/standard-profiles/{kfw-40, passive-house, plus-energy-house}; heating/heating-load-and-demand/geg-2024 |
| thermy | (quiz) heating, cooling; (task) cooling/cooling-load-and-demand; heating/heating-load-and-demand/sfh-2000s; cooling/cooling-load-and-demand/attic-flat; physics/energies/boil-water |
| venty | (task) cooling/air-change-rates, demand/standard-profiles; cooling/cooling-load-and-demand/office-passive-house; demand/final-energy/{kfw-40-heat-pump, passive-house-direct-electric}; heating/heating-load-and-demand/{kfw-40, gruenderzeit-retrofit} |
| **housy** | **(task) heating/heating-load-and-demand, demand/standard-profiles, demand/final-energy**; physics/power-or-energy/{heating-load, heating-demand-house, energy-certificate}; physics/powers/heating-load-old-house; physics/energies/heating-demand-house |
| windy | physics/powers/{wind-turbine, ice-train}; physics/energies/wind-turbine-year |
| solary | physics/power-or-energy/{pv-module-peak, pv-annual-yield}; physics/powers/sunlight-square-metre; demand/standard-profiles/plus-energy-house; demand/final-energy/kfw-55-gas-solar |
| battery | physics/power-or-energy/{ev-battery, phone-charge, wallbox}; physics/powers/wallbox; physics/energies/{ev-battery, phone-charge, petrol-tank} |
| boily | demand/final-energy/{kfw-55-gas-solar, enev-2014-gas, wschvo-1995-gas, gruenderzeit-gas, old-house-gas}; demand/standard-profiles/{unrenovated-old-building, wschvo-1995, enev-2014}; physics/power-or-energy/heating-oil-litre; physics/energies/heating-oil-litre |
| flamy | physics/power-or-energy/tea-light; physics/powers/tea-light; (task) physics/powers |
| servy | cooling/cooling-load-and-demand/data-centre |
| shady | cooling/cooling-load-and-demand/{passive-house-home, new-home-geg, office-passive-house, office-geg-shading, attic-flat, office-1970s}; heating/u-values/roller-shutter-box; demand/standard-profiles/passive-house |
| roofy | heating/u-values/{solid-roof-1950s, roof-geg, roof-passive-house}; cooling/cooling-load-and-demand/attic-flat; demand/standard-profiles/{unrenovated-old-building, plus-energy-house}; physics/power-or-energy/pv-annual-yield |
| waly | heating/u-values/{half-timbered-wall, hollow-brick-wall-1970s, masonry-wall-1980s, wall-geg, wall-passive-house}; heating/heating-load-and-demand/{plattenbau, gruenderzeit} |
| insuly | heating/u-values/{wall-geg, roof-geg, wall-passive-house, roof-passive-house, masonry-wall-1980s}; demand/standard-profiles/wschvo-1995; demand/final-energy/deep-retrofit-heat-pump; (task) heating/heating-load-and-demand, demand/final-energy |
| windowy | heating/u-values/{single-glazing, aluminium-window-1970s, box-type-window, window-geg, window-passive-house, triple-glazing}; cooling/air-change-rates/classroom; cooling/cooling-load-and-demand/{attic-flat, office-1970s}; demand/standard-profiles/plus-energy-house |
| kettly | physics/power-or-energy/kettle; physics/powers/{kettle, heating-load-old-house}; physics/energies/boil-water |

The owner's example is exactly housy's task grounds: the `heating` quiz page
lists `[u-values, heating-load-and-demand]`; `heating-load-and-demand` is housy's (and radiatory's, insuly's) → "push it out of the stack". The `demand` page lists `[standard-profiles, final-energy]`, both housy's.
Task-level grounds that pets could hit on a quiz page: 13 across 8 species; **`cooling/air-change-rates` (venty) and `physics/powers` (flamy)** are the only ones for the other tasks.

### 2.2 What is matchable today and what the quiz must add

| Ground level | On-screen element | Ids already in the DOM | Missing |
|---|---|---|---|
| `<quiz>` | home card `section[data-card="quiz:<q>"]` / host `[data-layered-card=<q>]` (`M/📖️quiz-page:82-93`); opened page `[data-page="quiz:<q>"]` | quiz id | — |
| `<quiz>` in run/results | the run/results cards | none | quiz id (only via `scene`) |
| `<quiz>/<task>` quiz page | `li` of `quiz-tasks` (`M/📖️quiz-page:171`) | none (position only) | task id **and** quiz id |
| `<quiz>/<task>` run | stepper `li` (`M/▶️run:137`); `section[data-card=task]` | task card: `data-presence-anchor="task:<id>"` | stepper: task id; quiz id |
| `<quiz>/<task>` results | `section[data-card=task-result]` | `data-presence-anchor="result:<id>"` | quiz id |
| `<quiz>/<task>/<item>` run | classification chip, sorting row, matching `tr` | `data-quiz-item` / `item:<id>` + task card anchor | quiz id (scene) |
| `<quiz>/<task>/<item>` results | `tr` / `li` of `ResultTable` | none | task, item, quiz |
| item in crowd figure | `tr[data-crowd-item]` in `figure[data-task]` | task, item | quiz |

**Proposed attribute** `data-pet-prop` (the host composes the key as the quiz id of the scene/page + task + item, or carries the full key). Exact lines (the `QuizCard` change is one prop):

| File:line | Now | Add |
|---|---|---|
| `M/🪟️chrome:60` | `data={{ "data-card": props.card, "data-presence-anchor": props.anchor, "data-revealed": props.revealed ? "" : undefined }}` | `"data-pet-prop": props.prop` and a new optional `prop?: string` prop of `QuizCard` |
| `M/📖️quiz-page:82-93` (card), `:162` (`card="quiz"`), `:168` (`card="quiz-tasks"`) | no key | `prop={quiz.id}` |
| `M/📖️quiz-page:171` | `<li key={task.id} className="flex min-w-0 items-center gap-double border …">` | `data-pet-prop={`${quiz.id}/${task.id}`}` |
| `M/▶️run:115-118` (run card), `:169-172` (task card) | anchor only | `prop={view.quiz}` / `prop={`${view.quiz}/${task.id}`}` |
| `M/▶️run:137` | `<li key={entry.id} className="min-w-0">` | `data-pet-prop={`${view.quiz}/${entry.id}`}` |
| `M/🗂️classification:50` | `<li key={item.id} data-quiz-drag="" data-quiz-item={item.id} …>` | `data-pet-prop={item.id}` (task/quiz from the ancestor `data-pet-prop` of the task card) |
| `M/↕️sorting:201-207` | `<li key={id} … data-quiz-item={id} …>` | same |
| `M/🃏️matching:114` | `<tr key={item.id} data-quiz-drop=… data-presence-anchor=…>` | same (but see §4: rows are not safe to move) |
| `M/🏁️results:262` | `<QuizCard key={taskResult.task} … card="task-result" anchor=…>` | `prop={`${view.quiz}/${taskResult.task}`}` |
| `M/🏁️results:77,107,152,128` | `<tr key={item.item}>` / `<li key={item.item}>` | `data-pet-prop={item.item}` |

Resolution rule for a pet: try item → task → quiz in decreasing specificity (a drawn-out item has no element, §1.3), and ignore `.quiz-drag-ghost` (the drag ghost is a **deep clone** of the row and so carries `data-quiz-item`, `data-presence-anchor` and any `data-pet-prop`/inline style: `M/🤏️drag:26-34`).

---

## 3. Where a pet could stand or hang without covering text or controls

Facts about what the pet layer already treats as ground and as no-go (`M/🐾️pets:82,87`, `PR/…/📡️survey:21-28,174-209`):

* Surfaces: `#quiz-main [data-card] [data-slot="window-chrome-chip-cap"]`, `… [data-slot="window-chrome-body-surface"]`, `.quiz-app > footer`, and the stage floor.
* Keep-outs: all controls, all `p, li, h1-6, label, figcaption, td, th, legend, dt, dd, …`, plus `[data-quiz-item], [data-quiz-drop], .quiz-app > header`, each grown 4 px; the focused element grown 8 px.
  **So every item row, bin, pool, table cell and list item is already a no-go box**; the card's *padding* (6.4/8.8 px) is the only non-text part of a body.

| Place | Free? | Evidence/size |
|---|---|---|
| Top edge of title chip / of the body glass (card tops) | yes, existing perches | cap row 26 px, chip ≈ 72 px wide, rest of the cap row empty (`📓️report-wp-j.md:183`) |
| Footer line `.quiz-app > footer` | yes, between the "Show pets" switch (left) and the legal links (right) | 25 px (desktop) / ≈32 px (touch), `border-t`, `px-double`; `M/⚖️legal:103` |
| Gutter beside a card (run/results/intro/identity) | **only ≥ ~1290 px viewport**: `(W−12.8−1152)/2+6.4 ≥ ~68 px` | 1440: 144 px per side (previous probe: floor perch `0..146`, `📓️report-wp-j.md:190`); minus a classic scrollbar (≈15 px) on the right; 1024/768/375: 6.4–8.8 px |
| Gutter beside the opened quiz page (`max-w-4xl`) | ≥ ~1030 px viewport | 1440: **272 px per side**; 1024: ≈64 px |
| Gutter beside wide pages (board, badges, learner) | like run | 144 px at 1440 |
| Card padding | **no** | 6.4 / 8.8 px; pets are ≥ 28 wide, ≥ 40 tall |
| Between two items of a list | **no** | `gap-single` 3.2 / 4.4 px |
| Between home cards | **no** | 12.8 px horizontally (cell padding both sides); vertically the free band is the cell minus the compact card (≈ 60 px above and below a 122 px card in a 241 px cell) |
| Vertical free edges to climb | the outer side edges of `section[data-card]` where a gutter exists; on home only the outermost left edge of the left column / right edge of the right column, and these are 6.4 px from the viewport edge, i.e. not climbable by a pet | the card edge is a 1 px SVG stroke (`window-silhouette-border`), the body glass `inset-x-0` |
| Phone home list | big vertical bands: card centred in a ≈724 px section with `pb-[88px]` | `UI/🥞️LayeredOverview:767`; but pets already show ≤ 2 at scale 0.8 and `wp-j` saw none on the list (`📓️report-wp-j.md:176,192`) |

**Home panorama constraints.** (a) The cards do not move with the pointer; the *pages behind the glass* pan (strip `transform`, `window pointermove` mouse only, `UI/🥞️LayeredOverview:687-698`). Cards only lift 2 px (`translate`) on reveal.
(b) Revealing a card (`pointerenter` mouse, focus-visible) glides the strip and punches a hole in the veil (`clip-path`); the revealed page is `inert`, so pets ignore it. (c) The overlay is `pointer-events-none` except the cards themselves;
the pet layer (z 35) is above the overlay (z 31) and below the presence overlay (z 40) and dialogs (z 50). (d) A hovered card's lift triggers `transitionend`, which the pets' survey already listens to (`📡️survey:265`).
(e) Because the cards are packed with 12.8 px gaps, **the only things a pet can do on home are stand on card tops and the footer, and draw decorations (ladder/rope/parachute) in its own layer**.

---

## 4. How far an element can be displaced visually

### 4.1 Who already writes `transform`, `translate`, `rotate`, `transition`, `will-change`

| Element | What | Source |
|---|---|---|
| Home card `section[data-card]` with `onOpen` | `transition-transform duration-200` + while revealed `-translate-y-0.5 motion-reduce:translate-y-0` — Tailwind 4 writes the **individual `translate`** property and `transition-transform` includes `translate`, `rotate` | `M/🪟️chrome:73` |
| `OverviewCard as="button"` | `hover:-translate-y-0.5` (not used by the quiz) | `UI/🃏️OverviewCard:112-114` |
| Primary action chevron | `group-hover/action:translate-x-0.5` | `UI/🃏️OverviewCard:163` |
| Layered strip | inline `transform` per frame, `will-change-transform` | `UI/🥞️LayeredOverview:486,784` |
| Drag ghost | inline `transform: translate(…)` (`.quiz-drag-ghost` fixed) | `M/🤏️drag:61`, `🎨️.css:312-321` |
| Peer marks | inline `transform` + `transition: transform 120ms linear`, `will-change` | `M/👥️presence:934`, `🎨️.css:374-381` |
| Task/item icons | keyframes on `.quiz-glyph[data-motion]` (transform on the inner glyph span; **not on rows**) | `🎨️.css:49-160` |
| `.quiz-column-bar` | `animation`/`transition: block-size` | `🎨️.css:458-463` |
| Item rows, stepper buttons, quiz-page `li`, result rows | **nothing** (`transition-colors` only) | classification:50, sorting:201, run:143, quiz-page:171 |
| Reduced motion | `.quiz-app * { transition-duration: 0.01ms !important }` | `🎨️.css:597-603` |

A pet that writes `translate`/`rotate` inline on a row clashes with nothing; on a home card it clashes with the Tailwind lift. **Write inline custom properties + a data attribute rule (`[data-pet-moved] { translate: … ; rotate: … }`) rather than `class`** (React rewrites `className`; WindowChrome observes `class` with `subtree` and re-renders, `UI/🗂️WindowChrome:326`; the pets' survey observes `class`). React does not touch `style` properties it does not set: rows have no `style` prop.
React unmounts/re-creates a chip when it moves between pool and bin (different parent `ul`), which drops inline style; a sorting reorder **keeps** the element (keyed `li`) and its inline style.

### 4.2 Clipping and overflow on the ancestor chain

| Container | Clips? | Source |
|---|---|---|
| `.quiz-app` | `overflow-hidden` (window never scrolls) | `Q/🟦️:454` |
| `Page` / `PageFrame` | `overflow-auto` scrollers: **a displaced child grows scrollWidth/scrollHeight** | `:290`, `chrome:107` |
| card `section`, stack | none (`overflow-visible`) | OverviewCard:97, WindowChrome:501 |
| card body `window-chrome-body` | `clip-path: polygon(silhouette)` — visual **and hit-test** clip at the card outline; z-index 1 (footer chips z 2 paint above it) | WindowChrome:459-463,568 |
| `.quiz-home-cell` | `overflow-y: auto` (x computes to auto too): **a translated card makes a scrollbar** | `🎨️.css:239` |
| `[data-layered-pane]` | `overflow-hidden`, `contain: layout paint` | LayeredOverview:377-378 |
| `ResultTable`, crowd figure, leaderboard, profile runs | `div.relative.max-w-full.overflow-x-auto` around the `table` | results:55, crowd:315 |
| `.pet-layer` | `position: fixed; inset: 0; overflow: hidden; contain: strict` | `PR/🎨️.css:5-11` |
| `position: sticky` | none in the quiz modules (grep) | — |

**Scroll growth (derived).** A row's right edge sits `body padding + (zone border/padding)` inside the scroller's padding box. Distances from the row's right edge to the scrollport's padding-box edge:
quiz-page `li`/sorting `li`: 6.4+6.4 = **12.8 px** (desktop narrow, cards touch the gutter, < ~1170 px) / 8.8+8.8 = **17.6 px** (phone); classification pool chips add `zone border 1 + padding` → 12.8+7.4 = ~20 / 17.6+9.8 = ~27 px.
So **a rightward shift ≤ ~12 px never creates horizontal scroll at any width; 24 px does** on phone/tablet and below ~1170 px (and trips `📱️phone` `sideways()`). A **leftward** shift (negative `translate`) never grows scroll in LTR (en/de only) and is clipped by the card outline.
`rotate`: a full-width row of width W turns its corners by ±(W/2)·sin θ vertically: **W = 1120 px, θ = 2° → ±19.5 px**, over a 3.2 px gap → it covers the neighbours above/below; 883 px → ±15 px; 330 px (phone) → ±5.8 px. A 2° rotation is only harmless for elements ≲ 150 px wide (±2.6 px).
Vertical shift of the last row of a card pushes scroll height by ≤ the shift only if the card is the last on the page (otherwise it slides under the footer chips, z 2).
Overlap rule: `translate`/`rotate` ≠ none creates a stacking context ⇒ the moved row paints and hit-tests **above all non-positioned siblings**: a vertical shift ≥ the 3.2–4.4 px gap lies over the neighbour's controls (grip, ↑↓ in the top 24 px of the next sorting row; the select in the lower half of a chip). Horizontal shifts keep the row in its own band (no overlap).

### 4.3 Who measures elements (so a displaced element shifts their output)

| Code | Reads | Effect of a displaced element |
|---|---|---|
| `cursorAt(anchor,x,y)` (`M/👥️presence:783-789`) via `document pointermove` (`:807-810`) | `anchor.getBoundingClientRect()` of the nearest `[data-presence-anchor]` | own cursor is sent relative to the **moved** box; peers draw it relative to the same anchor **unmoved** on their screen (error up to the shift) |
| `placePeers(layer, within?)` (`:918-941`) every render, resize, capture-scroll, focusin, **every 500 ms** (overlay) / **250 ms** (pane peers) | `getBoundingClientRect()` of every anchor that has a peer mark; frame width/`offsetWidth` for the pane scale; focus box of `document.activeElement` | marks of peers on a moved anchor follow the moved box (consistent locally); a rotated anchor's AABB grows → focus frames (`width/height` from the rect) become larger; the **pane-peer scale** `frame.width / within.offsetWidth` is wrong if `within` (= the `PageFrame` inner div) or an ancestor is rotated/scaled — never transform those |
| drag: `startPointerDrag` (`M/🤏️drag:38-86`) | `source.getBoundingClientRect()` for the ghost origin and width, `document.elementFromPoint` → `closest("[data-quiz-drop]")` | ghost starts at the visual position (ok) but takes the **AABB width** if rotated; drop resolution uses topmost element at the point, so a moved row covering another zone steals the drop (sorting rows are both source and zone) |
| WindowChrome `measureWindowSilhouetteMetrics` (`UI/🗂️WindowChrome:136-166`) | `getBoundingClientRect` of the stack and of `[data-window-silhouette-chip]`/gap/cap; `scale = painted/layout` per axis | moving a **chip, cap or footer chip** (or rotating/scaling a card or its ancestors) redraws the silhouette wrong and the stale outline stays (ResizeObserver does not fire on transforms). Never touch `[data-slot*=chip]`, cap, footer, stack or card |
| `LayeredOverview` | root rect for the pan; strip/veil geometry in percent | not affected by a displaced *row*; displaced *cards* break the "cards stay where they are" spec |
| `RadarChart` (`M/🕸️radar:402-412`) | `getBoundingClientRect().width` through a `ResizeObserver` | **rotating any ancestor of a classification bin re-lays the chart** (bins contain the chart) — don't rotate bins/zones |
| pets survey (`PR/…/📡️survey:174-209`) | rect of every surface and keep-out per survey (every 8 ticks while moving, every second while standing, immediately on scroll/resize/focus/`transitionend`) | moved rows move their keep-outs after the next survey; `style`/`data-*` writes do **not** mark it stale, so the pets must flag it themselves |
| `useFocusAfterRender` (`M/🧩️task:71-82`) | `getElementById().focus()` (scrolls into view by visual position) | none |
| e2e | `itemBoxes`, `boxOf`, `expectInside` | see §6 |

### 4.4 Tables (matching and results)
The matching task uses a real table: `<table className="w-full table-fixed border-collapse …">` with `<tr data-quiz-drop … data-presence-anchor>`, `th[scope=row]`, `td`, and per-cell `border-b`.
Per CSS Transforms, `table-row` is a transformable element, but engine behaviour for `tr` has been uneven historically and collapsed borders belong to the table, not the row: a moved `tr` would leave its border lines behind and the row would no longer line up with the 38 %/auto/3.25 em columns (*unverified in all three engines — not run here*).
**Rule: never transform `tr`/`td`/`th` or `table`; displace a block-level child instead** (a `display: inline-block` wrapper around the `th` label, or the non-table rows: the pool chips `li` of matching and the `ol > li` "true order" in sorting results).
The results/crowd tables additionally sit in `overflow-x-auto` wrappers.

---

## 5. What "the user interacts with the element again" means in code

| Interaction | Event path | Handler (file:line) |
|---|---|---|
| Press a **grip** (classification chip, sorting row, matching value card) | `pointerdown` | `DragGrip` `onPointerDown` → `startPointerDrag` (`M/🧩️task:93-99`; `M/🤏️drag:38`); calls `event.preventDefault()`, appends the ghost to `.quiz-app`, sets `.quiz-dragging` (opacity .4); listens on **window** `pointermove`, `pointerup`, `pointercancel`, `keydown` (Escape) (`:82-85`) |
| Drop | `pointerup` over `[data-quiz-drop]` | `release` → `onDrop(zone)` → `assign` / `move` / `set` |
| Change a category / card (mouse) | `pointerdown/click` opens the native popup; commit is `change` | `<select onChange>` classification:61, matching:125 (browser fires `change` for every arrow key on a closed select; hence the 400 ms announcement delay `SELECT_ANNOUNCEMENT_DELAY_MS`) |
| Keyboard on a select/button/input | `keydown`, `focusin`, `change`/`click` | native; Enter/Escape in the guess field `onKeyDown` (sorting:102-110,130); blur commits (`onBlur` `leave`, :97) |
| Numeric guess typing | `keydown`, `input`, `compositionstart…` | `GuessField` `onChange` (sorting:125) |
| Move up/down | `click` | `ICON_BUTTON_CLASS` buttons (sorting:218,221) |
| Unassign card | `click` | matching:140 |
| Keep the order | `click` | `BodyButton` (sorting:193) |
| Task stepper | `click` | run:138-146 `onClick={() => go(position)}`; `go` sets `moved` and focuses the task heading |
| Submit / previous / next / dialog | `click` | run:124,178,183,213-221; `Dialog` document `keydown` capture for Escape/Tab (`M/🪟️chrome:309-336`), makes every sibling of the dialog **`inert`** (`:295-303`, includes the pet layer) |
| Home card: hover | `pointerenter/leave` (mouse only) on the `display:contents` host | `LayeredCardHost` (`UI/🥞️LayeredOverview:416-419`) → `reveal`/`conceal` (glide, veil hole, `data-revealed`, lift) |
| Home card: keyboard focus | `focus`/`blur` with `:focus-visible` | same host `onFocus`/`onBlur` (:418-419) |
| Home card: click anywhere but on a control | `click` | wrapper `div.contents` `onClick`: `event.target.closest("a, button, summary, input, select, textarea, label") === null → onOpen()` (`M/🪟️chrome:82-87,93`) |
| Home: mouse moves between cards | `window pointermove` (passive, mouse only, only while no page is opened/revealed) | `follow(...)` pan (`UI/🥞️LayeredOverview:687-698`) |
| Escape closes an opened page | `window keydown` | LayeredOverview:678-685 |
| Presence (pointer/focus/drag sharing) | `document pointermove` (passive), `pointerdown` capture, `pointerup/cancel` capture on window, `keydown` capture, `focusin/out`, `window blur`, `documentElement pointerleave` | `usePresencePointer` (`M/👥️presence:803-850`) |
| Programmatic focus after render | `focus()` | `useFocusAfterRender` (task:71-82) |
| Playwright `selectOption`, `fill`, `check` | **no pointer, no key events**: only `focus`, `input`, `change` | `FIX:380-383,436-442` — so `pointerdown`+`keydown` alone would miss these |

**One capture-phase set that reliably sees all of it** — on `document` (not on the element), `{ capture: true, passive: true }`:
`pointerdown` (mouse, pen, touch, every drag start, select popups, card clicks), `keydown` (all keyboard, incl. Tab/Enter/Space/arrows/typing), `focusin` (Tab, programmatic `focus()`, assistive-tech focus moves), `input` and `change` (select/typing/paste/dictation/autofill, Playwright `fill`/`selectOption`), `click` (assistive-tech activation and `element.click()` without a pointerdown). Add `hashchange`, `visibilitychange`, `resize` and a `MutationObserver` on `<html style>`/`lang` (text size, language) as **release** reasons. The listener runs before React's root listeners and before `startPointerDrag`'s `onPointerDown`, so the reset is complete when the ghost is cloned (`ghostOf` clones inline style and `data-*`).
A single listener on the displaced element alone would miss: Tab landing on a neighbour, Playwright `selectOption` on a sibling, and a hover-revealed card.
Reset must be **instant** (no `transition`): `startPointerDrag` takes `getBoundingClientRect()` at `pointerdown`, and Playwright's stability check needs two identical rects.

---

## 6. Risks for the other tickets' e2e specs

The fixture opens every context with default preferences ⇒ **pets are on (`calm`) in all specs** (`M/🎛️preferences:75` default `"calm"`); `quiet` only on the run screen. Project order (`SITE/🎭️e2e/🎚️config:44-50`): `desktop` (first-visit, layered-home, quiz-runs, live-leaderboard, both-languages; 1440×900; fully parallel, 4 workers), `phone` (375×812, `isMobile`, `hasTouch`), then alone `presence`, `shortage`, `away`, `pets`. `actionTimeout` 20 s, `expect.timeout` 20 s, no retries.

| Spec | What it does | What a ≤24 px / ≈2° random displacement or a hit-target pet would break | Idle without input |
|---|---|---|---|
| 🎯️quiz-runs | `enter`; `playQuiz`; per task `openTask` (`nav button.click()`), `classify` (`selectOption`), `dragInto` (`grip.hover()`, `mouse.down`, moves to bin centre, polls ghost select + `quiz-drop-active` ≤20 s), `sortInto` (`[data-quiz-item=x] button.first().click()` + poll order), `moveDown` (`button.nth(1)`), `match` (`selectOption`), `unmatch` (`row.getByRole("button").click()`); `submitRun`; reads results via `innerText` | on the **run screen**: any moved row ⇒ unstable/intercepted clicks (20 s timeout), wrong ghost origin; on **results**: read-only → no effect. Pets are `quiet` in the run ⇒ safe **if mischief honours `quiet`** | none (steps ≤ 1–2 s) |
| 🥞️layered-home | exact box equality of all nine `[data-overview-card]` after pans (`:113`); pane positions; `board.hover()`; `mouse.move` to corners ±2 px of the overview; veil `data-veil`; hash open/Escape | **any home-card displacement fails** `toEqual(cards)`; a pet that is a hit target over a corner or a card centre swallows `pointerenter` and flips the reveal/veil assertions; hover flicker (card → pet → card = conceal/reveal) | `waitForTimeout(800)` once; polls of ≤500 ms glides |
| 🪪️first-visit | radio `check()`, text `fill`, `primary(form).click()` on intro/identity (`Page` screens, gutters up to 144 px), card clicks, `Escape`, dialog | intro/identity have no questions; any mischief there only risks the clicks | none |
| 🗣️both-languages | long journey (2–5 min): per page/task `expectBothLanguages` switches language twice, reads `button, h1…, th[scope=col]` text and `aria-label` (skipping `[inert]`/`[aria-hidden=true]`), opens every page by hash (**including every quiz page**), runs three quizzes | text-only checks, not geometry; a pet toggling `aria-*`/`lang`/`inert` or a hit-target pet over the language switch (`header button[lang]`) would; the quiz page rows are exactly where mischief would happen but are only read | none |
| 📱️phone | 375×812, `hasTouch`: list mode, plays cooling, `overflow()` = `scrollWidth−innerWidth ≤ 0`, **`sideways()` = max over every `overflow-x:auto/scroll` box in `#quiz-main` of `scrollWidth−clientWidth` must be 0** (tables excluded) | any rightward displacement > ~17 px (sorting/quiz-page rows), or rotation of a wide row, can make `Page` scroll sideways ⇒ fail. **No mischief at ≤ 767 px** | none |
| 👥️shared-presence | two devices; `itemBoxes(anna)` **`toEqual(alone)` three times** (`:79,88,93`) ("no item moved…"), `expectInside` of a peer cursor in its anchor ±1 px, `hover({position:{x:12,y:12}})` on the board card, `hover()` on `[data-presence-anchor="item:x"]` | any item displacement on the run screen fails the equality; a displaced anchor shifts peer marks until the next 500 ms `placePeers` | **yes, per device**: device A idles for tens of seconds while B answers/submits a whole quiz (`:90-93`), then `expect.poll` ≤ 20 s |
| 🔌️connection-shortage | reload mid-run; stops/starts the proctor (`proctor("stop")` HTTP), `expect(connection tone=alert, 30 s)`, `calm` ≤ 90 s, `openTask`, `answerTask`; `shownAnswers` compares selects | idle on the **run screen** (quiet) — fine if gated; `openTask(learner, 0)` after the wait is a stepper click | **up to 30 s + 90 s idle** (the timeouts; real durations not measured) |
| 📴️proctor-away | proctor off before arrival; `enter`, plays cooling, reload, waits for `[data-board-local]` ≤30 s, tone `calm` ≤90 s, **on the overview**, then reads `shownBest` (`li` text) and counts `li[data-earned]` | idle on **home with live pets**: the later steps are text reads, `enter` on a second device and `goHome`; low risk, but a displaced home card would still break the next `primary(card).click()` | **up to 30 s + 90 s idle on home** |
| 🏆️live-leaderboard (not asked, read) | `primary(card board).click()`, `row(name)` visible ≤ 40 s, clicks column headers/`choice(...)` | a displaced leaderboard header/row | **≤ 40 s idle on the opened board** |
| 🐕️pet-walk (project `pets`) | its own contract: layer `pointer-events: none`, `aria-hidden`, no descendant with `button/[tabindex]/[role]/[id]`, `takenTargets` = no pet is ever the `elementFromPoint` (`:237`), pets stand on edges, "stay beside a task" (`misplaced`), **still during a run** (`:501`), `scrollWidth − innerWidth ≤ 0` (`:481`), ≤ 2 small pets and never on a control on a phone (`:804`) | grabbing pets (hit target), pets in gutters/pushing, pets moving in a run each contradict an assertion here → this spec must change with the feature | by design |

Other facts:

* **How continuously the specs act.** Except the five idle cases above, every step is a Playwright action or a ≤ 20 s poll; `quiz-runs`, `phone`, `both-languages`, `first-visit`, `layered-home` never rest > 1–2 s (derived from reading). `desktop` runs five specs in parallel, so a page may be CPU-starved; a displacement animation could straddle an action.
* **Playwright mechanics** (documented behaviour): `click`/`hover`/`dragTo` wait for *visible, stable (same rect on two frames), enabled, receives events (hit-target at the click point)*; `selectOption`/`fill`/`check` wait for visible+enabled and dispatch DOM events directly. A moved element with a running CSS transition is "unstable"; a moved element overlapping another's centre or control "intercepts pointer events" until the 20 s timeout.
* Fixture hooks that must keep working: `[data-layered-card]`, `[data-layered-pane]`, `[data-overview-card-action=primary|secondary]`, `#quiz-main [data-card="run|task|results|task-result|dialog|identity|introduction"]:not([inert] *)`, `[data-quiz-item]`, `[data-quiz-grip]`, `[data-quiz-drop^="slot:"]`, `.quiz-drag-ghost`, `.quiz-drop-active`, `nav button` of the run card (`FIX:242-269,335-362`). A `data-pet-*` attribute or `[data-pet-moved]` rule changes none of them.
* **Safe rule that keeps all specs green** (see §8): mischief only on (1) the opened quiz page and (2) the results screen; never on run, home overview, intro/identity; only at ≥ 1024 px (non-touch), `calm`/`lively` (not `still`/`off`/forced colours/reduced-motion default), no dialog (`inert`), no `.quiz-drag-ghost`, not while an element in the same card has focus, document visible; first move only after ≥ 20 s (pet-time at `tempo` 1) without any document-capture event; stop and instantly restore on the capture set of §5. The owner wants the undo triggered *by the learner*; an unattended page keeps its displacement until then, unless a time limit is added (a design choice, not an owner request), and it must also be undone on scene change/`hashchange`/resize/locale or text-size change.

---

## 7. Free space for dragging a pet

Free = nothing interactive under the pointer (derived at scale 1; "pointer-free" means no control, no text block, no drop zone):

| Screen @ width | Free regions |
|---|---|
| Home 1440×900 | 12.8 px gutters between cards (too thin); per-cell vertical bands (≈ 40–120 px above/below each card; the three rows are 241/337/241 tall, cards 122 – ≈230); the footer middle (between switch and links, 25 px high); the navbar is a keep-out. **All of it is inside the overlay, so the pointer crosses hover-reveal cards** |
| Home 768×1024 | cells ≈ 384×187, gaps 17.6 px; bands per cell ≈ 0–60 px; footer ≈ 32 px |
| Home 375×812 (list) | per section: bands above and below a centred card, plus `pb-[88px]` ≈ ≥ 100–300 px tall × 357 wide, inside a `snap-y` scroller |
| Run/results/intro/identity 1440 | left and right gutter 144 px × ≈ 819 px (main height) each (minus the scrollbar); between cards `gap-double` 6.4 px |
| Opened quiz page 1440 | 272 px gutters (non-wide), 144 px (wide) |
| Run/results 1024 | 6.4 px (none); 768/375: 8.8 px (none) |
| Footer line | full width except switch (left) and links (right) |

Global pointer handling a dragged pet must not disturb:

1. **Presence cursor sending** — `document pointermove` (passive) maps the event target to its nearest `[data-presence-anchor]` and publishes at ≤ 15 Hz (`M/👥️presence:57,805-810`; `PRESENCE_FRAME_HZ`). With pointer capture on a pet element (not inside any anchor) the learner's cursor is **cleared** for the others (`presence.point(undefined)`); without capture the cursor keeps tracking anchors under the pointer. Neither touches items: a "drag" is only shared when the target is inside `[data-quiz-grip]` (`draggedItem`, :795-798).
2. **Home pan** — `window pointermove` (passive; mouse only; skipped while a card is revealed or a page opened) pans the strip with the mouse position, so a dragged pet between cards pans the pages (`UI/🥞️LayeredOverview:687-698`); `pan="none"` is the only prop switch (`panning` at :452).
3. **Card reveal** — React `onPointerEnter/Leave` on `[data-layered-card]` (mouse only); sliding over a card glides the strip and lifts the card 2 px; with pointer capture on the pet these `enter/leave` events do not fire.
4. **Card click** — `QuizCard` `onClick` on `div.contents` opens the page for a click anywhere but a control (`M/🪟️chrome:82-87`); a pointerup after a drag that began on the card's edge can still dispatch `click` to the common ancestor ⇒ the pet drag must swallow the next `click` (one-shot capture listener).
5. **Text selection** — only grips carry `select-none touch-none` (`M/🧩️task:95`); `startPointerDrag` relies on `preventDefault()` in `pointerdown`; `pets` currently listen passively and cannot cancel. A pet drag needs a non-passive `pointerdown` (or `user-select: none` on `.quiz-app` while dragging).
6. **Touch scrolling** — scrollers are `Page`/`PageFrame` and the phone list (`snap-y snap-mandatory overflow-y-auto overscroll-y-contain`); `LayeredCardHost` ignores touch pointers for reveal; a touch drag of a pet needs `touch-action: none` on the pet's own box only (the layer is `pointer-events: none` today, so touches scroll the page through it).
7. **Hit-target semantics** — the layer is `pointer-events: none` everywhere and `host.style.pointerEvents = "none"` is re-asserted by the running show (`PR/🔨️modules/🫧️layer:347-349`); grabbing is implemented today as a window `pointerdown` hit test of the actor boxes (`poked`, `📡️survey:305-312`), which does not intercept anything and is skipped over `PET_CONTROLS`.
   The same window-level route (hit test on `pointerdown`, `pointermove`/`pointerup` on window, no pointer capture needed) keeps hover and `elementFromPoint`-based specs untouched; a real `pointer-events: auto` pet box would change `pointerenter` semantics and break `pet-walk:469`.
8. **Dialog** — while a dialog is open everything but the backdrop is `inert`, including the pet layer ⇒ pets rest (`PR/…/🫧️layer:309-312`); a drag must end on `inert`.

---

## 8. Recommendations

**Displaceable kinds (safe set), by screen**

| Screen | Element (selector after the proposed attribute) | Max displacement | Pet stands | Notes |
|---|---|---|---|---|
| Opened quiz page `#<quiz>` (≥ 1024 px) | `[data-card="quiz-tasks"] li[data-pet-prop]` (the question stack, `M/📖️quiz-page:171`; 2–3 rows) | `translate: -100% 0` at most (slides under the card's left outline, clipped, no scroll growth); a rightward push ≤ 12 px; **no rotation** (row is ~880 px wide: 2° = ±15 px) | in the gutter right of the stack (272 px free per side at 1440; needs ≥ ~1030 px viewport), pushing leftwards so the row exits left. The gutter has **no perch at the row's height** (only the floor/footer): the pet reaches it by a ladder/rope drawn in its layer, or — for the first row only — stands on the card's top edge, which is directly above row 1 (the first `li` starts one body padding below the body glass top) | read-only row with no handler; no ids today → add `data-pet-prop` first. Only the pet whose ground matches the task (task level), e.g. housy ↔ `heating/heating-load-and-demand`. The moved row keeps its layout slot (the gap stays), it is only invisible behind the outline |
| Results (≥ 1024 px) | `ol > li` of "true order" (results:128, sorting results only) and `ul > li[data-earned]` badges (results:246) | same: `-100%` left, ≤ 12 px right; **no rotation** (rows ≈ 1100 px wide at 1440) | gutter (144 px at 1440, needs viewport ≥ ~1290 px), same reach problem | read-only text; never `tr` |
| Home overview, run, intro, identity, first visit | **nothing** | — | pets stand on card tops/footer, decorations drawn in the pet layer only | cards are clickable, hovered, lifted and measured; run is `quiet` |
| Tablet (768–1023) and phone (≤ 767) | **nothing** | — | — | `.touch` spacing 4.4/8.8 px, no gutters; `sideways()` guards; 2 pets at scale 0.8 |
| Matching tables, stepper buttons, any control, silhouette parts (`[data-slot*=chip]`, cap, footer, stack, card), `Page`/`PageFrame` and their ancestors, bins/zones with a radar | **never** | — | — | table rows (§4.4); controls (§5); silhouette measuring (§4.3); radar `ResizeObserver` (§4.3) |

**Mechanics**: a single attribute per moved element `data-pet-moved` plus inline custom properties `--pet-dx --pet-dy --pet-turn`, and one rule in `Q/🎨️.css` (`[data-pet-moved] { translate: var(--pet-dx) var(--pet-dy); rotate: var(--pet-turn) }`, `transition: none`, wrapped in `.quiz-app[data-pets="calm"], …="lively"`); never `class`, never layout properties, never DOM insertion (ladders, ropes and parachutes live in `.pet-layer`; the document stays untouched). The pets host also tells the survey to re-measure after each write (the existing observers will not).
Because the clip is the card outline, the reset can be a pure removal of attribute + properties (no animation needed); under `prefers-reduced-motion` the existing `0.01ms` rule makes any transition instant.

**Gates** (all must hold): screen is the opened quiz page or results; viewport ≥ 1024 px and not `.touch`; mode `calm`/`lively`; not forced colours; no `inert` ancestor of the layer (no dialog); no `.quiz-drag-ghost`/`.quiz-dragging`; focus not inside the card; ≥ 20 s since the last capture event (§5) in pet time (tempo-aware, 8× in pet-walk); release on the §5 capture set, `hashchange`, `resize`, locale/text-size change, scene change, `visibilitychange` and on unmount.

**What this keeps green** (read, not run): quiz-runs, phone, shared-presence (`itemBoxes` equality), layered-home (`toEqual(cards)`), first-visit, connection-shortage, proctor-away, live-leaderboard — none acts on a quiz page's `quiz-tasks` rows or on a result `ol`; `both-languages` only reads the quiz pages' text. `pet-walk` has to be extended (pets pushing from gutters, `misplaced`, `takenTargets`).

---

## 9. Not verified / open

* All pixel numbers except the quoted probe (`📓️report-wp-j.md:183-185`, heating card 399×122 at 1440) are *derived* from classes and tokens; nothing was measured at runtime. The adaptive-layout ticket will change item heights.
* `clip-path` not shrinking scrollable overflow, transformability of `tr` in Chromium/Firefox/WebKit, and the exact scrollbar width of the target browsers are *unverified*.
* `QUIZ_PET_SURFACES` selects `[data-slot="window-chrome-chip-cap"]` and `…-body-surface` (`M/🐾️pets:82`); the earlier probe predates that change, so edge positions should be re-probed before relying on 26 px / 73 px.
* Not read: `🚀️site-boot`, `📰️host-document`, `🧪️deploy`, `🧱️local-stack`, `🧪️catalog` (no UI interaction), the pets stage behaviour (`🎪️stage`) beyond `quiet`/`hushed`.
