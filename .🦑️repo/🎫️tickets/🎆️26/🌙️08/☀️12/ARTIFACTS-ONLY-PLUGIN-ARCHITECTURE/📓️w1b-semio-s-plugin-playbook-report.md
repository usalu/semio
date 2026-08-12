# W1b — `semio-s-plugin-playbook` → `.artifact(declaration())` conversion

`apa-status: released` — `register()` → `declaration()` conversion done, plugin root wired to `.artifact(…)`, `.setup()` narrowed to the one app-scope call `ArtifactDeclaration` deliberately has no field for, plugin root already closed (nothing to delete), 3 unrelated pre-existing compile bugs found and fixed to reach a clean `cargo check --all-targets`. **0 errors, real pasted output below.**

## Step 0 — clearance

Read `📓️plugin-release-status.md` (SMO ledger). `📖️playbook` appears under "RELEASED — lane
finished, compiles in the workspace check" (facet `📖️playbook`, "vocabulary MOVED from the
framework kernel into the plugin; ~470 framework lines deleted") — not HELD, not another
session's. Proceeded.

## Step 1+2 — `register()` → `declaration()`, plugin root wired

**`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
(region `🔖️Register`, was :16-85, now :16-97):
- Old `pub fn register()` (5 calls: `io_registry::register()`, `register_artifact_schema()`,
  `register_artifact_inference()`, `crate::apps::playbook::config::schema::register_app_schema()`,
  `register_pilot_languages()`, `register_document_codec_for_app::<PlaybookPlayApp>(…)`) replaced by:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.playbook")
          .schema(crate::artifacts::playbook::schema::playbook_artifact_schema_descriptor())
          .inferences([crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::playbook_artifact_inference_descriptor()])
          .composers(io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::playbook::PlaybookPlayApp>()
          .build()
  }
  ```
  `kind = "s.playbook"` matches `PLAYBOOK_DIALECT.artifact_kind` used by every composer entry in
  this same file's `io_registry` module (verified by reading it, not assumed) — the ownership
  check's loose layer (`writes == kind || kind ∈ reads`) passes for all 6 entries. `"s.playbook"`
  is 2 segments, so `ArtifactKindId::parse` does not match canonical `s.<plugin>.<artifact>` grammar
  (same situation note's `"s.note"` was in) — only the loose ownership layer is active today, per
  the W1 mechanism's own documented self-tightening design.
- Old `pub fn register_pilot_languages()` (side-effecting, called `dsl::register_language` 5×) →
  private `fn pilot_languages() -> &'static [dsl::LanguageSpec]`, `OnceLock`-backed, byte-identical
  5 `LanguageSpec` values, mirroring note's exact pattern (its own `dsl::passthrough_hooks` isn't
  `const fn`, so this can't be a `const` array either).
- Region `🔖️SchemaRegistry` (old :198-210, `register_artifact_schema()`/`register_artifact_inference()`)
  deleted outright — each had exactly one call site (inside the old `register()`), confirmed by
  grep before deleting; replaced by `.schema(...)`/`.inferences([...])` data on `declaration()`.

**`✏️s/🔌️plugins/📖️playbook/🦀️component.rs`** — `plugin()`:
```rust
.setup(crate::apps::playbook::config::schema::register_app_schema)
.artifact(crate::artifacts::playbook::engine::declaration())
```
replacing the old single `.setup(crate::apps::playbook::setup)` (which itself just re-exported the
engine's old `register()`).

**`✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🦀️component.rs`** — deleted the now-dead
`🔖️Setup` region (`pub use crate::artifacts::playbook::engine::register as setup;`); confirmed
zero other call sites of `crate::apps::playbook::setup` before deleting.

**`✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs:14-19`** —
updated `register_app_schema()`'s doc comment (was: "Called from
`artifacts::playbook::engine::register()`"; now points at the plugin root's `.setup()`).

## `.setup()` survives — for exactly one call, and why

`crate::apps::playbook::config::schema::register_app_schema` — registers `PlaybookConfig`'s
app-scope config/presence schema (`s.playbook.playbook` config facet) into the framework's
`AppSchemaRegistry`. `ArtifactDeclaration` has no field for `register_app_schema_descriptor` by
design (it's app-scope, not artifact-scope — see the W1 mechanism report's exhaustive §6 mapping
table). This is the **same one exception note's own exemplar keeps** — no other `.setup()` call
survives, and no other reason for `.setup()` to remain was found (grepped for any other
`register_*`/side-effecting free function reachable from the plugin root — none).

## Step 3 — plugin root already closed

`find ✏️s/🔌️plugins/📖️playbook -maxdepth 1` before touching anything: `AGENTS.md`, `🎛️apps`,
`🗿️artifacts`, `📦️packages`, `🦀️component.rs`, `🧩️extensions`. No `🛂️manifest/`, `🎟️capabilities/`,
`🔧️setup/` dirs, no stray `#[path]` mounts at root — nothing to delete. `🧩️extensions/🌀️procedural`
has its own `Cargo.toml` (`find … -name Cargo.toml` confirms) — crate-bearing, inventory-only per
the ticket's plugin-specific note (this vocabulary was moved out of the framework kernel very
recently; newly-arrived-looking is correct, not debt).

## Step 4 — escape hatches / deps

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` →
0 hits. `grep -rln "semio_framework_os::"` → 0 hits. Cargo.toml has no `semio-framework-os` dep at
all (only `semio-framework-os-kernel` and its own `-flow` re-export, different packages) — nothing
to purge.

## Step 5 — inventory

- `thread_local!` — 0 hits.
- `static`/interior-mutable state: 3 `OnceLock<Vec<…>>` — `PlaybookEngine`'s two composer-entry
  caches (`🗿️artifacts/📖️playbook/🦀️component.rs:94`, engine `io_registry::ENTRIES` at
  `⚙️engine/🦀️component.rs:220`) and the new `pilot_languages()` `LANGUAGES` cache
  (`⚙️engine/🦀️component.rs:38`). All three are lazily-built **derived data caches** over static
  inputs (no host/engine handle type held by any of them) — not a violation class, per the
  ticket's own distinction.
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]` — 0 hits.

## Step 6 — verification

1. **`#[path]` resolution** (`📦️glue.rs`): 70 `#[path]` mounts (excluding `#[path = "."]` grouping
   markers), scripted resolution check against the real files on disk — **0 missing**.
2. **`include_str!`/`include_bytes!` resolution**: every target in every `.rs` file under the
   plugin, resolved relative to its own file's directory against the real file — **0 missing**.
3. `cargo metadata --no-deps --format-version 1 >/dev/null && echo OK` → **`OK`**.
4. `RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-playbook --all-targets`:
   - 1st attempt: **5 errors**, all inside `semio-s-plugin-playbook` itself, none in
     `semio-s-plugin-stdio` — so not the documented stdio churn. Root-caused and fixed (see next
     section) rather than retried blind, since they were inside my own plugin, not upstream.
   - 2nd attempt (after those 3 fixes): playbook itself clean; failure moved to
     `semio-s-plugin-stdio` (5 errors, `SemioDrawingMutation::{GroupNodes,UngroupNode,FlattenNode,
     UnflattenNode}` kebab-mismatch panics + 1 non-exhaustive match) — **0 mentions of any
     `🔌️plugins/📖️playbook` path** in the output (grep-verified), matching the exact
     "stdio red, converging" pattern the W1 mechanism report documented for note (UCAS's live
     `#2548` rename in flight).
   - 3rd attempt: stdio errors **5 → 4** (the non-exhaustive-match one resolved itself — UCAS
     converging, not me).
   - 4th attempt: **0 errors.**
   ```
   warning: `semio-s-plugin-playbook` (lib) generated 10 warnings (run `cargo fix --lib -p semio-s-plugin-playbook` to apply 7 suggestions)
   warning: `semio-s-plugin-playbook` (lib test) generated 15 warnings (10 duplicates) (run `cargo fix --lib -p semio-s-plugin-playbook --tests` to apply 4 suggestions)
       Finished `dev` profile [unoptimized] target(s) in 2m 48s
   ```
   **0 errors.** Full raw output preserved at
   `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-playbook-check-4.txt`
   (intermediate attempts at `…-check.txt`, `…-check-2.txt`, `…-check-3.txt` in the same folder).

## The 3 unrelated pre-existing bugs found and fixed (all inside my own plugin, none in stdio)

All three predate this session (`git diff` against `HEAD` for each file was empty *before* my
edits — i.e. already committed, not concurrent churn from today; the mode/window files' mtimes
matched today's earlier "vocabulary moved out of the framework kernel" refactor the ticket's own
plugin-specific note flagged as pre-existing-and-correct, not debt of mine).

1. **`✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs`** (`modes::builder` region, was
   ~:414-424) — `pub mod builder { pub use component::*; pub mod windows {} }` had **no
   `#[path]` mount for `component`** and an **empty `windows` module** (no mount for the
   `🪟️windows/🏗️builder/🦀️component.rs` file that exists on disk) — an incomplete taxonomy mount,
   not something my `.artifact()` conversion touched or caused. Confirmed the correct shape against
   `✒️writer/📦️packages/🦀️rust/📦️glue.rs:373-387`'s identical `modes::<mode>::windows::<window>`
   pattern and fixed by mounting both:
   ```rust
   pub mod builder {
       #[path = "../../🎛️apps/📖️playbook/🎭️modes/🏗️builder/🦀️component.rs"]
       mod component;
       pub use component::*;
       pub mod windows {
           pub mod builder {
               #[path = "../../🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs"]
               mod component;
               pub use component::*;
           }
       }
   }
   ```
2. **`.../🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`** (playbook→json) and
3. **`.../🚪️io/📥️import/🧩️deserializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`** (playbook←json) —
   both called `serde_json::to_value`/`from_value` directly against `JsonSnapshot.value`, which a
   concurrent stdio wave retyped from `serde_json::Value` to stdio's own lexeme-preserving
   `JsonValue` enum (`#[serde(tag = "kind")]`) — the exact `stdio_gap`/foreign-lag defect class
   `🗒️note`'s own json export/import leaves already carry a fix for. **Copied note's fix verbatim**
   (structural `serde_json::Value ⇄ JsonValue` converters, `write_json_pretty`/`parse_json_text`
   for the byte-level entry points instead of raw `to_vec_pretty`/`from_str` — note's own comment
   flags the old `to_vec_pretty` path as a latent double-bug, serializing the internally-tagged enum
   verbatim instead of real JSON text; the same latent bug existed here and is now fixed too).

No other files were touched beyond what's listed above and in "Step 1+2".

## Files touched

- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`+`register_pilot_languages()`+`register_artifact_schema()`+`register_artifact_inference()` → `declaration()` + `pilot_languages()`.
- `✏️s/🔌️plugins/📖️playbook/🦀️component.rs` — `.setup(apps::playbook::setup)` → `.setup(apps::playbook::config::schema::register_app_schema)` + `.artifact(artifacts::playbook::engine::declaration())`.
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🦀️component.rs` — deleted dead `🔖️Setup` re-export region.
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs` — doc comment updated to the new call site.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs` — fixed pre-existing broken `modes::builder`/`windows::builder` mount (unrelated bug, found while verifying).
- `.../🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — fixed pre-existing `stdio_gap` (JsonValue retype), mirroring note's fix.
- `.../🚪️io/📥️import/🧩️deserializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — same, import side.

Nothing created, nothing deleted at the file level (one dead re-export region removed from within
an existing file; one dead free-function region removed from within an existing file).

## sharedFileRequests

None. Every file touched is inside `✏️s/🔌️plugins/📖️playbook/`. `semio-s-plugin-stdio`'s own
`SemioDrawingMutation` kebab-mismatch defects were observed (5→4 errors across two retries) but
never touched — UCAS's own `#2548` rename, in flight, not mine.

## apa-status

**released** — `.artifact(declaration())` wired, `.setup()` narrowed to the one justified app-scope
call, plugin root already minimal (nothing to close), no escape hatches found, inventory clean,
`#[path]`/`include_str!` resolution both 0-missing, `cargo metadata` OK, `cargo check -p
semio-s-plugin-playbook --all-targets` **0 errors** (real pasted output above).
