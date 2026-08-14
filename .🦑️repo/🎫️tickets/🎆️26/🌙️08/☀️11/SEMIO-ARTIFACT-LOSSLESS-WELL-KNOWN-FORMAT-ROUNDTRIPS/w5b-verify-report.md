# W5b Independent Verification Report

Verifier: W5b independent verification agent. Everything below was checked from disk (grep,
`git status`/`git log`, direct file reads, and fresh `cargo check`/`cargo test` runs executed by
this agent), not taken on the 8 implementers' word.

## 0. Report-integrity incident (found before any code review)

The task pointed at 8 expected files named `w5b-{note,layout,gis,shooting,procedural,raster,draw,puzzle}-report.md`.
Only 5 report files actually exist on disk under those or similar names, and two of the eight
plugins (**shooting**, **raster**) have **no report at all, anywhere** — confirmed by a recursive
`find` for `*shoot*`/`*procedural*`/`*raster*` in the ticket folder, which returned nothing for
shooting/raster.

Root cause, reconstructed from `git status`/`git show :<path>` (index vs. working tree):

- The task template's literal filename `w5b--report.md` was used **unqualified** (no plugin name)
  by at least 3 of the 8 agents (🗒️note, 🌀️procedural, and almost certainly 🎥️shooting and/or
  🖨️raster), so each one clobbered the previous one's write to the same path. `git status` shows
  this file as `AM` (staged-then-modified), and `git show :w5b--report.md` (the **staged**, i.e.
  earlier, version) is 🌀️procedural's report, while the **working-tree** version (what a naive
  reader would see) is 🗒️note's report — a different plugin entirely. I recovered 🌀️procedural's
  report from the git index (`git show :"...w5b--report.md"`); it is captured below. 🗒️note's own
  cargo-check/test logs (`w5b--cargo-check.txt`/`w5b--cargo-test.txt`, also unqualified names)
  turned out on inspection to actually be 🌀️procedural's logs (content matches procedural's
  reported error signature), so 🗒️note's own raw exit-checklist logs are **also gone** — I re-ran
  `cargo check -p semio-s-plugin-note` myself instead of trusting the ticket-folder file.
- **🎥️shooting and 🖨️raster's reports are unrecoverable** — not in the working tree, not in the
  git index, not anywhere in `git log` (nothing was ever committed this session). `git status`
  confirms both plugins have real, substantial code changes (`git status --porcelain` shows `M` on
  each engine file), so the work happened — only the first-person report/log narrating it is lost.
  Sections 1–7 below verify both of these two plugins **from the diff and current source directly**,
  with no report to cross-check against.
- 📏️layout, 🌍️gis, 🧩️puzzle avoided the collision by writing to `w5b--<plugin>-report.md`/
  `w5b-<plugin>-report.md`; 🖍️draw used `w5b-w-report.md` (idiosyncratic but collision-free, since
  `w5b-w-*` is not the shared literal name). Only the literal unqualified name collided.

**This is a real process failure the closer needs to know about**, independent of whether the
underlying code is good: 2 of 8 agents' first-person accounts are permanently lost, and a 3rd
agent's raw log evidence is silently wrong (procedural's logs mislabeled as note's).

## 1–5. Per-plugin findings (code-level, from disk)

| # | Plugin | `io_dispatch`/`io_compose_via` real? (not disguised hand-roll) | Ad-hoc SVG/DWG code actually gone? | Stub/gap honesty | Report exists? |
|---|---|---|---|---|---|
| 1 | 🗒️note | **YES** — `note_document_to_svg` (engine/component.rs:722) packs a real `SemioDrawingSnapshot`, calls `semio_framework_plugin::io_dispatch`, decodes `SvgSnapshot`. 10 real hits. | **YES** — `note_block_to_svg`/`escape_svg_text` zero hits anywhere in the plugin (only mentioned in doc comments explaining the deletion). | DWG import kept as a direct `DwgGeometry→NoteBlock` mapper (not routed through `io_dispatch`) — documented, honest reason: no `drawing↔dwg` stdio bridge exists, and routing it through a synthetic detour would drop DWG `TEXT.height`. Legitimate design call, not a disguised stub. | Working-tree copy of `w5b--report.md` (clobbered filename, see §0) |
| 2 | 📏️layout | **YES** — `compose_svg_from_drawing` (engine/component.rs:411) calls `io_dispatch`; `export_display_list_svg` in `engine/scene` routes through the same bridge. 7 real hits. | **YES** — `semio_framework_os::pages_rects_svg` (the framework hand-roller) has zero remaining call sites in this plugin (`export_display_list_svg`'s and `layout_document_json_to_svg`'s old hand-rolled bodies are gone). | DWG import path kept off `io_dispatch` (documented: no `drawing↔dwg` stdio bridge), but its output is now expressed as a real `SemioDrawingSnapshot`/`DrawNode` tree via `dwg_drawing_to_semio_drawing`, not raw tuples. Honest. | `w5b-layout-report.md` |
| 3 | 🌍️gis | **YES** — `render_drawing_to_svg` calls `io_dispatch` with an explicit `IoKey`; verified end-to-end by a real passing test (`svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge`). 6 real hits. | **YES** — `semio_framework_os::map_points_svg` has zero remaining call sites. | DWG entities genuinely lowered through `DrawNode::Path`/`PathSegment` (`dwg_geometry_to_draw_node`) — real mapping, not a stub; import stays off `io_dispatch` for the same documented stdio-gap reason as note/layout. | `w5b--gis-report.md` |
| 4 | 🎥️shooting | **YES**, verified directly in source — `shooting_drawing_to_svg_text` (engine/component.rs:294) builds a real `IoKey`/`ErasedComposeSource` and calls `semio_framework_plugin::io_dispatch`, decodes `SvgSnapshot`. 2 real hits (the low count is because shooting is a smaller, single-direction rewire — not evidence of a fake). | N/A — shooting's original pattern was a "real leaf, honest DWG stub" per w0 recon, not an ad-hoc SVG builder; the SVG side is confirmed genuinely rewired onto the bridge (see above). | **CONFIRMED still honestly stubbed**: `shooting_document_json_from_dwg` (line 389) takes `_drawing` (unused), always returns `default_snapshot()`, doc comment explicitly explains why (no wall/obstacle concept in this format) — unchanged, not silently deleted, not dishonestly "completed." | **NONE — lost, see §0** |
| 5 | 🌀️procedural | **Correctly 0 hits** — this plugin's task was pure deletion, not migration (recon and the (recovered) report both say so, and the code confirms it: no `MediaFormat`/registry entry ever called the two deleted functions). | **YES, confirmed deleted** — `procedural2d_document_json_to_svg`/`procedural2d_document_from_dwg` (the `title_card_svg`/`default_snapshot()` stub pair) have zero remaining references anywhere in the plugin; no dangling `register_dwg_import_handler`/`register_svg_export_handler` call sites either (grep: zero hits — they were never registered to begin with). | Deletion is honest — not fabricated into fake real content. Real generatable content elsewhere in the plugin (`generation_preview_layers` etc.) was correctly identified as already exposed through a separate, already-real app-level port and correctly left untouched (out of scope, not a duplicate). | Recovered from **git index only** (working tree was clobbered by 🗒️note, see §0) |
| 6 | 🖨️raster | **YES**, verified directly in source — real `io_dispatch`/`io_compose_via` (12 hits) including a genuine 2-hop `io_compose_via` seam; `raster_document_json_from_dwg` builds a real `DrawNode` tree from `DwgGeometry` via `drawing_snapshot_from_dwg` (not a stub — real geometry, 2 passing unit tests over polyline/empty-DWG cases in the source). | N/A — raster's own task was framework `title_card_svg` placeholder replacement + real usvg/resvg-based rasterization behind the bridge; confirmed via doc comments ("no more `title_card_svg` placeholder", "never a hand-rolled PNG writer"). | No stub to check (raster's DWG import is real content, not a Tier-C stub, by design — raster genuinely rasterizes). | **NONE — lost, see §0** |
| 7 | 🖍️draw | **YES** — `draw_document_to_svg` (engine/component.rs:1288) builds a `SemioDrawingSnapshot`, dispatches via `semio_framework::io_dispatch`, decodes `SvgSnapshot`. 5 real hits. | **YES, confirmed repo-wide** — `grep -rn` across the **entire repo** (not just draw) for `rgba_to_svg_color`/`path_segments_to_svg_d` returns **zero hits anywhere**. `draw_document_to_svg` as a *name* still exists, but its *body* is now the real bridge call (verified by reading it) — this matches the report's claim precisely: the function name was kept, its ad-hoc implementation was not. | DWG has **no replacement at all** (deleted outright, no stdio bridge exists, honestly reported as a real gap against the master plan's own acceptance scenario). | `w5b-w-report.md` |
| 8 | 🧩️puzzle | **YES** — `puzzle2d_document_json_to_svg` rewritten to dispatch via `io_dispatch`. 3 real hits. | **8/8 JsonCodec leaf files confirmed deleted** — checked every one of the 8 paths named in the report individually: all gone. `grep -rln "JsonCodec"` across the whole puzzle plugin: **zero hits**. | **CONFIRMED still honestly stubbed**: `puzzle2d_document_json_from_dwg` (line 1354) still returns `default_empty_fixture()` unconditionally, unchanged from before — not silently deleted, not dishonestly completed. | `w5b--puzzle-report.md` |

## 6. `cargo check -p <crate>` — run fresh by this verifier (not reused from any report)

| Plugin | Crate | Result |
|---|---|---|
| 🗒️note | `semio-s-plugin-note` | **FAIL** — 2× `E0432` unresolved import `STDIO_JSON_DOCUMENT_SCHEMA` in the plugin's own json import/export leaves (its "incidental fix" for the stdio JSON-shape drift). Real bug, not foreign: it imports `STDIO_JSON_DOCUMENT_SCHEMA` from `semio_s_plugin_stdio::artifacts::json::schema::snapshot::` — but that constant is actually defined and only ever `pub`-exported from `artifacts::json` (top-level `component.rs`), not re-exported from the `schema::snapshot` submodule. Confirmed this isn't fresh churn: the defining file (`🗿️artifacts/🔣️json/🦀️component.rs`) and the snapshot file are both **git-clean**, last touched by old, already-landed commits (`449c584855`, `ad0fc0019b`) — not concurrent edits. This contradicts the note report's "zero errors ever traced to any 🗒️note file" claim, though the report never actually got past the (separately real) foreign os-kernel blocker to see it. |
| 📏️layout | `semio-s-plugin-layout` | **BLOCKED — foreign, confirmed.** 2× `E0277` in `🧰️framework/…/🔨️modules/💡️inference/🦀️component.rs`. That module directory is `??` (untracked, brand-new) in `git status` — an in-progress foreign feature, not layout's code, not even the same foreign blocker layout's own report described (that one's since moved on). Zero errors in layout's own files. |
| 🌍️gis | `semio-s-plugin-gis` | **PASS.** `Finished 'dev' profile … in 2m 47s`, 18 pre-existing warnings, 0 errors. |
| 🎥️shooting | `semio-s-plugin-shooting` | **FAIL** — `couldn't read …/🎛️apps/🎥️shooting/📌️panels/📄️document/🦀️component.rs: No such file or directory`, from a stale `#[path]` mount in `glue.rs:613`. This is the **exact same** stale `document→artifact` rename every other plugin's agent independently found and fixed in its own write scope — but shooting has no report, and the fix was never made here. Blocks the crate from compiling at all. |
| 🌀️procedural | `semio-s-plugin-procedural` | **BLOCKED — foreign, confirmed.** Identical 2× `E0277` in the same untracked `💡️inference` module as layout. Zero errors in procedural's own files. |
| 🖨️raster | `semio-s-plugin-raster` | **FAIL** — identical stale `#[path]` glue.rs bug as shooting (`…/🎛️apps/🖨️raster/📌️panels/📄️document/🦀️component.rs: No such file or directory`, `glue.rs:510`). Same root cause, same fix every other plugin applied — never fixed here, no report to explain why. |
| 🖍️draw | `semio-s-plugin-draw` | **PASS.** `Finished 'dev' profile … in 2m 11s`, 4 pre-existing warnings, 0 errors. Matches the report's claim. |
| 🧩️puzzle | `semio-s-plugin-puzzle` | **BLOCKED — foreign, confirmed.** Fails deeper in the dependency graph now (`semio-framework-schema`, `E0004` non-exhaustive `StateClass::Inferred` match) — a **different** foreign error than the report described (`dsl_derive`), consistent with the report's own prediction that the foreign churn was still moving. `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs` (where `StateClass` lives) is currently `M` (dirty) in `git status` — confirmed live foreign edit, not puzzle's fault. |

**Net: 2/8 clean PASS (gis, draw), 3/8 genuinely foreign-blocked and unattributable to the agents
(layout, procedural, puzzle), 3/8 real FAIL (note, shooting, raster) — two of which (shooting,
raster) are the exact same one-line stale-glue.rs bug every other agent independently caught and
fixed, just never applied here because no closer/report ever touched these two plugins.**

## 6b. `cargo test -p <crate> --lib` — run fresh where check passed

| Plugin | Result |
|---|---|
| 🌍️gis | **PASS — 151 passed, 0 failed**, including both new bridge-exercising tests (`svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge`, `dwg_import_lowers_a_closed_polyline_...`). Matches the report's claimed numbers exactly. |
| 🖍️draw | **FAIL — 78 passed, 2 failed.** Both failures: `"no composer registered for s.stdio.semio/v1/drawing Export s.stdio.svg/1.1/*"` — `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing` (the ported svg test) and `draw_io_declares_vector_out_and_export_media_covers_both_ports`. Root cause: unlike note/layout/gis/shooting/puzzle, **draw never added a `std::sync::Once`-guarded `ensure_..._registered()` helper** to seed stdio's composer registry inside a bare `cargo test` process (every sibling plugin that ported a bridge test added exactly this). The report itself flagged this exact risk ("has not been machine-verified by an actual compiler run... should be re-run once the framework churn settles") — that re-run now shows the gap is real, not hypothetical. |

The other 6 crates' tests could not be run (5 blocked upstream of their own `cargo check` failure/foreign-block; blocked crates never reach their own test binary).

## 7. Cross-cutting gates

**`cargo test -p semio-s-plugin-stdio --lib`**: **1839 passed, 5 failed, 4 ignored** (this verifier's own fresh run). W4-closer baseline was 1657 passed / 0 failed / 1 ignored. All 5 failures are in `pptx`/`workflow` conformance-law tests
(`artifacts::pptx::…::fixture_honesty_law`, `artifacts::semio::…::workflow::…::{committed_facet_files_parse,fixture_honesty_law,ops_grammar_conformance_law,protocol_walk_law}`)
— **confirmed foreign**: `git status --porcelain` on both artifact trees shows dozens of `M`
files (grammar/protocol/spicy component files actively mid-regeneration), unrelated to any of the
8 W5b plugins, which are all stdio-**read-only** by every report's own scope statement (verified —
`git status` shows zero W5b-attributable changes under `✏️s/🔌️plugins/🗄️stdio/`). Not literally
byte-identical to baseline (task said "must stay green/unchanged"), but the deviation traces
cleanly to unrelated concurrent stdio work, not to W5b.

**`bun ./📜️script.ts policy`**: **21610 high-priority breaches across 25 rules** (fresh run). W4-closer
baseline was 21532. **+78, not zero-delta.** I could not fully attribute this delta in the time
available — the repo is under heavy concurrent multi-wave edit (this same session also landed W5a's
norm/animate/fem waves, which touch the same policy surface), and a clean isolated before/after
diff would need a policy snapshot taken immediately pre-W5b, which no W5b agent captured. Flagging
this open rather than asserting it's benign: **the closer should re-run policy once the repo
quiesces and diff against a true immediately-pre-W5b snapshot**, not accept this number at face
value.

## Overall verdict: **FAIL — not ready to close**

Reasons, in priority order:

1. **2 of 8 agents' reports are permanently lost** to a filename collision (§0) — a process
   failure independent of code quality, but it means this ticket currently has no first-person
   account of what shooting's/raster's agents actually did or why, only what this verifier could
   reconstruct from the diff.
2. **3 of 8 crates fail to compile for real, plugin-owned reasons**: note (bad import path in its
   own incidental json fix) and shooting+raster (the same stale `glue.rs` `document→artifact`
   path every sibling plugin fixed, left unfixed here because no report/closer ever touched these
   two).
3. **1 of the 2 crates that do compile (draw) fails its own test suite** (2/80 failing) for a
   missing-registration bug the report itself predicted might exist and asked to be re-verified.
4. Only **🌍️gis** is fully green end-to-end (check + test) and fully matches its own report's
   claims verbatim.
5. The 3 foreign-blocked crates (layout, procedural, puzzle) are legitimately not these agents'
   fault — confirmed via `git status` dirtiness on the exact blocking files — and should simply be
   re-run once the concurrent framework/inference/dsl-derive churn quiesces, per every one of
   their own reports' recommendations.
6. Where reports do exist and code could be checked, the underlying **svg/dwg → `io_dispatch`
   rewiring claims are real** in all 8 plugins (never a disguised local hand-roll), the ad-hoc
   draw SVG/DWG writer functions named in the recon are genuinely gone repo-wide, procedural's
   stubs are honestly deleted, shooting's and puzzle's DWG stubs are honestly still stubs, and
   puzzle's 8 JsonCodec leaves are honestly gone. The **substance** of the migration work is good;
   the **exit-gate discipline** (compile, test, report) is not, for 6 of the 8 plugins right now.

**Recommended next steps for the closer**: (a) fix note's one-line import path and shooting's/
raster's one-line stale glue.rs path — all three are mechanical, single-file fixes, same shape as
every other plugin's own "incidental fix"; (b) add draw's missing `Once`-guarded stdio-composer
test registration (copy the pattern from note/layout/gis/shooting's own engine files); (c) re-run
`cargo check`/`test` for layout/procedural/puzzle once `🔨️modules/💡️inference`, `📡️spr`, and
`🗣️dsl/✨️derive` git-clean (poll, don't chase, per standing guidance); (d) get a real before/after
policy diff once the repo quiesces; (e) going forward, every wave's task instructions should embed
each agent's own name in any shared-looking output filename to prevent recurrence of §0.
