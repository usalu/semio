# Result: manifest split (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`)

Executor: Sonnet, 2026-10-15. Source: section C of `SEAM-MAP-PLUGIN-MANIFEST-HUB.md`. Crate: `semio-framework` (facade; `🧰️framework/📦️packages/🦀️rust/🦀️.rs:36` mounts the file as `pub mod manifest`).

## 0. Verdict

The structural split is done and statically verified. **Compile verification is blocked by the source-boundary gate**, not by the code: `bun repo cargo ... check -p semio-framework` is refused before rustc starts, before and after the split (section 6). `cargo test` was never run, so no test result exists. Everything below that says "verified" was verified statically, not by rustc.

## 1. Line counts

| File | Before | After | Active lines (non-blank, non-`//`) |
| --- | --- | --- | --- |
| `🛂️manifest/🦀️.rs` (parent) | 8,138 | 589 (585 mine + 4 from a peer's `space_index` mount, see section 8) | 373 |
| `🧮️arg-schema/🦀️.rs` | new | 1,986 | 1,637 |
| `🕹️actions/🦀️.rs` | new | 925 | 585 |
| `🧳️product-vocabulary/🦀️.rs` | new | 1,787 | 1,249 |
| `🧱️app-surface/🦀️.rs` | new | 1,048 | 719 |
| `🧾️plugin-manifest/🦀️.rs` | new | 751 | 531 |
| `🖼️view-model/🦀️.rs` | new | 1,133 | 654 |

Largest child: 1,986 lines / 1,637 active (limit asked for: about 2,500 active). Deepest new path: 112 bytes from the repo root (limit 240).

## 2. Created files and wiring

Six child folders beside the parent, each `<emoji>️<kebab>/🦀️.rs` (emoji + U+FE0F), each starting with a `//! <emoji>` docstring and `use super::*;`. The parent wires them in a `//#region 🔖️Children` block, in this form, so all 323 public top-level items keep their paths:

```
#[path = "🧮️arg-schema/🦀️.rs"]
mod arg_schema;
pub use arg_schema::*;
```

Modules: `arg_schema`, `actions`, `product_vocabulary`, `app_surface`, `plugin_manifest`, `view_model`.

Deviation from the map: the map's C3 (`introduction-tutorial`) is widened into `product-vocabulary` (see section 7), so the child count stays six. Tool run, gumball, history-edit, `AppRole`, `PackageRole`, `OsMediaCapability`, `MediaClass`, `MediaForm`, `ActionKind` and `InteractiveJobClassification` were carved out of C1/C2/C4/C6/parent into it.

| Child | Content |
| --- | --- |
| parent | header and imports, `Keybinding`, `Platform`, `PlatformKeybinding`, `ActionRef`/`ActionAddress`/`ActionInvocation`, `Utility*`, `Command*`, `Tool*`, `ElementId`, `pub mod kernel`, the 6 parent test mounts (`media_vocabulary`, `app_label`, `example_picker`, `resolved_host_context`, `gumball_verb_audience`, `argument_retirement`) |
| `arg-schema` | `ArgSchema`, `ActionArgDef`, facets, `MutationInputs` (`InputSchemaReader`), `default_action_icon_id` |
| `actions` | `ActionSemantics`, `ActionDefinition`, history filter, host event, clipboard, interaction, set-active utility/tool, document transfer |
| `product-vocabulary` | see section 7 |
| `app-surface` | `Dialog`, `ModeDefinition`, `NonEmptyVec`, `WindowKindDefinition`, panels, `AppRef`, fault notices, `AppDefinition`, action-args resolvers, breadcrumb, examples |
| `plugin-manifest` | `TopicContribution`, `Version`, `VersionPin`, `tree_pin!`, `PluginDependency`, artifact contribution, `PluginManifest`, host-resolved args, dependency graph |
| `view-model` | `ViewModel`, view-context and public-invocation capacities, `UiRefreshSection`, package descriptor, agent contributions, media vocabulary |

## 3. Visibility changes

Only two, both inside `impl ActionArgDef` in `🧮️arg-schema` (they were private and are called from the `product-vocabulary` child and from the parent's `argument_retirement_tests`):

- `fn with_schema` to `pub(super) fn with_schema`
- `fn plain_string` to `pub(super) fn plain_string`

The map's Appendix B listed 12 names for C1; the static check (section 5) found that all others are only used inside C1. No private field, private top-level item, or private method is used across children. Nothing in `🎠️kernel/**` uses a manifest-private name.

## 4. Path rewrites and other edits

- 16 `#[path = "..."]` literals of moved code got a `../` prefix (every `🧪️tests/...` mount, `🌲️canonical` in `app-surface`, `🎞️media` in `view-model`). `#[path = "../🎠️kernel/🦀️.rs"] pub mod kernel;` and `🪛️utilities/🌅️initial` stay in the parent unchanged.
- `include_str!("🔣️input-labels.json")` became `include_str!("../🔣️input-labels.json")`; `include_str!("../🌱️value/🧬️schema/🔣️.json")` became `include_str!("../../🌱️value/🧬️schema/🔣️.json")`, both in `arg-schema`.
- `semio_framework_value::artifact_retire_leaf!(MediaType);` moved from the import header to right after `MediaType` in `view-model`; `artifact_retire_leaf!(ActionKind)` moved with `ActionKind`.
- Region markers: new `//#region 🔖️ActionKind`, `InteractiveJobClassification`, `GumballChrome`, `IntroductionAction`, `AppRole`, `PackageRole`, `MediaClassification` in `product-vocabulary`; all files are balanced.
- Test module paths changed because test mods moved with their code (`cargo test` filters are module-path substrings). Updated in `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (4 lines): `manifest::tool_run_actions_tests` and `manifest::history_edit_actions_tests` now live under `manifest::product_vocabulary::`; `manifest::mutation_inputs_tests` under `manifest::arg_schema::`; the `--exact` filter `manifest::package_descriptor_value_codec_tests::...` under `manifest::view_model::`. `argument_retirement_tests`, `resolved_host_context_tests` and `kernel::*` filters are unchanged. The `--exact` one would have run zero tests and still exited 0 if left stale.
- Source-text reader: the root `📜️script.ts` (about line 6274) read `🛂️manifest/🦀️.rs` for the `for_kind` and `bounded_catalog` Migrated checks. It now concatenates the parent and the six children. Not run (it is a repo policy script).

## 5. Commands and real results

Static verification (all run, all pass; scripts in `🪓️monolith-split/🗑️generated/manifest-split-*.py`, original saved outside the ticket at `scratchpad/manifest-split-original.rs`, sha256 `34a16a80e743415370e639fb5b7ad611b7bbab5c8ad267c1ba31f5f0eaa3cdbe`; recover with `git show HEAD:<path>` if the working copy changed):

| Check | Result |
| --- | --- |
| Lossless line multiset: original vs parent + 6 children | Only intended differences: the 16 path rewrites, 2 `include_str!` lines, 2 `pub(super)` lines, module headers, `mod`/`pub use` lines, region markers. No line lost or duplicated. (`manifest-split-verify-final.log` is a filtered extract.) |
| Public top-level items (`pub fn/struct/enum/trait/type/const/static`) | 323 in the original, 323 across the new files, identical name/kind multiset. So every `semio_framework::manifest::X` and root `pub use manifest::*` path is kept. |
| Duplicate top-level names across modules | 0 |
| `rustfmt --edition 2021 --check 🦀️.rs` and `--emit stdout` (follows every `#[path]` including nested test mounts and the kernel) | parses; exit 0 for `--emit stdout`, no stderr. A deliberately broken path was detected as an error, so path resolution is really exercised. |
| Every `#[path]` and `include_str!` target exists on disk | 7 files checked, none missing |
| Private-entity closure: all private fns, consts, structs, methods, fields and tuple fields against every other module, the mounted test files and `🎠️kernel/**` (comments stripped, strings kept) | only `with_schema` and `plain_string` cross modules (fixed, section 3) |
| Emoji admission (bun `\p{RGI_Emoji}`, FE0F present, on disk, no sibling collision) | 6/6 |
| Region balance | all 7 files balanced |
| Name-resolution differential (coordinator tip): `rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-name probe --edition 2021 --crate-type lib [--test] [--target wasm32-wasip2]` on a sandbox copy of the original (sha above) and of the split tree (peer's `space_index` removed from the copy), sorted `^(error|warning)` header multisets compared (`🗑️generated/manifest-split-oracle.sh`) | native: 1,711 vs 1,711 errors, identical. wasm32-wasip2: 1,711 vs 1,711, identical. `--test`: 2,504 vs 2,504; the only differing line is the sandbox directory name inside one `couldn't read ...🔽️choices/🔣️.json` message (a fixture outside the copied dirs, missing in both), so the multisets are identical. |

Compile gate (run, blocked):

- Baseline `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework --message-format=short`: first failed twice with a transient `cargo fetch --locked ... cannot update the lock file` (peer changed `Cargo.toml`), then with `error: Rust crate Source refused before compiler: semio-framework (Rust crate Source boundary exceeded before compiler byte allocation)`. Log: `manifest-baseline-native.log`.
- After the split, the same command, and the same with `--target wasm32-wasip2` and with `--tests`: all three fail with the identical `Rust crate Source refused before compiler` error (`manifest-after-native.log`, `manifest-after-wasm.log`, `manifest-after-tests.log`). Cause: `⚡️caching/🔣️policy.json` `storage.sourceBoundary` (262,144 bytes, 32 files, 4,000 active lines) is enforced per crate; the facade is far above it with or without this split (the manifest alone was 465 KB). It cannot pass until the facade source closure is decomposed or the gate changes.
- `-- test -p semio-framework --lib manifest` was **not run** (same gate; it would be refused identically). No test result is claimed.

## 6. Residual risk (what rustc would have caught)

Caveat on the differential: it runs standalone, so the ~1,700 baseline errors are unresolved extern crates and `crate::` imports (identical in both trees). Those can hide follow-on resolution errors for names that depend on a failed import, so equality is evidence, not proof.

The split relies on `use super::*` plus `pub use child::*`; text is byte-identical apart from section 4, so the remaining risks are privacy and glob resolution. Reviewed: no private item crosses modules; no `super::`/`self::` in code (only in mounted test files, which resolve through the child's glob); no duplicate names (no glob ambiguity); `#[macro_export] tree_pin!` unaffected; serde `default =`/`skip_serializing_if =` helper functions (`introduction_*`, `tutorial_*`, `is_neutral_tone`) sit in the same child as their types; `artifact_retire_*` macro calls sit next to their types. The first real `cargo check` after the gate is resolved should be run before anyone builds on this.

## 7. Wrong-direction product vocabulary (not fixed, grouped for a follow-up)

All in `🧳️product-vocabulary/🦀️.rs` (1,787 lines; region markers name each block). Original line numbers in the old file are in parentheses.

| Region in `product-vocabulary` | Items | Why wrong direction (map W3) |
| --- | --- | --- |
| `ActionKind` | `ActionKind` enum (old 92-114) | closed enum whose docs name camera, selection, hover |
| `InteractiveJobClassification` | enum with `Migrated`/`BatchOnlyPendingRewrite`/`Deleted` (2312-2327) | migration-state labels in the framework contract |
| `GumballChrome` | `GUMBALL_CHROME_ACTION_IDS`, `framework_fixed_audience(_violation)` (2295-2310) | names the product action `setTransformGumballFlag` |
| `HistoryEdit` | 13+ `HISTORY_EDIT_*` ids, `history_edit_action_definitions`, `history_edit_finalize_dialog`, `history_edit_actions_tests` (2664-2871) | history-edit feature ids |
| `ToolRun` | `pub use semio_framework_tool_run::{...}`, `app_declares_tool_run`, `tool_run_action_definitions`, `tool_run_actions_tests` (3205-3287) | re-export of a domain crate |
| `IntroductionAction`, `Introduction` | `START_INTRODUCTION_*`, `Introduction*` family (3316-3327, 3789-4254) | onboarding product model (co-located with tutorials; not in the map's W3 list) |
| `Tutorial`, `TutorialEngine` | `Tutorial*` family, `START/RECORD_TUTORIAL_*`, `validate_tutorial`, camera interpolation (4256-5058) | tutorial model with narration and video cues |
| `AppRole` | `AppRole` {Viewer, Editor} and impls (5565-5625) | closed enum |
| `PackageRole` | `PackageRole` {Plugin, Extension} (7353-7361) | closed enum |
| `MediaClassification` | `OsMediaCapability`, `MediaClass`, `MediaForm` (7699-7710, 7758-7802) | closed enums over product topics |

Left in place, to record where they are:

- **Kernel source compiled inside the manifest:** `#[path = "../🎠️kernel/🦀️.rs"] pub mod kernel;` stays in the **parent** (`🛂️manifest/🦀️.rs`, `//#region 🔖️Kernel`), so `semio_framework::manifest::kernel` does not change.
- **UI product types in the manifest:** the `ui_wgpu::wgpu::{ActionDescriptor, NamedLayout, SurfaceKind, WindowLayout, WindowOptions}` imports stay in the parent import header (lines 23-30), reached by every child through `use super::*`.

## 8. Notes

- A peer added `📇️space-index/🦀️.rs` and its `mod space_index; pub use space_index::*;` to the parent at 02:38, after my write. I did not touch it; it uses `TopicContribution::new`/`decode` (public) and `ArtifactDialect` (parent import), so it fits the structure. Its folder emoji is also unique among siblings.
- Nothing was committed, stashed or checked out. No ticket was closed. No `[DEBUG]` logs were added.
- Files changed: the parent, six new child files, `🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `📜️script.ts` (root, one line).

## Hardening (full-resolution differential, 2026-10-15)

Full configs, baseline construction, scan tool and residual risks: `RESULT-HARDEN-MANIFEST-PLUGIN-HUB.md`.

- Oracle: `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_framework --emit=metadata -o /dev/null` from the real facade root `🧰️framework/📦️packages/🦀️rust/🦀️.rs`; baseline = sandbox with `🛂️manifest/🦀️.rs` replaced by `git show HEAD:` (sha256 `34a16a80...`) plus the peer's 3-line `space_index` mount; all other files symlinked live. The coordinator patches (strip `extern crate`, neutralise external globs) are no-ops here: the 75 modules of the facade tree have 0 external globs and 0 `extern crate` lines.
- Results (total errors, baseline / split, 0 differing error keys in each): native 1790 / 1790, `--test` 2691 / 2691, wasm32-wasip2 1790 / 1790, `--test` wasm32-wasip2 2691 / 2691. The facade has no cargo features. No regression, no widening needed. Sensitivity: making `struct ActionArgDef` private adds 60 errors (E0422/E0425/E0433) in this oracle.
- Typeck is cut short (unresolved trait paths such as `impl ToValue for` abort rustc before typeck), so privately widened methods are not seen: making `with_schema` private is not reported. Definition scan (`scan.py`, receiver refinement `scan2.py`, item scan `scan3.py`) over the facade manifest subtree: 36 candidate uses of private fields/methods defined in the six children and `space-index`, all name collisions with other types (`InputVariant`, `ResolvedInput`, `InputSchemaReader` are private to `🧮️arg-schema`; `.first`, `.0`, `Self::new`); 0 type-matching uses; item scan 0; trait scope 0. The scan reproduces the seeded `with_schema` case.
- Residual: typeck-phase privacy and trait scope unproven by rustc.
