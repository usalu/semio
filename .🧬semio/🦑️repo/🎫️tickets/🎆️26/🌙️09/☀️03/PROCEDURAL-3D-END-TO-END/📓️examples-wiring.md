# 📚️ Example switcher wiring — gen3d/gen2d editor examples reach `manifest.examples`

## What changed

1. **Framework builder** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:437-482`.
   `editor::<E>(def)` (line 439) now delegates to a new private `editor_app::<E>(self, def, examples)`
   (line 458) that does exactly what the old `editor` body did, plus stamping
   `examples.into_iter().map(Into::into).collect()` onto `App.examples` instead of hard-coding
   `Vec::new()`. New public twin `editor_with_examples::<E>(self, def, examples: Vec<crate::app::ExampleSource>)`
   (line 451) delegates to the same `editor_app`. No behaviour change for existing `.editor::<E>(def)`
   call sites — `editor` still passes `Vec::new()`.
   `viewer_with_examples` was **not** added: `register_app_factory`
   (`🔌️plugin/🦀️.rs:24951-24961`) and the newer `SubsetDeclaration` commit walk
   (`🔌️plugin/🦀️.rs:27375`, `if surface.definition.role == AppRole::Editor { subset.examples… } else { Vec::new() }`)
   both confirm examples only ever attach to the editor surface repo-wide — no viewer carries examples
   anywhere in this codebase.

2. **Mounted the missing example leaf** — `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/🦀️.rs:1679-1680`
   adds `pub mod art_generation3d_hexagonal_mushroom_column;` (`#[path]` to the pre-existing
   `📚️examples/🎬️hexagonal-mushroom-column/🦀️.rs` leaf, which already exported `source()` but was never
   `mod`-mounted, unlike its 7 siblings).

3. **`examples()` on the gen3d editor** — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1220-1234`.
   New `pub fn examples() -> Vec<ExampleSource>`, listing all 8 `crate::examples::art_generation3d_*::source()`
   calls in the exact order `schema::is_generation3d_example_id` declares them (hex-column, rect-extrude,
   sphere-torus, box-fillet, sphere-box-fuse, face-sweep-extrude, rectangle-wire, box-shell). Replaced the
   stale `🚧️ SDK GAP` comment block that used to sit before `.build_definition()` (the gap it described is
   now closed). Added `ExampleSource` to the `semio_framework_plugin::{…}` import list (line 30).

4. **Wired at the plugin root** — `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:308,312`:
   - gen3d: `.editor::<…Generation3dPlayApp>(create_generation3d_app())` →
     `.editor_with_examples::<…Generation3dPlayApp>(create_generation3d_app(), crate::editor::generation3d::examples())`
   - gen2d: `.editor::<…Generation2dPlayApp>(create_generation2d_app())` →
     `.editor_with_examples::<…Generation2dPlayApp>(create_generation2d_app(), vec![crate::examples::art_generation2d_demo::source()])`
     (gen2d has exactly one example leaf, `📚️examples/🎬️demo/🦀️.rs`, already mounted as
     `crate::examples::art_generation2d_demo`, already exporting `source()`). Only this one line in the
     plugin-root file was touched for gen2d — no gen2d artifact/editor files were touched, per the
     instruction that another session owns those today.

5. **Bilingual labels (8 leaves)** — every `📚️examples/🎬️<slug>/🦀️.rs` under
   `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/` had
   `LocalizedLabel::native(en, en)` (German = duplicated English). Replaced the German half with the exact
   strings already used by `setActiveExample`'s `ActionArgOption`s in the editor
   (`✏️editor/🦀️.rs:1174-1181`):
   - hexagonal-mushroom-column → "Sechseckige Pilzsäule"
   - rectangle-extrude-volume → "Rechteck-Extrusionsvolumen"
   - sphere-cut-with-torus → "Kugel mit Torus geschnitten"
   - box-fillet-preview → "Kantenrundung Vorschau"
   - sphere-box-fuse → "Kugel und Quader vereinen"
   - face-sweep-extrude → "Fläche extrudieren"
   - rectangle-wire-preview → "Rechteck-Draht Vorschau"
   - box-shell-preview → "Hohlkörper Vorschau"
   (gen2d's single `demo` leaf label was left untouched — out of scope per instruction.)

6. **Tests**:
   - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`,
     new `#[test] fn examples_match_set_active_example_select_options()` inside `mod tests` (end of file,
     region `🔖️ExamplesTests`): walks `create_generation3d_app().window_kinds[*].actions` for the
     `setActiveExample` action, reads its first arg's `ActionArgControl::Select { options }`, and asserts
     `examples()`'s 8 ids equal the option values in order.
   - `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`, `mod surface_tests`, new
     `#[test] fn generation3d_manifest_examples_are_registered_on_the_editor_surface()`: builds
     `super::plugin()`, asserts `create_generation3d_app().id == "s.procedural.generation3d@1/*#editor"`,
     then asserts `plugin.manifest.examples` filtered to that `app_id` has 8 entries whose ids equal
     `crate::editor::generation3d::examples()`'s ids in order. No pre-existing
     `semio_framework_plugin::testkit` helper asserts manifest examples (checked — only
     `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` exist for this app pair), so
     both tests are hand-written against `PluginManifest`/`AppDefinition` fields directly.

## VERIFY — blocked, root cause identified precisely, NOT a file this ticket touched

`cargo check -p semio-framework-plugin` fails identically on every retry (4 attempts, ~19:16-19:32) with:

```
error: failed to load manifest for workspace member `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️📦️📦️packages/🦀️rust`
referenced by workspace at `/Users/ueli/Documents/semio/Cargo.toml`

Caused by:
  failed to read `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️📦️📦️packages/🦀️rust/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
```

Root cause: root `/Users/ueli/Documents/semio/Cargo.toml` is currently uncommitted-modified by another
session (`git diff --stat -- Cargo.toml` → 201 lines changed, mtime 19:30:04, no process still writing it
as of 19:32:47) and every `📦️packages` path segment across the ENTIRE workspace-members/patch table has
been mangled into `📦️📦️📦️packages` or `📦️📦️📦️📦️packages` (double/triple/quadruple-duplicated emoji) —
confirmed with `grep -n "📦️📦️📦️packages" Cargo.toml`, well over 100 matching lines including the
`semio-s-plugin-procedural` and `semio-framework-plugin` entries themselves. The real on-disk directories
still use the single `📦️packages` form (`git status --porcelain` on the fem plugin shows only file
contents modified, not the directory renamed). This is not an intentional in-progress rename — it reads
like an idempotency bug in some other session's bulk find/replace tool — and it blocks **every** `cargo`
invocation repo-wide (workspace manifest fails to parse before package selection happens), not just this
ticket's packages. Per this ticket's own status.md precedent for another session's live-work breakage
("wait and retry, then report precisely"), this was retried 4 times over ~16 minutes with no change and is
reported here rather than hand-patched — a 200+-line corruption to a file another session has open,
uncommitted, is out of scope to silently rewrite.

**Consequence: none of the three prescribed VERIFY commands (`cargo check -p semio-framework-plugin`,
`cargo check -p semio-s-plugin-procedural`, `cargo test -p semio-s-plugin-procedural --lib -- examples`)
could be run to completion this session — all fail identically at manifest-load, before reaching any
package this ticket touched.** `rustfmt`-level review of every edited file was done by hand (see the
"What changed" section's line anchors); no automated compiler confirmation exists yet for this change.

## 🗑️generated folder

Not deleted. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/🗑️generated/` still holds
the pre-existing files from earlier in this ticket's history (`dev-3d-boot6.txt`, `dev-3d-boot7.txt`,
`engine-flow-core-wasm.txt`, `gen3d-check-lib-shared.txt`, …) — confirmed present, untouched, before and
after this session's work. No files were written into it this session (the VERIFY commands never reached
the `tee`/redirect step because the manifest-load failure happens before compilation starts).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️{hexagonal-mushroom-column,rectangle-extrude-volume,sphere-cut-with-torus,box-fillet-preview,sphere-box-fuse,face-sweep-extrude,rectangle-wire-preview,box-shell-preview}/🦀️.rs` (8 files, label only)

Not touched: any generation2d artifact/editor file, `Cargo.toml` (root or otherwise), any stdio/BREP file.
