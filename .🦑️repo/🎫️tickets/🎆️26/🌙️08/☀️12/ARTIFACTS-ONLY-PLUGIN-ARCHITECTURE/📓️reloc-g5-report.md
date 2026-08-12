# G5 Relocation Report — `📐️cad`, `🖍️draw`, `📏️layout`

`declaration()` + private helper `pilot_languages()` moved out of each artifact's `⚙️engine/🦀️component.rs`
into that artifact's root `🗿️artifacts/<a>/🦀️component.rs` (inside the existing `//#region 🔖️ArtifactKind`
region, next to `artifact_kind()`). `pilot_languages()` stayed **private** (not `pub`) in all three —
each has exactly one caller, `declaration()`, which now lives in the same file.

## 📐️cad (`semio-s-plugin-cad`)

One artifact, one `declaration()` site.

- **Moved**: `declaration()` (was `⚙️engine/🦀️component.rs:916-924`) and `pilot_languages()`
  (was `⚙️engine/🦀️component.rs:801-859`, doc comment `797-800`) →
  `🗿️artifacts/📐️cad/🦀️component.rs:399-472` (pilot_languages then declaration, inside
  `//#region 🔖️ArtifactKind`, after `artifact_kind()`).
- **Engine file after**: the `//#region 🔖️Register` / `//#endregion 🔖️Register` markers stay
  (they now wrap only the pre-existing `//#region 🧩️Contributions` block, unchanged) — nothing else in
  the ~1156-line file was touched.
- **Call site**: `✏️s/🔌️plugins/📐️cad/🦀️component.rs:16`
  `crate::artifacts::cad::engine::declaration()` → `crate::artifacts::cad::declaration()`.
- **Move-both**: held cleanly. `declaration()`'s only unqualified local reference was
  `pilot_languages()`; everything else (`schema`, `inferences`, `composers`, `document_codec`) was
  already `crate::artifacts::cad::…`-qualified, including the `io_registry::entries()` call which
  reaches `crate::artifacts::cad::standards::v1::engine::io_registry` — a glob re-export
  (`pub mod engine { pub use super::subsets::any::engine::*; }` in `📦️glue.rs`) that resolves the
  same regardless of where the *caller* lives, so nothing needed re-qualifying. `dsl::…` inside
  `pilot_languages()` resolves via the crate-root `extern crate semio_framework_os_kernel as dsl;`
  extern-prelude alias (visible everywhere in the crate, not a file-local `use`), confirmed also used
  unqualified in the destination file already (`dsl::DslRecord` derives).
- **No deviation.**

## 🖍️draw (`semio-s-plugin-draw`)

One artifact, one `declaration()` site.

- **Moved**: `declaration()` (was `⚙️engine/🦀️component.rs:24-32`) and `pilot_languages()`
  (was `⚙️engine/🦀️component.rs:39-100`, doc comment `39-41`) →
  `🗿️artifacts/🖍️draw/🦀️component.rs:402-475` (inside `//#region 🔖️ArtifactKind`).
  `artifact_schema_registered()` (was lines `34-37`, sandwiched between the two moved functions) was
  **left in the engine file** — it is not part of the move-both pair.
- **Engine file after**: `//#region 🔖️Register` now contains only `artifact_schema_registered()`
  (~2054-line file otherwise untouched).
- **Call site**: `✏️s/🔌️plugins/🖍️draw/🦀️component.rs:15`
  `crate::artifacts::draw::engine::declaration()` → `crate::artifacts::draw::declaration()`.
- **Move-both: DEVIATED (qualified instead).** `declaration()`'s body called `io_registry::entries()`
  **unqualified**, resolving to `pub mod io_registry { … }` defined later in the *same* engine file
  (`⚙️engine/🦀️component.rs:2023`, now shifted after the removal). That module was **not** moved (only
  `declaration()`/`pilot_languages()` are in scope for this pass), so at the new location the call was
  rewritten to the full path `crate::artifacts::draw::standards::v1::engine::io_registry::entries()`
  (verified: `⚙️engine` mounts as `crate::artifacts::draw::standards::v1::engine` per `📦️glue.rs:49-50`,
  `pub mod engine;` directly, no `subsets::any` indirection like cad). This is exactly the
  "references some other unqualified local item" case the dispatch calls out — reported here as the
  deviation, not silently patched over.

## 📏️layout (`semio-s-plugin-layout`)

One artifact, one `declaration()` site.

- **Moved**: `declaration()` (was `⚙️engine/🦀️component.rs:55-63`) and `pilot_languages()`
  (was `⚙️engine/🦀️component.rs:68-126`, doc comment `65-67`) →
  `🗿️artifacts/📏️layout/🦀️component.rs:348-421` (inside `//#region 🔖️ArtifactKind`).
- **Engine file after**: the whole `//#region 🔖️Register` / `//#endregion 🔖️Register` wrapper is gone
  (it contained nothing else) — file drops from 860 to 758 lines; `//#region ⚠️Errors` now runs
  straight into `//#region 🔖️Io`, everything else (`LayoutError`, `layout_io()`, the rest) untouched.
- **Call site**: `✏️s/🔌️plugins/📏️layout/🦀️component.rs:15`
  `crate::artifacts::layout::engine::declaration()` → `crate::artifacts::layout::declaration()`.
- **Move-both**: held cleanly. `declaration()`'s `.composers(…)` call was already fully qualified as
  `crate::artifacts::layout::standards::v1::engine::io_registry::entries()` (unlike draw, this one
  never relied on same-file unqualified resolution) — same glob-reexport situation as cad, doc comment
  even notes the root-level `artifacts::layout::io_registry` is separate inert dead code, left alone.
  `dsl::…` resolves via the same crate-root extern-prelude alias, confirmed already used unqualified
  in the destination file (`dsl::DslRecord` derives).
- **No deviation.**

## Verify — the four greps, all three plugins

```
$ grep -rn "fn declaration" <plugin>          # exists at artifact root, gone from ⚙️engine
📐️cad:    🗿️artifacts/📐️cad/🦀️component.rs:466: pub fn declaration()
🖍️draw:   🗿️artifacts/🖍️draw/🦀️component.rs:473: pub fn declaration()
📏️layout: 🗿️artifacts/📏️layout/🦀️component.rs:420: pub fn declaration()

$ grep -rn "engine::declaration" <plugin>     # zero hits, all three
(no output — 📐️cad, 🖍️draw, 📏️layout)

$ grep -rn "pub fn pilot_languages" <plugin>  # zero hits, all three — nothing was widened
(no output — 📐️cad, 🖍️draw, 📏️layout)

$ every #[path] in each plugin's 📦️glue.rs resolves on disk
(scripted check over all #[path="…"] entries in all three 📦️glue.rs files — 0 missing targets)
```

## `#[path]` integrity

Scripted resolution check over every `#[path = "…"]` entry in all three plugins' `📦️glue.rs` — every
target exists on disk. No file was moved or renamed, only edited, so this was expected to hold; kept
as a real check rather than an assumption.

## cargo check — ONE run per crate, with the mandated override

Override used for all three:
`RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo check -p <crate> --all-targets`

### 📐️cad — `semio-s-plugin-cad`: **GREEN**
```
warning: `semio-s-plugin-cad` (lib) generated 10 warnings (run `cargo fix --lib -p semio-s-plugin-cad` to apply 6 suggestions)
warning: `semio-s-plugin-cad` (lib test) generated 18 warnings (10 duplicates) (run `cargo fix --lib -p semio-s-plugin-cad --tests` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 55s
```
0 `error` lines. Only pre-existing warnings (unused import, dead-code field, unused var, one
unnecessary-qualification lint, a `testkit` glob-ambiguity future-incompat warning from framework/os —
none touch the moved code).

### 🖍️draw — `semio-s-plugin-draw`: **GREEN**
```
warning: `semio-s-plugin-draw` (lib) generated 7 warnings (run `cargo fix --lib -p semio-s-plugin-draw` to apply 4 suggestions)
warning: `semio-s-plugin-draw` (lib test) generated 9 warnings (7 duplicates) (run `cargo fix --lib -p semio-s-plugin-draw --tests` to apply 1 suggestion)
    Finished `dev` profile [unoptimized] target(s) in 5m 59s
```
0 `error` lines.

### 📏️layout — `semio-s-plugin-layout`: **NOT GREEN — pre-existing/concurrent breakage, unrelated to this move**
```
error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc`
 --> …/🗿️artifacts/📏️layout/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:3:5
    | no `PageDoc` in `artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot`

error[E0560]: struct `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot` has no field named `page`
 --> …/🗿️artifacts/📏️layout/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:11:9
    | help: a field with a similar name exists: `pages`

error[E0609]: no field `page` on type `&semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`
 --> …/🗿️artifacts/📏️layout/…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:9:61

error: could not compile `semio-s-plugin-layout` (lib) due to 3 previous errors; 14 warnings emitted
error: could not compile `semio-s-plugin-layout` (lib test) due to 3 previous errors; 19 warnings emitted
```
Classification: **NOT mine.** All 3 distinct errors (counted twice, lib + lib test) are in layout's own
`🚪️io/📤️export/…/pdf/🔖️1.4` and `🚪️io/📥️import/…/pdf/🔖️1.4` PDF serializer files — files this pass never
touched (only `⚙️engine/🦀️component.rs` and the two `🦀️component.rs` root files were edited). The cause
is a mid-flight shape change on `semio-s-plugin-stdio`'s own `pdf` artifact (`PageDoc` relocated out of
`artifacts::pdf::schema::snapshot`, and `PdfSnapshot`'s field renamed `page` → `pages`) landing from
another session while this pass ran — `git status` on `✏️s/🔌️plugins/📏️layout` independently shows
concurrent staged changes from another session (`🎟️capabilities/🦀️component.rs` deleted,
`🔧️setup/🦀️component.rs` deleted, `🛂️manifest/🦀️component.rs` deleted, `🛂️manifest.json` renamed) —
consistent with in-flight plugin-bundle churn, not something this relocation introduced. Per instruction,
not patched, not worked around, not silently accepted as green.

`semio-s-plugin-layout` (lib) itself compiled its non-pdf modules fine before hitting these 3 errors;
this is not the `🗄️stdio`-all-targets carve-out (errors are outside `🗄️stdio`'s own tree) but is the same
"another session's in-flight refactor" situation the standing guidance describes — reporting instead of
guessing or fixing forward.

## apa-status

**complete but UNVERIFIED for 📏️layout** (pre-existing/concurrent pdf-schema breakage in files this pass
never touched, quoted above) — **complete and GREEN for 📐️cad and 🖍️draw**. `🖍️draw` required one
explicitly-reported deviation (qualifying `io_registry::entries()` instead of moving it); `📐️cad` and
`📏️layout` needed no deviation. No `pilot_languages` was widened to `pub` anywhere; `📕️norm`/`🧱️block`
were not touched.
