# F6b Verify Report — Independent Verification of 7 Fan-Out Agents

Scope: dwg ac1018, dwg ac1024, bmp, stl, las, gif87a, zip. Nothing from any
self-report was trusted — every claim below was re-derived by grepping the
actual `🦀️component.rs` files on disk and by re-running `cargo test` myself
in this session, from a cold-ish incremental build (`target/debug` already
warm from prior sessions; no `--release`).

Test filter convention confirmed from each artifact's own `f6-<artifact>-report.md`:
`cargo test -p semio-s-plugin-stdio --lib "<module path>"`.

Real-file location note: the actual `OpText`/`OpBinary`/`DiffCodec` impls
live in the **top-level** `🦀️component.rs` directly under `🧬️mutations/`
and `🔺️diff/` — NOT in the nested `📝️text/🦀️component.rs` or
`💾️binary/🦀️component.rs` files, which are pure grammar/protocol-grammar
`include_str!` stubs (5-6 lines each, unrelated to the codec impl). Grepping
the wrong (nested) file would falsely read as "nothing implemented" — I
caught this on the first pass (bmp) and corrected the search path for all
seven before drawing any conclusion.

## Per-artifact results

### dwg ac1018
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg::standards::v_ac1018"`
- **12 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: `#[derive(..., dsl::DslDiff)]` at line 45 → derived `DiffCodec`, not hand-rolled. This matches the artifact's own report and the recon report's classification (dwg ac1018 has no nested/nullable structure defeating the derive).
- `mutations/🦀️component.rs`: real `impl protocol::OpText for DwgMutation` (line 130) and `impl protocol::OpBinary for DwgMutation` (line 153) — hand-rolled, not derived (mutations never derive per the recon report's machinery limits).
- `serde_json::to_string`/`to_vec`/`from_str`/`from_slice` stub calls: **absent** in both diff and mutations files.
- Tests present and passing: `op_text_binary_roundtrip_law` (mutations, line 400) and `diff_codec_text_binary_roundtrip_law` (diff, line 187).

### dwg ac1024
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg::standards::v_ac1024"`
- **18 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: `#[derive(..., dsl::DslDiff)]` at line 117 → derived `DiffCodec`.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for DwgMutation` (line 158) and `impl protocol::OpBinary for DwgMutation` (line 181).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 498), `diff_codec_text_binary_roundtrip_law` (line 379).
- **architectural.dwg fixture, checked specifically**: `✏️s/…/🖊️dwg/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg` exists on disk (148,638 bytes) and is `include_bytes!`'d by `⚙️engine/🦀️component.rs:573` as `ARCHITECTURAL_FIXTURE`. Three tests exercise it directly against the real bytes (not a stub/mock): `real_fixture_d1_locates_every_named_section`, `real_fixture_d2_decompresses_every_section`, `real_fixture_page_directory_matches_header_cross_check` — all three passed in my own run (visible in the 18-test scoped output above and in the full-crate run).

### bmp
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::bmp"`
- **16 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: `#[derive(..., dsl::DslDiff)]` at line 257 → derived `DiffCodec`.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for BmpMutation` (line 215), `impl protocol::OpBinary for BmpMutation` (line 236).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 549), `diff_codec_text_binary_roundtrip_law` (line 509).

### stl
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::stl"`
- **23 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: **hand-rolled** — `impl protocol::DiffCodec for StlDiff` at line 563 (the file's own header comment at line 10 documents that the recon report's decision rule ruled out the derive here). No `dsl::DslDiff` derive present.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for StlMutation` (line 212), `impl protocol::OpBinary for StlMutation` (line 223).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 553) and `diff_codec_text_binary_roundtrip_law` — note this test is physically located in the mutations test module (`artifacts::stl::standards::v_ascii::subsets::any::schema::mutations::component::tests::diff_codec_text_binary_roundtrip_law`) rather than the diff file's own module; it still runs and passes, and it does exercise `StlDiff`'s hand-rolled `DiffCodec`. Cosmetic placement deviation only, not a coverage gap.

### las
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::las"`
- **23 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: **hand-rolled** — `impl protocol::DiffCodec for LasDiff` at line 1140 (file's own comment at line 788 states the derive path is not usable here — "STEP 1 classification done for" — consistent with the recon report noting las was missed by the original sweep and needed its own classification work). No `dsl::DslDiff` derive present.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for LasMutation` (line 385), `impl protocol::OpBinary for LasMutation` (line 394).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 857) and `diff_codec_text_binary_roundtrip_law` (line 1193, in the diff file's own test module this time).
- **las-specific check requested by the task, done explicitly**: confirmed las was NOT also skipped by its own fan-out agent despite being missed by the recon sweep. Both the `OpText`/`OpBinary` impls and the hand-rolled `DiffCodec` impl are real, substantive (~350+ line mutations file, DiffCodec impl spans from line 1140 through the test module), and covered by both required round-trip law tests, both of which pass live.

### zip
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::zip"`
- **40 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: **hand-rolled** — `impl protocol::DiffCodec for ZipDiff` at line 649. File's own header comment (line 8) states the derive "CANNOT be used on `ZipDiff`" with the finding confirmed by a real test run. No `dsl::DslDiff` derive present.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for ZipMutation` (line 253), `impl protocol::OpBinary for ZipMutation` (line 276).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 655) and `diff_codec_text_binary_roundtrip_law` (in the mutations test module, mirroring stl's placement pattern).

### gif87a
- Filter: `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::standards::v87a"`
- **27 passed, 0 failed** (re-ran myself, confirmed live)
- `diff/🦀️component.rs`: **hand-rolled** — `impl protocol::DiffCodec for GifDiff` at line 681. File's own comments (lines 179, 466) document that neither `DslRecord` nor `DslDiff` derives are usable here because `GifDiff` (and nested `GifImageDiff`) don't fit the derive's shape. No `dsl::DslDiff` derive present.
- `mutations/🦀️component.rs`: real `impl protocol::OpText for GifMutation` (line 173), `impl protocol::OpBinary for GifMutation` (line 194).
- `serde_json` stub remnants: **absent**.
- Tests present and passing: `op_text_binary_roundtrip_law` (line 298, mutations) and `diff_codec_text_binary_roundtrip_law` (line 878, diff file, explicitly noted in its own docstring as mirroring gif89a's sibling test — consistent with the recon report's gif89a worked example).

## Full crate test suite (run once, my own session, `cargo test -p semio-s-plugin-stdio --lib`)

**1047 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in ~7.4-7.7s.**

Full raw output saved to:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6b-verify-full-crate-test-run.txt`

This confirms the artifacts' own claims of a clean 1047-passed baseline
(zip's and gif87a's self-reports both independently landed on 1047/0 by
the time their sessions finished) and, more importantly, confirms it fresh
from my own uninvolved verification run rather than trusting any prior
session's number.

## Summary table

| artifact | tests (scoped) | diff codec | mutation ops | serde_json stub | op_text_binary_roundtrip_law | diff_codec_text_binary_roundtrip_law |
|---|---|---|---|---|---|---|
| dwg ac1018 | 12/12 | derived (`dsl::DslDiff`) | hand-rolled | gone | present, passes | present, passes |
| dwg ac1024 | 18/18 | derived (`dsl::DslDiff`) | hand-rolled | gone | present, passes | present, passes |
| bmp | 16/16 | derived (`dsl::DslDiff`) | hand-rolled | gone | present, passes | present, passes |
| stl | 23/23 | hand-rolled (`DiffCodec`) | hand-rolled | gone | present, passes | present, passes (in mutations test module) |
| las | 23/23 | hand-rolled (`DiffCodec`) | hand-rolled | gone | present, passes | present, passes |
| zip | 40/40 | hand-rolled (`DiffCodec`) | hand-rolled | gone | present, passes | present, passes (in mutations test module) |
| gif87a | 27/27 | hand-rolled (`DiffCodec`) | hand-rolled | gone | present, passes | present, passes |

Scoped counts sum to 159; the full crate run of 1047 also includes every
other artifact in the workspace (all other F1-F5/F6 siblings), so the
1047 total is not just these seven — it's the whole-crate baseline, which
came back clean.

## Deviations / notes worth flagging

1. Two artifacts (stl, zip) have their `diff_codec_text_binary_roundtrip_law`
   test physically located inside the `mutations` test module rather than
   the `diff` module. This is a cosmetic/organizational deviation from the
   pattern set by the other five artifacts, not a coverage gap — the test
   itself does exercise the respective `DiffCodec` impl and passes.
2. las was confirmed to have received full, real coverage despite being
   missed by the original recon sweep — this was explicitly checked per
   the task's las-specific instruction, and there is no evidence it was
   also skipped downstream by its own fan-out agent.
3. dwg ac1024's `architectural.dwg` fixture is real (148,638 bytes, present
   on disk) and is exercised by three tests that all pass — checked
   specifically per the task's dwg-ac1024 instruction.
4. No `serde_json::to_string`/`to_vec`/`from_str`/`from_slice` stub
   remnants found in any of the 14 files checked (mutations + diff,
   ×7 artifacts).
5. I did not touch any file outside my read-only verification scope; no
   edits were made to any artifact, `📦️glue.rs`, `📜️script.ts`, or any
   other shared/owned file.
