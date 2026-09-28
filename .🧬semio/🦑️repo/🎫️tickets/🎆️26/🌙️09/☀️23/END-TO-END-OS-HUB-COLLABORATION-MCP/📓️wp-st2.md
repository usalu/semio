# WP-ST2 — Stdio Completeness: Native Text Codecs, Nx Dist Inputs, Per-Family Components

Slice ST2, session 14 (2026-09-27 18:3x, Opus executor). Coordinator = `main`. Continues [ST1](📓️wp-st1.md) and
[CX1](📓️wp-cx1.md) (design: [LB](📓️wp-lb.md) "Option A"). Ports: hubs 8070–8079, serves 6570–6579. Scripts + patches:
`wp-st2/` (copies of `wp-cx1/cx1-apply.py`, `wp-st1/st1-{gen,apply}.py` evolve here as `cx1-apply.py`, `st2-gen.py`,
`st2-apply.py`). Captures `wp-st2/generated/`. Durable data `.🧬semio/🌐hub/s14-st2-*/`. Guest-linked edits = prepared
patches + overlay proofs until the coordinator announces WINDOW 3 OPEN in `📓️fleet-14-agents.md`.

## Session 14c

| # | Item | State | Evidence |
|---|------|-------|----------|
| 0 | live tree carries no half-applied ST2 guest edit beyond the landed chain fixes | **verified 17:0x**: generator identical partition (9 families, 158 + 18 apps); `st2-apply --part code` 10 edits / 27 new / 0 problems, `--part r10` 9 edits / 9 new / 1 move / 0 problems; `cx1-apply` 80 hunks / 34 files / 0 problems (generation anchor `bda0b90f…`); no `🗄️stdio/🧩️extensions`, `🧩️composition` unmoved; landed U1–U5 present (vcs/forms/architect xlsx on `project_workbook`/`build_minimal_xlsx`, svg pin `964a7571…` ×2, kind-formats `hostSnapshot`). code-dry diff vs the 12:20 overlay write = only the hub publisher rows added 12:3x | `generated/st2-gen-live-5.txt`, `st2-dry-live-{code,r10}-6.txt`, `cx1-dry-live-7.txt` |
| 0b | nx1b base drift | receipt law base changed 12:39 (R10's `[DEBUG]` sweep removed its log line; payload already has the emoji line) → reviewed-base sha re-pinned `5dd1a74e…`, dry run 8 files / 0 problems | `nx1b-dry-live-7.txt` (refusal), `nx1b-dry-live-8.txt` |
| 1 | both parts dry-run clean on the live tree | **20:01**: generator partition unchanged; `--part code` **13 edits** / 27 new / 0 problems (3 new artifact edits = classification fix; editor-catalog + shipped-fleet edits carry the details-window fix), `--part r10` 9 edits / 9 new / 1 move / 0 problems; CX1 80 hunks / 34 files / 0 problems; 1b `--part rebuild` 1 file / 0 problems. Overlay files == patch(live) for the changed transforms | `st2-gen-live-6.txt`, `st2-dry-live-code-13.txt`, `st2-dry-live-r10-9.txt`, `cx1-dry-live-11.txt`, `nx1b-dry-live-13-rebuild.txt` |
| 4 | 1b generator-inputs cache fix | **LANDED 17:26** (`nx1b-apply.py --part now`, 8 files, atomic writes). Found + fixed a gap in the prepared patch: the cache-contracts generator section asserted `guard.cache === true` for `repo:generator-inputs` → would have gone red (mutant proves it); new hunk asserts the uncached producer; same file drops the root `generate` aggregate a peer deleted 09-26. **HELD** `--part rebuild` (rebuild JSON drops `--skip-nx-cache` ×4 → changes the running chain's retries) → window 3. Laws: receipt law PASS live, vectors 34/34 live, generator section PASS live (verbatim-block runner `nx1b-generator-contracts.ts`, after an Nx graph refresh) | `nx1b-law-live-1.txt`, `nx1b-vectors-live-1.txt`, `nx1b-generators-live-3.txt`, `…-overlay-mutant-1.txt`; landing row |
| 3 | CX1 txt/tsv/html codecs | dry run 0 problems on the live tree (before and after 1b); live stored generation == derived `bda0b90f…` (OLD correct); overlay (CX1 applied) derives NEW preview **`7d60c569…`** (window 3 recomputes with the h14 tool `--write`); overlay hub `trusted-stdio-gis-bundle-check --source` **rc 0** with the preview written (bootstrap oracle codecs=31 = 29 stdio + 2 GIS); hub TS tsc 0 errors. Overlay native: html 42/42, tsv 27/27, txt 49/54 (5 pre-existing provenance reds, row F); CX1's own provider law was red (`compile_dsl(source, "")` now needs the ops `doc` header) → payload law fixed → **provider 8/8** | `cx1-dry-live-10.txt`, `cx1-generation-{live,overlay}-1.txt`, `cx1-bundle-check-overlay-2.txt`, `hub-tsc-overlay-1.txt`, `lane-job{3,4}-1.txt` |
| 2 | overlay proof of the families | overlay re-synced 17:0x (844 cloned) + CX1/1b/ST(all) applied; TS: stdio catalogue contract **88 editors / 10 packages / 36 formats** (patched union gate), taxonomy valid, overlay Nx graph resolves all 9 `@semio-tech/stdio-<family>-plugin` projects; overlay registry `generate` + **play pane coverage 11/11**; **native `cargo check --lib --tests` of the 10 packages rc 0** (18:03–18:08, 0 warnings in `🧩️extensions`, 915 elsewhere = real check); **wasm32-wasip2 `cargo check --lib` of the 10 packages rc 0** (18:08–18:17, 0 family warnings); laws found latent faults the families expose, fixed in the code part: interactive-job classification (wav, semio mesh/brep) and details-window-owned snapshot actions (`editor_catalog`, `shipped_fleet`) → **`shipped_fleet` 3/3 PASS** (job5, 19:59); `editor_catalog` stays red 1/89 on pre-existing classes the ST patch now makes visible (it was gated behind `full-app-catalog`): 74 `snapshot-edit.schema-unregistered`, 8 stale fixture paths, 6 jpg/gif/tiff default-encode (row F). Overlay private build-dir `s13-cx1-build` (7.8 GB) + target **deleted 20:01** (rule 23) | `stdio-contract-overlay-5.txt`, `play-pane-overlay-2.txt`, `lane-job{3,4,5}-1.txt` | `stdio-contract-overlay-5.txt`, `taxonomy-probe-overlay-5.txt`, `overlay-nx-projects-1.txt`, `play-pane-overlay-2.txt`, `lane-job3-1.txt` |
| F | pre-existing reds found (not ST2's) | full `⚡️caching` `bun ./📜️script.ts test` is red on the live tree at ≥ 5 independent points: root `generate` gone (fixed in item 4), `🔗️import-edges` law reads a vanished ticket snapshot `.tmp-ticket/wp-o2c/generated/nx-iso3/…`, wgpu boot inputs `1 !== 4` (`🧊️wgpu-browser-boot-cache-inputs`), publication `devOptimizedCargoConfigArgs is not defined`, inventory fixture `@semio-tech/flow-core` (project is `semio-framework-os-flow-core` since 09-24), `component-dev` `parallelism: false` vs the law. stdio `editor_catalog` (ungated by the ST patch): 74 editors `snapshot-edit.schema-unregistered` (`s.stdio.<kind>` absent from the kernel schema catalog when the law drives an editor), jpg ×2 / gif ×2 / tiff ×2 `EditorApp::default()` cannot pack-encode (new documents of those kinds fail), `editor-catalog` fixture snapshotEdits paths stale for gltf, obj, pdf14 ×3, xlsx ×3; txt 5× provenance `taxonomy_path` expectation stale (derive now names `🔣️mutation-authority.json`) | `cache-contracts-overlay-{6,8,9,11,12}.txt`, `lane-job{3,4}-1.txt` |

### Session 14c Log

- 16:58 start (successor, chain relaunched 16:55:46, freeze ON). Read preamble 14/14b/14c, AGENTS.md, fleet tail, this report.
  Predecessor's queued overlay job2 (13:07) never ran (capture has only QUEUED); nothing after 14:23 besides the `cx1-apply`
  generation anchor. Overlay `s13-cx1-overlay` holds the 12:28 CX1+1b+ST(all) application; its private build-dir
  `s13-cx1-build` is down to 2.1 GB (pruned), `s13-cx1-target` empty; `s14-st2-overlay` (1.4 GB, abandoned 09-27 clone) still exists.
- 17:07–17:11 overlay re-synced (`st2-overlay.py`: 79 517 wanted, 844 cloned, 17 305 stale removed), then `cx1-apply --write`,
  `nx1b-apply --write --part all`, `st2-apply --write --part all` on the overlay (`generated/{cx1,nx1b,st2}-write-overlay-5.txt`).
- 17:1x 1b: the full `⚡️caching` suite on the overlay (needs `SEMIO_TICKET_DIR` = a ticket with `🎫️ticket.json` under the root's
  `.🧬semio/🦑️repo/🎫️tickets`, `SEMIO_TEST_ARTIFACT_DIR`, a cloned binaryen tool dir — symlinks refused, a fresh Nx graph) stops on
  pre-existing reds one after another (row F); overlay-only bypass edits of those sub-laws reached the generator section, whose
  `guard.cache === true` would refute 1b → hunk added to `nx1b-apply.py`; proof via the verbatim-block runner instead of the suite.
- 17:22 overlay job3 queued (`st2-job3.zsh` via `st2-lane.zsh`, 180-min hold, detached pid 97501; play pane coverage, native
  `--lib --tests` + wasm32 `--lib` check of the 10 stdio packages, stdio census/provider, txt/tsv/html; disk gate 45 GiB).
- 17:26 1b `now` part LANDED on the live tree (row + laws above). 17:3x deleted my abandoned clone overlay `s14-st2-overlay` (1.4 GB).
- 17:3x CX1 generation: live stored == derived `bda0b90f…`; overlay derives `7d60c569…` (`cx1-generation-preview.ts`, read-only wrapper
  of H14's tool); written into the overlay fixture only → hub `trusted-stdio-gis-bundle-check --source` rc 0; hub TS `tsc -p
  🌎️hub/📦️packages/🟦️typescript` (`--incremental false`, covers the hub script's CX1 fence + ST family publisher rows) rc 0 / 0 errors
  (`generated/hub-tsc-overlay-1.txt`).
- 18:03 job3 hold START (overlay lane). Its play step failed at module load ("Unknown play pane variant: stdio-png") — expected: the
  overlay still had the live tree's generated registry. Ran `📇️registry` `bun ./📜️script.ts generate` in the overlay (69 plugin crates,
  155 playgrounds, 88 stdio variants; `registry-generate-overlay-1.txt`), then `🎡️play` `🔨️modules/🧩️runtime/🟦️.ts`: **play pane coverage
  11/11 PASS** (`play-pane-overlay-2.txt`). Whole file: 16/19 — the 1 red also on the live tree (17 norm/flow panes "would boot demo by
  accident", pre-existing, live 18/19 `play-pane-live-1.txt`) + 2 reds that need the families' committed descriptors (`🧩️extensions/<family>/🔣️.json`,
  written by window-3 `describe`): "names only examples the pane's own app publishes" (53 family panes) and "checks against a manifest"
  (79) (`play-pane-overlay-3.txt`) → runbook step 6 proves them after describe.
- 18:08 native `cargo check --lib --tests` of the 10 stdio packages rc 0; 18:17 wasm32 `--lib` rc 0 (0 family warnings).
- 18:17–18:33 job3 laws — three REAL findings the check could not show:
  (a) `shipped_fleet` 0/3 + `editor_catalog` (wav, semio mesh, semio brep): `app-definition.interactive-job-classification`:
  wav's Main-window `insert-frame`/`insert-channel`/`set-sample-rate` (`edit_audio::extra_actions()`, appended after the kit
  scaffold stamped its rows — LB2 p4's shape) and semio mesh/brep `set_vertex_action()` (window + app) are `bounded_catalog`
  rows never classified, although all are retained routes (bounded tool publication contracts / retained work). Latent on the
  live tree (none of the three is in the 18-app shipped subset); the families ship them → every family `plugin()` / describe
  would panic. FIX added to `st2-apply --part code`: stamp `Migrated` at the declaration (3 files).
  (b) `editor_catalog` 1/89: every editor "is missing setSnapshotValue" — the SDK now keeps actions a window kind explicitly owns
  out of the app roster (`app_roster`), and `snapshot_details_window_definition()` owns the six snapshot edit actions; the law
  still read `definition.actions`. Latent on the live tree (the law needs `full-app-catalog` there, nobody runs it). FIX added to
  the code part's editor-catalog edit: read the actions from the details window kind.
  (c) CX1's own law `text_document_codecs_compile_…` 7/8 provider: `compile_dsl(source, "")` now refused ("ops text has no
  document header") — the ops parser requires the `doc` header even for a new document. FIX in `cx1-payload` law: genesis ops =
  `doc cx1 schema="<codec.schema>"`. Pre-existing and not ST2/CX1: txt `--lib` 49/54 — 5× `canonical_leaf_metadata_matches_descriptor_and_provenance`
  expect `taxonomy_path` = `📚️library/🔣️taxonomy.json`, the derive now reports `🗣️dsl/✨️derive/🔣️mutation-authority.json` (peer 09-26/27;
  the txt tests are from 09-09); html 42/42, tsv 27/27.
- 18:4x all fixes applied to the overlay (st2-apply helpers + payload law); job4 queued 18:41 (5th in the overlay lane): wasm32
  check media+semio, native check wav+semio `--tests`, wav lib laws, stdio census/catalogue, provider laws. Dry runs on the live tree
  after the fixes: code 13 edits / 27 new / 0 problems (`st2-dry-live-code-9.txt`), CX1 0 problems (`cx1-dry-live-10.txt`).
- 19:01–19:14 job4: wasm32 check stdio-media + stdio-semio rc 0; native `--lib --tests` check wav + semio artifacts rc 0; wav lib
  37/37; **provider `native_openable_provider` 8/8 (CX1 law green with the genesis `doc` header)**; `shipped_fleet` 2/3
  (`every_stdio_package_ships_exactly_its_declared_bounded_fleet`, `every_stdio_kind_is_opened_by_exactly_one_package` PASS —
  classification fixed); the 3rd law read `app.actions` for the snapshot edit ids (same SDK roster change as (b)) → template fixed
  to the details window. `editor_catalog` 1/89: 70 × `snapshot-edit.schema-unregistered` (the per-editor law never assembled a
  package, and assembly is what registers artifact schemas) → the code part now assembles all 10 packages once (OnceLock) before
  driving an editor; remaining pre-existing editor/fixture reds (not ST2): jpg ×2 "empty image", gif ×2 "empty logical screen",
  tiff ×2 "requires an ImageWidth tag" — `EditorApp::default()` pack encode fails (a NEW jpg/gif/tiff document cannot be encoded);
  "no-op fixture path" ×8 (gltf, obj, pdf14 ×3, xlsx ×3 — `🧫️fixtures/✏️editor-catalog` snapshotEdits paths drifted from the
  peers' snapshot shapes). job5 (census + catalogue re-run) queued 19:16.
- 19:57–19:59 job5: **`shipped_fleet` 3/3 PASS**; `editor_catalog` 1/89 — a OnceLock assembly of all 10 packages before driving an
  editor did NOT register the schemas (74 × schema-unregistered unchanged, although stdio's csv declaration carries
  `.schema(csv_artifact_schema_descriptor())` → `builder.artifact` → `commit_artifact_declarations`; the per-artifact editor unit
  tests register their descriptor explicitly, e.g. mp4) → helper reverted from the patch and the overlay (no dead weight); root
  cause NOT established. Open question for the stdio editing owner: does a real stdio / family component register `s.stdio.<kind>`
  before a snapshot edit (S18's csv/tsv/json/xml "edits apply 0" at 14:3x may share it — unverified).
- 20:01 final dry runs on the live tree clean (row 1); overlay build-dir + target deleted (rule 23, 7.8 GB), overlay scratch
  (fake ticket, binaryen clone, artifact dirs) removed; the overlay-only cache-contracts bypass edits are undone by the next sync.
  Captures trimmed (job3 warnings stripped; `overlay-removed.txt` → `overlay-removed-summary.txt`).

## Session 14b

| # | Item | State | Evidence |
|---|------|-------|----------|
| 0 | live tree carries no half-applied ST2/CX1/1b edit | **verified 12:1x**: all three dry runs on the live tree 0 problems (every anchor exactly once, no "already applied" edit, no new file present); no `🗄️stdio/🧩️extensions`, no `🏘️composition`; diff vs yesterday's dry diff = context line numbers only | `generated/st2-dry-live-3.txt`, `cx1-dry-live-3.txt`, `nx1b-dry-live-3.txt`, `st2-gen-live-3.txt` |
| 1 | 1b registry Nx soundness | held for window 3: it edits chain inputs the running chain reads (`🔁️rebuild/🔣️.json` via `rebuild-all --from`, registry + caching `📋️project.json`, `⚡️caching/🔣️policy.json` → every `bun nx` graph) — rule 2 freezes them even though no guest links them | — |
| 2 | CX1 txt/tsv/html codecs (26 → 29) | dry run clean; overlay re-proof pending | — |
| 3 | ST per-family stdio components | generator + dry run clean on the live tree (9 families, 158 + 18 apps, 18 edits / 36 new / 1 move); overlay re-proof pending | — |
| 4 | every stdio kind opens/edits/exports in `s` + hub | window 3; hub publisher rows for the 9 families added to the code part (guest-codec packages, `linkedCodecRegistry: null`, right after `stdio` in `--packages all`) | `generated/st2-dry-live-code-5.txt` |
| U1 | URGENT chain fix (coordinator 12:4x): `XlsxSnapshot` became OPC `xml_parts` (peer 04:35) → vcs xlsx import/export red (E0609/E0560), same break in forms + architect program exporters | vcs: export = stdio `build_minimal_xlsx`, import = `XlsxSnapshot::project_workbook`, first column = col 0 (was B); calamine oracle law replaces the bare round trip; forms: same export fix + new calamine law; architect: exporter + unit test + oracle-case projection onto `project_workbook`. Test-only `calamine 0.36.1` dev-dep in vcs + forms with the two Cargo.lock edges (`cargo metadata --locked --offline` rc 0). Native test queued (priority stamp) | `generated/native-vcs-1.txt`, backups `vcs-xlsx-before/` |

| U2 | forms LIB red on the live tree (peer 04:41/04:46): 3 panels called `crate::editor::forms::forms_play_labels` (only `…::terminology::` has it) + test-only: inspection test on the old `render(…, labels, …)` signature, editor unit test `include_str!` one dir too high | obvious path fixes (non-test lib code → chain-failure category, like U1) | `generated/native-forms-1.txt`, `native-fix2-1.txt` |
| U3 | H14 relay: `🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json` fails its schema (`componentKind` const `hostSnapshot` since the 09-15 fixture→hostSnapshot terminology rename, fixture kept `"fixture"`; Rust `component_kind` is a free string) | fixture value → `hostSnapshot` (rule 22, test-only); hub `trusted-stdio-gis-bundle-check --source` now passes it and stops at the NEXT red: `🔗️compiled-dependencies` rawCases hex descriptors still carry `appChannelVersion` 18 vs constant 19 (H14's channel-19 fixture pass, hub owner) | `generated/kind-formats-source-1.txt`, `native-fix2-1.txt` |
| — | CX1 generation anchor | `BOOTSTRAP_GENERATION_OLD` → `76d1a92f…` (H14 channel 19); NEW in window 3 via `bun .tmp-ticket/wp-h14/h14-bootstrap-generation.ts 19` after the CX1 codec change (my overlay probe `cx1-generation.py` gave `0021ba99…` on the 12:26 channel-18 tree — superseded); dry run 0 problems | `generated/cx1-dry-live-5.txt` |

### Window-3 runbook (ST2, compile-atomic, one step at a time; start on "WINDOW 3 OPEN")

1. Re-dry-run on the live tree: `python3 wp-st2/nx1b-apply.py --part rebuild`, `cx1-apply.py`, `st2-gen.py` + `st2-apply.py --dry-run --part code`
   (all must report 0 problems; else re-derive the refused anchor first).
2. 1b remainder (14c: the `now` part LANDED 17:26): `nx1b-apply.py --write --part rebuild` (rebuild chain drops `--skip-nx-cache` ×4)
   → `bun wp-st2/nx1b-law.ts /Users/ueli/Documents/semio` → row.
3. CX1: `cx1-apply.py --write` → `bun .tmp-ticket/wp-h14/h14-bootstrap-generation.ts --write 19` (stdio receipts 26 → 29 change
   the stdio-gis generation; the tool writes all 4 occurrences) — 14c overlay preview `7d60c569…`; then `trusted-stdio-gis-bundle-check --source` (TS, needs
   `SEMIO_TEST_ARTIFACT_DIR` under a `🗑️generated` dir) → native lane: `cargo test -p semio-s-plugin-stdio --lib --tests`
   (+ `--features full-artifact-catalog --test native_openable_provider`), `-p semio-s-artifact-stdio-{txt,tsv,html} --lib --tests`,
   `-p semio-hub --lib` (fence/provider laws) → row.
4. ST code (14c: now also the wav + semio mesh/brep classification stamps and the details-window reads in `editor_catalog` /
   `shipped_fleet`; overlay: `shipped_fleet` 3/3, `editor_catalog` red on pre-existing classes, row F): `st2-apply.py --write --part code`
   → native: `cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog`
   + `cargo check -p semio-s-plugin-stdio-{image,media,cad,bim,mesh,pdf,office,semio,binary} --lib --tests`; wasm lane:
   `cargo check --target wasm32-wasip2` of the 10 stdio packages, then `bun nx run @semio-tech/stdio-plugin:editor-component-check`
   (links + validates every stdio package component; the 1M-function ceiling proof, stdio-semio = 38 apps is the largest) → row.
5. RELAY R10: `st2-apply.py --write --part r10` as its serialized step, then its refresh/taxonomy/render (158 stdio launch rows).
6. Descriptors: stdio + 9 families `describe` (wasm; stdio's committed descriptor goes stale with steps 3–4 — its
   `descriptor_is_fresh` law is red until then), `plugin-registry:generate`, play pane coverage law, stdio catalogue contract.
7. Item 4: `serve s react dev` (6570) opens/edits/exports one document per stdio kind (os-dev program-matrix rows), then a
   `--packages all` publish (families included via the hub publisher rows of step 4) → hub-document-sweep on the fresh hub.

### Session 14b Log

- 12:0x start (successor). Read preamble 14 (+14b), AGENTS.md, fleet log, this report, the Codex stdio ticket (plan, validation):
  the peer still ships the nine-editor stdio component; its gate `testEditorCatalogContract(…, shipping)` + `--full-catalog`
  diagnostic are the 19:11 shape the ST patch already rewrites (union over the ten stdio packages, no `[DEBUG]`).
- 12:09 generator re-run on the live tree: identical partition (`generated/st2-gen-live-3.txt`); plan/payload of 09-27 kept in
  `generated/prev-s14/`.
- 12:1x overlay `s13-cx1-overlay` re-synced (2 395 + 86 files; docx/xlsx changed 12:12–12:19 by the chain-green fixer) and
  CX1 + 1b + ST (all) re-applied (`generated/{cx1,nx1b,st2}-write-overlay-4.txt`). Overlay proofs: taxonomy load valid
  (`taxonomy-probe-overlay-3.txt`); stdio catalogue contract **88 editors / 10 packages / 36 formats** (`stdio-contract-overlay-3.txt`),
  mutant (family 🔢️binary moved away) **red 32/36** (`…-mutant-3.txt`); live tree still red 7/36 as the peer expects
  (`stdio-shipping-contract-live-3.txt`); 1b receipt law **PASS** (`lane-job1-1.txt`); native `cargo check -p semio-s-plugin-stdio`
  + the 9 families `--lib --tests` **rc 0** in 11.5 min (overlay lane, private build-dir; only warning in the families = unused
  `extern crate value_derive` → removed from the generator template together with the unused `semio-framework-value-derive` dep).
  Play pane coverage: wrong vitest filter (the law is registered in-source by `🔨️modules/🧩️runtime/🟦️.ts`) → rerun pending.
- 12:2x `st2-apply.py --part code|r10|all`: the r10 part (taxonomy, workspace-contract counts, root policy row, every
  `📋️project.json`, the composition move + referrers, launch seed/json) is R10's serialized window-3 step; code-then-r10 ==
  all byte-identical (scratch mini-root), re-application refused. RELAY R10 sent via main.
- 12:3x hub publication for the families (item 4 prep): the hub admits the same kind in several packages (codec identity =
  plugin + package + kind + schema; `GuestArtifactCodecBinding` for packages without a linked provider), so each family
  publishes like note/draw: own component-probed codec rows, open targets bound to them. Added to the code part
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`: 9 `TRUSTED_BOOTSTRAP_PACKAGES` rows + `--packages all` order) — lands only after
  W4's publish (the chain's `--packages all` reads that list).

## Session 14

| # | Item | State | Evidence |
|---|------|-------|----------|
| 1a | CX1 txt/tsv/html hub-native codec factories (26 → 29) | dry run on the live tree **clean** (80 hunks / 34 files / 0 problems, 18:3x); applied to the overlay 20:07; native tests queued in the overlay lane (11 waiters ahead) | `generated/cx1-dry-live-1.txt`, `generated/cx1-write-overlay-1.txt`, `generated/cx1-test-stdio-1.txt` |
| 1b | registry Nx soundness (drop `--skip-nx-cache`) | **patch prepared** `nx1b-apply.py` (8 files; dry run on the live tree **0 problems**): `generator-inputs` uncached (out of `cachedExact`), registry `check` keyed on the receipt, rebuild chain without `--skip-nx-cache` ×4; receipt law + Nx replay oracle **PASS** in the overlay, **red** with the cached producer (mutant); nx-contract policy vectors 34/34 (overlay + live). Lands window 3 (📋️project.json / rebuild JSON are frozen chain inputs) | `generated/nx1b-dry-live-1.txt`, `nx1b-law-1.txt`, `nx1b-law-mutant-1.txt` |
| 2 | ST1 per-family stdio components (88 subsets in `s`) | generator re-run on the live tree (9 families, 158 + 18 apps) + dry run **clean** (18 edits / 36 new / 1 move, 18:3x); overlay proof pending | `generated/st2-gen-1.txt`, `generated/st2-dry-1.txt` |
| 3 | every stdio kind opens/edits/exports in `s` + hub, laws + oracles | window 3 | — |

### Session 14 Log

- 18:3x start. Read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED yet; window 2 closed since
  15:45 → no guest-linked tree edits until WINDOW 3 OPEN), `📓️wp-st1.md`, `📓️wp-cx1.md`, `📓️wp-lb.md`, the A13-w3 rows,
  the session-13 coordinator log from 14:00. CX1's overlay warm build finished 16:08 (`wp-cx1/generated/warm-stdio-2.txt`)
  but started before its 16:00 overlay write → no CX1 law was run; ST1's overlay checks never ran.
- 18:3x dry runs on the live tree: CX1 **0 problems**; ST1 generator + apply **clean**.
- 19:0x coordinator: Codex peer (ticket 26/09/26 COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE) edits stdio; its plan wants all 88
  editors shipped (gate `testEditorCatalogContract`: 36/36 formats + 88 playground rows, red "7/36" on lb-p3's tree). Coordinator
  decision 19:2x: the family packages ARE the resolution (every editor shipped, each component under the 1M-function / rustc
  ceilings); that gate's intent is ST2's acceptance target; never put `full-app-catalog` back into one component; do not edit the
  peer's files while it is active in them; re-diff every target right before landing. ST1's patch already rewrites that gate as the
  union over the 10 stdio packages (36/36 formats, 88 rows, no `[DEBUG]`).
- 19:1x–19:4x overlay: new clone overlay `s14-st2-overlay` (APFS clonefile, `st2-overlay.py`, 22 min under load 100) — then chose
  CX1's overlay path instead (`s13-cx1-overlay`, synced to the live tree 19:4x–20:05: 631 cloned, 46 214 stale removed) because its
  private build-dir `s13-cx1-build` already holds the third-party dependency closure (same absolute path → warm units).
- 20:0x item 1b root cause (read-only, `nx show project` of registry + repo, captures `generated/registry-project-1.json`,
  `repo-project-1.json`): `generate` hashes `dependentTasksOutputFiles` of the receipt `repo:generator-inputs` writes, but
  `generator-inputs` is `cachedExact` in `⚡️caching/🔣️policy.json` with inputs = Cargo.toml/project.json/taxonomy/discovery only,
  while `registryCatalogInputPaths` digests every plugin descriptor `🔣️.json`, example and the implementation TS closure. A describe
  that rewrites a descriptor leaves the key unchanged → stale receipt → stale catalog = chain run 4's "catalog stale".
  Probe `wp-st2/nx-replay-probe.ts` (two throwaway git+Nx 23.2.0 workspaces, producer → gitignored receipt → cached consumer):
  cached producer: initial runs=1 v1 | unchanged runs=1 v1 | **edited runs=1 v1 (stale replay)**; uncached producer: initial 1 v1 |
  unchanged 1 v1 (hit) | **edited runs=2 v2**. Registry `check` is cache-forced by the `check*` family and hashes neither the
  receipt nor descriptors.
- 20:07 CX1 applied to the overlay (`cx1-apply.py --write --root overlay`, 34 files); `st2-cargo.sh` (overlay lane, private
  build/target dirs) queued pid 222: stdio + txt/tsv/html `--lib --tests` → `generated/cx1-test-stdio-1.txt`.
- 20:1x item 1b patch `wp-st2/nx1b-apply.py` + payload (`nx1b-payload/`): `⚡️caching/🔣️policy.json` drops `generator-inputs`
  from `cachedExact`; its target `cache: false` (inputs removed — an uncached producer's inputs mean nothing); registry `check`
  `dependsOn repo:generator-inputs` + `dependentTasksOutputFiles` of the receipt (its descriptor/example view was unkeyed);
  `🔁️rebuild/🔣️.json` drops `--skip-nx-cache` from generate/check/activate-s/verify-s (activate-s keys transitively through
  session-s → generate → receipt and materialize-dev outputs, which the probe shows Nx hashes even when gitignored; verify-s is
  uncached); nx-contract fixture row `generator-inputs` → `authored: true`, Gherkin scenario rewritten ("re-digests on every
  run"); receipt law (`🔏️inputs/🧪️tests/🔏️receipt`): asserts the uncached producer (authored + resolved through the plugin) and
  that `generate` AND `check` key on the receipt, plus the Nx replay oracle `testGeneratorInputReceiptReplay` (fixture section
  `replay`, Ajv-validated; producer = the REAL `publishGeneratorInputReceipt`; cached producer must replay a stale catalog,
  uncached must hit while unchanged and re-run once edited); its `[DEBUG]` log became an emoji result line.
  Proof (overlay, `bun wp-st2/nx1b-law.ts`): **PASS** 65 s (`generated/nx1b-law-1.txt`); mutant (overlay project back to
  `cache: true`): **AssertionError** "the receipt digests bytes its own cache key cannot name…" (`nx1b-law-mutant-1.txt`),
  restored. `bun wp-st2/nx1b-policy-vectors.ts` (the cache-contracts policy loop): 34/34 overlay and live.
