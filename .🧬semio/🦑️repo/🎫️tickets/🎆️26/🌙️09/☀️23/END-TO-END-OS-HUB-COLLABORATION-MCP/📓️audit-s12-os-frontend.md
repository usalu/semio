# S12 — os `s` Frontend Audit (Outcome 1: working `s`, all plugins/artifacts)

Read-only, no edits/builds/servers started by me (one already-running serve observed via `curl`, per the rules).
2026-09-25, ~22:53–23:4x. Session 12 started 22:50; the fleet (`📓️fleet-12-agents.md`) launched ~23:00, so several
slices (W2, S15, C10, H9, R8) had already resumed and edited their own reports by the time this audit ran — this is
a snapshot, not a final state. Sources read in full: `AGENTS.md`, `📓️session-12-preamble.md`,
`📓️session-11-handover.md`, `📓️audit-s11-os-frontend.md`, `📓️wp-s15.md`, `📓️wp-u5.md`, `📓️wp-t12.md`, `📓️wp-w2.md`
(session-11 bodies + each report's new `## Session 12` section), `📓️work-packages.md` §Session 11 log,
`📓️fleet-12-agents.md`, `🎫️ticket.json`; the `## Session 12` sections of `📓️wp-c10.md` and `📓️wp-h9.md` (targeted,
for hub-document gaps that gate outcome 1). Verified directly against the tree: `🔌️plugins.json` (60 rows, 34
plugin + 26 extension), the activation receipt (60/60, all `rebuiltAt` 18:15:24–18:55:59, i.e. W2's restage4), a
staleness check of `*_component.core.wasm` mtimes under `dist/runtime/react/dev/s/extensions/`, live process/port
state (`lsof`, `ps`, one `curl`).

## 0. Headline

- **Nothing was running at session-12 start (19:30 desktop restart killed everything); by audit time (~23:0x) two
  session-12 slices had already restarted work.** Confirmed live: a local-only `dev s` serve on **6540** (nx pid
  85035, started 22:55:56; vite pid 87082 bound 22:57:02; `curl http://127.0.0.1:6540/` → **HTTP 200**) — this is
  S15's session-12 re-serve (`wp-s15.md` §Session 12, item S12-1 "booting serve 6540"). No hub is listening on 7800
  or on any slice's hub range (8000–8109) at audit time: W2's session-12 item S12-1 (catalog **B2**, 9 packages) is
  `RUNNING` (chain pid 83333, started 22:54), and until it (or `--packages all`, S12-2) publishes, **outcome 1 has
  no live hub anywhere** — every hub-document, lazy-install, undo/redo-over-hub and collaboration proof below is
  blocked at the infrastructure level, not a per-plugin problem.
- **The last full "boot→mutate→undo→redo" matrix (60/75 editors, 67/70 viewers) is stale against the currently
  staged guests.** It was measured by S15 on W2's **05:58 restage** (`wp-s15.md` §6). Since then, **W2's restage4
  (finished 18:56, `consistent=60 diverged=0`, receipt confirms 60/60 fresh)** folded in **T12's fixes for every one
  of the 15 FAIL rows** (§S of `wp-t12.md`: stdio ×9, sourcing/demonstrator curation ×2, trinity jack/rewriting ×2,
  norm din18599/en1990 ×2, plus the 3 FAIL viewers — gis `setCamera`, procedural `generation2d`, vcs empty tree).
  T12's fixes are proven **compiled** (native `--lib --tests` green per crate, one wasm32-wasip2 check of all 15
  touched guest plugins EXIT 0) but **not yet re-verified live inside a running `s` browser session** — S15's own
  session-12 status table lists this re-run (`S12-1`) as still "booting serve 6540", not completed. **Treat "75/75
  editors, 70/70 viewers" as plausible, not proven**, exactly the same evidentiary gap the session-11 audit flagged
  for the 30/35→35/35 transition.
- **Session-11's stale-claim corrections still hold and are now further corrected forward**: `audit-s11-os-frontend.md`'s
  claim "stdio is N/A — I/O library, no standalone UI app" is **itself now stale** — S15's session-11 sweep found
  stdio has 9+ editor *programs* inside `s` (csv/tsv/txt/md/html/json×2/xml×2), all FAIL-then-fixed by T12. Block's
  "zero committed MCP descriptor" (09-18) is **superseded**: W2's log shows block has a committed descriptor with 3
  owned + 1 unowned (`kit.catalog`) codec row and all three open targets (2d/3d/5d.block) live in catalog B's 12/12
  open-plan probe.
- **Catalog membership as of audit time**: the **B list** (9 packages: stdio, gis, note, animate, block, writer,
  draw, puzzle, wfc) is mid-release toward **B2** (W2 S12-1, `RUNNING`); the other **25 plugins** (mathematical,
  procedural, flow, vcs, shooting, demonstrator, sequence, fem, architect, process, lowpoly, reasoning, forms,
  layout, cad, norm, playbook, imperative, remodel, energy, trinity, dag, raster, space, sourcing) are **all-only**,
  queued behind B2 (W2 S12-2, `QUEUED`). **Zero packages are published on any hub right now.**
- **stdio is the one plugin with no hub open target by design** (its 26 kinds are plugin-level `stdio.<x>`; its
  editors' dialects are `s.stdio.<x>`, an undeclared pairing) — recorded by W2 as intentional, not a bug, but it
  means stdio's kinds can be edited locally inside `s` but **can never be created as a hub document**.

---

## 1. Census — every plugin × staging / kinds / editors / viewers / catalog / matrix / owner

Legend: **Staged** = wasm on disk per the 18:55:59 receipt (all 60 rows carry this; not repeated per row unless a
plugin is an exception). **Kinds** = artifact-kind count as annotated by `audit-s11-os-frontend.md` §1 (not
independently recounted from the registry this pass — flagged where relevant). **Editor/Viewer** = PASS/FAIL from
S15's last live matrix (`wp-s15.md` §6, W2's **05:58** restage — STALE, see §0) with en/de identical except raster's
one label (see §3). **Catalog** = B (published toward B2, `RUNNING`) or all-only (`QUEUED`). **Owner** = who holds
the open item as of this audit.

| Plugin | Kinds | Editor (S15 matrix) | Viewer (S15 matrix) | Catalog | Open defect / owner |
|---|---|---|---|---|---|
| ✒️writer | 1 | PASS | PASS | **B** | Real edit verbs (`textEdit`/`setText`) `in_palette:false`, only typing exercises them (audit-s11, UNVERIFIED since — by design per S13) |
| ➗️mathematical | 1 | PASS | PASS | all-only | 10 mutation kinds lack 3rd-party oracle (audit-s11, UNVERIFIED since) |
| 🀄️wfc | 5 | PASS | PASS | **B** | `test-quick` was red on `descriptor_is_fresh` pre-restage4 (T12 §Status 8, stale committed descriptor) — restage4's describe-all should have refreshed it; **not re-confirmed post-restage4** |
| 🌀️procedural | 2 | PASS | **FAIL** (generation2d: guest panic `ordered-map root must be explicitly retired before drop`) | all-only | **T12 fixed at root + law** (viewer owner catalogue declared); compiled/wasm32-checked, **not live-re-verified** (S15 S12-1) |
| 🌊️flow (+9 ext.) | 1 (+9 ext) | PASS | PASS | all-only | none open |
| 🌍️gis | 2 | PASS | **FAIL** (viewer drops `setCamera`) | **B** | **T12 fixed at root + law** (retained camera window-config lane); compiled, **not live-re-verified** |
| 🌿️vcs | 1 | PASS | **FAIL** (empty history tree) | all-only | **T12 fixed at root + law** (document's own row); compiled, **not live-re-verified**; kind id unified to `s.vcs.vcs` (W2 §2.1) |
| 🎞️animate | 1 | PASS | PASS | **B** | none open |
| 🎥️shooting | 1 | PASS | PASS | all-only | none open |
| 🎪️demonstrator | 1 (6 sub-apps) | **FAIL** (curation) | PASS | all-only | **T12 fixed** (numeric arg types); compiled, **not live-re-verified** |
| 🎬️sequence | 1 (8 steps) | PASS | PASS | all-only | 4/8 step kinds uncovered by csv oracle (audit-s11, UNVERIFIED since); 8 sequence carrier-fixture contract rows still open, owned by "sequence owner" (T12 §4, unassigned to a session-12 slice) |
| 🏗️fem | 2 | PASS | PASS | all-only | txt import/export unimplemented (by design, audit-s11) |
| 🏛️architect | 1 | PASS | PASS | all-only | `setAdjacencyKind` duplicate-capability-id MCP blocker (audit P1-6 → G10) — **no session-11/12 report confirms fix or re-check; UNVERIFIED, still open**; stitched `ArtifactKindSpec` fixed by W2 (03:50) so it now has an open target |
| 🏭️process (+4 ext.) | 1 (+4 ext) | PASS | PASS | all-only | none open |
| 💠️lowpoly | 1 | PASS | PASS | all-only | `test-quick` red on `descriptor_is_fresh` pre-restage4 (T12); should be refreshed by restage4's describe-all, **not re-confirmed** |
| 💡️reasoning | 1 | PASS | PASS | all-only | none open at editor/viewer level |
| 📋️forms | 1 | PASS | PASS | all-only | none open |
| 📏️layout | 1 | PASS | PASS | all-only | none open (dag/other's small pan fix does not apply here) |
| 📐️cad (+4 ext.) | 1 (+4 ext) | PASS | PASS | all-only | F2 (T12): no wire vector while pane child materialization is absent from every codec — real fix needs a persisted child resolver, explicitly deferred past the freeze |
| 📕️norm | 15 | **FAIL** (din18599, en1990) | PASS | all-only | **T12 fixed at root + law** (snapshot decode arm); compiled, **not live-re-verified** |
| 📖️playbook (+1 ext.) | 1 (+1 ext) | PASS | PASS | all-only | none open |
| 📜️imperative (+5 ext.) | 1 (+5 ext) | PASS | PASS | all-only | none open |
| 📸️remodel | 1 | PASS | PASS | all-only | wall-clock laws fixed by T12 (root cause: `StepOverrunLedger` semantics, not per-reading) |
| 🔋️energy | 1 | PASS | PASS | all-only | none open |
| 🔱️trinity | 2 (jack/rewriting) | **FAIL** (both) | PASS | all-only | **T12 fixed** patchNodes/rail verbs; **rewriting's window vocabulary still unregistered** (F1: taxonomy move needed, routed to a post-publish rename slice) |
| 🕸️dag | 1 | PASS | PASS | all-only | horizontal two-finger pan **fixed** (U5 §6a, wasm rebuilt) |
| 🖍️draw | 1 | PASS | PASS | **B** | one test-only `unimplemented!()` in an FSM host test (audit-s11, documented, non-blocking) |
| 🖨️raster | 1 | PASS | PASS | all-only | History row "Hover" was untranslated German — **fixed in source** by U5, needs a descriptor regen (restage4 already happened, **not re-confirmed live**) |
| 🗄️stdio | 39 (26 plugin-level kinds) | **FAIL** ×9 (csv/tsv/txt/md/html/json×2/xml×2) | n/a (no standalone viewer) | **B** | **T12 fixed at root + law** (kit-verb registration + per-editor publication authority); compiled, **not live-re-verified**; **no hub open target, by design** (plugin-level `stdio.<x>` vs editor dialect `s.stdio.<x>`, undeclared pairing) — the one plugin that can never be created as a hub document |
| 🗒️note | 1 | PASS | PASS | **B** | second-window commands (`note-navigator`) refused `command owner mismatch` on a hub document — **C10, investigating (session 12)** |
| 🧩️puzzle | 3 (2d/3d/5d) | PASS | PASS | **B** | History rows en/de not measured live for puzzle (U5 §7, minor) |
| 🧱️block | 3 (2d/3d/5d) | PASS | PASS | **B** | audit-s11's "zero committed MCP descriptor" (09-18) is **stale/superseded** — W2 confirms a committed descriptor + 3 open targets |
| 🪐️space (home/space, host) | n/a (host) | n/a | n/a | all-only | Home's own space table: cause 1 (unauthenticated fold) **fixed + live**; cause 2 (≥9 spaces, 128-node budget) **fixed + tested**, live proof **pending** the space guest's restage — which restage4 (18:56) should carry (T12's `space_id` fix, per S15 session-12 header, landed 17:55) — S15 lists this as `S12-2 pending`, **not yet re-confirmed** |
| 🪵️sourcing (+3 ext.) | 1 (+3 ext) | **FAIL** (curation) | PASS | all-only | **T12 fixed** (same curation fix as demonstrator); compiled, **not live-re-verified**; curation verbs are `in_palette:false` by design (S13) |

**Extensions (26, not broken out)**: all staged per the 18:55:59 receipt; every one's runtime correctness is gated
on its parent plugin's row above; none has an independent boot-in-`s` or mutate/undo/redo proof in any source read
this pass (same finding as `audit-s11-os-frontend.md` §1) — **UNVERIFIED individually**. Cross-check on disk: of the
26 extension `*_component.core.wasm` files under the `s` host's own serving path
(`dist/runtime/react/dev/s/extensions/*`), only **2 have an mtime inside restage4's window (18:15–18:55)**; the
other **24 predate it** (09-24 19:47 through 09-25 17:18). This is most plausibly restage4/Nx skipping a byte-identical
recopy for extensions whose source did not change (consistent with W2's `consistent=60 diverged=0` verify, which is
a content-hash check, not an mtime check) — but **I did not run the verify script myself, so I cannot rule out a
staleness bug**; flagged **UNVERIFIED**, worth a one-line confirmation from W2 or S15.

---

## 2. Artifact kinds with no editor, no viewer, no open target, or no create path in `s`

1. **stdio's 26 plugin-level kinds** — have local editors inside `s` (the 9 kit-verb editors above, now fixed) but
   **no hub open target** (W2 §2 item 5, recorded as intentional: no declared pairing between plugin-level
   `stdio.<x>` and editor dialect `s.stdio.<x>`). **This is the one plugin that can never be created as a hub
   document**, only opened locally or read via a foreign document's stdio-backed subset. Not disputed by any
   report as a bug; listed here because it is a genuine "no create path over the hub" gap for outcome 1's "all
   artifacts" bar.
2. **`gis` viewer, `procedural/generation2d` viewer, `vcs` viewer** — had no working viewer as of the 05:58 restage
   (§1); T12 claims all three fixed at the root with laws, folded into restage4, but **no live re-verification
   exists yet** (S15 S12-1). Until that re-run lands, these three remain **unverified-fixed**, not confirmed-PASS.
3. **`trinity` rewriting's window vocabulary** — still genuinely unregistered (F1, T12): its state-lane config sits
   outside the surface-grammar taxonomy (`✏️editor/🪟️window/🎚️config` instead of
   `🎭️modes/<mode>/🪟️windows/<window>/🎚️config`), so no vector/oracle exists for it at all. Routed to the post-`--packages
   all`-publish rename slice, not scheduled inside any session-12 slice's current scope.
4. **`cad`'s object kinds (F2, T12)** — no wire vector can exist while pane-child materialization is absent from
   every codec; the real fix (a persisted child resolver) is explicitly deferred past the freeze, so this is a
   known, accepted, open gap, not scheduled this session.
5. **A plugin the local build does not list at all (no registry row)** — `wp-s15.md` §10 "Remaining gaps" (1):
   `installPlugin` refuses `missing-registry`; the hub source lists installed bundles but the shell registry does
   not merge them. This is a real "no create path" case for any plugin present in a hub catalog but genuinely
   absent from a given device's local build — narrower than, but adjacent to, the original "a plugin never staged
   locally cannot open a hub document" gap (which **is** now solved by option (a), the hub-served plugin-module
   bundle + durable Cache-Storage store, both measured live in session 11).

No plugin/kind was found with **zero editor and zero viewer both** — every plugin in the census has at least one
working surface (editor or viewer) as of the 05:58 measurement; the FAIL rows above are single-surface failures,
not total absence.

---

## 3. Cross-cutting gaps: lazy install, localization, accessibility, customization, devices, progress/cancellation

### Lazy install / hub-module resolution

- **Largely built and measured live in session 11**, not a gap in the "missing" sense any more: hub-served,
  content-addressed plugin-module bundles (schema v3, `wp-s15.md` §10), a hub-backed `PluginSource` with byte
  progress + cancel, catalog-generation pinning (a hub document always runs the bundle of the catalog that serves
  it, §13), and a durable Cache-Storage store with re-verify-on-load, eviction/reinstall notices, GC (§12) — all
  measured on hub **8040** in session 11 with real network-byte counts (24.5 MB download → 0 bytes on a later
  session, evicted-core reinstall, band cancel).
- **What remains open**: (a) proof on the **canonical** hub 7800 never completed — "S15's own watcher waits for the
  7800 rebuild" and died with the 18:xx crash (handover); session-12's W2 chain (S12-1/S12-2) is the blocker, still
  `RUNNING`/`QUEUED`. (b) the "no registry row at all" case (§2 item 5). (c) stdio has no hub open target by design
  (§2 item 1), so it can never exercise this path regardless.

### Localization (en + de, no default language)

- **Mostly DONE, corrects the stale audit-s11 claim of "2690/2795 hard-coded English strings, history panel
  English-only".** U5 measured 2800 label sites, found the German was present everywhere but frequently
  linguistically wrong (domain terms, compounds, word order — `wp-u5.md` §1.1), handcrafted ~930 corrections across
  60 crates, and the post-fix audit shows **2698/2800 "ok"**, the rest deliberate (symbols/proper names, envelope
  delegations). Chrome strings (Marketplace, Tasks, plugin-recovery panel, route-not-found, 16 total) localized and
  lint now scans the whole shell, not just the renderer target. Live en/de History rows measured for 11 plugins
  (§1.4). ShellSync, TaskManager, Approval affordance all localized and measured live.
- **Residual gaps**: (1) the hub's creation-catalog kind picker labels **every** kind "Editor"/"Editor" (a manifest
  limitation — `ArtifactKindSpec.name` is one plain string, not per-locale) — H9 session-12 item `L`, status `TODO`,
  blocks a user from telling `2d.drawing` from `text.document` when creating a hub document. (2) `fallbackLng: "en"`
  remains a soft implicit default (defensible per AGENTS.md's "SHOULD use English first, then German", but
  technically in tension with "no default language" if ever hit by an untranslated key — G5/S15, unresolved by
  design, not scheduled). (3) puzzle's en/de History rows not measured live (U5 §7, minor). (4) architect's
  `setAdjacencyKind` MCP duplicate-capability-id (affects `capabilities_search` catalog-wide, not just architect) —
  **UNVERIFIED**, no report confirms fix or re-check since 09-18.

### Accessibility

- Keyboard: **DONE and measured** — Tab traversal restored shell-wide (0→18 stops, S15 §5.3), full node-graph
  keyboard nav (U4, session 10, still cited as current), the new windowed `Table`/`TableRow` ARIA grid with
  Up/Down/PageUp/PageDown/Home/End/Enter/Space/Right/Left and a localized live-region row-range status (U5 §6).
- Contrast: **DONE and measured** — default light+dark+mono palettes now pass WCAG AA on every previously-failing
  chrome pair, both appearances, with the third-party `color` package as the contrast oracle and a permanent law
  guarding regressions (S15 §5.1).
- **Still open, unowned in the current session-12 fleet**: native (non-browser) wgpu/winit accessibility — the
  DOM-mirror ARIA bridge exists only for the browser host; audit-s11 P2-12 routed this to "WG7 backlog", and the
  session-11 handover repeats it as WG7 backlog without a session-12 owner picking it up yet (WG7's report has not
  been touched in session 12 as of this audit — see §4).

### Customization

- Theme/appearance (dark/light/system) persists across reload via an event-sourced preference, **measured live**
  (S15 §5.5). WCAG-compliant palette customization is DONE (above).
- **Layout persistence remains UNKNOWN** — not covered by any source read this pass, session 11 or 12; same gap as
  `audit-s11-os-frontend.md` flagged, still true, still unowned.

### Devices (desktop > mobile > tablet)

- **DONE and measured**: three real viewports (1440×900 desktop, 375×812 mobile, 768×1024 tablet), no horizontal
  overflow on any, tablet is now **auto-detected** (`UI_TABLET_MAX_WIDTH_PX`, `useUiDevice()`) rather than a manual
  override only — corrects audit-s11 P2-10 as resolved. A tablet-specific footer-label defect (labels pushed
  Tasks/History off the bar at 768px) was found and fixed (U5 §5).

### Progress + cancellation

- **Mostly DONE, corrects the audit-s11 P1-5 "TaskManager built but never mounted" finding as stale.** `TaskManager`
  is a real mounted window (Running tasks + Actors sections, localized, ARIA `role=progressbar` with
  `aria-valuetext`, named Cancel buttons) — U5 §2. Plugin-install progress+cancel is live with byte counts (S15
  §4/§10/§12). Agent tool-call cancel had a real defect (a cancelled call still waited out a 120 s approval
  deadline) — **found and fixed** (gateway `ApprovalResolution::Cancelled`, settled in 364 ms, U5 §2). The
  execution-target notice gained a "Cancel opening"/"Öffnen abbrechen" control (S15 §4).
- **Residual**: no live spawned-job row was ever observed in the Tasks window (no retained job ran during any
  probe — covered only by an integration test, not a live capture, U5 §7). The approval affordance does not
  disappear correctly after a cancel (gateway sends no `ApprovalResolved` on cancel, so the UI affordance lingers
  until its own countdown) — routed to G10/WG lanes, still open per U5 §7.

---

## 4. Ranked gap list — P0/P1/P2

Owner column marks whether the item is inside that slice's **current session-12 scope** (per its own status table)
or **unowned/parked**.

### P0 — blocks "working `s` with all plugins/artifacts" outright

1. **No hub is live anywhere right now.** Every hub-document, lazy-install-on-7800, collaboration and undo-over-hub
   proof is blocked until a catalog publishes. Concrete: `w2-release-par.sh b-warm` (chain pid 83333) → publish
   `w2-catalog-b2` → hub 7800 on a fresh root → rest-warm 25 → `--packages all` → 7800 restart again.
   **Owner: W2, session-12 owned (S12-1 `RUNNING`, S12-2 `QUEUED`).**
2. **Re-run the full editor/viewer/en/de matrix against restage4 (18:56), which now carries T12's 8-plugin guest
   fix set (stdio, sourcing, demonstrator, trinity, norm, gis, procedural, vcs).** Acceptance: the previously-FAIL
   rows (§1) flip to PASS with a rendered body, not a ledger-only pass; en+de both captured. **Owner: S15,
   session-12 owned (S12-1, serve 6540 booting at audit time).**
3. **Hub-document undo/redo refused (`action-owner-mismatch`) and second-window commands refused
   (`command owner mismatch`, e.g. `note-navigator`).** Blocks the mutate→undo→redo definition-of-done for every
   collaborative/hub-backed document and for every multi-window default layout (block, note, draw, and others).
   **Owner: C10, session-12 owned ("investigating", items 1–2 of its status table).**
4. **DB I/O credit exhaustion made long-lived hub documents read-only after ~20 edits (or ~5 h on 7800).** A fix
   landed in HEAD per H9 (`WalTailWindow` + idle-document unmount), but the db suite, hub suites and growth e2e have
   **not been re-run** since. Without this, no plugin's artifacts survive real, sustained collaborative use over a
   hub. **Owner: H9, session-12 owned (item `g`, `IN PROGRESS`).**

### P1

5. **stdio has no hub open target by design** — 26 plugin-level kinds can be edited locally but never created as a
   hub document. Not disputed as a bug by W2, but it is a real "all artifacts, all the way through the hub" gap for
   outcome 1. **Owner: unowned this session** (would need a stdio-editor dialect decision from a plugin-contract
   slice, e.g. T12 or W2, not currently scoped).
6. **The hub creation-catalog kind picker labels every kind "Editor"/"Editor"**, so a user cannot tell `2d.drawing`
   from `text.document` when creating a document. **Owner: H9, session-12 owned (item `L`, `TODO`).**
7. **A plugin absent from the local registry cannot be installed from the hub at all** (`missing-registry` refusal;
   `wp-s15.md` §10). **Owner: unowned this session** (S15's session-12 scope is the matrix re-run and the space
   index link, not this).
8. **architect's `setAdjacencyKind` duplicate-capability-id MCP blocker** (09-18) — status since is **UNVERIFIED**;
   if still present it degrades `capabilities_search` catalog-wide. **Owner: nominally G10 (audit P1-6), not visible
   in G10's session-12 scope** (G10's `wp-g10.md` has not been touched in session 12 as of this audit — see below).
9. **trinity rewriting's window vocabulary is unregistered** (F1) and **cad's object kinds have no wire vector**
   (F2) — both explicitly deferred past the freeze / to a post-publish rename slice. **Owner: parked** (rename
   slice, unscheduled in session 12's fleet table).
10. **Space/Home table's ≥9-space windowing fix needs its live proof** — mechanism landed and tested (U5), the
    space guest carrying the fix (T12's `space_id` link) should be in restage4, but the live 30-space proof has not
    re-run. **Owner: S15/U5, session-12 owned (S15's S12-2 lists the guest-link half as "pending"; U5 has not yet
    resumed in session 12 — its report is still time-stamped 07:53, before the fleet relaunch).**

### P2

11. **Approval affordance does not clear itself after a cancel** (no `ApprovalResolved` sent by the gateway on
    cancel). **Owner: G10/WG lanes, unscheduled in session 12's visible scope.**
12. **Native (non-browser) wgpu/winit accessibility story does not exist.** **Owner: nominally WG7 backlog; WG7's
    report has not been touched in session 12 as of this audit (`wp-wg7.md` last edited 17:41 on 09-25, i.e.
    session 11) — unowned in practice right now.**
13. **Layout persistence (window arrangement across reload) is UNKNOWN** — no report, session 11 or 12, has ever
    measured it. **Owner: unowned.**
14. **No live spawned-job row was ever observed in the Tasks window** (progress+cancel mechanism is real but
    untested against a live retained job). **Owner: unowned, minor.**
15. **Puzzle's en/de History rows and raster's fixed-in-source "Hover" label are not yet re-confirmed live** post
    restage4. **Owner: S15/U5, follow-on of item 2 above, not separately scheduled.**

---

## 5. Slices not yet visibly resumed in session 12 (as of this audit)

Per each report's own file mtime versus the fleet's ~23:00 launch: **W2, S15, C10, H9, R8** have already added a
`## Session 12` section and are actively working (confirmed live: W2's release chain pid 83333, S15's serve pid
85035/87082). **U5, T12, G10, WG7, WG8** show no session-12 edits yet (`wp-u5.md` 07:53, `wp-t12.md` 17:14,
`wp-g10.md` 15:43, `wp-wg7.md` 17:41, `wp-wg8.md` 17:28 — all session-11 timestamps). This matters for outcome 1
because several P1/P2 items above (architect's MCP duplicate id → G10; native wgpu a11y → WG7; puzzle/raster
locale re-confirm → U5) sit in those slices' backlog and have no visible session-12 movement yet. **UNVERIFIED
whether the coordinator has already messaged them** — not something a read-only pass can see.

---

## 6. Explicitly UNVERIFIED in this pass

- Whether the 24 extension `*_component.core.wasm` files with pre-restage4 mtimes under
  `dist/runtime/react/dev/s/extensions/` are content-identical to what restage4 would have produced (§1, tree
  check) — plausible (Nx/verify is content-hash based) but not independently confirmed by running the verify script.
- Every "T12 fixed, not live-re-verified" row in §1/§2 — compiled and native/wasm32-checked, not observed rendering
  inside a running `s` session by this audit.
- architect's `setAdjacencyKind` MCP duplicate-capability-id status since 09-18.
- Layout (window arrangement) persistence across reload.
- Whether U5/T12/G10/WG7/WG8 have been resumed by the session-12 coordinator (their reports show no edits yet).
