# W1b — `📜️imperative` (`semio-s-plugin-imperative`) — `register()` → `declaration()` conversion

`apa-status: complete`

## Clearance (Step 0)

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` lists
`📜️imperative` explicitly under **"RELEASED — Wave C / late Wave M lanes complete"**: *"struct-with-
`CollectionMutation` replaced by a 4-variant enum, 8 app handlers rewired"*, plus the blanket
`cargo check --workspace` → 0 errors in any `✏️s/🔌️plugins` crate note. Not present in either HELD
section. Free to edit.

## What changed

### 1. `📜️imperative`'s artifact engine — `register()` → `declaration()`

File: `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- **:39-125 (region `🔖️Register`)** — the old side-effecting `pub fn register()` (5 calls:
  `crate::artifacts::imperative::io_registry::register()`, `bootstrap_imperative_runtime()`,
  `register_artifact_schema()`, `register_artifact_inferences()`, `register_pilot_languages()`,
  plus a direct `register_document_codec_for_app::<ImperativePlayApp>` call) replaced by:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      bootstrap_imperative_runtime();
      semio_framework_plugin::ArtifactDeclaration::builder("s.imperative")
          .schema(crate::artifacts::imperative::schema::imperative_artifact_schema_descriptor())
          .inferences([crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::imperative_artifact_inference_descriptor()])
          .composers(io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::imperative::ImperativePlayApp>()
          .build()
  }
  ```
  Single artifact, single standard (`🔖️1`), single subset (`✳️any`) — one `declaration()` covers
  the whole artifact.
- **`kind: "s.imperative"`** — matches the composer table's own `IMPERATIVE_DIALECT.artifact_kind`
  (this same file's `io_registry` module, unmoved) exactly, mirroring note's exemplar keying off
  `"s.note"`. **Not** `"computation.imperative"` — that string is `ArtifactKindSpec.id` (a separate
  OS media-capability registration in the artifact root's own `artifact_kind()`, untouched) and
  would fail `register_all`'s ownership check against every composer entry (import entry +
  3 export entries), all of which read/write `"s.imperative"`.
- **`bootstrap_imperative_runtime()` called eagerly at the top of `declaration()`, not moved into
  `.setup()`.** This is NOT a §6 registrar (`register_language`/`register_artifact_schema_descriptor`
  are §6 and now live in the builder chain) — it is this artifact's own native-module bootstrap
  (`register_native_imperative_module` × 4 + `register_default_imperative_contributions`,
  populating imperative's compute-extension registry). It is `Once`-guarded, so calling it here
  reproduces the OLD `register()`'s eager timing exactly: native modules are populated before any
  `ImperativeHost`/`ImperativePlayApp::render()` call could otherwise observe an empty registry
  (`render()` calls `imperative_engine::sync_imperative_module_contributions` unconditionally on
  every render, with no bootstrap call of its own — verified by grep, only two call sites for
  `bootstrap_imperative_runtime` exist repo-wide: here and `ImperativeHost::from_snapshot`).
  Putting it in `.setup()` instead would have given `.setup()` a second purpose, which the ticket's
  own Step 2 rule treats as a reportable finding — see "Does `.setup()` survive" below for why I
  judged folding it into `declaration()` (still a plain `fn` with a documented, `Once`-guarded,
  idempotent side effect) the correct call instead.
- **`register_pilot_languages()` → private `pilot_languages() -> &'static [dsl::LanguageSpec]`** —
  same 5 language specs (`imperative.document`/`imperative.imperative.op`/`imperative.imperative.diff`/
  `imperative.pack`/`imperative.spr`), verbatim, now built once behind a `OnceLock<Vec<_>>` and
  leaked to `&'static` (mirrors note's own helper — `dsl::passthrough_hooks` isn't `const fn`).
- **`register_artifact_schema()`/`register_artifact_inferences()` deleted** (old :466-478, region
  `🔖️SchemaRegistry`) — both had exactly one call site each (inside the old `register()`, confirmed
  by grep before deleting); their bodies are now inline `.schema(...)`/`.inferences([...])` builder
  arguments.
- `.composers(...)` points at this same file's own `io_registry::entries()` (the real 4-entry
  table: `composer_entry_of::<ImperativeAnyComposer>()` plus 3 hand-written export entries —
  CSV/MD/JSON). This is the same table the artifact root file's own thin `io_registry::register()`
  wrapper already forwarded to via `register_composer_entries(v1::entries())` — the declaration
  points at the real source directly, not that wrapper.

### 2. Artifact root — orphaned wrapper left in place

`🗿️artifacts/📜️imperative/🦀️component.rs`'s own `io_registry` module (:92-114, thin
`entries()`/`compose()`/`register()` wrapper around the engine file's real table) is now orphaned
(zero call sites repo-wide, confirmed by grep) — left in place rather than deleted, matching what
the W1 report did for note's own equivalent orphaned module: removing it is unrelated cleanup
outside this wave's scope.

### 3. Plugin root — `.setup()` → `.artifact()`

File: `✏️s/🔌️plugins/📜️imperative/🦀️component.rs`

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .setup(crate::apps::imperative::config::schema::register_app_schema)
        .artifact(crate::artifacts::imperative::engine::declaration())
        .register_document_app::<crate::apps::imperative::ImperativePlayApp>(crate::apps::imperative::create_imperative_app())
        .build()
}
```
The old `fn register_exports() { engine::register(); config::schema::register_app_schema(); }`
free function is gone; `.setup()` now points directly at `register_app_schema`.

## Does `.setup()` survive, and why

**Yes, narrowed to exactly one call**: `crate::apps::imperative::config::schema::register_app_schema`.
This registers `ImperativePlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
(`register_app_schema_descriptor`), one of the two §6 functions `ArtifactDeclaration` deliberately
has no field for (mechanism doc at `🔌️plugin/🦀️component.rs:935-938`; W1 report's field-mapping
table). Identical shape to note's exemplar.

**One judgment call flagged prominently, per the dispatch's own instruction**: `bootstrap_imperative_runtime()` — imperative's native-compute-module bootstrap — is neither
a §6 artifact registrar nor app-scope schema registration. I did **not** add it as a second
`.setup()` purpose; I kept it as an eager call inside `declaration()` itself (a plain `fn`, still
executed once at the same point in program order as before — when `plugin()` builds — since
`declaration()` is evaluated as an argument to `.artifact(...)`). This preserves the exact old
timing/behavior (verified: only two call sites for this function exist repo-wide, both `Once`-
guarded, so idempotent regardless of call order) without giving `.setup()` a second reason to
exist. If this artifact ever grows a "genesis"/"engine-bootstrap" field on `ArtifactDeclaration`
proper, this call belongs there instead — noted for whoever runs that follow-up, same category as
the `🌿️vcs` `seed()` gap the W1 report flagged.

## Step 4 — escape hatches and deps

- `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_\|semio_framework_os::"` across the whole plugin: **zero matches**. `semio-framework-os` is not
  even a dependency in `📦️packages/🦀️rust/Cargo.toml` (only `semio-framework-os-kernel`, a
  different crate — the kernel). Nothing to relocate, delete, or purge.

## Step 5 — inventory

- `thread_local!`: **0** matches anywhere in the plugin.
- `static`/`OnceLock` declarations (5 total, all inventoried, none a host/engine handle):
  - `🗿️artifacts/📜️imperative/🦀️component.rs:97` `static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>` — the orphaned root-file wrapper's own cache (see above).
  - `⚙️engine/🦀️component.rs:11` `static ENTRIES: OnceLock<String>` — memoized default contributions JSON.
  - `⚙️engine/🦀️component.rs:27` `static ONCE: Once` — the bootstrap guard itself.
  - `⚙️engine/🦀️component.rs:66` (new) `static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>` — this conversion's own `pilot_languages()` cache.
  - `⚙️engine/🦀️component.rs:473` `static ENTRIES: OnceLock<Vec<ComposerEntry>>` — the real composer table cache (`io_registry::entries()`).
  - None hold a host/engine handle (no `OnceLock<ImperativeHost>` or similar anywhere) and none are
    interior-mutable app-gesture/draft state — all five are lazily-built immutable derived caches
    or a one-shot bootstrap guard.
  - `⚙️engine/🦀️component.rs:6` imports `Mutex` (`use std::sync::{Mutex, Once, OnceLock}`) but never
    uses it — a pre-existing unused import, unrelated to this conversion, not touched.
- Instance-owned interior mutability (not `static`, so not a violation-class item, listed for
  completeness): `🎛️apps/📜️imperative/🌉️wasm/🦀️component.rs:18` `ImperativeSession { state:
  Rc<RefCell<ImperativeSessionInner>> }` — a per-session WASM bridge handle owned by the struct
  instance, not a global.
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`: **0** matches anywhere
  in the plugin.

## Step 3 — plugin root closure

`✏️s/🔌️plugins/📜️imperative/` already contained only `🦀️component.rs`, `AGENTS.md`, `🎛️apps/`,
`📦️packages/`, `🗿️artifacts/`, `🧩️extensions/` before this session touched it — no
`🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs to delete, no stray root data files. `🧩️extensions/`
is crate-bearing (`Cargo.toml` under each of its 5 sub-dirs), confirmed inventory-only per the
plugin-specific note, not touched. Nothing to do for this step.

## Unrelated pre-existing breakage found and fixed (stdio_gap)

Getting `semio-s-plugin-imperative --all-targets` to compile for the first time surfaced 9 real,
pre-existing compile errors in this crate's own CSV/MD/JSON stdio bridge leaves — confirmed
pre-existing by `stat -f '%Sm'` (all six files last touched **Aug 12 10:50**, hours before this
session) and `git log` (last commits well before my edits): a concurrent stdio wave reshaped
`CsvSnapshot` (`headers`/`rows` → `has_header` + index-keyed `records: Vec<CsvRecord>`),
`MdSnapshot` (`body: String` → `blocks: Vec<MdBlock>`, a real CommonMark tree), and
`JsonSnapshot.value` (`serde_json::Value` → stdio's own lexeme-preserving `JsonValue`), and
imperative's own leaves were never updated to follow. This is the exact `stdio_gap` class the W1
report and `🗒️note`'s own JSON leaf already document; **`🔱️trinity`'s `🔌️jack` artifact already
carries the identical fix** (same `ArtifactDsl::parse_dsl`/`print_dsl` shape), so I ported its
established pattern rather than inventing one:

- **CSV** (`🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs`)
  — import now returns `ImperativeSnapshot::default()` (mirrors jack's own stub: an arbitrary CSV
  grid has no tabular correspondence to a `Path` of `Step`s — the old code's `{"headers":..,"rows":..}`
  bridge could never have deserialized into a real `ImperativeSnapshot` either, so this is no
  regression). Export now writes one header record (`"payload"`) + one data record holding the
  printed DSL text via `CsvField`/`CsvRecord`.
- **MD** — bridges through stdio's own `render_markdown_blocks`/`parse_markdown_blocks` to flatten/
  wrap the DSL text against the new block tree, exactly as jack does.
- **JSON** — bridges through stdio's own `write_json_text`/`parse_json_text`/`write_json_pretty`
  codec instead of a hand-rolled `JsonValue` converter (also fixes a latent bug the old code carried:
  `serde_json::to_vec_pretty(&value)` on the new `JsonValue` would have serialized the internally-
  tagged enum shape verbatim, not real JSON text).

All six files' `register()`/`deserialize_bytes`/`serialize_bytes` helpers that did **not** reference
the renamed fields were left untouched — only the bodies that actually referenced
`headers`/`rows`/`body`/mismatched `Value`/`JsonValue` were rewritten. This is squarely inside
`📜️imperative`'s own territory (not stdio's), the fix is a direct port of an already-compiling
sibling's pattern (not invented design), and leaving it broken would have made "one clean
`cargo check --all-targets`" impossible to honestly report.

## Verification

**1. `#[path]` mounts in `📦️glue.rs` resolve** — scripted (Python, normpath + `os.path.isfile`
against each literal `#[path = "..."]` string): plugin's own glue.rs **63 checked, 0 missing**; all
5 `🧩️extensions/*/📦️packages/🦀️rust/📦️glue.rs` files **1 checked, 0 missing** each.

**2. `include_str!`/`include_bytes!` targets resolve** — scripted (walked every `.rs` file under the
plugin, resolved each literal target relative to its own containing file's directory):
```
total include! targets: 49
missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-imperative --all-targets`** (RUSTC_WRAPPER disabled) — 5
attempts total, retry-and-wait protocol (never patched a foreign file):
- Attempt 1: `error: couldn't read .../🗄️stdio/.../✳️drawing/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs: No such file or directory` — a directory that didn't exist at all
  (`✳️drawing/🧬️schema/🧬️mutations/` itself missing), confirmed a concurrent session's in-flight
  restructure via `stat`/`git log` (uncommitted, mtime inside this session's window). Zero mentions
  of `📜️imperative` in the output.
- Attempt 2: stdio moved on, new error — `SemioDrawingMutation` missing `DeleteLayer`/`CreateLayer`
  variants (same concurrent stdio wave, different file). Zero mentions of `📜️imperative`.
- Attempt 3: stdio compiled clean, exposing the **9 real pre-existing errors in imperative's own
  CSV/MD/JSON leaves** (see above) — fixed as described.
- Attempt 4: stdio churned again — `#[derive(Mutations)]` const-eval panic on `SemioDrawingMutation`
  kebab-slug mismatches (`GroupNodes`/`UngroupNode`/`FlattenNode`/`UnflattenNode`), same drawing
  facet, different defect. Zero mentions of `📜️imperative`.
- **Attempt 5 — clean:**
  ```
  $ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-imperative --all-targets
  warning: `semio-s-plugin-imperative` (lib) generated 27 warnings (run `cargo fix --lib -p semio-s-plugin-imperative` to apply 18 suggestions)
  warning: `semio-s-plugin-imperative` (lib test) generated 28 warnings (26 duplicates)
      Finished `dev` profile [unoptimized] target(s) in 1m 59s
  ```
  **0 errors** (`grep -c "^error"` → 0). All 27/28 warnings are pre-existing style lints
  (unused imports, unnecessary qualifications, dead code) unrelated to this conversion; none
  reference a line this session touched.

## sharedFileRequests

None. All edits are inside `✏️s/🔌️plugins/📜️imperative/`, the plugin this dispatch assigned. The
stdio breakage encountered during verification (drawing mutation kebab-slug/variant churn) was
never touched — confirmed transient by re-running, resolved itself by attempt 3.

## Files touched

- `✏️s/🔌️plugins/📜️imperative/🦀️component.rs` — `.setup(register_exports)` → `.setup(register_app_schema)` + `.artifact(engine::declaration())`.
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inferences()` → `declaration()` + private `pilot_languages()`.
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{📊️csv/🔖️rfc4180,📝️md/🔖️commonmark,🔣️json/🔖️rfc8259}/✳️any/🦀️component.rs` (3 files) — stdio_gap fix, import direction.
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/{📊️csv/🔖️rfc4180,📝️md/🔖️commonmark,🔣️json/🔖️rfc8259}/✳️any/🦀️component.rs` (3 files) — stdio_gap fix, export direction.

Nothing created, nothing deleted at the file level.

## Honest pass/fail

- `register()` → `declaration()`: **built, compiles, exhaustively matches note's exemplar pattern.**
- `.setup()` narrowing: **done** — one call (`register_app_schema`); `bootstrap_imperative_runtime()`
  judgment call documented above rather than silently added as a second `.setup()` purpose.
- Step 3 (root closure): **already clean**, nothing to do.
- Step 4 (escape hatches/deps): **nothing found** (measured, not assumed).
- Step 5 (inventory): **done**, 5 static caches + 1 instance-owned RefCell, none a violation.
- Unrelated stdio_gap breakage: **found and fixed** (9 errors, 6 files), using an already-compiling
  sibling plugin's (`🔱️jack`) established pattern, not invented design.
- Verification: **`cargo metadata` OK, 0 missing `#[path]`/`include!` targets, `cargo check
  --all-targets` 0 errors** (pasted above, attempt 5 of 5 after 4 rounds of confirmed-transient
  concurrent stdio churn).
