# reloc-g7 — declaration()/pilot_languages() relocation report

Plugins: 🎪️demonstrator, 📖️playbook, 📜️imperative, 🗒️note.

## 🎪️demonstrator (crate `semio-s-plugin-demonstrator`)

**Pre-state found**: NOT a fresh site — the earlier (stopped/replaced) dispatch of this pass had
already run here. `declaration()` was already at the artifact root
(`🗿️artifacts/🎪️playground/🦀️component.rs:42`), but its helper `pilot_languages()` had been left behind
in `⚙️engine` and made **`pub`** (the exact mistake this ticket's revision calls out), with
`declaration()` reaching it via `crate::artifacts::playground::standards::v1::engine::pilot_languages()`.

**Action — revert + complete the move**:
- Moved `pilot_languages()` **and its five single-caller helpers** (`build_pilot_languages`,
  `playground_document_language`, `playground_op_language`, `playground_diff_language`,
  `playground_pack_language`, `playground_spr_language` — verified via grep, no other callers anywhere
  in the plugin) from
  `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:12-91` (old)
  to `🗿️artifacts/🎪️playground/🦀️component.rs` (new, appended after `declaration()`), **as `fn`, not
  `pub fn`**.
- Updated `declaration()`'s call from
  `crate::artifacts::playground::standards::v1::engine::pilot_languages()` to bare `pilot_languages()`.
- `.composers(...)` call was already correctly qualified
  (`crate::artifacts::playground::standards::v1::engine::io_registry::entries()`) and untouched.
- Plugin call site (`✏️s/🔌️plugins/🎪️demonstrator/🦀️component.rs:18`) was already
  `crate::artifacts::playground::declaration()` — no change needed there.

Move-both held cleanly once the `pub` was undone. **Deviation**: none in the "handle by qualifying"
sense — this was a straight revert of the earlier dispatch's mistake.

## 📖️playbook (crate `semio-s-plugin-playbook`)

One artifact, one `declaration()`. Clean move-both case:
- **Before**: `🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:24-95`
  (`declaration()` + private `pilot_languages()`, single caller, fully self-contained body).
- **After**: both moved to `🗿️artifacts/📖️playbook/🦀️component.rs` (inserted before `🔖️ArtifactKind`
  region), `pilot_languages()` kept private.
- `.composers(io_registry::entries())` was an unqualified local reference (the `io_registry` module
  stays in `⚙️engine`, unmoved) — qualified to
  `crate::artifacts::playbook::standards::v1::engine::io_registry::entries()` on the move (this exact
  qualified path was already in use elsewhere in the same root file, line 92, confirming it's correct).
- Call site `✏️s/🔌️plugins/📖️playbook/🦀️component.rs:15`:
  `crate::artifacts::playbook::engine::declaration()` → `crate::artifacts::playbook::declaration()`.
- Stale doc-comment cross-reference fixed:
  `🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs:20` (`...moved to
  crate::artifacts::playbook::engine::declaration()` → `...declaration()`).

**Deviation reported**: qualifying `io_registry::entries()` (already `pub fn` inside `pub mod
io_registry`, so no widening — just added its full path).

## 📜️imperative (crate `semio-s-plugin-imperative`)

One artifact, one `declaration()`. **Real deviation site** — `declaration()`'s body referenced two
unqualified local items besides `pilot_languages()`:
- `bootstrap_imperative_runtime()` — private, and NOT single-caller: also called from
  `ImperativeHost::from_snapshot` at old line 194, both inside `⚙️engine`. Move-both cannot apply to it.
- `io_registry::entries()` — same shape as playbook (module stays behind).

**Before**: `🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:51-123`
(`declaration()` + private `pilot_languages()`).

**After**:
- `declaration()` + `pilot_languages()` (private) moved to `🗿️artifacts/📜️imperative/🦀️component.rs`.
- `bootstrap_imperative_runtime()` **stayed in `⚙️engine`** (its second caller, `ImperativeHost::
  from_snapshot`, is also there) and was widened from private to **`pub(crate)`** — not `pub` — solely
  so the moved `declaration()` can call it by full path
  (`crate::artifacts::imperative::standards::v1::engine::bootstrap_imperative_runtime()`).
  `pub(crate)` is crate-internal only; it does not add to the public API surface the ticket's 45-function
  measurement was protecting, so it is not a repeat of the `pilot_languages`-pub mistake.
- `io_registry::entries()` qualified the same way as playbook:
  `crate::artifacts::imperative::standards::v1::engine::io_registry::entries()`.
- Call site `✏️s/🔌️plugins/📜️imperative/🦀️component.rs:15`:
  `crate::artifacts::imperative::engine::declaration()` → `crate::artifacts::imperative::declaration()`.

**Deviation reported**: `bootstrap_imperative_runtime` widened to `pub(crate)` (not moved, not made
`pub`) because it has a second caller in `⚙️engine`; `io_registry::entries()` qualified in place
(already `pub`, no widening).

## 🗒️note (crate `semio-s-plugin-note`)

One artifact, one `declaration()`. Clean move-both case, same shape as playbook but the composers call
was **already** fully qualified in the engine body:
- **Before**: `🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:21-92`.
- **After**: both moved to `🗿️artifacts/🗒️note/🦀️component.rs` (inserted before `🔖️ArtifactKind`
  region), `pilot_languages()` kept private, `.composers(crate::artifacts::note::standards::v1::engine::
  io_registry::entries())` carried over verbatim (no requalification needed).
- Call site `✏️s/🔌️plugins/🗒️note/🦀️component.rs:16`:
  `crate::artifacts::note::engine::declaration()` → `crate::artifacts::note::declaration()`.

**Deviation reported**: none — pure move-both.

Note: mid-verification, another live session's concurrent W1c work (`.setup()` deletion,
`register_app_schema` → `app_schema_descriptor` rename) touched `🗒️note/🦀️component.rs` and
`📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs` on top of my edits. Both files still
carry my `declaration()` call-site fix intact after that; not reverted, not touched further here — out
of this ticket's scope (W1c is a separate workstream per the shared task list).

## Verify — the four mandated greps (run across each plugin dir)

```
grep -rn "fn declaration" <plugin>       → exactly one hit each, at the artifact root 🦀️component.rs
grep -rn "engine::declaration" <plugin>  → zero hits, all four plugins
grep -rn "pub fn pilot_languages" <plugin> → zero hits, all four plugins
```
All confirmed zero/expected across 🎪️demonstrator, 📖️playbook, 📜️imperative, 🗒️note.

`#[path]` resolution: every `#[path = "..."]` in each plugin's `📦️glue.rs` was resolved against disk
with a small script (97/144/125/271 attributes respectively) — 0 missing in all four. Expected: no file
was created, deleted, moved, or renamed by this pass, only file *contents* edited.

## cargo check — one run per crate, with the mandated override

Override used: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p <crate> --all-targets`.

### 📖️playbook — GREEN
```
warning: `semio-s-plugin-playbook` (lib) generated 10 warnings (7 duplicates)
warning: `semio-s-plugin-playbook` (lib test) generated 15 warnings (3 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 5m 37s
```
Exit 0. Zero errors. Full log: `scratch-g7-playbook-check.txt`.

### 🗒️note — GREEN
```
warning: `semio-s-plugin-note` (lib) generated 8 warnings
warning: `semio-s-plugin-note` (lib test) generated 11 warnings (6 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 4.99s
```
Exit 0. Zero errors. Full log: `scratch-g7-note-check.txt`.

### 📜️imperative — complete but UNVERIFIED, one upstream (non-stdio) error, not mine
```
error[E0425]: cannot find value `register_app_schema` in module `crate::apps::imperative::config::schema`
  --> ✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/../../🦀️component.rs:14:57
14 |         .setup(crate::apps::imperative::config::schema::register_app_schema)
   |                                                         ^^^^^^^^^^^^^^^^^^^ not found
error: could not compile `semio-s-plugin-imperative` (lib) due to 1 previous error
```
Exactly one error, in `.setup(...)` on the plugin root `🦀️component.rs` — a line this pass never
touched (my only edit there was `.artifact(...)`). Root cause: `✏️s/🔌️plugins/📜️imperative/🎛️apps/
📜️imperative/🎚️config/🧬️schema/🦀️component.rs` already only defines `app_schema_descriptor()`, not
`register_app_schema` — the same live W1c cross-plugin rename observed touching note/playbook above,
mid-flight on imperative too but with its plugin-root call site not yet updated by that other session.
Confirmed zero errors reference `declaration`, `pilot_languages`, or `bootstrap_imperative_runtime` —
nothing this pass changed is implicated. Full log: `scratch-g7-imperative-check.txt`.

### 🎪️demonstrator — complete but UNVERIFIED, upstream peer-crate errors (not stdio, not mine)
`semio-s-plugin-demonstrator` depends on path-crates `procedural`, `puzzle`, `process`, `gis` (its own
`Cargo.toml`, confirmed). `cargo check -p semio-s-plugin-demonstrator --all-targets` compiles
dependencies first and never reached the demonstrator crate itself:
```
error: could not compile `semio-s-plugin-process` (lib) due to 3 previous errors
error: could not compile `semio-s-plugin-gis` (lib) due to 3 previous errors
error: could not compile `semio-s-plugin-procedural` (lib) due to 38 previous errors
```
44 `error[...]` lines total, all inside `🏭️process`, `🌍️gis`, `🌀️procedural`, `🧩️puzzle` source paths
(mismatched enum variants, unresolved imports, duplicate-definition clashes — unrelated domain code, own
in-flight refactors per other sessions). `semio-s-plugin-stdio` itself (also a dependency here) compiled
clean — no errors under its paths, consistent with the ticket's stdio-lib-compiles note. Zero errors
anywhere in the log reference `🎪️playground` or `🎪️demonstrator` paths — nothing this pass touched is
implicated, but the crate itself was never reached, so its own compile is genuinely unverified, not just
"probably fine." Full log: `scratch-g7-demonstrator-check.txt`.

## apa-status

Site count: 4 plugins, 4 `declaration()` sites total (one each — confirmed by
`grep -rln "fn declaration"` before starting; none had a second artifact). All four relocated to their
artifact root. `pilot_languages()` private in all four post-state (zero `pub fn pilot_languages` hits).
One pre-existing `pub` mistake found and reverted (🎪️demonstrator, from the earlier stopped dispatch).
One real deviation (📜️imperative's `bootstrap_imperative_runtime`, `pub(crate)`, second caller, not
widened to `pub`). Two of four crates compile clean (📖️playbook, 🗒️note). Two of four are
complete-but-unverified for reasons entirely outside this pass's own edits: 📜️imperative blocked by a
concurrent W1c rename on a line this pass didn't touch; 🎪️demonstrator blocked by unrelated peer-crate
(process/gis/procedural) compile failures upstream of ever reaching the demonstrator crate. `🧬️mutations/**`
untouched, no artifact-kind ids renamed, no `Cargo.toml`-bearing directory moved. `📕️norm`/`🧱️block` not
touched.
