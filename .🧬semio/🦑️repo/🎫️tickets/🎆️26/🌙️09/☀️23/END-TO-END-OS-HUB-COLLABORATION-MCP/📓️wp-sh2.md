# WP-SH2 — Space-Home IO-Owning Job, Space-Home Reds, Kernel Lib Reds (window 3)

Session 14 slice SH2 (Opus executor, coordinator `main`), continues `📓️wp-sh1.md`. Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP.
Rules: `📓️session-14-preamble.md` (+ 13/12). Inputs `wp-sh2/`, expendable captures `wp-sh2/generated/`, durable data
`.🧬semio/🌐hub/s14-sh2-*`. Ports: hubs 8080–8089, serves 6580–6589. Guest-linked edits: prepared patch + overlay proof
during the freeze, landed compile-atomic in window 3.

## Session 14

### Session 14b

Successor agent (2026-09-28 12:0x, guest freeze ON since 12:02:46).

| # | Item | Status |
|---|---|---|
| 1 | set dry run on live tree, overlay tests, land in window 3 | tree clean of SH2 edits (38/38 `apply`, no backup dir); 2 files rebased; set now 38 files incl. the envelope/catalog fixes + capitalized row labels; overlay native `check` (`--lib --tests`, component-app-assembly) **rc 0** 12:33 (`ov8-check.txt`); overlay tests queued (overlay lane ~6 deep); wasm32 = chain only during the freeze |
| 2 | space-home IO-owning job + reachable controls | in the set; the 4 ov6 law reds root-caused + fixed in the set (host envelope drops ×5, per-call catalog port) — re-measure queued (ov8) |
| 3 | kernel lib reds + space-home reds | space-home tree reds unchanged (27/1, 95/2 — fixed in the set); kernel lib-test compile red: paged-list fixture include path fixed (test-only, landed); kernel check queued (ov9) |
| 4 | Home windowed-table bug W1 | S18 fixed; confirmed live with the permanent harness (create-hub-space + reopen PASS en + de) |
| W2 | deleted hub space stays on the live Home | **root cause hub** (member-only event filter hides `space.deleted`); fix + 2 laws LANDED 13:27; native check rc 0 + laws 4/4 (13:31); regression batch queued; live proof after 7800 on ALL |
| 5 | host-owned local-studio catalog design + prepared patch | design written (section "Design B1 (session 14b)"); prepared patch pending |
| 6 | Home e2e probe as permanent harness (rule 17) | `verify home` written, tsc 0, run live en + de (4/7 each, expected pre-landing); spec relayed to R10 |

#### Log 14b

- 12:08 `sh2-apply.py` dry run on the live tree: **files=38 apply=38 applied=0 conflicts=0** (36 targets equal `.old`; `📜️script.ts` and
  ShellHost `🟦️.tsx` 3-way-merge clean over peer changes) and `.🧬semio/🌐hub/s14-sh2-backup/` does not exist → the tree carries no
  half-applied SH2 edit. Rebased those 2 (base := tree, stage := merge) and re-captured: 38/38 plain `apply`.
- 12:10 overlay pins/drops cleared (the peer's pinned set is now committed: every pinned file equals HEAD, dropped paths are tracked);
  `sh2-overlay-sync.py` copied 2 367 changed files; payload applied to the overlay (38 written).
- 12:11 overlay `check` launched detached (tag `ov7`, pid 48806, `generated/ov7-*.txt`); native re-measure on the tree queued (tag `b14`,
  pid 48890: space-home lib, space-home `component-app-assembly`, plugin-space, kernel `sync,ureq`).
- 12:12 ov7 red in `semio-framework-ui`: `colors::DIFF_ADDED` missing — the overlay mirror never copied the gitignored generated token
  table `🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` (`.gitignore` names it explicitly, it is not under a `🤖️generated/` dir). `sh2-overlay-sync.py`
  now also mirrors `.gitignore`'s explicit generated files (`overlay-extra-files.txt`).
- 12:1x root causes of the 4 overlay-ov6 law reds (read in code, re-measure in ov8):
  - **envelope Drop witness (3 laws)**, all in `🖥️host/🦀️.rs` (set file 025): `import_os_space_from_dsl` built its vcs from a temporary
    `create_document_envelope(..).vcs.clone()` (shell dropped undetached — every studio import aborted, also on the tree) → now
    `create_backbone_document`; the file backbone write (`SpaceBackbonePort::write`, File kind — the bind commit) read
    `parsed.envelope.vcs.initial_snapshot` and dropped the parsed shell → `parsed.into_snapshot()` (the `.os` mirror now prints the
    HEAD projection, not the genesis); the dsl-only file read built an envelope just to print its pack → `create_backbone_document` +
    `export_backbone_pack`; `decode_backbone_payload` dropped the shell on a schema mismatch → `into_envelope()` + `retire_unadopted()`;
    `import_os_space_from_pack` partial-moved the envelope → `into_envelope()`.
  - **persist listing 0 (1 law)**: `catalog_port()` minted a NEW `LocalStorageBackbonePort` per call, while the catalog tracks studio uris
    per port `Arc` address and the port keeps its fallback bytes per instance → anything bound/persisted/imported was gone on the next
    listing (and every call re-parsed + re-seeded the demo studio and leaked a tracking key). Space core (set file 019):
    `catalog_port_concrete` is now one process singleton (`OnceLock`, seeded once).
- 12:20 ov8 queued (overlay lane behind EN2): check, home-feature, home, plugin, space.
- 12:2x item 6 harness (rule 17, EXISTING dirs only, open during the freeze: host TS not under `🔌️plugin/`):
  `verify home` in os-dev (`🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts` route) → `runHomeE2eCli` / `runHomeE2e` in region `🔖️HomeE2e` of
  `🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts` (steps boot, import-control, [hub: sign-in, create-hub-space], import-studio, remove-from-home,
  reopen, [hub: delete-hub-space]; every step also fails on fault lines; `withAcceptanceRecord` + `publishAcceptanceCheckResult` check
  `home-e2e`, en + de, `blocked` on no serve / missing hub credential env `OS_HUB_PROBE_EMAIL`/`OS_HUB_PROBE_PASSWORD`). Session helpers
  are reused, not forked: 14 helpers of `👥️two-human/🟦️.ts` gained `export` (no body change). `tsc` over script.ts + the 3 files
  (1 170 files) **rc 0, 0 lines** (`wp-sh2/tsc/tsconfig-harness.json`, `generated/tsc-harness.txt`). **Not run yet** (needs the landed
  guests; one serve + browser deferred to window 3 to keep the machine free for the chain).
- 12:3x native tree re-measure (b14): space-home lib **27/1** (`structural_correspondence`), space-home `component-app-assembly` **95/2**
  (`retained_config_cancel_and_cleanup…`, `structural_correspondence`) — the same two reds the set fixes (`generated/b14-home*.txt`).
- 12:4x kernel lib-test red (relayed from WG11): `🧬️retained-clone/📋️paged-list` unit test `include_str!("../../../🧫️fixtures/📦️copy/…")`
  one dir too high (fixture lives in `📋️paged-list/🧫️fixtures/`) → `../../` (test-only, landed in the tree, copied into the overlay);
  kernel `check --lib --tests` queued in the overlay (`ov9`) to see whether the reported E0618 on `RetainedCloneGrant` is real or a
  follow-on of the missing include (no `let grant` shadowing found by reading).
- 12:4x **W1 re-run (S18's fix) with the permanent harness** on S18's serve 6540 → 7800 B3 (`zsh wp-sh2/sh2-verify-home.sh --serve
  http://127.0.0.1:6540/ --hub http://127.0.0.1:7800 --locale en|de`; credential env via `sh2-hub-env.sh`, never argv): en **4/7**, de
  **4/7** — boot, sign-in, create-hub-space (row now found — W1 fixed), reopen PASS; import-control + import-studio FAIL as expected
  (pre-landing guests); delete-hub-space FAIL → finding W2 (`generated/home-e2e/s14b-w1-{en,de}/report.json`). Harness fixes: an
  unhandled `filechooser` rejection when the control is missing; the delete action pattern anchored (`^delete\b`, the row name could
  contain the word); a failed delete reloads and records `goneAfterReload`.
- 13:0x–13:2x **W2 root cause** (probes `sh2-probe-delete-live.ts` with a DOM timeline recorder, `sh2-probe-hub-delete.ts` hub-only):
  after "Delete Space" the live Home keeps the row (75 s … >240 s; a reload drops it). The hub accepts `delete-space` in 21 ms with
  `space.deleted` in the reply, but `/directory/event-page/v1?after=631` answers `throughSeqInclusive 633` with **0 events** — raw
  directory events are member-only and judged at READ time; after the deletion nobody is a member, so `space.deleted` reaches no page
  and no socket (and `directory_access_change_for_reader` owes `access-changed` only for `member.removed`). Not the 30 s settle
  deadline. **Fix landed 13:27** (hub, open during the freeze): `decide(DeleteSpace)` emits `member.removed` for every member (owner
  included) before `space.deleted` → each member's global socket owes the existing `access-changed: revoked` → the worker re-reads from
  the origin, whose member-only pages no longer carry the space. Laws: decider `delete_space_revokes_every_membership_before_the_space`,
  live socket `a_member_of_a_deleted_space_is_told_its_access_was_revoked` (`sh2-w2-patch.py`, backup `s14-sh2-backup/w2/`).
  Native `check -p semio-hub --lib --tests --bins` + both laws in ONE hold (coordinator slot `FLEET_TICKET_STAMP=20260928120003`),
  13:28–13:31: **check rc 0** (83 warnings, type-checked), **decider laws 2/2, socket laws 2/2** (`generated/w2-hub-all.txt`). Regression
  batch around deletion (invite redemption, admin shutdown, CAS space-delete, access-changed) 13:46–13:49 **all ok** (lib 19, bin 7;
  `generated/w2-hub-regress.txt`). Live proof waits
  for 7800 on ALL (the chain builds os-hub after the publish).
- 13:3x serve 6580 stopped (idle; `serve-hold` log shows `stopped`).
- 13:2x Home row-action labels were lowercase localized words ("delete: <name>", de "löschen: …"), not raw ids (my first relay said
  ids — corrected) → set file 026 capitalizes them en + de ("Open/Rename/Share/Delete/Manage", "Öffnen/Umbenennen/Teilen/Löschen/
  Verwalten"); Home window laws match case-insensitively.
- 13:1x own serve for the probes: `serve-hold --serve http://127.0.0.1:6580/ --hub http://127.0.0.1:7800` (w2-detach pid 49307, log
  `.🧬semio/🌐hub/s14-sh2-serve/serve-hold-6580.log`), ready in 3 s.

## Session 14 (evening, predecessor)

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

### Design B1 (session 14b) — host-owned, event-sourced local studio catalog

Measured facts it builds on (code read 12:1x–12:3x, 09-28): the dev serve's `/semio-backbone` endpoint (`🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
`readBackbonePayload`/`writeBackbonePayload`) already reads/writes `file://<path>` (raw archive) and `folder://<dir>` (sqlite
`.semio/documents.db`, `documentId` default `studio`, the same convention as native `vcs::FolderSqliteStorage`); payloads are
canonical document archives (`encodeDocumentArchiveBytes` = `{parent_pack, parent_spr, members[]}`). The store worker already opens
persisted-local-only facets through `PersistenceBinding {kind:"folder", dataClass:"persistedLocalOnly", path:`${dataDir}/os`}`
(`identityActorConfig`), `dataDir` = `VITE_S_DATA_DIR`; without it the facet is ephemeral-local-only. Guests reach host IO only through
`Effect::ReplayShellCommand` (ShellHost routes `os.directory.*`, `os.create-space-artifact` to worker requests with request ids, progress
UI and `*-cancel`), and the host feeds Home through receipt-sealed, job-routed actions (`applyDirectoryEventPage`,
`ShellHost/📇️directory-bootstrap/`).

1. **Authority.** One host-owned event log `os.local-studios` (schema `os.config.local-studios`, singleton by schema id like
   `os.config.identity`), data class persisted-local-only (folder lane `${dataDir}/os`; no data dir → ephemeral-local-only and Home says
   so). Events (no CRUD): `studio-admitted {spaceId, name, storage: folder|file, target, archiveSha256}`, `studio-bound {spaceId, file}`,
   `studio-retired {spaceId}`. Each studio's own document lives beside it: `folder://${dataDir}/os/studios/<spaceId>` (persist/import) or
   `file://<path>` (bind), written with `writeBackbonePayload` semantics (dev endpoint in React, native fs in wgpu).
2. **Guest → host (commands).** Home's retained jobs keep validate/commit; on a host without a filesystem (every wasm32 guest) the commit
   stage emits `ReplayShellCommand os.local-studio.{admit,bind,retire}` with `{spaceId, name, target?, archive}` (archive = the studio's
   `export_backbone_pack` wrapped as a document archive, ≤ the retained raw budget) instead of refusing, and answers progress
   `space-home.catalog.requested`; import additionally admits in memory so the row shows at once (pending chip until the host echo).
3. **Host job.** ShellHost routes `os.local-studio.*` to worker request `local-studio-command {requestId, command}`; the worker validates,
   writes the studio document (progress = bytes written / archive bytes, cancellable until the write commits via
   `local-studio-command-cancel`), then appends the catalog event (the write-then-append order makes a cancelled or failed job leave no
   event). Refusals are named `os.local-studio.*` with en + de notice texts (replay refusal table).
4. **Host → guest (projection).** The worker streams `local-studio-page {throughSeqInclusive, events}`; ShellHost delivers it to Home as the
   job-routed `applyLocalStudioPage {pageJson}` (same receipt/settle protocol as `applyDirectoryEventPage`, owner module beside
   `📇️directory-bootstrap`). The guest folds the page into its (now process-singleton, set file 019) catalog port idempotently by
   `spaceId` (admit = `admit_os_space_document` of the decoded archive, retire = config tombstone already in this set), bumps
   `catalog_generation` once per page. On every Home mount the worker replays the log from seq 0 → **reopen after reload lists every
   persisted/bound/imported studio**.
5. **wgpu native shell.** Same route and page in its Rust host (`os-shell` native): the log on its native folder backbone, the studio
   documents via `open_folder_space_backbone`/`open_file_space_backbone` (host-side, where they compile).
6. **Laws.** Guest: commit on a no-filesystem host emits exactly one `os.local-studio.*` command and writes nothing; `applyLocalStudioPage`
   admits once per id (replay idempotent), retire hides, generation bumps once per page. Host (vitest): write-then-append ordering,
   cancel before commit leaves no event and no file, refusal codes, seq-0 replay on remount. Oracle: the written folder is read back by
   `bun:sqlite` (third party) and the archive by the TS archive codec; e2e: `verify home` gains `bind-studio-file` + `persist-studio` and
   `reopen` asserts they survive the reload.
7. **Scope for window 3+.** Guest part (space-home + space core) is a second prepared set after the current 38-file set lands; host part
   touches ShellHost + store worker + os wire schema (`BackboneWorkerRequest/Response`) → owner of the React shell lane (S18) or SH2 with
   a new `ShellHost/🏠️local-studio-catalog/` dir (R10 taxonomy registration needed).
8. **Alternative C found 13:4x (decide before the patch).** Guests already have host-owned async storage: kernel
   `Effect::StorageRead/StorageWrite/StorageDelete {req, key[, bytes]}` (wit `storage-read/-write/-delete`, browser bundle
   `hostAsync.storageWrite` → `effect-request`), executed by the plugin host's `dispatch_storage` against `services.storage_backend`
   with per-package byte quotas, a deadline and `ctx.cancel` (cancellation before dispatch answers `capability-revoked`) — i.e. the IO,
   quota and cancellation of step 3 exist. With it the guest's catalog port could persist through the host (Home's retained jobs
   await the `Respond` of their `StorageWrite` in the commit stage; Home open reads the catalog key) and no new shell route, worker
   request or page protocol is needed. Open questions (not verified): which storage backend the React shard host binds
   (persisted-local-only or memory), whether app-plugin guests (not only actor packages) may emit these effects under their
   capability grants, and how the retained job observes the `Respond`. B is proven-pattern but ~5 new host pieces; C is smaller if
   the backend persists. **Prepared patch not started** — the choice between B and C needs the React-host / plugin-host owner.
9. **Coordinator decision 13:4x: route B**, SH2 owns both halves. Because the event log is a new schema-first OS config vocabulary with
   Rust twins in `💻️os/🎚️config` (frozen), library taxonomy/schema-catalog rows and a plugin-host exhaustive case, the whole route
   lands as ONE window-3 set (`wp-sh2/b1/`, overlay-proven), modelled 1:1 on the opening-preferences vocabulary (`set-default-app`
   upsert keyed → `admit-local-document` keyed on `documentId`; `clear-default-app` → `retire-local-document`):
   - **vocabulary** `os.config.local-catalog` (`LocalCatalog { documents: [LocalDocument] }`, `LocalDocument { documentId, schema, name,
     storage: folder|file, target, admittedAtMs }`): leaves `📥️admit-local-document`, `🗑️retire-local-document` (manifest 🔣️.json,
     payload 🧬️schema, 🦀️.rs, 🟦️.ts, unit + fixture-vector tests, fixtures), dispatch in `🧬️mutations/{🦀️.rs,🟦️.ts}`, plugin-host case
     `🔌️plugin/🖥️host/🧪️tests/📇️mutate-os-config-local-catalog/`, oracle no-oracle decision, taxonomy + schema-catalog rows (R10).
   - **host TS**: `localCatalogActorConfig(actor, dataDir)` (worker, identity pattern: folder lane `${dataDir}/os`, else ephemeral);
     ShellHost region `🔖️LocalCatalog` (own module `🏛️ShellHost/📇️local-catalog/` — new dir, R10): open + fold at boot (local-first,
     no hub needed), replay route `os.local-catalog.admit {documentId, schema, name, storage, target, pack, spr}` = job (stage
     "writing": worker `open` with the folder/file binding + `send localDocumentArchive` + `close`; stage "recording": facet mutation
     `admitLocalDocument`; cancellable until the archive is sent; en + de notices), `os.local-catalog.retire {documentId}`,
     `os.local-catalog.open {documentId}` (openDocument with the entry's binding, as the sync card does); feed Home with
     `applyLocalCatalogPage {pageJson}` on Home mount and after every committed job.
   - **guest**: Home persist/bind/import commit stages on a host without a filesystem emit `os.local-catalog.admit` (pack/spr of the
     studio's `export_backbone_pack`) instead of refusing; `applyLocalCatalogPage` → Home config `localCatalogJson` projection → rows
     (origin local, `persistedLocalOnly`), their open → `os.local-catalog.open`; Remove-from-Home stays the config tombstone.
   - **laws**: leaf vectors (Rust + TS, same fixtures), plugin-host exhaustive case, worker/ShellHost vitest for the job (write before
     record, cancel before write leaves nothing), Home guest laws (commit emits exactly one admit; page apply idempotent), e2e
     `verify home` gains persist-studio + reopen-survives-reload.
10. **B1 build log (14:0x–):** stage `.🧬semio/🌐hub/s14-sh2-b1-stage`, base `…-b1-base`, list `wp-sh2/b1/b1-files.txt` (helpers
    `b1-stage.py add|rebase`, `b1-capture.py`, `b1-apply.py`; ShellHost's B1 base = the 38-file set's staged ShellHost, so B1 lands
    after it). Written so far (not compiled yet — overlay lane busy): the vocabulary (2 leaves × manifest, payload schema, Rust, TS,
    unit + vector tests, fixture quintet; dispatch Rust/TS; crate module decls + descriptor registration; oracle decision + catalog +
    manifest), plugin-host exhaustive case `📇️mutate-os-config-local-catalog` (feature, Rust, TS adapters), worker
    `localCatalogActorConfig`, host module `🏛️ShellHost/🗂️local-catalog/🟦️.ts` (request validation + target/binding resolution,
    whole-record envelope, archive codec, Home page arguments, en + de notices). Open path decided: the host keeps the durable copy;
    `os.local-catalog.open` re-hydrates the guest from the folder archive (pack/spr through the import lane) and then routes to the
    studio — later edits persist by persisting again (continuous folder sync of the studio app is a separate step).
11. **B1 state 14:4x (prepared, overlay-applied, verification queued):** 42 files (`wp-sh2/b1/payload/manifest.json`; dry run on
    the overlay 42/42 `apply`; on the tree 5 files conflict by design — their base is the 38-file set, so B1 lands right after it).
    Final shape: guest commits on wasm32 (persist → storage `folder` + the dialog's folder, bind → `file` + the path, import → `folder`
    + the device default) emit `os.local-catalog.admit {documentId, schema, name, storage, target, pack, spr}` (the studio's own
    `export_backbone_pack` pair, base64); the host validates (`localCatalogAdmissionV1`), writes through a worker actor with the
    folder binding and VERIFIES the write by reading the lane back (externalChanged → `documentArchiveReplaced` byte-equal), then
    records `admitLocalDocument` in the facet (envelope + archive) and hands the pair back as `applyLocalCatalogDocument` — a new
    chrome-audience retained Home job (validate: base64 + the pair is the named `s.space` document; commit:
    `import_os_space_from_pack` under its own id and manifest name — idempotent — retires the ephemeral draft, bumps the generation;
    never re-emits an admission). On every landing-app mount the host re-hydrates each kept document it has not handed that
    instance yet (read-back from its lane). Remove from Home of a persisted studio also emits `os.local-catalog.retire` (files stay
    on disk). Also fixed on the way (set 1, host 025): `import_os_space_from_pack` keeps the head manifest's name. Laws added
    (guest): re-hydration lists once under its name + idempotent + removal unkeeps; malformed/foreign pair refused by name; import
    commit emits exactly one admission whose pair decodes to the imported studio. `tsc` of the B1 host TS in the overlay: 0 errors in
    B1 code (the 3 remaining ShellHost errors are the overlay's dual-module-identity/stale channel-version class that the untouched
    identity `open` shares). Queued: overlay hold `ov12 b1` (config crate, space-home component-app-assembly, plugin-space, kernel
    lib tests). Not yet: generated test-host run of `📇️mutate-os-config-local-catalog`, the space plugin descriptor regen
    (`describe`, wasm lane, window 3), a live e2e.
- 14:4x **item 1 overlay proof (ov11, one hold 14:21–14:25):** space-home `component-app-assembly` **109/109**, space-home lib
  **28/28** (all four ov6 reds fixed); kernel lib-test compile red #2 found: `🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs:311` E0618
  — `let grant = grant(&fixture)` called a local `grant` binding shadowing the fixture fn (line 291) → the first binding renamed
  `copy_grant` (test-only, landed in the tree 14:28, backup `s14-sh2-backup/kernel-fixture/`).
