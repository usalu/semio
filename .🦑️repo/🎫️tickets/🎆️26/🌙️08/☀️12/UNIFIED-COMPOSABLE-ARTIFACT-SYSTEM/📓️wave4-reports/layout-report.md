# W4 (batch C) — `layout` composes stdio `drawing`, references `model`

**ucas-status: complete — 140/140 tests, 0 failures, reproduced stable across 2 independent foreground `cargo nextest run` passes; `cargo check -p semio-s-plugin-layout --all-targets` clean (0 errors).**

## What layout duplicated today (read first, before the change)

Read `🗿️artifacts/📏️layout/🦀️component.rs` and `📸️snapshot/🦀️component.rs` before touching anything, per the brief. Findings:

- **No literal `model`-shaped duplication exists anywhere in this plugin** — confirmed by grep across the whole plugin before starting (the only "model" hits were `ViewModel`, an unrelated framework type). The design map's `layout | C:drawing R:model` row's `R:model` half is therefore genuinely **new forward capability**, not a duplication removal — documented as such rather than invented as if it replaced something.
- **The real duplication was in `🚪️io/🦀️component.rs`**: `layout_snapshot_to_semio_drawing` already builds a real `SemioDrawingSnapshot` from the document's pages/frames for SVG export (fine, stays as the export-direction converter) — but `dwg_drawing_to_semio_drawing`/`layout_document_json_from_dwg` (the *import* direction) built a full `SemioDrawingSnapshot` from a decoded DWG file **only to read `path_bounds` back out of it for page framing, then discarded the rest of the imported geometry entirely.** That's the actual "content that should be composed but wasn't" this migration fixes.

## What changed

**Schema** (`🧬️schema/📸️snapshot/🦀️component.rs`, `🧬️schema/🦀️component.rs`, `🧬️schema/🔺️diff/🦀️component.rs`):
- `LayoutSnapshot` gained two persistent fields: `background_drawing: Option<store::ArtifactChild<SemioDrawingSnapshot>>` (`#[child(kind = "s.stdio.semio.drawing")]`) and `referenced_model: Option<store::ArtifactLink>` (`#[link_slot(roles("model"))]`). Type alias `LayoutDrawingChild` lives in the artifact root's new `🔖️ComposedTypes` region.
- Same two fields added to `LayoutArtifact` (full artifact state) and `LayoutDiff` (as `Option<Option<T>>`, per the recipe's §8 optional-slot diff-shape convention) — `to_snapshot`/`from_snapshot`/`set_snapshot`/`apply_to_artifact`/`MutationDiff::apply`/`absorb` all updated.
- Dropped the `dsl::DslArtifact` derive from `LayoutSnapshot` (kept `ArtifactSchema`, needed for the `#[child]`/`#[link_slot]` slot-table emission) — `ArtifactChild`/`ArtifactLink` have no `dsl::DslField` impl, the same wall cad/`✳️object`/`✳️kit` hit. Hand-rolled `store::ArtifactDsl`/`store::ArtifactPack` for the **whole struct**, mirroring cad's `📸️snapshot/🦀️component.rs` exactly: `enc_str`/`dec_str` (hex) for `schema`/`name`; `enc_json`/`dec_json` (JSON-then-hex) for every collection/nested-record field (`grid`, `paragraphStyles`, `characterStyles`, `stories`, `links`, `parentPages`, `spreads`, `pages`, `printTarget`, `dataFieldsJson`, `referencedModel`); `enc_child_opt`/`dec_child_opt` (bracket/hex handle codec) for `backgroundDrawing`. Binary codec mirrors the text codec field-for-field via LEB128 length-prefixed writes (`write_str_lp`/`read_str_lp`).
- **Real round-trip test added** (`📸️snapshot/🦀️component.rs`'s new `round_trip_tests` module): a snapshot with both composition slots populated round-trips losslessly through both `store::ArtifactDsl` (text) and `store::ArtifactPack` (binary) independently — this is the codec-completeness proof the recipe requires (§2's warning: `cargo check` never catches a codec gap, only a real round-trip test does).

**Composition (real, not stubs)** (`🚪️io/🦀️component.rs`, artifact root `🦀️component.rs`):
- `background_drawing_child_handle(source_tag, &SemioDrawingSnapshot) -> LayoutDrawingChild` — content-addressed mint (hashes the drawing content), mirrors cad's `cad_model_child_handle` exactly.
- `layout_document_json_from_dwg` now mints a real `background_drawing` child from the **full** decoded DWG drawing (previously discarded after rect-extraction) and caches its content via `cache_background_drawing_content`.
- `layout_snapshot_to_semio_drawing` (the SVG-export converter) now merges the cached background content's layers in **behind** every page layer when present — the real consumer of the working-scene cache, so an imported trace an author draws pages on top of survives to SVG export instead of only ever informing import-time page framing.
- `referenced_model` is schema/codec-complete but deliberately left inert beyond that (no mutation dispatch, no resolver read path) — genuinely new capability with no existing UI/converter to preserve, documented honestly in the artifact root's doc comment rather than wired to a fictional consumer.

**Working-scene cache** (artifact root `🦀️component.rs`, new `🔖️WorkingScene` region): a `thread_local!` `HashMap<child_id, SemioDrawingSnapshot>` (`cache_background_drawing_content`/`background_drawing_content`), populated at the one call site with literal decoded content (DWG import) and read through exactly one accessor every export call site funnels through. `EngineRep` contract: wholly derived, never a durable field. Staleness gap documented honestly (not fail-closed, matching cad/writer's posture rather than lowpoly's): store-level undo/redo bypasses `ArtifactApp::handle`, so the cache can go stale across an undo of an import — the one read path (SVG export merge) is render-only, not destructive-geometry-edit, so a documented gap is sufficient. **Note for whoever picks up W1's live composition work next**: mid-session a real `ArtifactView::with_children`/`ChildContentView` resolver seam landed in `🔌️plugin/🦀️component.rs` (framework, W1-owned) — the doc comment there explicitly frames it as the seam that should eventually replace exactly this kind of `thread_local!` cache. Not adopted here (out of a single fan-out pass's scope, and it was still being built live as this migration ran), but flagged for a future wave.

**Mutation vocabulary**: no changes needed — `LayoutMutation`/`dsl::Mutations` derive untouched, no `whole_document_operation` override existed to remove (layout never had one), `diff_set_snapshot` (the one whole-artifact-replace helper) was already dead code (only self-referenced in its own test) before this pass and stays that way — not a live `SetSnapshot`-shaped dispatch path.

**Two pre-existing, unrelated bugs fixed** (found blocking compilation, confirmed via `git log --date=iso` to predate this migration and this ticket's window):
- `🚪️io/📤️export/🧵️serializers/…/pdf/🔖️1.4/…` and the sibling import deserializer imported `PageDoc`/`PdfSnapshot` from stdio's crate-root re-export (`semio_s_plugin_stdio::artifacts::pdf::schema::snapshot`), which stdio's own `🗿️artifacts/📄️pdf/🦀️component.rs` had repointed to its 1.7-shaped `PdfSnapshot` (no `page`/`PageDoc` field) in a commit that landed 2026-08-13 13:05:26, an hour before this migration started, entirely inside stdio (never touched by me). This file lives in `1.4`'s own directory and always meant the 1.4 shape. Fixed by importing from stdio's version-pinned `standards::v1_4::subsets::any::schema::snapshot` module directly instead of the now-repointed generic alias.
- Two app-layer tests (`apps::layout::commands::export::tests::export_actions_wire_to_real_layout_exporters`, `apps::layout::component::tests::export_media_layout_out_returns_svg_of_first_page`) never called `ensure_stdio_semio_drawing_registered()` before exercising SVG export, unlike every sibling test that already does (`⚙️engine/🎬️scene`'s own SVG test). Under `cargo nextest`'s per-test process isolation this always fails regardless of my migration — confirmed pre-existing (I never touched export dispatch or `LayoutPlayApp::export_media`). Fixed by adding the same registration call sibling tests already use.

**Fixture regeneration** (recipe §7): `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was in the pre-migration nested-block grammar (`schema=layout.layout name=Demo` on one line, `grid { … }` blocks, etc.) — incompatible with the new hex/JSON-line codec by construction. Regenerated for real via the temporary-debug-test technique: added `#[cfg(test)] mod debug_fixture_regen` to `📸️snapshot/📝️text/🦀️component.rs`, dumped real `print_dsl(&default_document())` output via `cargo test … -- --nocapture`, captured the 15-line output, wrote it as the new fixture, removed the temporary module (verified via `grep -rn debug_fixture_regen` returning nothing).

## Verification (real, foreground)

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-layout --all-targets
→ 0 errors (only pre-existing dead-code/unused-import style warnings)

CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-layout --no-fail-fast
→ 140 tests run: 140 passed, 0 skipped     (reproduced twice, stable, not flaky)
```

Baseline (before any edit): `cargo check` was already red — 2 errors, both in `semio-framework-3d`'s `brep::classify`/`brep::boolean` (`unresolved import PointClassification`), zero errors in layout's own path. Traced to DKM's live `math`→`geometry`/`graph` crate-extraction rename (per `📌️important.md`'s standing warning) — not this migration's concern, and it cleared on its own as that work progressed.

## Concurrent-churn observations

This pass ran through an unusually active window of live framework mechanism work (all confirmed via `stat -f '%Sm'`/`git log --date=iso` showing uncommitted or just-committed edits in W1-owned files, never in `✏️s/🔌️plugins/📏️layout/**`):

1. **DKM's `math`→`geometry`/`graph` dissolution** — baseline's 2 `brep` errors, cleared during this pass.
2. **A state-class enum rename sweeping the whole repo live** (`StateClass::Persistent`→`Artifact`, `SharedUi`→`Presence`, `LocalUi`→`Config`, moved from `os_spr::StateClass` into `wire::codec::StateClass`) — surfaced as a `E0599 no variant named Persistent` error in `📡️spr/🎮️command/🦀️component.rs` (framework, W1-owned) that persisted across several retries before clearing. **This same sweep already rewrote the `#[state(persistent)]`/`#[state(shared_ui)]`/`#[state(local_ui)]` attributes I had just added to `LayoutSnapshot`/`LayoutArtifact`/`LayoutDiff`, in place, to the new names** — visible now as `git diff --cached` showing my own new fields under `#[state(artifact)]`/`#[state(presence)]`/`#[state(config)]` rather than the names I originally typed. Left as-is per the standing "don't revert an in-flight concurrent edit" instruction — it's semantically identical, just renamed, and the tests I ran afterward confirm it still compiles/passes correctly under the new names.
3. **A real `ArtifactView::with_children`/`ChildContentView` resolver seam landed live** in `🔌️plugin/🦀️component.rs` mid-pass (its own doc comment explicitly frames it as the intended eventual replacement for exactly the kind of `thread_local!` working-scene cache this migration built) — noted above for a future wave, not adopted here.
4. **Two other transient plugin-crate errors** (`plugin_sdk_fault` temporarily private, a missing `use std::sync::Mutex;`) surfaced and cleared within 1-2 retries — both in `🔌️plugin/🦀️component.rs`, both W1-owned, neither touched.
5. **A repo-wide "Mode Facet" scaffolding tool and a schema-facet-leaf regeneration** (`.graphql`/`.json`/`.proto`/`.ts` state-class annotations) are landing in the SAME live tree and touch files under `✏️s/🔌️plugins/📏️layout/**` that I never edited (new `📌️empty.md` stub docs under `🎭️modes/✏️edit/**`; state-class annotation renames in `🎚️config`/`👥️presence` schema facet leaves). **These are not mine** — `git status --porcelain -- ✏️s/🔌️plugins/📏️layout` currently shows ~45 changed paths, of which only the 13 listed below under "Files touched" are this migration's own edits; the rest is this concurrent sweep incidentally passing through my plugin's directory tree. One incidental finding worth flagging: `🧬️schema/📸️snapshot/🔗️component.graphql` (and its `.json`/`.proto`/`.ts` siblings) under layout's own `🏅️standards/🔖️1/…` tree contain **`JsonSnapshot`**, not `LayoutSnapshot` — a pre-existing content/path mismatch in these generated facet leaves, unrelated to and untouched by this migration; not investigated further (out of scope, actively being swept by another session as I found it).

None of the above blocked or altered the composition work itself — every retry that showed layout-boundary errors was actually my own bug (see next section), never someone else's churn.

## Real bugs I introduced and fixed during this pass (not pre-existing)

- Two `super::` path-depth mistakes in a hand-written test (`📸️snapshot/📝️text/🦀️component.rs`'s rewritten `parse_dsl_reports_hand_rolled_codec_errors`) reaching for `enc_str` one module level too shallow/deep — caught by `cargo check`, fixed.
- Two test literals in `📸️snapshot/🦀️component.rs`'s new round-trip tests used an invented `"semio://…"` URI scheme for `ArtifactRef::parse_uri` instead of the real `"<artifact_id>!<kind>@<standard>/<subset>"` wire form — caught by reading `🚪️io/🦀️component.rs`'s actual `to_uri`/`parse_uri` implementation before trusting my first guess, fixed before ever running the test.

## sharedFileRequests

None — every real edit is contained inside `✏️s/🔌️plugins/📏️layout/**`. (The `JsonSnapshot`-content mismatch noted above under layout's own schema-facet leaves is flagged for visibility, not requested as a shared-file change — it's pre-existing, out of this migration's scope, and was mid-sweep by another session at time of writing.)

## Files touched (this migration's own edits only)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs` — `LayoutDrawingChild` type alias, `background_drawing_child_handle`, `🔖️WorkingScene` region (`cache_background_drawing_content`/`background_drawing_content`).
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — full rewrite: two new fields, dropped `dsl::DslArtifact`, hand-rolled `ArtifactDsl`/`ArtifactPack`, `empty_layout_snapshot`, real round-trip tests.
- `…/🧬️schema/🦀️component.rs` (`LayoutArtifact`) — two new fields, `Default`, `to_snapshot`/`from_snapshot`/`set_snapshot`, 3 test-literal updates.
- `…/🧬️schema/🔺️diff/🦀️component.rs` (`LayoutDiff`) — two new `Option<Option<T>>` fields.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_to_artifact`/`MutationDiff::apply`/`absorb` wiring, 2 test-literal updates.
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — 2 test-literal updates, rewrote `parse_dsl_reports_*` for the new codec's real failure modes, temporary `debug_fixture_regen` module (added then removed).
- `…/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — 2 test-literal updates.
- `…/🚪️io/🦀️component.rs` — real DWG-import mint+cache, SVG-export background-layer merge, 2 new tests, 1 test-literal update.
- `…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` — version-pinned stdio import fix (pre-existing bug).
- `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` — same fix, import side.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (real `print_dsl()` capture, not hand-transcribed).
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/🐚️export/🦀️component.rs` — missing-registration test fix (pre-existing bug).
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs` — same fix, sibling test.

ucas-status: complete
