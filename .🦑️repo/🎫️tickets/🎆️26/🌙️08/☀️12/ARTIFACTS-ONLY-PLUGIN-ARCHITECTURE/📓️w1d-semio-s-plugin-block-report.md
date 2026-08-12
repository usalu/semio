# W1d — `semio-s-plugin-block`: eliminate `.setup(register_block_exports)`

Ticket: `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`. Plugin root:
`✏️s/🔌️plugins/🧱️block/🦀️component.rs`.

## Result

**`.setup()` is gone from the block plugin entirely — zero residue.** The two remaining hits for
`.setup(` in the plugin root are doc-comment prose (lines 140, 144), not calls; `Plugin::builder(...)`
now chains straight from `.version(...)` into three `.artifact(...)` calls with no `.setup(...)` call
anywhere in between.

## What `register_block_exports` registered, and where each item now lives

The old call was `.setup(crate::register_block_exports)` at plugin root:148, defined in
`📦️packages/🦀️rust/📦️glue.rs:2009`:

```rust
pub fn register_block_exports() {
    crate::apps::block2d::register();
    crate::apps::block2d::config::schema::register_app_schema();
    crate::apps::block3d::config::schema::register_app_schema();
    crate::apps::block5d::config::schema::register_app_schema();
}
```

block3d and block5d already had `.artifact(declaration())` landed by an earlier session (M1); only
block2d's four-registry `crate::apps::block2d::register()` (io_registry composers, pilot-language
grammars, artifact schema descriptor, artifact inference descriptor, plus the document codec) was
still going through `.setup()`, alongside all three apps' `register_app_schema()` calls.

| Old call | New home |
|---|---|
| `crate::artifacts::block2d::io_registry::register()` (→ `register_composer_entries`) | `ArtifactDeclaration::builder("s.block2d").composers(…)` |
| `register_pilot_languages()` (→ 5× `dsl::register_language`) | `.languages(…)` |
| `register_artifact_schema()` (→ `register_artifact_schema_descriptor`) | `.schema(…)` |
| `register_artifact_inference()` (→ `register_artifact_inference_descriptor`) | `.inferences([…])` |
| `register_document_codec_for_app::<Block2dPlayApp>(…)` | `.document_codec::<crate::apps::block2d::Block2dPlayApp>()` |
| `block{2,3,5}d::config::schema::register_app_schema()` (CONFIG/PRESENCE schema, app-scope) | `ArtifactApp::app_schema()` override on each PlayApp, returning the renamed `config::schema::app_schema_descriptor()` — auto-registered by `register_document_app` (`🔌️plugin/🏗️builder/🦀️component.rs:152`, confirmed by reading the framework source) |

Added `pub fn declaration()` + a `pilot_languages()` helper to `🗿️artifacts/◻2d/🦀️component.rs`,
mirroring block3d/block5d's already-landed pattern byte-for-byte (same builder chain, same
`OnceLock`-leaked `&'static [dsl::LanguageSpec]` construction). Plugin root now reads:

```rust
.artifact(crate::artifacts::block2d::declaration())
.artifact(crate::artifacts::block3d::declaration())
.artifact(crate::artifacts::block5d::declaration())
```

Renamed all three `config::schema::register_app_schema()` → `app_schema_descriptor()` (returns the
descriptor instead of self-registering — exact pattern copied from `s/plugin/note`'s
`app_schema_descriptor()`/`ArtifactApp::app_schema()` pair, confirmed by reading both). Added
`fn app_schema()` to `Block2dPlayApp`/`Block3dPlayApp`/`Block5dPlayApp`'s `ArtifactApp` impls.

Also corrected two stale doc-comment references in block3d/block5d's `declaration()` (they still said
"`register_app_schema()` … one exception, still called from `.setup()`", which became false the moment
this pass emptied `.setup()`).

## `kit.catalog`

Confirmed and **not renamed**. It is `KIT_CATALOG_ARTIFACT_ID = "kit.catalog"` in
`🎛️apps/◻2d/🦀️component.rs:39`, used only as an `ArtifactKindSpec.id` inside
`create_block2d_app()`'s app manifest (the media port block2d's `"catalog:out"` produces) — an
app-scope `ArtifactKindSpec`, not a plugin-scope `ArtifactDeclaration.kind`. It is untouched by this
pass.

## Files touched (all inside this plugin, all pre-existing files edited, none created/deleted)

- `✏️s/🔌️plugins/🧱️block/🦀️component.rs` — removed `.setup(crate::register_block_exports)`, added `.artifact(crate::artifacts::block2d::declaration())`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` — deleted `register_block_exports` and its doc comment
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🦀️component.rs` — added `declaration()` + `pilot_languages()`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️component.rs`, `🗿️artifacts/🖐️5d/🦀️component.rs` — fixed stale doc comment only
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🦀️component.rs` — deleted `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()`, added `app_schema()` override
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🦀️component.rs`, `🎛️apps/🖐️5d/🦀️component.rs` — added `app_schema()` override
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`, `🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`, `🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs` — renamed `register_app_schema()` → `app_schema_descriptor()`

Note: `git status` in this shared tree also shows three unrelated files modified
(`🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🏅️standards/…/📥️import/🧩️deserializers/…/🔣️json/…/component.rs`) —
**not touched by this pass**; per this ticket's own churn-attribution rule, that is another
concurrent session's edit, not mine.

## Verification

**`#[path]` resolution** — every `#[path = "..."]` in `📦️glue.rs` resolves (script-checked, 765/765
found on disk).

**`include_str!`/`include_bytes!` resolution** — every leaf resolves across the whole crate
(script-checked, 159/159 found on disk).

**`cargo metadata --no-deps`** — exit 0 (ran workspace-wide; `-p` is not a valid `cargo metadata`
flag, confirmed by trying it first and reading the actual error).

**`cargo check -p semio-s-plugin-block --all-targets`** with the mandated
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target` override — **exit 101, NOT green.** Full output at
`scratch-w1d-block-check.txt` in this ticket folder. Classified:

```
error[E0428]: the name `inferences` is defined multiple times
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:8614:29
error[E0428]: the name `inferences` is defined multiple times
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:8719:29
error[E0433]: cannot find `inferences` in `schema`
   --> ✏️s/🔌️plugins/🗄️stdio/.../🗿️artifacts/💾️binary/.../⚙️engine/🦀️component.rs:117:88
error[E0119]: conflicting implementations of trait `ArtifactInferrer` for `Mp3Builder`
error[E0119]: conflicting implementations of trait `ArtifactInferrer` for `WavBuilder`
error: could not compile `semio-s-plugin-stdio` (lib) due to 5 previous errors; 603 warnings emitted
```

**(c) upstream, not caused by me.** All 5 errors are in `semio-s-plugin-stdio`'s own paths
(`✏️s/🔌️plugins/🗄️stdio/…`) — zero touch anything under `🧱️block`. `semio-s-plugin-block`'s
`Cargo.toml` lists `semio-s-plugin-stdio` as a direct dependency, so cargo compiles stdio first;
since stdio fails, **`semio-s-plugin-block` itself is never reached** — no `Checking
semio-s-plugin-block` line appears in the log, let alone `Finished`. `stdio` is on this ticket's
peer-held list (`✒️writer 🌊️flow 🌿️vcs 🎞️animate 🎬️sequence 🏛️architect 🏭️process 💡️reasoning
🗄️stdio`) — I did not and must not touch it. `stdio/📦️glue.rs` mtime is `Aug 12 23:56:02`, i.e.
live/minutes-old at measurement time, matching this ticket's `📓️baselines.md` note about a
mutation-vocabulary rename repeatedly leaving a dangling `inferences` mount in stdio through the
evening (three prior instances logged there). This is blocked-churn, not a verified green — the same
outcome the framework agent's W1d pass hit for energy/puzzle, for the same upstream reason.

**What I can and cannot claim:** static structure of the block crate (every `#[path]`, every
`include_str!`/`include_bytes!`, every function/type reference I added or renamed, traced by hand
against the block3d/block5d precedent already landed and green in an earlier pass) is sound by
inspection. I did **not** get a live `rustc` pass over block's own source in this run, because stdio
never let cargo schedule it. Re-run `cargo check -p semio-s-plugin-block --all-targets` (same
override) once stdio's `inferences` mount is fixed by its owning session to get the first live
confirmation.

## Summary for the caller

`.setup()` is fully eliminated from `semio-s-plugin-block`'s plugin root — no residue, no narrowed
call, nothing left that `ArtifactDeclaration`/`ArtifactApp::app_schema()` couldn't express. All five
of `register_block_exports`'s registrations (composers, languages, schema, inferences, document
codec) moved into `crate::artifacts::block2d::declaration()`, mirroring the already-landed
block3d/block5d pattern; all three apps' CONFIG/PRESENCE schema moved into
`ArtifactApp::app_schema()` per ticket W1c. `kit.catalog` is untouched, confirmed as an app-scope
`ArtifactKindSpec`, not a plugin-scope `ArtifactDeclaration.kind`. Static checks (path/include
resolution, `cargo metadata --no-deps`) are clean. The mandated `cargo check --all-targets` run is
**blocked-churn** (exit 101): 100% of the 5 errors are pre-existing, unrelated, upstream failures
inside the peer-held `semio-s-plugin-stdio` crate (dangling `inferences` mount, live edit, matches a
pattern already logged 3× this evening in this ticket's baselines doc) — `semio-s-plugin-block`
itself was never reached by the compiler, so this is not yet a verified-green result for block's own
code.
