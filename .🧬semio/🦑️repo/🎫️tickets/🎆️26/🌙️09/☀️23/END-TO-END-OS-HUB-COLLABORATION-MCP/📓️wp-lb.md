# WP-LB — Landing Window: Plugin Open Kinds, Capability Descriptions, Oracles

Slice LB, session 13 (2026-09-26 19:0x). Coordinator = main chat. Ports 8110–8119 / 6610–6619 (none used).
Inputs: T12 (`📓️wp-t12.md` §S12-1, §S12-4 item 5, rows 1b/1c/1d/3b/5; `wp-t12/`), D1 (`📓️wp-d1.md`; `wp-d1/`).
Captures: `wp-lb/generated/` (expendable). Binaries: `CARGO_TARGET_DIR=.tmp-ticket/wp-lb/target`.
Landing rows: `📓️landing.md` § Session 13 Landing Window. Guest rebuild requests: `wp-w3/requests/lb.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | T12 `postpublish-open-kinds.py` (stdio `s.stdio.<x>` kinds, editor declarations, gis terrain kind, hub fence, census law, bootstrap, rotation) | **landed** 26 19:08 (79 files) + own fence-law borrow fix; **native green** (every lib + my test targets, `--keep-going`); wasm32 = chain b3; hub `--tests` re-check pending (peer hub errors 05:19, lock stall 06:0x); census law needs W3's fresh descriptors | `check-set1-native-12.txt`, landing row |
| 2a | fem3d oracles | **landed** 26 19:26; Python oracles through the platform: load 60/60, material 19/19, analysis 6/6, boundary 17/17, mesh 74/74, any-mesh 24/24 | `oracle-fem3d-1/2.txt`, `oracle-item2-1.txt` |
| 2b | outline-row-ids | **landed** 27 05:06; platform "feature profile" **6/6**; full platform 116/122, the 6 reds are peers' (norm Wave C contract rows, drawing, protocol drift, native-second registry, stdio fixture census, root script) — none names the new outline-id refusal; bestest 29/30 + epJSON 0/16 as T12 stated (subject-only `model-round-trip`; epJSON parity 28/32: its 4 reds are the newly exercised `energyplus-run-*` rows = LB-F2 energy codec gap); procedural io **6/6** | `platform-*-1.txt`, `rows-*.txt` |
| 2c | example-geometry | **landed** 27 05:06; **8/8** | `oracle-item2-1.txt` |
| 2d | energy reference drift | **landed** 27 05:06; energy-model native green; oracle **1 148/1 149**, the 1 = subject-only `identity-round-trip` (no oracle registration, as T12 stated) | `rows-mutate-energy-model-1.txt` |
| 2e | kit pack twin | **landed** 27 05:06; `🧰️mutate-semio-kit` **46/46** (was 45/46) | `oracle-item2-1.txt` |
| 2f | brep reference carrier | **landed** 27 05:06; Python **40/40**; parity: Rust subject 7 reds = LB-F3 (brep enums' member casing vs schema) → prepared patch `lb-p1-brep-field-casing.py` | `oracle-item2-1.txt`, `parity-lb-1.txt` |
| 2g | txt oracle refusal | **landed** 27 05:06; parity **20/20** | `parity-lb-1.txt` |
| 3 | D1 `d1-frozen.py --apply` + audit + census + `search::long` | **landed** 26 19:25; native green; landed-source projection (11:1x, `wp-lb/d1-project.py` → `s13-lb-d1-projected`): audit **0** findings, **3** description findings = a peer's 3 NEW draw verbs (`editFill`, `editPath`, `editSelection`, draw descriptor 01:52) — every gis/vcs/stdio verb described; Rust = AJV (3 = 3); real audit + `search::long` after the chain describes stdio/gis/vcs | `d1-frozen-apply-1.txt`, `audit-projected-2.txt`, `oracle-audit-projected-2.txt` |
| F1 | LB-F1: 52 stdio editors' tool-job items in the wrong impl | **landed** 26 20:12; native green; now part of the SHIPPED 88-app stdio component | `lb-f1-write-1.txt`, landing row |
| R | Codex stdio rollout's 3 test-only reds (+ deflate test) | **fixed** 06:1x; `--lib --tests` of stdio plugin/json/semio/deflate **EXIT 0** (native lane, 10:01) | `check-stdio-tests-2.txt` |
| 4 | cross-plugin verb-arg census → framework law + plugin fixes | **not started**: needs guest edits (SDK + ~27 plugins) → after W3 announces 7800 on B3 (rule 30) | |

### Log

- 19:05 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-t12.md`, `📓️wp-d1.md`. Load 23, 4 rustc, 113 GiB free, wasm mutex free.
- 19:08 item 1 dry run on the current tree: **79 files / 0 problems** (`open-kinds-dry-1.txt`) → `--write` 19:08:39, 79 files
  (`open-kinds-write-1.txt`); touched crates: 36 stdio artifact crates + gisterrain + plugins stdio/gis + `semio-hub`
  (`open-kinds-crates.txt`). Remaining `"stdio.<x>"` literals outside stdio are format kinds (`*_stdio_kinds`) and
  generated descriptor/catalog JSON (W3's describe/generate rewrites them), as the script documents.
- 19:1x–19:24 waited for rustc ≤ 14 (peaks at load 80: peers' stdio closures recompiling after the edit).
- 19:2x item 2 dry runs on the current tree, all clean: fem3d 7/0, outline-row-ids 9/0, example-geometry 2/0, energy 3/0,
  kit 1/0 (50 019 → 50 018 bytes), brep 4/0 (13/13 vectors, 26 rows apply+invert), txt 1/0 (`item2-dry-1.txt`).
- 19:25 item 3 `d1-frozen.py --apply` (dry run clean first): gis2d +10 descriptions/+8 Chrome, gis3d +2/+1, vcs +5/+1,
  stdio contract fn + 62 `setActiveExample` sites (`d1-frozen-apply-1.txt`). Applied together with item 1 so the stdio/gis
  closure recompiles ONCE for the fleet; one combined native check of the union (44 crates + demonstrator,
  `--features semio-s-plugin-stdio/full-app-catalog,semio-hub/integration-fixtures --lib --tests`), nohup pid 29917,
  capture `check-set1-native-1.txt`.
- 19:26 item 2a fem3d `patch.py --write` (dry run 7/0 again first): 5 references + 2 features. Python oracle through the
  platform: `🏋️mutate-fem3d-1-load` **60/60** (308 s, `oracle-fem3d-1.txt`); the other 5 cases detached (pid 36595,
  `oracle-fem3d-2.txt`).
- 19:33 combined native check run 1 **EXIT 101** (`check-set1-native-1.txt`) — NOT from the landed hunks: with
  `semio-s-plugin-stdio/full-app-catalog` the avi/mp3/mp4/wav editors fail (E0407 `register_tool_job_factories`,
  `build_tool_job`, `bounded_first_step_tool_proofs!` … "not a member of trait `ArtifactOwnedToolJobFactory`",
  E0433 `semio_framework_job`). **Finding LB-F1:** 52 stdio editors outside the shipped component (every editor except
  the 7 text/data ones of `component-app-assembly`) carry the `ArtifactEditor` tool-job methods INSIDE
  `impl ArtifactOwnedToolJobFactory for <X>EditorExampleFactory` (in HEAD since ≤ 09-23, e.g. wav `34c9fca3239`); the
  library-only `full-app-catalog` fleet has not compiled since and nothing builds it (the shipped component and the
  hub's `full-artifact-catalog` never compile those editor modules). Census script output above (52 files).
- 19:35 check run 2 without `full-app-catalog` (shipped features: stdio `plugin-root`, hub default +
  `integration-fixtures`), nohup pid 36493, `check-set1-native-2.txt`.
- 19:5x coordinator: take LB-F1 after items 1–3, before item 4. Codemod `wp-lb/lb-f1-editor-tool-jobs.py` written: moves
  the six misplaced items into `impl ArtifactEditor for <X>` after its `const DOCUMENT_SCHEMA` (the txt layout), keeps
  `Owner/TOOL_IDS/DOCUMENT_SCHEMA/PUBLICATION_CONTRACTS` on the factory, adds `semio-framework-job = { workspace = true }`
  to the 19 crates lacking it; dry run **52 editors, 19 manifests, 20 crates, 0 problems** (not applied yet).
- 20:0x fem3d oracles: material **19/19**, analysis **6/6**, boundary EXIT 1 without a summary line (runner discarded the
  log; re-run with `oracle-keep.sh`). Coordinator rule 22 (swap 24.3/25.6 GB: one cargo/nx at a time) → stopped my oracle
  chain (pids 36595/53916/53921 + children) while the check runs; mesh + any-mesh + boundary re-run after it.
- 20:11 check run 2 (shipped features) **EXIT 101** with 2 errors (`check-set1-native-2.txt`): (a) MINE — T12's fence-law
  hunk `terrain_target.profiles[0].open_targets.push(… package: terrain_target.profiles[0]… )` (E0502, IndexMut +
  shared borrow) → fixed by binding `package` first; (b) a PEER's in-flight hub edit `🏗️bootstrap/🦀️.rs:9897`
  (`admin_observability` returns `Json<serde_json::Value>` but builds `HubObservabilityV1`, file mtime 20:04) — not
  touched. All stdio/gis/vcs/demonstrator crates compiled. **Run 2 predates 20:14 (rule 24: false-fresh fingerprints
  from P8's scratch clone) → superseded, re-run below.**
- 20:12 LB-F1 `--write`: 52 editors + 19 manifests (`lb-f1-write-1.txt`).
- 20:22 run 3 (started 20:12, pre-20:14) killed (my pids 60328/60330/60336); run 4 started 20:22 (pid 66564):
  set-1 crates except the hub with `semio-s-plugin-stdio/full-app-catalog --lib --tests`
  (`check-set1-native-4.txt`), then `semio-hub --lib --profile test` (`check-hub-libtest-2.txt`; the bin test target is
  blocked by the peer's bootstrap edit).
- 20:22–21:02 run 4 stalled 39 min at "Checking wasmtime-wasi" (0 % CPU, no rustc; 28 idle cargos fleet-wide) → killed
  mine, told main. 21:02 run 5 was stopped by the coordinator's cleanup (EXIT 143). Coordinator: default build-dir jammed
  by a Codex peer's cargos + orphans; rule 27 → landing slices build in
  `CARGO_BUILD_BUILD_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/build-landing`. `check-native.sh` exports it now.
- 21:15 run 6 (cold landing build-dir), pid 22456: set-1 + LB-F1 crates `--features semio-s-plugin-stdio/full-app-catalog
  --lib --tests` (`check-set1-native-6.txt`), then hub `--lib --profile test` (`check-hub-libtest-4.txt`).
- 21:15–21:30 run 6 in the cold landing build-dir; the session was cut ~21:30 (usage limit), run 6 died with it (no EXIT).
- 09-27 04:57 resumed (rule 28). Reconciled: every applied set is in the tree exactly once (re-dry-runs report "already
  applied" anchors; B/C/E laws and declarations present once; D1 62 stdio sites; LB-F1 0 remaining); the peer's hub
  `admin_observability` break is fixed by its owner. 05:06 applied item 2b–2g (fresh dry runs 9/2/3/1/4/1 files, 0
  problems): `item2-write-1.txt`.
- 05:09 check run 7 (build-landing, pid 96829): set-1 + LB-F1 + demonstrator + energy-model with
  `semio-s-plugin-stdio/full-app-catalog --lib --tests` (`check-set1-native-7.txt`), then
  `semio-hub --features integration-fixtures --lib --bins --tests` (`check-hub-native-1.txt`).
- 05:09 run 7 refused at resolve: a peer removed `full-app-catalog` overnight — `component-app-assembly` (default via
  `plugin-root`) is now "one complete editor/viewer fleet for all 36 artifact families and 88 subsets", so the 52 LB-F1
  editors are in the SHIPPED component now (LB-F1 is required, not optional). Checks use default features from here.
- 05:14 run 8: peer break `semio-framework-plugin-host` (`AppCommand::TransactionPrepare` missing `prepared_child_ops`),
  fixed by its owner at 05:14. Hub check (`check-hub-native-1.txt`, 05:19): 2 peer breaks in `semio-hub` lib
  (`os_directory::valid_user_preference_record_v1` path; `AuthorityProgressStage::GuestCompiling` not matched) — not
  mine; hub re-check later.
- 05:22 run 9: one error, a peer's new (uncommitted, 01:56) deflate editor test `&unknown.to_value()` without the trait
  in scope → one-token fix `&dsl::ToValue::to_value(&unknown)` (rule 14; peer file noted here). 05:33 run 10 (pid 16637).
- W3 requests written: `wp-w3/requests/lb.txt` (stdio + 36 artifact crates + contract, gis ×3, vcs ×2, demonstrator,
  energy ×2).
- 05:35 run 10: peer mid-edit of the SDK (`AppFactory` gained `codec`; builder updated 05:34). 05:41 run 11: peer's
  in-flight json editor test. 05:42 run 12 with `--keep-going` (06:02, `check-set1-native-12.txt`): **every lib + my test
  targets green**; 3 reds, all peers' in-flight test files: json editor unit test (`oracle(&payload.value)` now a
  `JsonValue`, file 01:34), semio brep editor unit test (`SnapshotEditingCommand` wrapper, peer's editor change), stdio
  `🧪️tests/✏️editor-catalog` (`EditorSurfaceApp` not exported). Landing rows written (`📓️landing.md`, 4 LB rows).
- 06:02 wasm32 check queued (pid 40492; stdio, gis, vcs, energy, demonstrator `--lib`), 5 slices ahead (h12 holding since
  05:52) → per rule 29 I leave the queue at 06:50 and wasm32 = REBUILD fast gate. Hub re-check started (pid 40782,
  `check-hub-native-2.txt`).
- 06:1x coordinator: stdio `--lib` native reported GREEN to main (run 12). Fixed the Codex stdio rollout's 3 TEST-only reds
  at the root, keeping their intent (rule 14): (1) json editor unit test compared `oracle(&payload.value)` over `DslValue`
  while the set-scalar payload carries the snapshot's own `JsonValue` → the test's local `oracle` fn dropped, the emitted
  value goes through the crate's `From<&JsonValue> for serde_json::Value` and is compared with serde_json's independent
  parse; (2) semio brep editor unit test expected a bare `SemioBrepEditCommand` although `Command` is now
  `editing::SnapshotEditingCommand<SemioBrepEditCommand>` → expects `SnapshotEditingCommand::Native(..)`; (3) stdio
  `🧪️tests/✏️editor-catalog` imported `EditorSurfaceApp` from the crate root (it lives in `semio_framework_plugin::app`)
  → `use semio_framework_plugin::app::EditorSurfaceApp;`. Also 05:2x deflate editor test `dsl::ToValue::to_value(&unknown)`.
- 06:03 hub re-check stalled (idle cargo, no rustc, waiting on build-landing locks) → stopped at 06:23 (my pids 40782/40786).
  06:23 stdio test check (`check-stdio-tests-1.txt`) started; the fleet was cut ~06:35 and it was killed at 09:52 (EXIT 143)
  before finishing.
- 09:55 resumed (rule 30: REBUILD START 09:5x, chain b3; native cargo only via the `native` mutex lane in build-fleet-b,
  nice 15). My wasm queue entry is gone (rule 29 → wasm32 = the chain's gate). Landing rows updated. Stdio test re-check
  queued in the native lane (pid 70700, `check-stdio-tests-2.txt`).
- 10:01 `check-stdio-tests-2.txt` **EXIT 0** (native lane, build-fleet-b): the 3 rollout test fixes + deflate compile.
- 10:0x Python oracles (`oracle-item2-1.txt`, per-case logs): fem3d boundary 17/17, mesh 74/74, any-mesh 24/24; energy
  model 1 148/1 149 + bestest 29/30 (each 1 = subject-only round trip with no oracle registration, `rows-*.txt`); epJSON
  0/16 "no raw subject output from rust; run its subject phase" (byte-decoding oracle → parity); procedural io 6/6;
  example-geometry 8/8; kit 46/46; brep 40/40. Test platform: feature profile 6/6; whole suite 116/122 (6 peer reds, none
  from the outline-id refusal: `platform-all-1.txt`).
- 10:09 parity (txt, brep, epJSON) queued in the native lane (pid 85024, 9 waiters ahead, `parity-lb-1.txt`).
- 10:28 parity (native lane, `parity-lb-1.txt`): **`📝️mutate-txt-utf-8` 20/20** (txt refusal set proven);
  `🧊️mutate-semio-brep` 73/80, parity 13/40 — Python 40/40, the 7 reds are the RUST subject refusing the feature's
  (schema-conformant, camelCase) doc strings: "planned mutation payload must decode: curve.missing field `radius_major`"
  (+ `major_radius`, `half_angle`, `control_points`); `🏛️export-epjson-runs-in-energyplus` 28/32, parity 3/16 — the
  4 reds are the Python oracle's `energyplus-run-{600,600ff,900,900ff}` rows (below).
- **Finding LB-F3 (brep, guest code → prepared patch `wp-lb/lb-p1-brep-field-casing.py`, dry run 1 file / 3 hunks /
  0 problems):** `BrepCurve`, `BrepCurve2`, `BrepSurface` are `#[value(tag = "kind", rename_all = "camelCase")]`
  with a doc comment claiming the container `rename_all` also cases struct-variant members; since 09-12 the value
  derive follows serde (`🌱️value/✨️derive/🧪️tests/🐫️variant-field-casing`: members follow `rename_all_fields`), so
  Rust reads/writes `radius_major`/`control_points`/`half_angle` while the brep schema (`📸️snapshot/🔣️.json`), the
  Python reference and the doc strings say `radiusMajor`/`controlPoints`/`halfAngle`. Fix = `rename_all_fields =
  "camelCase"` on the three enums + truthful comments. Lands after W3 announces 7800 on B3 (rule 30), then brep lib
  tests + `test-parity --case 🧊️mutate-semio-brep`.
- **Finding LB-F2 (energy epJSON, owner = energy):** with the outline-row-ids fix the `energyplus-run-*` rows are
  exercised for the first time (before, the index ids `energyplus-run-1..4` matched no registration). The
  semio-written epJSON of 600 and 900 carries NO `ZoneHVAC:IdealLoadsAirSystem` (both the Rust subject and the Python
  oracle then treat the case as free-float; the oracle's `reference["freeFloat"]` is `None` for a conditioned case →
  `TypeError`), and EnergyPlus run directly on 600FF and 900FF yields IDENTICAL free-float temperatures (min 6.1803,
  max 48.2451, mean 29.3708 °C) against the honeybee reference (600FF −12.56/63.92/24.96, 900FF 1.25/44.30/25.13) —
  the export drops the thermostat/ideal loads and the heavyweight/lightweight construction difference. Real codec gap
  (guest code), not a landing regression; reported for the energy owner.
- 10:35 hub `--lib --bins --tests` (native lane, `check-hub-native-3.txt`): lib + lib test GREEN (fence law, census law,
  A4 fixtures); the `os-hub` bin fails on a peer's in-flight bootstrap edit (E0596 `sender` not mut, file 10:33).
- 10:41 hub laws (`test-hub-fence-1.txt`): **8/8** — `local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_opens_every_package_target`
  (incl. the terrain target case), `trusted_profile_generation_binds_zero_target_package_and_every_codec_row`, 6
  `native_openable_provider` laws (the provider consumes the stdio factory closure with the `s.stdio.<x>` kinds).
- 10:51 editor kind laws (`test-editor-kinds-1.txt`): txt/tsv/html `the_editor_declares_the_artifact_kind_it_edits`
  **3/3**.
- 11:04 stdio integration tests: `editor_catalog` (rollout test) does not build — E0432 `semio_framework_plugin::artifact_app_laws`
  (SDK feature `artifact-app-testing` never enabled by stdio's dev-deps) → prepared `wp-lb/lb-p2-stdio-test-feature.py`
  (dry run 1 file / 0 problems; plugin Cargo.toml = no edit during the rebuild). `native_openable_provider` 6/7
  (`test-stdio-integration-2.txt`); the red `native_catalog_dependency_is_exactly_its_compiled_owner` is S17's exact-pin
  change vs the hub fixture `🔗️compiled-dependencies` (`"*"` → "a dependency pins an exact version `=X.Y.Z`") — S17's.

### Item 4 plan (verb-arg law) — blocked by rule 30 until 7800 is on B3

- Measured 11:1x (`wp-lb/undeclared-args-census.py` = T12's heuristic over the committed `🔣️.json`,
  `undeclared-args-census-1.txt`): **519 candidates in 27 plugins** (puzzle 84, space 60, wfc 38, procedural 34, block
  33, lowpoly 31, shooting 26, note 25 …). The heuristic overcounts badly: note shows 25 while its real law found 11 and
  is green since 09-26 (every note verb that needs arguments declares them) — a text match on `args` inside an arm is not
  a read that the verb needs.
- The sound law is behavioural and already half exists: `artifact_app_laws::assert_declared_actions_bridge_to_commands`
  bridges every window-kind verb with its staged declared defaults and panics on failure — a verb whose arm REQUIRES an
  undeclared argument fails exactly there (note's `app.command.invalid-args` law is the same predicate with a code).
  It is called by 28 test files; P8's stronger `assert_declared_verbs_honour_their_declarations` by 3.
- Landing plan (after B3, compile-atomic, one plugin per `cargo check`): (1) SDK: the bridge law returns its offenders
  (`declared_verbs_needing_undeclared_arguments<A>(definition) -> Vec<String>`) and asserts none, covering app-level
  verbs too; (2) every plugin's `🧪️tests/🔬️surface` calls it for each app it registers (the census of callers becomes
  a repo contract rule: an app registered by `PluginBuilder` without a surface call is a finding); (3) each offender
  declares its args (`ActionArgDef::…` with en + de labels) or refuses by name; (4) re-measure.
- 11:1x item 3 measured on the landed source (D1's projection, copied to `wp-lb/d1-project.py`: output
  `.🧬semio/🌐hub/s13-lb-d1-projected`; the stdio `setActiveExample` text is now the source's
  `set_active_example_description()`, which D1's literal-only regex cannot read, so the copy always projects it):
  `semio-os-mcp audit` (this tree's binary, built 10:34) **0 audit findings, 3 description findings** — the draw
  editor's new `editFill`/`editPath`/`editSelection` (peer, no description; guest edit → after B3, draw owner);
  Rust = AJV **3 = 3** (`oracle-audit-projected-2.txt`). First projection run showed 12 (9 = the regex artefact above).
