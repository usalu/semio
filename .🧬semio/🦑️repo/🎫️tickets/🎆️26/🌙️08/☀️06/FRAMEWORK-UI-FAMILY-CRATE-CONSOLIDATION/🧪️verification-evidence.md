# 🧪️ Verification evidence — merged `semio-framework-ui`

All runs used the TEMPLATE.md §3 temporary `[workspace]` overlay on
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (the crate is not a root member until the
registrar swap). Overlay + nested `target/` + nested `Cargo.lock` **deleted before handoff** — verified
absent. Toolchain: `cargo 1.99.0-nightly`, `DEVELOPER_DIR=/Library/Developer/CommandLineTools`
(`/Applications/Xcode.app` refuses to link — unsigned license agreement on this machine).

| # | Command (`cargo … --manifest-path Cargo.toml`) | Result |
|---|---|---|
| 1 | `check --features tui` | ✅ clean |
| 2 | `check --features wgpu` | ✅ clean |
| 3 | `check --features typegen` | ✅ clean (only ts-rs's pre-existing "failed to parse serde attribute" notes) |
| 4 | `check --features tui-terminal` | ✅ clean |
| 5 | `check --features tui,wgpu,typegen` | ✅ clean — both targets coexist in one crate |
| 6 | `check --features wgpu-engine` | ✅ compiles (5 unused-import warnings, all in files the concurrent godfile-split agent created) |
| 7 | `check --target wasm32-wasip2 --features wgpu` | ✅ clean — program-component admission gate |
| 8 | `check --target wasm32-unknown-unknown --features tui` | ✅ clean |
| 9 | `check --target wasm32-unknown-unknown --features tui-bindgen` | ✅ clean |
| 10 | `clippy --features tui-terminal,wgpu-engine,typegen` | ⚠️ 36 warnings, 0 errors — see below |
| 11 | `bun ./📜️script.ts generate` then `check` | ✅ "ui axes are fresh (2 locales, 2 terminologies)" |

Raw logs: `verify/`.

## Clippy composition (36 warnings, none an error)

Pre-existing campaign-baseline style lints on *relocated* source, not introduced by the merge:
13× `map_unwrap_or`, 3× `cloned_instead_of_copied` on `IconName`, 3× `map(…).unwrap_or(false)`,
2× recursion-only parameter, 2× manual checked division, plus one each of `too_many_arguments` (×2),
derivable `impl`, `should_implement_trait` on `from_str`, `explicit_deref_methods`, `sort_by_key`,
`semicolon_if_nothing_returned`, `vec_init_then_push`.

4 unused imports + 1 unnecessary qualification are in `🎯️targets/🧊️wgpu/🦀️{draw,events,engine,widgets}.rs`
— files that did not exist when this ticket started; they were produced minutes earlier by the
concurrent `UI-ELEMENT-CO-LOCATION-RESTRUCTURE` godfile split and were deliberately left to that owner.

## `cargo test` — red, and not from this merge

`cargo test --features tui-terminal,wgpu-engine` fails to build the **test** harness with 88 errors while
the **lib** builds clean in every feature combination above. Every error is inside a `#[cfg(test)]`
module and every one names an item the concurrent godfile split relocated:

- `cannot find type UiTreeActionPlacement in this scope` (test mods in the split-out region files)
- 80+× `the trait bound label_impl::Label: From<&str> is not satisfied` — `label_impl` is the module
  name introduced by the split's `#[path = "🦀️label.rs"] mod label_impl;`; the test-only
  `impl From<&str> for Label` that used to sit beside `Label` in the godfile was not carried over.

Nothing in this ticket touches test bodies: the only source edits here were the mechanical
`crate::` → `crate::{tui,wgpu}::` prefixing and the feature-gate renames (527 substitutions, script:
`rewrite-target-paths.mjs`). Re-run `cargo test` after the split agent finishes.

## TypeScript side

No TS file was moved by this ticket — `@semio-tech/ui-react` was already at its Shape V2 path when this
session resumed, and the 41 k-line `📦️index.tsx` (and `renderer-react`'s 32 k-line file, which lives
under the os renderer family, not here) were deliberately **not** split: the plan tickets that as a
separate follow-up.

`bun nx run @semio-tech/ui-react:typecheck` cannot run right now: nx fails at project-graph construction
with *"projects defined in multiple locations"* for `@semio-tech/framework-core-rs`,
`@semio-tech/schema-rs` and `@semio-tech/dsl-family-catalog-rs` — all from the concurrent
`FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH` / os-kernel work, none of them ui. Confirmed no ui project
name is duplicated, so the `@semio-tech/ui-wgpu-rs` + `@semio-tech/ui-tui-rs` → `@semio-tech/ui-rs`
rename introduces no collision.

## Storybook (C9)

Static consistency check of `.storybook/scopes.ts` — every literal path in the hand-curated `ui` scope
plus both ui-react aliases resolve to existing files. `.storybook/**` was not edited (ui-react did not
move in this ticket). Two dangling sourceRoots belonging to other families are listed in
`📋️registrar-handoff.md` §5.

## Deletions

- `🎨️styling/📦️packages/🐍️python/.venv/` — confirmed a real uv venv (`pyvenv.cfg`: CPython 3.14, uv 0.11.15) before removal.
- `🎨️styling/📦️packages/🔷️dotnet/` — held a single source-less `Elements.Styling.csproj`; `Monorepo.sln`
  references the *compose* copy (`compose/client/lib/net/…`), never this one. Genuinely orphaned.

## Flagged, not fixed (pre-existing, outside this ticket)

`🎨️styling/📦️packages/🦀️rust/🛂️manifest.jsonadapters.manifest.json` has a corrupt concatenated filename,
and both that manifest and `🎨️styling/📦️packages/🦀️rust/📜️script.ts` contain literal `"ln"` where a
generated filename belongs (same corruption class visible in `compose/client/lib/net/…csproj`'s
`🛂️n.manifest.json`). Left alone: pre-dates this ticket and touching the styling generator was out of scope.


## Finish pass (2026-08-06 ~13:10)

Root `cargo check -p semio-framework-ui-wgpu|tui|styling` blocked: target `Cargo.toml` files correctly
absent post-consolidation, but root members not yet swapped (registrar). Also core→ui_wgpu still points
at the deleted wgpu target path.

Isolated overlay on merged `semio-framework-ui` (optional `kernel_3d_scene` temporarily removed so
resolve does not walk core→deleted ui-wgpu):

| Features | Result |
|---|---|
| `tui` | ✅ |
| `wgpu` | ✅ (pulls styling) |
| `typegen` | ✅ |
| `tui-terminal` | ✅ |
| `tui,wgpu,typegen` | ✅ |
| `wgpu-engine` | ⛔ blocked on registrar dependent repoint (§9) |

Also fixed `🎯️targets/{tui,wgpu}/📦️lib.rs` element `#[path]`s after UI-ELEMENT emoji folder rename.
Logs: `verify/recheck-*.txt`.

## Finish pass (2026-08-06) — final

- Root members/workspace.deps/consumers already swapped to `semio-framework-ui` (registrar ahead of handoff).
- Per-target `Cargo.toml` absent; parent owns `build.rs`.
- Element `#[path]`s updated for UI-ELEMENT emoji folders.
- Isolated overlay: `tui`, `wgpu`, `typegen`, `tui-terminal`, `tui+wgpu+typegen` all ✅ (`verify/recheck-*.txt`).
- Root `-p` blocked by unrelated duplicate `semio-framework-editor`.
- Ticket closed via filesystem (`🎫️ticket.json` status=closed).
