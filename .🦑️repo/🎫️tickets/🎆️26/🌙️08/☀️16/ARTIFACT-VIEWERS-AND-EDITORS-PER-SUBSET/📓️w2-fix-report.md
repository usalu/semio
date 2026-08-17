# W2-FIX Report — Puzzle Wiring + Framework Referrer + SDK Re-export Gap

Lane: W2-FIX, closing dangling wiring left by three earlier packets so the plugin dependency graph
compiles again. Recipe followed: `📓️w2-cad-report.md`. Contract: `📋️contract-freeze.md` §1–§2.

## Job 1 — puzzle plugin wiring

`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`: the old `//#region 🎛️Apps` (1721–2348, mounting
`apps::{puzzle2d,puzzle3d,puzzle5d}` from `../../🎛️apps/<kind>/…`) replaced by two independent regions,
`//#region ✏️Editor` (`pub mod editor { pub mod puzzle2d/puzzle3d/puzzle5d { … } }`, mounted from each
kind's `🗿️artifacts/<kind>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`) and `//#region 👁️Viewer` (same
three kinds, `…/👁️viewer/…`, only the root/`🎭️modes/👁️view`/one window per kind — the rest of each
viewer's required facets are still scaffold `📌️empty.md`). Built by taking the OLD apps-region text
verbatim and doing a scoped string substitution of `../../🎛️apps/<kind>/` →
`../../🗿️artifacts/<kind>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (the editor tree's internal
directory structure is byte-identical to the old apps tree's, confirmed by diffing the two file
listings before touching anything) — this is why the editor region needed no hand-typed emoji paths.
The viewer region (much smaller — no such shortcut existed) was hand-built from the real on-disk
listing and the module names each viewer's own root `component.rs` already imports
(`crate::viewer::puzzle2d::modes::view::windows::board`, `…puzzle3d…::windows::main`,
`…puzzle5d…::windows::world3d`). **Caught myself hitting the exact "🏅️standards vs 🏅️标准" typo trap
CLAUDE.md warns about** while hand-typing the viewer region's puzzle3d mode path — the disk-verification
script (adapted from `📓️w2-cad-report.md`'s snippet) flagged it as MISSING before it ever reached the
real file; fixed and re-verified 0 missing. Also repointed the bottom `//#region 📚️Examples` →
`examples::apps` submodule (demo-session mounts, outside the `🎛️Apps` region but pointing at the same
now-deleted path) at each kind's new `✏️editor/📚️examples/🎬️demo-session/…` — name kept ("apps"), per
`📓️w2-cad-report.md`'s precedent for this exact spot. **Verified with the disk-resolution script against
the WHOLE file, not just the touched regions: 950 `#[path]` attrs total, 0 missing**, both before and
again after deleting the `🎛️apps` tree.

`✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`: the three `.document_app::<crate::apps::…>` calls became six
— `.editor::<crate::editor::<kind>::…PlayApp>(…)` + `.viewer::<crate::viewer::<kind>::…Viewer>(…)` per
kind. Added `#[cfg(test)] mod surface_tests` calling the **real**
`semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
(landed by W0-F after the cad pilot, which had to use local stand-ins) — six tests, one
never-mutates/share-dialect pair per kind.

`📦️packages/🦀️rust/Cargo.toml`: five `app = "puzzle<N>d-play"` playground/asset rows → the real derived
surface ids read off each kind's own `Dialect` const (`🚪️io/🦀️component.rs`, e.g.
`PUZZLE2D_DIALECT: Dialect { artifact_kind: "s.puzzle2d", standard: StandardId("1"), subset:
SubsetId("*") }`) — **`s.puzzle2d@1/*#editor` / `s.puzzle3d@1/*#editor` / `s.puzzle5d@1/*#editor`**, not
the ticket brief's illustrative `s.puzzle.puzzleNd@1/*#editor` (validated against the actual const
rather than assumed — the brief's example string doesn't match any real dialect in this plugin).

`📦️packages/🟦️typescript/🧪️vitest.config.ts`: the `include` array had two independent staleness bugs —
the three `🎛️apps/<kind>/📚️examples/🎬️demo-session/🧪️tests/🟦️test.ts` entries (now-deleted path) AND
the six artifact-level example test entries (`🌲️concrete-forest`/`🏗️nakagin-capsule-tower`) were
already missing the `🏅️standards/🔖️1/🪆️subsets/✳️any` segment predating this packet (confirmed stale
against a working sibling, `📐️cad`'s own `tsconfig.json`, which has the segment). Fixed all nine
entries; verified all nine resolve on disk.

`✏️editor/🧠️precompute/🦀️component.rs` (puzzle5d): lines 20/31 (`inner: crate::apps::puzzle3d::…` /
`Self { inner: crate::apps::puzzle3d::… }`) repointed at `crate::editor::puzzle3d::precompute::…`.

Cosmetic (own-plugin doc comments only, no compile dependency, fixed for hygiene): three
`crate::apps::puzzle3d::precompute` mentions in puzzle3d's `🧬️schema/🦀️component.rs` and
`🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`; one `🎛️apps/🖐️5d/🦀️component.rs` mention in puzzle5d's
`🧬️schema/🧬️mutations/🦀️component.rs`; the editor `⚙️engine/🦀️component.rs` doc comment that said the
framework renderer "still read the OLD path as of this packet, … not fixed here" — updated now that
Job 2 fixed it.

**Deleted** `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/` (only a stub `🦀️component.rs` doc-comment file remained;
confirmed 0 references to it anywhere in the repo, in real code or config, before removing).

## Job 2 — framework + demonstrator referrers

`🧰️framework/…/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`: **6** real
`puzzle::apps::puzzle2d::engine::BoardHost` call sites, not the 5 the brief listed — `:1601`
(`with_board_host`) was missed by the brief's enumeration; caught by a full-file grep before declaring
done. All 6 repointed to `puzzle::editor::puzzle2d::engine::BoardHost` (`:72, :1418(doc), :1492, :1576,
:1592, :1601`).

**Layering note** (per the task's ask, not restructured): this file is framework/OS renderer code
importing a specific plugin's (`puzzle`) internal engine module directly, bypassing the
plugin/surface/artifact boundary entirely. That's an inverted dependency (framework → plugin) that
predates this ticket and this fix only preserves it under the new module path. Worth a follow-up ticket
to either move `BoardHost`'s host-facing surface into a framework-owned trait the plugin implements, or
make the framework renderer route board-2d hosting through the same `AppRouter`/surface mechanism this
whole ticket is building for everything else.

`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs`: `use puzzle::apps::puzzle3d::{…}` →
`use puzzle::editor::puzzle3d::{…}`; `.document_app::<Puzzle3dPlayApp>(create_puzzle3d_app())` →
`.editor::<Puzzle3dPlayApp>(create_puzzle3d_app())`; the `bundle_registers_the_six_demonstrator_surfaces`
test's literal `"puzzle3d-play"` → `"s.puzzle3d@1/*#editor"` (derived id, same validation method as Job
1's Cargo.toml rows). The `contribution_consumers_declare_the_hidden_app_command` test's list doesn't
include puzzle3d, so untouched. Touched ONLY these puzzle-referring lines — demonstrator's own cad row
(`.editor::<CadPlayApp>`), procedural/process/gis/sourcing rows, and everything else in this file is
untouched, as instructed.

`📦️packages/🦀️rust/Cargo.toml`: three `app = "puzzle3d-play"` rows (playground `aggregator` variant,
`mesh-collection` asset, `static-dir` `/infinite-fixture` asset) → `"s.puzzle3d@1/*#editor"`.

Cosmetic, fixed (cheap, plain data, no import — per the brief): `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs:125-126`, the two hardcoded
`"puzzle2d-play"`/`"puzzle3d-play"` seed strings → the derived ids. Neither string is actually asserted
by that test (it only checks the JSON tree's document-path/label shape), so this was pure hygiene.

Checked and deliberately left alone: `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`'s
`"puzzle3d-play-kinds.objects"` etc. — UI gesture/tree-node ids in a demo script, unrelated namespace
(same call the cad W0-F pilot made for `CAD_SHAPE_WINDOW_ID`); and
`🧰️framework/…/📺️renderer/…/⚛️react/🧪️index.test.ts`'s `"puzzle2d-play"`/`"puzzle3d-play"` — arbitrary
`controllerId` string literals in framework renderer component tests that never import the `puzzle`
crate or resolve through the real app registry.

## Job 3 — SDK re-export gap (window kits + dialect types)

`🧰️framework/…/🔌️plugin/🦀️component.rs`, the curated crate-root `pub use app::{ … };` block (the same
one W0-F extended with the seven surface traits/adapters): added, alphabetically among neighbors, in
the same idiom —

- `Dialect` (between `DerivedArtifactSpec` and `DraftView`)
- `MeshView`, `MeshWindowKit` (between `MeshDwgDocumentImporter` and `ModeSpec`)
- `StandardId`, `SubsetId` (between `PresenceView` and `TransientView` — there was no other S-word in
  the list before this)
- `WindowKit` (between `WindowKindSpec` and `WireArtifactInferenceBudget`)

Confirmed all six live directly inside `pub mod app { … }` at the single indentation level the existing
re-export list already draws from (`Dialect`/`StandardId`/`SubsetId` via `app`'s own `pub use
semio_framework::{…}`; `WindowKit`/`MeshWindowKit`/`MeshView` defined directly in `app`, no further
nesting) — no blanket glob added, each name listed individually. Checked for naming collisions at crate
root before editing: none. `TextWindowKit`/`TableWindowKit`/`TreeWindowKit`/`ImageWindowKit`/
`DocumentWindowKit`/`MediaWindowKit` (the other six window kits) were **not** added — out of this
packet's named scope (only `MeshWindowKit`/`MeshView`/`WindowKit` were named), flagging for whoever next
needs them since the same gap almost certainly applies.

## Verification

`🧪️w2-fix-cargo.txt` (ticket folder) has the full output of four serial runs:

1. **`cargo check -p semio-s-plugin-puzzle --all-targets --keep-going`** (two runs, ~unchanged between
   them): 186–187 errors, **all inside `semio-s-plugin-stdio`**'s own files (confirmed:
   `grep -B2 -A6 "^error" | grep -c "🧩️puzzle"` → 0 both times). `git status --porcelain` shows
   `✏️s/🔌️plugins/🗄️stdio` modified-uncommitted right now; `git log --date=iso -1` shows a commit at
   `2026-08-16 14:18:35` today — exactly the "known-broken by live peer session" case the brief named.
   **0 errors ever attributed to a `🧩️puzzle` file.**
2. **`cargo check -p semio-framework-plugin --all-targets --keep-going`**: **clean, `Finished`, exit 0,
   0 errors** (only pre-existing warnings, none in the Job 3 edit region or the W0-F `SurfaceTestkit`
   region).
3. Extra (not required by the brief, run anyway since I made a real edit to framework code):
   `cargo check -p semio-framework-os-renderer-wgpu` (the crate hosting `EngineCanvas`, i.e. Job 2's
   real fix) — also blocked by the same `semio-s-plugin-stdio` chain, **0 errors attributed to
   `EngineCanvas`**. It surfaced one *additional*, unrelated pre-existing failure in
   `semio-framework-plugin-host` (a different crate — `🖥️host/…/🦀️component.rs:2707`, an
   `AppFrame::Error` pattern missing a `report` field): confirmed via `git status`/`git log` that this
   file is untouched by me and the error is already present in the last real commit to it
   (`2026-08-16 12:10:56`, the peer plugin-dependency-parity/AppRouter work) — not caused by this
   packet, out of this packet's lease, not fixed.

`0 missing` — the disk-resolution script (adapted from `📓️w2-cad-report.md`) ran against the ENTIRE
`glue.rs` (not just the touched regions): 950 `#[path]` attributes, 0 unresolved, checked once before
and once after deleting `🎛️apps` to confirm the deletion broke nothing.

## Files touched

Edited:
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` (✏️Editor + 👁️Viewer regions, examples::apps repoint)
- `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs` (`.editor()`/`.viewer()` × 3, `surface_tests` module)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` (5 playground/asset `app =` rows)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️vitest.config.ts` (9 `include` paths)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️component.rs` (2 real refs)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (doc, 1 ref)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (doc, 3 refs)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (doc, 1 ref)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs` (doc, 1 ref)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` (6 refs)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (6 names added to curated re-export list)
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` (import, builder call, 1 test literal)
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (3 `app =` rows)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs` (2 seed-string literals)

Deleted:
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/` (whole tree — the plugin's only remaining app-facet stub)

Scratch (ticket folder): `🧪️w2-fix-cargo.txt`.
