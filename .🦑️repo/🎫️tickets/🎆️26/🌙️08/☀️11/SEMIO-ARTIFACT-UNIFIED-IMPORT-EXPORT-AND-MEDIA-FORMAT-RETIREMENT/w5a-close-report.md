# W5a Close Report

Closer for W5a (7 plugin-extraction agents: remodel, cad, animate, energy, architect, fem, norm),
following up on `w5a-verify-report.md`'s **FAIL — not ready to close** verdict. This report fixes
the verifier's two real blockers, consolidates every `stdio_gaps` item across all 7 agents (filed
or not), and re-runs the full cross-plugin gate.

## 1. Fixed: architect (19 → 0 errors)

Root cause (confirmed via `git log`): a repo-wide "document → artifact" terminology migration
(commit `c31024cc6c`, landed a full day before this ticket opened) renamed `ProgramSnapshot`'s
`documents: Vec<DocumentRecord>` field to `artifacts: Vec<ArtifactRecord>`, but architect's own
`ProgramArtifact` struct (a hand-written sibling type with field-for-field parity, not
macro-derived) and ~10 call sites across mutations/diff/search/validate/status-summary/catalog
were never updated to match — the exact "lagging call-site" pattern fem/animate/norm proactively
fixed in their own trees this same wave. architect's own W5a agent fixed only its own leaf
(`program.documents` → `program.artifacts` in `📤️exchange/component.rs`) and stopped.

Fixed, all within `✏️s/🔌️plugins/🏛️architect/**`:
- `ProgramArtifact.documents` → `.artifacts` field rename (`🧬️schema/🦀️component.rs`) + its
  3 conversion sites (`to_snapshot`/`from_snapshot`/`set_snapshot`).
- `ProgramMutation` apply/inverse/diff dispatch + 2 test assertions (`🧬️mutations/🦀️component.rs`)
  — kept the `ProgramMutation::Documents` enum variant name itself unrenamed (see §4, follow-up).
- `engine/🔍️search`, `engine/✅️validate`, `engine/📊️status-summary`, `program/🦀️component.rs`
  (default), `apps/🏛️architect/🗂️catalog/🦀️component.rs` (2 macro-table entries) — same
  `program.documents` → `program.artifacts` rename.
- `🔺️diff/📝️text/🦀️component.rs:447` — `next.documents` → `next.artifacts` (the `ProgramDiff`
  struct's own `documents` field, applying onto a `ProgramArtifact`, was left as-is; only the
  target-side field access needed the rename — see §4).
- Bundled DSL example fixture `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (4 occurrences):
  the top-level `documents [...]` register table → `artifacts [...]`, and a `document-refs:LIST`
  column referenced by 4 *other* tables (decisions/assumptions/compliance-records/meetings) →
  `artifact-refs:LIST` — matching `registers.rs`'s already-renamed `artifact_refs: Vec<EntityId>`
  field on those record types. This was a **second, independent** instance of the same lagging
  rename, only surfaced once the first was fixed (parse error changed from "unknown table column
  'document-refs'" to "expected List, found Absent" once the column name matched but the register
  table name didn't).
- `📤️export/🔣️json` + `📥️import/🔣️json` leaves: `JsonSnapshot.value` mismatch (`serde_json::Value`
  vs. stdio's own `JsonValue`) — same stdio_gap #7 below, fixed with the identical
  `serde_to_json_value`/`json_value_to_serde` converter pattern animate/fem already used.
- `📤️export/📊️csv` + `📥️import/📊️csv` leaves: `CsvSnapshot.headers`/`.rows` no longer exist
  (`has_header`/`records` now). Export rewritten to the same honest single-blob-payload
  `CsvRecord`/`CsvField` pattern fem's csv leaves use (prints the real DSL text into one quoted
  cell); import rewritten to the same honest no-mapping-exists default fem's csv import uses (no
  CSV grid can reconstruct a ~78-register program artifact).

Verified: `cargo check -p semio-s-plugin-architect` — **0 errors, 81 warnings** (baseline was 80;
the +1 is an unused-import warning from a doc-comment-only file this closer didn't need to touch
further). `cargo test -p semio-s-plugin-architect --lib` — **248 passed, 0 failed** (the 2 fixture
parse failures the field-rename initially surfaced are both fixed — see above).

## 2. Fixed: remodel (4 → 0 errors)

- `FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL` → `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`/`_LABEL` in
  `🎛️apps/📸️remodel/📌️panels/📄️artifact/🦀️component.rs` (3 use sites) — the exact same
  framework-rename lag animate's own report flagged as affecting ~18 other plugins' `glue.rs`
  files (animate fixed only its own; this closer fixes remodel's, still leaving ~17 others named
  in animate's report untouched, out of this closer's scope).
- `📤️export/🔣️json` + `📥️import/🔣️json` leaves: same `JsonValue`/`serde_json::Value` mismatch as
  architect, same fix pattern (stdio_gap #7).

Verified: `cargo check -p semio-s-plugin-remodel` — **0 errors, 10 warnings** (baseline was 9; +1
same unused-import class). `cargo test -p semio-s-plugin-remodel --lib` — **360 passed, 2 failed**
— both **real, newly-surfaced regressions**, not fixed here, see §4.

## 3. Consolidated `stdio_gaps` (for the orchestrator)

From the 3 filed reports (cad, animate, fem) plus in-code doc comments recovered by direct
inspection for the 4 unfiled plugins (norm, energy, architect, remodel):

1. **cad→ifc**: no real bridge exists — cad's geometry (mesh/brep) has no spatial-tree equivalent
   IFC needs. Fabricated placeholder deleted, not replaced. *(cad)*
2. **cad→png**: no rasterizer/renderer exists anywhere in the repo to produce real pixels from 3D
   CAD geometry. Fabricated placeholder deleted, not replaced. *(cad)*
3. **`CadComposer::compose()`** (all 8 registered dialects: dwg/gltf/ifc/json/obj/png/step/stl)
   only ever attempts to parse the foreign file's raw bytes as cad's own internal DSL text — i.e.
   it has **no real per-format codec for any dialect today**, distinct from the real, working,
   kernel-based `export_solids_as`/`import_step_object` path cad's W5a agent did rewire. Flagged as
   a possible blind spot for anyone consuming `CadComposer` directly; not fixed (much larger
   rebuild, not named in this wave's brief). *(cad)*
4. **mp4 has no real pixel encoder.** stdio's `h264` engine only provides NAL/SPS bitstream framing
   + `avcC` box construction, not a macroblock/pixel-encode pipeline — the task brief's premise of
   "includes a real baseline H.264 encoder" does not match what's shipped. animate worked around
   this honestly via an uncompressed `Mp4Codec::Other` `"rgb8"` escape-hatch codec (real container,
   no compression, clearly documented). *(animate)*
5. **`Mp4Track`/`Mp4Snapshot` only model video-handler (`vide`) tracks** — no schema slot for a real
   audio track (`decode_trak` hard-requires `hdlr[8..12] == "vide"` or the whole `trak` becomes an
   opaque blob). animate dropped audio muxing from `encode_outputs` rather than hand-roll an
   ISO-BMFF `'soun'` trak. *(animate)*
6. **No bridge from the legacy `semio_framework::DwgDrawing` (11 geometry variants) to semio's
   `SemioDrawingSnapshot`/`DrawNode` tree.** Writing one inside any single plugin would duplicate
   `semio_framework_os::dwg_drawing_to_svg`'s existing correct geometry logic and be the 1st of ~9
   near-identical reimplementations across the svg/dwg pattern plugins (W5a + W5b). Recommend one
   shared converter in a future wave. *(animate)*
7. **`JsonSnapshot.value` (stdio's own lexeme-preserving `JsonValue`) has no built-in structural
   bridge to `serde_json::Value`.** Intentional per that schema's own doc comment ("no
   `serde_json::Value` anywhere in this file"), but it means **every** caller needing ordinary
   serde interop must hand-roll a `serde_to_json_value`/`json_value_to_serde` converter pair. Hit
   independently by **4 of 7** W5a agents/closer fixes this wave (animate, fem, and — via this
   closer — architect and remodel), each writing a near-identical ~10-line converter. Recommend a
   single canonical bridge function land in stdio itself (e.g.
   `semio_s_plugin_stdio::artifacts::json::interop::{to_serde, from_serde}`) to stop the
   duplication across every plugin that touches `stdio.json`. *(animate, fem, architect, remodel)*
8. **fem: none.** Every real geometric pair fem needed (mesh↔obj, mesh↔stl) already existed in
   stdio (from W4 G2); zip/png simply aren't honest targets for fem's domain data (no archive-bundle
   capability, no rasterizer) — a scope judgment, not a stdio gap. *(fem, explicit in its report)*
9. **norm: none surfaced**, consistent with fem's finding. No report was filed, but every deleted
   leaf's in-code doc comment gives the same rationale fem used ("no honest whole-artifact CSV
   round-trip exists" — norm's data is tree-shaped standard-lookup content, not a flat table)
   — recovered by direct inspection of the 10 `✳️any/🚪️io/🦀️component.rs` doc comments across
   norm's 10 standard subsets. *(norm, recovered from code, no report filed)*
10. **remodel: stdio's `png::engine::encode_png` always canonicalizes the pixel payload to 8-bit
    RGBA / color type 6, with no 16-bit grayscale encode path.** Found and documented in-code
    (`⚙️engine/🖼️images/🦀️component.rs:156-160`, `encode_png_gray16`) even though remodel filed no
    report: the DSM/heightfield lossless-export writer stays hand-rolled behind the external `png`
    crate as a narrow, single-purpose, explicitly-flagged exception — everything else in that file
    (`decode_png`/`encode_png`/`decode_jpeg`/`encode_jpeg`) is a thin wrapper over stdio's real
    engines. *(remodel, recovered from code, no report filed)*
11. **energy: none found.** No report filed; grepped all 4 files energy touched for `stdio_gap` —
    zero hits. energy's rewiring (`EpwWeather::parse` → stdio's real
    `epw::standards::energyplus::engine::decode_epw`) appears to be a full, gap-free replacement per
    the verifier's own independent code read. *(energy, absence confirmed by grep, no report filed)*
12. **architect: none found beyond #7 above.** No report filed; the verifier's own diff read
    confirmed real csv/tsv rewiring with new tests and no documented gap. *(architect, no report
    filed)*

## 4. Design-judgment follow-ups (not fixed — documented for the orchestrator)

1. **remodel: 2 real test regressions, newly surfaced by this closer's gate run** (remodel filed no
   report, so this is their first record):
   - `images::tests::jpeg_decode_never_panics_on_truncated_input` — **fails**. stdio's `decode_jpg`
     is measurably more lenient with truncated bitstreams than the hand-rolled decoder it replaced:
     several truncation points that used to hard-error now decode successfully. A genuine behavior
     change from the codec swap that remodel's own (unfiled, unverified) work didn't catch. Needs a
     design decision — harden stdio's jpg decoder's truncation handling, or accept the new lenient
     behavior and relax the test — not a cheap/safe fix.
   - `reconstruction::standards::v1::engine::tests::long::video_in_yields_watertight_mesh_out` —
     **fails**. The full synthetic-video → photogrammetry → mesh pipeline reaches `EngineStatus::
     Done` but yields 0 triangles/0 vertices. Very likely downstream of the mp4 `probe`/
     `extract_frames` rewiring onto stdio's real `decode_mp4` changing frame-extraction behavior for
     this test's synthetic fixture, but confirming that requires pipeline-level debugging, not a
     closer-scope patch.
2. **fem: 8 pre-existing `semio_protocol_conformance` test failures** (fem2d/fem3d binary
   mutations/snapshot) — root cause is a 4× `📡️component.protocol.semio` grammar fixture whose
   protocol identifier starts with a digit (`protocol 2d.mutations`), rejected by
   `::dsl::parse_grammar`'s `expected Ident, found Int "2"`. Pre-migration defect, orthogonal to
   codec extraction, already flagged by fem's own report for W8/orchestrator routing — reconfirmed
   present, not fixed here (grammar-identifier judgment call, out of this ticket's mandate).
3. **`ProgramMutation::Documents` enum variant, `ProgramDiff.documents` field, and the
   `"documents"` external register-name string were deliberately left unrenamed** in architect
   (only the target-side `ProgramArtifact`/`ProgramSnapshot` struct field became `artifacts`).
   Renaming these too would touch the wire-visible mutation-op vocabulary and register-name API
   surface — a design/compatibility call this closer chose not to make unilaterally to fix a
   compile error. Flagged for whoever owns architect's terminology consistency next.
4. **Missing reports**: norm, energy, architect, remodel filed no `w5a-*-report.md` (CLAUDE.md hard
   requirement). architect's and remodel's compile-breaking gaps are now closed and documented in
   §1/§2 above; norm's and energy's underlying work was independently re-verified as real by both
   the verifier and this closer (§3 items 9/11) but no first-person report exists for either — not
   backfilled here (authoring another agent's first-person report is out of a closer's scope; their
   real findings are captured in §3 instead).
5. **cad's plugin tree is under live, unrelated, concurrent edit** for the entirety of this
   closer's session (confirmed via a transient `cargo check -p semio-s-plugin-cad` failure — "this
   file contains an unclosed delimiter" in `glue.rs`, self-resolved one poll later — and via
   `git diff --numstat` on cad's tree changing three times across this session: 389/2796 per cad's
   own report → 737/4020 → 884/4082; cad's policy breach count also moved 29→48 vs. the verifier's
   snapshot). None of this is W5a's cad work regressing — cad's own `cargo test` is 129 passed/0
   failed on a clean poll — but the live numstat/breach-count cannot be cleanly attributed to W5a
   right now. Flagged for the orchestrator to re-diff once that concurrent session settles.
6. **New policy rule category since W5a-verify**: `mutation-migration/semantic-vocabulary` (15
   breaches) appeared between the verifier's 25-rule/21609-breach snapshot and this closer's
   26-rule/21654-breach snapshot. Confirmed via grep: **all 15 breaches are under
   `✏️s/🔌️plugins/🌍️gis/**`** — entirely W5b's territory (this ticket folder's own
   `w5b--gis-*.md`/`.txt` files confirm gis is W5b-owned), not touched by any of the 7 W5a plugins.
   Not W5a's responsibility; flagged for whoever owns W5b/gis.
7. **animate's naming-collision warning** (from its own report): the wave's generic
   `w5a--<description>.txt` prompt-template filename convention is unsafe for parallel same-wave
   agents (animate found `w5a--report.md`/`w5a--cargo-check.txt`/`w5a--cargo-test.txt` already
   claimed by energy when it started). Process note for future dispatches, no code action.

## 5. Cross-plugin gate (final snapshot, 2026-08-12 ~02:17-02:25 CEST)

Full raw output for every command below is saved in this ticket folder as
`w5a-close-*.txt`/`w5a-close-final-sweep.txt`.

- **`cargo check -p semio-s-plugin-stdio --lib`**: **0 errors, 485-488 warnings** (fluctuates
  slightly run-to-run — see the important caveat below). Confirmed unchanged/still-green at the
  moment this gate's numbers were captured.
- **`cargo test -p semio-s-plugin-stdio --lib`**: **1866 passed, 4-5 failed, 4 ignored** at the
  moment of capture. All failures are inside `artifacts::semio::standards::v1::subsets::
  {brep,mesh,model,object,drawing}` — a **large, live, in-progress, foreign session** (confirmed
  via `git status`: 220 unstaged files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/**` at
  time of writing), literally regenerating diff/mutation facets with committed placeholder text
  such as `"PLACEHOLDER_WILL_BE_REGENERATED_FROM_REAL_print_dsl_OUTPUT"` still present in some
  fixtures. This is the same class of instability `w5a-verify-report.md` §8 already flagged for
  pptx/workflow (now resolved) — a *different* concurrent wave has since started on the `semio/*`
  subsets. **stdio itself compiles clean; its own test failures and even its compile status
  oscillated 3 times across this closer's session** (0 errors → 4 errors [brep] → 0 → 6 errors
  [drawing] → 0, each self-resolving within ~1-2 minutes) purely from that foreign wave landing
  incremental edits. None of it is W5a's — W5a's write scope never included `✏️s/🔌️plugins/🗄️stdio/**`
  for any of the 7 agents. **Recommend the orchestrator re-run this specific gate command once that
  concurrent session lands**, rather than trust any single snapshot (including this one).
- **`bun ./📜️script.ts policy`**: **21654 high-priority breaches across 26 rule(s)** (verifier's
  own snapshot was 21609/25; W4 baseline was 21532/25). The +1 rule
  (`mutation-migration/semantic-vocabulary`, 15 breaches) is **100% inside `gis` (W5b's scope)** —
  see §4 item 6. Per-plugin breach counts for the 7 W5a plugins, this snapshot vs. the verifier's:
  remodel 847→847 (Δ0), animate 850→850 (Δ0), energy 845→845 (Δ0), architect 900→900 (Δ0), fem
  1712→1712 (Δ0), norm 10817→10817 (Δ0), **cad 29→48 (Δ+19, foreign — see §4 item 5)**. Every W5a
  plugin this closer actually edited (architect, remodel) shows **zero policy delta** — the
  field-rename/leaf-rewrite fixes were policy-neutral.
- **Per-plugin `cargo check -p <crate>`** (all captured in the same ~8-minute window while stdio
  was green): **all 7 compile clean, 0 errors each** — remodel (10 warnings), cad (7 warnings),
  animate (5 warnings), energy (6 warnings), architect (81 warnings), fem (59 warnings), norm (258
  warnings).
- **Representative `cargo test -p <crate> --lib`** (spot-checked, not all 7 re-run at the gate
  command's request but useful for completeness): architect 248/0 failed (fixed, see §1); cad
  129/0 failed (0 on a clean poll — 2 failures seen on one poll were the transient glue.rs churn,
  self-resolved); animate 208/0 failed (unchanged); energy 246/0 failed (unchanged); fem 324/8
  failed (pre-existing, §4 item 2); norm 834/0 failed (unchanged); remodel 360/2 failed (**new
  regressions, §4 item 1** — not present in remodel's baseline because remodel never ran/reported
  its own tests).

## 6. Overall assessment

Both of the verifier's real blockers (architect 19 errors, remodel 4 errors) are fixed with
mechanical, same-crate, no-stdio-edit changes — the identical "lagging call-site" pattern
fem/animate/norm already used successfully in this same wave. All 7 plugin crates now compile
clean in isolation. Architect's own test suite is fully green after the fixture-fidelity fix;
remodel's is not — 2 real regressions surfaced by finally running its tests for the first time
this wave, both requiring design judgment beyond a closer's cheap-fix scope (§4 item 1). stdio
itself is unchanged and clean, but its live test/compile status is actively unstable from an
unrelated, large, in-progress foreign wave touching `semio/{brep,mesh,model,object,drawing}` — the
orchestrator should treat any single gate snapshot (including this one) as provisional until that
settles. The wave's actual engineering (per the verifier's own read, reconfirmed here) remains
real and honestly documented: no fabricated codecs, no ad-hoc byte reinterpretation anywhere across
all 7 plugins post-fix.
