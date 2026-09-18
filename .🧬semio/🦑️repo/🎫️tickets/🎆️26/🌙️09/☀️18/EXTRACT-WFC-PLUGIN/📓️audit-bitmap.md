# 🕵️ Audit — `s.wfc.bitmap` (bitmap artifact) vs. the brief and plan §3/§7

Scope: read-only conformance audit of `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap` (crate
`semio-s-artifact-wfc-bitmap`) against `.../EXTRACT-WFC-PLUGIN/📓️plan.md` §1/§3/§7 and the slice's
own `📓️bitmap.md`. No files were edited, no cargo/git commands were run; test-result lines below are
quoted from the newest logs already committed under `🗑️generated/bitmap/`.

The brief in the auditor's own words: *"the common WFC algorithm use case — the bitmap editor has an
input bitmap and an output bitmap that looks locally similar to the input."* Everything below is
weighed against that sentence first, the plan's contract second.

---

## Verdict table

| # | Area | Verdict | Evidence |
|---|---|---|---|
| 1 | Document model (plan §3 bitmap) | ✅️ **Matches**, no deviations | `🧬️schema/📸️snapshot/🦀️.rs:220-237` — `BitmapSnapshot{schema,seed,input,output,model,pinned}` exactly; `BitmapInput{width,height,palette,pixels}` (`:123-128`), `BitmapOutputSpec{width,height,periodic}` (`:155-159`), `BitmapOverlappingModel{pattern_size,symmetry,periodic_input,ground}` (`:175-181`, ranges `2..=5`/`1..=8` at `:192,195`), `BitmapPinnedPixel{x,y,color}` (`:203-207`) |
| 2a | Interactive input window, coalesced paint | ⚠️ **Partially verified** — Rust side is correct; the coalescing itself is unproven in-repo | see §2 below |
| 2b | Active palette colour in window config | ✅️ **Matches** | `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs` (`BitmapInputWindowConfig{active_color,zoom}`); `✏️editor/🦀️.rs:194-205` `set_active_color` refuses an out-of-range index and writes `WindowConfigMutation`, never a document mutation |
| 2c | Palette editing reachable | ✅️ **Matches** | `add-palette-color`/`change-palette-color`/`remove-palette-color` are all catalogued input-window actions, `🪟️windows/🖼️input/🦀️.rs:44-46` |
| 2d | Read-only output window renders REAL inferred pixels through `ArtifactEditor::render` with the REAL transient (the wfc2d blocking finding) | ✅️ **Fixed, verified** | `✏️editor/🦀️.rs:153-163` `render_with_request_context` calls `Self::render_bodies(body_key, doc.snapshot, cfg, transient.snapshot)` — the REAL `TransientView`, not `Default`. Only the base `render()` (no request context, `:147-149`) uses `BitmapTransient::default()`, which is correct: that is the boot/no-solve-yet path, and the framework's own `render_with_request_context` is what the live output pane calls (this is exactly the pattern the coordinator flagged wfc2d for getting wrong) |
| 2e | Row 50/50 layout | ✅️ **Matches** | `✏️editor/🎭️modes/✏️edit/🦀️.rs:17-19` `create_default_layout(&[input,output], "row", Some(&[50.0,50.0]), ...)` |
| 2f | Pixel payloads paged, never inline in the 32 KiB spine | ✅️ **Matches, budget-guarded** | `🦀️.rs:56-80` `bitmap_layers_json` re-coarsens block size until `text.len() <= BITMAP_LAYERS_JSON_BUDGET_BYTES = 24 KiB`; test `the_layer_budget_is_respected_by_coarsening` (green, quoted in test-final2 below) |
| 2g | Viewer shows both windows | ✅️ **Matches** | `👁️viewer/🎭️modes/👁️view/🪟️windows/{🖼️input,🧩️output}/🦀️.rs` exist and render; test `viewer::bitmap::component::tests::the_manifest_binds_both_windows` green |
| 3a | `extract_2d` + symmetry group, N×N patterns, periodic-input flag | ✅️ **Matches** | `💡️inferences/🦀️.rs:287-296`, `symmetry_group()` at `:145-148` |
| 3b | `Grid2dTopology` with output periodicity + ground row | ✅️ **Matches** | `:298-304` (boundary from `output.periodic`), `push_ground_pins` `:341-352` |
| 3c | Resumable `WfcJob<Grid2dTopology>` | ✅️ **Matches** | `:159-184` struct fields, `:326-329` job construction, `Restore`/`Solve` stages `:499-545` |
| 3d | Three job bugs + one-page-per-step rule handled | ✅️ **Matches, all four present** | unsatisfiable-as-answer `:513-531`; dual-payload retirement `retire_payload` `:21-29` called both in Restore (`:503-504`) and Solve (`:534`); ownership-only `terminal_is_empty` `:657-662`; one-page-per-step encoder `:397-467` |
| 3e | Test: every N×N window of output occurs in the (symmetry-expanded) input | ❌ **Missing from this artifact** | see §3 below — this is the single test the brief's core claim rests on, and it does not exist in the bitmap crate's 170 tests |
| 3f | Contradiction test | ✅️ **Matches, quoted** | see §3 below |
| 4 | Tests: 10× fixture quintet + mounted test + feature row + oracle branch + oracle vector | ✅️ **Matches, all 10** | fixture dirs and oracle manifest `vectors` both list all 10 kinds; feature file `Examples:` table lists all 10 |
| 4b | Render tests both windows, both examples | ✅️ **Matches** | `editor::bitmap::modes::edit::windows::input::component::tests::every_example_renders_a_non_empty_canvas`, `viewer::.../output/tests::every_example_renders_its_declared_output_extent` — both green |
| 4c | Green runs from newest logs | ✅️ **Confirmed** | quoted below |
| 5a | Verbs `fix`/`clear` for pin/unpin | ✅️ **Matches** | `📌️pin-pixel/🦀️.rs:27` `verb: "fix", record: "Fixed"`; `📍️unpin-pixel/🦀️.rs:26` `verb: "clear", record: "Cleared"` |
| 5b | `#[value(rename_all = "camelCase")]` on every payload | ✅️ **Matches** | present on every struct read (`BitmapColor`, `BitmapInput`, `BitmapOutputSpec`, `BitmapOverlappingModel`, `BitmapPinnedPixel`, `BitmapSnapshot`, `SetInputPixels`, `BitmapInferenceCommit`) |
| 5c | Icon ids valid | ✅️ **Matches** | `"image"`, `"grid-3x3"`, `"pencil"` all present in `🧰️framework/.../🪪️icon-name/🦀️.rs` `#[serde(rename=...)]` list |
| 5d | Floats `N.0` | ✅️ **N/A, no violation** | this artifact has no `f64` document fields (colours are `u32`); fixture JSON (`⚙️change-model` vector) carries only integers |
| 5e | No dependency on another app-plugin crate | ✅️ **Matches** | `Cargo.toml` deps are all `semio-framework*`, `pack`, `semio-s-plugin-wfc-engine` — no `semio-s-plugin-{draw,raster,...}` |
| 5f | Labels en+de | ✅️ **Matches** | every `LocalizedLabel::native(en, de)` call sampled carries both |
| 5g | Emoji docstrings, no comments inside definitions | ✅️ **Matches** | every file opens `//! <emoji> ...`; spot-checked `set-input-pixels/🦀️.rs` — clean, no inline comments in the struct/impl bodies |
| 6 | `📓️bitmap.md` self-reported gaps, rated | see §4 below | |

---

## 1. Document model — confirmed, no deviations

`🧬️schema/📸️snapshot/🦀️.rs:220-237`:

```rust
pub struct BitmapSnapshot {
    #[state(artifact)] pub schema: String,
    #[state(artifact)] pub seed: u64,
    #[state(artifact)] pub input: BitmapInput,
    #[state(artifact)] pub output: BitmapOutputSpec,
    #[state(artifact)] pub model: BitmapOverlappingModel,
    #[state(artifact)] #[value(default)] pub pinned: Vec<BitmapPinnedPixel>,
}
```

Every field, type and range in plan §3's `BitmapSnapshot` line is present verbatim, including the
`ground: Option<u32>` model field (`:180`) and the `2..=5`/`1..=8` range constants (`:192,195`). All
10 plan-named mutations exist as their own folders (`🎲️change-seed` … `📍️unpin-pixel`) with
`🦀️.rs`/`🔺️diff/🦀️.rs`/`↩️inverse/🦀️.rs`/`🧬️schema/🔣️.json` siblings. No deviation to report.

---

## 2. Windows — the wfc2d blocking finding is fixed here, one design choice stated but unverified

**The transient wiring (the point that mattered).** `✏️editor/🦀️.rs` declares two render entry
points on `BitmapEditor`:

```rust
fn render(...) -> ... { Self::render_bodies(body_key, doc.snapshot, cfg, &BitmapTransient::default()) }          // :147-149
fn render_with_request_context(..., transient: &TransientView<'_, Self::Transient>, ...) -> ... {
    Self::render_bodies(body_key, doc.snapshot, cfg, transient.snapshot)                                          // :153-163
}
```

`render_with_request_context` — the path the live host actually calls for a rendered pane — passes
the REAL `transient.snapshot`, not `Default`. This is the exact defect class the ticket brief calls
out from wfc2d, and it is NOT present here. The plain `render()` legitimately uses a default (no
transient available at that call site; it is the pre-boot/manifest path), which is fine because the
output window's own `render()` (`🪟️windows/🧩️output/🦀️.rs:55-62`) discards a stale cache whose
extent no longer matches `document.output` (`fresh` check `:56`) rather than rendering it — so even
a caller that somehow reached this path with a mismatched transient cannot show a wrong-looking
answer.

**Gesture coalescing — code is right, but nothing in this repo proves the coalescing happens.** The
document comment (`✏️editor/🦀️.rs:9-12`) and `📓️bitmap.md` both assert "a pointer drag ... coalesces
into ONE `set-input-pixels` on release." What actually exists:

- `BitmapEditorCommand::StrokeBegin`/`StrokeExtend` variants (`:56-59`) that `handle` turns into
  `Ok(Emit::default())` (`:117`) — true no-ops at every lane, which is correct if a client uses them.
- Neither `stroke-begin` nor `stroke-extend` is registered as a window `ActionDefinition`
  (`🪟️windows/🖼️input/🦀️.rs:41-50` lists only 8 actions — no stroke verbs), so neither one receives
  `InteractiveJobClassification::Migrated`. Per this repo's own dispatch law (an unclassified verb is
  dispatch-dead in the shell — project memory `interactive-job-classification-gates-dispatch`), these
  two commands, as declared, are **not reachable from a live client at all** — they round-trip their
  binary encoding in a unit test (`every_command_round_trips_its_binary_op`) but nothing exercises
  `handle`/`ephemeral` through the actual dispatch path.
- There is no `stroke-end`/`stroke-commit` command anywhere in the enum. The claimed mechanism —
  "the settled rectangle arrives once, on release, as a single `set-input-pixels`" — must mean a
  client-side canvas tool computes the dirty rectangle locally and dispatches `SetInputPixels`
  directly (which IS a classified, reachable action). That is a legitimate design (matches raster's
  own free-hand-then-batch-commit precedent per the patterns report §2.4), but it means
  `StrokeBegin`/`StrokeExtend` are, as committed, dead code: declared, encodable, semantically
  documented, never wired to anything that can invoke them.
- No TS/React pointer-handling code exists yet anywhere under this artifact (only barrel `🟦️.ts`
  stub files) — expected, since `📓️bitmap.md` §8.6 explicitly defers playground/browser wiring to
  W2. So the "coalesce into ONE mutation" claim is unverifiable in this repository today; it is a
  Rust-side contract that assumes a host implementation that does not exist yet.

This is **not** a regression of the wfc2d finding (the transient/render defect class) — it is a
separate, narrower gap: the gesture-coalescing story is asserted but not demonstrated, and two
enum variants are presently unreachable dead code.

**Everything else about the windows is as specified**: `Canvas2dScene` (not `Paint2dScene`, stated
and justified reason `🦀️.rs:5-9`), row 50/50 layout, `wfc-bitmap-input`/`wfc-bitmap-output` window
kind ids matching plan, `BitmapInputWindowConfig{active_color,zoom}` /
`BitmapOutputWindowConfig{show_pins,zoom}`, both registered via `register_window_config_owners`
(`✏️editor/🦀️.rs:98-101`).

---

## 3. "Locally similar" — the mechanism is right; the artifact-level proof is missing

The inference route matches plan §3 and §7 exactly (`extract_2d` + `symmetry_group`, `Grid2dTopology`
with `output.periodic` boundary, ground-row pins, resumable `WfcJob`, the three named job-lifecycle
bugs and the one-page-per-step encoder — all present, cited in the verdict table above with line
numbers).

**Contradiction test — present, quoted** (`💡️inferences/🧪️tests/🔬️unit/🦀️.rs:73-95`):

```rust
fn a_sample_that_cannot_tile_the_output_reports_a_contradiction() {
    // sample "0 1 0" with N=2, periodic input, pins colour 1 on two adjacent output cells...
    let solved = solve_with_job(&snapshot).expect("the job completes even when the spec is unsatisfiable");
    assert!(solved.contradiction, "a spec with no consistent assignment is a verdict, never a wrong bitmap");
    assert!(solved.pixels.is_empty(), "a contradiction carries no output pixels");
    ...
}
```

**The "every N×N window of the output occurs in the (symmetry-expanded) input" test does NOT exist
in this artifact.** I read every test in `💡️inferences/🧪️tests/🔬️unit/🦀️.rs` (9 tests:
roster identity, descriptor leaves, symmetry-prefix, deterministic-seed, different-seed-valid,
contradiction, unreachable-pin refusal, admission ceiling, prior entropy, solve-field) and every test
in `📚️examples/🧪️tests/🧩️outcome/🦀️.rs` (well-formed problem, real pattern universe, solve
terminates with a verdict + honours pins). None of them decodes the output bitmap and checks its
N×N windows against the input's (symmetry-expanded) window set. The closest existing assertion,
`the_solve_is_deterministic_for_a_seed` (`:52-62`), only checks the output decodes, has the right
cell count, and every index names a real palette entry — it never checks local similarity to the
input at all.

The property IS proven, but one layer down and in a weaker form: the engine crate carries
`extract_2d_output_is_locally_similar_to_its_sample` at
`✏️s/🔌️plugins/🀄️wfc/⚙️engine/🧪️tests/🔬️grid-job-drive/🦀️.rs:99-134`, quoted:

```rust
#[test]
fn extract_2d_output_is_locally_similar_to_its_sample() {
    ...
    let extracted = extract_2d(&[sample.clone()], &Extract2dConfig { window: WINDOW, periodic_input: true, symmetry: SymmetryGroup2d::None }).expect("overlapping extraction");
    ...
    let allowed = sample_windows(&sample, WINDOW);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let mut cells = ...;
            assert!(allowed.contains(&cells), "output window at ({x}, {y}) does not occur in the sample");
        }
    }
}
```

This proves the property generically for the engine's `extract_2d`+`WfcJob` pair, but: (a) it uses
`SymmetryGroup2d::None`, never exercising the symmetry-expanded case the plan explicitly calls out
("the (symmetry-expanded) input"); (b) it bypasses `BitmapSnapshot`/`solve_with_job` entirely, so it
never runs through the bitmap artifact's own pipeline (base64 codec, ground row, pin anchoring,
palette); (c) it is not reachable from, or referenced by, the bitmap crate's own test suite — a
reader auditing bitmap alone would not find it. Given the brief states the local-similarity property
as the entire reason this artifact exists, its absence from the bitmap crate's own 170 tests, in a
form that exercises the actual document (including a symmetry > 1 case), is the most material gap
in this audit.

---

## 4. Test inventory & green runs — confirmed from the newest logs

Newest test log `🗑️generated/bitmap/test-final2.txt` (14:21, post-dates the `test-final.txt` bitmap.md
itself cites), tail:

```
test tests::the_layer_budget_is_respected_by_coarsening ... ok
test examples::tests::every_example_solve_terminates_with_a_verdict ... ok

test result: ok. 170 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 15.27s
```

Matches the 170/0/1 claim in `📓️bitmap.md` §1.

`🗑️generated/bitmap/clippy-final.txt` — every `warning: ... generated N warnings` line names a
framework dependency (`semio-framework-plugin`, `-os-kernel`, `-replication`, ...); **no line names
`semio-s-artifact-wfc-bitmap`**, confirming the "zero warnings from this crate" claim. (Note:
`check-8.txt`, an earlier intermediate log timestamped 13:43, does show 13 "unnecessary
qualification" warnings from this crate's own test files — but it predates `clippy-final.txt`
(14:18) and `test-final2.txt` (14:21), so it reads as a pre-fix snapshot, not the current state.)

`wasm-2.txt` tail: `Finished 'dev' profile ... target(s) in 17.55s` after `Checking
semio-s-artifact-wfc-bitmap v0.1.0` with no error — wasm32-wasip2 green, matches claim.

All 10 mutations are covered by fixture quintet (before/after/mutation/diff/outcome under
`🧫️fixtures/🧬️mutations/<kind>/<scenario>/`), a mounted Rust test
(`🧬️mutations/<kind>/🧪️tests/<scenario>/🦀️.rs`), a `🥒️.feature` `Examples:` row (both
`@id-mutate`/`@id-inverse` scenario outlines list all 10 kinds), and an oracle manifest vector
(`🔮️oracles/🔣️.json` → `mutationCatalogs[0].vectors` lists all 10 `mutationId`s, and
`mutationManifests[0].mutations` lists all 10 with `oracleRequirements`). Confirmed by direct read,
not by trusting the slice report's count.

---

## 5. Rating `📓️bitmap.md`'s self-reported gaps (§8)

| bitmap.md's own gap | rating |
|---|---|
| §8.1 `verify taxonomy enforce` red for a repo-wide, pre-existing reason (unsorted compiler-input manifest; unrelated fixture digest mismatch), reproduced on untouched `procedural/assembly` too | **nice-to-have** for this slice — plausibly not this artifact's fault as described, but I did not independently re-run the gate (out of scope: no cargo/build commands), so I cannot confirm the reproduction claim myself; flag for W2 to re-verify before relying on it |
| §8.2 nine mutation directory names hand-inserted into `semanticDirectoryMemberKinds` because the scaffolder refuses an already-populated directory | **nice-to-have** — a stated, narrow, surgical workaround; `loadTaxonomy()` allegedly validates green after, unverified independently here |
| §8.3 plugin root needs `Transient=BitmapTransient`/`TransientMutation=BitmapTransientMutation`, two window-config owners | **blocking-for-plugin-root** (slice P dependency), not a defect in this slice |
| §8.4 export list the plugin root needs | **blocking-for-plugin-root**, informational for this slice |
| §8.5 `semio-framework-os-infinite` dropped because it doesn't compile on a peer's in-progress edit | **nice-to-have** — correctly scoped/justified (Board2d unused here), re-check once the peer's fix lands |
| §8.6 describe/registry/playground/browser probes/`verify dependencies literal-external` not run | **blocking-for-brief indirectly** — this is exactly why the gesture-coalescing claim in §2 above is unverifiable; W2 must close this before the "interactive input window" requirement can be called proven end-to-end |
| §8.7 diff has no text/binary Rust facet (TS-only barrel stubs) | **nice-to-have** — stated as intentional (a diff is never an authored document), consistent with sibling artifacts |

---

## Ordered fix list

1. **[blocking-for-brief]** Add a bitmap-artifact-level test that decodes a real `solve_with_job`
   output and asserts every `N×N` window occurs in the symmetry-expanded input windows (mirror
   `extract_2d_output_is_locally_similar_to_its_sample` from the engine crate, but drive it through
   `BitmapSnapshot`/`solve_with_job` with `model.symmetry > 1` on one of the two committed examples
   or a dedicated fixture). This is the direct, artifact-level proof of the brief's defining claim,
   and it does not currently exist anywhere the bitmap crate's own suite would catch a regression.
2. **[nice-to-have, cheap]** Either wire `stroke-begin`/`stroke-extend` into the input window's
   `ActionDefinition` catalog (so they are `Migrated` and actually reachable), or remove them from
   `BitmapEditorCommand` until a client exists that calls them — as committed they are unreachable
   dead code that documents an interaction contract nothing can invoke yet.
3. **[nice-to-have, tracking]** When slice P/W2 resumes, close bitmap.md §8.6 (playground boot +
   browser probe) specifically to demonstrate the pointer-down/drag/up → single `set-input-pixels`
   coalescing claim end-to-end, since it is presently asserted in doc comments only.
4. **[nice-to-have]** Re-run `verify taxonomy enforce` scoped to `🖼️bitmap` once the repo-wide
   compiler-input-manifest/fixture-digest issue bitmap.md blames is independently fixed, to confirm
   this artifact is actually clean rather than merely "failing for the same reason as everything
   else."
5. **[nice-to-have]** Re-run clippy/check for this crate after any future edit to confirm the 13
   "unnecessary qualification" warnings seen in the intermediate `check-8.txt` stay fixed (the
   current `clippy-final.txt`/`test-final2.txt` pair shows them gone, but there is no standalone
   `cargo check --tests` log newer than `check-8.txt` to double-confirm outside of clippy's own
   check pass).
