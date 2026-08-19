# terra / fleet-trinity-recipe — Trinity migration to `.declare_artifact(...)`

Packet: `fleet-trinity-recipe`, executor `terra`, ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.

## Goal recap

🔱️trinity's `Plugin::builder("trinity")...try_build()` failed assembly (`"no declared <kind>
capability owns the runtime claims"`) because it still registered through the OLD
`.artifact(declaration())` channel. Task: migrate to the NEW `.declare_artifact(artifact())` channel,
following `🗒️note`/`🖍️draw` as templates, without touching any path containing `🚪️io/` or any
`✏️editor/🦀️component.rs` (excluded — a live peer packet, `io-async-signatures`, is mid-sweep there).

## Result

`cargo check -p semio-s-plugin-trinity --lib` (own `CARGO_TARGET_DIR`) **PASSED** — `Finished `dev`
profile [unoptimized] target(s) in 17.81s`, exit code 0, zero errors, zero warnings in any file this
packet touched (all warnings in the log are pre-existing, in framework files unrelated to this change).
Full log kept at `terra-fleet-trinity-recipe-cargo-check.txt` in this folder.

Trinity now registers both owned artifacts (`jack`, `rewrite`) exclusively through
`.declare_artifact(...)`. The old `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` calls are
gone from the plugin root; `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay (orthogonal
opt-in, not a second registration channel, per `🗒️note`'s own doc).

## Files changed

### `✏️s/🔌️plugins/🔱️trinity/🦀️component.rs` (plugin root)

Before:
```rust
.artifact(crate::artifacts::jack::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
.artifact(crate::artifacts::rewrite::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
.editor::<crate::editor::jack::TrinityJackPlayApp>(crate::editor::jack::create_trinity_jack_app())
.editor_mutation_roster::<crate::editor::jack::TrinityJackPlayApp>()
.viewer::<crate::viewer::jack::TrinityJackViewer>(crate::viewer::jack::create_trinity_jack_viewer())
.viewer_mutation_roster::<crate::viewer::jack::TrinityJackViewer>()
.editor::<crate::editor::rewrite::TrinityRewritePlayApp>(crate::editor::rewrite::create_rewrite_app())
.editor_mutation_roster::<crate::editor::rewrite::TrinityRewritePlayApp>()
.viewer::<crate::viewer::rewrite::TrinityRewriteViewer>(crate::viewer::rewrite::create_trinity_rewrite_viewer())
.viewer_mutation_roster::<crate::viewer::rewrite::TrinityRewriteViewer>()
```
After:
```rust
.declare_artifact(crate::artifacts::jack::artifact())
.declare_artifact(crate::artifacts::rewrite::artifact())
.editor_mutation_roster::<crate::editor::jack::TrinityJackPlayApp>()
.viewer_mutation_roster::<crate::viewer::jack::TrinityJackViewer>()
.editor_mutation_roster::<crate::editor::rewrite::TrinityRewritePlayApp>()
.viewer_mutation_roster::<crate::viewer::rewrite::TrinityRewriteViewer>()
```
`.activation(...)`/`.execution(...)`/`.requests(...)` unchanged, as required.

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs`

- `fn pilot_languages()` → `pub fn pilot_languages()` (needed by the new subset-root file, see below).
- `definition()` KEPT verbatim (debt D1, unread — real en/de localized names `"Jack"`/`"Buchse"` live
  only there), doc comment updated to match `🗒️note`/`🖍️draw`'s wording.
- `declaration()` (old-channel `ArtifactDeclaration::builder(definition()?).schema(...).inferences(...)
  .composers(...).languages(...).document_codec(...).try_build()`) **DELETED**.
- New `pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration`:
  ```rust
  pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
      use semio_framework_plugin::app::declarations::ArtifactDeclaration;
      use store::os_io::ArtifactKindId;
      ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.jack").expect("canonical jack kind"), localization: &[], standards: vec![crate::artifacts::jack::standards::v1::standard()] }
  }
  ```

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs`

Same three changes, mirrored for rewrite (`kind: "s.trinity.rewrite"`).

### NEW `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🦀️component.rs`

`pub fn standard() -> StandardDeclaration`, mounting `subsets::any::subset()`. `mimes` is a documented
synthesis (`application/vnd.semio.jack+json`, same convention `🗒️note`/`🖍️draw` used — no real MIME
claim existed pre-migration); `extensions: &["trinity"]` is the real carried-over value from the old
codec capability row (jack's own D2 comment: the codec extension is `"trinity"`, not `"jack"`).

### NEW `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🦀️component.rs`

Same shape for rewrite: `mimes: ["application/vnd.semio.rewrite+json"]`, `extensions: ["rewrite"]`.

### NEW `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`

`pub fn subset() -> SubsetDeclaration` — dialect `TRINITY_JACK_DIALECT`, `schema` from the existing
`schema::jack_artifact_schema_descriptor()`/`schema::inferences::jack_artifact_inference_descriptor()`,
`viewer`/`editor` from the existing `viewer::create_trinity_jack_viewer()`/`editor::create_trinity_jack_app()`
(both already built via `Viewer::builder(TRINITY_JACK_DIALECT)`/`Editor::builder(TRINITY_JACK_DIALECT)`,
so `check_surface_id` is satisfied for free), `examples` from `crate::artifacts::jack::examples::demo::source()`.

`io` is the one field that deviates from the template — see "What did NOT generalize" below.

### NEW `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`

Mirror of the jack file for rewrite (`TRINITY_REWRITE_DIALECT`, `RewriteSnapshot`/`RewriteRuleMutation`,
`REWRITE_RULE_SCHEMA`).

### `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs`

This repo's Rust module tree is **hand-maintained** in this one file via `#[path = "..."] mod
component; pub use component::*;` — it is NOT auto-derived from the directory tree. New
`🦀️component.rs` files (the two `standard()` roots and the two `subset()` roots above) do not become
reachable at `crate::artifacts::{jack,rewrite}::standards::v1[::subsets::any]` until a matching mount
is added here. Added, for both `jack` and `rewrite`, inside the existing `pub mod v1 { ... }` block
(new `standard()` mount, first line inside `v1`, before `pub mod subsets`) and inside the existing
`pub mod any { ... }` block (new `subset()` mount, first line inside `any`, before `pub mod schema`).
**This file is the trap every one of the remaining 20 plugins will also hit** — see the recipe below.

## THE RECIPE — migrating the remaining 20 old-channel plugins

Preconditions: read `🗒️note`'s `🦀️component.rs` + `🗿️artifacts/🗒️note/🦀️component.rs` +
`🏅️standards/🔖️1/🦀️component.rs` + `🪆️subsets/✳️any/🦀️component.rs` end to end first (or `🖍️draw`'s
equivalents) — they are the ground truth this recipe compresses.

1. **Inventory the plugin's artifacts.** `python3 -c "os.walk(...)"` the plugin's `🗿️artifacts/`
   directory, depth ≤1, for `🦀️component.rs` files — one per owned artifact (trinity had 2: `jack`,
   `rewrite`; all 11 already-migrated plugins had exactly 1, so trinity was the first proof that
   `.declare_artifact(...)` is genuinely repeatable — confirmed at
   `🧰️framework/…/🔌️plugin/🏗️builder/🦀️component.rs`'s own doc: `"Declares one artifact this plugin
   owns. Repeatable."`). Do the WHOLE recipe below once per artifact.

2. **Read the artifact root `🦀️component.rs`.** Find `definition()` (old `ArtifactDefinition`
   capability rows) and `declaration()` (old `ArtifactDeclaration::builder(definition()?)
   .schema(...).inferences(...).composers(...).languages(...).document_codec(...).try_build()`).
   Grep the WHOLE repo for other callers of `declaration()` before deleting it (`grep -rn
   "<plugin>::declaration"` — in trinity's case the only caller was the plugin root itself).

3. **Find these 5 pre-existing functions** (already exist for any plugin that had a working
   `declaration()` — none of this is new work, just relocation):
   - the schema descriptor fn, e.g. `<x>_artifact_schema_descriptor()` — under
     `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`.
   - the inference descriptor fn — under `.../🧬️schema/💡️inferences/🦀️component.rs`.
   - the `Dialect` const (e.g. `TRINITY_JACK_DIALECT`) — usually on the artifact root file, `pub`.
   - the editor/viewer manifest fns (`create_<x>_app()`/`create_<x>_viewer()`) — under
     `.../✏️editor/🦀️component.rs` / `.../👁️viewer/🦀️component.rs`, built via
     `Editor::builder(DIALECT)...build_definition()` / `Viewer::builder(DIALECT)...build_definition()`.
     If they already use the artifact's own `Dialect` const this way, `check_surface_id` (the new
     channel's own preflight, `commit_artifact_declarations` in the framework's plugin
     `🦀️component.rs`) passes for free — nothing to change here.
   - the example source fn — under `.../📚️examples/🎬️demo/🦀️component.rs::source()`. **Import it via
     the plugin-root SHIM path** (`crate::artifacts::<x>::examples::demo::source()`), not the deep
     `standards::v1::subsets::any::examples::demo::source()` path — the deep path does NOT resolve
     unless the plugin's `📦️glue.rs` happens to mount `examples` at that exact nesting (jack/rewrite's
     did not; only `schema`/`io`/`op`/`dsl`/`spr`/`diff`/`mutations`/`snapshot`/`examples` are
     re-exported as shims directly under the artifact root — see trap 1 below).

4. **Write the standard-root file** `🏅️standards/🔖️1/🦀️component.rs` (does not exist pre-migration —
   you are creating it): `pub fn standard() -> StandardDeclaration { StandardDeclaration { id:
   StandardId("1"), media: MediaDeclaration { mimes: &[...], extensions: &[...] }, subsets:
   vec![subsets::any::subset()] } }`. `extensions` = the real value from the old `definition()`'s
   `"codec"` capability row's `extension` claim (read it, do not guess — jack's own D2 comment shows
   codec extension can diverge from the artifact's own name, e.g. `"trinity"` not `"jack"`). `mimes`
   has no real source pre-migration for ANY of the plugins checked so far (note/draw/jack/rewrite all
   lacked a mime capability row) — synthesize `application/vnd.semio.<identity-root>+json` where
   `<identity-root>` is the old `ArtifactIdentity::parse("s.<x>")` string, and say so in a doc comment
   exactly like `🗒️note`'s.

5. **Write the subset-root file** `🪆️subsets/✳️any/🦀️component.rs` (also new): `pub fn subset() ->
   SubsetDeclaration { dialect: <DIALECT_CONST>, schema: SchemaDeclaration { descriptor:
   schema::<x>_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services:
   Vec::new() }, io: io::io(), viewer: viewer_surface::<Viewer>(viewer::create_<x>_viewer()), editor:
   editor_surface::<Editor>(editor::create_<x>_app()), examples: examples() }`, with `examples()`/
   `inference_descriptors()` as `OnceLock`-cached private fns exactly like `🗒️note`/`🖍️draw`. **`io:
   io::io()` only works if `🚪️io/🦀️component.rs` already has (or you are allowed to add) an `io()`
   function** — see trap 2.

6. **Mount both new files in `📦️glue.rs`** (trap 3, the one that will silently block every packet
   here — `cargo check` errors `E0425 cannot find function 'standard' in module
   '...::standards::v1'` even though the file compiles standalone). Find the artifact's
   `pub mod <artifact> { ... pub mod standards { pub mod v1 { pub mod subsets { pub mod any { ... } }
   } } }` block. Insert, as the FIRST line inside `pub mod v1 { ... }` (before `pub mod subsets`):
   ```rust
   #[path = "../../🗿️artifacts/<artifact>/🏅️standards/🔖️1/🦀️component.rs"]
   mod component;
   pub use component::*;
   ```
   and, as the FIRST line inside `pub mod any { ... }` (before `pub mod schema`):
   ```rust
   #[path = "../../🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
   mod component;
   pub use component::*;
   ```
   Every `#[path]` in this file is relative to `📦️glue.rs`'s OWN directory regardless of nesting depth
   (every leaf uses the same `../../🗿️artifacts/...` prefix) — copy that exact prefix, do not try to
   shorten it per nesting level.

7. **Update the plugin root**: replace every `.artifact(<artifact>::declaration()...)`,
   `.editor::<E>(...)`, `.viewer::<V>(...)` call with one `.declare_artifact(<artifact>::artifact())`
   per artifact. Keep `.editor_mutation_roster()`/`.viewer_mutation_roster()`,
   `.activation(...)`/`.execution(...)`/`.requests(...)` untouched.

8. **Add `pub fn artifact()` to the artifact root, delete `declaration()`, keep `definition()`** (debt
   D1 — do not delete it repo-wide, this pass is not that ticket). Make any `fn` that used to be
   `declaration()`-private but is now needed by the new subset-root file (e.g. `pilot_languages()`)
   `pub`.

9. **`cargo check -p <plugin-crate> --lib`** with an isolated `CARGO_TARGET_DIR` once, at the end.

### What varies per plugin (check before assuming trinity's shape)

- **Number of artifacts** — most of the remaining 20 likely have 1 (matches all 11 already-migrated
  plugins); confirm with the depth-≤1 `🦀️component.rs` walk in step 1 before assuming.
- **Composers/languages present or not** — trinity's jack/rewrite both had hand-authored
  `pilot_languages()` (5 real `dsl::LanguageSpec`s: document/op/diff/pack/spr) AND real
  `store::ArtifactCodec::of::<Snapshot, Mutation>(...)` bounds already satisfied by the OLD
  `.document_codec::<EditorApp<...>>()` call. If a plugin's old `declaration()` had NO
  `.languages(...)` call at all, its `NativeCodecs` should use `LanguagePair { text: None, binary:
  None }` for every role (matches design.md's documented fallback), not fabricated grammars.
- **Inference services** — none of jack/rewrite/note/draw had any; if a plugin's old
  `declaration()` called `.inference_services(...)`, carry the real list into `SchemaDeclaration.
  inference_services` instead of `Vec::new()`.
- **Standards/subsets count** — trinity, note, draw all had exactly 1 standard × 1 subset (`"1"` /
  `"any"`). A plugin with more (the framework's own W1-C fixture test proves 2 standards × 3 subsets
  is supported) needs one `🏅️standards/🔖️<n>/🦀️component.rs` + one `🪆️subsets/✳️<subset>/🦀️component.rs`
  per (standard, subset) pair, and `ArtifactDeclaration.standards: vec![...]` listing all of them.

## What did NOT generalize / deviated from the template

**`io: io::io()` could not be used.** `🗒️note`/`🖍️draw` both delegate to a same-named `io() ->
IoDeclaration` function their OWN migration added to `🚪️io/🦀️component.rs`. This packet's boundary
excludes every path containing `🚪️io/` (live peer packet `io-async-signatures` mid-sweep). Trinity's
`🚪️io/🦀️component.rs` files (jack and rewrite) are STILL on the fully old channel — `ArtifactComposition`
+ `io_registry::entries()` (`ComposerEntry`-style, ticket 26/08/12), not the new `Serializer<S>`/
`Deserializer<S>` + `serializer_entry`/`deserializer_entry` typed-trait shape note's `🚪️io/🦀️component.rs`
uses. Converting them is real, non-trivial migration work (hand-authoring typed `Deserializer`/
`Serializer` impls per foreign format, matching note's own `📥️import/🧩️deserializers`/
`📤️export/🧵️serializers` tree) that belongs inside the excluded file.

Worked around by defining `io_declaration()` locally in each new subset-root file instead of `io::io()`:
- `native: NativeCodecs` is REAL — reuses the artifact's own (now-`pub`) `pilot_languages()` for the
  five `LanguageSpec`s (same index mapping note's `io()` uses: `[0]`=document→snapshot.text,
  `[3]`=pack→snapshot.binary, `[2]`=diff→diff.text, `[1]`=op→mutations.text, `[4]`=spr→mutations.binary)
  and a real `store::ArtifactCodec::of::<Snapshot, Mutation>(SCHEMA)`.
- `entries: &[]` — the foreign-format hops (jack: svg/csv/md/png/json import+export; rewrite:
  txt/pdf/docx/md/json import+export) are **not registered on the new `io_mechanism` channel**. This
  is a real, honest gap, not an oversight — every foreign-format composer capability the OLD
  `definition()` claimed is now unreachable from the new channel (the new channel never reads
  `definition()` at all). `try_build()` still succeeds because `entries: &[]` trivially passes
  `preflight_io_entries`/the `io_register` commit step (empty batch).

I predict every one of the remaining plugins with real composers hits the exact same wall UNLESS its
own `🚪️io/🦀️component.rs` is not excluded from that packet's ownership — in which case that packet
should do the FULL migration (relocate the `io()` aggregator into `🚪️io/🦀️component.rs` per the
template, hand-author the typed `Serializer`/`Deserializer` impls) rather than reproduce this
workaround. This workaround should be treated as a stopgap specific to trinity's boundary conflict
with `io-async-signatures`, not a new pattern to copy forward.

## Lease-request

```
File: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs
File: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs

Once io-async-signatures (or whichever packet next owns these paths) is done: add a
`pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration` to each file, built the
same way `🗒️note`'s own `🚪️io/🦀️component.rs::io()` is (native codec unchanged — copy verbatim from
this packet's `io_declaration()` in the sibling `🪆️subsets/✳️any/🦀️component.rs` file, which is
already correct — only `entries` needs real work): hand-author `Deserializer<JackSnapshot>`/
`Serializer<JackSnapshot>` (and the rewrite equivalents) impls for each of the OLD channel's
foreign-format composer rows currently in this same file's `derived_composition`/`io_registry`
modules (jack: svg/csv/md/png/json; rewrite: txt/pdf/docx/md/json — both import+export), wire them
through `serializer_entry`/`deserializer_entry`/`_text`, and set `entries` to the resulting
`&'static [IoEntry]`. Then, in
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs and its
rewrite twin: delete the local `io_declaration()` fn and its `⚠️ DEVIATION` doc comment, change
`use ...` to add `crate::artifacts::<x>::standards::v1::subsets::any::io`, and change
`io: io_declaration()` to `io: io::io()` — matching 🗒️note/🖍️draw exactly, closing this packet's one
open deviation. Also safe to delete the OLD `ArtifactComposition`/`io_registry` modules in the same
🚪️io/🦀️component.rs file at that point, once nothing references them.
```

## Acceptance

```
$ CARGO_TARGET_DIR=.../scratchpad/target-trinity cargo check -p semio-s-plugin-trinity --lib
   ... (framework deps compile; only pre-existing warnings, none in files this packet touched) ...
    Checking semio-s-plugin-trinity v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 17.81s
$ echo $?
0
```
Exit code 0. Full log saved at
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-fleet-trinity-recipe-cargo-check.txt`.

Not run: `cargo test` (forbidden by this packet's brief — build lock held by another session).
`try_build()`'s `Result` is therefore unverified at runtime; reasoning only: the specific bug this
ticket targets (`require_declared_capability_or_record`'s claim-set equality check) lives exclusively
in the OLD channel's `ArtifactDeclaration::builder(...).try_build()` path (`🧰️framework/.../🔌️plugin/
🦀️component.rs` ~line 2901), which trinity's plugin root no longer calls at all — the new channel's
own `commit_artifact_declarations`/`preflight_artifact_declarations` never runs that check. Every
other preflight it does run (schema/inference/language/codec/format/surface-id) uses data this packet
reused unchanged from what the old, previously-compiling `declaration()` already fed the SAME
underlying preflight functions — so failure would indicate a real regression the coordinator's test
run should surface, not blind confidence expressed here as a passing claim.

## Files touched (for `ticket_close`, NOT run by this packet)

- Modified: `✏️s/🔌️plugins/🔱️trinity/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs`
- Created: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
- NOT touched: anything under `🚪️io/`, `✏️editor/🦀️component.rs` (excluded per packet brief).
