# S3 (session 10) — Foreign-Kind Open And Block Inside `s` (G11 gaps G1 + G3)

Slice S3, session 10, 2026-09-24 16:40–17:45. Brief: G11 §C G1 (open a proven-interactive `raster` and a
formerly `BatchOnlyPendingRewrite`-refused `dag` inside ONE running `s` session), G3 (🧱️block 2d/3d/5d boot +
render in a browser, one mutation verb through the Actions rail), the hub workspace inside `s`, and a permanent
nx target + launch row. No cargo, no activation, no hub started. Captures: `wp-s3/generated/*.txt|png`.

**Successor of** `📓️s3-s-host-home-surface-and-foreign-open.md` (fleet 5, 2026-09-20) and of the
`📓️s11-…`/`📓️s12-…`/`📓️s13-…` sweep reports; §5 names what was proven then versus what is observed now.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only.

> **Headline (measured).** Inside one `s` session on W1's 12:20 staging: `PLAYGROUND_SESSION.plugins.length ===
> 60`, 60/60 registry rows `loaded`. `dag` opens from the Home palette and PAINTS its graph, `addNode` →
> undo → redo round-trips (`edits [0,1,0,1]`, the DSL window's text gains `n1:math.add`), 0 fault lines.
> 🧱️block 2d/3d/5d all open from Home, render (2d board text `6 → 7 Handle Kinds`, 3d a WebGL mesh
> 1427×780, 5d board + world text) and round-trip one migrated verb each through the Actions rail. `raster`
> does NOT open: its staged `core.wasm` is gone from disk (one of 15 guests deleted at 12:54), and the shell
> now answers with a visible, localized refusal (`„semio · raster“ konnte nicht geöffnet werden.` in `de`)
> instead of the silent `pageerror` it used to drop. Two React-shell root fixes made that true: every
> spawned window whose kind id differs from its body key was stuck on the loading skeleton forever
> (cache-key mismatch), and `spawnProgram` swallowed every failed open. The hub workspace opens from the
> footer pill AND from the host-only `/hub` route, sign-in works on hub 7891, and a listed space opens into
> its space index window.

## 0. Infrastructure (measured)

| what | value |
|---|---|
| staged lane | `…/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/s/activation/🔣️receipt.json` 2026-09-24 12:20, `semio.dev.activation/v1`, variant `s`, **60 plugins** — never re-activated by this slice |
| serve | `bun ./📜️script.ts serve s react dev`, `S_OS_PORT=6400`, `S_HUB_URL=http://127.0.0.1:7891` — wrapper **73917**, vite **73935**, esbuild **73952**; log `wp-s3/generated/s3-serve.txt` (`[dev-local-hub] reusing healthy hub at http://127.0.0.1:7891`; no hub started) |
| first serve | 6380 (pids 66326/66358/66370), killed by pid at 16:59 when the coordinator reassigned 638x to C8; capture `s3-foreign-kind-g1a.txt` was taken on it |
| hub (read-only) | 7891, WG6's (`⚡️cache/hs1/os-hub-7611`, pid 51991), `/readyz` 503 only on `artifactAuthority: trusted-catalog-never-published-in-this-data-root`; `/auth/sessions/me` 401 |
| browser | playwright chromium headless, `--use-angle=metal`, 1440×900 |
| W1 during the slice | describe-all retry + catalog A held the wasm mutex (`w1 16:36`, `16:53`, `17:25`); `dist/dev/🔌️plugin-modules` and the receipt were not rewritten during any probe run |

### 0.1 The staged lane is no longer 60/60 on disk (measured, `s3-staged-core-wasm.txt`)

W1's 12:20 proof was "dist == staged for 60/60". At 16:5x, 15 guest directories under
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/` hold every file
their `.nx-artifact.json` lists **except** `semio_s_plugin_<id>_component.core.wasm`, all with directory mtime
**12:54**: wfc, flow, gis, vcs, animate, demonstrator, architect, process, cad, norm, energy, trinity, **raster**,
stdio, puzzle (consistent with W1 §4.5's 12:43–12:54 external sweep). The receipt still lists 60, so
`serve` does not re-activate and the shell reports all 60 `loaded` — the registry row is descriptor-level; the
actor's wasm is fetched lazily on first `createApp`. Routed to W1: `.tmp-ticket/wp-w1/requests/s3.txt`
(re-materialize those 15 + activate). No answer appended at 17:36.

## 1. G1 — plugin census and foreign-kind open inside ONE `s` session

Probe `🐍️s3-foreign-kind-in-s.mjs` (new; imports the witness helpers of `🐍️s6-all-kinds-sweep.mjs`). One page,
no reload; census first, then each target opened from the Home palette (`spawn.<pluginId>` / app-qualified row),
studio fallback only when the Home open yields neither a window nor a refusal notice. Final run through the nx
target: `wp-s3/generated/s3-foreign-kind-nx.txt` (+ `s3-nx-stdout.txt`, screenshots `s3-nx-*.png`).

### 1.1 Census (measured, every run)

`await import("/@id/virtual:semio-playground-session")` in the page → `PLAYGROUND_SESSION.plugins.length = 60`,
`variant: "s"`, `hostMode: true`; `window.__semioOsCatalogProbe`: 60 rows, **60 `loaded`, missing `[]`**, 148
spawnable programs. Boot fetches only descriptors (`🔣️.json`) for raster/dag/block; bridge + component + core
wasm are fetched at the open (captured per row as `moduleRequestsDuringOpen`).

### 1.2 Per-step table (measured, nx run 17:2x unless noted)

| step | raster | dag |
|---|---|---|
| opened from | Home (`s-home-main` only) | Home (after raster's refusal) |
| palette row | `spawn.raster` (`Spawn semio · raster`) | `spawn.dag` |
| network at open | `🖨️raster/🌉️bridge.js`, `🟨️.js`, `semio_s_plugin_raster_component.js`, **`…core.wasm` → 404** | `🕸️dag/🌉️bridge.js`, `🟨️.js`, `…component.js`, `…core.wasm` (200) |
| window | none | `dag-4::dag-main`, `dag-4::dag-compiled-dag` |
| before the fix | `pageerror: WebAssembly.compile … HTTP status code is not ok` ×1 per attempt, **no notice** — silent drop (`s3-foreign-kind-g1a.txt`, `…g1-raster-diag.txt`, unhandled rejection from `ShardClient.handleMessage`) | windows present, bodies = loading skeleton (§3.2) |
| after the fix | notice `shell.spawnProgram.open-failed` — en `“semio · raster” could not be opened.`, de `„semio · raster“ konnte nicht geöffnet werden.` (`s3-foreign-kind-g1-de.txt`, locale seated through Settings, `lang=de`); console `[os-shell] spawnProgram raster s.raster.raster@1/*#editor: shell.spawnProgram.open-failed Error: …`; 0 pageerrors | main: 2 canvases 966×781 + `dag.play.main`; DSL: `slider:slider … screen:screen …` (`s3-foreign-kind-g1c.txt`, `s3-g1c-dag-after-redo.png` shows the graph) |
| mutation | — (no window) | `addNode`, ledger `Add Node↶`, edits `[0,1,0,1]`; after redo the DSL text contains `n1:math.add` |
| undo / redo lane | — | rail/rail (g1c), chord/chord (nx run) |
| fault lines | 0 | 0 |
| verdict | **refused visibly, localized** — the open itself needs W1's restage | **PASS** |

`installPlugin` / `openArtifactWithAppRef`: in host mode neither runs for a palette open — all 60 registry rows
are installed at boot and the palette calls the shell's own `spawnProgram` → `handle.createApp` (source,
`🏛️ShellHost/🟦️.tsx` `spawnProgram`). The lazy half that DOES run at open is the actor instantiation, visible as
the per-plugin module fetches above. `openArtifactWithAppRef` runs only on the hub creation saga
(`openReadySpaceArtifactCreation`) and the `os.open-artifact` relay; no hub listening today has a published
catalog (7891 `artifactAuthority` 503), so that lane stays unobservable (§4).

## 2. G3 — 🧱️block 2d/3d/5d inside `s` (measured, `s3-foreign-kind-g3c.txt` + nx run)

Staged block descriptor/wasm are the 2026-09-23 22:40 build (not stale for this lane). All three opened from the
Home palette of the running `s` session (block2d from Home, 3d/5d next in the same session).

| variant | window(s) | render (rail overlay subtracted) | verb (rail) | edits | undo/redo | faults |
|---|---|---|---|---|---|---|
| block2d | `block-N::block2d-board` | UiNodes `block2d-play-board.{body,summary,counts}`: `Node kind: Hexagonal Cut Concrete Forest Left 6 Handle Kinds, 11 Handles` → after redo **`7 Handle Kinds`** | `addHandleKind` (`Add Handle Kind↶`) | `[0,1,0,1]` | rail / rail | 0 |
| block3d | `block-N::block3d-world` | `block3d.play.world`, WebGL canvas **1427×780**, mesh visible (`s3-g3c-s-block-block3d-1-editor-opened.png`) | `addRepresentation` | `[0,1,0,1]` | rail / rail | 0 |
| block5d | `block5d-board` + `block5d-world` | board `Part kind: … 2d grips: 1`; world `… mesh: /mesh/🧊️hexagonal-cut-concrete-forest-left.glb` | `addGripKind` | `[0,1,0,1]` | rail / rail | 0 |

**G3 is observed inside `s`, not in the single-plugin playgrounds.** Before §3.2's fix the same three rows
round-tripped their verbs while every body painted only the skeleton (`s3-foreign-kind-g3a/g3b.txt`, screenshots),
and block2d's rail read `railRows: 0` because its chip sat under the open Artifact panel (§3.3).

## 3. Root causes found and fixed

### 3.1 `spawnProgram` dropped every failed open silently (React shell, fixed)

`🏛️ShellHost/🟦️.tsx` `spawnProgram`: `if (!pluginEntry || !session) return;` and an unguarded
`await pluginEntry.handle.createApp(...)` under `void spawnProgram(program)`. A guest whose instantiation rejects
(here: 404 wasm) became an unhandled rejection — no window, no notice, no ledger row. Fix: three named refusals
`program-not-installed | session-not-ready | open-failed`, each one `console.warn` line and one transient notice
with code `shell.spawnProgram.<reason>`, text `{en, de}` naming the program's breadcrumb. Vocabulary owned by
`🏛️ShellHost/🪟️spawned-program/🟦️.ts` (`SpawnProgramRefusalReasonV1`, `SPAWN_PROGRAM_REFUSAL_LABELS_V1`,
`spawnProgramRefusalNoticeTextV1`, `spawnProgramRefusalCodeV1`), shaped like `📣️replay-refusal`.

### 3.2 Spawned window bodies read the refresh cache under the wrong key (React shell, fixed)

`refreshSpawnedUi` sends `buildUiRefreshRequest(…, windowKinds …)`, whose windows are keyed by window id
(`key: instance.id`, cached as `window:<id>`, `🛠️ShellHelpers/🟦️.tsx:5609`), but read the result back as
`cache.get(\`window:${kind.bodyKey}\`)`. For any kind whose id ≠ body key (dag `dag-main`, block2d
`block2d-board` → `block2d-play-board.body`, …) the read missed and `pendingWindowUiNode()` (`activity:
"loading"`) was published forever — measured: `pane-host-root` held only the `aria-busy` skeleton after 8 s
(`🐍️s3-window-body-diagnose.mjs`, `s3-window-body-diagnose.txt`). The primary-session path already reads by id
(`:5406`). Fix: read `window:${kind.id}`. The ledger/edit-count witness the S11–S13 sweeps scored on could not
see this: verbs, undo and redo reached the guest; only the paint was missing.

### 3.3 Shared sweep: rail chips pressed through covering panels (probe, fixed)

`🐍️s6-all-kinds-sweep.mjs` `unfoldActionsRail` force-clicked each engagement chip; a docked Artifact panel over
block2d's chip swallowed the press (`railRows: 0`). Chips are now pressed through the existing
`clickUncovered` (b3a §16.1 point 3's retirement). The sweep also exports its helpers behind an
`import.meta.main` guard so single-journey probes share one witness instead of copying it.

### 3.4 Laws (measured)

`🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` +4 laws: refusal vocabulary en/de/fallback + codes;
`spawnProgram` has no silent return and catches instantiation; the refresh cache keys a body by window id when
id ≠ body key (behavioural, against `buildUiRefreshRequest`/`applyUiRefreshResponseToCache`); `refreshSpawnedUi`
reads by `kind.id`. `SEMIO_TEST_LEVEL=long bun x vitest run --config ./🧪️tests/🎚️config/🟦️.ts
🪟️spawned-program-session` (in `🎯️targets/⚛️react`) → **37 passed** (33 before), `s3-law-spawned-program.txt`.
Scoped `bun ./📜️script.ts typecheck` (`@semio-tech/framework-os`, whole os tsconfig) → rc 0, 0 errors before
and after (`s3-typecheck-before/after.txt`); it demonstrably covers the edited files (it caught my own
`request.windows` optional-chain slip, and `--listFilesOnly` lists both spawned-program files). No cargo.

## 4. Hub workspace inside `s` (measured, `s3-hub-in-s.txt`)

Existing probe `🐍️c1c-s-host-probe.mjs`, extended (checks 4–5, `C1C_EMAIL/C1C_PASSWORD`), run against serve
6400 → hub 7891 as `ada@example.org` (au3's documented credential, provisioned on 7891 by WG6). **All checks
passed**:

| check | observed |
|---|---|
| 1 signed-out Home | `s-home-main` published, sign-in badge 1, 0 space rows |
| 1 footer pill | opens `[data-semio-hub-workspace]` inside `s` |
| 2 signed in | badge 0, Home still published, palette tab present |
| 4 `/hub` route (host-mode only) | `page.goto(/hub)` → route `/hub`, workspace overlay over the live `s-home-main`, session survives the reload; 7 spaces listed (`Open wg6 live … — Author`) |
| 5 open a listed space | route `/spaces/01a0d3ed-5fc7-77f6-84ab-f2ef1054d6ed`, overlay closed, window `framework.window.table` = the space index (`Create Artifact · ID · Name · Kind · Subset · Updated · Updated By · Presence`), no notices |

Not observed: Home's own space table stays empty after sign-in (`spaceRows: 0`) although the workspace lists 7
spaces; creating an artifact in the opened space (7891 has no published catalog, and this slice is read-only on
peers' hubs), hence `openArtifactWithAppRef` via the creation saga. The workspace shows `Signed in as
01a0d3d2-…` (principal id) rather than the display name.

## 5. Then vs now

| claim | proven then | observed now (W1's 2026-09-24 staging) |
|---|---|---|
| `s` serves 60 plugins | receipt 60/60 (W1 12:20); G11: console read never done | **console read done**: 60/60 `loaded`; but 15 guests' `core.wasm` missing on disk since 12:54 (§0.1) |
| foreign kind opens inside `s` | S3 fleet-5 (09-20): NO — `spawnApp` dropped on Home; S5/S11–S13 (09-20/22): via a studio, verb round trips on the ledger | opens **from Home**; **bodies now paint** (they did not for id≠bodyKey kinds under the ledger-only witness) |
| raster inside `s` | S11 (09-22): PASS `addLayer` | **refused** (wasm 404) — visibly, localized; needs W1 restage |
| dag inside `s` | S11 (09-22): PASS `addNode` (ledger) | PASS with the graph painted and the added node visible in the DSL window |
| block in a browser | b3a §16 (09-20): 2d/3d/5d in single-plugin playgrounds; S11: block2d `addHandleKind` inside `s` (ledger) | 2d/3d/5d **inside `s`**, rendered (3d WebGL mesh), one rail verb each |
| hub inside `s` | S3 fleet-5: sign-in + Home publish (c1c all PASS); `/hub` teardown fixed; no space opened | footer pill + `/hub` route + sign-in on 7891 + a listed space opened |

## 6. Permanent wiring

Existing nx target `@semio-tech/framework-os-dev:s-host-foreign-kind-s` (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`,
added by S3 fleet-5 next to `cold-boot-check-s`) retargeted to `🐍️s3-foreign-kind-in-s.mjs`; existing launch row
`🛠️dev🪐️os-s🔭️foreign-kind` (group `3_dev`, order `387.066`, `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`)
now defaults to `http://127.0.0.1:6070/ --tag launch raster dag block=…block2d… block=…block3d… block=…block5d…`
(6070 = the `🛠️dev🪐️space⚛️react` serve port). **Executed through nx** (`NX_DAEMON=false bun nx run
@semio-tech/framework-os-dev:s-host-foreign-kind-s -- http://127.0.0.1:6400/ --tag nx --out wp-s3/generated …`) →
`Successfully ran target`, PASS 4/5 (raster = §0.1). `🐍️s2-s-host-foreign-kind-probe.mjs` is kept as history
(it drives the retired sign-in → create-space → studio journey) and is no longer wired. Honest note, unchanged from
fleet-5: the target runs a ticket-folder probe rather than a `📜️script.ts` verb; its permanent home is
`🧑‍💻dev/🧪️tests/` once the sweep witness it shares moves there too.

## 7. Measured vs unverified, honest gaps

- **Measured**: everything in §1–§4 with the named captures; laws 37/37; typecheck rc 0; nx target run.
- **raster open inside `s`** — blocked on W1's restage of 15 guests (request filed); not rerun after a restage.
- **`installPlugin`/`openArtifactWithAppRef` lines for a lazily installed kind** — not observable: host mode
  installs all 60 at boot; the hub creation saga needs a hub with a published catalog (none listening today).
- One diagnose run (16:5x) saw the boot beacon `data-semio-os-error="s"` once; every other run (≥10) booted
  `ready:s`. Not reproduced, cause unknown.
- The shared `closeWindows` helper does not close spawned windows (`<id>.windowControls.close` does not exist
  on dock tabs), so later rows open from the previous program's window, not from Home; the first open of each
  run is from Home.
- The refusal notice names the breadcrumb (`semio · raster`), not the localized app label.
- 3.2's regression window (introduced 2026-09-20 23:20 per `git blame`) means the S11–S13 PASS rows for kinds
  with id ≠ body key were scored on the ledger with an unpainted body; not re-swept here (G2's scope).

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `spawnProgram` refuses visibly (3 reasons); `refreshSpawnedUi` reads `window:${kind.id}`; import |
| `…/🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts` | new region `🚫️SpawnProgramRefusal` (type, en/de labels, notice text, code) |
| `…/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` | +4 laws (§3.4) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | `s-host-foreign-kind-s` → `🐍️s3-foreign-kind-in-s.mjs` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `🛠️dev🪐️os-s🔭️foreign-kind` default args |
| `🐍️s3-foreign-kind-in-s.mjs` (ticket, new) | the G1/G3 journey probe |
| `🐍️s6-all-kinds-sweep.mjs` (ticket) | helpers exported behind `import.meta.main`; chips pressed via `clickUncovered` |
| `🐍️c1c-s-host-probe.mjs` (ticket) | checks 1-pill, 4 (`/hub`), 5 (open a listed space); `C1C_EMAIL/PASSWORD` |
| `🐍️s3-home-open-diagnose.mjs`, `🐍️s3-window-body-diagnose.mjs` (ticket, new) | diagnostics behind §1.1 and §3.2 |
| `.tmp-ticket/wp-w1/requests/s3.txt` (new) | restage request for the 15 missing `core.wasm` |

Processes started by this slice: serve 6380 (66326/66358/66370, killed 16:59) and serve 6400 (73917/73935/73952,
killed at the end of the slice).
