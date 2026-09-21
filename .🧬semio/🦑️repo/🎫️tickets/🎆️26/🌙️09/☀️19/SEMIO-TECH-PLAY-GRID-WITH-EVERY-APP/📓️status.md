# 📓️ Status — Semio Tech Play

## 2026-09-19 — session 1 (manual ticket bookkeeping: repo MCP `ticket_open` answered "invalid tool params")

### Done
- `🏢️semio-tech/🎡️play` created (design: `📋️design.md`). 58 panes = every playground registry row except the
  `s` host shell and the six `entwerfen-mit-bestand-*` re-skins. English + `native` terminology locks.
- Schema-first pane catalog `🔨️modules/🧩️runtime/🔣️.json` + `🧬️schema/🔣️.json`, registered as schema scope
  `semio-tech.play.runtime` (`bun ./📜️script.ts schema generate`; the committed catalog was already stale vs disk,
  so the regeneration also carries peers' unrelated scope churn).
- Activation: 26 pane lanes (greedy cover of the 53-component union) merged into `dist/♻️activation/dev`;
  the `s` host lane was tried first and fails on the unlinkable stdio component. Framework:
  `ActivationComponentSpec.installDirectory` so the freshness report reads each extension's own lane dir.
- Dev server `bun ./🔨️modules/🧩️runtime/📜️script.ts serve` on :6033; acceptance via
  `bun nx run @semio-tech/semio-tech-play:test-e2e` (overview + one boot test per app).
- Wiring: root `package.json` workspace + `dev:semio-tech:play` / `build:semio-tech:play`; launch seed entries
  (`🛠️dev🏢️semio-tech🎡️play`, activate, serve, `📦️build…`, `⚖️gate…🎭️e2e`); taxonomy kinds `semio-tech`,
  `members-of-semio-tech`, area `clean`, layer `implementation`, schema scope level `product-module`;
  UI primitives allowlist + chrome-i18n scanned root.
- Unit tests (19, green): pane coverage vs registry, every descriptor editor app reachable, Ajv schema
  validation, icons registered, activation receipt rule, grid geometry, live-pane budget, locks.

### Acceptance history
- Run 1: 50/59 — eight apps dropped the boot `setActiveExample` (undeclared), gis3d boot panic.
- Run 2: 59/59 after the ShellHost example gate and the gisterrain fixture fix.
- Run 3 (2 s settle, refused inputs fail): 48/59 — see `📓️app-boot-defects.md`; puzzle5d fixed since,
  the rest delegated per plugin group.
- Run 4 (strict, after all plugin fixes + full play activation): **59/59 passed** (5.2 min).
- Release: `bun nx run @semio-tech/semio-tech-play:build` succeeded (136 min cold, 172 tasks) →
  `dist/site` 17 428 files; served statically, cad/generation3d/puzzle3d/gis2d/fem3d/energy/note/flow reach
  ready; only 404 is the dev-only `/__semio/agent-bridge`.

### Fixes outside play (found on the way)
- `📐️cad` picking: missing `CadEdgeCurve` import (peer commit 13:48) broke every cad wasm build.
- Demonstrator vite config imported `../../../🔨️modules/📦️site/🗺️tile-serve-mode` (one level too high) since 09-17.

### Open follow-ups
- Raster composite canvas shows no visible demo content in the browser pane; JSON export + mutation JSON bridge
  still serialize whole Raster snapshots.
- Flow test app never finishes closing; rename/patch-widget paths still drop live hosts.
- Architect/Writer derived children load empty (content not persisted in the parent pack); writer demo asset
  child id ≠ target id. CAD pane content resolves from a shipped-example catalogue only.
- Pre-existing red suites the agents measured (unchanged by this ticket): cad 17, shooting 48, writer 44,
  process3d 32, raster editor 25, block2d schema 37.
- `verify layering` already fails repo-wide (222 files over baseline); play adds the same reference kinds the
  demonstrator has (UI allowlist, taxonomy, package.json).
- The committed schema catalog was stale; `schema generate` output includes peers' scopes.

### Known parity debt (same findings in `♻️mit-bestand/🧺️demonstrator`)
- `verify taxonomy report --scope`: `⚛️*.tsx`/`🪧️brand.ts`/`🎨️globals.css` leaf names and unregistered
  `🔨️modules/*` directory kinds.
- tsc: `OwnedBuildPlugin` vs Vite `PluginOption`, `createUiErrorBoundary` vs React JSX, untyped `🟨️.mjs`.

## 2026-09-19 23:35 — session 3 (goal: play shows ALL plugins, finished end to end for all plugins)

Session 2's fix fleet died with a process restart (partial logs: `🗑️generated/stdio-a`, `stdio-ifc-zip`,
`stdio-semio`; baseline `🗑️generated/baseline/summary.tsv`: 52 ok / 111 not ok of 163 plugin crates).
Relaunched as fleet v2 under `📋️fleet-brief-v2.md`; every agent writes `🗑️generated/<topic>/REPORT.md`:
- Opus fix agents: `xcut-toolproof` (tool factory proof `generated_migrated=false`, 94 failures, 15 crates),
  `xcut-dict` (neural `final Dictionary ownership`, 179 failures; owns flow-extension-*, imperative-*, playbook),
  `norm-a`, `norm-b`, `stdio-a2`, `stdio-b2`, `stdio-semio2`, `block-puzzle`, `engineering` (fem, energy,
  process, cad, sourcing, gis), `knowledge` (note, writer, forms, mathematical, reasoning, dag, trinity,
  architect), `media` (sequence, animate, vcs, shooting, space, demonstrator), `design` (flow, procedural,
  draw, raster, layout, remodel, lowpoly, wfc).
- Sonnet audits: `audit-coverage` (plugins/apps without a pane: stdio, space, extensions), `audit-parity`
  (demonstrator → play capability gaps), `audit-visual` (per-pane screenshots + content verdicts on :6033).
Recovery after a restart: read each topic's REPORT.md / RECIPES.md / STATUS.md, re-run crates, relaunch the rest.

### 2026-09-20 00:10 — audit conclusions (session 3) and second wave
- Coverage audit: 32 of 34 plugins have every editor app in a pane. Gaps: `stdio` (no apps, no playground row, full wasm
  unlinkable), `space` (home/space apps excluded together with the `studio` host), the five
  `imperative-extension-*` components are in NO activation union (imperative Cargo metadata lacks
  `consumes = ["imperative.module"]`), and the coverage gate silently skips `playbook` (no root `🔣️.json`).
- Parity audit: P0 no per-pane default example (`playPaneBrand` sets no `defaults.exampleId`; demonstrator brands do);
  P1 `schedulePlayIdle` never wired (no background warm boot), cards lost description + open CTA, touch-scroll
  next-pane preload missing; P2 no map-tile serve-mode test, `🔒️dependencies.json` predates play, 6 trailing grid
  cells, `extensionDir()` build-mode fallback to check.
- Second wave (Opus): `play-landing` (P1/P2 landing items), `play-defaults` (catalog `example` field + curated
  defaults + pinning test + example inventory), `play-coverage` (imperative `consumes`, space home/space panes,
  playbook gate, all-plugins/all-components law), `play-stdio` (feasibility → linkable stdio app + playground + pane).
- Machine note: 16 agents saturate 10 cores / 32 GB (swap ~17 GB used); :6033 answers in ~15 s. Browser verdicts
  taken under this load are valid for content, not for timing.

## 2026-09-20 01:30 — session 4 (recovery)
Process restart at ~01:18 killed the coordinator and the whole v2 fleet mid-flight (no REPORT.md anywhere; orphaned
cargo runs kept writing logs). Repo MCP `ticket_reopen` still answers "invalid tool params" → bookkeeping stays manual.
Relaunched every topic under `📋️fleet-brief-v3.md` (recovery protocol: successor reads the topic folder, re-runs,
continues, keeps a per-topic `STATUS.md`). Topics: play-stdio, play-coverage, play-landing, play-defaults,
xcut-toolproof, xcut-dict, norm-a, norm-b, stdio-a2, stdio-b2, stdio-semio2, block-puzzle, engineering, knowledge,
media, design (Opus); audit-visual, audit-followups, audit-artifacts (Sonnet, reports as files via general-purpose).
Fleet v3 launched 01:40 (resume a rate-limited/killed agent by SendMessage to its id while this coordinator lives;
after a coordinator restart relaunch the topic from its STATUS.md): play-stdio aa7075a1fd8edfe70 · play-coverage
a72a5d4100328eff0 · play-landing a840a37ca84ded74f · play-defaults a052c2c890e1d1a7a · xcut-toolproof
ad1ebd81e22d0fb96 · xcut-dict a8cc462c37bf8ef59 · norm-a aeb92792913fb97d9 · norm-b a8746ede20b4be230 · stdio-a2
a8c1c9b2c3b74fea5 · stdio-b2 ac264a30b0c885e44 · stdio-semio2 a1842b36009067ac6 · block-puzzle a66f4036588a27a57 ·
engineering a5474a5e1786c5135 · knowledge a422ef1ab6bd2acae · media a0ecca054ae2ae8c1 · design ad61ef16b42193158 ·
audit-visual adc299625bf483222 · audit-artifacts a41c9ecc4a2af26d6 · audit-interaction a1a704077be6d57ca.

### 02:35 — session 4 progress
- Harness refuses subagent `REPORT.md` writes ("return findings as text") → topic reports live in each `STATUS.md`
  and in the coordinator's `📓️*.md` notes. Audit of artifacts/apps/extensions: `📓️audit-artifacts.md`.
- :6033 crash-looped on the imperative-extension union until play-coverage re-activated (up 01:49); then Vite wedged
  on the fleet's edit storm → play gained a FROZEN serve mode (`SEMIO_TECH_PLAY_FROZEN=true` → `server.watch = null`;
  `serve-test` always frozen), supervisor runs it frozen and recycles on `🗑️generated/serve-restart.request`.
- play-landing DONE: gapless-first grid geometry (60 panes → 9×7, trailing row centred, pan clamped to occupied
  columns), `playExtensionDirectory` build fallback tested, `🔒️dependencies.json` users, en+de landing/card labels,
  navigation landmark + alert role. Unit 58/62 (4 red = stdio ×3, playbook ×1, sibling-owned); chrome-i18n 0;
  3 browser probes 0 errors. Details `🗑️generated/play-landing/STATUS.md`.
- Serve log lists most staged components `[stale] source-newer` → a full play re-activation is required before the
  final acceptance run (after the plugin fix agents finish their production edits).

## 2026-09-21 13:55 — session 5 (coordinator Fable 5.1; goal: play working end to end, all plugins)
- Found: `🗑️generated/` swept (all topic STATUS.md, baseline, audits lost); :6033 down, no supervisor; play committed
  2026-09-20 23:20 (60 panes / 8 groups / 27 lanes, activation receipt 2026-09-20 16:23); play unit 58/62 (4 red =
  stdio pane missing). Peers: `End-to-end repo completion` + `Semio-tech play folder setup` sessions and a Codex fleet
  sweeping `LocalizedLabel` mutation labels (~3.4k files) and rewriting `💻️os`; a `semio-s-plugin-stdio` wasm-release
  build (not ours) running. Machine: load ~30, swap 12.4/13 GB, 59 GB disk free.
- Relaunched: serve supervisor (frozen) for :6033; full plugin native baseline sweep (`🗑️generated/baseline`,
  `🧪️baseline-plugin-tests.py`, one cargo at a time) + 10-min test-binary watchdog; fleet v4 (`📋️fleet-brief-v4.md`,
  tracked per-topic reports `📓️<topic>.md`).
- 14:00–14:35 fleet v4 launched (resume a killed agent by SendMessage to its id while this coordinator lives; after a
  coordinator restart relaunch the topic from its `🗑️generated/<topic>/STATUS.md` / tracked `📓️<topic>.md`):
  Sonnet audits audit-visual a6342dd5c4a7881eb · audit-followups ab91ffd865f18f8f8 · audit-stdio ababe6884121f7eb3;
  Opus fixes play-stdio af5e6f73e247e98a9 · raster a54e59ef6c6dddb6f · flow a5b2801bfff69da09 (guest code peer-owned →
  proposed diffs only) · knowledge-children a996195935fe8d21b · cad-content a951bf26a4494c143 · engineering
  a8eca3c74fa0d0d14 · design a7251883c7737a7d0 · stdio-a a6555f77913b638cc · stdio-b a57adfe706b86af55.
  Wave 3 (14:40): knowledge a2eeb244622dc06a5 · media a7f5f0d8fdf039eeb · block-puzzle a9ffb07b791b50560 — covering knowledge (note, forms, mathematical, reasoning, dag, trinity, imperative), media
  (shooting, vcs, demonstrator), block-puzzle; norm/space/sequence/procedural/playbook guest code is peer-owned (S10).
- 14:10 :6033 serve died on `Stale play activation lane: flow` (peer re-activated the flow lane 09-20 19:15 after the
  demonstrator lane 14:56); aligned the demonstrator receipt's nine flow-* rows to the staged flow lane (dist only);
  :6033 up 14:25. Peer (`End-to-end repo completion`, ticket 26/09/18/OS-HUB…) owns the LocalizedLabel sweep and the
  stdio wasm-release build; all wasm32 cargo must go through their `📜️wasm-build-mutex.sh` (see brief v4).
- 14:30 stopped the serial baseline sweep (2 crates in 30 min, held the shared build-dir lock ahead of every fix agent); each topic baselines its own crates. Peer-owned plugins (norm, space, sequence, procedural, playbook) get a later run.
- 14:30 play-stdio DONE: 9 stdio panes (group documents), 28 lanes, play unit 62/62, 69 panes boot on :6033 with 0 errors (📓️play-stdio.md).
- 14:45 launched stdio-examples ae784cfb650546e45 (curated examples for the 9 stdio panes) and registry-projection a46e3357db7b29e0a (plugin-registry 'live home' test red since the 09-20 home/space rows).
- 14:50 audit-visual DONE (📓️audit-visual.md): 60/60 ready, 0 refused/panics; content defects: raster + block3d blank,
  architect stuck "Waiting", reasoning-wires + animate blank, mathematical/generation2d gated (no setActiveExample),
  wfc3d/gis2d boot the wrong example, and a log-bridge bug emits every extra argument of a debug message as a bare
  `console.error` (16+ bogus errors on 44/60 panes → strict e2e would fail). Launched console-spam + default-example
  (Opus); block3d → block-puzzle, reasoning-wires/mathematical → knowledge, animate/architect/writer → knowledge-children.
  generation2d (procedural) guest code is peer-owned → to hand to the peer.
- 14:55 registry-projection DONE: isHostPlaygroundFilter now ignores app-pinned host rows (home/space), registry test green (📓️registry-projection.md).
- 15:05 default-example DONE: no defect — the audit compared example ids to rendered labels; new play law pins
  label.native.en per curated example (unit 63/63). console-spam DONE (diagnosis): vendored preview2-shim
  `consoleStream().flush()` releases partial buffers → one console call per fmt fragment; fix is a proposed diff in
  peer-frozen 🌐️browser-bundle (📓️console-spam.md), asked the peer to apply or allow; verified 16→0 / 172→0 on a
  patched served copy.
- 15:20 registry-projection follow-up DONE: launch name prefixes now plugin+artifact+subset folders, assertDistinctLaunchNamePrefixes, registry suite 60/60, play 63/63. Peer approved the browser-bundle log-bridge fix → console-spam applying it.
- 15:30 console-spam APPLIED (peer-approved): patchPreview2ShimGuestLogLineRelease in browser-bundle materialization + wasi-activation law; laws pass; served vendor byte-identical.
- 15:35 watchdog raised 10 → 30 min: under load ~70 the raster artifact test binary was SIGKILLed mid-run (~80 tests in) — not a hang.
- 15:40 stdio-a DONE: all 19 crates green (1 588 tests); ifc Part-21 header conformance, stale fixtures regenerated (📓️stdio-a.md).
- 16:00 flow DONE (diagnosis + test-side): 58→5 reds in flow-flow, 1 in plugin-flow; remaining 6 are guest/framework (maintenance_step 26-stage rotation tax makes real surfaces unable to close in bound) → proposed diffs handed to the peer (📓️flow.md §5). Fleet told: private CARGO_TARGET_DIR allowed, watchdog 30 min.
- 16:05 activate.request/raster received; activations are batched into ONE final play activate-dev through the mutex (queue: tc3c pz1 s10 rb1 c7 stdio-a stdio-examples) — asked the peer for a slot before rb1.
- 16:10 peer granted: mutex slot before rb1 via 📜️wasm-build-mutex-play-slot.sh (fixed ticket 20260921135030-<pid>-play), and 'apply' for flow guest diffs (flow crates only) → flow agent applying.
- 16:35 raster report: blank composite root cause = raster never enabled stdio-semio conversion-image/-drawing features (no composer → assets {} → zero textures); fixed + retained add-layer-asset id/decode defects + a real hang (4 KiB grant vs 16 KiB pages); artifact crate 186 ok / 43 red (test debt) → agent resumed on the 43.
- 16:45 stdio-b report: 17/18 artifact crates green (4 828 tests); svg 2 red = XmlDeclaration has no quote slot → agent resumed to land the schema-first fix; plugin-stdio descriptor_is_fresh waits on the queued describe (stdio-examples).
- 16:50 flow DONE (guest diffs applied with peer approval): flow-flow 251/2, plugin-flow 3/1, 9 extensions green; 3 reds are framework/backbone (routed to peer FP7/S10); found: back-to-back window-config commands drop the second (live defect).
- 16:55 peer routed flow's 3 framework reds + batch-publication zeros + dropped window-config command to its FL2 slice. knowledge-children report landed.
- 17:15 engineering report (partial: 6 process3d root causes, energy, sourcing, gisterrain ×2 fixed; open: gismap child projection, gisterrain vectors, process3d rest, fem-3d presence leases, fem-2d wall-clock, gis descriptor) → agent resumed. rb1 (peer 60-component wasm-release) holds the mutex since 16:56 → my activation slot missed; asked peer for ETA/yield; told agents not to block on queued wasm checks.
- 17:20 queued the full play activate-dev (28 lanes) in the mutex slot right after rb1 (ticket 20260921135200-<pid>-play; peer ETA for rb1 19:00–20:00; tc3d slots behind me). Log 🗑️generated/activation/activate-dev-*.txt.
- 17:35 requeued the mutex job as 📜️describe-all-then-activate.sh (describe all 34 plugins → play activate-dev): activation lanes copy committed descriptors and never regenerate them; every descriptor is stale (label sweep + new setActiveExample actions). knowledge-children DONE (children content persisted as sibling state fields; architect/writer/animate demos non-empty; residual test debt buckets 1–3), design DONE-partial (draw SIGABRT fixed, layout 92→~10, wfc unmeasured, verification run waits on peer compile).
- 17:40 stdio-b DONE: XmlDeclaration.quote landed schema-first (xml 97/0), 28/29 targets green; svg's 2 byte-exact laws on an untracked file → restated as tracked fixture + fixpoint laws (agent resumed).
- 17:50 stdio-b DONE: all 18 artifact crates green (svg laws restated on a tracked fixture). raster pass 2: 202 ok / 22 red (3 RasterDiff defects, 2 composed-child persistence, 13 raster-side, 4 framework proposed) → pass 3 running.
- 18:20 USAGE LIMIT: seven Opus agents (engineering, media, knowledge, cad-content, knowledge-children, block-puzzle,
  raster) died mid-edit on the session limit (resets 20:20 Berlin); design's detached poller and the queued
  describe-all→activate mutex job keep running. Edits are on disk; each topic's STATUS.md + 📓️<topic>.md hold the
  trail. Resume by SendMessage to the ids above once the limit resets (user said "try again").
- 20:30 limit reset: resumed raster, engineering, media, knowledge, cad-content, knowledge-children, block-puzzle by
  SendMessage. Mutex job ran 17:31–19:50 (rb1 run 2 ended early): describe 29/34 ok (failed: stdio, gis, vcs on a missed
  `XmlDeclaration { quote }` literal site in stdio-semio — stdio-b fixing; puzzle, norm — reasons below), activate-dev
  staged 25/28 lanes (gis/stdio/vcs component-dev failed on the same error) and then got SIGINT (rc=130, origin
  unknown). :6033 still serves the 14:21 generation. rb1 run 3 holds the mutex since 20:12; asked the peer for the next
  slot for a re-run (~25 min, nx-cached).
- 20:40 describe failures explained: norm wasm-dev component > 256 MiB describe bound (peer-owned, told them); puzzle owned describe() hit the 30-min epoch deadline under load (→ block-puzzle to check eager work at describe time). Re-run queued as 20260921235850-88484-play after the peer's tc3d (~60–90 min).
- 21:35 mutex ticket moved to 20260921205000-<pid>-play (peer: right after the running s10 restage).
