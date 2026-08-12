# W5b Close Report

Closer: W5b closer agent. Scope: fix cheap/safe verifier-flagged issues directly, consolidate
`stdio_gaps`, run the full cross-plugin gate, document design-judgment follow-ups, append a
STATUS.md entry. All commands below were re-run by this agent, not taken on any prior agent's word.

## 0. Starting point

Read `w5b-verify-report.md` (verdict: **FAIL — not ready to close**, 2/8 reports permanently lost to
a filename collision, 3/8 crates failing to compile, 1/8 compiling-but-test-failing, 3/8
foreign-blocked) and all 8 available build reports (`w5b--report.md` = 🗒️note's, recovered
`🌀️procedural` report from the git index via `git show :"...w5b--report.md"`, `w5b-layout-report.md`,
`w5b--gis-report.md`, `w5b--puzzle-report.md`, `w5b-w-report.md` = 🖍️draw's; 🎥️shooting and 🖨️raster
have **no recoverable report**, confirmed independently — not in the working tree, git index, or
`git log`).

## 1. Fixes applied (cheap/safe, mechanical — not design judgment)

All fixes below are single-file or few-line, follow an already-established pattern at least one
sibling W5b plugin had already applied, and were verified by a real `cargo check`/`cargo test` run
after each change (never asserted without running).

| # | Plugin | File(s) | Fix |
|---|---|---|---|
| 1 | 🗒️note | 2× json io leaves | `STDIO_JSON_DOCUMENT_SCHEMA` unresolved import (`E0432`) — the constant lives in `artifacts::json` (top-level), not re-exported from `artifacts::json::schema::snapshot`. Split into two `use` statements. |
| 2 | 🎥️shooting | `📦️glue.rs` | Stale `#[path]` mount `📌️panels/📄️document/…` → real dir is `📄️artifact/` (same repo-wide rename every sibling plugin already fixed). |
| 3 | 🎥️shooting | 2× json io leaves | `JsonSnapshot.value` retype fallout, surfaced only after fix #2 unblocked the module tree. Mirrored 🗒️note's converter pattern. |
| 4 | 🖨️raster | `📦️glue.rs` | Same stale `panels::document`→`artifact` `#[path]` mount as shooting. |
| 5 | 🖨️raster | `🎛️apps/🖨️raster/📌️panels/📄️artifact/🦀️component.rs` | `FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL` renamed repo-wide to `_ARTIFACT_ID`/`_LABEL`; raster's artifact panel was the one file that never got updated (surfaced after fix #4). |
| 6 | 🖨️raster | 2× json io leaves | Same `JsonSnapshot.value` retype fallout as shooting. |
| 7 | 🖨️raster | `⚙️engine/🦀️component.rs` | Missing `Once`-guarded stdio composer registration — `io_dispatch` calls had no registry entry in a bare `cargo test` process (same class of gap the verifier flagged for 🖍️draw, §2 below; raster had it too, just hadn't compiled far enough for the verifier to see it). |
| 8 | 🖨️raster | `⚙️engine/🦀️component.rs` (`dispatch_drawing_to_svg`) | **New finding, not in any report or the verify report**: used `ArtifactDsl::print_dsl` (wraps the `.semio` envelope preamble) instead of `write_svg_xml` (bare XML) for its bridged SVG text. Harmless for note/gis/draw (nothing downstream strictly parses their SVG output), but raster's own `raster_document_json_from_dwg` feeds this string straight into `semio_framework_os::rasterize_svg_to_png_base64` (a real usvg-based XML parser), which failed with `"unknown token at 1:1"` once composer registration (fix #7) let the call actually reach it. Switched to `write_svg_xml`, matching 🗒️note's/🖍️draw's own usage of the same bridge. |
| 9 | 🖍️draw | `⚙️engine/🦀️component.rs` | Missing `Once`-guarded stdio composer registration (verifier §6b, confirmed: `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing` and `draw_io_declares_vector_out_and_export_media_covers_both_ports` failed with `"no composer registered"`). Added `ensure_semio_drawing_bridge_registered()`, called unconditionally from `draw_document_to_svg` (mirrors 🗒️note's own production-code placement, not test-only). |
| 10 | 🌀️procedural | 4× json io leaves (`procedural2d`×2, `procedural3d`×2) | Same `JsonSnapshot.value` retype fallout procedural's own report flagged as a `stdio_gap` it declined to work around unilaterally. Once note/gis/shooting/raster had all independently landed the identical mirrored-converter pattern, applying the same established pattern here is mechanical, not a stdio-side design call. |
| 11 | 📏️layout | 2× json io leaves | Same `JsonSnapshot.value` retype fallout. |
| 12 | 📏️layout | dwg export leaf | `E0063` missing `codepage`/`maintenance_version` fields on `DwgSnapshot` — added as honest `0` (stdio's own `DwgSnapshot::default()` also zeroes both; this leaf already emits synthetic JSON-as-DWG-bytes with `DwgDecodeStatus::SentinelOnly`, so zero header fields are consistent with the leaf's existing honesty, not new fabrication). |

All 12 fixes were verified with a real `cargo check` (and `cargo test` where the crate compiles)
immediately after the edit, not batched and asserted at the end.

## 2. Design-judgment issues — NOT fixed, documented as follow-ups

- **📏️layout's pdf io leaves (3 remaining errors)**: stdio's `PdfSnapshot` was restructured from a
  single `page: PageDoc{width,height,text}` field to `pages: Vec<PdfPage>` (a real multi-page
  model). This is not a rename — layout's `LayoutSnapshot` (a single flat page list of its own) needs
  a real design decision for how it maps onto a `Vec<PdfPage>`. Layout's own report called this out
  explicitly ("real design work, not a one-line fix... squarely inside `✏️s/🔌️plugins/📏️layout/**`
  so any future agent can do it without touching stdio") and this closer agrees — left unfixed.
  **Follow-up**: a future session should design the `LayoutSnapshot ↔ Vec<PdfPage>` mapping (both
  directions) in `🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import,📤️export}/…/📄️pdf/…`.
- **📏️layout's other scene exporters** (`export_document_png_cpu`, `export_document_pdf`,
  `scene_png_from_display_list`) remain hand-rolled (raw PNG raster fill, hand-built PDF byte
  stream). Out of W5b's named scope (only the two `*_document_json_to_svg`/`*_json_from_dwg`-shaped
  functions were targeted), but stdio does have real `drawing↔pdf` and `image↔png` bridges a future
  wave could route these through, the same way `export_display_list_svg` now routes through
  `drawing↔svg`. **Follow-up**, not a defect.

No other design-judgment gaps were found blocking compilation once the mechanical fixes above
landed — the remaining 7/8 plugins are fully green (see §4).

## 3. Consolidated `stdio_gaps` (from all 8 plugins' reports/recon, deduplicated)

1. **No `s.stdio.semio/v1/drawing ↔ dwg` bridge in stdio.** Reported independently by 🗒️note,
   📏️layout, 🌍️gis, and 🖍️draw (4/8). The drawing subset's io tree only bridges `svg`/`dxf`/`pdf`;
   DWG bridges through the separate `cad` subset (`ac1024`) instead, per the master plan's own
   format lattice — this is architecturally expected, not a defect, but it means DWG *import* could
   not be rewired onto `io_dispatch` in any of the 4 plugins that hit it. Each kept an honest,
   non-hand-rolled fallback instead (direct `DwgGeometry`→domain-type mapping in note/layout/gis;
   an honest stub with no replacement in draw, since draw's own `ac1018` DWG dialect is a third,
   distinct standard from the `cad` subset's `ac1024`, making even a future 2-hop route
   unreachable without draw switching dialects — a bigger change than this ticket's scope).
2. **`stdio::artifacts::json::JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
   own lexeme-preserving `JsonValue` enum with zero conversion helpers exported** (no `From`/`Into`,
   no `to_serde_json`/`from_serde_json` — confirmed by grep, zero hits anywhere in stdio).
   🌀️procedural's report was the one that named this explicitly as a `stdio_gap` and declined to
   work around it unilaterally ("the conversion belongs in stdio or a shared helper, not
   copy-pasted per plugin"), but by the time this closer ran the gate, **7 of 8 plugins**
   (🗒️note, 🌍️gis, 🎥️shooting, 🖨️raster, 🌀️procedural, 🖍️draw indirectly via its own leaves, 📏️layout)
   had each independently hand-written the identical recursive `JsonValue ↔ serde_json::Value`
   converter as a per-file mirrored fix. **Recommendation carried forward from procedural's report,
   endorsed by this closer**: stdio should export one shared, documented conversion helper (or a
   real `From`/`Into` impl pair) instead of this pattern being copy-pasted a 7th+ time by every
   future consumer.
3. **`DrawNode::Text` has no font-size/font-weight field.** Reported by both 🗒️note and 🖍️draw
   independently. Real, inherited fidelity loss on SVG export — per-block/per-layer text styling
   cannot round-trip through the semio/drawing subset today.
4. **`DrawStyle` has no `blend_mode`/`fill_rule`, and `Group`/`Image` nodes carry no opacity slot
   at all** (🖍️draw). Per-layer blend mode, fill rule, and image/group opacity are honestly dropped
   crossing the bridge, not fabricated.
5. **stdio's `PdfSnapshot` shape change (`page: PageDoc` → `pages: Vec<PdfPage>`) has no
   migration path for consumers** (📏️layout) — see §2 follow-up above. Same *class* of gap as #2
   (a stdio schema change landing with no consumer-facing bridge), but structurally different
   (real multi-page redesign, not a value-representation swap) and not mechanically fixable the
   way #2 turned out to be.

## 4. Full cross-plugin gate (all commands re-run fresh by this closer)

The repo was under **live, concurrent, unrelated churn twice** during this gate — once in stdio's
`brep` subset (grammar/protocol/spicy regeneration, 25 dirty files, resolved after ~9 min of
polling) and once in stdio's `semio/drawing` binary snapshot reader (`read_f32_le` method renamed
mid-edit, error signature changed on every retry, resolved after ~2 min of polling). Both were
confirmed foreign via `git status` dirtiness on files this ticket never touches, matching the
repo's known "Concurrent Cargo Workspace Churn" pattern — polled rather than chased, per standing
guidance. All numbers below are from the **final, settled, stable** state.

**`cargo check -p semio-s-plugin-stdio --lib`**: clean, 0 errors (483 pre-existing warnings).

**`cargo test -p semio-s-plugin-stdio --lib`**: **1869 passed, 0 failed, 3 ignored.** (W4-closer
baseline: 1657/0/1. Verifier's own run mid-churn: 1839/5/4 — the 5 pptx/workflow conformance
failures the verifier saw were themselves foreign-churn artifacts and have since resolved as that
concurrent work landed; fully green now.)

**`bun ./📜️script.ts policy`**: **21651 high-priority breaches across 26 rules** (final run, after
all fixes above). W4-closer baseline: 21532. Verifier's mid-churn run: 21610/25 rules. Grepped the
full breach list for every file this closer edited (12 files, §1) — **zero breaches attributable to
any of them**. The remaining ~119-breach drift from the W4 baseline traces to the same ongoing
concurrent repo-wide churn documented above (this session alone observed two live foreign edits in
under 20 minutes), not to W5b. Matching the verifier's own recommendation, a true isolated diff
would need a policy snapshot taken at a moment of full repo quiescence, which was not available in
this session either.

**`cargo check -p <plugin-crate>` — per plugin, final state:**

| Plugin | Crate | Result |
|---|---|---|
| 🗒️note | `semio-s-plugin-note` | **PASS** (0 errors, 5 pre-existing warnings) |
| 📏️layout | `semio-s-plugin-layout` | **FAIL — 3 errors**, all in the pdf io leaf, genuine design-judgment gap (§2), not fixed |
| 🌍️gis | `semio-s-plugin-gis` | **PASS** (0 errors, 15 pre-existing warnings) |
| 🎥️shooting | `semio-s-plugin-shooting` | **PASS** (0 errors, 9 pre-existing warnings) |
| 🌀️procedural | `semio-s-plugin-procedural` | **PASS** (0 errors, 36 pre-existing warnings) |
| 🖨️raster | `semio-s-plugin-raster` | **PASS** (0 errors, 9 pre-existing warnings) |
| 🖍️draw | `semio-s-plugin-draw` | **PASS** (0 errors, 4 pre-existing warnings) |
| 🧩️puzzle | `semio-s-plugin-puzzle` | **PASS** (0 errors, 88 pre-existing warnings) — the foreign `dsl_derive`/`os_spr`/`os_store` churn the verifier saw blocking puzzle has since quiesced |

**`cargo test -p <plugin-crate> --lib` — for every crate that compiles:**

| Plugin | Result |
|---|---|
| 🗒️note | **71 passed, 0 failed** |
| 🌍️gis | **155 passed, 0 failed** |
| 🎥️shooting | **92 passed, 0 failed** |
| 🌀️procedural | **191 passed, 0 failed** |
| 🖨️raster | **55 passed, 0 failed** |
| 🖍️draw | **88 passed, 0 failed** (including both tests the verifier caught failing — `draw_document_to_svg_bridges_…` and `draw_io_declares_vector_out_…` — now passing) |
| 🧩️puzzle | **421 passed, 0 failed** |
| 📏️layout | not runnable — `cargo check` fails upstream (§2) |

**Net: 7/8 plugins fully green (compile + test, 0 failures each); 1/8 (layout) blocked on a real,
documented, out-of-mechanical-reach schema-design gap in stdio's `PdfSnapshot`, not a W5b defect.**

## 5. Files touched by this closer

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/📌️panels/📄️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`

Log files written to this ticket folder: `w5b-close-cargo-check-stdio.txt`,
`w5b-close-cargo-test-stdio.txt`, `w5b-close-policy.txt` / `w5b-close-policy-full.txt` /
`w5b-close-policy-final.txt`, `w5b-close-cargo-check-{note,layout,gis,shooting,procedural,raster,draw,puzzle}.txt`,
`w5b-close-cargo-test-{note,gis,shooting,procedural,raster,draw,puzzle}.txt`.

## 6. Recommendation for next steps

1. **📏️layout's pdf io leaves** need a real `LayoutSnapshot ↔ Vec<PdfPage>` design (§2) — assign as
   a small standalone follow-up ticket/task inside `✏️s/🔌️plugins/📏️layout/**`, no stdio changes
   needed.
2. **stdio should export a shared `JsonValue ↔ serde_json::Value` conversion helper** (§3, gap #2)
   — 7 of 8 W5b plugins independently reinvented the identical converter; a single shared helper
   would remove that duplication for this and future consumers.
3. Going forward, per the verify report's own recommendation: every wave's task instructions should
   embed each agent's own name in any shared-looking output filename (the root cause of §0's lost
   shooting/raster reports).
