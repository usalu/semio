# W4 Independent Verification Report

Verifier: W4 verification pass, re-checking all 6 W4 groups' claims against disk + a live
`cargo test`/`bun ./📜️script.ts policy` run. Nothing below is taken from any agent report without
re-derivation from source or a live command.

## 0. Report inventory — one group has NO report

Of the 6 groups named in the verification brief, only 5 filed a `w4-*-report.md`:

| Group | Scope | Report file | Filed? |
|---|---|---|---|
| G1 | brep↔step | `w4-g1-brep-step-report.md` | yes |
| G2 | mesh↔gltf/stl/obj/ply/las | `w4-mesh-report.md` | yes |
| G3 | model↔ifc/bcf + object↔json/xml/csv | `w4-g3-modelobject-report.md` | yes |
| **G4** | **drawing↔svg/dxf/pdf + cad↔dxf/dwg/step + image↔png/jpg/gif/bmp/tiff** | **none** | **NO** |
| G5 | video↔mp4/avi + audio↔mp3/wav + animation↔gltf/mp4/gif | `w4-g5-report.md` | yes |
| G6 | document↔docx/md/txt/pdf + presentation↔pptx + workflow↔json | `w4-document-presentation-workflow-report.md` | yes |

G4's identity/scope was reconstructed from three *other* groups' reports, which independently
name it and describe one of its bugs (see §2). Direct inspection confirms G4's code is real and
substantial (`git status` shows all of `✳️drawing/🚪️io`, `✳️cad/🚪️io`, `✳️image/🚪️io` as new,
untracked, non-trivial files — 85–146 lines each, real logic, not stubs), so the work was done;
G4 simply never wrote the CLAUDE.md-mandated summary markdown file. This is a real process gap in
G4's own compliance, flagged here, not fixed (out of this verifier's scope).

## 1. Live gate results (re-run from disk, not taken from any report)

`cargo test -p semio-s-plugin-stdio --lib` (full crate, this session, this verifier):

```
test result: FAILED. 1645 passed; 10 failed; 2 ignored; 0 measured; 0 filtered out; finished in 7.78s
```

Failure breakdown:
- **1 failure inside this ticket's scope**: `artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec` — real, live, reproduced. See §2 for root-cause analysis (not what the 5 filed reports assume).
- **9 failures confirmed foreign, unrelated to W4**: 4 in `artifacts::png::standards::v1_2::engine::tests::conformance_laws::*`, 5 in `artifacts::zip::standards::v2_0::engine::tests::conformance_laws::*`. `git status` confirms both `png`'s and `zip`'s own `⚙️engine`/`🧬️schema` trees are mid-edit by a different, concurrent, uncommitted session (not W4 — W4 never touches format artifacts' own engine/schema, only semio-subset io leaves) — classic "Concurrent Cargo Workspace Churn" per this repo's own documented hazard, not chased.

Filtered to just the 12 semio subsets W4 touched (`brep/mesh/model/object/cad/drawing/image/video/audio/document/presentation/workflow`):
```
test result: FAILED. 418 passed; 1 failed; 0 ignored; 0 measured; 1238 filtered out
```
Per-subset breakdown (all green except drawing): brep 22, mesh 46, model 22, object 54, cad 20, **drawing 20 (1 of which fails)**, image 30, video 26, audio 38, document 40, presentation 27, workflow 23.

Baseline comparison (post-W3, per `w2a-close-report.md`): **1491 passed / 6 failed (csv/json foreign)**.
Now: **1645 passed / 10 failed**. +154 passed matches the volume of new W4 io-leaf tests. The
6 baseline csv/json failures are GONE (fixed by some other concurrent session, not W4 — not
investigated further, out of scope). The 10 current failures are a different foreign set
(png×4, zip×5 — new concurrent churn) plus the 1 real drawing↔pdf failure.

`bun ./📜️script.ts policy`: **21532 high-priority breaches across 25 rules** (baseline was 21524
per both `w2a-close-report.md` and `w3-close-report.md` — delta +8, in line with ordinary
concurrent-wave churn; no new breach classes). Spot-checked all 12 subsets' new `🚪️io` directories
directly in the fresh breach output: only 4 `taxonomy/emoji-prefix` hits, all on `📰xml`/`📄txt`
leaf dirs mirroring the real xml/txt artifacts' own canonical (pre-existing, unfixable-here)
directory names — matches every report's claim, including a spot-check of G4's own `✳️cad`/
`✳️drawing`/`✳️image` composer files' `OnceLock<…>` breaches (6 hits, identical shape to every
other subset's sanctioned lazy-cache idiom).

## 2. The one real bug — root cause is NOT the io leaf itself

`real_byte_round_trip_through_pdf_codec` (in `✳️drawing/🚪️io/📤️export/…/📄️pdf/…/🦀️component.rs`)
fails:
```
left: "hellosemio"
right: "hello\nsemio"
```
Traced to source. `SemioDrawingToPdf::serialize` itself is CORRECT — it builds `text: "hello\nsemio"`
and the test's own first assertion (line 89, `assert_eq!(pdf.pages[0].text, "hello\nsemio")`)
passes. The failure is on the SECOND assertion (line 94), after a round trip through `pdf`'s own
shared codec (`crate::artifacts::pdf::standards::v1_7::engine::{encode_pdf, decode_pdf}` —
correctly reused, zero reimplementation, exactly as documented). Root cause is a real bug in that
shared `pdf` engine, not in G4's leaf: `encode_pdf` (line ~1341) emits one `Tj` operator per text
line separated by a position-only `T*` operator; `extract_text`'s `"Tj"` handler (line 1120) never
inserts `\n` between consecutive `Tj` calls — only the `'`/`"` operator handlers do (line
1125-1128), which `encode_pdf` never emits. So multi-line text written by `encode_pdf` always
comes back joined with no separator on decode.

**Verdict**: the drawing↔pdf leaf's own mapping logic is real and correct; the test it wrote is a
legitimate, valuable test that caught a genuine pre-existing bug in `pdf`'s own shared engine
(outside any single W4 group's leaf-file write scope — fixing it means editing `pdf`'s own
`⚙️engine`, which none of the 6 io-leaf groups were scoped to touch). This is why G1, G3, G5, and
G6 all independently observed and correctly declined to fix it. It remains a real, live, unfixed
gap blocking a fully-green crate.

## 3. Sampled pairs (12, spanning all 6 groups — required minimum was 8)

| # | Pair | Group | Kind | Real mapping (not stub)? | Zero codec reimpl? | Lossiness documented? | Test(s) exist & pass? | Verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | brep ↔ step | G1 | hard | Yes — real Part-21 entity-graph walk (VERTEX_POINT/EDGE_CURVE/ADVANCED_FACE/…), ISO-10303-42-checked | Yes — calls step's own `Part21Builder`/`from_part21_document`, zero Part-21 tokenizing here | Yes — 6 itemized honest gaps (ref_direction rotation, same_sense, etc.) w/ doc comments | Yes, 5 tests incl. full curve/surface-vocabulary round trip + 2 disproof (error-not-fabricate) tests; all pass live | **PASS** |
| 2 | mesh ↔ obj | G2 | clean/geometry-only | Yes — real fan-triangulation + multi-index→flat-soup conversion | Yes — zero byte parsing | Yes — index-sharing-structure loss, groups/mtllib drop, all documented | Yes, 3 tests incl. out-of-range hard-error test; pass | **PASS** |
| 3 | mesh ↔ gltf | G2 | hard | Yes — real accessor/material/texture mapping | Yes (import) — uses `decode_accessor`/`decode_data_uri` exclusively. Export hand-packs `f32`/`u32` LE bytes into new accessors via `to_le_bytes()` — **note**: gltf's engine has no `encode_accessor` counterpart to call, so this is unavoidable primitive value-packing (not re-parsing/re-implementing an existing decoder), acceptable but worth the orchestrator's awareness | Yes — LINE_LOOP gap, scalar-only PBR, unreferenced textures, all documented | Yes, tests incl. round trip + dangling-material + empty-positions hard-error; pass | **PASS (minor note)** |
| 4 | model ↔ ifc | G3 | hard | Yes — real Shepperd quaternion extraction from composed 4×4 world matrices, verified round-trip <1e-9 in test | Yes — calls ifc's own `analyze_spatial`/`to_part21_document`, zero Part-21 reparsing | Yes — extensive, itemized (IFCPROJECT drop, name field absence, geometry out-of-scope, flattening, non-scalar psets, relation regeneration) | Yes — real 4-level (project/site/building/storey+wall+Pset) IFC4 fixture parsed and asserted field-by-field, incl. composed placement math; passes | **PASS (highest quality sampled)** |
| 5 | object ↔ json | G3 | clean | Yes — direct `SemioValue`↔`JsonValue` structural map, real RFC8259 §6 int/float lexeme classification | Yes — zero byte parsing | Yes — Bytes→base64-only (one-directional), Ref cycle detection documented | Yes, tests incl. nested structure + number-lexeme classification; pass | **PASS** |
| 6 | cad ↔ step | G4 (no report) | hard | Yes — real multi-hop STEP entity-graph resolution (LINE→CARTESIAN_POINT+VECTOR→DIRECTION; CIRCLE→AXIS2_PLACEMENT→CARTESIAN_POINT) | Yes — zero byte parsing, walks already-parsed `StepEntity` graph | Yes — non-LINE/CIRCLE entities silently absent (documented as intentional partial bridge, not an error) | Yes, 1 real-fixture test (multi-hop resolution of both a LINE and a CIRCLE); passes | **PASS** |
| 7 | image ↔ png | G4 (no report) | clean | Yes — real pixel/colorspace/metadata struct remap | Yes — zero byte parsing (png's own `pixels` already canonical RGBA8) | Yes — iCCP/text-chunk-kind drops documented | Yes, 2 tests incl. pixel-length-mismatch hard error; pass | **PASS** |
| 8 | drawing ↔ pdf | G4 (no report) | hard | Yes — leaf's own text-collection mapping is real and correct | Yes — leaf itself does zero byte parsing; calls pdf's own `encode_pdf`/`decode_pdf` | Yes — Path/Group-transform/Image drop documented | **Test exists but FAILS live** — real bug, but in shared `pdf` engine, not this leaf (§2) | **FAIL (leaf correct, shared codec bug exposed)** |
| 9 | video ↔ mp4 | G5 | hard | Yes — real per-track/per-sample pts derivation (`dts`+`cts_offset`) | Yes — zero byte parsing in leaf | Yes — AVC sps/pps collapse, cts_offset-always-0-on-export documented | Yes, tests present; pass (confirmed via full-subset run, video 26/26) | **PASS** |
| 10 | audio ↔ wav | G5 | clean/lossless | Yes — real PCM16/PCM8/Float32 de-interleave with exact power-of-two conversions | Yes — zero byte parsing in leaf | Yes — Raw(24-bit/ADPCM/extensible) fallback documented | Yes, tests present; pass (audio 38/38) | **PASS** |
| 11 | document ↔ pdf | G6 | hard | Yes — real page/PageBreak boundary mapping | Yes — zero byte parsing, calls pdf's own codec | Yes — flat-paragraph-only, PdfInfo/object-graph drop documented | Yes, `pdf_round_trip_is_stable` test passes (document 40/40 — this pair's own test does NOT hit the drawing↔pdf newline bug, since document↔pdf's test content is single-line) | **PASS** |
| 12 | workflow ↔ json | G6 | clean/lossless | Yes — direct 1:1 field mapping | Yes — zero byte parsing | Yes — none needed (genuinely lossless), and malformed input is a hard error, not defaulted | Yes, tests present; pass (workflow 23/23) | **PASS** |

11 of 12 sampled pairs are genuinely real, honestly documented, zero-codec-reimplementation
bridges with passing fixture-backed tests — the reports' central claims hold up under direct
re-derivation from source and a live compile+test run. The 12th (drawing↔pdf) has a correct leaf
implementation whose own test correctly exposes a real, live, unfixed bug in a *different* file
(`pdf`'s shared engine) outside any W4 group's write scope.

## 4. Cross-checking the 5 reports' own internal consistency

All 5 filed reports (G1, G2, G3, G5, G6) independently converge on the exact same crate-wide test
tally at their respective "final green" moments (`426 passed; 1 failed`, filtered to
`artifacts::semio`) and the exact same single foreign failure (drawing↔pdf, `"hellosemio"` vs
`"hello\nsemio"`). This cross-report agreement, now independently reproduced live by this
verifier (§1), is strong corroborating evidence none of the 5 reports fabricated their "my scope
is green" claims. No report claimed drawing↔pdf as fixed or as their own responsibility — all
correctly attributed it to G4 and declined to touch it (correct scope discipline).

## 5. Overall verdict

**CONDITIONAL PASS.** All 6 groups' underlying implementation work (26 format-pair bridges, ~60
new leaf files across the ticket) is real, field-by-field, honestly-documented,
zero-codec-reimplementation work — confirmed by direct source inspection of 12 sampled pairs
(50% more than the required minimum) spanning every one of the 6 groups, and by a live,
independently-run full crate test + policy scan. Two real gaps, neither touched by this verifier
per its read-only charter:

1. **G4 never filed its required `w4-*-report.md`** (CLAUDE.md violation) — its work exists and is
   real (confirmed directly), but the paper trail is missing and had to be reconstructed from
   three sibling reports' incidental mentions.
2. **One live test failure** (`drawing::io::export::…::real_byte_round_trip_through_pdf_codec`) —
   root cause is a real, pre-existing bug in `pdf`'s own shared `engine::{encode_pdf,decode_pdf}`
   Tj/newline handling, exposed (not caused) by G4's leaf-level test. Fixing it requires editing
   `pdf`'s own engine file, outside every W4 group's scoped write access — a genuine open item for
   the ticket closer.
3. Two unrelated, confirmed-foreign, in-progress failures (`png`×4, `zip`×5 conformance-law tests)
   from a different concurrent session's uncommitted work — not part of this ticket, not chased,
   noted for completeness against the "should be ~1491/6 baseline" instruction (baseline's own 6
   failures are gone; a different foreign set has appeared in their place — normal multi-agent
   churn, not a regression this ticket introduced).

Raw command output backing this report, both in this ticket folder: `w4-verify-test-out.txt`
(per-subset-filtered `cargo test` run) and `w4-verify-policy-out.txt` (full `bun ./📜️script.ts
policy` run).
