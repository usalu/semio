# g4 relocation report — trinity, fem

Moved `declaration()` + its private helper `pilot_languages()` out of each artifact's
`⚙️engine/🦀️component.rs` and into the artifact root `🦀️component.rs`, per the revised
(move-both, widen-nothing) recipe. `pilot_languages` stayed private everywhere — no `pub` added.

## 🔱️trinity (crate `semio-s-plugin-trinity`)

Two artifacts, two `declaration()` sites. Both moved cleanly — move-both premise held, no deviation.

### 🔌️jack

- **Before**: `declaration()` at
  `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:102`,
  `pilot_languages()` at same file `:33`.
- **After**: both moved to `🗿️artifacts/🔌️jack/🦀️component.rs`, new `//#region 🔖️Register` block
  inserted between `// #endregion 🔖️Runtime` and `// #region 🧪️Tests` — `pilot_languages()` at
  `:372`, `declaration()` at `:438`.
- Body was already fully qualified except the call to its own `pilot_languages()` — no other local
  reference. Move-both applied with zero edits to the body.
- Call site updated in `🦀️component.rs:15` (plugin root):
  `crate::artifacts::jack::engine::declaration()` → `crate::artifacts::jack::declaration()`.

### ♻️rewrite

- **Before**: `declaration()` at
  `🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:495`,
  `pilot_languages()` at same file `:426`.
- **After**: both moved to `🗿️artifacts/♻️rewrite/🦀️component.rs`, new `//#region 🔖️Register`
  block inserted between `//#endregion 🔖️ArtifactKind` and `//#region 🚪️DerivedIoRegistry` —
  `pilot_languages()` at `:104`, `declaration()` at `:170`.
- Body fully qualified except its own `pilot_languages()` — no deviation.
- Call site updated in `🦀️component.rs:16` (plugin root):
  `crate::artifacts::rewrite::engine::declaration()` → `crate::artifacts::rewrite::declaration()`.

Both `⚙️engine/🦀️component.rs` files remain in place with everything else untouched (jack:
216→133 lines, rewrite: 609→526 lines — only the two functions removed).

## 🏗️fem (crate `semio-s-plugin-fem`)

Two artifacts, two `declaration()` sites. **Both deviated from clean move-both** — see below.

### 🧊️3d (fem3d)

- **Before**: `declaration()` at
  `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:30`,
  `pilot_languages()` at same file `:43`.
- **After**: both moved to `🗿️artifacts/🧊️3d/🦀️component.rs`, new `// #region 🔖️Register`
  block inserted between `// #endregion 🔖️ArtifactKind` and `// #region 🧪️Tests` —
  `declaration()` at `:274`, `pilot_languages()` at `:287`.
- **Deviation**: `declaration()`'s body called `io_registry::entries()` **unqualified** — that
  `io_registry` is a `pub mod` defined further down in the *same* engine file (line ~778, post-move),
  which per the instructions stays behind (only `declaration`/`pilot_languages` travel). Left as an
  unqualified reference it would silently resolve to nothing (no sibling `io_registry` module at the
  artifact root) and fail to compile. **Qualified it** to
  `crate::artifacts::fem3d::standards::v1::engine::io_registry::entries()` — the real module path
  (confirmed against `📦️glue.rs`'s `#[path]` nesting for `artifacts::fem3d::standards::v1::engine`),
  matching the exact pattern jack/rewrite's `declaration()` already used for the same call. This is
  the only body edit made anywhere in this pass.
- Call site updated in `🦀️component.rs:16` (plugin root, `🏗️fem`):
  `crate::artifacts::fem3d::engine::declaration()` → `crate::artifacts::fem3d::declaration()`.

### ◻2d (fem2d)

- **Before**: `declaration()` at
  `🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:19`,
  `pilot_languages()` at same file `:32`.
- **After**: both moved to `🗿️artifacts/◻2d/🦀️component.rs`, new `// #region 🔖️Register`
  block inserted between `// #endregion 🔖️ArtifactKind` and `//#region 🚪️DerivedIoRegistry` —
  `declaration()` at `:259`, `pilot_languages()` at `:272`.
- **Deviation**: identical case to fem3d — `declaration()` called `io_registry::entries()`
  unqualified, referencing the sibling `pub mod io_registry` left behind in the engine file (line
  ~585, post-move). Qualified to
  `crate::artifacts::fem2d::standards::v1::engine::io_registry::entries()`.
- Call site updated in `🦀️component.rs:15` (plugin root, `🏗️fem`):
  `crate::artifacts::fem2d::engine::declaration()` → `crate::artifacts::fem2d::declaration()`.

Both `⚙️engine/🦀️component.rs` files remain in place with everything else untouched (fem3d:
871→789 lines, fem2d: 678→596 lines — only the two functions removed).

## VERIFY — the four greps, run repo-wide over each plugin

**`grep -rn "fn declaration" <plugin>`** — exists once at each artifact root, gone from `⚙️engine`:

```
🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs:438:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs:170:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
🏗️fem/🗿️artifacts/🧊️3d/🦀️component.rs:274:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
🏗️fem/🗿️artifacts/◻2d/🦀️component.rs:259:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
```

**`grep -rn "engine::declaration" <plugin>`** — zero code hits in both plugins. Each plugin's
`📦️glue.rs` still has exactly one doc-comment mentioning the old name (historical prose, not a
call — left alone, not our file to edit for prose):

```
🔱️trinity/📦️packages/🦀️rust/📦️glue.rs:958:/// now lives in each artifact's own `engine::declaration()`, walked by `PluginBuilder::build()` via
🏗️fem/📦️packages/🦀️rust/📦️glue.rs:51:/// document-codec registration now flows through `.artifact(engine::declaration())` in the plugin root
```

**`grep -rn "pub fn pilot_languages" <plugin>`** — zero hits in both plugins (nothing widened;
confirmed both before and after the move — none of the four sites had `pub` and none gained it).

**`#[path]` resolution** — no file was moved, renamed, or deleted; only function bodies moved
between two files that were both already `#[path]`-mounted in each plugin's `📦️glue.rs`. Spot-checked
that both source and destination paths for all four sites still resolve on disk (`ls` against each
literal `#[path]` string in `📦️glue.rs`) — all four the artifact-root `🦀️component.rs` and all four
`⚙️engine/🦀️component.rs` files exist unchanged in location.

## cargo check — ONE run per crate, override in effect

`RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p <crate> --all-targets`

### semio-s-plugin-trinity

Result: **`Finished \`dev\` profile [unoptimized] target(s) in 1m 43s`** — **0 errors**. 51 warnings
on `(lib)`, 52 on `(lib test)` (45 duplicates) — all pre-existing categories (unused imports, dead
fields on `TrinityGraphEngine`/`RewriteRuleEngine`, one ambiguous-glob-imports future-incompat note
from `os_spr`/`os_pack` re-exports — none touch the four moved sites). **Classification: clean, no
upstream-stdio errors, nothing attributable to this pass.** Full output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g4-trinity-cargo.txt`.

### semio-s-plugin-fem

Result: **`Finished \`dev\` profile [unoptimized] target(s) in 5m 55s`** — **0 errors**. (Run took
long real time because the shared `CARGO_TARGET_DIR` was contended by ~15 concurrent sibling
sessions doing this same ticket pass — confirmed via `lsof` on `.cargo-build-lock`/`.cargo-lock` and
`ps aux`, not a bug in this change; it simply serialized behind other builds for a while before
`cargo` acquired the lock and started compiling.) 33 warnings on `(lib)`, 50 on `(lib test)` (31
duplicates) — unused imports/functions, one future-incompat glob-import ambiguity note carried from
upstream `semio-framework-os-kernel`'s `os_spr`/`os_pack` re-exports (same note trinity's build also
surfaced) — none touch the four moved sites or the two qualified `io_registry::entries()` calls.
The build chain passed cleanly through `semio-s-plugin-stdio` (checked immediately before `fem` in
the same run, per the dependency graph) with **zero errors from stdio** on this run, so the
"UNVERIFIED, upstream stdio errors" caveat does not apply here — `--all-targets` compiled clean
end-to-end. **Classification: clean, no upstream-stdio errors, nothing attributable to this pass.**
Full output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g4-fem-cargo.txt`.

## apa-status

Both plugins in this dispatch (`🔱️trinity`, `🏗️fem`) are relocated: 4/4 `declaration()` +
`pilot_languages()` pairs moved from `⚙️engine` to their artifact roots, both plugin-root call sites
repointed, zero `pub fn pilot_languages` anywhere (nothing widened), zero live `engine::declaration()`
call sites left (only two historical doc-comment mentions, untouched), and both crates `cargo check
--all-targets` clean with the required override. `📕️norm` and `🧱️block` were not touched, as
instructed — that's another session's 17 sites.
