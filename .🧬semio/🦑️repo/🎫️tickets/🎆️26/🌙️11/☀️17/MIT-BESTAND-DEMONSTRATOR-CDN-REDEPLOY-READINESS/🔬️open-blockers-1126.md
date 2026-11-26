# Open CDN Blockers, Consolidated (Read-Only Audit, 2026-11-26)

Scope: the 91 markdown notes in this ticket modified since 2026-11-24 (ls -t), read in full or by tail where long. Newest notes win. Latest timestamps: `release-verification-current.md` and `flow-extension-operator-replay.md` (2026-11-25 20:21 to 20:28 PST). `🚀️redeploy-log.md` ends 2026-11-25 08:40 and is superseded wherever a later note disagrees. `🗂️changed-files.md` (214 KB) is a path inventory and was not used for state. No build, test, lock or source change was made.

Status keys: OPEN = still blocks release. RESOLVED-SRC = fixed in source, not yet proven by a fresh build and browser run. RESOLVED = proven on the cited evidence. SUPERSEDED = replaced by a later note.

## 1. Snapshot (latest evidence, 2026-11-25 20:22 PST)

- Roster is 12 plugins plus 21 extensions = 33 components (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json`). SUPERSEDED: 11 plugins + 21 = 32 (release-verification 05:49 baseline, vcs-stdio-runtime-custody).
- Byte confirmations: 12 of 33 (`release-verification-current.md`, 20:22). `🗑️generated/confirmed/` holds 12 files. SUPERSEDED: 1 of 32 (`site-build-preflight-1125.md`, Nov 24).
- Browser matrix: 28 pass / 24 fail of 52 baseline cases (14 of 26 EN, 14 of 26 DE, zero skipped or flaky). Source: `release-verification-current.md`, `📦️strict-cdn-acceptance.md`. SUPERSEDED: 10 of 16 (`📊️status-2026-11-25.md`, 05:50).
- Regular production Vite candidate: 2,103 files, 826,856,511 bytes, 32 encoded cores, largest asset 72,504,360 bytes. This candidate predates the menu-renderer fix and the GIS/Flow/Stdio descriptor refresh, so it is not a release candidate. SUPERSEDED: 2,496,793,374-byte archive `site-20261125T134801Z.tar.gz` (05:50).
- Fleet: execution scheduler configured but gRPC probe returns `ERR_HTTP2_STREAM_CANCEL`; all three cache probes unreachable (`🔬️fleet-recheck.md`, 2026-11-25 18:32). Builds run native-only (`SEMIO_OFFLOAD=never`). Environmental blocker, not an app defect.

## 2. Site-Level Gates

### G1. Vite build (regular production target)
- Latest state: complete at 2,103 files, but it must be regenerated after the open guest rebuilds (Demonstrator, Procedural, Flow standalone, GIS, FEM, Stdio, Process, Puzzle, Playbook, Sourcing). The menu-renderer empty-option fix "postdates the current Vite candidate" (`release-verification-current.md`).
- Stale plugins identified by the comparison: 8, including Playbook and standalone Sourcing (same note).
- Known build-side faults: `native-wasm.sh site` runs `🔨️modules/📦️site/📜️script.ts build` directly and does not invoke surface, editor, node-graph, puzzle, flow and canvas prerequisites (`🔬️site-build-preflight-1125.md`, blocker 3, Nov 24; not re-verified since).
- Live-process hazard from the same note: `overlay-serve.mjs` pid 37114 serving an old Vite output. Current status not re-verified.
- Status: OPEN.
- Source: `native-site-verify.sh`, `native-wasm.sh`, `captured-site.py`, `🔨️modules/📦️site/📜️script.ts`.
- Reproduce: `bash "$T/native-site-verify.sh"` (line 13 runs `MEM="${MEM:-3000}" bash native-wasm.sh site`; the `VERIFY_QUEUE_RECEIPTS` switch named in the Nov 24 preflight no longer exists in the script).

### G2. 96-case English/German Chromium acceptance
- Roster: 52 baseline cases (26 per locale, including 16 pane boots) plus 44 behaviour cases (22 per locale). Discovery is verified before Chromium starts (`accept-site.py`).
- Latest result: 28 pass / 24 fail (diagnostic run on a compact site with older guest bytes). Not a release result.
- Failures on the diagnostic run (quoted from `📊️release-verification-current.md`, `📦️strict-cdn-acceptance.md`):
  - Generator (EN and DE, boot and undo/redo): empty 3D preview, `[DEBUG]` console flood. Per-app entry below.
  - Aussuchen preview: "No selection" (see Demonstrator).
  - Energie run/cancel: `EnergySimulationRunJob` assertion at run-job line 278 (see Energy).
  - Statik undo/redo: queued Results caption (see FEM).
  - Koordinator and Aggregator undo/redo: unscoped catalogue row IDs (test fixed; guest still fails, see CAD and Puzzle).
  - Verfolgen undo/redo: unscoped Inspector ID (test fixed; GIS now passes on raw overlay, see GIS).
- Status: OPEN. Requires one consistent test revision run against freshly built guests and the final Vite output. `accept-site.py` writes `🗑️generated/static-accepted.sha256` only after coverage, origin and manifest checks pass.
- Reproduce: `repo tools exec --light python -- -I "$T/accept-site.py" "$OUT"` (or `accept-site.sh [port]`).

### G3. package-site.sh and verify-release-archive.py (GitHub Pages 1 GB limit)
- Limit: `PAGES_MAX_PUBLISHED_BYTES = 1_000_000_000` (`package-site.py:13`). The checksum manifest counts toward it.
- Historical archive: 2,496,793,374 bytes, over the limit. SUPERSEDED for the candidate: 826.9 MB regular Vite output, 774.9 MB compact diagnostic.
- Per-file: the GitHub Pages 100 MB per-file limit was exceeded by nine files in the old site (largest 284 MB, `📊️status-2026-11-25.md`). The candidate's largest file is 72.5 MB, so it passes. The README targets the CDN; GitHub Pages is a fallback only.
- Status: OPEN until a final candidate is packaged; the limit itself is no longer the blocker.
- Gate order: `release-receipts.py status` must show 33 of 33; `accept-site.py` must write the accepted manifest; `package-site.py` re-reads it and runs `verify-release-archive.py` on every member.
- Reproduce: `package-site.sh <SITE> <OUT_DIR> "$generated/static-accepted.sha256"`.

### G4. Release receipts (`release-receipts.py`)
- Latest: 12 of 33 confirmations. Missing 21 extensions and 9 main roots (Demonstrator, Procedural, Process, Sourcing, Playbook, Puzzle, Stdio, Flow and others, see below).
- Caveat from the Nov 24 preflight still applies: `record` does not check the authored semantic descriptor. Only `verify` does (`authored-release-descriptors.ts`).
- Status: OPEN.

### G5. Fleet and compiler capacity (cross-cutting)
- Repeated host-warden cancellations at the 1,536 MiB library ceiling and 4,096 MiB final ceiling: actor-input host unit 1,638 to 2,179 MiB; FEM library 1,871 MiB; FEM and GIS finals 4,168 and 4,279 MiB; PDF family 8 at 5,985 MB; Process/Demonstrator and Procedural finals 6.4 GB in older runs.
- Status: OPEN for Demonstrator (root library over memory with `no-endpoint`), Process (`semio_hub_process` waits for a larger worker), and FEM (composition library). Mitigated by carving the apps into their own libraries (see per-app).
- Warden policy fix for false danger (`🔬️demonstrator-oom.md`) is RESOLVED-SRC; 27 of 27 warden tests pass.

## 3. Per-Component Blockers

### CAD (Koordinator; plugin `cad`, extensions `cad-extension-aec-building`, `-energy`, `-structure`, `cad-extension-spatial-shape`)
- Work-limit refusal, `interactive-job.worker-continuation` "WorkLimit" and `authority (work-limit)` in Koordinator and Bearbeiten. Root cause in `🔬️work-limit-root-cause-1125.md` is REFUTED (`redeploy-log`, 2026-11-25 08:40: the held-fuel regression test found `held_fuel == 0`). The cap was reverted (`🔬️work-limit-return-reverted.md`). Status: OPEN; the `first_work_refusal` probe is the next decider.
- `addObject` refused publication: "existing pane member reached the child store without original command authority". Fix: retained `addObject` producer in `🔬️cad-add-object-1125.md`. Status: RESOLVED-SRC; native typecheck and strict browser undo/redo pending.
- `setContributions` stall, `one-item publication blocked` x4094. Not root-caused (`🔬️contributions-stall-1125.md`). Status: OPEN.
- Set Active Example hang on the November 21 guest. Guest is stale, so status: OPEN until rebuild.
- Four CAD extensions were withheld from the old output. Now all enabled, with browser cases added. Status: RESOLVED-SRC (21 extension confirmations recorded, CAD ones included).
- Retained-command native boundary: production resolver refused `semio-s-artifact-cad-cad --test cad_editor_unit` at 266,480 bytes and 2,640 active lines; after the move 262,130 bytes and 2,611 lines. Status: RESOLVED-SRC. Broader plugin registry digests still differ (`🔬️cad-retained-command-native-boundary.md`).
- Source: `🧰️framework/.../🧵️retained-command/🧯️raw-allocation-close-oracle/🦀️.rs`, `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`.
- Reproduce: `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-cad-cad --test cad_editor_unit` (native); Koordinator cases via `accept-site.py`.

### Demonstrator (Aussuchen, Bearbeiten; plugin `demonstrator`, Process and Sourcing mounted inside)
- Aussuchen preview stays "No selection". Staged descriptor has no `rows` interaction subscription on Preview. Current source subscribes Preview to rows. Aussuchen also still runs on the November 23 guest (`runtimeVariant`). Status: OPEN until new descriptor and browser run (`🔍️stock-selection-1125.md`).
- `[DEBUG] big work quote 49999995 available=Some(49999995)` console line in Aussuchen and Bearbeiten boots (`📊️status-2026-11-25.md`). Work-quote change reverted, so the line remains unless removed separately. Status: OPEN.
- Bundled demonstrator guest cannot link: final link 4.7 to 4.9 GB vs 4,096 MB budget (`redeploy-log` 2026-11-25 04:25). Mitigated by six-surface split into separate compiler units (`🔬️demonstrator-mounted-app-codegen.md`, router `semio_hub_demonstrator_router`), whose release link is not yet run. Status: RESOLVED-SRC; OPEN on build.
- Unit 1,047 of 1,062 refused: "fleet path reported `no-endpoint`", host memory admission above 12.27 to 12.47 GB RSS (`🔬️compact-release-headroom.md`, `💽️`). Status: OPEN.
- Component size: `semio-hub-demonstrator-component` 4.9 GB and `semio-hub-process-component` 5.5 GB need carving to rebuild on a 16 GB host (`📊️status-2026-11-25.md`). Partially addressed by the split. Status: OPEN.
- Old Demonstrator OOM (`🔬️demonstrator-oom.md`, SIGKILL of rustc by the warden) SUPERSEDED by the warden policy fix (critical no longer equals danger).
- Source-boundary violation for `semio-s-artifact-stdio-pdf-lift` (4075 then 4094 of 4000 lines, `🔬️source-boundary.md`, Nov 24) RESOLVED by the lifter split (2548 lines).
- Source: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json`, `🔨️modules/🧩️runtime/🔣️.json` (`withheldExtensions`, `runtimeVariant`).
- Reproduce: `native-final.sh` (verify receipts, then `demonstrator-prepare`); `native-queue.sh demonstrator`.

### Energy (Energie; plugin `energy`)
- `EnergySimulationRunJob` panic, assertion at run-job line 278: numerical admission holder not terminal after moving out its job; the job became visible before its Source settled. Fixed in `🔬️energy-admission-handoff.md`. Native `numerical_admission_handoff_leaves_terminal_owner` passed (19:23 PST, 2026-11-25). Status: RESOLVED-SRC; release rebuild and `energie run-cancel` browser case pending.
- Renderer trap in simulation on the old guest (`redeploy-log`, Nov 25). Status: OPEN until the rebuilt guest runs.
- Packet page admission, 10 corpus rows: RESOLVED-SRC (`🔬️energie-packet-page-admission.md`).
- Core is now 49.5 MB compact; the old 267.6 MB figure is SUPERSEDED.
- Source: `✏️s/🔌️plugins/🔋️energy/Cargo.lock`, `⚡️simulation/⚙️engine/🧪️sim/♻️close/🧪️tests/🦀️.rs`, `🧪️sim/🧺️batch/🧪️tests/🦀️.rs`.
- Reproduce: `SEMIO_OFFLOAD=never repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked --manifest-path '✏️s/🔌️plugins/🔋️energy/Cargo.toml' -p semio-s-plugin-energy-engine-sim --lib --test numerical-admission-handoff`.

### FEM (Statik; plugin `fem`)
- Lean pair `wL2CqpJ`: FEM library terminated at 1,871 MiB against 1,536 MiB ceiling (`🔬️fem-mounted-app-codegen.md`, `🔬️native-final-link-bounds-1125.md`). Final link cancelled at 4,168 MiB against 4,096 MiB (`wKvvlU8`, PID 75917). Mitigation: four mounted apps split into role libraries plus `semio_hub_fem_router`. Status: RESOLVED-SRC; no successful FEM build yet.
- Statik Results queued caption "Analysis queued" for 60 s; fallback for `result_stage == None` (`🔬️statik-automatic-solve-1125.md`). Status: OPEN; the first check is `statik undo-redo` expecting `Case: dead` or `Lastfall: dead` on the Concrete Forest route.
- FEM and GIS aggregate pair never sealed because FEM failed (`🔬️gis-fresh-descriptor-and-browser.md`). Status: OPEN.
- Reproduce: `native-final.sh`; `verify-fem-app-carve.py`.

### Flow (standalone engine; `flow`, 9 extensions)
- `Flow Wasm exports and exact memory are required` (Generator boot, EN). Cause: shipped asset `flow_core_bg-DtORhipd.wasm` (4,920,217 bytes, SHA `c2244d53...ffdb`) exports only memory, none of the seven `flow_bridge_*` functions (`🔬️flow-standalone-browser-abi.md`). Fix: corrected standalone producer, 15,884,064 bytes, SHA `488b8438...cf57`, seven functions present, bindings probed. Status: RESOLVED-SRC on the private Vite build (2,093 files, `flow_core_bg-DbbG8ydm.wasm`). OPEN until the final Vite output carries it.
- `plugin.completed.decode` on the history cases (math and BREP requests refused by the Procedural completion decoder, 420.1 s run). Partly addressed by the Pack fault reader (`🔬️completed-extension-frame.md`). Status: OPEN; the case-specific return tag still needs the captured browser event.
- Generator preview empty: `scheduled toolRunStart` refused during document load. Fixed by holding scheduled work on the per-instance load gate (`🔬️scheduled-document-dispatch.md`, `🔬️renderer-continuation-document-load.md`, 32 DOM tests pass). Status: RESOLVED-SRC; browser replay pending.
- Standalone build: `E0433` missing `RetainedCloneStep` in storage close stage. Fixed, rebuilt. RESOLVED.
- Standalone producer depended on umbrella SDK, which pulled `semio-plugin-actor-input-wire` into the browser graph. Replaced by two owner crates; 938 units, zero actor-input units (`🔬️flow-standalone-extension-owners.md`). RESOLVED-SRC.
- Queue-slot starvation of the Generator preview: witness `generator-queue-slot-law.rs` passed. RESOLVED (native); browser pending.
- Flow BIM and Dictionary lost transpile records. Retained raw hashes match their original receipts (`strict-cdn-acceptance`). RESOLVED.
- Flow host read/cancel changes: standalone Flow executable custody predates them (`heavy-pair-schedule-1125.md`). OPEN until rebuild.

#### Flow extension operators (9 modules: `flow-extension-bim`, `-brep`, `-dictionary`, `-draw`, `-list`, `-logic`, `-math`, `-primitive`, `-text`)
- Exact error: `plugin.completed` fault, origin `framework`, code `invariantViolated`, message `registry admission requires its original source slot and caller`, zero retained progress (258-byte Pack bodies, SHA-256 `e9edb2b710a0ba932bb97f261a50da366c8fca84fd1e7d29a3753e08f1a81fe3`; `🔬️generator-extension-registry-custody.md`).
- Cause: `SharedRegistryIssuer` compared the per-turn recipient and original epoch exactly on every slice. Source fix `9b948fc54a4` adds forward epoch continuation (`RESOLVED-SRC`). Native registry admission suite passed three tests.
- Status: OPEN. All nine retired binaries are excluded from publication. `retired-release-wasm.json` records their raw digests; `test-retired-release.py` refuses them even if restored. Confirmations are 12 of 33 with nine extension roots pending. `flow-extension-operator-replay.md` (20:23): all nine returned the same registry-invariant refusal on the diagnostic site.
- Reproduce: `probe-staged-flow-operators.ts`, `evaluate-flow-extension-browser.ts`, `test-retired-release.py`, `verify-flow-browser.mjs`.

### GIS (Verfolgen; plugin `gis`, now also requires `stdio`)
- `interactive-job.publication-stalled` on Add Feature (`noteShellCommand`, `🔬️statik-verfolgen-fresh-run-audit.md`). SUPERSEDED: fresh GIS component (`e68bca31...60a6`, 144,126,651 bytes) passes boot, Add Feature, undo and redo, routes and cycle on the private raw overlay (`🔬️gis-fresh-descriptor-and-browser.md`, 17:53 and 17:58 PST; 267 requests, zero failures). Status: RESOLVED on diagnostic overlay; OPEN for the normal receipt and sealed release.
- Descriptor pair: the checked pair (`fb68e4...`) differed from the fresh semantic digest (`13a0a7f8...`). Reconciled through the descriptor writer. RESOLVED-SRC.
- Standalone GIS catalogue closure now `["gis", "stdio"]` (`🔬️vcs-stdio-runtime-custody.md`, 14 assertions pass). RESOLVED.
- Drawing import fallback (1041 units, PDF umbrella reachable through the Drawing importer): conditional, no source change. Status: OPEN, apply only if the next locked GIS release graph still includes the PDF umbrella.
- Final-link ceiling: GIS final 4,279 MiB against 4,096 MiB in `wKvvlU8` (PID 84943). Lean GIS linked at 144 MB. Status: RESOLVED on the lean profile; browser pending.
- Source: `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️definition/🦀️.rs`, `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import`.

### Playbook (plugin `playbook`, extension `playbook-module-procedural`)
- Descriptor stale: authored `753cbc43b969` vs staged `0c2df8650dc2` (hover target and granularity arguments; `🔬️restored-roster-descriptor-custody.md`). Status: OPEN until rebuild.
- Included in the Flow/Playbook tail pair and the release-root boundary table, which passed (2,044 closures, zero over limit). RESOLVED for boundaries.
- Reproduce: `native-queue.sh playbook`.

### Procedural (Generator; plugin `procedural`)
- Guest linked: `wZeuq82`, 178,180,715 bytes, SHA `dc2e8c4a...caf7`, 522,732 functions. Release custody not closed: descriptor pass 4 cancelled at 1,759 MiB against 1,536 MiB (`actor-input` host). Status: OPEN.
- Host actor-input library cancelled at 2,179, 1,638, 1,833 and 1,759 MiB (`🔬️descriptor-actor-input-ceiling.md`, `🔬️descriptor-emitter-release-handoff.md`). Status: OPEN; a split is RESOLVED-SRC in `actor-input-wire-carve.md` (third carve pending).
- Generator 0 of 4 on the earlier overlay (`generator-before-flow-fix`). Empty preview root cause (`🔬️generator-preview-stall.md`: one wire byte per two turns) RESOLVED-SRC (phase-1 loop).
- PDF model family 8 refused at 5,985 MB in the Generation3d editor (`redeploy-log`, `💽️compact-release-headroom.md`). Drawing export now writes through the PDF codec; Procedural graph no longer includes PDF umbrella or family 8. RESOLVED in graph; compile pending.
- `[DEBUG] gen-preview` prints and `debug_gen_preview` re-export: temporary, supposed to be removed; status not verified. OPEN.
- Tool-run tick, scheduled start during load: see Flow.
- Reproduce: `native-queue.sh procedural` (`JOBS=2`); `verify-generator-overlay.py`.

### Process (Bearbeiten's process3d mount, `process` plugin, four `process-extension-*`)
- `semio_hub_process` waits for a larger fleet worker (`🔬️process-mounted-app-codegen.md`, run `wUc5aA5` stopped at 1,138 units). Status: OPEN; split into app libraries RESOLVED-SRC.
- Standalone Process descriptor semantically differs from the November 21 staged descriptor. Status: OPEN.
- Process 3D editor compiled optimised at 2,843,230,208 bytes peak RSS (`🔬️release-stage-job-scheduling.md`). Stage scheduling change RESOLVED-SRC (12 tests, 54 assertions).
- Aggregate Demo + Process pair `wUc5aA5` XLSX error `E0609` (`WindowKitStableCellEdit.locale`): an older admitted producer had five fields, current has six. Fix in tree; a fresh consistent admission is needed. Status: OPEN.
- Reproduce: `native-queue.sh --pairs demonstrator process`.

### Puzzle (Aggregator; plugin `puzzle`)
- 5D editor at 4,006 active lines, over the 4,000-line boundary. Rewrite adapter moved into `semio-s-artifact-puzzle-5d-rewriting`; editor now 3,459 lines (`🔬️puzzle5d-rewrite-source-boundary.md`). Status: RESOLVED-SRC; Rust rewrite suite awaits compilation.
- Descriptor stale: authored `e116c3e5282c` vs staged `3ac61866f928` (asset digest and media declarations). Status: OPEN.
- Aggregator undo/redo: caller-loan errors (`strict-cdn-acceptance`). Status: OPEN.
- Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/...` (paths in `🔬️puzzle5d-rewrite-source-boundary.md`).
- Reproduce: `native-queue.sh puzzle`.

### Sourcing (`sourcing` plugin; `sourcing-module-beams`, `-slabs`, `-windows`)
- Staged guest fails `setContributions` (Aussuchen, `runtime-variant-carve.md`). Descriptor stale: Preview subscribes to rows (`🔍️stock-selection-1125.md`). Status: OPEN until new Demonstrator/Sourcing build.
- Sourcing guest 4096-unit stall (same class as CAD). Status: OPEN, RESOLVED-SRC by lease fix (unverified).
- Sourcing Windows proof: retained Wasm replayed through the native transpiler; external core `b97103cc...4f54` matches (`strict-cdn-acceptance.md`). RESOLVED for the three sourcing modules.
- Standalone Sourcing marked stale (`release-verification-current.md`). OPEN.
- Reproduce: `inspect-stock-selection.cjs` (probe); Aussuchen cases via `accept-site.py`.

### Stdio (`stdio` plugin; now a required actor of GIS and Demonstrator)
- Not previously in the release union; now added. Roster and session publication RESOLVED-SRC (`🔬️release-session-roster.md`, `🔬️vcs-stdio-runtime-custody.md`).
- No Stdio component has compiled or staged. Estimate 1,442 units, 16.76 GiB including reserve (`heavy-pair-schedule-1125.md`). Status: OPEN; the singleton runs after the primary app groups.
- Split into nine linked app libraries plus `semio_hub_stdio_router` (`🔬️stdio-mounted-app-codegen.md`; 1,450 WASI units). Status: RESOLVED-SRC; compile pending.
- Old measured library 8,953 MiB and 9,306 MiB warden record (old unit identity). Status: SUPERSEDED by the split.
- Release policy now LTO off, 16 CGUs, matching Sourcing and GIS (`🔬️native-final-link-bounds-1125.md`). Status: RESOLVED-SRC; final pending.
- Conversion authority for 26 rows, 14 drifted leaves (`🔬️stdio-conversion-authority.md`). RESOLVED.
- TXT artifact compiled at 1,808 MiB (`🔬️txt-production-admission.md`). RESOLVED.
- Presentation member compiled (`🔬️presentation-member-admission.md`). RESOLVED.
- Reproduce: `native-queue.sh stdio`.

### Extension and descriptor infrastructure (cross-app)
- Eight stale plugins and 21 extension confirmations: all 21 extension byte confirmations are recorded; 12 of 33 total (`release-verification-current.md`).
- Descriptor emitter: host compile above ceiling and retained-source drift (`🔬️descriptor-immutable-source.md`, OPEN: production source mirror not wired). Emitter re-use path RESOLVED-SRC (`🔬️descriptor-emitter-queue-reuse.md`, 24 tests).
- Exact compiler digest error `Exact compiler input/artifact digest required` in `completedCargoClosureRosterV1` (`🔬️emitter-digest.md`): witness classification fixed; SUPERSEDED unless it recurs.
- Descriptor pass 4 `actor-input` ceiling: the actor-input wire split (`🔬️actor-input-wire-carve.md`, 146 to 147 lock graphs reconciled) is RESOLVED-SRC; the third carve is pending.

## 4. Superseded or Resolved (do not action)

| Item | Superseded by | Evidence |
| --- | --- | --- |
| Work-limit cap in `with_original_effect_at` (Nov 24 15:50) | Reverted | `🔬️work-limit-return-reverted.md`, `redeploy-log` 2026-11-25 08:40 |
| `🔬️work-limit-root-cause-1125.md` and `🧩️work-limit-fix.patch` | REFUTED | `redeploy-log` 2026-11-25 08:40 |
| 10 of 16 strict boots | 28 of 52 cases, 14 of 26 per locale | `release-verification-current.md` |
| 2.5 GB archive exceeds Pages limit | 826.9 MB candidate | `release-verification-current.md` |
| 11 plugins + 21 extensions roster | 12 + 21 | `cases.json` |
| Demonstrator warden SIGKILL | Warden policy fix (27 tests) | `🔬️demonstrator-oom.md` |
| PDF lift source boundary 4075 | Lifter split (2548) | `🔬️source-boundary.md` |
| BREP Engine 4006, Solid Transaction 4074 | Owner moves, 2,044 closures clean | `🔬️release-boundaries-after-puzzle5d.md` |
| Puzzle 5D editor 4006 | Rewrite owner, 3,459 | `🔬️puzzle5d-rewrite-source-boundary.md` |
| E0277 `dyn ErasedSnapshotRetirement` (BREP) | No longer reproduces | `🔬️brep-jobs-sync.md` |
| E0308 instance-jobs | Missing semicolons added | `🔬️instance-jobs-mismatch.md` |
| Stale shard worker `original-turn.succession-unoffered` | Republished | `redeploy-log` 2026-11-24 23:50 |
| Missing Libertinus font 404 | Renderer support copied | `🔬️renderer-static-support.md` |
| Marketplace install/remove routes on static host | Capability removed from static build | `🔬️marketplace-static-capability.md` |
| Stale Verfolgen / GIS Add Feature stall | Fresh GIS passes diagnostic overlay | `🔬️gis-fresh-descriptor-and-browser.md` |
| Fleet cache outage reader (1.4 s per call) | 2 ms bounded reader | `🔬️fleet-outage-reader.md` |
| `🔬️debug-print-audit.md` (81 hits) | Still open for 14 runtime prints; only browser-worker line:549 reaches the console | open |

## 5. Runbook Commands

All scripts are in the ticket folder `$T`. The `repo tools exec` wrapper is the normal entry. Run from the workspace root.

- `JOBS=2 native-queue.sh [--pairs] <plugin-id>...` (`native-queue.sh`). Per id: `FORCE=1 native-lanes.sh step browser-<id> "" repo ./📜️script.ts components describe <id> --facet browser`, then `python3 release-receipts.py record <modules> <confirmed> <id>`. Pairs run `components describe <left> <right>`.
- `native-final.sh`: `release-receipts.py verify <modules> <confirmed>`, `release-sessions.py`, `native-lanes.sh step browser-support ...`, `native-lanes.sh step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare`.
- `native-site-verify.sh` (`bash "$T/native-site-verify.sh"`): status, then `captured-site.py`, `verify-flow-browser.mjs --site`, `renderer-diagnostic-assets.ts --inventory`, `verify-renderer-http.py`, `accept-site.py`, `package-site.py`. Build step is `MEM="${MEM:-3000}" native-wasm.sh site`, not 4096 as the Nov 24 preflight said.
- `package-site.sh <site-folder> <output-dir> <accepted-manifest>`: `repo tools exec --light python -- -I "$T/package-site.py" "$@"`. No default output directory.
- `accept-site.sh [port]`: `repo tools exec --light python -- -I "$T/accept-site.py" --port "${1:-0}"`.
- `release-receipts.py {record|verify|status} <modules> <evidence> [module] [--site <dir>]`: `status` and `verify` are read-only. `record` writes a confirmation without checking the authored descriptor.
- Node/Bun helpers used by the runbooks: `verify-flow-browser.mjs`, `renderer-diagnostic-assets.ts`, `verify-renderer-http.py`, `check-actor-abi-native-receipt.py`, `test-release-receipts-status.py`, `verify-release-archive.py`.

## 6. Critical Path (Ordered)

1. Fix the descriptor and Wasm custody blockers: Procedural host actor-input ceiling; Stdio singleton compile; FEM composition library; Demonstrator six-surface link; Process worker.
2. Rebuild and re-record all 33 confirmations; run `release-receipts.py verify`, not only `record`.
3. Rebuild the regular Vite output (`native-wasm.sh site`, prerequisites first), then `native-site-verify.sh`.
4. Run the 96-case acceptance on that exact output with one test revision. Clear the Generator, Aussuchen, Statik, Energie and Koordinator failures before the package step.
5. `package-site.sh` to a directory outside the ticket; verify the archive under 1 GB and every member under 100 MB.

## 7. Critical Blockers (Summary)

1. Only 12 of 33 release components have byte confirmations; `release-receipts.py status` cannot reach 33 of 33 until the host and Stdio builds land.
2. Nine Flow extension operators fail with `registry admission requires its original source slot and caller` (`invariantViolated`); fix is in source, binaries must be rebuilt.
3. Generator 0 of 4: `Flow Wasm exports and exact memory are required` (fixed in private build, not yet in final output) and `plugin.completed.decode`.
4. Koordinator and Bearbeiten `WorkLimit` / `authority (work-limit)`; the old root cause is refuted and the open cause is unidentified.
5. Aussuchen Preview "No selection": staged descriptor lacks the rows interaction subscription.
6. Demonstrator final link 4.7 to 4.9 GB against 4,096 MB; the root library hits `no-endpoint`.
7. Procedural host actor-input compile cancelled at 1,638 to 2,179 MiB against 1,536 MiB.
8. FEM composition library 1,871 MiB; FEM and GIS aggregate pair cannot seal.
9. Stdio has no compiled component; estimated 16.76 GiB with reserve.
10. Playbook, Puzzle and Sourcing descriptors stale; Process standalone descriptor stale.
11. Fleet execution and cache endpoints unreachable (`ERR_HTTP2_STREAM_CANCEL`); builds run native-only.
12. Energie `EnergySimulationRunJob` panic is fixed in source; browser `energie run-cancel` still unverified.
13. `[DEBUG]` console prints (`big work quote`, gen-preview) still reach the browser on Aussuchen and Bearbeiten.
14. 96-case acceptance: last result 28 of 52 baseline passes (24 failing), with 44 behaviour cases not yet run on a fresh candidate.

## 8. Not Verified in This Audit

- No builds, tests or Chromium runs were executed. All statuses come from the notes, which were written on the dates shown.
- `native-wasm.sh site` prerequisites (Nov 24 preflight item 3) and the overlay server pid 37114 were not re-checked.
- The 1 GB and 100 MB limits were checked against `package-site.py:13` and the notes; no archive was produced.
