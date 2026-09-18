# P2 — Dormant Plugin Compile Sweep (2026-09-18)

Slice P2. Scope: the two broken sampled crates from `📓️audit-build-infra.md` (equation half — `semio-hub`
belongs to another worker) plus every crate of the **12 UNTESTED plugins** from `📓️audit-plugins-artifacts.md`
Q1 (writer, mathematical, vcs, animate, sequence, architect, reasoning, norm, playbook, imperative, dag, space).
No `cargo test` run (wave 4 owns tests). `🌎️hub/**` and `🧰️framework/🛍️products/🖥️server/**` untouched.
One cargo at a time, always `--message-format short`.

## Headline

- **46 crates enumerated** across the 12 plugins (`cargo metadata --no-deps`, filtered by `manifest_path`).
- **41 of 46 native-green**, including both crates I had to fix.
- **2 real root-cause defects found and fixed** (2 files, +9/−1 lines) — both dormant-code drift, both now green.
- **5 crates still red — none of them for a reason inside the 12 plugins.** All five fail transitively on a
  **live, uncommitted peer edit** to `semio-s-artifact-stdio-pdf` (ticket `26/09/18/PDF-ARTIFACT-SPEC-COMPLETE`).
  Left alone deliberately per AGENTS.md (never revert/collide with a concurrent agent's edits).
- **3 of 4 requested `wasm32-wasip2` component builds green** (`space`, `dag`, `imperative`); the fourth
  (`writer`) is blocked by the same peer-owned `stdio-pdf` breakage, not by wasm-gated code.

## The `component-app-assembly` feature does not exist on these crates

Only **2 of the 46** crates carry the feature — `semio-s-artifact-space-home` and `semio-s-artifact-space-space`.
`cargo check … --features component-app-assembly` on any of the other 44 is a hard cargo error before any
compilation (`error: the package '<x>' does not contain this feature`), so those 44 ran with **no feature flag**,
which is the correct fallback: these plugins gate nothing behind that feature, so a bare check does compile
their app code (confirmed — the `E0422` errors the infra audit reported for `mathematical` reproduced on a
bare check and disappeared only after the source fix).

Full feature inventory of the 46:

| feature set | crates |
|---|---|
| *(no features at all)* | writer ×2, mathematical ×2, vcs ×2, sequence ×2, architect ×2, reasoning ×2, playbook ×3, imperative-procedure, imperative (root), dag ×2 |
| `default`, `preview-window`, `winit` | animate ×2 |
| `default`, `cross-fem` | norm ×17 |
| `default`, `extension-entry` | imperative extensions ×5 |
| `default`, `plugin-entry` | `semio-s-plugin-space` |
| `default`, **`component-app-assembly`** | `semio-s-artifact-space-home`, `semio-s-artifact-space-space` |

## Root causes fixed

### 1. `semio-s-artifact-mathematical-equation` — E0422 ×2, missing `use` (task 1)

`EquationCamera`'s real home is
`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🦀️.rs:6`,
already surfaced by the `…/🕸️graph/🎚️config` module (the sibling `…/🕸️graph/🦀️.rs:10` imports it exactly that way,
and the crate root re-exports it at `🗿️artifacts/➗️equation/🦀️.rs:152`). The editor file already imported two other
names from that same `config` module but not this one, so the two struct literals at lines 535 and 1318 were
unresolved. Fixed by widening the existing import — **no re-export shim added, no new `pub use`**:

`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24`

```
-use crate::editor::equation::modes::edit::windows::graph::config::{EquationGraphWindowConfigMutation, EquationGraphWindowConfigOwner};
+use crate::editor::equation::modes::edit::windows::graph::config::{EquationCamera, EquationGraphWindowConfigMutation, EquationGraphWindowConfigOwner};
```

Why it was dormant: both use sites are inside `EquationCommand::NodeGraphViewport` handling — the window-config
camera path added by the per-window-config refactor. Nothing else in the workspace depends on this crate, so the
breakage sat at HEAD unnoticed (matches the audit's "UNTESTED" classification).

### 2. `semio-s-plugin-playbook-procedural` — E0277, snapshot missing `ArtifactCompositionFields`

`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:571` (`type Snapshot = ModuleRenderPayload;`) failed:

```
error[E0277]: the trait bound `component::ModuleRenderPayload: ArtifactCompositionFields` is not satisfied
```

`ArtifactApp::Snapshot`'s bound list (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11677`) requires
`semio_framework_schema::ArtifactCompositionFields`. The framework gives two ways to satisfy it:

- `#[derive(ArtifactSchema)]` (`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs:13`), which emits the impl alongside
  `ArtifactSchemaFields` — used by the *schema-first* snapshots (e.g. `📖️playbook`'s own artifact, `🕸️dag`'s); or
- a hand-written impl next to the hand-written codecs — used by every **P6 handcrafted-codec** snapshot in `✏️s`
  (`🧩️puzzle` 2d/3d/5d, e.g. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🧬️mutations/🦀️.rs:454`).

`ModuleRenderPayload` is squarely the second kind — its `ArtifactDsl` and `ArtifactPack` impls in the same
`🔖️ArtifactCodec` region are both explicitly labelled "Handcrafted (P6)", and its `params` field is a deliberately
untyped `DslValue` escape hatch that no `ArtifactSchema` slot table can describe (the struct's own docstring says
so). So the correct root-cause fix is the truthful composition declaration, matching the puzzle siblings:

`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:168-175` (new)

```rust
/// 🧒️ Composition view of the module payload — every field is a scalar or the untyped `params`
/// escape hatch, so this block-kind module owns no child artifacts and links to none.
impl store::os_schema_composition::ArtifactCompositionFields for ModuleRenderPayload {
    fn visit_child_refs<'a, V: store::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}
```

No new dependency was needed: `store` is `semio_framework_os_kernel` (via
`semio_framework_plugin::plugin_app_close_prelude`, `🧰️framework/…/🔌️plugin/🦀️.rs:39513`), which owns
`pub mod os_schema_composition` — the same trait `semio_framework_schema` re-exports
(`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs:154`). This is the only one of the 26 nested extension crates in the
12 that had drifted; the 5 `imperative` extensions and `playbook`'s root were already green.

## Result table — all 46 crates

Result "before" is the state at HEAD + peer working tree when P2 started; "after" is the state at the end of this slice.

| plugin | crate | features used | before | after | files changed |
|---|---|---|---|---|---|
| ✒️writer | `semio-s-artifact-writer-writer` | *(none — no CAA)* | 🔴 45 errs in dep `stdio-pdf` | 🔴 same (peer-owned) | — |
| ✒️writer | `semio-s-plugin-writer` | *(none)* | 🔴 dep `stdio-pdf` | 🔴 same (peer-owned) | — |
| ➗️mathematical | `semio-s-artifact-mathematical-equation` | *(none)* | 🔴 E0422 ×2 | 🟢 green | `…/✳️any/✏️editor/🦀️.rs:24` |
| ➗️mathematical | `semio-s-plugin-mathematical` | *(none)* | 🟢 | 🟢 green | — |
| 🌿️vcs | `semio-s-artifact-vcs-vcs` | *(none)* | 🟢 | 🟢 green | — |
| 🌿️vcs | `semio-s-plugin-vcs` | *(none)* | 🔴 dep `stdio-pdf` (via `semio-s-plugin-stdio`) | 🔴 same (peer-owned) | — |
| 🎞️animate | `semio-s-artifact-animate-presentation` | *(none — has `winit`/`preview-window`, not CAA)* | 🔴 dep `stdio-pdf` | 🔴 same (peer-owned) | — |
| 🎞️animate | `semio-s-plugin-animate` | *(none)* | 🔴 dep `stdio-pdf` | 🔴 same (peer-owned) | — |
| 🎬️sequence | `semio-s-artifact-sequence-sequence` | *(none)* | 🟢 | 🟢 green | — |
| 🎬️sequence | `semio-s-plugin-sequence` | *(none)* | 🟢 | 🟢 green | — |
| 🏛️architect | `semio-s-artifact-architect-program` | *(none)* | 🟢 | 🟢 green (158 s cold) | — |
| 🏛️architect | `semio-s-plugin-architect` | *(none)* | 🟢 | 🟢 green | — |
| 💡️reasoning | `semio-s-artifact-reasoning-wires` | *(none)* | 🟢 | 🟢 green | — |
| 💡️reasoning | `semio-s-plugin-reasoning-mindmap` | *(none)* | 🟢 | 🟢 green | — |
| 📕️norm | `semio-s-artifact-norm-contract` | *(none — has `cross-fem`)* | 🟢 | 🟢 green | — |
| 📕️norm | `…-en1990` … `…-en1999` (10) | *(none)* | 🟢 | 🟢 green | — |
| 📕️norm | `…-din4108`, `…-din16798`, `…-din18599` | *(none)* | 🟢 | 🟢 green | — |
| 📕️norm | `…-iso16757`, `…-vdi3805` | *(none)* | 🟢 | 🟢 green | — |
| 📕️norm | `semio-s-plugin-norm` | *(none)* | 🟢 | 🟢 green (67 s) | — |
| 📖️playbook | `semio-s-artifact-playbook-playbook` | *(none)* | 🟢 | 🟢 green | — |
| 📖️playbook | `semio-s-plugin-playbook` | *(none)* | 🟢 | 🟢 green | — |
| 📖️playbook | `semio-s-plugin-playbook-procedural` | *(none)* | 🔴 E0277 ×1 | 🟢 green | `🧩️extensions/🌀️procedural/🦀️.rs:168` |
| 📜️imperative | `semio-s-artifact-imperative-procedure` | *(none)* | 🟢 | 🟢 green | — |
| 📜️imperative | `semio-s-plugin-imperative` | *(none)* | 🟢 | 🟢 green | — |
| 📜️imperative | `…-control`, `…-effect`, `…-logic`, `…-math`, `…-text` | *(none — have `extension-entry`)* | 🟢 | 🟢 green | — |
| 🕸️dag | `semio-s-artifact-dag-dag` | *(none)* | 🟢 | 🟢 green | — |
| 🕸️dag | `semio-s-plugin-dag` | *(none)* | 🟢 | 🟢 green | — |
| 🪐️space | `semio-s-artifact-space-home` | **`--features component-app-assembly`** | 🟢 | 🟢 green | — |
| 🪐️space | `semio-s-artifact-space-space` | **`--features component-app-assembly`** | 🟢 | 🟢 green | — |
| 🪐️space | `semio-s-plugin-space` | *(none — has `plugin-entry`)* | 🟢 | 🟢 green (100 s) | — |

Totals: **41 green / 5 red**, red all transitive on one peer-owned crate.

## Task 3 — `wasm32-wasip2` component builds

`--features component-app-assembly` does not exist on any of these four plugin-root crates, so all four ran bare.
None took anywhere near the ~15 min budget (the shared `wasm32-wasip2` build dir was warm at 54 GB).

| crate | command | elapsed | result |
|---|---|---|---|
| `semio-s-plugin-space` | `cargo check -p semio-s-plugin-space --target wasm32-wasip2 --message-format short` | 16 s | 🟢 green |
| `semio-s-plugin-dag` | `cargo check -p semio-s-plugin-dag --target wasm32-wasip2 --message-format short` | 42 s | 🟢 green |
| `semio-s-plugin-imperative` | `cargo check -p semio-s-plugin-imperative --target wasm32-wasip2 --message-format short` | 40 s | 🟢 green |
| `semio-s-plugin-writer` | `cargo check -p semio-s-plugin-writer --target wasm32-wasip2 --message-format short` | 10 s | 🔴 dep `stdio-pdf` (peer-owned) |

**No wasm-gated breakage found.** Nothing under `cfg(target_arch = "wasm32")` in `space`/`dag`/`imperative` failed
that the native check had hidden — the wasm component surface of these three dormant plugins compiles clean today.

## Still red — and why I did not fix it

All five red crates fail with the **same 45 errors, all inside `semio-s-artifact-stdio-pdf`**, zero errors in any
file under the 12 plugins. Evidence this is a live peer edit, not committed breakage:

- `git status --porcelain` on that path: `M …/🧱️base/🧬️schema/📸️snapshot/🦀️.rs` and an **untracked new directory**
  `…/🧱️base/🔨️modules/` (a lexer module being added).
- mtimes at the time of the sweep: `…/🔨️modules/🔤️lexer/🧪️tests/🔬️unit/🦀️.rs` 21:00, `…/🔤️lexer/🦀️.rs` 20:59,
  `…/📸️snapshot/🦀️.rs` 20:57 — i.e. minutes before my first check.
- Last commit touching the path is `3250e6cb90` (2026-09-15), so the breakage is **not** at HEAD.
- Owner: ticket `26/09/18/PDF-ARTIFACT-SPEC-COMPLETE` (listed as "today" in `📓️audit-plugins-artifacts.md`).

The shape of the errors is a classic mid-refactor snapshot rename: `PdfPage.text` turned from a field into a
method, `PdfInfo` gained `creation_date`/`modification_date`/`extra`/+1 field, `PdfStreamFilter` gained
`Lzw`/`Dct`/`Jpx`/+3 variants, and `io::text_document` was removed — with the `🚪️io` / `🔺️diff` / `💡️inferences`
callers not yet migrated. Fixing those is squarely inside another worker's in-flight edit and would either be
reverted or would collide, so per AGENTS.md I left every one of those files untouched.

Representative distinct errors (from `🗑️generated/p2-check-semio-s-plugin-writer.txt`, paths shortened to `pdf/…`):

```
pdf/…/🧬️schema/📸️snapshot/🦀️.rs:2273:59  error[E0425]: cannot find function `text_document` in module `crate::standards::v1_7::subsets::base::io`
pdf/…/🚪️io/🦀️.rs:2391:21                 error[E0063]: missing fields `creation_date`, `extra`, `modification_date` and 1 other field in initializer of `…::PdfInfo`
pdf/…/🚪️io/🦀️.rs:2556:51                 error[E0615]: attempted to take value of method `text` on type `&…::PdfPage`
pdf/…/🚪️io/🦀️.rs:1736:92                 error[E0560]: struct `…::PdfPage` has no field named `text`
pdf/…/🚪️io/🦀️.rs:2165:25                 error[E0004]: non-exhaustive patterns: `…::PdfStreamFilter::Lzw { .. }`, `…::Dct { .. }`, `…::Jpx` and 3 more not covered
pdf/…/🧬️schema/📸️snapshot/🦀️.rs:1939:25  error[E0502]: cannot borrow `out` as immutable because it is also borrowed as mutable
pdf/…/🧬️schema/📸️snapshot/🦀️.rs:1940:25  error[E0499]: cannot borrow `out` as mutable more than once at a time
error: could not compile `semio-s-artifact-stdio-pdf` (lib) due to 45 previous errors
```

**Recommendation for the coordinator**: re-run exactly these five once `26/09/18/PDF-ARTIFACT-SPEC-COMPLETE` lands.
Nothing else is needed — none of the five has any error of its own.

```
cargo check -p semio-s-artifact-writer-writer     --message-format short
cargo check -p semio-s-plugin-writer              --message-format short
cargo check -p semio-s-artifact-animate-presentation --message-format short
cargo check -p semio-s-plugin-animate             --message-format short
cargo check -p semio-s-plugin-vcs                 --message-format short
cargo check -p semio-s-plugin-writer --target wasm32-wasip2 --message-format short
```

## Exact commands + output tails

Task 1 (`🗑️generated/p2-math-check.txt`). The requested form was rejected by cargo before any compilation:

```
$ cargo check -p semio-s-artifact-mathematical-equation --features component-app-assembly --message-format short
error: the package 'semio-s-artifact-mathematical-equation' does not contain this feature: component-app-assembly
help: packages with the missing feature: semio-s-artifact-fem-2d, semio-s-artifact-stdio-csv, … (58 listed)
```

so the capture holds the bare check, which is the one that exercises the app code:

```
$ cargo check -p semio-s-artifact-mathematical-equation --message-format short
…
warning: `semio-framework-plugin` (lib) generated 40 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 34 suggestions)
    Checking semio-s-artifact-mathematical-equation v0.1.0 (…/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1m 14s
exit=0
```

The 40 `semio-framework-plugin` warnings are pre-existing and shared by every capture here (they are the same 40
the infra audit recorded for `semio-s-artifact-note-note`) — proof the check really expanded and type-checked the
framework surface rather than short-circuiting.

Task 2/3 driver (per crate, sequentially, never more than one cargo live):

```
cargo check -p <crate> [--features component-app-assembly] --message-format short  > 🗑️generated/p2-check-<crate>.txt 2>&1
cargo check -p <crate> --target wasm32-wasip2 --message-format short               > 🗑️generated/p2-wasm-<crate>.txt 2>&1
```

Green tails all look like:

```
    Checking semio-s-plugin-playbook-procedural v0.1.0 (…/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 4.0s
exit=0
```

```
    Checking semio-s-plugin-space v0.1.0 (…/🪐️space/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 16s          # --target wasm32-wasip2
exit=0
```

## Captures kept / deleted

Kept in `🗑️generated/`:

- `p2-math-check.txt` — task 1's required capture.
- `p2-check-semio-s-artifact-writer-writer.txt`, `p2-check-semio-s-plugin-writer.txt`,
  `p2-check-semio-s-artifact-animate-presentation.txt`, `p2-check-semio-s-plugin-animate.txt`,
  `p2-check-semio-s-plugin-vcs.txt` — the five still-red crates, full error text.
- `p2-wasm-semio-s-plugin-space.txt`, `p2-wasm-semio-s-plugin-dag.txt`, `p2-wasm-semio-s-plugin-imperative.txt`,
  `p2-wasm-semio-s-plugin-writer.txt` — task 3.
- `p2-check-semio-s-plugin-playbook-procedural.txt` — the post-fix green of the second defect.

Deleted (own files only, listed explicitly — no folder sweep): the 39 other `p2-check-*.txt` green captures,
whose only content was the shared framework warning block plus `Finished … exit=0`, now recorded in the table above.

## Environment notes

- Host was busy throughout (peer `rustc`/`cargo` live at every sample; `semio-s-artifact-remodel-remodeling`,
  a `wasm32-wasip2` plugin build and the `stdio-pdf` work all concurrent). No lock wait ever exceeded a few
  seconds and no deadlock was hit — one cargo at a time, no process killed.
- Disk at slice end: **111 GiB free of 926 GiB (88 % used)**; `wasm32-wasip2` build dir 54 GB. This slice added
  no measurable growth (checks only, no codegen).
- `cargo metadata --no-deps --format-version 1` parsed with `python3`; scratch copy kept out of the repo.
