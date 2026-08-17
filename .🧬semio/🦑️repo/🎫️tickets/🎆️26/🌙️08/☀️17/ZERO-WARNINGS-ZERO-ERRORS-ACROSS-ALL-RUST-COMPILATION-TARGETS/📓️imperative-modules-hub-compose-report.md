# Imperative Modules + Hub + Compose — Warning Sweep Report

Scope (6 workspace members, all verified via individual `cargo check -p <name> --message-format=short`):

1. `semio-s-imperative` — `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust`
2. `semio-s-imperative-extension-sdk` — `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/📦️packages/🦀️rust`
3. `semio-s-plugin-imperative` — `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust`
4. `semio-hub` — `🌎️hub/📦️packages/🦀️rust`
5. `semio-compose-query` — `compose/client/lib/query/rs`
6. `semio-compose-gql` — `compose/client/bin/gql/rs`

## Results summary

| Crate | Starting warnings | Ending warnings | Errors |
|---|---|---|---|
| `semio-s-imperative` | 0 (already clean) | 0 | 0 |
| `semio-s-imperative-extension-sdk` | 0 (already clean) | 0 | 0 |
| `semio-s-plugin-imperative` | 13 | 0 | 0 |
| `semio-hub` | 0 (already clean) | 0 | 0 |
| `semio-compose-query` | 0 (already clean) | 0 | 0 |
| `semio-compose-gql` | 0 (already clean) | 0 | 0 |

All 6 crates' `(lib)` targets are now at 0 warnings / 0 errors. `#1`, `#2`, `#4`, `#5`, `#6` were
already clean before this session touched anything — no edits were needed there, confirmed by a
plain `cargo check -p <name>` producing only `Checking`/`Finished` lines, zero `warning:`/`error:`
lines.

## `semio-s-plugin-imperative` — the only crate needing fixes (13 → 0)

All fixes are in this plugin's own files under
`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
No `#[allow(...)]` used anywhere; nothing suppressed.

### 1. `🚪️io/🦀️component.rs` (2 warnings)
- **`hidden_lifetime_parameters`** at the `derived_composition::ImperativeComposerComposition::compose`
  method (line 70): `sources: &[ComposeSource]` → `sources: &[ComposeSource<'_>]`. This is the same
  recurring `ComposeSource<'_>` pattern already fixed across ~10 other plugins this session (see
  `📓️progress.md`).
- **`unused_imports`**: deleted `use semio_framework_plugin::ArtifactAnalyzer as _;` (line 51).
  Traced the call site (`ImperativeAnalyzer::analyze(&[native])`, line 77) back through
  `derive_artifact_facets!` (`🧰️framework/…/🔌️plugin/🦀️component.rs:18653`) — the macro generates
  **both** a trait impl (`impl ArtifactAnalyzer for $analyzer`) **and** an inherent
  `impl $analyzer { pub fn analyze(...) }` that shadows it for bare-path calls. Since the call site
  never needs the trait itself in scope (inherent method wins), the glob/`as _` trait import was
  genuinely dead — same "dead `ArtifactAnalyzer as _` import" shape noted as recurring in
  `📓️progress.md`.

### 2. Per-command editor helper duplicates (11 warnings, `dead_code`)
Each file under `✏️editor/🎮️commands/🔧️<command>/🦀️component.rs` carries its own **private,
non-`pub`, copy-pasted** `//#region 🔖️Helpers` block (`next_step_id`, `path_ref_from`, `steps_at`,
`resolve_contains`) — not a shared module, so dead-in-one-file does not imply live elsewhere. Since
these are file-scoped `fn` (no `pub`), a crate-wide grep for callers was unnecessary beyond each
file's own `handle()` — confirmed no other file references another file's copy (they can't; not
exported). This is the same "per-command dead helper duplicates, each editor command is a
self-contained leaf" shape already established for the `trinity` plugin this session.

Verified per-file which helpers `handle()` actually calls, then deleted exactly the unused ones,
keeping the ones each command genuinely needs (e.g. `add-step` is append-only post- ticket
`26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`, so it no longer needs owner/slot resolution
at all):

| File | Deleted (dead) | Kept (live) |
|---|---|---|
| `🔧️add-step/🦀️component.rs` | `path_ref_from`, `steps_at`, `resolve_contains` | `next_step_id` |
| `🔧️add-step-at/🦀️component.rs` | `steps_at`, `resolve_contains` | `next_step_id`, `path_ref_from` |
| `🔧️remove-step/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |
| `🔧️remove-step-at/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |
| `🔧️move-step/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |
| `🔧️move-step-at/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |
| `🔧️set-step-params/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |
| `🔧️set-step-params-at/🦀️component.rs` | `next_step_id` | `path_ref_from`, `steps_at`, `resolve_contains` |

In every case the surviving helpers keep the imports they need (`PathRef`, `Step` types stayed
imported because at least one surviving helper/handle site still uses them) — no import cleanup
was required beyond the function bodies themselves.

## Nothing left alone / no judgment calls needed
Every warning in-scope for these 6 crates was resolved; none were deliberately left as intentional
scaffolding.

## Out-of-scope, noted but not touched (per task brief)
- `semio-framework-plugin` (12 warnings) and `semio-s-plugin-stdio` (4 warnings) show up as
  "Checking" dependencies in `semio-s-plugin-imperative`'s build graph and print their own warnings
  — these are **not** part of this scope, already independently tracked/partially handled elsewhere
  in `📓️progress.md` (stdio's 4 remaining are documented judgment calls from the stdio report;
  framework-plugin's count here (12) differs from the 2-remaining figure in an earlier progress note
  — looks like a concurrent session may have touched it again since; not investigated, not this
  ticket's scope).
- `semio-compose-rs` (`compose/client/lib/rs`) — 89 warnings, explicitly out of scope per the task
  brief (blocked by the cross-cutting `Mutation::apply`/`::diff` migration described in
  `📓️progress.md`). Its `(lib)` target does compile (no errors) as a dependency of both
  `semio-compose-query` and `semio-compose-gql`, so those two crates' own `(lib)` targets check
  cleanly; `semio-compose-rs`'s own warnings were left untouched as directed.
- One transient failure was hit and resolved itself: an early `cargo check -p
  semio-s-plugin-imperative` run failed with 3 errors (`E0308`/`E0631`/`E0599`) in
  `semio-framework-os-kernel`'s `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` — confirmed via
  `git status` (`MM` on that file plus a new untracked `🧬️schema/🦀️component.rs`) to be a
  concurrent session's in-flight edit, not caused by this session. A retry ~20s later, and all
  subsequent checks, compiled that crate cleanly — not a real bug in scope here, just workspace
  churn from concurrent multi-session editing (per `📓️progress.md`'s established pattern).

## Files touched this session
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step-at/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️remove-step/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️remove-step-at/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️move-step/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️move-step-at/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️set-step-params/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️set-step-params-at/🦀️component.rs`

(8 files total, all within `semio-s-plugin-imperative`'s own tree. No files touched in the other 5
crates — they were already clean.)

## Final verification commands (all run individually, synchronously, foreground)
```
cargo check -p semio-s-imperative --message-format=short              → 0 warnings, 0 errors
cargo check -p semio-s-imperative-extension-sdk --message-format=short → 0 warnings, 0 errors
cargo check -p semio-s-plugin-imperative --message-format=short        → 0 warnings, 0 errors
cargo check -p semio-hub --message-format=short                        → 0 warnings, 0 errors
cargo check -p semio-compose-query --message-format=short              → 0 warnings, 0 errors
cargo check -p semio-compose-gql --message-format=short                → 0 warnings, 0 errors
```
