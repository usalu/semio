# Fleet 1108 Audit: Requests Open Items Reconciliation

Lane `audit-requests` (read-only audit). Source list: `📌️important/open-items.md` (510 distinct items, mapped to `📌️important/requests.md`). Current state checked against the norm tree at `✏️s/🔌️plugins/📕️norm` (grep and rg over `.rs/.ts/.py/.json/.feature/.proto/.graphql`, source-boundary scan `🧰️scripts/source-boundary-scan.py -v`, path-length scan). No source or requests were edited.

Method and limits: `done` means the item is implemented in the tree (cited identifier or file found, or a gate now passes). `stale` means the named code, path or script no longer exists and the concern is superseded. `open` means still needed, or not verifiable from source (build, golden-bless and printed-standard items are open until a green build or a standards check). Classification is grep-based and heuristic; a few `done` and `open` calls are marked unverified in the notes.

## Summary

| Class | Count |
|---|---:|
| done | 142 |
| stale | 7 |
| open | 361 |
| **total** | **510** |

| Lane | Open | Done | Stale | Total |
|---|---:|---:|---:|---:|
| shared | 134 | 53 | 2 | 189 |
| en1990 | 17 | 11 | 1 | 29 |
| en1991 | 12 | 9 | 0 | 21 |
| en1992 | 30 | 4 | 0 | 34 |
| en1993 | 16 | 11 | 0 | 27 |
| en1994 | 20 | 9 | 1 | 30 |
| en1995 | 22 | 6 | 0 | 28 |
| en1996 | 26 | 7 | 0 | 33 |
| en1997 | 21 | 9 | 0 | 30 |
| en1998 | 21 | 8 | 1 | 30 |
| en1999 | 14 | 15 | 0 | 29 |
| din-a | 1 | 0 | 0 | 1 |
| iso-vdi | 1 | 0 | 0 | 1 |
| ts-schema | 17 | 0 | 2 | 19 |
| browser-e2e | 1 | 0 | 0 | 1 |
| host-runtime | 8 | 0 | 0 | 8 |
| **total** | **361** | **142** | **7** | **510** |

Lane mapping: `shared` covers `📇️registry`, `⚖️compliance`, `🎬️structural-scene`, `🧮️mechanics`, `🔮️oracles`, SURFACE/app-surface and cross-artifact build gates. `en1990`..`en1999` cover each artifact under `🗿️artifacts/`. `din-a`, `din18599`, `iso-vdi` are the non-Eurocode apps (only item 504 is cross-app). `ts-schema` covers codegen, schema generate, twins, Cargo.lock/umbrella and typed-enum conversions. `host-runtime` covers shell, keyboard, drag-drop and hub manifests. `browser-e2e` covers accessibility renderer wiring.

Overlong paths: 8 fixture paths are 257 bytes from `/home/deck/semio/` (over the 256 limit): `⚙️change-rail-derailment-assumed-line-load/✅apply/🔣️.mutation.json`, `⚙️change-rail-load-group-assumed-axle-load`, `⚙️change-support-collision-assumed-force-y`, `⚙️change-support-collision-assumed-force-x`, `🧱️change-buckling-length-beam-stiffness-b`, `🧱️change-buckling-length-beam-stiffness-a`, `📍️change-seismic-slope-topographic-factor`, `✏️change-bridge-pier-cracked-inertia-ratio` (all under `🗿️artifacts/…/🧪️tests/🧫️fixtures/🧬️mutations/`). Source-boundary: 3 crates OVER budget (`semio-s-artifact-norm-en1993-mut-fastened-joints` 67 files, `…-mut-buckling-lengths` 34 files, `…-mut-fatigue-stresses` 34 files; limit 32 files). Scene golden files `🎬️scene/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` are 0 bytes in every artifact and in the shared structural-scene.

## Open items per lane

### shared (134 open)

- #1 [8] 🎬️structural-scene/🦀️.rs lacks `mod diagram_kit` mount while 📈️diagram-kit/** uses `crate::diagram_kit`; restore `#[path = "📈️diagram-kit/🦀️.rs"] pub mod diagram_kit;` (build break)
- #6 [16] `ChecksViewState::encode()` absent (0 hits; only decode exists); implement setChecksView payload and `cancelEvaluation`/`openClause` command lane
- #7 [17] 20 artifact files call `render_checks_with`, but no `interaction_domain: Some("norm.…")` ChecksWindowSpec exists (0 hits); wire interaction domain per variant
- #12 [33] StructuralProjection trait exists with entities/fingerprints (🎬️structural-scene/🎯️projection/🦀️.rs:145-152) but only mock implements it; artifact implementations open (see 68)
- #17 [53] field-meta rows for placements[].origin.x|y, footings[].loadCases[].horizontalVariable and layers[].depthTop not confirmed in lookup_field_meta; add rows (unverified)
- #19 [59] editor still commits via setField; no `setFields` (0 hits) and previewRemedy batch commit missing
- #20 [60] visual_layout(variant, device) still builds WindowLayout (📇️registry/…/🪟️manifest-and-media/🦀️.rs:122); outliner-role mismatch not verified as fixed
- #21 [61] Viewport3dWindowConfig still a plain value (…/🪟️visual-windows/🧊️viewport3d/🎚️config/🦀️.rs:103); persisted config crate 🪟️viewport/🎚️config exists but is not wired
- #22 [62] render path still receives only `handle` with InteractionView; focus derivation via setSelectedCheckIndex remains (unverified fix)
- #28 [106] view.activeCombination const exists (🗿️artifacts/⚖️en1990/…/🕹️handles/🦀️.rs:91) but routing of view.* writes to view state in viewport interaction not verified
- #29 [107] CheckResult still has no structured combination link; scene parses `leading=` (combinations::column_for_check, 4 hits); add combination field to CheckResult
- #37 [196] CheckResult.stations exists (Station type in 🎬️structural-scene/🧮️tessellation and en1993 frame-eigen) but `utilization == max(stations)` contract and per-station binding not verified
- #38 [205] binding_of_check exists but UtilisationBinding.per_station absent (0 hits); add per_station to UtilisationBinding
- #44 [292] LiveView still calls synchronous live::report (74 hits); switch to responsive lane (responsive exists in compliance macro 🛰️live/🦀️.rs:52)
- #46 [306] member_modes::modal absent in 🧮️mechanics (only buckling/deflection); en1995 scene still needs modal helper
- #49 [344] NoTransient reducer still used by vdi3805 and din18599 viewers/editors (30 hits); viewer ephemeral lane missing
- #50 [347] lane-clock exists (⏱️lane-clock) but 🕹️direct-manipulation/➕insert-tool present, LiveView::with_lanes not verified; compile status unknown
- #53 [387] en1993.mechanics.portal-sway case not located as test row; mechanics kernel owner to confirm (unverified)
- #57 [409] binary mutation codec tag_u8 still used (89 hits); no varint codec found; 366 en1993 leaf kinds blocker unverified
- #63 [429] live_scene! macro used by only 3 scenes (en1991, en1994, en1996); roll out to all ten artifacts
- #68 [454] only 🎬️structural-scene mock-projection implements StructuralProjection; no artifact implements it
- #71 [464] 8 mutation fixture paths are 257 bytes from /home/deck/semio (limit 256); shorten fixture folder names (see report section 6)
- #73 [468] artifact scenes still monolithic; project_entity appears 4 times, Out::wants absent (1 def only); per-entity projection not done
- #76 [486] per-entity projection (Out::wants(id) checks) not implemented (1 definition only)
- #79 [493] third-party*extended.json oracle cases not confirmed; feed inputs to evaluator (unverified)
- #81 [497] scene_of still used at 648 sites; viewport call sites not switched to live_scene!
- #87 [528] catalogue evaluators still return English `missing input {key}` reasons (≥3 evaluator files); localise reason codes
- #88 [529] "The site already has this value." still English in en1998 diff files; MutationOutcome message needs fault-notice keying
- #100 [625] adapters for mechanics.cracked-continuous / PyNite split-member cross-checks absent (0 hits)
- #103 [638] 36 new f64 fields across model/twins/SQLite/fixtures not verified; treat as open pending generator run
- #112 [697] QuantityKind has no Velocity kind (site response reports period only); add Velocity kind through VdiQuantityKind mapping
- #113 [715] no host turns a drag on an entity without a row into PlacementCommit::of + insert batch (PlacementCommit::of exists, 35 hits)
- #115 [740] many literal EN 1991/1993/1996 tables not in NDP registry (cpe/Cf, stainless ladder, K/f_vk0 etc.)
- #117 [770] scene golden 🎬️structural-scene/…/🏛️scene/🔣️.json is 0 bytes; StructuralScene diagrams not wired (scene-law unbound)
- #120 [789] structuralcodes adapter ignores t0_adj/alpha_cement (Eq. B.9) for en1992.3.1.creep-shrinkage; fix adapter and re-bless
- #122 [797] SceneKit scalar_owner default method not found (0 hits); add to SceneKit trait
- #124 [831] sectionproperties adapter get_beta_p not exposed (1 hit only); expose Wagner coefficients and bless free-outline-tee-wagner
- #125 [835] no Rust subject adapter for E1-xx catalogue rows (girder_cross, girder_as_min, end_plate, crack_band_k6) found (0 hits)
- #128 [862] scalar_text still formats with Locale::En (📇️registry/…/🧾️properties/🦀️.rs:121 pattern); DE number separators not localised
- #129 [863] English-only Err/ok_or diagnostics in edit-rules/command-args/value-path not localised (paths moved; unverified)
- #130 [922] SEMIO_REGEN_EN1997_ASSETS / SEMIO_UPDATE_GOLDEN re-bless still pending: scene golden 0 bytes, pack assets stale
- #136 [1006] UnitSpec N/m not added to UNITS (N/m only used as literal in en1996 diaphragm scene)
- #137 [1010] one_leaf_edit_rebuilds test and kit::dependencies_of coverage not verified (incremental-evaluation)
- #139 [1018] Rows::row extra dependency on referenced member is coarse; refine incremental dependency declaration
- #140 [1019] unit-converted remedies still applicable=false; add unit-aware remedy applicability
- #141 [1025] Emitted::with_case_catalogue absent (0 hits); add API to feed demo rows
- #146 [1074] older compliance oracle ignores weldRuns[].hazExtent; port hazExtent to ⚖️compliance/🐍️.py or retire oracle
- #149 [1087] EN annex branch S/TB/TC/TD and gamma_I issues in old compliance oracle (⚖️compliance/🐍️.py) unverified; check against current annex code
- #150 [1093] a14.deflection* .sampled(10,…) utilisation semantics unverified; confirm sampled max replaces utilisation correctly
- #151 [1103] FieldBinding is numeric only; text leaves (panels[].layup) cannot be dragged; add string-capable write for layer thickness grips
- #152 [1104] bottom-face notch (cutouts[].face = bottom) input absent in case engine; add face input
- #157 [1131] mechanics::section::torsion_constants, pub evaluate_case_file and scene_law_rows absent (0 hits); oracle-harness adapters for en1994/en1999 open
- #158 [1137] Rows::global/Rows::part still read `*` (global_reading absent); T0 verdict units re-run on every drag
- #160 [1172] checks-special-frames mounts thin-walled checks (file exists); its test not verified green; crate status unknown
- #164 [1176] Cargo.lock of en1995 lacks new mut-* packages; run lock sync
- #165 [1189] HandleKind has no 2D translate; fasteners need two ScalarAlong grips; add planar handle kind
- #178 [1303] spec_trace::Emitted::with_report absent (0 hits); add next to with_checks (35 hits)
- #187 [1404] oracle harness has no adapter for geotechnics design cases (🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json present)
- #193 [1443] with_scene_targets snapping: live_handles present (38 hits) but grips snapping to member ends not confirmed
- #195 [1479] RemedyBound::Insert absent (0 hits); add insert remedy for stiffeners in compliance Remedy
- #197 [1489] drag-to-reorder: move_item_edit exists (10 hits) but no moveItem tool commit path (case-edits ui error); outliner UI commit
- #201 [1507] SpecTrace::new takes one EN-only &str; add LocalizedCopy description (LocalizedCopy in 1050 hits, SpecTrace still EN-only)
- #202 [1508] EN-only check texts in en1990 catalogue (line 364, 742-743) and en1993 built-up checks (line 76)
- #203 [1509] steps("…") takes EN-only String (9+ calls, e.g. snow/🦀️.rs:68); add bilingual steps like steps_de
- #204 [1510] ClauseRef "Annex X" labels are EN-only; add bilingual section label or Annex enum
- #206 [1513] family and part labels are EN-only in clause column; use stable id or bilingual label
- #207 [1514] mesh, thin-walled and other EN-only error strings (🕸️mesh/🦀️.rs:93, 🪶️thin-walled/🦀️.rs:56) not localised
- #211 [1547] ndp_awaits_value lives in en1990 checks (7 hits); move next to ndp_number_opt in compliance annex module (absent)
- #213 [1558] oracle harness tolerance checks at 🔮️oracles/🏃️harness tests (path absent); tighten tolerances or justify
- #214 [1559] app-surface edit-rules match RemedyBound with `_` arm hiding AtLeast/AtMost/Exactly (RemedyBound 183 hits); add explicit arms
- #217 [1600] reliabilityClass project setting: ReliabilityClass present in en1990 snapshot only; add to en1992/1994/1995/1996 documents and K_FI lookup
- #218 [1621] geotechnics fixtures: en1997.G.pile-dynamic-tests and downdrag cases present in fixture file but adapter not wired
- #223 [68] regenerate generated indexes build-owners.json and dependencies.json for new crates (verify build-owners sync)
- #225 [133] project target @semio-s/plugin-norm-structural-scene:test unverified; run after build owner confirms project name
- #226 [135] structural-scene projection-kit mounted (root line 12) but rebuild and contract build not reported
- #227 [194] compliance and structural-scene rebuild/test pending (Station and CheckResult.stations exist)
- #229 [214] compliance and app-surface compile/test not reported green
- #231 [220] structural-scene unit tests (cages_points_and_weld_runs…) compile/test not reported
- #232 [248] SURFACE checks/report-tree/catalogue tests not reported green via norm-artifact-contract-rs
- #233 [255] SURFACE direct-manipulation code (🕹️direct-manipulation exists) check/test pending
- #234 [256] compliance incremental-evaluation WP-05 compile/test pending
- #235 [269] structural-scene parameter_tracks test and ParameterTrack types: build/test pending
- #236 [291] compliance rows (🧱️rows) and Scope kinds compile/test pending
- #237 [305] rebuild norm-en1992/en1995/en1996-rs for scene fidelity; new EN 1995 scene dependency edge
- #238 [343] ViewLane in NormTransient present (79 hits); twin check and compile pending
- #239 [371] CheckStatus::Info and CheckBuilder::informative absent (0 hits); add kernel change and rebuild dependents
- #243 [388] groundhog imports plotly/pyproj/jinja2 at module level; harness inert-import finder must be reviewed
- #247 [455] contract-rs build+test (view lane units, unit picker, ViewUnits) and norm-enXXXX-rs rebuilds pending
- #248 [458] Cargo.lock sync for en1991/en1996 scene dependency on mechanics; dependencies.json regenerate
- #249 [461] bless committed pack assets after first green compile (4 pack files and glulam-footbridge assets)
- #250 [470] contract-rs scene memo and tests rebuild pending
- #260 [641] contract-rs scene memo and tests rebuild pending
- #261 [657] viewport config crate not reported green; unit picker persistence blocked
- #264 [698] scene registry goldens need SEMIO_UPDATE_GOLDEN once after soilLayer kind added (golden 0 bytes)
- #267 [704] scene golden re-bless after first green build (section-geometry, zone-section diagrams); golden 0 bytes
- #272 [788] first green build of nine live_law test modules and p95 report pending
- #274 [808] codec impls in crates other than the defining crate (orphan rule); io/📝️text/📸️snapshot impl placement unresolved
- #277 [842] Cargo.lock entries for new mutation crates and clauses-annex; verify via lock tooling
- #278 [843] scene golden re-bless for ROW_DECKS and ring/web/deck entities (golden 0 bytes)
- #280 [857] scene golden re-bless for fatigue, strut-and-tie and prestress-loss entities (golden 0 bytes)
- #281 [866] LocalizedCopy::new("", "") placeholders in structural-scene projection and scene-builder not verified overwritten
- #284 [895] Cargo.lock synchronise for en1999 mut-collection-rows, hub, bridge and fixture-sweep locks; verify build-owners sync
- #286 [938] 19 crates have no umbrella script.ts packages entry; add umbrella or entry
- #289 [992] en1992 build timed out in nx daemon journal replay (infra, no rustc output)
- #293 [1031] verify build-owners sync for en1993-mut-plated-stiffener-spacing crate (present, 91 lines)
- #296 [1111] Cargo.lock sync: en1999 lacks 7 mut-* packages; en1995 umbrella dev-dependency edges
- #297 [1115] en1993 build stalled in nx journal replay (infra); no diagnostics
- #298 [1129] viewport-config crate not reported green; unit picker persistence still blocked (duplicate of 261)
- #301 [1245] contract-rs (view lane check, view-steps, check-parameters) and norm-enXXXX-rs rebuild pending
- #303 [1305] regenerate scene goldens, bar and lintel bindings, detailing rows after first green build
- #305 [1391] bless scene golden (fire:wall, fire:window, fire:opening, dynamic-cscd-frequency) after first green build
- #310 [1517] io/🦀️.rs ends with pub use mutation_bridge::* after module moved to mutations crate; resolve import (E0432)
- #315 [14] register unverified EN 1995 NDP rows from en1995-completion in compliance annex-and-tables / ndp registry (printed standards needed)
- #316 [18] NDP provenance glyph and description segment in CheckResult/UtilisationBand (UnverifiedBasis exists, 105 hits; glyph unverified)
- #317 [27] register unverified EN 1996 NDP rows (UNVERIFIED_NDP) in ndp registry
- #318 [34] adopt EN 1990 NDP rows from ndp-tables into compliance registry (printed standard values)
- #324 [175] adopt EN 1992 NDPS table rows into compliance registry (ec_verified/de_verified fields)
- #328 [290] NDP public verification second pass: no registry statuses flipped; code still contradicts verified-public values
- #330 [341] 30 NDP ids cited by wired checks (ndp.en1993.v-250.1 …) not in NDP_TABLE; add and verify
- #346 [669] 46 prose feature files parse clean; discovered test dirs (check-stations, ndp-adoption) need step coverage
- #357 [1119] every artifact owner: add annex-dependent normative numbers to registry JSON and read via ndp_number(id, annex)
- #359 [1200] NDP conflicts with blue-prints library (EN 338 tables); human decision against printed standard
- #360 [1235] registry hygiene: 36 ndp.en1997.xi.* and sls.* rows carry code EN 1998; correct code field
- #361 [1236] unit test asserts unverified_ndp_entries().count() == NDP_TABLE.len(); false once rows verified; fix test
- #363 [1270] 119 max/min calls on NDP values without clause citation (report-only check 19); add citations
- #364 [1271] 38 explanation strings say unverified/ungeprüft without an unverified registry row (report-only check 20a)
- #365 [1272] 62 artifact files use unverified rows without UnverifiedBasis path (heuristic check 20b); add outcome path
- #371 [1460] local gamma constants (gG, gQ, xi) in several artifacts instead of ndp_number registry rows (cross-artifact)
- #393 [1518] diff_text!/diff_binary! expansions impl foreign traits in io crates (orphan rule); move impls into defining crate
- #394 [1519] diff unit test calls mutation.diff(base) and diff.inverse() not provided by imported traits; fix imports or API
- #404 [897] set rules select rows by position (en1991 23, en1994 92, en1996 87, en1998 161); convert to id selectors
- #405 [923] E0080 unit symbol in en1991 snapshot missing from framework UNITS (dsl/token); add UnitSpec
- #423 [1336] per-field classification of 121 check-21 enum/entity candidates in report; owners convert with template
- #505 [564] editor/viewer layout() hard-codes desktop 22/50/28 tree in all artifacts; replace with visual_layout_with per device

### en1990 (17 open)

- #13 [35] editor does not surface `checks::catalogue::{registry,evaluate_case}` (0 catalogue hits); add E90.01-E90.30 verification-case panel
- #210 [1546] KitDoc.option packs traffic group into one string (view_option/split_option); add typed group option
- #258 [560] include en1990 in next build run and bless scene golden (en1990 golden 0 bytes)
- #275 [821] first rustc log for en1990 never produced; closure-capture errors in rail-and-frame-sls expected; run build
- #295 [1109] bridge-sls-rail generator rewrote mounts; restore other mounts and derive mounts from leaf dirs
- #332 [354] EN 1990 S1: gamma_G sup/inf chosen by action kind not effect sign; review-en1990-en1991.md audit absent
- #336 [459] PSI_TABLE only in en1990 ndp-tables; en1991 psi_1_2 separate; move psi table into compliance registry
- #376 [104] persisted editable combination groups and action group field, TestSeries/ReliabilityAnalysis wiring for Annex D; generator supports exclusive groups, snapshot does not
- #378 [108] field-meta rows for handle/marker leaves (members span/precamber, permanents gk, variables qk) not verified in field-meta
- #379 [287] e90-26.1 EQU γG sup/inf 1.05/0.95 vs EN 1.10/0.90; confirm against printed EN 1990 and correct
- #384 [630] 6 of 21 blessed oracle cases lack evaluator entry (mechanics.beam-envelope M_sag/M_hog/R_max/R_min); add continuous envelope API
- #386 [792] unwired: en1993_cut_off_ranges (6 hits), exact_failure_probability (4 hits), en1991 tandem_reaction_n, rail_lm71_mid_moment_nm, drift_mu_w; wire into checks
- #387 [867] check-results LocalizedCopy::new("", "") at ⚖️compliance/✅️check-results/🦀️.rs:291 must be overwritten before render; verify
- #388 [893] E0063 factor_overrides in En1990Snapshot initializer (io sqlite snapshot read side); verify build fixed
- #389 [1037] with_case_browser absent (0 hits) while render_inspector_with_cases used (30 render_body hits); wire case browser for add/duplicate/remove
- #392 [1488] scene side of traffic group: ViewLane.group read and combination handle absent (0 hits)
- #395 [1573] dev-dependency cycle in inferences package (io, examples, schema depend on inferences); remove cycle

### en1991 (12 open)

- #32 [130] roofs[] handles (pitchDeg, parapetHeight, driftObstructionHeight) lack EDIT_RULES set-field rows (unverified owner; roofs[] appears in en1991/en1996 scenes)
- #127 [849] E13.13 spec entry exists in en1991 spec-trace (🧭️spec-trace/🦀️.rs:42); anchor of cases collection after thermalSections still to verify
- #199 [1495] scalar-owner classification for altitude, tMax, tMin of en1991.1-5.isotherm check; golden/law bless pending
- #290 [1009] EN 1991 scene golden re-bless for trafficTravel layer and overlay (golden 0 bytes)
- #322 [78] EN 1991 TypeScript twins under editor/viewer (pretty-printed JSON inputs, seven-column report) stale; regenerate
- #356 [1097] oracle gaps: wind faces and snow rows only in older compliance oracle (snapshot_checks); add differential tests
- #396 [266] face geometry (hipped roofs, zone polygons, windFaces[].polygon, LM1 tandem positions) absent (0 hits); add mechanics-ready member model
- #397 [286] coast WZ4 q_p 1.55 vs spec 1.50, islands WZ4 values and wind zone data (v_b0, q_b) need printed EN 1991-1-4 check
- #398 [491] lm1_tandem_moment_nm models one axle; LM1 tandem (1265.6 kNm, R_max 562.5 kN) not implemented
- #399 [631] 8 blessed oracle cases lack evaluator (mechanics.lm1-tandem R_max, rail-lm71-mid-moment); add part_2 tandem_reaction_n
- #403 [880] handles for windBridges, dynamicElements, internalExplosions, thermalSections not confirmed in handles (windBridges 25 hits)
- #507 [1095] part_1_6 construction_qk values (machinery, concreting 3.0 kN/m²) not in EN 1991-1-6 Table 4.1; correct against printed standard

### en1992 (30 open)

- #36 [176] editor/viewer do not surface `checks::registry()`/`evaluate_case` for VZ-03/VZ-14 section_resistance (0 catalogue hits)
- #41 [231] tendon P(x) ColourChannel::Fields with FieldSamples::Station exists (101/15 hits) but stirrup link zones from stirrups.zones[] not wired in en1992 visual
- #94 [570] no `registry`/`evaluate_case` catalogue in en1992 (0 hits); add German-reasons test once a catalogue exists
- #142 [1026] oracle rows 7.2.sigma-s/sigma-c and 5.10.prestress rely on current bar conventions; 5.10.decompression oracle missing
- #153 [1118] FastenerKind::Rebar has no bend_start (0 hits); EN 1992 two-ended hooks not modelled
- #159 [1161] spec ids BR-09, LQ-08, DI-01 not in spec-trace/📐️specs; add as document-level checks
- #181 [1313] bar-cover row en1992.4.4.bar-cover realises D-03 but no oracle row exists
- #194 [1467] per-stud positions (rail id collection) not persisted; schema/twins/kinds/fixtures missing
- #205 [1511] snapshot titles EN-only ("Failing/Compliant prestressed beam" en1992 snapshot 🦀️.rs:344, "Weak RC frame" en1998 🦀️.rs:197); make bilingual
- #221 [1631] MoveItemRule absent (0 hits); only members list has reorder rules; add rules and with_reorderable for en1992 outliner
- #270 [757] sqlite snapshot still builds RcMember/Anchor with placement_x|y|z|yaw (🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:110,112); finish placement conversion
- #279 [856] cargo:update failed: semio-s-plugin-norm-mechanics missing from en1992 workspace.dependencies; add dependency
- #329 [302] effect-profile and member-evaluation additions (📉️effect-profile) not found; implement profile module for member checks
- #340 [578] ndp.en1992.kk-ageing-chi default 0.8 vs Volterra 0.82-0.86; informational, decide value
- #350 [702] fire and serviceability constants (ndp.en1992.psi-fire-lead …) still local constants; register as unverified rows
- #375 [1619] materials_and_time::conc uses private tuple array; read ndp.en1992.t3-1.ecm.* rows
- #406 [45] add setField EDIT_RULES and mutations for prestress.eccentricity, punching.columnWidth/Depth, anchors.c1/d
- #407 [46] snapshot fields to own: Placement per member/anchor, section outline polygon, explicit bar positions (not in schema)
- #408 [47] CheckUnits by entity and per-station utilisation (flexure, shear, anchorage, punching); checks expose stations
- #409 [49] no creep/shrinkage, tendon loss or strut-and-tie data in snapshot; VZ-02/VZ-09/VZ-13 curves not drawn
- #410 [177] bars, section polygons, tendon control points, stud rings, anchor groups absent from schema
- #411 [283] shear DE values (k1 0.12, C_Rd,c 0.15/γc, v_min κ1 0.0525/0.0375) need printed DIN EN 1992 NA check
- #412 [327] helpers asw unit conversion (* 1e3 vs * 1e-3) and V_Rd,s min with V_Rd,max; fix member shear
- #413 [382] Python mirror flexural_resistance_nm 561.5 kNm vs Rust; S1 verify flexure with independent calc
- #414 [384] conc() rounded Table 3.1 values vs structuralcodes formulas (info; keep or evaluate formulas)
- #416 [649] 18 blessed oracle cases lack evaluator entry (creep strain, A_sw,max, nu_1, tau_t, punching beta); add entries
- #417 [756] change-reinforcement-ductility fixture still carries newDuctility "b-alt"; update to DuctilityClass enum
- #418 [860] serviceability check steps are English-only (lines 35-333); add DE steps
- #420 [1064] members[].tightness optional set/clear kinds absent; nested_patch needs optional clause
- #424 [1537] InsertPlacement/RemovePlacement inverse use index/placement field names; align naming convention

### en1993 (16 open)

- #48 [342] `:check`/`:test` of norm-en1993-rs never confirmed green; first-compile fixes pending (no build evidence in tree)
- #58 [410] template test `every_special_part_leaf_resolves_to_its_whole_entity_update` not found (0 hits)
- #143 [1030] second_order in frame-eigen uses non-public kernel items (System::assemble and Element::{k,geometric} absent); refactor to public kernel API
- #144 [1032] member-checks lambda_y/z sqrt(A_eff/A) for class 4 and ΔM = N e_N in mn_interaction_eta not verified (correctness)
- #240 [374] source boundary: en1993-mut-fastened-joints (67 files), mut-buckling-lengths (34), mut-fatigue-stresses (34) are OVER budget; split crates
- #263 [693] en1993 scene golden (0 bytes) and DSL/pack regeneration pending
- #273 [803] en1993 scene golden re-bless for tf-buckling, fire-ltb, block-tearing diagrams pending (golden 0 bytes)
- #343 [616] ids ndp.en1993.v-026.2 and v-420.2 are empty placeholders (symbol ?, no value); key with values
- #348 [687] materials[].rambergOsgoodN (3 hits) lacks field-meta row and per-leaf kind verification; add EN/DE label
- #352 [815] annex-block-checks cite ndp.en1993.v-258.1, v-376.1, v-806.1, v-881.* not in registry; register
- #374 [1601] gamma_mf(method) literals 1.0/1.15/1.35 instead of registry rows (only ndp.en1993.gamma-mf)
- #426 [651] 18 of 47 blessed oracle cases have gaps (section properties, mcr-i, a_w hollow, tau_Ed I)
- #427 [670] frame-eigen feature (5 scenarios) has no python/rust step adapter; add clause-vector adapters
- #428 [859] field-meta row for materials[].rambergOsgoodN missing (snapshot leaf ramberg_osgood_n); add EN/DE row
- #429 [861] joint-components governing text English-only (checks/🧮️clauses/🔗️joint-components/🦀️.rs:399); make bilingual
- #431 [1352] verify_fixtures.py en1993 has 22 failures (nested insert/remove-action kinds addressed by id+index); fix addressing

### en1994 (20 open)

- #101 [626] facts.layout (catalogue::continuous::Layout) not consumed by beam scene (facts.layout 1 hit); supports/hogging from layout unverified
- #135 [991] En1994{Beams,Columns,Slabs}ActionsRows carry only modified (no removed/inserted); nested action insert/remove not implemented
- #170 [1242] point-load xM honoured only for simply supported beams (action_internals 47 hits); continuous spans still mid-span idealisation
- #182 [1314] studRings[] ring parameters only; per-stud collection or phase angle not added (0 studRings hits in fixtures)
- #209 [1527] columns[].reinforcementProduct (choice hot_rolled|cold_worked) not added: field-adder limitation (choice leaves unsupported)
- #215 [1560] edit-rules `_ => resolve_with_id_removals` swallows MoveItem (MoveItem 31 hits); add explicit MoveItem arm
- #259 [624] Cargo.lock synchronize for en1994 (63+6 kinds, 9 group crates) and first compile
- #311 [1522] edit-rules resolve_edit regenerated with two `_` arms; remove shadowing arm and keep nested removals
- #370 [1452] beam fire module reuses NDP rows; unexposed-surface exchange 9 W/m²K recalled from memory, verify
- #433 [67] scene calls checks::{part_1_1,part_column,…} helpers; confirm API stability and tests for effective width/plastic moment changes
- #434 [90] facts scene effective_width_m b0 semantics (distance between outer connector rows) to verify in visual
- #435 [91] EDIT_RULES leaf coverage for en1994 (only 19 leaves per audit); add missing leaf mutations per en1994-completion.md
- #436 [363] two-span coefficients in checks/🦀️.rs:131,139 (UDL 9qL²/128, point load 5PL/32, V 11P/16) and pattern loading to verify and fix
- #439 [672] 3 blessed ltb-restrained cases lack evaluator (quantity Iw); add catalogue::ISection::iw()
- #440 [890] part_1_1::section_class uses outstand (b - t_w)/2 without root fillet r (Table 5.2)
- #441 [907] warping constants from analyse_section(profiles::i_section) not wired; iw() absent (0 hits)
- #443 [1046] script.ts calls runArtifactRustPackageMain twice (stale subset at line 4); remove duplicate call
- #444 [1136] nested action position leaf actions[].xM and change-{beam,slab}-action-position-m kinds not confirmed (6 hits)
- #445 [1521] testkit demo_mutation_cases lacks ChangePlacementX/Y/Z/Yaw, InsertPlacement, RemovePlacement cases
- #446 [1599] part_en1990 in 🧬️schema/💡️inferences/⚖️checks reported structurally broken (accumulate_column followed by beam match); blocker to verify

### en1995 (22 open)

- #5 [15] editor/viewer do not surface `checks::timber::registry()`/`evaluate_case` (0 catalogue hits in 🪵️en1995 ✏️editor and 👁️viewer); add verification-case panel
- #54 [406] CatalogueCase/labelEn/labelDe entity: CatalogueCase present in en1995 snapshot per request 450; cases entity rollout unverified
- #55 [407] timber document-case shape (`cases`, num/text maps, T-nn) not adopted as checks::timber::Case mapping (0 hits for checks::timber::Case in norm)
- #147 [1078] misleading KN names (nKN, mKNm, vKN, nTKN, fC90KN) still present (976 hits); rename to N/Nm conventions
- #148 [1079] T-97 rolling shear and T-120/T-121 document rows absent; CLT gamma method needs independent derivation
- #166 [1190] GrainLayer grain direction has no hatch cue; add directional hatch
- #168 [1192] T-94/T-95/T-96 typed entities not built; schema.py and kinds.py generator scripts absent (MISSING)
- #175 [1292] E0277 risk in mutation unit tests (ChangeAnnex, ChangeConnectionActionFK ... 500+ hits) not verified fixed
- #188 [1408] enum families still open: members/connections actions kind, category, loadDuration, fastenerType (0 hits for kind-enum fields)
- #292 [1015] en1995 rebuild pending for 5 new snapshot collections (tapers, cutouts, fasteners, panels, floors)
- #314 [1587] en1995 scene golden (case-kser) changed by form-drawn case; bless after rebuild (golden 0 bytes)
- #325 [284] EN 338 C16/C24/C30 values (f_t,0, f_c,90, G) need printed-standard check against registry
- #333 [358] ContinuousTwoSpan coefficients (q l^2/14, F l/6, 0.6 V) wrong vs exact support values; fix load-combinations
- #337 [498] nine EN 338 rows with differences (C20 rho_mean, C27/C35/C40/C45/C50 values); resolve against printed EN 338
- #347 [673] three blessed en1995 strength-class cases (t14/t22/t30) lack evaluator entry (e_t_0_mean/e_t_0_k lookup)
- #355 [1016] register unverified creep time-constant, floor f1-limit rows (from memory; needs printed NA)
- #367 [1284] 117 k_mod/k_def rows marked verified without source; downgrade to unverified or add source
- #373 [1552] bracing drawing keyed on case count; n_braced not used and "0.35 N/mm²" hand-written in diagrams; read from ndp
- #447 [95] fire module charring_rates and cladding_times not re-exported from ⚖️timber (only d_ef_mm); add pub use
- #449 [303] effect-profile module (Profile, deflection_at, row_shares) and Assessed::{along, fielded, timed} absent in ⚖️timber; implement
- #450 [405] CatalogueCase/CaseNumber/CaseText in snapshot; evaluation rows (En1995Units), evaluate_document and insert-catalogue-case kinds not wired
- #451 [865] DE title "Nicht nachweisbarer Mehrfachversagen-Holzbau" awkward; reword German title in 📚️examples/❌️multi-fail-timber/🟦️.ts

### en1996 (26 open)

- #9 [28] editor/viewer do not surface `checks::masonry::registry()`/`evaluate_case` or MnSection curve (0 catalogue hits); add verification panel
- #91 [549] scene handle writes (isothermDepthM, concentrated[].positionM, distanceA1M, planXM) need edit rules and field-meta rows confirmed (unverified)
- #102 [634] add/remove placement named insert-/remove-placement present; change-placement kinds present; en1996 lanes exist: verify en1997 lanes and grips (unverified)
- #132 [967] field-adder has no en1996 profile; M-95 bracing/diaphragm snapshot fields absent
- #133 [971] scene Spec.path hard-wired to walls[id=..]; chase/joint entities need cases[id=..] owner path
- #154 [1122] inPlaneShearN/inPlaneMomentNm never enter shear() (469/462 hits, only design_effects used); wire into V_Ed
- #155 [1123] N_Rd floored at 1e-9 N gives utilisation 6.6e14 for Phi=0 (change-support-sides fixture); return capped status and sane utilisation
- #179 [1304] diaphragm solved by 200-element shear-beam tridiagonal in catalogue crate; no mechanics dependency (🥞️diaphragm)
- #244 [401] en1996 build run 6 died at NX cache capacity; rerun build (gate unverified)
- #257 [547] en1996 scene golden and EN 1996 binary pack regeneration pending (golden 0 bytes)
- #287 [970] scene golden re-bless for annex-e wall entities, chase and joint entities (golden 0 bytes)
- #334 [359] f_vk0 material-independent table (normative-tables 204-225) wrong for CS/concrete/AAC; fix per printed table
- #339 [548] add ndp.en1996 m70.bed-joint-bar row (bed joint thickness ≥ bar diameter + 5 mm)
- #351 [813] review NDP tagging of wall evaluators (load-case-checks, fire-durability) for clause assignment
- #354 [968] register ndp.en1996.annex-e.<class> rows (36 classes, value null) and m70.bed-joint
- #368 [1288] nested masonry wall row kinds (length, thickness, direction, storey) needed for masonry.simple 9.7.2
- #369 [1416] Table 9.3 minimum wall area not available; masonry.simple still applies five unverified ratio rows
- #454 [122] WallOpening has no horizontal offset and ConcentratedLoad no position along wall (offsetXM absent); scene spreads evenly
- #455 [123] cavity walls with ties, flange walls, bracing layout not snapshot entities; add model rows
- #456 [124] export public masonry::wall_profile(annex, class, situation, wall, lc) for scene and checks
- #457 [125] fire_state (μ_fi) and shear/flexure structs private (fire_state 0 pub hits); export for diagrams
- #458 [229] reinforcement bars at real positions via Pattern::Points (16 hits) partly used; offsetXM for lintels pending
- #459 [285] EN Table 3.3 K values (clay GP, thin layer, calcium silicate) inconsistent with F96 rows; fix per printed standard
- #460 [304] load-case-checks Compression sections/capacity and wall-phi stations (0, 0.5, 1 of height) to finish
- #462 [674] 3 blessed phi-m cases (lambda, A_1, u) lack evaluator; add phi_m_terms
- #464 [864] String::from("Invalid tie index.") English-only fatal text in change-tie-* diffs; make localised

### en1997 (21 open)

- #24 [70] editor/viewer do not surface `checks::catalogue::registry()`/`evaluate_case` (0 hits); add verification-case panel
- #25 [71] nested-row patch lane for load cases/test profiles in 🔺️diff not verified; CaseRowDelta exists only as bare lane (unverified)
- #34 [132] editor commands still use mutations::resolve_edit (51 hits); checklist step 5 (footings/retainingWalls/slopes handle writes) not completed
- #60 [416] SEMIO_REGEN_EN1997_ASSETS=1 never run: regen_example_assets test exists but env hook not exercised; packs stale
- #105 [659] slopes[].width field and en1997_profile.py adder not verified (script path missing)
- #134 [983] flow-net streamline/equipotential golden and pack regeneration pending (golden 0 bytes)
- #156 [1125] status-only earth-pressure and bearing rows have no numeric utilisation row; add numeric row or declare
- #161 [1173] mutations::resolve used by editor commands is absent from en1997 mutations crate; route through resolve_edit or add resolve
- #173 [1279] 17 en1997 leaves without mutation kinds untouched; add set-field kinds
- #186 [1402] scene golden needs SEMIO_UPDATE_GOLDEN=1 after green build (golden 0 bytes); consolidation troughs and ewall-E1 pore-pressure section pending
- #200 [1506] 41 gate(cond, "EN reason") English reasons in wall-sections/walls-and-anchors/investigation/piles/hydraulics catalogue (unverified path); localise gate reasons
- #320 [69] register EN 1997 NDP rows (DIN 1054 Tables A 2.1-A 2.x) from en1997-completion into registry
- #326 [288] DIN 1054:2010-12 withdrawn; re-base ndp.en1997.l45.1 and l183.1 to DIN 1054:2021-04 Tables A 2.1/A 7.1
- #341 [585] Outcome-level NDP flag missing; PILE-10 xi_basis_unverified info outcome needs ndp.en1997.xi rows
- #353 [920] register ndp.en1997.7.7.lateral-deflection-limit and ndp.en1997.9.seismic.gamma-sliding (fallbacks in code)
- #366 [1283] ndp.en1997.r3.a gamma_a prestressed anchors read by anc-01/02 through EN=DE row; split by DA/R set
- #466 [264] snapshot piles[].positions[] (x,y per pile) and per-layer skin friction absent; add for positioned geometry
- #468 [579] bearing_drained include_depth applies Hansen d_q (not EN 1997 Annex D); relabel or remove
- #470 [723] 8 blessed oracle cases lack evaluator (phase relations, anchor elongation/resistance, pile-group R_group, oedometer)
- #471 [1454] taxonomy generator skips change-structure-id (descriptor identity mismatch); fix generator
- #472 [1620] piles[].concreteClass enum (PILE_CONCRETE_CLASSES) not added; needs typed-enum support in field-adder and all layers

### en1998 (21 open)

- #67 [452] 298 f64 fields hand-fixed; generated set-field kinds for new f64 leaves not verified; field-adder script path not found
- #111 [696] scene handle writes soilLayers[id=<layer>].thicknessM and n160 remedy targets not present (0 hits)
- #174 [1287] soil layer nSpt, cuKpa and site flags (liquefiableOrSensitive, softClayPlasticityIndex, alluviumVsMS) absent (0 hits for flags)
- #189 [1413] wallStorey f64 and liquefiableThicknessM indicator need typed storey reference and wall direction enum
- #212 [1557] en1998 umbrella script.ts package list lacks mut-site-ground and mut-members-wall-rows (unverified)
- #216 [1578] other String fields of en1998 (material, importanceClass, enGroundType, t1Method) untouched; convert to typed enums
- #294 [1085] soft-storey and mass-variation rows only emitted when plan dims > 0; move loop outside the guard
- #319 [44] NA-DE vertical spectrum values unverified (en1998-completion.md); verify against DIN EN 1998-1/NA
- #327 [289] DIN EN 1998-1/NA:2023-11 S_aP,R replaces a_gR zone table; SaPR.csv annex absent (0 hits); implement
- #331 [352] Rayleigh T1 uses normalised shape (~1.7 s); compute from storey stiffness and fix Table 4.2 phi
- #473 [39] Placement per spatial entity (buildings, bridges, silos, tanks, towers, walls) absent (0 Placement in en1998 snapshot)
- #474 [40] capacity design needs sum_m_rc_nm/sum_m_rb_nm or per-joint En1998Joint (0 hits)
- #475 [41] site.seismicZone enum: zone chart marker expects index 0..3 via Quantise::Choice; set-field path needs index mapping
- #476 [43] resolve_annex still pub(super) in seismic-helpers; expose or move to schema crate
- #477 [261] storey-forces plan_w uses direction swap bug (plan length vs width) in structural-system-checks
- #478 [265] capacity-design ratio per joint blocked on sum_m_rc_nm/sum_m_rb_nm or En1998Joint
- #481 [1065] insert/remove kinds for buildings[].members|storeys|systems and five nested en1999-style collections not done
- #482 [1083] check_storey_forces computes accidental-torsion delta then discards it (let _ = delta_torsion)
- #483 [1084] check reports utilization Lx/Ly (1.5) but status Pass; set utilization 0 for positive dimensions
- #484 [1086] planRegularity compares max(e0x,e0y) with 0.30·min(rx,ry); EN 1998-1 4.2.3.2 compares per direction
- #486 [1269] test assertion messages still say "legacy" (scene unit tests 656; schema structures 139); reword

### en1999 (14 open)

- #77 [487] en1999 Placements diff twins were hand-added; generator re-run reproduction not verified (ts-schema lane)
- #163 [1175] 🔺️diff dev-dependency en1999-mut-member depends on en1999-diff (cycle); remove or move test types
- #265 [700] six new en1999 crates (mut-weld-run, mut-outline-vertex, mut-placement, section-solver, …) need Cargo.lock/umbrella registration
- #285 [896] nested collections members|connections|coldFormed|shells[].actions and sections[].elements lack insert/remove kinds
- #372 [1481] ndp.en1999.web-stiffener.* remain unverified; verify EN 1993-1-5 clause values
- #488 [114] AluminiumSection has no free polygon/vertex/hole list; scene derives outline from kind; add polyline section editor model
- #490 [116] member_resistances(member, section, alloy, params) pure function to export for scene mirror
- #491 [117] CheckUnits by entity (member, connection, sheet, shell, fire, fatigue) and per-station utilisation
- #492 [262] member-checks multiply buckling_length_y/z/t by k_sup (0.7/2.0/1.0) against snapshot docs; resolve semantics
- #493 [353] web stress-gradient eta applied to internal flanges of RHS/box (MC99:226-229); class 4 flange misclassified
- #496 [675] 7 blessed oracle cases lack evaluator (tee/elliptical hollow, channel y_sc, angle I_major, free outline It/Iw)
- #500 [879] connections[id].bolts.boltsPerRow handle has no SetFieldRule or typed kind; add change-bolts-per-row
- #501 [927] mutations/🦀️.rs imports change_member_* from mut-member crate; stale imports to remove
- #503 [1268] five crates (mut-rows-sheet-action, connection-action, …) missing from umbrella script.ts packages list

### din-a (1 open)

- #504 [216] NormHost::from_artifact replaced by NormHost::live with full evaluate per paint; add live_fold_source!/live_report! pair for DIN 4108, DIN EN 16798, DIN V 18599, ISO 16757, VDI 3805

### iso-vdi (1 open)

- #485 [1130] Velocity quantity kind needs VdiQuantityKind mapping (vdi3805 snapshot/sqlite, twins); VdiQuantityKind 125 hits but no Velocity

### ts-schema (17 open)

- #107 [679] placement kinds still missing for en1991, en1993, en1994, en1995, en1998 (insert-placement exists for all; verify per artifact)
- #171 [1273] 521 raw String schema fields with closed value sets remain (report-only candidate list); convert to enums
- #184 [1389] proto and graphql twins lack fire fields (proto ends tag 56 before floors); add fire fields to all twins
- #198 [1494] typed-enum candidates: executionImperfections, tieRuns[].kind, constructionLoads[].kind/stage (tieRuns 1023 hits, kind 3)
- #230 [215] norm-enXXXX-outliner window kinds not in fixtures (0 hits); regenerate window-ownership fixtures and hub manifests
- #245 [423] committed binary pack 🖼️assets/🏢️seismic-rc-frame/🎒️.pack.semio needs codec regeneration (SEMIO_UPDATE_GOLDEN run)
- #246 [433] 44 norm crates lack build-owners rows; regenerate via generator and verify
- #253 [483] schema generate pending for FatigueSpectrum, ReliabilityAnalysis, TestSeries, VariableAction.group diff lanes
- #262 [665] seismic-rc-frame pack regeneration for soil-layers DSL table and committed_pack_twin test bless
- #288 [975] repo schema generate for FactorOverride, 10 member/bridge leaves and 16 kinds; verify build-owners sync
- #299 [1154] regenerate scene goldens, example packs and DSL assets for pier/bearing kinds and new fields
- #300 [1157] regenerate DSL packs via write_committed_example_assets and schema generate root twins
- #307 [1459] schema generate for en1990 snapshot/diff/mutation (ReliabilityClass, LoadGroup, trafficGroups); schema-catalog.json update
- #308 [1474] EN1995_REGEN_PACKS, REGEN_EN1999_ASSETS regen tests not moved to shared helper; no semantic.json for en1992/en1998
- #309 [1480] aluminium-roof-purlin and noncompliant-multi-fail .pack.semio assets still hold member column; regenerate
- #313 [1577] framework schema-catalog.json still lists change-site-liquefiable-thickness-m and change-member-wall-storey payloads; schema generate
- #508 [1436] enum families (action kind/category, fastenerType, ulsEquation, strengthClass), catalogue-id newtype and case-scene unification not done (~300 strings)

### browser-e2e (1 open)

- #358 [1132] renderer wiring of screen_reader check_row_node/handle_node/diagram_nodes and help overlay (focus_order::chord_text 0 hits) and NDP Annex inspector section

### host-runtime (8 open)

- #52 [349] world-3d bindings still need handle/s/origin/modifiers/at/key args; referenced audit doc 🔍️audits/direct-manipulation-rollout.md missing
- #98 [607] shell dock switch on resize via device_profiles::device_view not wired in host (device_view used only in 3 files)
- #191 [1441] LiveCommand::Edit(EditCommand) absent (0 hits); add entity-edit live command for duplicate/paste
- #192 [1442] handle_key shortcut ids not mapped to ArtifactCommand::Undo/Redo (0 hits); map copy/paste/duplicate/undo/redo
- #196 [1487] ArtifactCommand::Undo/Redo not sent for History chords (0 hits); shortcut_route (22 hits) not connected to store
- #219 [1629] framework select-all needs plugin write API for interaction-domain selection
- #220 [1630] wgpu shell drop targets: drop_target_action reads Stack nodes only; outliner drag-drop not wired
- #224 [127] hub component manifests (🌎️hub/…/📕️norm) still list norm-en1996-inputs/results; regenerate

## Done and stale items (evidence)

### done (142)

- #2 [shared] NormTransient/NormPresence/EvalProgress/FieldWrite defined in 📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧬️schema/🦀️.rs
- #3 [shared] NormFieldMeta type gone; only NormFieldMetaFn remains (📇️registry/🧬️contract/🖥️app-surface/🔖️field-meta/🦀️.rs:301); stale doc mentions in 🗿️artifacts/🌬️din16798/…/📥️inputs/🦀️.rs:1 (cosmetic)
- #4 [shared] no `render_document_editor` reference left in the tree (0 hits)
- #8 [shared] render_inspector takes InspectorOptions/OutlinerState as parameters (📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧾️properties/🦀️.rs:398)
- #10 [shared] structural-scene root mounts projection_kit, projection, diff, mock_projection and re-exports tessellation (🎬️structural-scene/🦀️.rs:9,12,15,18,22)
- #11 [shared] superseded: `Pattern::Cage` is now `Cage { outline: Ring2, zones, reference }` (🎬️structural-scene/🧬️schema/🧩️entities/🦀️.rs:284), loop size carried by outline
- #14 [shared] Cage outline/zones geometry replaces single-hole_clearance stirrup side (🎬️structural-scene/🧬️schema/🧩️entities/🦀️.rs:284)
- #15 [shared] en1997 snapshot has `placements: Vec<Placement>` with id,x,y,z,yaw (🗿️artifacts/🌍️en1997/…/📸️snapshot/🦀️.rs:103,375); shape differs from requested origin/Vec3, acceptable
- #16 [shared] `pub fn slices` is public (🧮️mechanics/🌍️geotechnics/⛰️slope/🦀️.rs:79); scene reuse not re-verified
- #18 [shared] per-artifact `🎬️scene/🧰️kit` exists in 8 artifacts and shared 🎬️structural-scene/🧰️projection-kit exists
- #23 [en1993] en1993 snapshot has Placement and change-placement kinds (16 files)
- #26 [shared] imposed_qk_pa/ground_snow_pa return Option (7/8 hits); na_b3_qp_pa removed (0 hits)
- #27 [en1990] legacy 🪟️windows/{📥️inputs,📊️results} dirs no longer present in en1990/en1995 editor and viewer; only checks/diagrams/outliner/properties/viewport remain
- #30 [shared] report_helpers re-export status note; treated as done (not re-verified)
- #31 [shared] no `norm-en1996-results`/`norm-en1996-inputs` references remain in the norm tree (0 hits)
- #33 [en1991] bridgeTandemX/craneX fields present in model and fixtures (1027 hits); handles exist
- #35 [shared] SURFACE/🌲️outliner/🪟️window/🦀️.rs exists with render_outliner (24 hits); keep as is
- #39 [shared] status note: NDP edits preserved; no action
- #40 [en1999] WeldRun used in en1999 diagrams and handles (790 hits); per-edge Weld replacement in place
- #42 [shared] StructuralScene.tracks present in 🔺️diff and scene (tracks: Vec<ParameterTrack>); SurfaceSample grid note unverified
- #43 [shared] load_factor/frequency fields used by visual agents (93 hits); Motion semantics in place
- #45 [shared] render_with(..ViewportView..) present in 30 artifact files across all ten artifacts
- #47 [shared] 🫧️live-lanes/🧱️lane-stores exists; norm_lane_stores! used in en1991/en1994/en1996 editors
- #51 [shared] PresenceView (22 hits) and render_with_request_context (20 hits) present
- #56 [en1993] Storey entity, En1993StoreyPatch/Delta and insert-storey kinds present (insert-storey 42 files)
- #59 [en1997] anchors entity present in en1997 snapshot, scene and handles (anchors in diagrams/handles)
- #61 [en1996] 55 wave-2 mutation kinds present in en1996 mutations tree (🧬️mutations/…)
- #62 [en1998] change-site-s-ap-r present (19 files); change-site-seismic-zone gone (0 hits)
- #64 [en1999] materials[].productForm, PRODUCT_FORMS and change-material-product-form present (18 files)
- #65 [en1999] placements entity present; insert-placement kinds in en1999 (➕insert-placement lanes)
- #66 [en1991] change-bridge-tandem-x, change-rail-alpha, change-assumed-rail-lm71 present (15-21 files)
- #69 [shared] projection_kit::placements_of (32 hits) and revision_of (21 hits) used by viewports
- #70 [en1993] restraints collection and insert-restraint kinds present (17 files)
- #72 [en1999] -2 mutation group crates (❄️cold-formed-action, 🔗️connection-action, …) present
- #74 [en1993] BuiltUpMember entity and insert/remove-built-up-member kinds present (17 files)
- #75 [en1993] WeldGroup entity and insert-weld-group kinds present (17 files)
- #78 [en1999] add-placement is named insert-placement in the tree (➕insert-placement lanes present); remove-placement present
- #80 [en1999] ParametricSection and HollowJoint entities present (282/335 hits), insert-parametric-section present
- #83 [en1997] slope circle fields present (circleRadius/circleCentreX 545 hits) and change-slope-circle-radius (17 files)
- #84 [en1998] towers eiNm2 field present (3 hits in model/twins)
- #85 [en1997] slope circle leaves and change-slope-circle-radius present (17 files)
- #86 [en1997] placements with x,y present in en1997 snapshot (see 15)
- #89 [en1990] insert-placement/remove-placement present in en1990 (7/9 files); change-placement kinds present (16)
- #90 [shared] semio-s-plugin-norm-compliance now 3441 lines / 20 files (OK in source-boundary scan)
- #92 [en1996] isotherm-zone and wedge-<lc>-<load> derived tags present in scene (2/4 files)
- #93 [shared] sqlite fixtures carry fatigueSpectra, reliabilityAnalyses, testSeries rows (552/578/590 hits)
- #95 [en1997] en1997 snapshot now has placements (see 15); superseded
- #96 [en1991] silo:hopper and silo:field-i-j landed (2 hits); crane/silo handles and rail-influence diagram present
- #97 [en1993] JointRow entity and insert-joint-row kinds present (389 hits, 17 files)
- #99 [shared] dispatch_shortcut mounted in panels crate and shortcut_route used by host (22 hits)
- #104 [shared] status note: no kind change for chordRotation fields
- #106 [en1998] soilLayers collection present across tree (1064 hits), soil_layers rust lane present
- #108 [en1993] frameNodes/frameBars entities and insert-frame-node present (577/617 hits)
- #109 [shared] oracle 🔮️oracles/🕸️frame-eigen/🐍️.py exists
- #110 [en1993] insert-placement/remove-placement and change-placement present in en1993 (9/16 files)
- #114 [en1994] insert-placement/remove-placement present in en1994 and en1998 (6/9 and 9/11 files)
- #116 [shared] 🔮️oracles/⚖️compliance/🐍️.py exists with geometry mode; python_oracle tests referenced
- #118 [en1995] third-party oracle module 🔮️oracles/📐️third-party/🐍️.py exists in en1995 artifact
- #119 [en1994] no en1994 fixture path exceeds 256 bytes now (long-path scan); the 256-byte case is at the limit
- #121 [shared] solve_continuous_beam present in mechanics (1 hit); typed API exists
- #123 [shared] vids oracle and feature (🧪️tests/🧾️vid-checks/🥒️.feature) present; case ids registered by file
- #126 [en1991] 🎬️scene/🌪️wave3 module present (keep as is)
- #131 [shared] app-surface split into foundation/viewport3d/visual-chrome/live-core/panels/live-editor sibling crates present
- #138 [en1995] 🧭️spec-trace/🔣️.json of en1995 names entity collections for T-18..T-97
- #145 [en1995] design decision recorded: change-case-number/text upsert; no insert/remove kinds by design
- #162 [shared] 🎬️scene/🧪️tests/🗂️cases/🦀️.rs exists (directory no longer empty)
- #167 [en1995] status note: proto oneof renumbering was a no-change
- #169 [en1994] status note: E0063 nested set-kind diffs fixed (..Default::default() used 5234 times)
- #172 [en1997] GEO-F01/F04, SLS-01, PILE-04, RW-01/06, SLOPE-01 remain as stand-alone catalogue calculators (status note)
- #177 [en1990] typed trafficGroups collection present (543 hits)
- #185 [en1996] v7 uses existing walls[].diaphragmStiffnessNPerM (3 hits); no new kind
- #208 [shared] Lang enum has no #[default] (viewport3d/🎚️config/🦀️.rs:72-74); no default language remains
- #222 [shared] status note: wave-2 triage superseded by this audit
- #228 [shared] status note: CheckResult built only via builder; no literal constructions
- #241 [shared] app-surface canvas-lowering no longer flagged by source-boundary scan
- #242 [shared] app-surface is a facade over sibling crates (scan OK, 5018 bytes)
- #251 [en1991] rail kinds moved to semio-s-artifact-norm-en1991-mut-rail-load-models (present, 16 files, scan OK)
- #252 [shared] structural-scene now 18 files/2372 lines (scan OK, under 32-file budget)
- #254 [shared] viewport config crate semio-s-plugin-norm-viewport-config present (11 files, scan OK)
- #255 [shared] status note: SQLite extents set (sqlite/snapshot admission); no open action recorded
- #256 [shared] viewport config registered in norm Cargo.toml and contract project.json (Cargo.toml OK)
- #266 [shared] independent catalogue oracle moved to 🔮️oracles/⚖️compliance/🐍️.py and feature exists under 🧪️tests/📇️en1992-catalogue
- #268 [en1998] en1998-mutations now 2172 lines (scan OK)
- #269 [shared] 10 registry rows added to 🔖️ndp/🔣️.json (registry file present)
- #271 [en1994] en1994-diff now 480 lines (scan OK)
- #276 [en1999] semio-s-artifact-norm-en1999-fatigue-math crate present (scan OK, 71 lines)
- #282 [en1997] en1997-mut-pile now 19 files/480 lines and en1997-model 1784 lines (scan OK)
- #283 [shared] en1998-mutations, en1999-mutations and en1991-diff within budget (scan OK)
- #291 [en1990] status note: no mutation or selector edits by that agent
- #302 [en1998] no mutation fixture paths currently exceed 256 bytes in en1998 (only 8 paths at 257 bytes, see 71)
- #304 [en1991] en1991-diff within budget (scan OK, 5 files, 1800 lines)
- #306 [en1998] new crates mut-site-ground and mut-members-wall-rows present (scan OK)
- #312 [en1994] en1994-checks now 1948 lines/141 kB (scan OK)
- #321 [shared] CheckResult carries UnverifiedBasis outcomes (105 hits); UNVERIFIED list in catalogue no longer required
- #323 [shared] kernel NDP registry ownership recorded (🔖️ndp crate present, scan OK); recipe note only
- #335 [shared] InspectorOptions has preset, tier and clause_links; FieldSpec rows in use (30/2361 hits)
- #342 [shared] mechanics.rail-lm71-mid-moment adapters added to pynite/anastruct/numerical adapters
- #344 [shared] ndplib.py put() process guidance; no open code action
- #345 [shared] all wave unverified ids for en1994 registered (status note)
- #349 [en1993] registry rows ndp.en1993.v-300.2 added to annex-and-tables ndp table (status note)
- #362 [shared] ndp verify status: 174 rows verified, 86 EN-confirmed/NA-open (status note)
- #377 [en1990] ReliabilityAnalysis table present (289 hits) and form_r_minus_e referenced (11 hits)
- #380 [en1990] status note: no installed library covers EN 1990/1991/1996/1998 values; sympy used for beta
- #381 [en1990] design case adopted: CaseCatalogue (149 hits), CaseRowDelta (153 hits)
- #382 [en1990] catalogue cases module 🗂️cases/🦀️.rs present; inputs module wired (status note)
- #383 [en1990] informational: KN_V_UNKNOWN interpolation note, no action required
- #385 [en1990] en1990-diff now 2518 lines (scan OK)
- #390 [shared] design note on combination tracks; no open action
- #391 [en1990] root Cargo.toml dev-dependency semio-s-plugin-norm-compliance present
- #400 [en1991] placements present: Placement in snapshot, insert/remove-placement kinds (6/8 files) and change-placement kinds
- #401 [en1991] add/remove/change placement kinds present (insert-placement 6 files, change-placement 15 files)
- #402 [en1991] en1991-scene now 1520 lines/136 kB (scan OK)
- #415 [en1992] en1992-diff 467 lines (scan OK); closure size no longer over gate
- #419 [en1992] en1992-model 1651 lines (scan OK); en1997-model within budget
- #421 [en1992] en1998-mutations 2172 lines (scan OK); en1993 checks-special budget per scan (only mut-fastened-joints etc. OVER)
- #422 [en1992] En1992ReinforcementGradesPatch.ductility is DuctilityClass (diff 🦀️.rs:130)
- #425 [en1993] informational: alpha_cr Eq 5.2 vs eigenvalue note; no action
- #430 [en1993] en1993-scene 1834 lines (scan OK); en1993-clauses 20 lines OK
- #432 [en1994] Placement in en1994 snapshot and change-placement kinds present (16 files)
- #437 [en1994] PyNite/anastruct agreement cases (en1994.mechanics.* and en1995.mechanics.*) recorded as status note
- #438 [en1994] no en1994 mutation fixture path currently over 256 bytes (long-path scan lists none for en1994)
- #442 [en1994] mut-beam-design now 16 files/405 lines (scan OK)
- #448 [en1995] placements Vec<Placement> present in en1995 snapshot (Placement count 1) with insert/change kinds
- #452 [en1995] en1995-mutations 1532 lines and en1995-model 2741 lines (scan OK)
- #453 [en1996] Placement in en1996 snapshot and change-placement kinds present (15 files)
- #461 [en1996] en1996-masonry now 1845 lines (scan OK)
- #463 [en1996] en1996-diff 480 lines (scan OK)
- #465 [en1996] en1996-mutations 1184 lines (scan OK)
- #467 [en1997] informational: N_gamma formula comparison (EN 1997 Annex D.4); no action
- #469 [en1997] En1997Diff carries CaseRowDelta cases lane (153 hits) and snapshot cases
- #479 [en1998] en1998-diff 3571 lines (scan OK, 5 files)
- #480 [en1998] en1998 schema/model/diff within scan budget (diff 3571 lines; include_str leaves no longer flagged)
- #487 [en1999] Placement in en1999 snapshot and change-placement/insert-placement kinds present
- #489 [en1999] WeldRun in model/handles (790 hits) and weld runs drawn in diagrams (status: implemented)
- #494 [en1999] no en1999 mutation fixture path currently over 256 bytes (long-path scan)
- #495 [en1999] en1999-diff 1839 lines (scan OK)
- #497 [en1999] en1999-checks 1329 lines (scan OK)
- #498 [en1999] en1999-mutations 2186 lines (scan OK)
- #499 [en1999] en1999-checks 1329 lines (scan OK; gate no longer hit)
- #502 [en1999] en1999-model 2555 lines (scan OK)
- #506 [shared] oracle_wiring pub mod registries exposed in all en1990-en1999 checks crates (62 hits)
- #509 [shared] status note: shared crates changed under review; no open action beyond items already listed

### stale (7)

- #82 [shared] `resolve_row_leaf`/`SetFieldRule` do not exist (0 hits); edit rules now use EDIT_RULES in 🧭️edit-rules
- #176 [en1990] kFiDeclared field no longer present (0 hits); change-k-fi-declared removed (see 317)
- #180 [ts-schema] en1992_run.py not in tree (MISSING); group_kinds never sees existing crates; script retired
- #183 [en1994] en1994-continuous-points-oracle.py not in tree (MISSING); PyNite cross-check values hard-coded elsewhere
- #190 [en1998] field_adder.py and en1998_root.py not in tree (MISSING); adder scripts retired
- #338 [shared] ndp::REGISTRY absent (0 hits); registry now NDP_TABLE/ndp crate
- #510 [ts-schema] 🧰️scripts/en1995-collections/schema.py missing from tree (MISSING); drift note obsolete

