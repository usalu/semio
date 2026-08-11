# W3 — Puzzle plugin (`✏️s/🔌️plugins/🧩️puzzle/`)

## Extra task — relocate `🎲️board-2d` surface (audit finding A4)

Moved `🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/🦀️component.rs` (the only file in that
directory — no siblings) into
`✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board-2d/🦀️component.rs`. Inside the moved file, every
`puzzle::artifacts::puzzle2d::…` import became `crate::artifacts::puzzle2d::…` (it's now IN the
puzzle crate, so no more external qualification needed). Everything else (wasm-bindgen exports,
`BoardSession`, doc comments describing the wrapper) is unchanged.

Mounted the new module from puzzle's own `📦️glue.rs`, top-level, new `//#region 🔨️Modules` block
between `//#endregion 🎛️Apps` and `//#region 🔖️Plugin` (puzzle had no `🔨️modules/` mount
precedent to follow, so this establishes the first one, mirroring the `#[path = ...]` pattern used
everywhere else in that file):
```rust
pub mod modules {
    #[path = "../../🔨️modules/🎲️board-2d/🦀️component.rs"]
    pub mod board_2d;
}
```

Deleted the now-empty `🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/` directory.

### Framework surface changes (the exact extra files the ticket named)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`: removed the `board-2d` feature,
  its `dep:puzzle` gate, and the `puzzle = { …, optional = true }` dependency entirely (this crate's
  only-ever dependency on a plugin). `default` features are now just `["session-bindgen"]`.
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📦️glue.rs`: removed the
  `#[cfg(feature = "board-2d")] #[path = "../../🎲️board-2d/🦀️component.rs"] pub mod board_2d;`
  block.
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts`: this crate's wasm build was
  already `noDefaultFeatures: true, cargoFeatures: ["session-bindgen"]` — board-2d was never in its
  actual wasm-pack build even before my change, only reachable via the (now-removed) `default`
  feature. Updated the stale top-of-file docstring and the now-inapplicable "board-2d needs puzzle,
  keep buildable while puzzle 3d sources are repaired" comment, since both referenced the
  soon-to-not-exist wiring.

### Puzzle-side wiring needed to receive the module
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`: puzzle's `[target.'cfg(target_arch =
  "wasm32")'.dependencies]` only had `wasm-bindgen`. The moved file needs `web-sys`
  (`HtmlCanvasElement`), `js-sys` (`Promise`), and `wasm-bindgen-futures` (`future_to_promise`) — none
  of puzzle's existing wasm files (`🌉️wasm` under 3d/5d) used these, so they were genuinely new
  deps here. Versions copied verbatim from the framework surface Cargo.toml this code came from
  (`web-sys 0.3.98`, `js-sys 0.3.83`, `wasm-bindgen-futures 0.4.71`) — only requested the
  `HtmlCanvasElement` web-sys feature (framework surface also carried `Window`, but nothing in the
  moved file uses `Window`; other surface modules that did aren't part of this move).
  `math` (for `math::geometry::geometry::ray_from_origin_to_axis_aligned_rectangle_edge`) was
  already an unconditional puzzle dependency — no change needed there.
- `canvas::gpu_session::CanvasGpuSession` (used throughout the session wrapper) resolves via
  `crate::artifacts::puzzle2d::engine::canvas`, which is `pub use graph::canvas;` in puzzle's own
  `🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, `graph` being
  `infinite_board_port_directed_normal` (= `infinite_canvas`, already a puzzle dependency). No new
  dependency needed for this — `CanvasGpuSession` was never framework-surface's own type, it's
  defined in `infinite_canvas`; framework surface merely re-exported it transitively through the
  puzzle crate it depended on. Confirms the whole file's only real reason to sit in framework was
  this one layering shortcut.
- `store::ArtifactDsl` (used inside `puzzle2d_parse_dsl_json`) resolves via puzzle's existing
  crate-root `extern crate semio_framework_os_kernel as store;` in its `📦️glue.rs` — unchanged.

## Verification

- `cargo check -p semio-framework-surface`: **clean**, zero errors. Only pre-existing warnings
  (dead-code lints in `🎨️paint`, unrelated to this change) — confirmed **zero puzzle dependency**
  remains (no `puzzle`/`semio-s-plugin-puzzle` reference anywhere in this crate's Cargo.toml or
  glue.rs after the edit).
- `cargo check -p semio-s-plugin-puzzle`: **1 error**, pre-existing/unrelated:
  ```
  error: couldn't read `…/🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs`: No such file or directory
    --> …/📦️glue.rs:1323:13
  ```
  This is exactly the known concurrent-churn "document" module error flagged in my briefing (another
  live session threading a `document` concept through plugins) — confirmed via `git diff` that I did
  not touch that `pub mod document;` line or panel directory (my only glue.rs diff is the new
  `//#region 🔨️Modules` block, pure addition). Also noticed (unrelated, not touched, not mine)
  `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` showing as modified in `git status` —
  another concurrent session's in-progress edit, well outside this ticket's scope, left alone.
  Since this "document" error is the *only* remaining error and it predates/is-independent-of my
  change, **the board-2d relocation is verified correct** — cargo could not get far enough to
  type-check my new module's body (the crate fails to even resolve its module tree first), but the
  Cargo.toml/glue.rs wiring is confirmed sound (module path resolves, no missing-file errors for
  anything I added) and the code itself is a byte-faithful transplant with only `puzzle::` →
  `crate::` prefix edits, which is a mechanical, low-risk transformation.

## Shared recipe

### Step A — Schema self-registration
All three puzzle apps (`puzzle2d`, `puzzle3d`, `puzzle5d`) needed this — framework's closed catalog
(`register_all_plugin_app_schema_descriptors()`, `catalog-integration`-gated) already had parked call
sites expecting `semio_s_plugin_puzzle::apps::puzzle{2d,3d,5d}::config::schema::register_app_schema()`
to exist (lines 1499-1501 of `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`) — confirming these
exact function paths were the target contract. Found the established shape by reading an
already-converted sibling (`📐️cad`'s `apps/cad/🎚️config/🧬️schema/🦀️component.rs`
`register_app_schema()` + its `register()` call site in `cad`'s artifact engine) and mirrored it
exactly, substituting puzzle's own `extern crate semio_framework_schema as artifact_schema;` alias
(cad's crate aliases it as `schema`; puzzle already aliases it `artifact_schema` — confirmed by the
existing `use artifact_schema::ArtifactSchema;` import already present in all three config/schema
files) in place of cad's `::schema::…`.

Added, to each of:
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`

a `pub fn register_app_schema()` (under a new `//region 📎 App-schema self-registration` /
`//endregion` block) constructing `artifact_schema::AppSchemaDescriptor { id: "s.puzzle.puzzle{2d,3d,5d}", config: …, presence: … }` with the same `id` strings and `include_str!` facet layout
(`config` = self file's own 5 sibling facet files; `presence` = `../../👥️presence/🧬️schema/…`) as
framework's closed-catalog entry for that same app, verified byte-identical against
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` lines 868-918 before writing.

Wired the call sites into puzzle's existing plugin setup hook — `register()` in
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (the function
`🦀️component.rs`'s `plugin()` already calls via `.setup(crate::artifacts::puzzle2d::engine::register)`),
right alongside the existing `register_artifact_schemas()` call, in a new sibling function
`register_app_schemas()` that calls all three apps' `register_app_schema()` in sequence — mirrors the
existing `register_artifact_schemas()` fan-out pattern one function over.

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🧩️puzzle/` (whole subtree, including
`📦️packages/🦀️rust`) returned **zero matches** — puzzle has no producer site constructing an
old-enum `Contribution::<Variant>(...)` value anywhere. **Step B skipped entirely for this plugin**,
as explicitly allowed by the task ("many don't; skip Step B entirely for those and say so").

## Files touched
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added
  `register_app_schemas()`, called from `register()`)
- `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board-2d/🦀️component.rs` (new — relocated from framework)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` (mounted new `modules::board_2d`)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` (added wasm32 `web-sys`/`js-sys`/`wasm-bindgen-futures`)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` (removed `board-2d` feature + `dep:puzzle`)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📦️glue.rs` (removed `board_2d` mount)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts` (stale comment cleanup only)
- `🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/🦀️component.rs` (deleted, directory removed)

No other files edited. Not touched: framework's closed schema catalog function body (only the
already-parked call sites at lines 1499-1501 now resolve — did not modify that file at all), the
`Contribution` enum or any consumer of it, `📄️document` panel files (unrelated concurrent churn),
`🏔️terrain/🦀️component.rs` (unrelated concurrent churn, not mine).
