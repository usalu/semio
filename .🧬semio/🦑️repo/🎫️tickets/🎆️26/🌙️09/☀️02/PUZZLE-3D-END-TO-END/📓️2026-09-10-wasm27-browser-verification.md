# 🔬️ Wasm #27–#31 browser verification — 2026-09-10

## 00. Wasm #31 (~06:35) — NAKAGIN RENDERS END TO END 🎉

- Boot 5.5 s, zero faults. pick/context/switch all clean.
- **Nakagin Capsule Tower fully renders in both viewports** (~50 s switch): stacked capsule tower in
  Perspective, complete site plan in Top. Screenshot
  `🗑️generated/probe-2026-09-10T04-30-13-example-switch.png`. The whole chain now holds: chunked
  one-undo setActiveExample (W-X) → scene lanes under the fixed caps (W-P) → bounded surface envelopes
  (W-X §6) → symmetric packed-leaf unpack (W-X §6 follow-up) → paged brush-mesh upload (W-M2).
- Wasm #30 interlude: boot was blocked by the packed-leaf asymmetry
  (`SyntaxError: Unterminated string in JSON at position 511` at `sectionValueFromBuiltNode`,
  probe `probe-2026-09-10T03-23-39.md`) — fixed by W-X's shared `packedTextLeaf`/`packed_text_leaf`
  helpers in every consumer; native law `reserved_refresh_section_payloads_admit_into_the_retained_section_carrier`
  went green. Suite 639/1 (foreign VCS law only).
- ~~OPEN (wave W-G3): fill tool body renders EMPTY~~ **FIXED, browser-proven** (renderer-TS only, no
  wasm rebuild): the panel stayed on the Tool BRANCH node and `Panel` only mounts trees for leaf nodes;
  `drillOnOpen` was dropped by `Panel` and skipped on the category-only closed-press path. Fix: apply
  `drillOnOpen` on every closed-host open, forward it through `Panel`, and render the remembered/first
  descendant leaf via `resolvePanelBranchBodyLeaf` (Panel + mobile Layout). Proof
  `🗑️generated/probe-2026-09-10T05-19-26*`: one press on `#framework.category.tool` → tab=tool.fill,
  treeItems=3, body "Count | 0 | Hexagonal Cut Concrete Forest Left | 100% | Distribution", faults=0.
- **OPEN (W-G3 follow-up): fill build never ticks in the browser.** Coordinator probe
  `probe-2026-09-10T05-22-32.md`: Count slider stays 0 (aria 0/1000) after 40+ s with the tab open and
  after keyboard End (clamp-to-available with available=0); the guest's leftover `[DEBUG] fill_build_tick`
  eprintln never prints in the console → `fillBuildTick` is never dispatched. Guest chain is proven
  natively (fill family 59/0, heap law ready>0), so the gap is host wiring: tool never ARMS on tab select
  (the in-tree `toggle-group-item#tool.fill` was removed as a duplicate id) and/or the retained fill job
  is never scheduled. W-G3 resumed with the live browser loop.
- Probe gained `--fill` (tool tab + fill activation + ready wait) and the widened FAULT_RE
  (`Credits {`, `NodeCapacity`, `SemioFaultError`).
- Still to browser-verify after W-G3: fill ready>0 + apply, switch undo as ONE step, clipboard
  copy/cut/paste, marquee rectangle, import/export round trip, locked-volume refusal notice, brush,
  gumball drag, suggestions.

## 0. Wasm #28 update (~03:40, includes W-U3 editor-correctness wave)

- Nakagin switch: `history-panel.row-action-args` arena exhaustion **GONE** (W-U3's outliner +
  inspection paging fixed it, exactly as §2 predicted). Zero faults through boot + all interactions +
  Nakagin switch (probe `probe-2026-09-10T01-15-23.md`).
- **NEW DEFECT (open): the world never re-renders after `setActiveExample`.** Even with a 150 s
  settle the viewports keep the old platform while the navbar shows "Nakagin Capsule Tower"
  (probe `probe-2026-09-10T01-18-06.md`).
  Console evidence: `Puzzle3dSetActiveExampleWork` (Migrated interactive job) steps ONE item per
  command-ingress round trip (~1.5 s each: thunk start → settled → completion apply op N →
  applyHostEffects refresh `windowBodies:["puzzle3d.play.composite"]`), reached op ~100 in 150 s and
  was still going — Nakagin's extent (attractions+objects+target_volumes) means minutes of churn.
  TWO defects: (a) ARCHITECTURE — a whole-document switch must be ONE emit (or few bounded chunks),
  not 150 round trips; (b) RENDER — ~100 partial refreshes naming the composite window body never
  updated the world-3d scene in either viewport (scene-surface republish or TS lane application is
  not driven by the partial-windowBodies refresh path in the real browser; the native lane laws pass,
  so the gap is in the refresh→scene-surface wiring or the renderer's carrier application).
- Disk incident resolved: /System/Volumes/Data hit 100% (120 MiB free) — cause of the silent serve
  deaths and cargo incremental corruption. Deleted two stale 52 GB coordinator target dirs
  (`target-p3d-p`, `target-p3d-f`); ~11 GiB free after. `target-p3d` (in use) kept.

# 🔬️ (original) Wasm #27 browser verification — 2026-09-10 ~02:35

Serve: release pipeline (materialize → prepare → activate → serve) on 127.0.0.1:6013, wasm built
02:00 from the green tree (scene lanes, brush-mesh registry, fill-session cursor, all landed fixes).
Probe: `🔍️browser-probe.ts` (now dismisses the "Welcome to Puzzle 3D" tour — its Skip button is not
inside a `[role=dialog]`, which had silently blocked every click of the earlier `--interact` runs).

## 1. Results

| Check | Verdict |
| --- | --- |
| Boot (2 windows, 2 canvases) | ✅ 5.6 s, **zero faults** (previously: intake-budget-exhausted + fixed-capacity at scene-surface.encode) |
| activate-perspective / pick-object / context-menu | ✅ no new faults |
| Example switch → Concrete Forest | ✅ renders (platform mesh in Top + Perspective) |
| Example switch → **Nakagin Capsule Tower** | ❌ world body never updates; ONE fault (below) |
| Nakagin: `ui.fixed-capacity` at scene-surface.encode | ✅ GONE (scene lanes work in the browser) |
| Nakagin: `intake-budget-exhausted` / zero-progress | ✅ GONE |

Nakagin fault (deterministic, aborts the whole publish, so the stale UI persists):

```
[DEBUG] typed-operation completion effects failed SemioFaultError:
ui.fixed-capacity: fixed UI admission failed at history-panel.row-action-args
```

## 2. Root-cause analysis

- The failing site (`🔌️plugin/🦀️.rs` ~:9849) allocates a ONE-entry map (`entrySeq`) per revertible
  history row — tiny. The failure is `UiMapBuilder::try_new()` → `reserve_collection` returning
  `None`: the process-global `UI_VALUE_ARENA` (`🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:908`)
  is **exhausted** by the time the history panel assembles.
- Arena budget is one live page of rows (`UI_VALUE_LIVE_PAGES = 1`,
  `UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX - 1`, ≤5 collections/row). Designed backpressure:
  publish at most a page of action-bearing rows at a time.
- `setActiveExample` is ONE command → one history entry (`🎮️commands/🛍️set-active-example/🦀️.rs`
  swaps the whole fixture) — history is NOT the hog. The hog is whichever earlier-assembled panel
  emits UNPAGED per-row arg maps for Nakagin's ~140 volumes: the artifact outliner rows and/or the
  inspection `ids` list. Those are exactly wave W-U3's defects 3 (outliner continuation row) and
  4 (inspection ids paging). History panel is the victim because it assembles last.
- Concrete Forest (few volumes) fits inside the arena, which is why everything else passes.

**Handoff: W-U3 defects 3+4 close this.** After they land + rebuild #28, rerun
`bun 🔍️browser-probe.ts --interact --example=Nakagin` and require: no faults, world body shows the
tower. Do NOT bump the arena constants — the budget is the law; the panels must page.

## 3. Serve infrastructure notes

- Rebuild #27 (`rebuild-until-ok.sh`) built OK in 6 m 56 s but its serve step died silently; every
  `nohup`-backgrounded serve/materialize from coordinator shells was reaped within ~60 s (no OOM log,
  no crash report, exit line never written). Foreground/harness-managed runs work. The serve now runs
  as a harness-managed background job; the vite dynamic-import warning at `backbone-worker.ts:29079`
  is benign (present on healthy serves).
- Probe changes: tour dismissal after boot; `--example=<regex>` picks a specific `[role=option]` in
  the navbar example combobox.

## 4. Screenshots

- `🗑️generated/probe-2026-09-10T00-33-02-example-switch.png` — Concrete Forest rendering.
- `🗑️generated/probe-2026-09-10T00-34-22-example-switch.png` — Nakagin selected in navbar, world
  still showing the previous scene (the aborted publish).
