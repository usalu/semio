# W1b — `semio-s-plugin-trinity` → `.artifact()` declarations

`apa-status: done` — both artifacts (`🔌️jack`, `♻️rewrite`) converted from `register()` free
functions called via `.setup()` to `ArtifactDeclaration`s walked via `.artifact()`, following the
`🗒️note` exemplar pattern exactly. `cargo check -p semio-s-plugin-trinity --all-targets` is 0
errors.

## Clearance

Read `📓️plugin-release-status.md` first: `🔱️trinity` is listed under **RELEASED — lane finished,
compiles in the workspace check** (`🔌️jack`, `♻️rewrite` facets) with no HELD entry anywhere else
in the file. Free to edit.

## What changed

### 1. `✏️s/🔌️plugins/🔱️trinity/🦀️component.rs` (plugin root, 9→17 lines)

`plugin()` now calls `.artifact(crate::artifacts::jack::engine::declaration())` and
`.artifact(crate::artifacts::rewrite::engine::declaration())` in place of the old
`.setup(crate::register_trinity_exports)`. `.setup()` survives, narrowed to
`crate::register_trinity_app_schemas` (see below).

### 2. `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- `register_pilot_languages()` (5 `dsl::register_language` calls) → private `pilot_languages() ->
  &'static [dsl::LanguageSpec]`, `OnceLock`-backed (same reason as note: `dsl::passthrough_hooks`
  isn't `const fn`), same 5 specs (`jack.document`, `jack.op`, `jack.diff`, `jack.pack`, `jack.spr`).
- `register_artifact_schema()` / `register_artifact_inference()` / `register()` (the
  `//#region 🔖️Register` block) → `pub fn declaration() -> ArtifactDeclaration`:
  ```rust
  ArtifactDeclaration::builder("s.jack")
      .schema(crate::artifacts::jack::schema::jack_artifact_schema_descriptor())
      .inferences([crate::artifacts::jack::standards::v1::subsets::any::schema::inferences::jack_artifact_inference_descriptor()])
      .composers(crate::artifacts::jack::standards::v1::engine::io_registry::entries())
      .languages(pilot_languages())
      .document_codec::<crate::apps::jack::TrinityJackPlayApp>()
      .build()
  ```
  `crate::apps::jack::config::schema::register_app_schema()` — the one call in the old `register()`
  with no `ArtifactDeclaration` field — moved to the plugin root's narrowed `.setup()`.

### 3. `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

Identical treatment: `pilot_languages()` (rewrite.document/op/diff/pack/spr), `declaration()` built
as `ArtifactDeclaration::builder("s.rewrite")` with `.schema(rewrite_artifact_schema_descriptor())`,
`.inferences([rewrite_artifact_inference_descriptor()])`,
`.composers(...standards::v1::engine::io_registry::entries())`, `.languages(pilot_languages())`,
`.document_codec::<crate::apps::rewrite::TrinityRewritePlayApp>()`.

### 4. `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs:957-962`

`register_trinity_exports()` (called `artifacts::jack::engine::register()`,
`artifacts::rewrite::engine::register()`, and two `register_document_codec_for_app::<...>(...)`
calls) → `register_trinity_app_schemas()`, now just:
```rust
fn register_trinity_app_schemas() {
    apps::jack::config::schema::register_app_schema();
    apps::rewrite::config::schema::register_app_schema();
}
```
Everything else that function did is now inside each artifact's own `declaration()`
(schema/inference/composer/language registration) or the `.document_codec::<A>()` builder call
(which reads `A::DOCUMENT_SCHEMA` — confirmed identical to the literal schema string the old code
passed explicitly: `TrinityJackPlayApp::DOCUMENT_SCHEMA = TRINITY_GRAPH_SCHEMA`,
`TrinityRewritePlayApp::DOCUMENT_SCHEMA = REWRITE_RULE_SCHEMA`, both at
`🎛️apps/{jack,rewrite}/🦀️component.rs`).

## Kind-string choice, verified not guessed

`ArtifactDeclaration::builder(kind)`'s ownership check requires every composer entry to
produce-or-consume `kind`. I traced the actual `Dialect` each artifact's typed composer erases to,
rather than assuming `kind` should equal the `ArtifactSchemaDescriptor.id` (which is a *different*
string, `"s.trinity.jack"` / `"s.trinity.rewrite"` — the `#[artifact_schema(id = ...)]` attribute):

- `🗿️artifacts/🔌️jack/…/🧬️schema/🦀️component.rs:213` —
  `impl ArtifactAnalysis for JackAnalyzerAnalysis { const DIALECT: Dialect = Dialect {
  artifact_kind: "s.jack", … } }`, which `derive_artifact_facets!` (line 249) wires into
  `JackComposer::WRITES`. So `kind = "s.jack"` — confirmed to match, not assumed.
- Same trace for rewrite: `…/♻️rewrite/…/🧬️schema/🦀️component.rs:179` —
  `RewriteAnalyzerAnalysis::DIALECT.artifact_kind == "s.rewrite"`.

Both also match the hand-written `JACK_DIALECT`/`REWRITE_DIALECT` constants each artifact's own
`⚙️engine` export-composer block already used for `rebuild_native_snapshot`'s dialect matching —
three independent sources agreeing, not a single guess.

## `.setup()` — survives, narrowed, why

Kept for exactly one call, `register_trinity_app_schemas` (jack's and rewrite's own
`register_app_schema()`). Both register `CONFIG`/`PRESENCE` schema for their `ArtifactApp` owner —
app-scope, not artifact-scope; `ArtifactDeclaration` has no field for this by design (see its own
doc comment and note's own W1 exemplar report, §"loudly missing" table). No other reason for
`.setup()` survives — everything else that used to run through it is now declarative.

## Step 5 — inventory (found, not touched)

- **`thread_local!`**: none in `🔱️trinity`.
- **`static`/interior-mutable state**: `ENTRIES`/`LANGUAGES` `OnceLock`s in the two engine files
  and each artifact root's `io_registry` module — derived caches (composer table, language specs),
  same category as note's own convention, not a violation. One additional hit:
  `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🗣️language-service/🦀️component.rs:14`
  — `static MANIFEST: OnceLock<math::graph::manifest::GraphManifest>` — a memoized parsed-manifest
  cache (domain data), **not** a host/engine handle (no `BrepEngineHost`-shaped type anywhere in
  this plugin) — inventoried, not a violation class hit.
- **`std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`**: one hit,
  `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs:32` — `std::process::exit(1)` inside the
  jack-shell CLI binary's own `main()`. `🔨️modules` is a real, Cargo.toml-bearing crate directory
  (confirmed: `find … -name Cargo.toml` hits both
  `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml` and
  `🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/Cargo.toml`) — inventory-only per the plugin-specific
  dispatch note ("`🔨️modules` holds real crates … inventory only, never move"), not touched.

## Step 4 — escape hatches

- `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*`:
  zero hits anywhere in `✏️s/🔌️plugins/🔱️trinity` (grepped `.rs` files). Nothing to remove.
- `semio_framework_os::` (the call-site form): zero hits.
- `semio-framework-os` (the bare crate dependency) in `Cargo.toml`: not present — only
  `semio-framework-os-kernel` and `infinite_canvas` (package `semio-framework-os-infinite`) are
  declared, both distinct crates actually imported elsewhere in this plugin (`math::graph`,
  `infinite_canvas` types), so nothing to purge.

## Step 3 — plugin root closure

Root listing: `AGENTS.md`, `README.md`, `🎛️apps`, `📦️packages`, `🗿️artifacts`, `🦀️component.rs`,
plus `🔨️modules`. `🔨️modules` is **not** one of the six allowed root entries, but it is
Cargo.toml-bearing (jack shell + LSP crates, confirmed above) and the plugin-specific dispatch note
says explicitly not to move it — flagged here per instructions, left untouched, not a finding I'm
claiming to have fixed.

## Verification

**1. `#[path]` mounts** — 310 in `📦️glue.rs`, scripted resolution check against the real
filesystem: **0 missing**.

**2. `include_str!`/`include_bytes!`** — 104 across the crate, scripted resolution against the real
target file (not pattern-substituted): **0 missing**.

**3. `cargo metadata`**:
```
$ cd ✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust && cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-trinity --all-targets`** (`RUSTC_WRAPPER=""`,
`CARGO_TARGET_DIR=".../🎯️target"`) — **3 attempts, exactly as documented in note's own W1 report as
the expected shape of concurrent churn**, not silently retried away:
- Attempt 1: `error: couldn't read …/🗄️stdio/…/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs:
  No such file or directory` — a `#[path]` mount inside **`semio-s-plugin-stdio`'s own** glue.rs,
  zero mentions of `🔱️trinity` anywhere in the output (grep-verified,
  `scratch-w1b-trinity-check-1.txt`). `stdio`'s `glue.rs` mtime was minutes old at the time —
  live churn, matching `📓️plugin-release-status.md`'s own note that `🗄️stdio` is UCAS's
  in-flight roster restructure, not frozen.
- Attempt 2: different failure, **9× `E0599` inside `semio-s-plugin-stdio`'s own**
  `SemioDrawingMutation` enum (`MoveNode`/`DragNodes`/`Rotate`/`Scale`/… variants not found) — again
  zero mentions of `🔱️trinity` (`scratch-w1b-trinity-check-2.txt`). Error content changing between
  attempts while trinity stays clean is exactly the stdio-mutation-rename-in-flight signature
  `📓️plugin-release-status.md` documents.
- Attempt 3: clean.
```
warning: `semio-s-plugin-trinity` (lib) generated 53 warnings (45 duplicates) (run `cargo fix --lib -p semio-s-plugin-trinity` to apply 6 suggestions)
warning: `semio-s-plugin-trinity` (lib test) generated 54 warnings (2 duplicates) (run `cargo fix --lib -p semio-s-plugin-trinity --tests` to apply 47 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 5m 58s
```
**0 errors** (`grep -c '^error' scratch-w1b-trinity-check-3.txt` → `0`). Full raw output preserved
at `scratch-w1b-trinity-check-{1,2,3}.txt` in this ticket folder. The only two trinity-attributed
warnings are pre-existing dead-code notes on `TrinityGraphEngine`/`RewriteRuleEngine`'s unread
`artifact`/`snapshot` fields — unrelated to this diff (those structs and fields are untouched).

**5. Runtime ownership-check confidence** (not exercised by `cargo check`, since
`ArtifactDeclaration::register_all`'s asserts only run when `plugin()` is actually invoked by a
host/boot path, not by `cargo check`/`cargo test`): traced `kind` against the derive-generated
`ArtifactComposer::WRITES` for both artifacts (see "Kind-string choice" above) rather than relying
on compilation alone to catch a wrong string.

## sharedFileRequests

None. Everything touched is inside `✏️s/🔌️plugins/🔱️trinity`. `🔨️modules` (Cargo.toml-bearing,
inventory-only per this plugin's own dispatch note) was read but not modified or moved.

## apa-status

`done` — mechanism applied to both of trinity's artifacts, `.setup()` narrowed to its one legitimate
app-schema call, 0 compile errors on `--all-targets` (3rd attempt, first two failed exclusively
inside `semio-s-plugin-stdio`'s own in-flight churn, verified by grep and mtime, not trinity's).
