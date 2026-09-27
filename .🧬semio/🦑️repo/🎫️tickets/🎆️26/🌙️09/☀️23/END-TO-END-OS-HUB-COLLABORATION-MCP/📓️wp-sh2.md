# WP-SH2 — Space-Home IO-Owning Job, Space-Home Reds, Kernel Lib Reds (window 3)

Session 14 slice SH2 (Opus executor, coordinator `main`), continues `📓️wp-sh1.md`. Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP.
Rules: `📓️session-14-preamble.md` (+ 13/12). Inputs `wp-sh2/`, expendable captures `wp-sh2/generated/`, durable data
`.🧬semio/🌐hub/s14-sh2-*`. Ports: hubs 8080–8089, serves 6580–6589. Guest-linked edits: prepared patch + overlay proof
during the freeze, landed compile-atomic in window 3.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | space-home `bindSpaceFile` / `importSpace` / `deleteVirtualFileSystemNode`: IO-owning job, event-sourced, progress + cancel, en+de, laws + oracle, reachable control | **prepared, overlay native check green** (35-file set `wp-sh2/payload/`, `sh2-apply.py` dry run 35/35 clean on the tree; `generated/ov5-check.txt` rc 0 19:19); overlay laws queued (overlay lane 4 deep); bind cannot work in any real shell → blocker B1 (design below) |
| 2 | space-home reds (`structural_correspondence` outcomeClasses, config `retained_config_cancel_and_cleanup…`, all other lib reds) | tree baseline measured: space-home lib **27/28** (`generated/base-home.txt`); fixes in the set (2 stale tests + 4 stale TS oracles/schemas found on the way: Home retained-limits schema, SpacePlay schema, event-page oracle, identity-rows oracle) |
| 3 | kernel lib reds (directory client ×5, open-plan fixture, en1993 grammar) | pending re-measure (SH1 overlay 15:34: 1233/1234, only en1993 left) |
| 4 | space/home e2e in `s` (create, bind/import, delete, reopen) locally + hub 7800 | probe written (`wp-sh2/sh2-home-e2e.mjs`); **baseline (pre-SH2 guests, serve 6580 → 7800 B3):** no import control (expected), **create-space row never appears — Home table never streams past row 27** (finding W1, routed to main); full run after landing |

### Log

- 18:2x read AGENTS.md, preambles 14/13/12, fleet-14 log (no CHAIN LAUNCHED yet), `📓️wp-sh1.md` + `wp-sh1/`, `📓️wp-lc.md`,
  `📓️wp-ld.md`, inventory row SH1, fleet-13 log 14:00→16:0x.
- 18:3x `python3 wp-sh1/sh1-apply.py` (dry run) on the live tree: **files=25 apply=25 conflicts=0** (every target still equals
  SH1's `.old`, no peer changes since 15:56).
- 18:37 baseline native-lane runs queued detached (`wp-sh2/sh2-native.sh`, pid 44771, tag `base`: space-home lib, space-home
  `component-app-assembly`, space-space, plugin-space, kernel `sync,ureq` lib) → `generated/base-*.txt` (native lane 6 deep).
- 18:3x overlay = SH1's `.🧬semio/🌐hub/s13-sh1-overlay` refreshed by `wp-sh2/sh2-overlay-sync.py` (tracked + untracked files,
  size/mtime diff copy, 64 s, 329 files). Overlay runs: `wp-sh2/sh2-overlay.sh` via `fleet-mutex.sh overlay`, PRIVATE
  build/target dirs `s13-sh1-{build,target}`.
- 18:40–18:52 overlay check of SH1's payload blocked twice by a PEER's in-flight edit (not ours, not staged): the
  COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE work (`🌱️value/✨️derive/⚙️expansion` + `🔁️codec` + `🧬️schema/✅️validator`
  + `🗣️dsl/🧬️schema` + stdio contract `✏️editing/🩹️patch/` (untracked) …): the worktree fails in `semio-framework-actor`
  (8× E0282 in derive expansions, `generated/ov1-check.txt`); pinning only the value files to the index broke stdio contract
  (`generated/ov3-check.txt`). Overlay now pins that peer's whole set to HEAD (`wp-sh2/overlay-pin-head.txt`) and drops its
  untracked files (`overlay-drop.txt`). The live tree is red in the same place until that peer finishes — my native-lane
  baseline will measure whatever state the tree has when its turn comes.
- 18:5x design review of SH1 (read the code paths, not assumed):
  - Every real shell runs guests as wasm32 (browser jco, native wgpu via wasmtime); the local studio catalog in a guest is
    process-global memory (`LocalStorageBackbonePort` = memory fallback on wasip2; `set_host_backbone_port` never called),
    and `createStudio kind=folder` / `persistLocally` / the old `bindSpaceFile` are silent no-ops under `cfg(wasm32)`. So
    local studios never survive a reload in any shell, and a guest can never bind a file. The root fix is a host-owned local
    studio catalog service (React: store worker + dev folder/file endpoint `writeBackbonePayload`; wgpu native: file IO)
    reached by a `ReplayShellCommand` channel like `os.directory.*` / `os.create-space-artifact` (which already carries
    progress + cancel). Out of this slice's window-3 scope → recorded as blocker B1 below.
  - Host defect found: ShellHost intercepted `importSpace` on the landing controller, clicked a hidden input and re-dispatched
    `importSpace {json}` — which the same intercept caught again (a picker loop), with an arg the guest never reads (`dsl` /
    `payload`), and routed `.pack` to `importSpacePackPayload` on Home (only the studio app declares it). The guest already
    answers a bare `importSpace` with `RequestFileOpen{accept ".os", importAction "importSpace"}`, which the generic host path
    serves chunked (`{payload, name, chunk, chunkCount}`, 32 KiB chunks).
  - Host defect found: `import_os_space_from_dsl` admitted the studio with an EMPTY document name (the catalog lists by
    document name) → SH1's import law would have been red (`catalog_entries_named(name) == 1`).
- 19:0x SH2 payload = SH1's 25 files + 4 (`wp-sh2/sh2-files.txt`, stage `.🧬semio/🌐hub/s14-sh2-stage`, base
  `s14-sh2-base`, `sh2-capture.py` → `wp-sh2/payload/`, `sh2-apply.py` dry run **29/29 apply, 0 conflicts** on the tree):
  - import: decode refuses `chunkCount > 1` by name (`s.home.import-space.oversized`, bound = `IMPORT_CHUNK_BYTES`),
    reads `payload` via `kernel::IMPORT_ARGUMENT_PAYLOAD`; validate refuses an unnamed manifest (`…unnamed`); host
    `import_os_space_from_dsl` keeps the manifest's name.
  - controls: Home toolbar row `#s-home-toolbar` = `#s-home-create-space` + `#s-home-import-studio` ("Import Studio" /
    "Studio importieren", `importSpace` without args); every local row gets "Remove from Home" / "Aus Home entfernen"
    (`deleteVirtualFileSystemNode`, icon eye-off).
  - ShellHost: the landing `importSpace` intercept, its hidden input and the now unused `landingControllerId` removed.
  - laws added: decode (1 chunk / 2 chunks / bare), studio `.os` export → import round trip, unnamed refusal; Home window
    tests re-stated (local row = open + remove; ephemeral = open + promote + persist + remove; toolbar buttons by key;
    German toolbar/removal labels).
- 19:2x surface-schema: the new `HomeConfig.retiredLocalStudioIds` made the viewer's projected config lane drift (`bun …/📇️registry/📜️script.ts
  surface-schema --plugin space --check`: tree 0 drift, overlay 1 drift) → regenerated in the overlay, the 5 viewer projection leaves joined the set.
- 19:3x space TS oracles (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`), measured on tree AND overlay:
  `home-directory-projection-persistence-check` 11 clean both; `plugin-identity-check` 11 clean both; `persistence-data-class-check` 11
  clean both; **reds on the tree (pre-existing, not SH1's):**
  - `home-directory-event-page-owner-check`: the Home retained-limits schema (`HomeRetainedCommandLimits`) is self-contradictory since
    the shared `RetainedCommandLimits` shape changed 09-15 (owner narrowing demands `scalarBytes`, shared byte budget forbids it and
    requires `configValueBytes`) and still pinned the pre-S4 4 MiB / 16 MiB step budgets (code: 1 MiB). Fixed: fixture + schema →
    shared byte budget (`configValueBytes` = new `HOME_CONFIG_VALUE_BYTES` = base + 136, the literal the config admission used);
    Rust law pins all seven limits to code constants (was 2). Then its source-string proof was stale since the page emission moved
    into `HomeConfig::directory_event_page_emit` (09-26) → re-stated against the config source (hostile mutations `replaceAll`).
    Overlay: **27 checks clean**.
  - `home-directory-identity-rows-check`: 3 stale source strings (manageSpace import line, `spaceEngine` read the same file twice →
    count 6 ≠ 3, `render_rows_wrapped` gained `&TreeWindows::unhosted()`) → fixed; overlay **54 checks clean**.
  - `interactive-job-catalog-check`: studio `SpacePlayRetainedCommandLimits` narrowing stated 16 contracts / 15 bounded / 25 batch;
    the fixture (Rust-pinned to code) has 40 / 40 / 0 → schema re-stated from the fixture by the one-off `wp-sh2/sh2-spaceplay-schema.py`.
    Overlay: now stops only at `descriptor publishes s.space.home@1/*#editor:bindSpaceFile as batchOnlyPendingRewrite, source says
    migrated` — the generated `✏️s/🔌️plugins/🪐️space/🔣️.json` flips only with `describe` (window 3, wasm lane), as SH1 noted.
- 19:48 native-lane baseline on the tree: space-home lib (no feature) **27 passed / 1 failed** (`generated/base-home.txt`, the
  `structural_correspondence` red); other baselines still queued.
- 19:5x asked main: land pre-freeze (like H13) or hold for window 3; default hold.

### Blocker B1 — local studios cannot persist or bind in any real shell (design, not in this slice's scope)

Measured in the code: every shell runs guests as wasm32 (browser jco; native wgpu shell via wasmtime), the guest's local studio catalog
is process-global memory (`LocalStorageBackbonePort` = memory fallback on wasip2, `store::set_host_backbone_port` has no caller), and
`createStudio kind=folder`, `persistLocally` and (before this set) `bindSpaceFile` were silent `cfg(wasm32)` no-ops answering success.
So local studios (ephemeral drafts, imports) vanish on reload and no guest can bind a file; hub spaces are unaffected. Root fix: a
host-owned, event-sourced local studio catalog — React: one `os.local-studios` document in the store worker bound to the existing
folder lane (`${S_DATA_DIR}/os`, dev endpoint `writeBackbonePayload` already writes `file://`/`folder://`), wgpu native: the same
document on its native folder backbone; guests reach it through `ReplayShellCommand os.local-studio.{persist,bind,retire}` (the
`os.directory.*` / `os.create-space-artifact` pattern, which already carries progress + cancel) and the host re-seeds the guest
catalog on Home open. This set makes bind refuse by name (`s.home.bind-space-file.filesystem-unavailable`) instead of lying.
- 20:1x persistLocally + folder `createStudio` had the same lie as bind (IO in `handle`, `let _ =`, success on wasm32): persist is now
  a third `HomeCatalogWork` route (dialog without a folder / validate / commit via the new public host
  `admit_os_space_document`, which also fixes bind + persist listing the studio with an EMPTY name — `import_os_space_from_pack`
  drops the document name); folder `createStudio` refuses by name (`folder-path-required`, `io-failed`, `filesystem-unavailable`).
  Laws: persist job validate→commit once (folder written, listed once under its name, draft retired, repeat refused), persist
  refusals by name, folder create without folder refused, bind keeps its name. Set = 38 files, dry run clean.
- 20:1x baseline e2e on the orphan session-13 serve 6580 (F2's, `S_HUB_URL` 7800 B3, pre-SH2 guests; one headless browser,
  closed): `bun wp-sh2/sh2-home-e2e.mjs sh2-base-en http://127.0.0.1:6580/` → boot PASS, sign-in PASS, toolbar has only
  Create Space (no Import Studio) FAIL-as-expected, **create hub space FAIL**: `POST /directory/commands` 202, but the row never
  appears (`generated/sh2-base-en-report.json`). Root of that: **finding W1** — Home's windowed table stamps
  `offset=0 length=27 total=41` and never moves when scrolled or wheeled (`sh2-probe-window.mjs`,
  `generated/sh2-probe-window-1.txt`); every space past row 27 is unreachable. Suspect (read, not proven): the ShellHost
  tree-window scheduler decides window-vs-panel body from the PRIMARY session's app, Home is the landing app → refreshed as a
  panel body. Routed to main (S18?).
