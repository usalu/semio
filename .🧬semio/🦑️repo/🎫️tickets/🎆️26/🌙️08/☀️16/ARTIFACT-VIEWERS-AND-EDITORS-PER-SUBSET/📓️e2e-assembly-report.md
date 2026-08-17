# E2E-ASSEMBLY — cad plugin assembly fix

## Root cause (one sentence)
cad's `definition()` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs`) declared composer capabilities for its 8 stdio import/export dialects but never for its own native round-trip dialect (`s.cad@1/*`), and separately misdeclared its document codec's schema claim as the stale `"cad.document"` instead of the actual `ArtifactEditor::DOCUMENT_SCHEMA` value `"cad.scene"` — both are `require_declared_capability_or_record` mismatches in `🔌️plugin/🦀️component.rs`, and each one alone is enough to collapse assembly into the `"assembly-failed"` manifest stub.

## Reproduction (native, not browser)
Added a permanent test module `assembly_tests` to `✏️s/🔌️plugins/📐️cad/🦀️component.rs`:
- `cad_composer_entries_have_declared_capabilities` — walks every `ComposerEntry` from `io_registry::entries()`, computes the exact dialect-coordinate claim `require_declared_capability_or_record` derives from `entry.writes`, and checks it against `definition().capabilities_of(&ArtifactCapabilityKind::composer())`. Before the fix this printed:
  ```
  composer entry writes="s.cad@1/*" declared_capability_found=false
  composer entry writes="s.stdio.ifc@4/*" declared_capability_found=true
  ... (7 more, all true)
  ```
  i.e. the *only* mismatching entry among cad's 9 composer entries is the native self-compose entry (`composer_entry_of::<CadAnyComposer>()`, `writes = CadComposerComposition::WRITES = Dialect{"s.cad","1","*"}`), not any of the 8 stdio format bridges.
- `cad_plugin_assembles_with_editor_and_viewer_apps` — calls `crate::plugin()` directly and asserts a real `pluginId` plus both `s.cad.cad@1/*#editor` and `s.cad.cad@1/*#viewer` in the manifest's app ids.

Before any fix, `cad_plugin_assembles_with_editor_and_viewer_apps` failed with:
```
PluginAssemblyError { code: "artifact-definition.runtime-capability", message: "no declared composer capability owns the runtime claims" }
```
After adding the missing composer row, the *same test* progressed to a second, distinct failure:
```
PluginAssemblyError { code: "artifact-definition.runtime-capability", message: "no declared codec capability owns the runtime claims" }
```
This is `.document_codec::<EditorApp<CadPlayApp>>()`'s check. `CadPlayApp: ArtifactEditor` sets `const DOCUMENT_SCHEMA = CAD_DOCUMENT_SCHEMA` = `"cad.scene"` (see the file's own header doc: "the `cad.scene` document schema"), but the declared `s.cad.codec.document.v1` row claimed codec schema `"cad.document"` (`CAD_PLAY_DOCUMENT_SCHEMA`, a different constant used only as the in-snapshot `schema` data field, e.g. in `empty_cad_snapshot()`). Two constants, one row pointed at the wrong one.

## Fix (both in `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs`, `definition()`'s `rows`)
1. Added the missing native composer declaration, matching the `composer.native` convention already used correctly by `jack`, `rewrite`, `remodel`, `raster`, `playground`, `block2d/3d/5d`, `dag`, `shooting`, and `puzzle3d`:
   ```rust
   ("s.cad.composer.native", "composer", "s.cad@1/*", &[("dialect", "s.cad@1/*")], None),
   ```
2. Corrected the codec row's schema from the stale `"cad.document"` to the real `ArtifactEditor::DOCUMENT_SCHEMA`, `"cad.scene"`:
   ```rust
   ("s.cad.codec.document.v1", "codec", "cad.scene:cad", &[("codec", "cad.scene"), ("extension", "cad")], None),
   ```

Both `cad_composer_entries_have_declared_capabilities` and `cad_plugin_assembles_with_editor_and_viewer_apps` pass after these two changes. **cad's manifest now carries both surfaces**: `s.cad.cad@1/*#editor` and `s.cad.cad@1/*#viewer`, with `pluginId: "cad"` (not `"assembly-failed"`).

Nothing in `🔌️plugin/🦀️component.rs` (the framework mechanism, owned by a live peer session, last touched 2026-08-16 21:52) was weakened, loosened, or touched — both fixes are cad-side declaration corrections, exactly matching the two failure modes the mechanism is designed to catch (missing declaration, misdeclared claim).

## Other plugins with the identical missing-native-composer bug
Confirmed by grep + manual check (each has a `composer_entry_of::<...AnyComposer>()` native entry in its `io_registry::entries()` but no matching `"composer"` row for its own `s.<slug>@1/*` dialect in its `definition()`'s `rows`) — **these will hit the same `assembly-failed` stub** and were left unfixed as out of scope for this cad-only lane:
- `mathematical` — `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs` (missing `s.mathematical@1/*`)
- `note` — `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs` (missing `s.note@1/*`)
- `forms` — `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` (missing `s.forms@1/*`)
- `layout` — `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs` (missing `s.layout@1/*`)
- `imperative` — `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs` (missing `s.imperative@1/*`)
- `curate` (sourcing) — `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs` (missing `s.curate@1/*`)

Plugins already correctly declaring their native composer row (used as the precedent for cad's fix): `jack`, `rewrite` (trinity), `remodel`, `raster`, `playground` (demonstrator), `block2d`/`block3d`/`block5d`, `dag`, `shooting`, `puzzle3d`, plus the norm family (`din4108`, `din18599`, `din16798`, `en1990`–`en1999`, `vdi3805`, `iso16757`) via their own `"composer.any"` convention.

## Unrelated pre-existing failure (not caused by this fix, not fixed here)
`cargo test -p semio-s-plugin-cad --lib` (full suite) has one unrelated failure: `editor::cad::component::tests::two_instances_converge_disjoint_edits_via_backbone`, panicking inside `🏪️store`'s VCS layer (`module.vcs`, "invalid edit reference"), deterministic across reruns. This is unrelated to artifact/plugin assembly (doesn't call `plugin()`/`definition()`/`declaration()` at all) and lines up with the live VCS/mutation-outcomes refactor visible in recent repo history (commit `5a1367dfcc`, "Replace SPR CRDT module with conflict resolution component", "Mutation Outcomes, Merge Policies and First-Class Conflicts") — a concurrent peer session's in-progress work, per this repo's known cargo-workspace-churn pattern. Left untouched; flagged here rather than silently ignored.

## Files touched
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs` — added `s.cad.composer.native` row; corrected `s.cad.codec.document.v1` row's schema from `"cad.document"` to `"cad.scene"`.
- `✏️s/🔌️plugins/📐️cad/🦀️component.rs` — added permanent `assembly_tests` module (`cad_plugin_assembles_with_editor_and_viewer_apps`, `cad_composer_entries_have_declared_capabilities`).

## Verification commands (full output in `🧪️e2e-assembly.txt` in this folder)
- `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-cad --lib assembly_tests -- --nocapture` → 2 passed
- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-cad --all-targets --keep-going` → 0 errors (pre-existing privacy warnings only, unrelated to this change)
- `cargo build -p semio-s-plugin-cad --target wasm32-unknown-unknown --lib` → succeeds
- `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-cad --lib` (full suite) → 150 passed, 1 unrelated pre-existing failure (see above), 1 ignored
