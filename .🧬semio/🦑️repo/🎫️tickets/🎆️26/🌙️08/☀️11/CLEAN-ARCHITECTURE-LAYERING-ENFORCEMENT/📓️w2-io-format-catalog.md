# W2 — Generic Format-Metadata Registry on `🚪️io` + Stdio Scaffold

Scope: wave 2, additive-only. Adds the generic `FormatDescriptor` registry to `🚪️io` as the
eventual replacement for `🔺️mesh`'s hardcoded `STDIO_FORMAT_CATALOG` (mesh itself is untouched —
eviction is a later wave), and proves the shape end-to-end with a small illustrative subset wired
into the `🗄️stdio` plugin's manifest facet.

## Files touched (only files in this agent's ownership)

1. `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`
   - New region `//#region 🔖️FormatCatalog`, added directly before the file's closing
     `//#endregion 🔖️ErasedRegistry` / `// #endregion io`, after `//#region 🔖️Wire`.
   - Follows the exact same idioms as the rest of the file (`IO_REGISTRY`'s
     `OnceLock<RwLock<HashMap<...>>>` pattern, `register_composer_entries`'s additive/callable-many-times
     convention, `register_subset_validator`'s "log on overwrite, never panic" boot-ordering policy).

2. `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs` (was an empty stub)
   - New region `//#region 🔖️FormatCatalog` with the illustrative subset + registration helper.

## Full API added to `io` (all in the new `//#region 🔖️FormatCatalog`)

```rust
pub struct FormatDescriptor {
    pub kind_id: String,
    pub short_id: String,
    pub aliases: Vec<String>,
    pub mime: String,
    pub extension: String,
    pub name: String,
    pub full_name: String,
    pub neutral: bool,
    pub dir_name: String,
    pub is_binary: bool,
}

pub fn register_format_descriptors(rows: Vec<FormatDescriptor>);
pub fn format_descriptor(kind_or_short_or_alias: &str) -> Option<FormatDescriptor>;
pub fn normalize_format_kind(input: &str) -> Option<String>;
pub fn format_accept_filter(kind_ids: &[&str]) -> String;
pub fn formats_csv() -> String;
```

Backing store: `static FORMAT_CATALOG: OnceLock<RwLock<HashMap<String, FormatDescriptor>>>`
(module-private `format_catalog()` accessor, mirrors `io_registry()`/`subset_validator_registry()`).

- `register_format_descriptors` is additive and callable multiple times (once per plugin that owns
  formats — does NOT assume a single caller, per the task's explicit requirement). Each row is
  indexed under three kinds of keys: its `kind_id`, its `short_id`, and every string in `aliases` —
  so `format_descriptor` resolves any of the three forms in O(1) without a linear scan. A key
  collision (two different rows claiming the same kind_id/short_id/alias string) overwrites and
  logs `[DEBUG] io::register_format_descriptors overwrote an existing entry for key ...` rather
  than panicking, matching `register_subset_validator`'s existing "boot ordering across concurrent
  plugin loads shouldn't crash the process" policy.
- `format_descriptor` / `normalize_format_kind` / `format_accept_filter` are the generic successors
  to `mesh::stdio_format_entry` / `mesh::stdio_format_kind_id` / `mesh::stdio_accept_filter`.
- `formats_csv` is the generic successor to `mesh::stdio_mimes_csv` — same header shape
  (`MIME,Extension,Name,FullName,Neutral,Dir,Kind`), sourced from the registry instead of the
  hardcoded const slice, deduplicated by `kind_id` and sorted for determinism. Note: `neutral` here
  is `bool` (per the task's struct spec) not mesh's `&'static str` type-name field — serialized as
  `"true"`/`"false"` in the CSV body. This is a deliberate semantic difference from mesh's `neutral`
  field, not an oversight — flagging for whoever does the wave-3 mesh eviction to reconcile.

## Stdio scaffold (`🛂️manifest/🦀️component.rs`)

```rust
pub fn stdio_format_descriptors() -> Vec<FormatDescriptor>   // json / png / obj, 3 of the eventual 28
pub fn register_stdio_format_descriptors()                    // calls register_format_descriptors(stdio_format_descriptors())
```

The 3 illustrative rows (json, png, obj) were copied verbatim (mime/extension/dir_name/name/full_name/
is_binary) from `mesh::STDIO_FORMAT_CATALOG`'s own rows in
`🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` (lines ~1060/1064/1073) — cross-checked against the
real artifact directories under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🔣️json,📷️png,🧊️obj}`, so this
is accurate data, not placeholder guesses. `aliases: vec![]` for all three (mesh's
`normalize_stdio_format_kind` only defines aliases for jpeg/tif/stp/markdown — none of which are
json/png/obj). `neutral: true` for all three (judgment call: these are common non-proprietary
interchange formats; no existing "neutral: bool" precedent to follow since this field's semantics
are new in `FormatDescriptor`).

`// TODO(wave-3): full 28-entry roster migrates from 🔺️mesh's STDIO_FORMAT_CATALOG here.` left
directly above `stdio_format_descriptors`.

### Call-site wiring — NOT done, and why

`register_stdio_format_descriptors()` exists but is **not called** from anywhere yet. The task
asked me to wire it in from wherever `🗄️stdio`'s `🔧️setup`/init registration already happens
(mirroring the `crate::artifacts::<x>::engine::register()` call-site style already used in
`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`'s `plugin()` fn). I traced that call site precisely — it's
`plugin()` in `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, which already calls one `engine::register()`
per artifact before building the `Plugin` — but **that file is not in this agent's file ownership**
(only `🚪️io/🦀️component.rs` and `🛂️manifest/🦀️component.rs` are). Per the hard ownership rule
("only edit files explicitly listed... do not touch any other file, even if related"), I left the
actual call out rather than editing a file another agent owns.

**Action needed from a follow-up wave/agent that owns `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`:**
add one line inside `plugin()`, alongside the existing `engine::register()` calls:
```rust
crate::manifest::register_stdio_format_descriptors();
```
(Also note: `🛂️manifest` is not currently mounted as a module anywhere in
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — no `pub mod manifest`/`#[path=...]` line
exists there yet either. Confirmed by grep. That mount is a second prerequisite the same follow-up
agent needs to add, in a file also outside this agent's ownership.)

## Import path used in the stdio scaffold

```rust
use semio_framework_plugin::io::{register_format_descriptors, FormatDescriptor};
```

This resolves without touching any file outside ownership because
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (not owned, not edited) already
has a pre-existing top-level `pub use semio_framework::*;` glob re-export (verified at line 9267),
which glob-imports the `pub mod io;` module declared in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
(also not owned, not edited) into `semio_framework_plugin`'s own namespace as
`semio_framework_plugin::io`. New `pub` items added inside `🚪️io/🦀️component.rs` (the file I do
own) become reachable through that existing chain automatically — no re-export list anywhere needed
updating.

## Verification performed

1. `cargo check -p semio-framework` — clean, only pre-existing unrelated warnings (ambiguous glob
   re-exports in `os` glue, an unused `len` var in the dsl lexer, etc. — none touch my new code).
   Full output not saved (ran interactively); rerun on demand with same command.

2. `cargo check -p semio-s-plugin-stdio` — clean, 238 pre-existing warnings (dead `artifact_state`/
   `snapshot_state` fields across many engines, etc.), zero errors. **Caveat**: `🛂️manifest` is not
   currently mounted into `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (confirmed via grep —
   no `manifest` mention anywhere in that file), so this check does NOT actually compile my new
   `🛂️manifest/🦀️component.rs` file as part of the stdio crate. This command alone would have given
   a false pass.

3. Because of (2)'s gap, I could not honestly claim the manifest scaffold compiles from
   `cargo check -p semio-s-plugin-stdio` alone. Built a standalone, throwaway verification crate
   (kept in this ticket folder, `target/` build cache deleted afterward to avoid bloating the repo)
   at `verify-manifest-stub/` — `Cargo.toml` with its own empty `[workspace]` table (so it does NOT
   join the root workspace) depending on `semio-framework-plugin` by path, and `src/lib.rs` mounting
   `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs` verbatim via `#[path = ...]`. Ran `cargo check`
   inside that directory: `Checking verify-format-catalog v0.0.0 ... Finished` — 0 errors, only the
   same pre-existing `semio-framework-plugin` warnings as above (none referencing my file). This
   confirms `stdio_format_descriptors()` / `register_stdio_format_descriptors()` and the
   `semio_framework_plugin::io::{FormatDescriptor, register_format_descriptors}` import genuinely
   type-check against the real dependency graph, independent of whether the file is mounted into the
   real crate yet.

Net: both new API surfaces (the `io` registry itself, and the stdio scaffold consuming it)
independently verified to compile. The one remaining gap (mounting `manifest` into stdio's
`glue.rs` + calling `register_stdio_format_descriptors()` from `plugin()`) is explicitly out of
this agent's file-ownership scope and logged above for the next wave.
