# Reloc G1 — declaration()/pilot_languages() Relocation Report

Scope: `➗️mathematical`, `🌀️procedural` (2 artifacts), `🎥️shooting`, `📋️forms`. `📕️norm`/`🧱️block` excluded (owned by another session).

Transform per site: move `pub fn declaration()` and its private helper `pilot_languages()` (kept **private**, no `pub`) from `🗿️artifacts/<a>/…/⚙️engine/🦀️component.rs` to the artifact root `🗿️artifacts/<a>/🦀️component.rs`, in a new `//#region 🔖️Declaration`. Update the plugin's `.artifact(...)` call site. Everything else in the engine files left untouched.

---

## ➗️mathematical (crate `semio-s-plugin-mathematical`)

**1 artifact, 1 `declaration()`.**

**PRE-EXISTING DEVIATION (found, not introduced by me):** `declaration()` was **already** at the artifact root before this pass (`➗️mathematical/🦀️component.rs`, docstring cites this same ticket) — evidence of an earlier, now-superseded dispatch. But `pilot_languages()` had been left behind in `⚙️engine/🦀️component.rs` **as `pub fn`**, called via the full qualified path `crate::artifacts::mathematical::standards::v1::engine::pilot_languages()`. This is exactly the mistake the current instructions call out ("a previous dispatch...told agents to make `pilot_languages` `pub`"). Fixed: moved `pilot_languages()` next to `declaration()` at the root and reverted it to private.

- `pilot_languages()`: `🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:20` (`pub fn`) → `➗️mathematical/🦀️component.rs:145` (`fn`, private)
- `declaration()`: already at `➗️mathematical/🦀️component.rs` (was line 148) → now line 212 (region added around it); body's `.languages(...)` call updated from the fully-qualified `crate::artifacts::mathematical::standards::v1::engine::pilot_languages()` to the local `pilot_languages()`.
- `.composers(...)` was already `crate::artifacts::mathematical::standards::v1::engine::io_registry::entries()` (fully qualified) — untouched, correct as-is.
- Engine file: `pilot_languages()` definition block (was lines 15–79, region `//#region 🔖️Register` … `//#endregion 🔖️Register`) deleted entirely; region tags removed since nothing else occupied that region.

Move-both held for `declaration()` (already done); the **pilot_languages half** required a revert-to-private rather than a fresh move — reported as the deviation above.

## 🌀️procedural (crate `semio-s-plugin-procedural`)

**2 artifacts, 2 `declaration()` sites.**

### 🌀️procedural2d
- `pilot_languages()`: `…/procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:272` (`fn`, already private) → `🌀️procedural2d/🦀️component.rs:58` (`fn`, private)
- `declaration()`: `…/⚙️engine/🦀️component.rs:233` → `🌀️procedural2d/🦀️component.rs:123`

**DEVIATION:** `declaration()`'s body called `.composers(io_registry::entries())` **unqualified**, referring to a `pub mod io_registry` defined later in the *same engine file*. The artifact root ALSO has its own `pub mod io_registry` (a wrapper that re-exports the engine's entries as `&'static [&'static ComposerEntry]` — a different, incompatible return type than the engine's own `&'static [ComposerEntry]`). Left bare after the move, the call would have silently rebound to the root's wrapper and failed type-check. Fixed by qualifying to `crate::artifacts::procedural2d::standards::v1::engine::io_registry::entries()` (verified via `📦️glue.rs`: `pub mod v1 { pub mod engine; }`). Documented inline in the moved doc-comment.

### 🧊️procedural3d
- `pilot_languages()`: `…/procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:874` (`fn`, already private) → `🧊️procedural3d/🦀️component.rs:58` (`fn`, private)
- `declaration()`: `…/⚙️engine/🦀️component.rs:645` → `🧊️procedural3d/🦀️component.rs:122`

Same `io_registry::entries()` unqualified-local-reference deviation as procedural2d; fixed identically (`crate::artifacts::procedural3d::standards::v1::engine::io_registry::entries()`).

Engine files: after removing `declaration()`+`pilot_languages()`, the `//#region 🔖️Register` wrapper in procedural2d's engine file held nothing else and was deleted with its contents; in procedural3d's engine file, `//#region 🔖️Register` also wraps `register_dwg_mesh_bridge()` (unrelated, kept in place) — its own `//#endregion 🔖️Register` tag was restored after `register_dwg_mesh_bridge()` so the region stays balanced (confirmed via full `#region`/`#endregion` grep after edit — balanced in both files).

Plugin call site `🌀️procedural/🦀️component.rs:35-36`: `crate::artifacts::procedural{2d,3d}::engine::declaration()` → `crate::artifacts::procedural{2d,3d}::declaration()`.

*Note:* while editing, another live session concurrently modified `🧊️procedural3d/…/⚙️engine/🦀️component.rs` elsewhere in the same file (unrelated region, earlier in the file). Re-verified after the fact: my edit region intact, `#region`/`#endregion` balanced, zero `declaration`/`pilot_languages` hits remaining in that file.

## 🎥️shooting (crate `semio-s-plugin-shooting`)

**1 artifact, 1 `declaration()`.**

- `pilot_languages()`: `🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:40` (`fn`, already private) → `🎥️shooting/🦀️component.rs:45` (`fn`, private)
- `declaration()`: `…/⚙️engine/🦀️component.rs:27` → `🎥️shooting/🦀️component.rs:120`

**DEVIATION:** same unqualified `io_registry::entries()` local-reference issue as procedural (root file has its own incompatible `io_registry` wrapper module at `🎥️shooting/🦀️component.rs`). Fixed by qualifying to `crate::artifacts::shooting::standards::v1::engine::io_registry::entries()` (verified via `📦️glue.rs`).

Engine file: the whole `//#region 🔖️Register` held only `declaration()` + `pilot_languages()` — deleted cleanly along with its tags.

Plugin call site `🎥️shooting/🦀️component.rs:15`: `crate::artifacts::shooting::engine::declaration()` → `crate::artifacts::shooting::declaration()`.

## 📋️forms (crate `semio-s-plugin-forms`)

**1 artifact, 1 `declaration()`.**

- `pilot_languages()`: `🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:51` (`fn`, already private) → `📋️forms/🦀️component.rs:47` (`fn`, private)
- `declaration()`: `…/⚙️engine/🦀️component.rs:37` → `📋️forms/🦀️component.rs:114`

Move-both held cleanly — no deviation. `declaration()`'s body already used the fully-qualified `crate::artifacts::forms::standards::v1::engine::io_registry::entries()` and the plain local `pilot_languages()`, both unaffected by the move.

Engine file: `//#region 🔖️Register` held only these two functions (plus the `//#region 🔖️Types` block above it, untouched) — deleted cleanly along with its tags.

Plugin call site `📋️forms/🦀️component.rs:15`: `crate::artifacts::forms::engine::declaration()` → `crate::artifacts::forms::declaration()`.

---

## VERIFY — the four greps, run across all four plugin dirs together

```
$ grep -rn "fn declaration" <plugin>       # exists exactly once per artifact, at the root
➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs:212
🌀️procedural/🗿️artifacts/🌀️procedural2d/🦀️component.rs:123
🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs:122
🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs:120
📋️forms/🗿️artifacts/📋️forms/🦀️component.rs:114

$ grep -rn "engine::declaration" <plugin>  # zero hits, all four plugins

$ grep -rn "pub fn pilot_languages" <plugin>  # zero hits, all four plugins (mathematical's
                                               # pre-existing `pub` reverted to private)
```

`#[path]` resolution: every `#[path = "…"]` attribute in each plugin's `📦️glue.rs` still resolves to a file on disk (checked programmatically against all four plugins) — no dangling paths from the region deletions.

## CARGO CHECK — one run per crate, override applied, classified

All four commands run as:
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p <crate> --all-targets`

### semio-s-plugin-mathematical — 4 errors, **all upstream, none mine**
```
error[E0560]: struct `MdSnapshot` has no field named `body`
  --> …forms.../🚪️io/📤️export/🧵️serializers/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:8
error[E0308]: mismatched types (JsonValue vs Value)
  --> …/🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9
error[E0609]: no field `body` on type `&MdSnapshot`
  --> …/🚪️io/📥️import/🧩️deserializers/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:9
error[E0308]: mismatched types (JsonValue vs Value)
  --> …/🚪️io/📥️import/🧩️deserializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9
```
Root cause: `MdSnapshot`/`JsonSnapshot` are defined in `🗄️stdio` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📝️md,🔣️json}/…`) and are mid-rename upstream (`body`→`blocks`, `serde_json::Value`→stdio's own `JsonValue`). None of these files were touched by me; none is `⚙️engine` or the artifact root I edited.

### semio-s-plugin-procedural — 54 errors, **all upstream, none mine**
All errors are `E0252`/`E0432`/`E0599` inside `🧬️mutations/**` (`Procedural2dMutation`/`Procedural3dMutation` missing variants `SetWidget`/`Generation`, duplicate-import collisions) plus `testkit` glob-import ambiguity in `os` framework glue. `🧬️mutations/**` is explicitly off-limits to me per the dispatch. Verified: zero errors anchor to any `⚙️engine/🦀️component.rs` or artifact-root `🦀️component.rs` file I edited.

### semio-s-plugin-shooting — 1 error at check time, **upstream, none mine**
```
error[E0425]: cannot find value `register_app_schema` in module `crate::apps::shooting::config::schema`
  --> 🎥️shooting/🦀️component.rs:14
```
Line 14 (`.setup(crate::apps::shooting::config::schema::register_app_schema)`) was untouched by me (I only changed line 15, the `.artifact(...)` call). The target function had already been replaced by `app_schema_descriptor()` in `🎛️apps/🎥️shooting/🎚️config/🧬️schema/🦀️component.rs` (whose own doc-comment cites "ticket …W1c") — the in-flight W1c app-schema migration (a different, then-pending task per the shared task board) hadn't yet updated this `.setup()` call site. Confirmed pre-existing/concurrent, not caused by this pass.

**Post-check update:** while writing this report, the owning session landed its W1c fix live in the shared tree — `🎥️shooting/🦀️component.rs` now reads `.artifact(crate::artifacts::shooting::declaration())` with `.setup()` removed entirely (confirmed via the harness's file-change notification, not re-run through cargo). My own line (`.artifact(...)`) is untouched by their edit. Not re-verified with a fresh `cargo check` — flagging so this isn't mistaken for something I ran.

### semio-s-plugin-forms — 15 errors at check time, **all upstream, none mine**
`E0425 register_app_schema` (W1c, `📋️forms/🦀️component.rs:14`, untouched by me) plus 11× `E0599` missing `FormMutation` variants (`RemoveStep`/`AddBlock`/`MoveStep`/etc. — `🧬️mutations/**`, off-limits) plus 2× `E0308` `JsonValue`/`Value` (same stdio `JsonSnapshot` churn as mathematical).

**Post-check update:** same concurrent W1c landing hit `📋️forms/🦀️component.rs` too — `.setup()` removed, `.artifact(crate::artifacts::forms::declaration())` (my line) intact (confirmed via file-change notification, not re-run). The `🧬️mutations/**` and stdio `JsonSnapshot` errors are untouched by that landing and, per the mutations/stdio prohibitions in this dispatch, remain unverified by me.

**No error in any of the four `cargo check` runs anchors to a file I moved code into or out of** (verified per-crate by grepping error locations against the touched `⚙️engine`/artifact-root paths — zero matches).

## apa-status

**complete but UNVERIFIED — every remaining error observed at check time was upstream/concurrent churn (stdio `MdSnapshot`/`JsonSnapshot` field rename, `🧬️mutations/**` variant rename off-limits to me, and the in-flight W1c `register_app_schema`→`app_schema_descriptor` migration, which landed live in shooting and forms after my check ran), none in files this pass touched.** The four grep proofs (declaration-at-root / zero engine::declaration / zero pub pilot_languages / resolving `#[path]`s) are clean for all four plugins. One genuine deviation pattern found and handled across `🌀️procedural2d`, `🧊️procedural3d`, `🎥️shooting`: `.composers(io_registry::entries())` was qualified to `standards::v1::engine::io_registry::entries()` on the move, since the artifact root already owns its own differently-typed `io_registry` wrapper module and a bare reference would have silently rebound to it. `➗️mathematical` additionally had `pilot_languages()` already stranded as `pub` in `⚙️engine` from an earlier (now-reverted) dispatch instruction — reverted to private and folded into the move alongside the already-relocated `declaration()`.
