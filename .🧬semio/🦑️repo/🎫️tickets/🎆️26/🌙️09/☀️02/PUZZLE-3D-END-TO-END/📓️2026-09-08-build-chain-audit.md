# 🔍️ Puzzle 3D Rust Build-Chain Audit — 2026-09-08 23:50 CEST

Read-only audit. No source edited, no cargo/bun run, no git-mutating command used. All conclusions
below come from reading source, `Cargo.toml`/workspace manifests, `stat`/`find` mtimes, `git status`
(uncommitted state only, no commits made), existing on-disk cargo logs, and a live `ps` snapshot.

## TL;DR

1. **Verdict: UNKNOWN, leaning "not cleanly right now."** Not because of a known bug in puzzle's own
   code — every previously-diagnosed blocker checked below looks **fixed in current source** — but
   because two crates the puzzle chain depends on (`semio-framework-plugin` directly;
   `semio-framework-os-flow`/`semio-framework-os-kernel` transitively via the combined wgpu-renderer
   bundle) are under **live, uncommitted, multi-agent editing at this exact moment**.
2. `semio-s-artifact-puzzle-3d`'s real compile bug from an old log (§5 below,
   `replace_attraction_semio_framework_geometry` E0433) is **verified fixed** in current source.
3. The §45-§46 `findings-2026-09-05.md` blocker — `semio-framework-os-flow`'s `use crate::artifact::*`
   / `vcs` collision / `protocol::value::ordered` — is **verified resolved** in current source (§2).
4. A stale log also showed `semio-framework-os-kernel` failing on `SpaceHost` missing
   `ensure_durable_group_idle`/`pump`/`bump`/`replace_backbone_retained`; current source has all four
   correctly under `impl ArtifactStore`, matching the peer ticket's own note that they moved them (§5).
5. **Most likely present-moment blocker if any exists right now:** `semio-framework-plugin` — a
   *direct* dependency of `semio-s-plugin-puzzle` — is HOT: its crate root and `⚛️reactor/🦀️.rs` were
   edited 6-10 minutes before this audit, and a live `cargo test … surface_context` process is
   recompiling `semio_framework_plugin` as this is written (§6).
6. `semio-framework-os-kernel`'s own `📡️spr` submodule got a synchronized 17-file batch edit ~7 min
   ago, all identical timestamp — looks like an automated pass, confined to `#[cfg(test)]` fixtures,
   unlikely to affect a plain lib build (§1/§6).
7. Machine is extremely loaded: **36 live cargo/rustc processes** across at least 4 independent ticket
   sessions, including one giant `--keep-going` check that literally lists every puzzle crate, flow,
   os-kernel, all stdio artifacts and the wgpu renderer on one command line — still mid-compile,
   no fresh pass/fail captured for the puzzle target during this audit (§7).
8. stdio `✳️base`→`🧱️base` rename: **directory rename is complete** (0 `✳️base` dirs, 12 `🧱️base`
   dirs); the 20 remaining `✳️base` string hits are all doc-comment prose in unrelated `avi`/`ifc`
   modules, not on puzzle's path (§2b).
9. §32's `raw_display_handle` webgpu fix / `surface` missing `dag` imports / `ui-scene` workspace dep:
   all three look resolved or moot in current source (§2c).
10. Puzzle5d's 4 `PluginCloseStep::AwaitingInput` handling sites are all still present, unchanged
    (§3). Only 2 puzzle-subtree files are currently staged-uncommitted, edited ~20-25 min ago —
    consistent with ordinary single-session work, not peer churn (§4).

---

## 1. Puzzle crate direct dependencies — mtimes and churn

`semio-s-plugin-puzzle` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`) direct `[dependencies]`:

| crate | path | newest `.rs` mtime | files <30min | files <120min | files <15min (hot?) |
|---|---|---|---|---|---|
| `semio-s-artifact-puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` | 23:00 (`🔺️diff/📝️text/🦀️.rs`) | 1 | 60 | 0 — cold |
| `semio-s-artifact-puzzle-3d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` | 23:05 (`editor/tests/unit/🦀️.rs`) | 2 | 13 | 0 — cold |
| `semio-s-artifact-puzzle-5d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d` | 23:05 (`viewer/🦀️.rs`) | 2 | 12 | 0 — cold |
| `semio-framework-dispatch-macros` | `🧰️framework/🔨️modules/🔀️dispatch` | 21:35 | 0 | 0 | 0 — cold |
| `semio-framework-async-macros` | `🧰️framework/🔨️modules/⏳️async/✨️macros` | 22:11 | 0 | 2 | 0 — cold |
| `semio-framework-os-kernel` | `🧰️framework/🛍️products/💻️os` (own module tree only — see below) | 23:38 (`📡️spr` batch) | 18 | 118 | **18 — hot, but test-fixtures only** |
| `semio-framework-plugin` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin` | 23:40 (`📡️backbone/…/unit-standalone/🦀️.rs`) | ~50 | 120 | **9 — HOT, includes crate root** |

`semio-framework-os-kernel`'s `[lib] path` is `🦀️.rs` under
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, but its `#[path]` attributes pull its *actual*
module tree from 13 sibling directories under `🧰️framework/🛍️products/💻️os/🔨️modules/{⚙️engine,
🌿️vcs, 🎒️pack, 🏪️store, 💡️inference, 📇️directory, 📡️spr, 🗣️dsl, 🧩️extension, 🧬️semio, 🪪️identity}`
plus two under `🧰️framework/🔨️modules/{🚪️io, 🧬️schema/🧩️composition}` — **not** the sibling
`🔌️plugin`/`🌊️flow`/`♾️infinite` directories, which are separate crates. The table above scopes to
that real tree (verified via `grep -o '#\[path = "[^"]*"\]' 🦀️.rs`).

**Hot detail — `semio-framework-plugin`** (direct dep, edited 6-10 min before this audit, current
time 23:50):

```
23:30:25/26  9 test-fixture files (batch, single tool pass)
23:33:10     📇️registry/🤖️generated/{hosts.rs, artifacts.rs}   ← generated registry, not hand-edited
23:38:51     ⚛️reactor/🦀️.rs
23:38:51     🦀️.rs                                              ← the crate root itself
23:40:31     📡️backbone/🔗️binding/…/unit-standalone/🦀️.rs
23:40:56     🏗️builder/…/schema-stamping/🦀️.rs, 2 more test files
```

The crate-root edit at 23:38:51 is squarely inside the `pub trait ArtifactViewer` definition
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23848` onward) whose `handle(...)` method
signature (line ~23959, `fn handle(command, doc, cfg, interaction, view_state: Option<&ViewModel>,
engines)`) is the trait puzzle5d's viewer implements against. This matches the COMPOSABLE-STDIO ticket's
own note (`📓️integration-review.md`, quoted in §6) about moving code between `ArtifactStore` and
`SpaceHost` in this same file region — i.e. this crate root is under **active, in-progress editing**,
not settled.

**Hot detail — `semio-framework-os-kernel`'s `📡️spr`:** 17 files, all under
`📡️spr/{🎮️command,🧪️testkit}/🧪️tests/…`, **all with the identical timestamp `23:38:31`** — a
single mechanical batch write (script or codemod), not incremental hand-editing. Per the discriminator
this ticket's own prior findings established ("check mtimes: identical-timestamp batch ⇒ landed-and-
possibly-abandoned; still-changing ⇒ poll"), this reads as a completed batch rather than an
in-progress edit — and it is confined to `#[cfg(test)]` fixture directories, which a plain
`cargo check -p semio-s-plugin-puzzle` (no `--tests`) does not compile.

## 2. Findings-2026-09-05 §45-§46 blockers — re-checked against current source

### 2a. `semio-framework-os-flow` artifact extraction (§46)

Crate root: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs` (mtime
**20:47:27**, i.e. ~3h before this audit — cold/stable).

- Line 29: `use semio_framework_artifact_flow_flow::{artifact, retained};` — **present now** (the
  finding said this import was missing / `mod artifact` was absent). Because this is a plain `use` at
  the crate root, and crate-root items are visible to every descendant module regardless of `pub`,
  `bridge` and `host`'s `use crate::artifact::*;` (`🌉️bridge/🦀️.rs:9`, mtime 20:47:27; `🖥️host/🦀️.rs:20`,
  mtime 21:15:13) **do resolve** against it.
- `vcs` collision: current crate root declares its **own** `pub mod vcs;` (`#[path =
  "../../🌿️vcs/🦀️.rs"]`, line ~63) and does **not** re-import `vcs` from
  `semio_framework_artifact_flow_flow` — exactly the "deliberately not vcs" resolution the finding
  anticipated. No collision in current source.
- `protocol::value::ordered`: resolves. `semio-framework-replication`'s `[lib] name = "protocol"`
  (`🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml:18`); its crate root
  (`…/🦀️rust/🦀️.rs:34`) declares `#[path = "../../../🌱️value/🦀️.rs"] pub mod value;`, and
  `🧰️framework/🔨️modules/🌱️value/🦀️.rs:11` declares `pub mod ordered;`. `Grant`, `UpdateCursor`,
  `OrderedMap`, `OrderedSet` all exist in `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` (lines
  183, 301, 76, line 10 re-export respectively). `semio-framework-os-flow`'s own
  `Cargo.toml` depends on `semio-framework-replication = { workspace = true }`.
- The extracted crate `semio-framework-artifact-flow-flow`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs`, mtime **21:15:12**)
  now declares `pub mod artifact;` and `pub mod vcs;` itself — the "source half" the finding said was
  missing has landed.
- Corroboration from the peer ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/📓️framework-flow-package.md`
  (mtime 20:27) states *"The final native runtime check passed all 37 Flow artifact tests."*

**Conclusion:** the §46 blocker is resolved in current source and has been stable (cold, ~3h
untouched) since ~21:15. Not the current risk.

### 2b. stdio `✳️base` → `🧱️base` rename

```
0   ✳️base directories remaining under ✏️s/🔌️plugins/🗄️stdio
12  🧱️base directories now present
20  .rs files still containing the string "✳️base"
```

All 20 hits are doc-comment prose (`//!`, `///`) inside `📼️avi` and `🏗️ifc` standard/subset modules
(e.g. `…/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🔮️oracle/🦀️.rs:3`), referring to a sibling
subset named literally `✳️base` (a taxonomy label, not the old rename target) or stale prose — none is
an import path or code reference, and none of these modules are on the puzzle dependency path anyway.
**Not a puzzle blocker.**

### 2c. webgpu / surface / ui-scene (§32)

- `raw_display_handle` / `RawDisplayHandle` / `DisplayHandle` / `HasDisplayHandle`: **zero matches**
  anywhere under `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu` (the
  `semio-framework-ui-backend-webgpu` crate). The code path the finding described no longer exists in
  this form — either already fixed and the workaround since removed, or refactored away. No trace of
  the described bug remains to re-verify against.
- `semio-framework-surface` (`🧰️framework/🔨️modules/🗺️surface`) missing `dag` imports: **present and
  used extensively**. `🕸️node-graph/🦀️.rs:22`: `pub use infinite_canvas::board::ports::directed_dag
  as dag;`, then `dag::` used at lines 24, 132, 167-168, 377, 543-570, 638, 644 and in its own unit
  tests (`🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs:385-391`). Not missing.
- `semio-framework-ui-scene` in `workspace.dependencies`: **present**, root `Cargo.toml:416`:
  `semio-framework-ui-scene = { path = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust" }`.

All three §32 items look resolved or moot.

## 3. Puzzle5d `PluginCloseStep::AwaitingInput` handling

```
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1891
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1971
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2251
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3503
```

All 4 sites still handle it identically: `Ok(PluginCloseStep::AwaitingInput { .. } |
PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked`.
Also present (not one of the "4" but consistent): `🌉️wasm/🦀️.rs:125` and 3 occurrences in
`🧪️tests/🔬️puzzle5d-retained-retirement-laws/🦀️.rs`. **Unchanged, no regression.**

## 4. Puzzle subtree git status — who's editing it right now

```
git status --short -- "✏️s/🔌️plugins/🧩️puzzle"
M  ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs        (staged; mtime 23:25:08)
M  ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs  (staged; mtime 23:25:30)
```

Both are **staged, no unstaged diff** — `git diff --stat` for the puzzle subtree is empty; only
`git diff --cached --stat` shows the 2 files (`3 ++-` / `2 --`, i.e. 2 insertions/1 deletion and 2
deletions total). Content:

- `👁️viewer/🦀️.rs`: `Viewer::view`/an inherent method gained a `_view_state: Option<&
  semio_framework_plugin::ViewModel>` parameter, and `render`'s pre-existing `view_state` param was
  renamed to `_view_state` (now unused). This lines up with `semio-framework-plugin`'s trait `handle`
  signature (`view_state: Option<&ViewModel>` — see §1's hot-detail). **Adapted ~20-25 min ago**, i.e.
  *before* the plugin crate root's most recent edit (23:38:51) — so it matched the trait shape as of
  23:25, but the trait file has moved since; not re-verified against the very latest edit.
  Recommend re-running `cargo check -p semio-s-plugin-puzzle` once `semio-framework-plugin` goes cold.
- `editor/🧪️tests/🔬️unit/🦀️.rs`: two `dispatch(...)` calls to `setLocale`/`setTerminology` were
  removed from a German-terminology test (a de-scoping/simplification of that test, unrelated to any
  dependency churn).

Both mtimes (23:25:08, 23:25:30) predate the current audit by ~25 min and are close together —
consistent with ordinary single-session editing, not multi-agent churn. No other puzzle-subtree files
are modified.

## 5. Existing cargo logs found on disk (>6h stale ones noted, freshest highlighted)

Two useful scratchpad directories under
`/private/tmp/claude-501/-Users-ueli-Documents-semio/*/scratchpad/` (read-only, not this session's
own):

**`bf26a951-.../scratchpad/check-artifact3d{,-b,-c}.txt`** (mtimes 19:45 / 19:53 / 19:58 — **~4h
stale**, from an earlier run this evening, well before tonight's fixes):

- `check-artifact3d-b.txt`: `error: could not compile semio-s-artifact-puzzle-3d (lib) due to 2
  previous errors` — `E0433: cannot find replace_attraction_semio_framework_geometry in super` (2
  sites: `🧬️mutations/🦀️.rs:140` and `🧮replace-attraction-geometry/↩️inverse/🦀️.rs:10`).
  **Re-checked against current source: FIXED.** Current
  `…/🧬️mutations/🦀️.rs:140` reads `pub use super::replace_attraction_geometry::mutation::
  {replace_attraction_geometry, ReplaceAttractionGeometry};` (correct name, no `_semio_framework_geometry`
  suffix), and the inverse file's call site at line 10 matches. This was a real puzzle-3d-side bug at
  19:53 and is gone now.
- `check-artifact3d-c.txt`: `error: could not compile semio-framework-os-kernel (lib) due to 5
  previous errors` — `SpaceHost<M>` missing `ensure_durable_group_idle`/`envelope`/
  `replace_backbone_retained`/`pump`/`bump` (`🧰️framework/…/🏪️store/🦀️.rs:18925-18929` in that
  build's line numbering). **Re-checked against current source: FIXED.** All four methods now exist
  under `impl<P, Mutation> ArtifactStore<P, Mutation>` (`🏪️store/🦀️.rs:13908` onward, methods at
  14075, 14577, 16697/16701, 16800) — matching the COMPOSABLE-STDIO ticket's own
  `📓️integration-review.md` note: *"the new attach_hot_backbone body belonged to ArtifactStore but had
  been inserted into SpaceHost; it was moved unchanged next to attach_backbone."*

**`bf26a951-.../scratchpad/build-v9.txt`** (mtime 23:19, ~30 min old) and **`cfg-1.txt`** (mtime
23:26, ~24 min old) — freshest logs found, but from a **different** target (`semio-framework-os-config`,
90 errors, mostly `MutationLeaf` trait-bound gaps unrelated to puzzle) and end with `trunk serve failed
for wgpu renderer` (a dev-server harness failure, not a library compile result for puzzle). Not
puzzle-informative.

No log anywhere on disk (`target/`, `target-gen3d-opt/`, ticket `🗑️generated/` dirs, or any session
scratchpad) shows a **clean, current, full compile result for `semio-s-plugin-puzzle` itself** — the
freshest puzzle-specific signal is the 19:53/19:58 logs above, both ~4h stale and both since fixed.

## 6. Live cargo/rustc processes (read-only `ps`, nothing touched)

**36 cargo/rustc processes running** at audit time, spanning at least 4 independent ticket sessions
(distinguished by `CARGO_TARGET_DIR`):

| PID (top) | etime | command (essentials) | owning ticket (by `CARGO_TARGET_DIR`/cwd evidence) |
|---|---|---|---|
| 59711 | 06:08 | `cargo check --all-targets --keep-going -j2` naming **every** `-p semio-s-plugin-puzzle -p semio-s-artifact-puzzle-{2d,3d,5d} -p semio-framework-artifact-flow-flow -p semio-framework-os-kernel -p semio-framework-os-renderer-wgpu` plus all stdio artifacts and ~100 more `-p` flags | `…/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target` (older, unrelated ticket, still running its own giant sweep) |
| 52649/52651 | 09:44 | `cargo check --offline -p semio-framework-os -p semio-framework-os-flow -p semio-framework-os-infinite -p semio-s-plugin-space --features os-host-full --lib --keep-going`, tee'd to `framework-host-integration-7-current-jobs2.txt` | `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` |
| 37328/37329 | 21:00 | `cargo test -p semio-s-artifact-gis-gismap -p semio-s-artifact-gis-gisterrain` | `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` |
| 41721 | 16:50 | `nx run @semio-tech/forms-forms-rs:check` | `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` |
| 36876 | 21:35 | `cargo test -p semio-framework-os-kernel --lib --features sync --no-run --message-format=json` | unclear (ppid 24175, own target dir not `COMPOSABLE-STDIO`) |
| 62518/62236 | 05:07 | `cargo build --lib --profile wasm-release --locked --message-format=json`, actively compiling `semio_framework_graph`, `semio_framework_ui`, `semio_framework_artifact_infinite_dag`, **`semio_framework_artifact_flow_flow`** | separate wasm-release build, unclear owner |
| 68368→70362/70364 | 01:56 | `cargo test --lib surface_context -- --nocapture`, actively compiling **`semio_framework_plugin`** via `sccache` at audit time (0:47 elapsed on this rustc invocation) | unclear owner, but this is the process actively recompiling puzzle's hot direct dependency |
| 44798/44776, 38166/38321, 39262+chain, 62518 children, etc. | various | forms check, trinity-jack check, norm-contract nextest, GIS default runtime, etc. | mostly `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` |

None of the processes that name `semio-s-plugin-puzzle` (only PID 59711's giant sweep does) has
reached puzzle in its compile order yet — at read time it was still compiling `stdio-binary` /
`stdio-deflate` and `wasm-bindgen-macro-support`, several steps before puzzle's own dependencies.
**No process observed during this audit produced a fresh, complete puzzle result.** Per instructions,
nothing was killed or signaled.

## Notes on scope / caveats

- "Compiles" here means `cargo check`/`cargo build` of the library target; test-only churn (§1's
  `📡️spr` batch, most of `🔌️plugin`'s 9 hot files) does not gate a plain lib build.
- The `semio-framework-plugin` risk (§1, §4) is the one open question this audit could not close:
  its trait shape looked compatible with puzzle5d's own recent adaptation as of ~23:25, but the crate
  root was edited again at 23:38:51, after this audit's own comparison was made — a live re-check
  (once that file goes cold, i.e. untouched for >15 min) is the only way to be certain.
- Machine load (36 concurrent cargo/rustc processes, `CARGO_BUILD_JOBS=2` throttling visible in
  several peer commands) matches the "load ~60" context given for this audit; no build was attempted
  here to avoid adding to that load.
