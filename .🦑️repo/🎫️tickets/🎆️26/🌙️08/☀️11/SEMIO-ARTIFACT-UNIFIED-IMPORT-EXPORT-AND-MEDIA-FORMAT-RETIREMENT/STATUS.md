# STATUS

Append-only real-state log for ticket `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`.

## 2026-08-11 — W2b closer

**Outcome: W2b CLOSED.** Fixed all verifier-flagged compile bugs across the 7 W2b subsets
(document, image, video, audio, animation, presentation, workflow), discovered and fixed 2
additional test-fixture bugs (workflow, animation) found only once the crate finally compiled,
built the `✳️any` envelope subset end-to-end (real `SemioSnapshot`/`SemioDiff`/`SemioMutation`
tagged unions over all 13 domain subsets, real `ArtifactComposer` + `SubsetValidator`, all 8
laws), and burned down 8 `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entries in `📜️script.ts`.

- `cargo test -p semio-s-plugin-stdio --lib`: **1483 passed, 14 failed** (crate now compiles —
  it did not before this closer's session). All 14 remaining failures are OUTSIDE W2b's scope:
  5 in `brep`/`mesh`/`model` (W2a subsets — real bugs, discovered only now that the crate
  compiles, left untouched per this ticket's write-scope rules) and 9 in `csv`/`json` (unrelated
  format artifacts, foreign to this ticket entirely — pre-existing). Zero failures remain in
  `document`/`image`/`video`/`audio`/`animation`/`presentation`/`workflow`/`✳️any`.
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** (verifier's own
  snapshot immediately prior was 21523 — net +1, ordinary concurrent-wave churn, not from this
  session's edits: this session's own 7 subsets + `✳️any` each carry exactly 2 breaches, the same
  pre-existing sanctioned patterns every other real subset in this program carries, with `image`
  at 13 — a known structural outlier flagged by the verifier, documented as a follow-up below, not
  fixed this session).
- W2a status: **no `w2a-close-report.md` exists yet** — W2a has not formally closed. Direct code
  inspection (not just report-reading) confirmed all 6 W2a subset snapshot types (brep, mesh,
  model, object, cad, drawing) are real, substantial, non-scaffold implementations (5 have their
  own `w2a-<subset>-report.md`; `cad` has none but its code is fully real, matching every other
  complete subset's shape) — the `✳️any` envelope was built against these real types, NOT faked.
  This should be independently reconfirmed once `w2a-close-report.md` lands.

See `w2b-close-report.md` for full detail.

## 2026-08-11 — W3 closer

**Outcome: W3 CLOSED.** Fixed all 4 verifier-flagged compile bugs across the 4 W3 sub-agents'
scopes (mp4+avi, mp3+wav, epw+tsv, html — 7 new format artifacts), unblocking crate-wide compile
for the first time in this ticket's W3 phase. Once tests could actually run, discovered and fixed
9 more real bugs (mp4 sample-entry encoder byte-count, 2 mp4/avi/mp3 test-fixture bugs, wav's
internally-tagged-enum serde limitation, wav's fixture-rounding mismatch, epw/tsv's
impossible-simultaneous-removed-and-added test assertions) — none design-judgment, all mechanical
and self-contained within their own artifact's tree. Removed 10 now-satisfied
W1b-seeded shrink-only allowlist entries from `📜️script.ts` (5×`POLICY_DIFF_COMPLETENESS_ALLOWLIST`,
5×`POLICY_ROUND_TRIP_TEST_ALLOWLIST`), verifying each before removal and confirming zero regression
via a before/after policy diff. Spot-checked `catalog.json`'s `depends` arrays for all 7
artifacts — accurate, no fix needed (mp4/avi/mp3/wav → `binary`; epw/tsv/html → `txt`; confirmed via
grep for cross-artifact imports and `DEP_` composer dialects, none found beyond the primitives).

- `cargo test -p semio-s-plugin-stdio --lib`: **1484 passed, 13 failed**. All 13 failures are
  OUTSIDE W3's scope (4 `csv`, 5 `json`, 4 `semio` `mesh`/`model` — confirmed foreign via
  `git status --porcelain`, stable across two consecutive runs). **Zero failures anywhere in
  mp4/avi/mp3/wav/epw/tsv/html** — each artifact's own scoped test suite (129 tests total across
  the 7) passes 100% clean (25/19/17/16/13/14/25).
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** — byte-identical to
  the pre-allowlist-edit snapshot, confirming the 10 removed entries introduced zero regressions.
- Two allowlist entries deliberately kept (not stale): mp4/avi's `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
  rows (both artifacts genuinely lack a `DiffCodec` impl for their Diff type — real open gap, see
  follow-up in `w3-close-report.md` §7) and html's `POLICY_ROUND_TRIP_TEST_ALLOWLIST` row (html's
  real round-trip test lives in its snapshot file, not its engine file — a rule/architecture
  mismatch, also documented as a follow-up).

See `w3-close-report.md` for full detail.

## 2026-08-11 — W2a closer

**Outcome: W2a CLOSED.** object/cad/drawing were already clean per `w2a-verify-report.md`. Fixed
all 3 real bugs the verifier found in brep/mesh/model, plus a 4th latent bug (mesh mutation-level
`RemoveX` inverse losing position) exposed only once the diff-level fix landed:

- **brep**: `field_sweep` test-fixture bug (`sweep_b`'s `e1.end_vertex` didn't actually differ from
  `sweep_a`'s) — fixed the fixture.
- **mesh**: `NamedTripleDiff.added` had no positional fidelity (same root cause `object` already
  found and fixed in its own subset) — ported `object`'s local `NamedAdded<T>{index,item}` wrapper
  fix into mesh's diff engine, switched `DiffAlgebra::inverse` to the generic
  `mid=apply(base); between(mid,base)` derivation, and fixed 4 `RemoveMesh`/`RemovePrimitive`/
  `RemoveMaterial`/`RemoveTexture` mutation-level inverses to preserve original position (same
  remove-tail/re-add technique `object`'s own `RemoveMapEntry` inverse uses).
- **model**: `op_text_binary_roundtrip_law` double-`Option` serde bug on `SetElement.spatial_id`
  (verifier-confirmed) and `SetSpatialNode.parent_id` (same shape, fixed proactively) — standard
  `skip_serializing_if` + `deserialize_with` workaround.
- **cad**: backfilled the missing `w2a-cad-report.md` from direct inspection (code itself needed no
  changes — already real and passing).

Removed all 6 of W2a's now-satisfied `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entries from
`📜️script.ts` (brep/cad/drawing/mesh/model/object), each verified to carry a real
`impl protocol::DiffCodec for Semio<X>Diff` before removal, re-run confirmed zero regression.

- `cargo test -p semio-s-plugin-stdio --lib`: **1491 passed, 6 failed** (up from W2b/W3's 1483-1484
  passed / 13-14 failed — the 5 semio failures W2b's closer flagged as "outside scope, W2a's to
  fix" are now gone). All 6 remaining failures are `csv`/`json` standards engines — entirely
  outside this ticket. **Zero failures anywhere in `artifacts::semio::standards::v1::subsets::
  {brep,mesh,model,object,cad,drawing}`** (111 tests across the 6 scoped runs: 17/21/15/32/13/13,
  all green).
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** — byte-identical
  total to the verifier's own snapshot immediately prior; direct inspection of
  `.🦑️repo/⚡️cache/breaches/compose.json` confirms zero new breaches for any of the 6 subsets and
  zero `dsl-migration/diff-completeness` breaches remaining for them (the 6 allowlist removals were
  clean).
- Deferred (real design/policy-authoring judgment, not fixed here): grammar-honesty allowlisting for
  binary leaves, facet-mirror-drift allowlisting for `🔺️diff` facets, and the shared
  `⚙️engine/🧰️triples::NamedTripleDiff<K,D,T>` gaps (spurious `T: Default` bound + no built-in
  positional `added` wrapper) that brep/mesh/model/object each independently worked around locally —
  all documented as follow-ups in `w2a-close-report.md` §5, none touched (editing shared engine
  files or repo-wide allowlist policy is outside a single closer's scoped write access this session).

See `w2a-close-report.md` for full detail.

## 2026-08-11 — W4 closer

**Outcome: W4 CLOSED.** 26 real format-pair bridges (52 leaf files: brep↔step; mesh↔gltf/stl/obj/
ply/las; model↔ifc/bcf; object↔json/xml/csv; cad↔step (+dxf/dwg per G4); drawing↔svg/dxf/pdf;
image↔png/jpg/gif/bmp/tiff; video↔mp4/avi; audio↔mp3/wav; animation↔gltf/mp4/gif;
document↔docx/md/txt/pdf; presentation↔pptx; workflow↔json) delivered across 6 parallel groups, all
confirmed real/honestly-documented/zero-codec-reimplementation by `w4-verify-report.md`'s 12-pair
sample (50% over the required minimum). Real (lossless-in-scope) mappings: object↔json,
workflow↔json (both genuinely lossless), audio↔wav (byte-exact PCM), mesh↔ply (real indexed
round-trip). Documented-lossy (real mapping, honest gaps, never fabricated): brep↔step (ref_direction
rotation, same_sense), model↔ifc (geometry/name fields out of scope), model↔bcf (issue-tracker vs.
spatial — narrow by design), mesh↔gltf/obj/stl/las (index-structure/material/format-cardinality
losses), drawing/document↔pdf (flat structure, page boundaries only), presentation↔pptx (media/table
shapes), video↔mp4/avi, audio↔mp3 (decode-only, no encoder — zero codec reimplementation), animation↔
gltf/mp4/gif (tangent/cardinality losses) — every loss itemized in-file, never silently defaulted.

Fixed the one real bug the verifier found (§2 of `w4-verify-report.md`): `pdf`'s shared
`⚙️engine/🦀️component.rs::extract_text` never emitted a newline for the `T*` (move-to-next-line)
content-stream operator, so multi-line text written by `encode_pdf` (which emits `T*` between every
line's `Tj`) came back joined with no separator on decode — a spec-correct one-arm fix
(`"T*" if in_text => out.push('\n')`, PDF32000-1 §9.4.2), not specific to any one leaf. Fixed the
shared-infra gap flagged since W2 in `⚙️engine/🧰️triples/🦀️component.rs`: added the
`#[serde(bound(...))]` override bcf's own local copy already used (stops a spurious `T: Default`
bound on `IndexedTripleDiff`/`NamedTripleDiff`) and hoisted `object`'s local `NamedAdded<T>
{index,item}` positional-fidelity wrapper (+ generic `enc_named_added`/`dec_named_added` codec
helpers) into the shared file as the new canonical copy — the 5 subsets with local workarounds
(bcf, brep, mesh, model, object) were deliberately left untouched (still correct), only future W4/W5
consumers benefit. Checked `📜️script.ts` for now-satisfied shrink-only allowlist entries tied to
io-leaf coverage/composer-dependency: verified there were none to remove (both rules are fully
computed, not allowlist-gated, and already show zero breaches; no allowlist anywhere names any of
W4's 12 subsets or their format pairs) — no speculative edits made.

- `cargo test -p semio-s-plugin-stdio --lib`: **1657 passed, 0 failed, 1 ignored** — fully green
  (up from the verifier's 1645/10-failed snapshot: the pdf fix cleared the 1 real failure, and the 9
  foreign png/zip conformance-law failures the verifier saw from a different concurrent session's
  in-progress work were already resolved by that session before this closer's own gate run).
- `bun ./📜️script.ts policy`: **21532 high-priority breaches across 25 rules** — byte-identical to
  the verifier's own snapshot, confirming both fixes introduced zero new breaches and zero
  regressions.
- Not fixed, documented as a follow-up for W5: **G4 (drawing/cad/image) never filed its required
  `w4-*-report.md`** (a real CLAUDE.md process gap; its underlying code is real and substantial per
  direct inspection by both the verifier and this closer, so nothing to fix there beyond the missing
  paper trail — not backfilled here, out of a closer's scope to author another group's first-person
  report).

See `w4-close-report.md` for full detail.

## 2026-08-12 — W5a closer

**Outcome: W5a's 2 real blockers fixed; underlying wave's engineering confirmed real.** The
verifier (`w5a-verify-report.md`) found 2 of 7 plugin crates (architect: 19 errors, remodel: 4
errors) did not compile, and 4 of 7 agents (norm, energy, architect, remodel) filed no required
`w5a-*-report.md`. This closer fixed both compile blockers — same-crate, mechanical, no-stdio-edit
"lagging call-site" fixes matching the pattern fem/animate/norm already used successfully this
wave — and re-ran the full gate. Full detail in `w5a-close-report.md`.

**architect (19 → 0 errors)**: a pre-existing repo-wide "document → artifact" rename
(`ProgramSnapshot.documents` → `.artifacts`, commit `c31024cc6c`, predates this ticket) was never
propagated to architect's own `ProgramArtifact` struct or ~10 call sites (mutations/diff/search/
validate/status-summary/catalog) — architect's own W5a agent fixed only 1 of ~11 instances. Fixed
all of them, plus a second independent instance of the same lag in the bundled DSL example fixture
(`document-refs` column → `artifact-refs`, top-level `documents` table → `artifacts`), plus the
`JsonSnapshot.value`/`CsvSnapshot.headers`+`.rows` stdio-schema-drift breaks in architect's own
csv/json leaves (same fix pattern animate/fem used). `cargo check`: 0 errors, 81 warnings. `cargo
test`: 248 passed, 0 failed (fixture parse failures the rename initially surfaced are also fixed).

**remodel (4 → 0 errors)**: `FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL` → `_ARTIFACT_ID`/`_LABEL`
(the same framework-rename lag animate's report flagged as affecting ~18 plugins; only remodel's
was fixed here) + the same `JsonSnapshot.value` stdio-schema-drift fix in remodel's json leaves.
`cargo check`: 0 errors, 10 warnings. `cargo test`: **360 passed, 2 failed — 2 real, newly-surfaced
regressions**, not fixed (design-judgment, see below):
- `jpeg_decode_never_panics_on_truncated_input` — stdio's `decode_jpg` is measurably more lenient
  with truncated input than the hand-rolled decoder it replaced (decodes instead of erroring at
  several truncation points).
- `reconstruction::long::video_in_yields_watertight_mesh_out` — the full synthetic-video →
  photogrammetry → mesh pipeline reaches `Done` but now yields 0 triangles, likely downstream of
  the mp4 `decode_mp4`/`probe` rewiring changing frame-extraction behavior.

**LOC deleted per plugin** (ad-hoc byte-encoding/subprocess/tokenizer code retired; net diff line
counts vary in sign because real domain/error-handling logic replacing a deleted subprocess call or
hand-rolled parser is often longer than what it replaced — this is expected, not a red flag):
- **cad**: -2407 net (389 ins / 2796 del, 32 files) — byte-reinterpret placeholder exporters (stl/
  ifc/obj/png/json/gltf/step, 14 files) + 11 orphaned JSON Schema files + a 475-line TS STEP-import
  helper cluster + a 173-line TS STEP-writer region, all deleted; real `semio/mesh`+`semio/brep`
  bridging built in their place. *(live numstat on cad's tree is currently unstable from an
  unrelated concurrent session — see follow-ups below; -2407 is cad's own agent-verified figure.)*
- **norm**: -6645 net (525 ins / 7170 del) — 150 fabricated leaf files (json/csv/txt/xlsx/zip ×
  import/export × 10 standard subsets) deleted outright, none replaced (no honest mapping exists
  for norm's tree-shaped standard-lookup data).
- **remodel**: -1966 net (446 ins / 2412 del) — a genuine hand-rolled H.264/AVC bitstream encoder +
  ISO-BMFF/RIFF box muxer (5163-line video engine) + PNG/JPEG byte encoders (962→49 lines), all
  rewired through stdio's real mp4/avi/png/jpg engines.
- **fem**: 280 LOC deleted (16 leaf files: obj/stl import + zip/png both directions, both 2D/3D
  shapes) — obj/stl export direction rewired through a new real mesh bridge into stdio's real
  codecs; import/zip/png deleted with no honest replacement (documented per-pair in fem's report).
- **animate**: FFmpeg subprocess path (`Command::new("ffmpeg")`, `run_ffmpeg`/`concat_partials`/
  `mux_audio_track`) deleted outright and rewired onto stdio's real in-process `encode_mp4`/
  `encode_gif`; net whole-plugin diff is **+354** (470 ins / 116 del) because real in-process codec
  logic (raw-frame mp4 muxing, gif quantization/scaling) is inherently longer than the subprocess
  call it replaced — zero ad-hoc container-byte-encoding remains.
- **energy**: `EpwWeather::parse`'s hand-rolled, silently-defaulting CSV split deleted, rewired onto
  stdio's real, lossless `decode_epw`; net **+56** (133 ins / 77 del), same reason as animate.
- **architect**: hand-rolled CSV/TSV tokenizer (`write_delimited`/`parse_delimited`/`parse_record`/
  `escape_field`) deleted, rewired through stdio's real csv/tsv engines with new round-trip tests;
  net **+85** (225 ins / 140 del, includes this closer's fixes).

**What's now real vs. what stdio gaps remain** — 12-item consolidated `stdio_gaps` list in
`w5a-close-report.md` §3 (cad→ifc/png have no bridge target at all; `CadComposer::compose()` has no
real per-format codec for any of its 8 dialects; mp4 has no real pixel encoder, only container
framing; no audio-track schema slot in `Mp4Snapshot`; no `DwgDrawing`→`semio/drawing` bridge; **4 of
7 agents independently hand-rolled the same `serde_json::Value`↔stdio's `JsonValue` converter** —
recommend stdio grow one canonical bridge function; remodel's 16-bit grayscale PNG stays
hand-rolled, documented, narrow). Everywhere else: the extraction/rewiring work is real, honestly
documented in-code, and zero fabricated codecs or ad-hoc byte reinterpretation remain across any of
the 7 plugins post-fix (reconfirmed by this closer, matching the verifier's own independent read).

**Final gate** (raw logs: `w5a-close-*.txt`): `cargo check -p semio-s-plugin-stdio --lib` — 0
errors (unchanged/still green). `cargo test -p semio-s-plugin-stdio --lib` — 1866 passed, 4-5
failed, all inside `semio::standards::v1::subsets::{brep,mesh,model,object,drawing}`, a **large,
live, unrelated, in-progress foreign wave** (220 unstaged files, literal
`"PLACEHOLDER_WILL_BE_REGENERATED..."` text still in some fixtures) whose compile/test status
oscillated 3 times across this closer's session, self-resolving each time — not W5a's, flagged for
the orchestrator to re-check once it lands. `bun ./📜️script.ts policy` — 21654 breaches/26 rules
(verifier's snapshot: 21609/25); the +1 rule (`mutation-migration/semantic-vocabulary`, 15
breaches) is 100% inside `gis` (W5b's scope, confirmed via grep) — every W5a plugin's own breach
count is byte-identical to the verifier's snapshot except cad (29→48, from the same unrelated
concurrent session touching cad's tree noted above). **All 7 plugin crates compile clean (0 errors
each)** in a consistent same-session snapshot: remodel, cad, animate, energy, architect, fem, norm.

Not fixed, documented as follow-ups in `w5a-close-report.md` §4: remodel's 2 test regressions
(design judgment — stdio codec-leniency change and a reconstruction-pipeline behavior change, both
need investigation beyond a closer's cheap-fix scope); fem's 8 pre-existing protocol-fixture test
failures (grammar identifier can't start with a digit, pre-migration defect, already flagged by
fem's own report for W8 routing); norm's and energy's missing `w5a-*-report.md` (their real
findings are recovered into the consolidated `stdio_gaps` list instead of backfilling a first-person
report neither agent wrote); cad's tree under live unrelated concurrent edit throughout this
closer's session.

## 2026-08-12 — W5b closer

**Outcome: 7 of 8 plugins now fully green (compile + test, 0 failures); 1 (layout) blocked on a
real, documented, out-of-mechanical-reach stdio schema gap.** The verifier (`w5b-verify-report.md`,
verdict **FAIL**) found 2/8 agents' reports permanently lost to a filename collision (🎥️shooting,
🖨️raster — recovered from diff/source directly, no first-person account exists), 3/8 crates failing
to compile for real plugin-owned reasons (note, shooting, raster), 1/8 compiling but 2 tests failing
(draw), and 3/8 foreign-blocked (layout, procedural, puzzle) by then-live framework/inference/
dsl-derive churn. This closer fixed all cheap/mechanical issues, left the one genuine design-judgment
gap documented, and re-ran the full gate fresh.

**12 mechanical fixes applied** (full detail + file list in `w5b-close-report.md` §1): note's
`STDIO_JSON_DOCUMENT_SCHEMA` wrong import path (2 files); shooting's and raster's stale
`panels::document`→`artifact` `#[path]` glue.rs mounts (the same one-line fix every other W5b
plugin's own agent had already independently applied — shooting/raster just never got a
report/closer to do it); shooting's, raster's, procedural's (2d+3d), and layout's `JsonSnapshot.value`
(`serde_json::Value`→stdio's own `JsonValue`) schema-drift fallout in their json io leaves (10 files
total, mirroring the identical converter pattern note/gis had already established); raster's
`FRAMEWORK_PANEL_TAB_DOCUMENT_ID`→`_ARTIFACT_ID` stale constant; draw's and raster's missing
`Once`-guarded stdio composer registration (the verifier's §6b finding for draw — `io_dispatch` had
no registry entry in a bare `cargo test` process — turned out to also affect raster, not caught by
the verifier only because raster never compiled far enough to be tested); a **new finding, not in
any report**: raster's SVG bridge used `ArtifactDsl::print_dsl` (envelope-wrapped) instead of
`write_svg_xml` (bare XML), which broke once composer registration let raster's DWG-import path
actually feed that string into a real XML parser (`"unknown token at 1:1"`) — switched to
`write_svg_xml`, matching note's/draw's own usage of the same bridge; layout's missing
`DwgSnapshot.codepage`/`.maintenance_version` fields, added as honest `0` (consistent with that
leaf's already-synthetic, `SentinelOnly` DWG output).

**Not fixed, documented as a follow-up** (`w5b-close-report.md` §2): layout's pdf io leaves — stdio's
`PdfSnapshot` was restructured from `page: PageDoc{width,height,text}` to `pages: Vec<PdfPage>`, a
real multi-page model change, not a mechanical rename. `LayoutSnapshot`'s own single-flat-page-list
shape needs a genuine design decision for how it maps onto `Vec<PdfPage>` — left to a future session,
matching layout's own report's assessment.

**Consolidated `stdio_gaps`** (`w5b-close-report.md` §3, 5 items deduplicated across all 8 plugins):
(1) no `s.stdio.semio/v1/drawing ↔ dwg` bridge (note/layout/gis/draw, architecturally expected per
the master plan's lattice — dwg pairs with the separate `cad` subset instead); (2) stdio's
`JsonSnapshot.value` retype shipped with zero conversion helpers, forcing 7 of 8 plugins to each
hand-write the identical recursive converter — recommend stdio export one shared helper;
(3) `DrawNode::Text` has no font-size/font-weight field (note, draw); (4) `DrawStyle` has no
blend_mode/fill_rule, `Group`/`Image` nodes have no opacity slot (draw); (5) stdio's `PdfSnapshot`
multi-page restructure has no migration path (layout, see follow-up above).

**Final gate** (all commands re-run fresh; raw logs `w5b-close-*.txt`). The repo hit **two separate
live concurrent-churn incidents** during this gate — stdio's `brep` subset (grammar/protocol/spicy
regeneration, resolved after ~9 min polling) and stdio's `semio/drawing` binary reader (a method
renamed mid-edit, resolved after ~2 min polling) — both confirmed foreign via `git status`
dirtiness, polled rather than chased. Numbers below are the final, settled state:

- `cargo check -p semio-s-plugin-stdio --lib`: **0 errors.**
- `cargo test -p semio-s-plugin-stdio --lib`: **1869 passed, 0 failed, 3 ignored** (verifier's
  mid-churn run: 1839/5/4 — the 5 failures were themselves foreign-churn artifacts, since resolved;
  W4-closer baseline was 1657/0/1).
- `bun ./📜️script.ts policy`: **21651 high-priority breaches across 26 rules** (W4 baseline: 21532;
  verifier's mid-churn run: 21610/25). Grepped every one of this closer's 12 edited files against
  the full breach list — **zero breaches attributable to any of them**; the ~119-breach drift from
  baseline traces to ongoing concurrent repo-wide churn (two separate live foreign edits observed in
  under 20 minutes of this session alone), not to W5b.
- Per-plugin `cargo check -p <crate>`: **7/8 PASS** (note, gis, shooting, procedural, raster, draw,
  puzzle — puzzle's foreign `dsl_derive`/`os_spr`/`os_store` blocker the verifier saw has since
  quiesced). **1/8 FAIL** (layout — 3 errors, all in the pdf leaf, genuine design gap above).
- Per-plugin `cargo test -p <crate> --lib`, all 7 compiling crates: note 71/0, gis 155/0, shooting
  92/0, procedural 191/0, raster 55/0, draw 88/0 (including both tests the verifier caught failing,
  now passing), puzzle 421/0 — **zero failures anywhere.**

See `w5b-close-report.md` for full detail, all 12 files touched, and raw log filenames.

## 2026-08-12 — W6/V7 closer (MediaFormat/ArtifactCodec deletion — FINAL)

**Outcome: W6 CLOSED. `MediaFormat`/`ArtifactCodec` retirement (V7) is DONE.** The independent
verifier (`w6-verify-report.md`) returned **PASS** with only 2 non-blocking report-wording nits
(mesh module's net-line count, one foreign-error crate misattribution) — no FAIL items, nothing
cheap/safe to fix, so this closer made **zero code changes** and re-ran the full definitive gate
fresh from disk to confirm.

**Scope migrated**: 32 files carried `MediaFormat` text at W6 start (per `w6-census-report.md`) —
10 framework/OS files (definition site + 9 call-site files) and 22 plugin files across 12 plugin
crates (remodel, raster, process, cad, stdio, animate, space, gis, shooting, layout, draw,
lowpoly). All 32 confirmed at 0 hits by the delete-report's exit gate, the verifier, and this
closer's independent re-run.

**Framework code deleted**: the `MediaFormat` enum, the `ArtifactCodec<T>` trait + 20 concrete
codec impls, `StdioFormatEntry`/`STDIO_FORMAT_CATALOG` + 4 lookup fns, and 7 neutral document-model
types (`RasterImage`/`PageDoc`/`TableDoc`/`TextDoc`/`Archive`/etc.) were deleted outright from
`🔺️mesh/🦀️component.rs` — **net −1037 lines** in that one file (verifier's own `git diff --stat`,
correcting the delete-report's "−1105" which was total diff churn not net). 13 other
framework/OS/plugin files were rewired (type-signature-level: `MediaFormat`-typed params/fields →
string-kind (`format_kind()`/`FormatDescriptor`) equivalents), not bulk-deleted. Deliberately kept
(flagged, not a gap): `MeshExporter`/`MeshImporter` traits (real consumers in 9+ plugins never
touched by W5's `MediaFormat`-grep-based census) and the hand-rolled DWG codec (~1226 LOC, 19 real
external consumers including load-bearing OS 2D-export infra) — both renamed
`format()` → `format_kind()` only, zero-touch for existing call sites.

**Final gate, re-run fresh this session** (raw logs: `w6-close-workspace-check.txt`,
`w6-close-stdio-test.txt`, `w6-close-policy.txt`):
- `grep -rn "MediaFormat" --include="*.rs" ✏️s 🧰️framework | grep -v "🎫️tickets" | wc -l` → **0**.
- `cargo check --workspace --keep-going` → 118 error lines, **0 mention MediaFormat**. Breakdown:
  57 `semio-framework-os-kernel-db` (foreign `db_*` module cascade), 22 `semio-compose-rs` (foreign
  `dsl`/`vcs` unresolved), 14 `semio-framework-os --features os-host-full` (pre-existing
  duplicate-field merge artifact, default features clean), 10 stale `#[path]` panel breakages
  across `block`/`dag`/`forms`/`imperative`/`mathematical`/`reasoning-mindmap`/`sequence`/
  `sourcing`/`vcs`/`flow` (unrelated `📄️document`→`📄️artifact` rename fallout), 3
  `semio-s-plugin-playbook` (foreign live stdio `JsonValue`/`Value` churn — same class the
  delete-report saw on `process` instead). **Zero MediaFormat-attributable errors anywhere.**
- `cargo test -p semio-s-plugin-stdio --lib` → **1930 passed; 0 failed; 3 ignored** — exact match
  to the delete-report and verifier.
- `bun ./📜️script.ts policy` → **21654 high-priority breaches across 26 rules**, **0** attributable
  to MediaFormat (grepped the full breach listing).

**Follow-ups documented, not fixed** (all pre-existing/foreign, none block V7 acceptance — full
detail in `w6-close-report.md`): (1) `AppIo` lacks the string-kind-id peer field
(`export_stdio_kinds`/`import_stdio_kinds`) that `ArtifactKindSpec` already gained this wave — 10
plugins' `AppIo.{export,import}_formats` are now dead `vec![]`, harmless but asymmetric, flagged
for a future framework session; (2) the 10-plugin stale `#[path]` panel breakage above; (3)
`semio-framework-os --features os-host-full`'s 14 pre-existing errors; (4)
`semio-framework-os-kernel-db`/`semio-compose-rs`'s 79 combined foreign errors, unrelated crates
under live concurrent development; (5) stdio's `JsonValue`/`Value` deserializer mismatch,
documented across multiple W5b/W6 reports as needing one shared converter function.

See `w6-close-report.md` for full detail and raw log filenames.

## 2026-08-12 — W7 closer (os-run fix + cross-plugin IoRouter test)

**Outcome: W7 closed to the extent cheap/safe; one design-scope follow-up left open.** No
`w7-report.md` existed at hand-off (per `w7-verify-report.md`'s FAIL) despite real, substantial,
uncommitted W7 code already sitting in the tree — the RunArtifact `ArtifactDsl`/`ArtifactPack`
codec fix in `🔁️workflow/🦀️component.rs` and a new, genuinely real cross-plugin
`io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` test in
`🔌️plugin/🖥️host/🦀️component.rs`. This closer wrote the missing `w7-report.md` retroactively
(documenting only claims already independently verified), then fixed the two cheap/safe issues the
verifier's own `cargo check` output flagged: an unused `dsl_core` extern-crate alias in
`🏃️run/📦️packages/🦀️rust/📦️glue.rs` (confirmed dead in this crate specifically, real/used
elsewhere) and a dead-code warning on `run_fault_bytes` in `🏃️run/🦀️component.rs` (confirmed
test-only via all 5 call sites, fixed with `#[cfg(test)]`, zero behavior change).

**Not fixed — design-scope, follow-up opened**: `ArtifactStore<P, Mutation>::projection_json` has
no definition anywhere in the repo despite ~10+ plugins' wasm-binding files calling it (cad, jack,
raster, process, writer, gis, shooting, puzzle, animate/present, trinity/rewrite). This blocks
building any second real `.wasm` plugin component next to stdio's, which is exactly what the new
cross-plugin `IoRouter` test needs to exercise its real routing assertions instead of hitting its
(intentional, convention-matching) silent-skip guard. The master plan's W7 gate ("wasm builds
succeed") is therefore **not met** — recommend a dedicated follow-up ticket for
`projection_json` before declaring W7 fully done.

**Final gate, re-run fresh this session** (raw logs: `w7-close-osrun-check.txt`,
`w7-close-osrun-test.txt`, `w7-close-stdio-check.txt`, `w7-close-stdio-test.txt`,
`w7-close-policy.txt`):
- `cargo check -p semio-framework-os-run` → **0 errors, 0 own-crate warnings** (down from 2
  own-crate warnings before this session's two fixes).
- `cargo test -p semio-framework-os-run --lib` → **15 passed, 0 failed** (same pre-existing
  `run_lib` suite; unaffected by the fixes).
- `cargo check -p semio-s-plugin-stdio --lib` → 0 errors (493 pre-existing warnings, untouched,
  out of scope).
- `cargo test -p semio-s-plugin-stdio --lib` → **1930 passed, 0 failed, 3 ignored** — exact match
  to W6's close.
- `bun ./📜️script.ts policy` → **21654 high-priority breaches across 26 rules** — byte-for-byte
  the same count as W6's close gate, confirming this session's two-line fix introduced zero new
  breaches.

See `w7-report.md` (retroactive implementation report) and `w7-close-report.md` (this closer's
full detail, including the inherited FAIL's 5 points and their disposition) for full detail.

## 2026-08-12 — W7-fix (projection_json lagging rename)

**Outcome: CLOSED.** W7's remaining blocker — cad's wasm build failing, which silently made the
new cross-plugin `IoRouter` compose test take its guard-skip path instead of really routing —
was a pre-existing, repo-wide, unrelated lagging rename: `ArtifactStore::projection_json` was
renamed to `snapshot_json` at some point, but 9 plugins' wasm-bindgen bridge files (jack, rewrite,
raster, process, cad, writer, animate/present, gis, shooting) never updated their internal call
site. Fixed all 9 (outer JS-facing `pub fn projection_json` wrapper names left untouched — only
the inner `ArtifactStore` method call changed). Rebuilt cad's wasm (4.78MB, deterministic).
Re-ran `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` — confirmed
via both wasm files' on-disk presence and direct test-source reading that the real routing path
executed (2-plugin registration, real cross-instance compose, byte-identical round trip), not the
silent-skip guard. Independently re-verified PASS. `cargo test -p semio-s-plugin-stdio --lib`:
1930 passed, 0 failed. No regressions in any of the 9 touched plugin crates or the workspace.

See `w7fix-report.md`/`w7fix-verify-report.md` for full detail.

## 2026-08-12 — W8 final gate

**Outcome: READY TO CLOSE.** Built/confirmed real end-to-end tests for scenarios (a) cad→semio/brep→
step→reimport→semio/brep→semio/mesh→gltf (bounding-box geometry equivalence proven through decoded
gltf bytes), (b) draw→semio/drawing→svg (real, re-parses; dwg direction confirmed as a genuine,
correctly-documented capability gap — no drawing↔dwg bridge exists in stdio by design, dwg only
reaches through the separate cad/ac1024 subset), (c) animate→semio/video→mp4 (real box-walk/
track/duration invariant assertions against a genuinely decodable mp4). Scenario (d) — native
cross-plugin IoRouter compose — already proven in W7-fix. Audited every shrink-only allowlist this
ticket seeded: zero entries were removable (all remaining ones — mp4/avi's missing DiffCodec,
html's round-trip test file location — are real, still-open, correctly documented gaps, not
oversights). Policy: 21654/26 rules vs W0's 21564/24 baseline, net +90 — NOT literally zero
(a documented deviation from the plan's literal wording), but fully explained: a real −180 from
W1's schema-representation generalization, real diff-completeness burn-down across every wave,
netted against the expected inherent breach classes every new schema-owning subset/artifact
carries (taxonomy/emoji-prefix on `📄set-snapshot` dirs, os-state-authority/item-scope-global on
composer `OnceLock`s — present on every real subset in the repo, not unique to this program) plus
one rule (`artifact-io/io-matrix-migrated`) deliberately deferred until W4 wired semio's owner row.
Full workspace check: 14 failing crates, all independently confirmed foreign via git log/status
(dated before this ticket opened, or actively mid-edit by other live sessions). Fresh-eyes final
verifier re-ran every gate from scratch, re-read every named "known gap" against the live tree,
found nothing inflated, nothing silently fixed-and-unreported, nothing silently worse than
documented. Verdict: **READY TO CLOSE.**

See `w8-scenarios-report.md`, `w8-audit-report.md`, `w8-final-verify-report.md` for full detail.
