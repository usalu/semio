# S15 — `s` frontend (React shell) with all plugins and artifacts, as a user experiences it

Slice S15, session 11, 2026-09-25. Continues S3 (session 10, `.tmp-ticket-0918/📓️s3-foreign-kind-open-and-block-inside-s.md`),
S13/S14 (35-kind sweep), U4 (pinch / diagram / contrast), G5 (UX audit), peer audit G11 §C G1–G3, audit
`📓️audit-s11-os-frontend.md` §4 P0 1–3. Ports: serves 6540–6549, hubs 8040–8049. Captures: `wp-s15/generated/`.
Coordinator changes: the approval-surface unification, TaskManager/cancel UI, ShellSync + hub indicator, tablet
breakpoint, dag horizontal pan, Home space table and mutation-label German text moved to **U5** (`📓️wp-u5.md`).

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Status

| # | task | state | evidence |
|---|---|---|---|
| 1 | serve `dev s` (React) detached, census `PLAYGROUND_SESSION` | **measured**: 60/60 loaded, 148 programs, missing `[]` | §1 |
| 2 | every kind inside `s`: spawn → mutate → undo → redo, en + de, matrix | **measured, twice (09-24 guests and W2's 05:58 restage): editors en 60/75, de 60/75 (same 60, every History row German), viewers 67/70.** All FAIL rows are guest-side (stdio kit verbs ×9, trinity ×2, curation ×2, norm ×2, gis viewer, generation2d viewer panic) and requested with roots, + vcs viewer empty tree; 5 host root fixes landed | §2, §6 |
| 3 | block 2d/3d/5d render + verb; raster | **measured PASS** block 2d/3d/5d + raster (after the chip fix) | §3 |
| 4 | lazy install from hub catalog (hub 7800, W2 handoff) | **measured**: a hub document's actor installs from the hub catalog with progress + cancel and opens inside `s` (after the descriptor-admission fix); undo on the hub document refused (C10's `action-owner-mismatch` lane); a plugin never staged locally still cannot open (session needs the local module) — gap | §4 |
| 5 | UX bar: WCAG default light palette, en/de chrome, keyboard, mobile/tablet, persisted customization | **measured**: palettes AA (8 pairs fixed, both themes, both appearances); Marketplace/recovery chrome localized + lint now scans the shell elements; Tab traversal restored (0 → 18 stops), chip focus ring, focus moves into an opened program; phone/tablet usable, no horizontal scroll; appearance persists across reload | §5 |
| 6 | re-run full matrix after W2 full restage | **measured** on the 05:58 restage: en 60/75, de 60/75, viewers 67/70, S3 probe 5/5, refused de ✓ | §6 |
| 7 | option (a): trusted plugin module bundle — schema, hub routes, bootstrap, hub `PluginSource`, live proof | **measured live** on S15's hub 8040 (stdio+gis+note v3 catalog built by the bootstrap from this tree): note absent locally → opening a new hub note installs note from the hub (24.5 MB verified with progress) → opens → addBlock/undo/redo ✓; a later session on the same device installs from the device (0 network bytes, 12/12 module requests from the HTTP cache). Laws green (Rust 40/40 + 6, TS 36/36, kernel 89/89, quick 9/9). Found + fixed on the way: install-then-route read a stale plugin set. Canonical hub 7800 (W2's catalog B, published 11:44 with this step) serves 9 plugin modules, all 198 files re-verified by the shell's own verifier | §10 |
| 8 | G10 §9: transient boot error (~1 s) on `s` boots | **root-caused + fixed + probe**: boot install unstamped → connect-time snapshot replayed as a hot-swap of the host plugin; stamped installs drop the replay. Probe 0/16 error frames (was 4/4 without the fix); re-measured 17:3x on the current tree 0/8 | §11 |
| 9 | durable local-first store for hub-installed plugin modules (Cache Storage + service worker, re-verify every load, persist, eviction/quota notices en/de, GC) | **measured live** on 8040: every hub program commits to the store; a later session loads from it with **0** hub files; an evicted 14.9 MB core is reinstalled alone with the en/de notice; a band cancel at 19.4/29.0 MB commits nothing; laws os **459/459** | §12, §13 |
| 10 | design decision: a hub document's module is resolved by the serving catalog generation (hub programs `pluginId@bundleSha256`; staged module used only when byte-identical) | **measured live** on 8040 (catalog B) with STALE staging: draw, writer, note × en + de create → open → edit/undo/redo on the hub's module, **0** "document target changed"; identical staging → **local** (0 hub files); never staged → **hub**; later session → **store**. Laws: resolution fixture + Ajv/`node:crypto`, hub `extendsPluginId` 40/40, quick 12/12, engine 696/696. **7800 blocked** on W2's os-hub rebuild | §13 |
| 11 | shell refused catalog kinds whose kind id ≠ dialect (`2d.drawing` → `s.draw.drawing`); probe kind picker | **fixed + laws**; kind picker lists catalog B's 12 kinds and creates draw/writer/note | §13 |
| 12 | G10 4b / C10 F5: Space app index empty, `documents/index/socket-grants` 404 | **root-caused (3-link chain), host links fixed + laws** (space lane, full-history fold, index space fill; 0× 404 measured); guest link (index `space_id` never set → fold selects no space) **requested** from T12/W2 | §13 |
| 13 | re-run the full matrix after T12's guest fixes and W2's restage | **waiting**: W2 restage3 materializing at 17:5x; 7800 proof chain armed (auto-runs when 7800 serves the new index) | §6, §13 |

## 0. Infrastructure (measured)

| what | value |
|---|---|
| serve | `S_OS_PORT=6540 S_LOCAL_ONLY=1 NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:serve-s-react-dev --excludeTaskDependencies` (the registered nx serve target = `bun ./📜️script.ts serve s react dev`; `--excludeTaskDependencies` skips its `plugin-registry:session-s → generate` dependency, which is W2's). nohup pid **58164** (nx) → 59476 (`script.ts serve`) → vite **59531**; log `generated/s15-serve-6540.txt` |
| staged lane | receipt `…/dist/runtime/react/dev/s/activation/🔣️receipt.json` 2026-09-24 12:20, 60 plugins; **all 60 plugin dirs hold a staged `*core.wasm`** (mtimes 2026-09-24 19:46–20:17; only `🧵️shard`/`🪞️vendor` hold none, they are not guests) — S3's "15 deleted" is stale |
| browser | playwright chromium headless `--use-angle=metal`, 1440×900 |
| hub | none (local-only) until W2's hub 7800 handoff |

## 1. Serve + census (measured)

`generated/s3-foreign-kind-s15-census.txt`, `generated/s15-programs.json`: beacon `ready:s`;
`PLAYGROUND_SESSION.plugins.length = 60`, `variant s`, `hostMode true`; `__semioOsCatalogProbe` 60 rows, **60 `loaded`,
missing `[]`**, **148 spawnable programs** (35 plugins with programs, 80 editors + 68 viewers; `space` home/studio
excluded from the matrix as host chrome).

## 2. Every kind inside `s` — matrix (en, de, viewers measured)

Probe `wp-s15/s15-matrix.mjs` (new; imports the shared S6 witness from `.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs`
as a fresh module instance so per-kind pins reach it). One row per program, opened from Home through the palette
by the shell chord (never by clicking chrome), judged on: open · **body rendered** (not the skeleton, no window
fault, own content beside the chrome) · **every Actions/Utilities chip hit-testable** (`elementFromPoint` at the
chip centre is the chip) · one rail verb · undo · redo (ledger + `Check in (n)` + structural render digest) · the
verb's own History row read back in the run's locale · 0 fault lines · closed by its dock tab (`mode-dock-tab-close`).

Trial (`generated/s15-matrix-trial2.json`, after the two fixes of §2.1): **4/4 PASS** — block3d `addRepresentation`
`[0,1,0,1]` rail/rail `Add Representation↶`; dag `addNode` chord/chord `Add Node↶`; note `addBlock` rail/rail
`Add Block↶`; raster `addLayer` rail/rail `Add Layer↶`; 0 faults each; every window closed by its tab.

### 2.1 Root fixes (TS host / framework)

1. **Raster's `Actions` chips were dead to a pointer** (measured, `generated/s15-ghost-raster.json`, `s15-cover-raster.json`):
   `elementFromPoint` over both raster windows' `Actions` chip answered a transparent `absolute inset-0 z-30`
   `DIV` — `🖌️Paint2dHost`'s full-bleed pointer overlay — because the host root was no stacking context and `z-30`
   out-ranks the window chrome panes at `--z-pane: 20`. Same defect `NODE_GRAPH_HOST_CLASS` fixed for the node graph
   on 09-14. Fixed at two levels: `PaneHost`'s root (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, the window body) is now
   `isolate`, so NO body content can out-stack the window chrome; and `PAINT_2D_HOST_CLASS` / `INK_CANVAS_HOST_CLASS`
   (new constants, emoji docstrings) isolate the two hosts that paint z-indexed interactive layers, so they cannot
   cover a floating pane either. Before: raster `no Actions rail row after unfolding` (`s3-foreign-kind-s15-g1.txt`);
   after: `addLayer` PASS (`s3-foreign-kind-s15-g1-raster2.txt`, screenshot `s3-s15-g1-raster2-raster-after-redo.png`
   shows the rail open and the new pixel layer painted in both windows).
2. **Closing a spawned program's last window always produced a refused dispatch** (measured on every row of
   `generated/s15-matrix-trial.json`: `noteShellCommand refused: instance-retired (user window=…)`). `onWindowClose`
   journals `shell.windowClose` into the owning program (routed by `windowId`) and then called `destroyApp` in the
   same tick, so the note always reached a retired instance. `noteShellCommand` now answers its ledger outcome and
   the last-window close retires the instance only after the note settled (`🏛️ShellHost/🟦️.tsx`). After: 0 fault
   lines on close (trial2).

3. **A spawned program whose default layout declares named instances opened EMPTY** (measured,
   `generated/s15-open-demonstrator-s-puzzle-puzzle3d-1-editor.{json,png}` before/after). The demonstrator's puzzle3d
   (default layout = the views `puzzle3d-main-top` / `puzzle3d-main-perspective` of kind `puzzle3d-main`) painted
   "Drag windows from Display in the navbar, or restore a saved layout." with `[data-window-id] = []`: the spawned
   seed renamed only window KIND ids, so every declared-instance leaf was pruned; the seeded active window stayed
   the raw `puzzle3d-main-top`, and every later window-scoped shell note in the whole session was refused with the
   user notice "That window is no longer open." (5 fault lines across the next 5 kinds in `s15-matrix-en1.json`).
   Fix (schema = the program's own `defaultLayout`): `frameworkLayoutDeclaredInstances` (ShellHelpers) reads the
   declared instances; `🪟️spawned-program` gains `SpawnedDeclaredInstanceV1`, `spawnedGuestWindowInstancesV1`, and
   `spawnedProgramWindowInstancesV1`/`spawnedLayoutRenameV1` take the declared instances; ShellHost uses them for
   the spawned windows (titles from the layout), the layout seed, the refresh request + `windowInstances`, the
   dispatch view state, the per-window arg staging, the close ladder and the agent census. After: `Top` +
   `Perspective` windows mount and paint the 3D scene; the guest answers for `puzzle3d-main-top`.
   Laws: fixture `🏛️ShellHost/🧫️fixtures/🪟️declared-instances.json` + 6 laws in `🪟️spawned-program-session`
   (incl. a REGRESSION law that reproduces the empty canvas against the real `Mode` renderer) → **43/43**
   (`generated/s15-law-spawned-program.txt`).

Typecheck `bun ./📜️script.ts typecheck` in `@semio-tech/framework-renderer-react`: after fixes 1–2 rc 0, 0 errors
(`generated/s15-typecheck-renderer-1.txt`, `tsc --listFilesOnly` lists every edited file); after fix 3 15 errors,
ALL in `🤖️AgentDelegations/` (a peer's in-flight story/test), 0 in any file this slice touched
(`s15-typecheck-renderer-2.txt`). Window chrome-stacking law (fixture `🪟️Window/🧫️fixtures/🪜️chrome-stacking.json`,
suite `🪟️Window/🧪️tests/🧩️component`, registered in the ui-react vitest config) → **4/4**
(`generated/s15-window-stacking-vitest.txt`); the live oracle is Chromium's own hit-testing, asserted per chip on
every matrix row.

4. **A spawned board program had no board session** (measured on `puzzle2d`/`puzzle5d`: 6 × `Error: The current app has
   no registered board session factory.`). `ShellHost` resolved the board-2d session factory against the HOST session
   (`space/home`), never against the focused spawned program. Now `resolveAppSurfaceSessionFactory(…, focusedProgram)`
   (declared right after `focusedProgram`). Law in `🪟️spawned-program-session` (REGRESSION + fixed, against the real
   resolver) → 44/44. After: puzzle2d `addNode` PASS, puzzle5d `addNode` PASS, 0 faults.

### 2.1b The matrix, en (measured, `generated/s15-matrix-en2.json`, table `generated/s15-matrix-en2-table.md`)

**60/75 editor programs PASS** — every one opened from Home, body rendered, every chip hit-testable, a real rail verb,
undo and redo through the rail (dag: chord) moving `Check in (n)` 0→1→0→1, the verb's own `…↶` History row, 0 fault
lines, closed by its dock tab. PASS: space, block 2d/3d/5d, animate, architect, cad, dag, draw, energy, fem 2d/3d, flow,
forms, gis map/terrain, imperative, layout, lowpoly, mathematical, note, playbook, playbook-module-procedural, procedural
2d/3d, process, puzzle 2d/3d/5d, raster, reasoning, remodel, sequence, shooting, vcs, wfc ×5, writer, demonstrator ×6
(playground, generation3d, cad, puzzle3d, process3d, gismap), norm ×13. Measured through 3 runs of the same probe
(first pass 42/75 → per-kind pins + fixes → resume 57 → 60); rows were re-run only after a root fix or pin.

| FAIL (15) | cause | owner |
|---|---|---|
| stdio csv, tsv | kit verb `set-cell` refused `interactive-job.missing-factory: typed command 'set-cell' has no exact controller/owner/factory/tool/schema proof` | guest (stdio typed-operation registration) — requested |
| stdio txt, md, html | same for `replace-text` | guest — requested |
| stdio json ×2, xml ×2 | `set-node` dispatches, moves nothing | guest — requested |
| sourcing/curation, demonstrator/curation | `curationSetCount` journals a non-undoable row, no edit (staged guest predates S14's palette promotion) | guest restage — requested |
| trinity jack, rewriting | `patchNodes` moves nothing; `textSelect`/`nodeGraphEdit` refused for missing args | guest — requested |
| norm din18599, en1990 | `setSnapshot` with the standard's own `➡️after` fixture refused `invalid document text: expected Enum/Float, found Absent at 1:1` (13 other norm standards accept theirs) | guest — requested |

Requests appended to `.tmp-ticket/wp-w1/requests/s15.txt` (plus the puzzle3d `[DEBUG]` leftover that fires dozens of
times per second). The staged guests are W1's 09-24 build; H9's ABI change (13 → 14 owned exports) is not in them —
W2's restage re-runs this matrix (§6).

### 2.1c The matrix, de and viewers (measured)

**de** (`generated/s15-matrix-de1.json`, table `s15-matrix-de1-table.md`; locale seated through Settings → Sprache →
Deutsch, `lang=de` on every boot): **60/75**, exactly the en PASS set, the same 15 guest FAILs. Every PASS row's own
History row reads German: `Artefakt umbenennen↶`, `Griffart hinzufügen↶`, `Darstellung hinzufügen↶`, `Kachel
hinzufügen↶`, `Adjazenzart festlegen↶`, `Knoten hinzufügen↶`, `Schema ändern↶`, `Element hinzufügen↶`, `Auswahl
duplizieren↶`, `Schritt hinzufügen↶`, …, `Text festlegen↶` — no English mutation label left on a passing kind.

**viewers** (`s15-matrix-viewers1.json`, table `s15-matrix-viewers1-table.md`, on 6541): open from Home, body rendered,
every chip hit-testable, 0 faults, closed by tab — **67/70**. FAIL: `gis/gismap#viewer` (guest drops `setCamera`: the
viewer's map window declares no such action), `procedural/generation2d#viewer` (guest panic `ordered-map root must be
explicitly retired before drop`, `🌱️value/🗂️ordered/🦀️.rs:81`) — both requested; `vcs/vcs#viewer` paints an empty
`framework.window.tree` (0 text, 5 elements; the editor paints its counter) — not diagnosed (empty genesis history vs.
a projection gap), listed in §7.

### 2.2 Serve stability (measured)

The first full run (`s15-matrix-en1.json`) died after 19 rows on `Execution context was destroyed`: with HMR on, the
page reloaded 4× in 45 s while peers (U5) edited `ShellHost`/`TaskManager`/`Interpreter`
(`s15-open-demonstrator-…json` console, "Could not Fast Refresh … export is incompatible"). The serve now runs with
the vite config's own switch `SEMIO_VITE_HMR=0` (pid 92221 → vite 93208): every boot still transforms current
source, nothing reloads mid-row. The probe re-boots on any interruption and retries the row once, re-boots after
every failed row (so one bad row cannot poison the next), and `--resume` keeps PASS rows.

## 3. Block + raster (measured)

S3's own probe re-run on this serve (`generated/s3-foreign-kind-s15-g1.txt`, then raster alone after fix 2.1.1
`s3-foreign-kind-s15-g1-raster2.txt`):

| kind | windows | render | verb | edits | undo/redo | faults |
|---|---|---|---|---|---|---|
| raster | composite 1024×781 canvas + navigator 394×781 canvas | pixel layer painted after `addLayer` | `addLayer` | `[0,1,0,1]` | rail/rail | 0 |
| dag | `dag-main` 2 canvases + DSL window text | graph + DSL | `addNode` | `[0,1,0,1]` | chord/chord | 0 |
| block2d | `block2d-board` | `6 Handle Kinds, 11 Handles` | `addHandleKind` | `[0,1,0,1]` | rail/rail | 0 |
| block3d | `block3d-world` WebGL 1427×780 | mesh | `addRepresentation` | `[0,1,0,1]` | rail/rail | 0 |
| block5d | `block5d-board` + `block5d-world` | board + world text | `addGripKind` | `[0,1,0,1]` | rail/rail | 0 |

**Refused kind (measured, `generated/s15-refused-raster-{en,de}.json` + png):** probe `s15-refused.mjs` answers 404 for
every staged raster module except its descriptor (a guest that cannot be instantiated on this device) and opens raster
from Home: no window, no page error, one visible `role=status` notice `shell.spawnProgram.open-failed` — en
`“semio · raster” could not be opened.`, de `„semio · raster“ konnte nicht geöffnet werden.` (`lang=de` seated through
Settings). The same session opens the proven-interactive raster when the modules are served (above).

S3's "raster refused (wasm 404)" is superseded: raster's `core.wasm` is staged again and raster opens, renders and
round-trips inside `s`. Failed responses during the run: only `/__semio/agent-bridge` 404 (local-only serve has no
agent bridge) and `/🧩️extension-modules/watch` 404 — no plugin module 404.

## 4. Lazy install from the hub catalog (measured on hub 7800)

Second serve for the hub lane: `S_OS_PORT=6541 S_HUB_URL=http://127.0.0.1:7800 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev`
(the nx target's own command; nx refuses a second concurrent run of `serve-s-react-dev`, measured: "Waiting for
…serve-s-react-dev in another nx process"), pid 21695. Probe `wp-s15/s15-hub-journey.mjs` (S12's person-driven
journey — sign in as `user1@semio.dev` → hub workspace → open space → space index `createArtifact` (kind from the hub
catalog) → the creation saga opens it — instrumented with a recorder for every `[data-semio-execution-target-status]`
stage/progress and the creation notices). My space: **`S15 Space 743`** (`01a0d5e2-b97e-7c25-9bf3-6b3e038b60dd`).

**What "installs lazily from the hub catalog" means in this tree (read + measured):** for every hub document the
store worker already downloads the document's actor from the hub's trusted catalog — `open-plan` → execution-target
`manifest` → `component` (streamed, sha256 + blake3) → `descriptor` (canonical pack + field admission) → the
**closed browser actor** (`browser-actor`, streamed, verified, imported, decoded, compiled, described, verified,
opened, cold pair) — each stage published as `execution-target-status` with progress. The local plugin only provides
the shell session (app manifest, windows). The shell sets no `installedTarget`, so the actor ALWAYS comes from the
hub catalog.

| run | what | result |
|---|---|---|
| `hub2` (`s15-hub-document-hub2.txt`) | note document | manifest ✓ → component 14 939 324 B streamed ✓ → verify 2/3 → **alert "The document component could not be verified. Reopen the document."** → "The artifact was created, but it could not be opened." |
| root cause (`s15-note-descriptor-admission.txt`, `s15-catalog-descriptor-check.txt`) | replayed the worker's `parseVerifiedPackageDescriptorV1` field by field on the hub's lease manifest + `descriptor.semio` | every field ✓ except `manifest.artifactKinds has kind+schema` — the note descriptor declares `manifest.artifactKinds = []` and its editor app `artifactKinds = [s.note.note / note.document]`. Same for draw, writer, puzzle, raster, dag (declaration-tree plugins); only gis and stdio populate the plugin-level list. The hub's own pairing rule `app_opens_kind` (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1010`) admits app-declared kinds; the browser admitted the plugin-level list only |
| **fix** | `surfaceOpensArtifactKindV1` (`📇️directory/🧬️schema/🟦️.ts`, the browser twin of `app_opens_kind`) used by the worker's descriptor admission; shared fixture `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json` (6 cases) replayed by new suite `💻️os/🧪️tests/🗂️surface-opens-kind` → **7/7** (`s15-law-surface-opens-kind.txt`) | |
| `hub3`/`hub4` (`s15-hub-document-hub3.txt`, screenshot `s15-hub-document-hub3-created.png`) | same journey after the fix | descriptor 55 482 B ✓, verify 3/3 ✓, browser actor 20 374 253 B streamed + transferred + verified + imported, decode 14 886 046 B, compile, describe, verify, open, cold pair 958 B → **the hub note document opens inside `s`** (`semio · note`, windows `note-composite` + `note-navigator`, footer `live · 1 peer`, `Persisted`) |
| `hub5` (`s15-hub-document-hub5.txt`) | shared S6 witness on the opened hub document | `addBlock` **applies** (`edits 0→1`, ledger `Add Block↶`); **undo/redo refused**: `undo refused: dispatch-failed — action-state-unconfirmed`, then every dispatch `owner-mismatch — action-owner-mismatch`; every shell note `noteShellCommand refused: dispatch-failed — action-refused`. This is the browser-actor action lane C10 owns (C7's `action-owner-mismatch` family) — handed over, not fixed here |
| `hub-cancel` (`s15-hub-document-hub-cancel.txt`) | new **"Cancel opening" / "Öffnen abbrechen"** control on the execution-target notice (`ExecutionTargetStatusNotice` → `closeDocument(runtimeKey)` → worker `docAbort`) | control present from `manifest` through `component` 14.9 MB and `verify`; pressed at stage `verify` → download discarded, space index back, the saga reads "created, but it could not be opened. You can safely try opening it again." Law in `🧪️tests/⚡️quick` → **6/6** (`s15-law-quick.txt`) |
| `hub-unstaged` (`s15-hub-document-hub-unstaged.txt`) | `S15_BLOCK_PLUGIN=🗒️note` answers 404 for every locally staged note module (a device that never staged note) | boot census `note:failed`; the hub note document is created but **not opened**: `SemioFaultError: no surface registered for s.note.note@1/*#editor` (`resolveOpeningApp`) → "could not be opened". **Gap, not implemented:** the actor comes from the hub, but the shell SESSION (app manifest, window kinds, `createApp`) still needs a locally staged plugin module. See §7 |

**The remaining gap, precisely (read + measured, not implemented).** Lazy installation of the document's *actor* from
the hub catalog is live. What is not: a plugin whose module was never staged on this device. The shell SESSION
(`openArtifactWithAppRef` → `installPlugin(owner)` → `pluginSource.moduleUrl` → the shard worker's
`registerManifest`/`createApp`) always loads the jco plugin module (`🟨️.js` host shim + `…_component.js` +
`core.wasm`) from the LOCAL dev `PluginSource`; the hub serves no shard-loadable plugin module (its routes are the
execution-target `manifest|component|descriptor|browser-actor` of ONE document plus `🧩️extension-modules`). The hub's
closed browser actor (`semio.os.browser-jco-1.34.0-jspi.v1`) is a different module shape run in the worker's sandboxed
child, and it already renders the document window (`browser-actor-ui-patch-offer` → `UiDocumentStore`). Closing the
gap needs one of two designs, both beyond a host-only patch: (a) a hub route + schema for a trusted, content-addressed
**plugin module bundle** (same catalog generation, verified like the component) and a hub-backed `PluginSource`
(install on first open, `installing` progress + the existing install-band cancel), or (b) a **descriptor-backed session**
(`AppDefinition` from the verified package descriptor the worker already admits, windows rendered only by the browser
actor, no local `createApp`). (a) keeps every shell lane (other windows, panels, Actions rail) working; (b) is smaller
but leaves every non-document window of that program without a guest. Recorded for the coordinator as a design
decision; not started.

Note: my first `hub2` run created one note document in C10's space `C10 Studio 314491` before the probe preferred its
own space (the create-space form answered after the probe's 90 s window). It is an extra, harmless row there.

## 5. UX bar (measured)

### 5.1 WCAG AA on the default palettes (measured, fixed)

Census `wp-s15/s15-contrast-census.ts` (`THEME_CHROME_CONTRAST_PAIRS` on the default `semio` theme,
`generated/s15-contrast-census-before.txt`): light **3 failures** (U4's) — `mutedForeground/panel 2.34`,
`mutedForeground/base 3.54`, `activeForeground/activeHover 4.45`; dark **5 failures** — `foreground/hoverInteractiveFill
3.54`, `mutedForeground/panel 3.71`, `accentForeground/accent 3.22`, `activeForeground/activeBase 3.22`,
`activeForeground/activeHover 3.88`. The `mono` preset (`🌓️theme/🔣️.json`) failed the same pairs. Candidates graded
through the theme's own resolver (`s15-contrast-candidates.ts`, `generated/s15-contrast-candidates.txt`); the lightest
palette tokens that clear AA were chosen, identical token mapping for both themes:

| appearance | paint | before | after | worst pair after |
|---|---|---|---|---|
| light | `mutedForeground` | `gray` | `dark-dark-gray-2` | panel 5.53 |
| light | `activeHover` | `mix(primary, black, 0.9)` (darkened under dark text) | `mix(primary, light, 0.8)` (lightens) | activeForeground ≥ 5.36 |
| dark | `mutedForeground` | `gray` | `light-5-9` | panel 4.84 |
| dark | `accentForeground`, `activeForeground` | `light` | `dark` (as light already does on the red accent) | 5.36 |
| dark | `activeHover` | `mix(primary, black, 0.9)` | `mix(primary, light, 0.8)` | ≥ 5.36 |
| dark | `hoverInteractiveFill` | `gray` | `dark-4-7` | foreground ≥ 4.5 |

Regenerated through the permanent generator (`NX_DAEMON=false bun nx run @semio-tech/ui-styling-tokens:generate`,
`generated/s15-styling-generate.txt`; `check-generated` → fresh; the regenerated Rust tokens compile:
`cargo check -p semio-framework-ui-styling` rc 0, `generated/s15-cargo-check-styling.txt`). New law in the styling
suite: **every shipped theme passes AA on every chrome text/surface pair, both appearances, with the third-party
`color` package's WCAG contrast as oracle** (`s15-styling-contrast-law.txt` 4/4 of the pair group; full suite
`s15-styling-suite.txt` 66 pass / 1 fail = U2's documented pre-existing `panel-tab toggle dividers` assertion; styling
vitest `s15-styling-vitest.txt` 78/78).

### 5.2 en/de chrome completeness (measured, fixed)

Probe `s15-ux.mjs locale` (`generated/s15-ux-locale.json`, screenshot `s15-ux-locale-de.png`): Settings → General →
Language → Deutsch sets `lang=de`; every visible chrome text/aria/title compared en vs de. Strings identical in both:
`Chat`, `Name`, `Normal`, `System`, `Layout`, `Inline`, `Tooltips`, `Editor` (same word in German), the app ids and
breadcrumbs (data), and **`Marketplace`, `Tasks`**. Tasks had its key (`ui.panelToggle.taskManager` = `Aufgaben`)
but the tab node was memoized without the locale, so it kept the boot language — same for Marketplace and Chat; their
`useMemo`s in `🏛️ShellHost` now depend on `uiLocale` (after: `Marktplatz`, `Aufgaben`; re-measured
`s15-ux-locale.json`: only words that are identical German — `Chat`, `Editor`, `Name`, `Normal`, `System`, `Layout`,
`Inline`, `Tooltips` — and data remain). **Marketplace was also a hard-coded `uiDataLabel("Marketplace")`**. Reading `📌️ChromePanels` found 16 more hard-coded English chrome strings:
the whole Marketplace extension section (`Install extension`, `From URL`, `Install from URL`, the URL prompt, `From
file`, `Install from file`, `enabled/disabled`, `Enable/Disable`, `Uninstall`, `Marketplace unavailable`), the plugin
crash-recovery panel (`Plugin Recovery`, `This program crashed.`, `…quarantined…`, `Restart App`, `Disable Plugin`) and
`Route not found: …`. All now read compile-checked keys (`ui.plugins.marketplace*`, `ui.plugins.extension.*`,
`ui.plugins.recovery.*`, `ui.common.routeNotFound`) added to the schema (`📚️I18n/🟦️.tsx`) and to BOTH bundles
(`uiChromeTranslationBundles` de + en, normal + beginner tier; the bundle type is `satisfies Record<UiLocale, …>` so a
missing locale does not compile). **Why the lint missed them:** `check-chrome-i18n` scanned only the renderer's React
target, not `🧑‍🎨engine/🧱️elements` where the whole shell lives. The root now scans the elements too (tests and stories
skipped as fixtures; 3 allowlisted non-chrome files: the scoped-presence harness page, the retained UI intake's internal
diagnostics, the `"Interactive"` lane default) → `no chrome i18n violations` (`generated/s15-chrome-i18n-lint.txt`).
Renderer typecheck rc 0 (`s15-typecheck-renderer-4.txt`). No default language: the shell's `fallbackLng: "en"` remains
(G5 §1.4, compile-checked bundles make it unreachable for chrome keys).

### 5.3 Keyboard (measured, fixed)

Probe `s15-ux.mjs keyboard` (`generated/s15-ux-keyboard.json`, `s15-tab-diagnose.mjs`):

| check | before | after |
|---|---|---|
| Tab from Home | **0 stops** — 30 presses never left `mode-dock-panel-root`: `installElementsSurfaceBrowserDefaultSuppression` `preventDefault`ed every Tab outside a text field, shell-wide | **18 distinct stops** in document order (Artifact, Chat, Fullscreen, Editor/Viewer roles, Studios tab, Close, Home body, Create Space, Actions chip, Display, Remote, Sign in, Settings, Marketplace, History, Tasks, Command), cycling |
| visible focus on every stop | — | 17/18 → **18/18**: the docked `Actions` chip had none (unlayered `[data-window-silhouette-chip] > * { box-shadow: none }` beat its layered `focus-visible:ring`); new `:focus-visible` rule → `inset 0 0 0 2px #ff344f` measured |
| palette | `Meta+P` opens it, input focused, `dag` + Enter opens dag | same |
| focus after opening a program | `<body>` (next Tab restarts at the navbar) | the program's dock tabpanel; next Tab lands inside `dag-main` (the node graph's `application` stop) |

Fixes: `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (Tab suppression removed; chords still `preventDefault` in the shell's keybinding
loop), `🎨️styling/🖌️ui/🎨️.css` (chip focus ring), `🏛️ShellHost/🟦️.tsx` (focus effect, `SPAWNED_FOCUS_FRAME_BUDGET`).
Laws: the existing suppression law now asserts Tab is NOT prevented (`s15-law-tab-traversal.txt` 2/2); Window suite
+1 CSS law (`s15-window-stacking-vitest.txt` 5/5); spawned-program suite +1 (45/45). Typecheck rc 0
(`s15-typecheck-renderer-6.txt`).

### 5.4 Phone and tablet (measured)

Probe `s15-ux.mjs devices` (`generated/s15-ux-devices.json`, screenshots `s15-ux-{phone,tablet}-{home,program}.png`),
touch contexts: **phone 375×812** — Home window 357×702, no horizontal scroll (`scrollWidth 375`), palette by chord,
`note` opens as ONE full-width window (357×675, the mobile flat stack); **tablet 768×1024** — Home 750×914, note opens
as composite 536 + navigator 203 side by side, no horizontal scroll. 0 page errors. (Tablet is not auto-detected as a
distinct layout — U5's scope.)

### 5.5 Customization persists (measured)

Probe `s15-ux.mjs persist` (`generated/s15-ux-persist.json`, `s15-ux-persist-dark.png`): Settings → Appearance → Dark
→ `.dark` applied; stored as an event (`semio.os.config` → `os.config.ui-preferences` `{"events":[{"mutation":
"setAppearance","appearance":"dark"}]}`, event-sourced, not a CRUD value); **after a reload still Dark** (class + the
Settings control). Restored to System afterwards. Per user: the store is the device profile (`localStorage`); with a
hub session the same config travels in the user's session (not re-measured here).

## 6. Re-run on W2's one-tree restage (measured)

W2's restage (describe 60/60, generate + check green, `activate s react dev` verify 60/60 at 05:58, H9's 14-export ABI
in every guest). Both serves restarted after it (6540 nx pid 30143 → vite 30331; 6541 pid 30506 → vite 30515), both
print `[fresh] 60 staged components match their sources and the activation receipt`; census unchanged: 60/60 loaded,
148 programs (`s15-programs.json`, the 09-24 census kept as `s15-programs-0924.json`). **My serves never build**: their
logs hold no materialize/cargo/component line — the `[stale] … run: … activate` lines at a boot are the read-only
freshness report (the 05:39 note `component-dev` rebuild W2 saw was not from these serves).

| run | result |
|---|---|
| editors en (`s15-matrix-w2en.json`) | **60/75 PASS** — the same 60 as on the 09-24 guests (one first-pass `gis/gismap` FAIL was a contained guest `[DEBUG] close cleanup fault … zero-progress (9400 µs > 8000 µs)` at close under load ≈ 80; the resume PASSes it, 0 faults). The same 15 FAIL: stdio ×9 (kit verbs still `interactive-job.missing-factory` — root named in the request: no typed-operation tool registration exists for `set-cell`/`replace-text`/`set-node`), curation ×2, trinity ×2, norm din18599/en1990 |
| S3 probe (`s3-foreign-kind-s15-w2.txt`) | **5/5 PASS**: raster `addLayer` (composite 1024×781 + navigator canvases), dag `addNode` (chord), block2d `addHandleKind`, block3d `addRepresentation` (WebGL 1427×780), block5d `addGripKind`; 0 faults |
| refused (`s15-refused-raster-de.json`) | `„semio · raster“ konnte nicht geöffnet werden.` (`lang=de`), no window, no page error |
| editors de (`s15-matrix-w2de.json`, table `s15-matrix-w2de-table.md`) | **60/75 PASS** — the identical set; every passing kind's History row is German |
| viewers (`s15-matrix-w2viewers.json`) | **67/70 PASS** — the same three FAIL as on the 09-24 guests (gis viewer `setCamera`, generation2d viewer guest panic, vcs viewer empty tree) |

**Verdict for outcome 1 on the restaged tree:** 60 editor kinds + 67 viewer kinds of all 35 plugins open inside ONE
served `s` from Home, render, answer a real verb with undo/redo in en and de; raster and 🧱️block 2d/3d/5d proven; a
refused kind answers a localized notice. What still fails is guest source (18 rows, requested with named roots) and the
hub-only-plugin gap of §4.

## 7. Honest gaps

- **vcs viewer** paints an empty `TreeWindowKit` history tree (screenshot `s15-stack-vcs.png`): a fresh viewer has no
  history, and the kit renders no empty-state text (the editor's history window prints `—`). Reading, not a host
  defect; not fixed.
- **Soft freeze** (coordinator, ~06:30): nothing in this slice touches the guest ABI, pack schemas or codec hashes —
  all changes are TS host / styling / tests.
- **15 editor kinds FAIL for guest reasons** (§2.1b) — requested, not rebuilt by me (W2 owns restage).
- **Undo/redo on a hub-opened document** refused (`action-state-unconfirmed`, then `action-owner-mismatch`) — the
  browser-actor action lane (C10 / C7's `action-owner-mismatch` family); `addBlock` itself applies.
- **A plugin never staged locally** cannot open a hub document (§4 gap, design decision needed).
- **`worker` suite**: `bounds retained document backbone bytes until terminal Ack…` times out at 5 s with AND without my
  admission change (baseline run `s15-worker-vitest-baseline.txt`) — pre-existing under load, not mine; 121/122 others pass.
- **os typecheck**: 2 errors in peers' files (`🌉️mcp/…/resolvemcpbinarypath`, `🔗️hub-projection`), 0 in mine
  (`s15-typecheck-os-1.txt`). Renderer-react typecheck rc 0.
- **Styling suite**: 1 pre-existing failure (U2's `panel-tab toggle dividers`). `check-no-px`/`check-no-raw-colors`
  report 7/207 pre-existing hits, none in files I touched (`s15-styling-lints.txt`).
- **HMR off** on my serves (`SEMIO_VITE_HMR=0`): every page load transforms current source; a peer edit reaches the
  matrix at the next boot, never mid-row.
- The probe's neutral dispatches (`Clear Selection`) are journaled rows too; `historyRows.verbRows` filters them.
- `space/space` scores `createArtifact` `[1,1,0,1]` — its rename is what round-trips (S11 §3.3); PASS by the witness.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | `PaneHost` root `isolate`; Tab traversal no longer suppressed; de + en bundle keys `ui.plugins.marketplace*`, `ui.plugins.extension.*`, `ui.plugins.recovery.*`, `ui.common.routeNotFound` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` | schema for the new keys |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` | docked-chip `:focus-visible` ring |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`, `…/🌓️theme/🔣️.json` | AA chrome paints, light + dark, semio + mono (generated artifacts regenerated via nx) |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` | shipped-theme AA law with the `color` oracle |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧩️component/🟦️.tsx` (new), `…/🪟️Window/🧫️fixtures/🪜️chrome-stacking.json` (new), ui-react vitest config | chrome-stacking + chip focus laws |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | suppression law: Tab is NOT prevented |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | `check-chrome-i18n` scans `🧑‍🎨engine/🧱️elements`, skips tests/stories, 3 allowlisted non-chrome files |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx`, `🖋️InkCanvasHost/🟦️.tsx` | `PAINT_2D_HOST_CLASS`, `INK_CANVAS_HOST_CLASS` (isolated roots) |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | close note settles before `destroyApp`; declared instances (windows, seed, refresh, dispatch, staging, close, census); board factory for the focused program; focus into an opened program; execution-target cancel wiring; panel tabs relabel on locale |
| `…/🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts` | `SpawnedDeclaredInstanceV1`, `spawnedGuestWindowInstancesV1`, declared-instance parameters |
| `…/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx` | "Cancel opening" / "Öffnen abbrechen" on the execution-target notice |
| `…/🧱️elements/🏛️ShellHost/🧫️fixtures/🪟️declared-instances.json` (new) | declared-instance fixture |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `frameworkLayoutDeclaredInstances`; `sessionWindowInstances` takes `{id, windowKindId}` |
| `…/🧱️elements/📌️ChromePanels/🟦️.tsx` | 16 hard-coded chrome strings → i18n keys |
| `…/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx`, `…/🧪️tests/⚡️quick/🟦️.ts` | +8 laws (declared instances, board factory, focus, cancel) |
| `💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` | `surfaceOpensArtifactKindV1` (browser twin of the hub's `app_opens_kind`) |
| `💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | descriptor admission uses the shared pairing rule |
| `💻️os/🧪️tests/🗂️surface-opens-kind/🟦️.ts` (new), os vitest config | pairing law |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json` (new) | shared pairing fixture |
| ticket `wp-s15/*.mjs|*.ts` | matrix, hub journey, UX, census, diagnostics probes |
| `.tmp-ticket/wp-w1/requests/s15.txt` | guest requests (stdio, curation, trinity, norm ×2, puzzle3d `[DEBUG]`); option (a) heads-up, LANDED line, GIS bootstrap blocker |
| **option (a)** `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/{🔣️.json,🦀️.rs}` | 6 new `$defs`/projections, `pluginModule` required, `schemaVersion` 3, exports 25 |
| `…/🔏️trusted-catalog/🧩️plugin-module/🦀️.rs` (new) + `🧪️tests/🔬️unit/🦀️.rs` (new) | rules, canonical codec, content addressing, media types, assemble/write; 6 laws |
| `…/🔏️trusted-catalog/🧫️fixtures/🧩️plugin-module/🔣️.json` (new) | language-agnostic law (23 + 6 + 4 cases) |
| `…/🔏️trusted-catalog/🦀️.rs` | loader verifies + retains every module file; publication re-verifies; generation frames the record; `plugin_module_index`, `plugin_module(sha)`; fixture module writers |
| `…/🔏️trusted-catalog/🧪️tests/{🔬️unit,📤️publication}/🦀️.rs`, `🧫️fixtures/{👥️two-package,🧱️generation-stage,📤️publication,🔗️compiled-dependencies,🧬️stdio-gis-bootstrap}/🔣️.json` | v3 records, module hostiles, serving/tamper law, refreshed bootstrap fixture |
| `🌎️hub/🧪️tests/{🔬️bin-unit,🔏️trusted-catalog-profile}/🦀️.rs` | v3 bundles with modules; relocation copies `plugin-modules/` |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | routes `GET /trusted-catalog/plugin-modules[/{sha}[/{*path}]]` |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `trustedBootstrapPluginModuleV1` + fence, candidate staging, rotation carry, fixture proofs, v3 |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | `PluginSource.acquireModule` (async, progress, cancel, stamp), `PluginModuleUnavailableError`, multiplex fall-through, `pluginDescriptorUrl` |
| `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` | plugin-source laws on the new contract |
| `💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/{🟦️.ts,🧬️schema/🟦️.ts}` (new) | hub `PluginSource`; TS twin of the contract |
| `💻️os/🧪️tests/🧩️plugin-module-bundle/🟦️.ts` (new), os vitest config | fixture + Ajv oracle + hub-source laws (36) |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx`, `🐚️Shell/🟦️.tsx` | local-first multiplex with the hub source; awaited acquisition under the install abort; byte progress band; stamped installs (boot race); ref advances with the upsert |
| `…/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts` | +3 laws (boot stamp, progress text en/de, upsert); +4 (hub programs beside local ones + extension routing + invocation plugin id, space directory history, index opening space) |
| **store (§12)** `💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/{🟦️.ts,🗄️store/🟦️.ts,👷️service-worker/🟦️.ts,🧬️schema/{🔣️.json,🟦️.ts},🧫️fixtures/🗄️store/🔣️.json}` | Cache Storage store + service worker, re-verify every load, locks, persist, notices, GC; `installProgram` (store / local / hub) |
| `💻️os/🧪️tests/🗄️plugin-module-store/🟦️.ts` (new), os vitest config | store fixture replay + hub source flows incl. `installProgram` |
| `🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`, `🧑‍💻dev/🚚️distribution/📇️layout.json` | `Service-Worker-Allowed: /` for the store worker; distribution row |
| **resolution (§13)** `…/🌎️hub-source/🔍️resolution/🟦️.ts` (new), `…/🧫️fixtures/🔍️resolution/🔣️.json` (new), `💻️os/🧪️tests/🔍️plugin-module-resolution/🟦️.ts` (new) | hub program ids, staged roots, source decision, dialect owner, same-generation closure; law + oracles |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/{🧬️schema/{🔣️.json,🦀️.rs},🦀️.rs,🧩️plugin-module/🦀️.rs,🧫️fixtures/🧩️plugin-module/🔣️.json,🧪️tests/🔬️unit/🦀️.rs}` | index entry `dialectArtifactKinds` + `extendsPluginId` (required on the wire), validation, fixture cases |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, `…/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` | `ActivationRegistry.registerExtension`, `manifestPluginId` on activation manifest entries; cascade law |
| `…/🧱️elements/🔌️PluginRuntime/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx` | `loadPluginModule(…, manifestPluginId)`, `registerProgramExtensionV1` |
| `…/🧱️elements/🐚️Shell/🟦️.tsx` | `catalogModule`, `localProgramsV1`, `sessionProgramsV1`, `programPluginIdV1` |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | `installHubProgram`/`installHubDocumentProgram`, hub opening path, local-program aggregates, open-with over the closure, `invocationPluginIdV1`, `extensionProgramForV1`, creation kind≠dialect, space lane + history fold, `spaceIndexOpeningArgsV1` |
| `…/🏛️ShellHost/📇️space-directory/🟦️.ts` (new) | `SpaceDirectoryHistoryV1` |
| `💻️os/🟦️.ts`, `💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` | worker wire `directory-space-open/close`, `directory-space-events`, `parseDirectoryEventV1`; catalog/ready parsers no longer equate kind and dialect |
| `💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | space directory lane (sealed pages + global stream), TS routing |
| `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`, `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | laws: space lane wire, catalog-named ready/catalog, create + open `2d.drawing` |

## 9. Processes started

| pid | what | state |
|---|---|---|
| 58164 (→59476→59531) | first serve 6540 (HMR on) | stopped 01:20 by pid |
| 92221 (→ vite 93208) | serve 6540, `SEMIO_VITE_HMR=0`, local-only | stopped 05:39 by pid (restage) |
| 21695 (→ vite 21768) | serve 6541 → hub 7800, `SEMIO_VITE_HMR=0` | stopped 06:2x by pid (vite 21768 orphaned, killed by pid) |
| 9277 (→ vite 9500) | serve 6540 on the 05:36 staging | stopped 06:2x by pid |
| 30143 (→ vite 30331) | serve 6540 on the 05:58 restage | stopped ~07:05 by pid |
| 30506 (→ vite 30515) | serve 6541 → hub 7800 on the 05:58 restage | stopped ~07:05 by pid (vite killed by pid) |
| 86254 | second nx serve attempt (blocked by nx) | stopped by pid |
| 79802, 96056, 50185 | matrix runs en1/en2/resume | finished |
| 78877, 82632 | matrix `de1`, `viewers1` | finished |
| 636 (→ 639, 641 → vite) | serve 6540 local-only (boot probes) | stopped ~12:20 by pid |
| 83262 | hub mutex → plugin-module Rust laws | finished |
| 19337 | `s15-catalog-8040.sh` (wasm hold, bootstrap stdio+gis+note) | finished rc=1 at the GIS creation proof (see §10) |
| 27479 → 27482 | hold + os-hub 8040 (W2's hold script, own state dir) | stopped ~12:20 by pid (hub exits on pipe close) |
| 28372 (→ vite) | serve 6541 → hub 8040 | stopped ~12:20 by pid |
| 52664 | front 6542 (`s15-unstaged-front.ts`) | stopped ~12:20 by pid |
| 57502 (→ vite), 57503 | serve 6543 → hub 7800, front 6544 | stopped ~12:20 by pid |

Session 11 afternoon (running while §12/§13 are proven):

| pid | what | state |
|---|---|---|
| 47658 → 47661 | hold + os-hub 8040 (catalog B copy, binary from the tree with `dialectArtifactKinds` + `extendsPluginId`), data root `.🧬semio/🌐hub/s11-s15-hub-8040` | running |
| 53261 → 53295 | serve 6541 → hub 8040 (dev lane staged from the current tree = stale against catalog B) | running |
| 40510 → 40573 | serve 6543 → hub 7800 (waiting for W2's os-hub rebuild) | running |
| 53262 | front 6542 (`unregister` draw: a shell built without draw) | stopped 17:5x by pid |
| 7314 | front 6544 (`mirror` note: staged note = catalog B's bundle) | stopped 17:5x by pid |
| (chain) | `s15-proof-chain-7800.sh` (nohup): polls 7800's index every 30 s for ≤ 3 h, then runs draw + writer × en + de through serve 6543 | waiting for W2's os-hub rebuild |

Browser profiles of the proofs (a later session on the same device): `.🧬semio/🌐hub/s11-s15-profiles/<run>`.
Earlier in the session: no process of this slice was running at ~12:20 (checked: ports 6540–6544 and 8040 answered nothing). Restart recipe: `S_OS_PORT=6540 S_LOCAL_ONLY=1 SEMIO_VITE_HMR=0 NX_DAEMON=false bun nx
run @semio-tech/framework-os-dev:serve-s-react-dev --excludeTaskDependencies` (repo root); hub lane: `S_OS_PORT=6541
S_HUB_URL=http://127.0.0.1:7800 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev` in `🧑‍💻dev/📦️packages/🟦️typescript`;
then `bun wp-s15/s15-matrix.mjs http://127.0.0.1:6540/ --tag <t> [--locale de] [--roles viewer] [--resume]`.

## 10. Option (a): trusted plugin module bundle (coordinator decision 07:4x)

**Contract (schema-first).** `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` gained
`TrustedPluginModulePathV1`, `TrustedPluginModuleFileV1`, `TrustedPluginModuleBundleV1` (the manifest:
`semio.hub.trusted-plugin-module/v1`, identity, `sourceComponentSha256`, `sourceDescriptorByteSha256`, `moduleDirectory`,
`entry`, `files[{path, byteLength, sha256, blake3}]`), `TrustedBundlePluginModuleV1` (the package record) and the route
answer `TrustedPluginModuleIndexV1`/`…EntryV1`. `TrustedBundlePackageV1.pluginModule` is required, `schemaVersion` 2 → 3.
Rust projection in the hub schema (exports 19 → 25), TS projection
`💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts` (every bound read from the JSON module). Rules (both
languages): paths are 1–16 URL-safe segments, files strictly ascending by UTF-8 bytes, only under `<moduleDirectory>/` or
`🪞️vendor/`, entry = `<dir>/🌉️bridge.js`, `<dir>/🔣️.json` present, `<dir>/🛂️.descriptor.semio` present AND byte-equal to the
package's own trusted descriptor, identity + source digests equal to the package, manifest bytes canonical (compact JSON in
schema field order + `\n`).

**Language-agnostic law.** Fixture `🧫️fixtures/🧩️plugin-module/🔣️.json` (23 manifest cases, 6 index cases, 4 file-tamper
cases, canonical bytes + both digests, media types), generated by `wp-s15/s15-plugin-module-fixture.ts`. Replayed by Rust
`🧩️plugin-module/🧪️tests/🔬️unit` (6 laws, incl. assembling + writing the content-addressed module) and TS
`💻️os/🧪️tests/🧩️plugin-module-bundle` with **Ajv as the structural oracle** for every case: **36/36**
(`generated/s15-law-plugin-module-ts-2.txt`, incl. 4 hub-source laws below).

**Hub.** Generation layout: `packages/<id>/plugin-module.json` + every file once under `plugin-modules/<sha256>` (shared across
packages: vendored shims, fonts). The loader verifies the manifest (length, sha256, blake3, canonical, bound to the package)
and every file (sha256 + blake3), retains each as a re-verified `TrustedCatalogAsset`; publication re-verifies the manifest
and every file after candidate validation; the generation id frames the record (`path`, `byteLength`, sha256, blake3) —
the TS bootstrap frames it byte-identically. Routes (unauthenticated like the catalog they index, immutable cache):
`GET /trusted-catalog/plugin-modules` (index of the current generation), `…/{bundleSha256}` (manifest),
`…/{bundleSha256}/{*path}` (one file, reread and re-verified; a tampered file is never served). Laws: `cargo test -p
semio-hub --lib -- trusted_catalog` **40/40** (incl. the new loader/serving/tamper law and the publication leaf-substitution
law extended with `pluginModule`); bin check-in process fixture + native stdio readiness **5/6** — the 6th,
`space_artifact_creation_routes_…`, is the known audit P0-1 genesis failure (`📓️audit-s11-hub.md`), unrelated
(`generated/s15-hub-laws-2.txt`). `cargo check -p semio-hub --lib --bin os-hub --tests` with and without
`integration-fixtures`: 0 errors.

**Bootstrap.** `trustedBootstrapPluginModuleV1` (hub `📜️script.ts`): from the package's own fresh component and staged
descriptor — host shim, jco component module (`transpilePluginComponentAsync`, **no wasm-opt**), bridge, `🔣️.json` (exact JSON
projection of the trusted descriptor, the dev format), `🛂️.descriptor.semio` (the trusted bytes), preview2 shims, guestslim
fonts — then the core module is checked against the fresh receipt's `coreSha256` (jco's core extraction is name/map
independent: measured identical sha256 for both spellings on catalog A's note). The generation fence, candidate staging,
publication fence and the stdio+GIS rotation carry the modules (the rotation re-derives Stdio's module from its rewritten
descriptor; its stale `openTarget` summary was fixed to the multi-target form). TS proofs: `trusted-stdio-gis-bundle-check
--publication-source` rc 0 — generation-stage **33 cases** (5 new plugin-module hostiles: tampered file, tampered manifest,
missing file, unlisted extra file, post-read substitution) and publication fence 6 cases (+`plugin-module`)
(`generated/s15-publication-source-1.txt`). The `🧬️stdio-gis-bootstrap` fixture was stale before this change (its open target
had no `packSchemaHash`, its GIS map hash had drifted); refreshed through the script's own encoding and extended with the
plugin-module generation law. `--source` mode stops earlier on a peer's `framework.manifest/ArtifactKindFormatsFixture` drift.
**Measured inside W2's catalog B run (10:30–10:39, on this tree):** stdio, gis, note, animate each staged
`plugin-module.json` (stdio 22 files / 62.1 MB, gis 61.3 MB, `plugin-modules/` 32 shared blobs) before the run stopped at
block's own codec probe (`kit.catalog`, W2's lane, not the module step).

**Shell.** `PluginSource.moduleUrl` (sync) became `acquireModule(pluginId, rebuiltAt, {signal, onProgress}) →
{moduleUrl, rebuiltAt}`; a source that cannot serve a module throws `PluginModuleUnavailableError` and
`multiplexPluginSources` asks the next one (any other failure stays a failure). Local sources check the module is served
(`HEAD <dir>/🔣️.json`); the new `createHubPluginSource` (`🌎️hub-source/🟦️.ts`) reads the hub index through the shell's
same-origin hub mount `/_semio/hub`, downloads the manifest by its content address and every file with byte progress and
cancellation, verifies sha256 + blake3, records the verified bundle in persisted local-only storage and answers the
content-addressed module URL (relative `../🪞️vendor/…` imports and the fonts fetch resolve inside the bundle). A later
boot or a hub outage installs from the recorded bundle (the immutable responses are in the browser cache). It announces
nothing: a hub module installs when something opens it. ShellHost: `local…, hub` multiplex when the shell has a hub;
`installPlugin` awaits the acquisition under the existing install-band abort; the band shows
"Loading plugin note · 12.3 of 81.2 MB verified" / "Plugin wird geladen note · 12,3 von 81,2 MB geprüft" with a
`<progress>`. Laws: kernel plugin-source suite **89/89**, hub-source 4 laws (install once with progress, second install
reads the record, offline install from the record, unavailable without one, tampered file refused and not recorded,
cancel settles), renderer quick **8/8**, renderer typecheck rc 0.

**Live proof (measured).** Hub **8040** (`wp-s15/bin/os-hub`, the 10:30 tree build, ad-hoc signed copy; data root
`wp-s15/hub-8040`, one dev credential `user1@semio.dev` provisioned with the hub's own `credential set`; hold = W2's
`w2-hub-hold.ts`, own state dir `wp-s15/state-8040`) on a **stdio+gis+note schema-v3 generation built by
`trusted-catalog-bootstrap --packages stdio,gis,note` from this tree** (`wp-s15/s15-catalog-8040.sh`, one wasm hold:
components, descriptors, codecs, closed actors, plugin modules, generation fence 8/8, candidate hub started and loaded it,
GIS cold-map laws passed; the run then stopped at the GIS creation proof — see the blocker below — so `current.json` was
copied from the validated candidate). Hub ready in 21 s with the modules verified. Routes measured over HTTP: index (3
modules), note manifest by content address (5 721 B), all 22 files re-verified by sha256 (24 483 679 B), bridge served
`text/javascript; charset=utf-8`, an unlisted path 404. Serve **6541** (`S_HUB_URL=http://127.0.0.1:8040`,
`SEMIO_VITE_HMR=0`); `/_semio/hub/trusted-catalog/plugin-modules` answers through the dev proxy.

"Never staged on this device" is simulated by a same-origin front **6542** (`wp-s15/s15-unstaged-front.ts`): every local
`/🔌️plugin-modules/🗒️note/…` answers 404 and the dev watch stream's snapshots omit note; everything else (HTTP buffered, SSE,
the hub lane's websockets) passes through. No browser request interception — Playwright routing disables the HTTP cache,
which runs 5–6 measured (every module request hit the network). Probe `wp-s15/s15-hub-journey.mjs` (sign in → space →
`createArtifact` note → the creation saga opens it → S6 sweep witness), twice in ONE persistent browser profile, each with a
Chromium net log parsed by `wp-s15/s15-netlog-modules.py`:

| run | boot census | install band | module traffic (net log) | open | edit |
|---|---|---|---|---|---|
| `hub-module-10a` (fresh device) | `note:available` (not installed) | "Loading plugin note · 0.0 → 24.4 of 24.5 MB verified" + cancel | 35 requests: 33 network (25.0 MB = verification download), 2 from cache (the core module the verifier had just fetched) | creation saga opens `note-composite` + `note-navigator` | addBlock mutated, undo ✓, redo ✓ (edits 0→1→0→1) |
| `hub-module-10b` (later session, same device) | `note:available` | "Loading plugin note" (no download: the hub source read the index, found its recorded verified bundle) | **12 requests, 0 network bytes, all 12 from the HTTP cache (15.6 MB)** | opens | addBlock, undo, redo ✓ |

(`generated/s15-hub-document-hub-module-10{a,b}.txt`, screenshots `…-opened.png`, `…-mutated.png`.) Both runs also show the
execution-target lane's own statuses (manifest → component → verify → actor decode/compile/describe/verify/cold) ending in its
`renderer-unavailable` status, which the worker emits whenever the closed actor rendered no UI patch of its own
(`🏪️store/👷️worker/🟦️.ts:2564`); the document renders through the shell session the hub module now provides. That lane is C10's.

**Found and fixed on the way (measured).** Runs 1–3 downloaded and verified note but the opening failed with
`no surface registered for s.note.note@1/*#editor`: `installActivationOwnerAndResolve` awaits `installPlugin(owner)` and then
builds the router from `loadedPluginsRef.current` — which was assigned during render, one render behind the upsert, so the
freshly installed owner was missing. This also bit every opening whose owner was not yet resident, local or hub. Fix:
`upsertLoadedProgramV1` (🐚️Shell, the reducer's one upsert) and `installPlugin` advances the ref with the dispatch. Law in the
renderer quick suite (**9/9**); spawned-program suite 45/45; renderer typecheck rc 0; boot probe re-run 0/10 error frames
(`generated/s15-boot-probe-{s,hub}-3.json`). Run 4 (after the fix) opened and edited.

**Found and reported (not S15's):** every `trusted-catalog-bootstrap` on the current tree dies in the candidate proof at
`POST /spaces/{s}/artifact-creations` for `s.gis.gismap` → **409, empty body** (the proof parses before it checks the
status: `JSON Parse error: Unexpected EOF`; C10's 10:39 run died identically). Root (measured with
`wp-s15/s15-creation-diag.ts` on 8040): the generation carries TWO gis editor open targets for `s.gis.gismap`
(`s.gis.gismap@1/*#editor` and `s.gis.gisterrain@1/*#editor`), because the current GIS descriptor declares
`artifactKinds = [s.gis.gismap]` on the gisterrain editor app, and creation requires exactly one editor. Filed in
`wp-w1/requests/s15.txt` (11:05); W2 found the same at 11:11 and refined the one Rust pairing rule (a plugin-level kind is
opened only by the app whose dialect names it), then published catalog B at 11:44.

**Canonical hub 7800 (measured 12:1x).** W2's catalog B (generation `e8167ce8…`, built by this bootstrap step) serves 9
plugin modules; `wp-s15/s15-verify-hub-modules.ts` (the shell's own index/manifest/file verifiers over HTTP) re-verified all
**198 files, 0 failures**: animate 29.9 MB, block 27.4, draw 24.5, gis 61.3 (deps stdio), note 24.5, puzzle 38.7,
stdio 62.1, wfc 34.5, writer 29.0 (`generated/s15-verify-hub-modules-7800.txt`). The browser journey on 7800 did not reach
creation: with 12 creatable kinds the space index's staged `createArtifact` arguments stay folded and the probe finds no kind
option (probe mechanics, not the product); the 8040 runs above are the browser proof.

The 8040 data root, hold state and binary copy were deleted after the runs (the data root was inside the ticket folder,
which the auto-commit stages; nothing of it is needed again: `wp-s15/s15-catalog-8040.sh` rebuilds it).

**Remaining gaps:** (1) a plugin the local build does not list at all (no registry row) is still not installable from the
hub: `installPlugin` refuses `missing-registry`; the hub source already lists installed bundles, the shell registry does not
merge them. (2) The local copy is the browser's HTTP cache (immutable content-addressed responses) plus the recorded
verified bundle; the cache is evictable, and a first-visit shell needs the hub. A Cache-Storage copy served by a service
worker would make it independent of eviction; not built (the shard workers are created before a first-visit worker could
control them). (3) W2's final `--packages all` needs the GIS blocker above resolved first.

## 11. G10 §9 — the transient boot error (root cause, fix, probe)

**Probe.** `wp-s15/s15-boot-probe.mjs`: an init script installs a MutationObserver on the document before the app runs and
records every change of `data-semio-os-ready` / `data-semio-os-error` and every `[role=alert]` text with its time; a boot
fails on ANY error frame. It cannot miss a 1 s flash between samples.

**Root cause (measured).** Boot installs the host plugin (`space`) with `rebuiltAt = undefined`, so the shell recorded no build
stamp for it. The dev watch stream's connect-time snapshot then arrives with `rebuiltAt = <build time>`, and
`pluginAvailabilityRouteV1(alreadyLoaded, undefined, t)` answers **hot-swap**: the shell reloads the plugin that owns its own
session, retires the Home instance and re-creates it — and a refresh/intake still in flight on the retired instance surfaces
for ~1 s (`no actor for instance 1`, `plugin-ui.intake-rejected:intake:actor-activation.revoked`). Locale was a red herring;
it is timing (snapshot before vs after the boot install settles). A temporary `[DEBUG]` line proved it on every boot:
`space stamp=<boot version> event=<build time> route=drop unstamped-route=hot-swap`.

**Fix.** `acquireModule` answers the stamp the loaded module is at least as new as (a local source: the `rebuiltAt` it
cache-busts with, its boot version before any `built` event) and `installPlugin` records that stamp, so the snapshot of the
build a boot just loaded is a replay (`drop`) and only a strictly newer build hot-swaps. Law (renderer quick): "stamps every
boot install so the connect-time snapshot of the build it loaded is a replay, never a hot-swap".

**Measured** on serve 6540: with the fix **0 error frames in 16 boots** (`s` en×5 + de×5, `?plugin=note` en×3 + de×3;
`generated/s15-boot-probe-s-2.json`, `…-note-2.json`); the same probe with the stamp reverted **4/4 boots with an error frame**
(`generated/s15-boot-probe-s-unstamped.json`). The `[DEBUG]` line is removed.

**Sweep note (12:19–12:35).** The external low-disk cleanup deleted every capture cited in §10 and §11 (`wp-s15/generated/`). The numbers
above were read from them before the sweep. Re-measured on the current tree (17:3x): boot probe on serve 6541 **0 error frames in 8 boots**
(en×4, de×4; `generated/s15-boot-probe-s-3.json`); hub trusted-catalog laws **40/40** (`generated/s15-hub-laws-lib-2.txt`); the hub
proofs of §10 are superseded by §13 (same catalog B, hub programs).

## 12. Durable local-first store for hub-installed plugin modules (coordinator item 1)

**Mechanism (one).** Cache Storage cache `semio-plugin-module-store-v1`, content-addressed: `blob/<sha256>` (files shared across bundles),
`manifest/<bundleSha256>`, `record/<generationId>/<bundleSha256>` written LAST (a record always names a complete bundle). A module service
worker (scope `/`, `Service-Worker-Allowed: /` from the dev server and the distribution layout) serves `/_semio/plugin-modules/<generation>/
<bundle>/<path>` and re-verifies record → manifest (sha256) → file (length + sha256 + BLAKE3) on EVERY load; a corrupt entry is deleted and
answered 404. Web Locks: a shared `semio.plugin-module:<bundle>` per bundle a page runs, the exclusive `semio.plugin-module-store` for
commits and GC. `navigator.storage.persist()` after the first commit. Notices en/de: "reinstalling" (a stored bundle lost a file) and
"quota exceeded" (commit refused, nothing half-written). GC removes records of superseded generations no page holds, then unreferenced blobs.

**Schema-first.** `🌎️hub-source/🧬️schema/🔣️.json` (`PluginModuleStoreV1` constants, `PluginModuleStoreRecordV1`, `HubProgramIdV1`,
`PluginModuleSourceV1`), TS twin in `🧬️schema/🟦️.ts`.

**Laws.** `💻️os/🧪️tests/🗄️plugin-module-store/🟦️.ts` replays `🧫️fixtures/🗄️store/🔣️.json` (8 serve cases: every file, media types,
tampered file/manifest, missing file, other generation, unlisted path; 4 collection cases) with Ajv as the record oracle, plus the hub
source flows (install once with byte progress then 0 downloads; reinstall notice; offline from the device; quota notice; GC while held).
os suite **459/459** (includes the resolution law of §13).

**Live** (hub 8040, catalog B): every hub program in §13 lands in the store (`records: ["e8167ce8ed3e/<bundle>"]`, 22 blobs, page
controlled by the store's worker); the second-session and eviction rows are in §13's table.

## 13. A hub document runs on the module of the catalog generation that serves it (coordinator design decision)

**Rule.** A hub document's plugin module is resolved by the hub catalog generation that serves it, never by what the device staged. It runs
on its own **hub program** `pluginId@bundleSha256` (`HubProgramIdV1`), loaded from the store's verified copy of that generation's bundle,
beside the device's own program of the same plugin (local documents keep the local plugin). The bytes come from the first source holding
the bundle's exact content (`🌎️hub-source/🔍️resolution`): **store** (the generation's bundle is complete on this device), **local** (every
file of the bundle is byte-identical — length, SHA-256, BLAKE3 — in the staged module; compared module directory first, stopping at the
first difference), **hub** otherwise (differing, missing, or another entry). The hub program's closure is resolved in the SAME
generation: dependencies first, extensions (new index field `extendsPluginId`) indexed under their parent program before anything
activates, extension requests routed to same-generation peers.

**Host changes.** `loadPluginModule(programId, url, signal, manifestPluginId)` (descriptor identity checked against the plugin id, every
host key is the program id; `ActivationRegistry.registerExtension`, `manifestPluginId` in extension activation events); hub programs are
marked `catalogModule` and kept out of routing, the launcher, contributions, app registrations and the dialect index
(`localProgramsV1`), a hub session's contributions/"Open with…" come from its own closure (`sessionProgramsV1`); every invocation
addresses the descriptor's plugin id (`invocationPluginIdV1`) — the verified execution target refused the program id as
`command owner mismatch` (measured, fixed); an opening from a space index goes through `installHubDocumentProgram` (hub path when the
opening names a space) with the install band, byte progress and cancel of every install.

**Creation fixes found on the way (catalog B).** The hub catalog names a creation kind and the dialect that opens it independently
(`2d.drawing` → `s.draw.drawing`, `text.document` → `s.writer.writer`). Three shell rules demanded they be equal and refused catalog B:
the worker-wire catalog parser (the kind picker stayed empty — "Artifact kinds are unavailable", pageerror `invalid kind identity`), the
ready-status parser and `spaceArtifactCreationRequestFromAction`/`spaceArtifactCreationReadyOpening`. All removed; laws in
`🧪️backbone-envelope-io` (a catalog-named ready and catalog decode) and `engine-contract` (create + open `2d.drawing`). The probe opens
the select trigger by keyboard (`button#kindChoice`; a forced click landed on the tree row) and picks by index (the hub labels every
kind "Editor"/"Editor" — hub-side label defect, routed below).

**Space index empty (G10 4b / C10 F5, coordinator item).** Root cause is a chain of three: (1) the shell opened a document-scoped
directory stream for the pseudo-document `index` → hub 404 (no descriptor; and such a stream carries only that document's events);
(2) the guest folds the FULL history of its space but the shell sent single events; (3) the index document's `space_id` is never set in
the browser (local genesis `""`), so the guest's fold finds no space. S15 fixed (1) + (2) in the host: worker space lane
(`directory-space-open/close` → sealed `/directory/event-page/v1` from 0, then the global stream, `directory-space-events`), a per-index
seq-deduplicated history folded whole (`📇️space-directory`, law in quick), and an empty `spaceId` on openings and directory commands
from the index filled from its scope (`spaceIndexOpeningArgsV1`, law). Measured: 0× `documents/index/socket-grants` 404 (was 26 per
visit), the history arrives and is folded (8 → 11 events). (3) is guest source → requested (`wp-w1/requests/s15.txt` 16:3x, T12/W2).

**Laws.** `💻️os/🧪️tests/🔍️plugin-module-resolution/🟦️.ts` replays `🌎️hub-source/🧫️fixtures/🔍️resolution/🔣️.json`
(generator `wp-s15/s15-plugin-module-resolution-fixture.ts`): 12 source cases (matching, matching + extra files, differing core,
differing vendor, longer bridge, missing file, missing staging, other entry, stored-complete × 2, stored-incomplete + matching /
differing), 3 program ids + 7 hostile ids, 5 staged roots, 8 closure cases (dependency, extensions, cycle, missing), 4 owner cases;
oracles: Ajv (`HubProgramIdV1`, index) and `node:crypto` SHA-256 per staged file. Store law gains the three `installProgram` flows
(stale staging → hub, identical staging → local with 0 downloads, missing / other entry → hub). Hub index `extendsPluginId`: hub lib
`trusted_catalog` **40/40** (index fixture 13 cases incl. extension, extends outside dependencies, missing/blank extends). Renderer quick
**12/12**, renderer long (every engine suite) **2055/2055 in 105 files** (`generated/s15-engine-long-1.txt`), framework kernel **189/189** (+ `registerExtension` cascade law),
os **459/459**, `tsc` (os) clean except the pre-existing `🔗️hub-projection` test error.

**Live (hub 8040 = catalog B `e8167ce8…`, serve 6541 = dev lane staged from the current tree, i.e. STALE against catalog B: local draw
component `f95d2d6a…` vs catalog `476fe9c0…`, writer `b7918ff0…` vs `5b6af71b…`).** Every row: the Space app's Create action (kind picker
opened by keyboard, kind by catalog index), the creation saga opens the new hub document, then the probe's sweep dispatches a rail verb,
undo and redo on it. Captures `wp-s15/generated/s15-hub-document-<run>.txt` (+ `-console.txt`), probe `wp-s15/s15-hub-journey.mjs`,
chains `s15-proof-chain{,-de,-evict,-cancel}.sh`.

| run | locale | device | hub program source | hub files fetched | edit / undo / redo | "document target changed" |
|---|---|---|---|---|---|---|
| `draw-en-4` | en | stale staging | **hub** (24.5 MB verified, band + cancel) | 22 | `addLayer` ✓ / ✓ / ✓ (edits 0→1→0→1) | 0 |
| `writer-en-3` | en | stale staging | **hub** (29.0 MB) | 22 | `setText` ✓ / ✓ / ✓ | 0 |
| `note-en-5` | en | stale staging | **hub** | 22 | `addBlock` ✓ / ✓ / ✓ | 0 |
| `note-de-2` | de | stale staging | **hub** ("Plugin wird geladen note · 24,5 von 24,5 MB geprüft") | 22 | `addBlock` ✓ / ✓ / ✓ | 0 |
| `draw-de-2` | de | stale staging | **hub** | 22 | `addLayer` ✓ / ✓ / ✓ | 0 |
| `writer-de-1` | de | stale staging | **hub** ("… 29,0 von 29,0 MB geprüft") | 22 | `setText` ✓ / ✓ / ✓ | 0 |
| `note-en-store` | en | same profile as `note-en-5` (later session) | **store** | **0** | `addBlock` ✓ / ✓ / ✓ | 0 |
| `note-en-mirror` | en | front 6544: staged note = the bundle's bytes | **local** (24.5 MB verified from the staged files) | **0** (index + manifest only) | `addBlock` ✓ / ✓ / ✓ | 0 |
| `draw-en-unstaged` | en | front 6542: draw never staged nor registered | **hub** | 22 | `addLayer` ✓ / ✓ / ✓ | 0 |
| `draw-de-unstaged` | de | front 6542 | **hub** | 22 | open blocked by hub 8040 DB I/O credit exhaustion (below) | 0 |
| `note-en-evict` / `note-de-evict` | en / de | store with the core (14.9 MB) evicted | **hub**, notice "Plugin note was incomplete on this device and is being reinstalled." / "… war auf diesem Gerät unvollständig und wird neu installiert." | **1** (only the evicted core) | open blocked by hub DB I/O credit (below) | 0 |
| `writer-cancel-en` / `-de` | en / de | fresh profile, band cancel at 19.4 of 29.0 MB | — | cancelled | store stays empty (0 records, 0 blobs); saga: "created, but it could not be opened … Open artifact" / "… konnte aber nicht geöffnet werden …" | 0 |

Persistence: `navigator.storage.persist()` is requested after each commit; headless Chromium answers `persisted: false` (no engagement),
so the store relies on the reinstall path above when the browser evicts.

**Findings routed, not S15-owned.** (a) Hub 8040 (binary from the tree) reached `DB I/O process aggregate credit exhausted` after ~20
documents: opens then loop on rebootstrap (`sync conflict … unavailable`) — the hub DB I/O budget (C10's earlier blocker, H9). (b) An
app command issued in a second window kind of an actor-bound document (note's `note-navigator`, `noteShellCommand`) is refused by the
worker as `command owner mismatch`: the verified execution target admits only its own surface's window kind (`fields.surface.windowKindId`)
— C10's browser-actor lane. (c) The hub creation catalog labels every kind with its editor APP label ("Editor"/"Editor"): the picker cannot
tell `2d.drawing` from `text.document`; a localized per-kind label source is missing (manifest `ArtifactKindSpec.name` is one plain string)
— hub/guest descriptor owners. (d) 7800 (canonical catalog B) still runs the binary without `dialectArtifactKinds`/`extendsPluginId`: the
7800 run waits for W2's os-hub rebuild (`wp-w1/requests/s15.txt` 13:0x + 13:4x).
