# F3 Independent Verification — gif (87a+89a), png, md, dxf

Role: independent verifier of F3's fan-out agents' self-reports (`f3-gif-report.md`,
`f3-png-report.md`, `f3-md-report.md`, `f3-dxf-report.md`). Nothing below is taken on trust from
any agent's own report — every claim was re-checked against disk (fresh `grep`/`Read`) and every
test count was reproduced by re-running `cargo test` myself, in this session, just now.

**Note on `f3-closer-report.md`**: that closer report (dated earlier in this same wave) found gif
and dxf "NOT done" — it predates a later gif+dxf mop-up pass (`f3-gif-report.md`/`f3-dxf-report.md`,
both now present on disk) that is the actual current state. My findings below supersede the closer
report for gif/dxf; the closer report's png/md findings still match what I independently found.

## Per-artifact filtered test runs (reproduced fresh, this session)

| artifact | filter | passed | failed |
|---|---|---|---|
| gif (87a+89a combined) | `artifacts::gif::` | 55 | 0 |
| gif `💃️dancing` fixture | `artifacts::gif::examples::dancing` | 4 | 0 |
| png | `artifacts::png::` | 22 | 0 |
| md | `artifacts::md::` | 24 | 0 |
| dxf | `artifacts::dxf::` | 13 | 0 |
| **whole crate (no filter)** | — | **853** | **0** |

Whole-crate run is fully green right now — no failures anywhere to classify as
internal-vs-external-wave churn. (The brief's "large concurrent subset-multiplicities wave" for
svg/jpg/tiff/etc. was not observably mid-edit at the moment of this run; its files are outside this
verification's scope regardless.)

## Grep gates, per artifact's own `🔺️diff/🦀️component.rs`

### gif 87a (`🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)
- `impl DiffAlgebra<GifSnapshot> for GifDiff` — present, line 417.
- `snapshot: Option<` — **zero struct-field hits** (only 2 doc-comment mentions describing what
  was deleted, lines 3 and 355).
- `field_sweep` — present: `field_sweep_covers_every_mutable_field` (line 569), passing.

### gif 89a (`🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)
- `impl DiffAlgebra<GifSnapshot> for GifDiff` — present, line 633.
- `snapshot: Option<` — **zero struct-field hits** (only 2 doc-comment mentions, lines 3 and 548).
- `field_sweep` — present: `field_sweep_covers_every_mutable_field` (line 799), passing.
- **Old op-slot shape confirmed GONE**: read the actual `pub struct GifDiff { ... }` definition
  (line 552-580) directly — it is the sparse-triple shape (`width/height/gct/
  background_color_index/pixel_aspect_ratio/loop_count: Option<...>` scalars +
  `frames/comments/app_extensions: Option<XsDiff>` collection triples). Grepped for every old
  op-slot field name (`insert_frame`, `remove_frame_at`, `set_frame_delay`, `set_loop_count`,
  `set_frame_disposal` as struct fields) — **zero hits**. The flat per-mutation-kind `Option<T>`
  field shape the closer report described as still present is gone.
- **3 canonical absorb tests — confirmed real, passing, not just documented**:
  - `absorb_insert_then_remove_before_shifts_index` (Insert+Remove-before) — pass.
  - `absorb_insert_insert_same_index_both_survive` (Insert+Insert-same-index, both survive) —
    pass. This is the exact case the old LWW-based absorb could not handle.
  - `absorb_insert_then_set_field_patches_into_added` (Insert+SetField patches into added) —
    pass.
  - Same 3 test names, same pass status, also present and passing in **87a's** diff file (its own
    equivalent trio over `images` instead of `frames`).

### png (`📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)
- `impl DiffAlgebra<PngSnapshot> for PngDiff` — present, line 908.
- `snapshot: Option<` — zero struct-field hits (1 doc-comment mention, line 3).
- `field_sweep` — **not** in the diff file itself; lives in the sibling
  `🧬️mutations/🦀️component.rs` as `field_sweep_covers_every_mutable_field` (line 498, inside a
  `//#region 🔖️field_sweep` block) — present and passing. (The brief's grep target was the diff
  file specifically; png's own report already flagged this file-location choice, and it's a
  reasonable one — the mutation-level sweep exercises the diff type through
  `apply_png_mutation`.)

### md (`📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)
- `impl DiffAlgebra<MdSnapshot> for MdDiff` — present, line 355.
- `snapshot: Option<` — **zero hits of any kind** (not even a doc-comment mention).
- `field_sweep` — not in the diff file; lives in
  `🏅️standards/🔖️commonmark/⚙️engine/🦀️component.rs` as `field_sweep_covers_every_mutable_field`
  (line 1301) — present and passing.

### dxf (`🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)
- `impl DiffAlgebra<DxfSnapshot> for DxfDiff` — present, line 1087.
- `snapshot: Option<` — zero struct-field hits (1 doc-comment mention, line 3). This directly
  contradicts the earlier `f3-closer-report.md`'s finding of a "pristine pre-overhaul scaffold"
  `DxfDiff{snapshot: Option<DxfSnapshot>}` — that finding predates the (now-present)
  `f3-dxf-report.md` mop-up pass; current disk state is the full rewrite.
- `field_sweep` — not in the diff file; lives in the sibling
  `🧬️mutations/🦀️component.rs` as `field_sweep_every_mutable_field_changes` (line 503) — present
  and passing.

## Conclusion

All 4 artifacts (gif 87a, gif 89a, png, md, dxf) independently confirmed on disk, right now, in
this session:
- Real `impl DiffAlgebra<XSnapshot> for XDiff` present in every diff module.
- No `snapshot: Option<XSnapshot>` full-replace slot as a struct field anywhere (doc-comment
  mentions of the deleted shape don't count and were checked by hand, not just grep count).
- A real `field_sweep`-named test present and passing for every artifact (3 of the 4 keep it in a
  sibling mutations/engine file rather than the diff file itself — confirmed by direct inspection,
  not just assumed from the brief's suggested grep target).
- gif 89a specifically: the old flat op-slot `GifDiff` is fully replaced by the sparse-triple
  shape, and all 3 canonical absorb cases from the brief are real, passing unit tests in both gif
  standards (not just prose in a doc comment).
- The `💃️dancing` gif fixture passes (4/4) standalone.
- Whole-crate suite: 853 passed, 0 failed, no regressions anywhere.

No discrepancies found between any fan-out agent's self-report and the actual state of the code.
