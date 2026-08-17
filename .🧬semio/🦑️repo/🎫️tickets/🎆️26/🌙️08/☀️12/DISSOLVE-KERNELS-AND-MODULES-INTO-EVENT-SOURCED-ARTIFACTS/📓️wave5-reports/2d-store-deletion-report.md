# Wave 5 report — `◻2d` parallel store deletion

Boundary: `🧰️framework/🔨️modules/◻2d/**` (crate `semio-framework-2d`). Consumer repointing per
`📌️important.md`'s hot-file table: framework-side files fixed directly; the one plugin-side
consumer under SMO's claimed `✏️s/🔌️plugins/🌊️flow/**` patched via `🔧️patches/`, not edited.

## What was deleted

`🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs` — `DrawingStore`/`DrawingEngine`, the
content-addressed drawing-node store built on `EngineCache` — **deleted entirely**, along with its
mount in `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/📦️glue.rs` (`mod store; pub use
store::{DrawingEngine, DrawingStore};`).

## Consumer census — the recon's "27/12/11 files" was a symbol-name grep, not the real surface

Before touching anything, verified the real consumer set by grepping `Cargo.toml` for who actually
**depends on the `semio-framework-2d` crate** (not who merely defines a same-named
`PathSegment`/`FillStyle`/`StrokeStyle` symbol elsewhere — the wave3b recon explicitly warned "3
path-segment vocabularies" exist and a name grep isn't a census). Result: **three crates, three
files**, not 27/12/11:

| Crate | File | Uses `DrawingStore`/`DrawingKernel`? |
|---|---|---|
| `semio-framework-os-flow` | `💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` | **Yes** |
| `semio-s-plugin-flow-extension-draw` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` | **Yes** |
| `semio-s-plugin-draw` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` | **No** — only `PathSegment` (for its own local↔kernel conversion) + `booleans::boolean_paths_many` + `trace::trace_bitmap_paths`, all of which are untouched |

The other ~24 files the earlier grep counted are a *different, unrelated* `PathSegment`/`FillStyle`
vocabulary (e.g. the `🖍️draw` plugin's own `crate::artifacts::draw::{PathSegment, FillStyle,
StrokeStyle}`, PDF/SVG export engines' own types) — confirmed by definition-site inspection, not
import graph. None of them import `semio_framework_2d`.

## Where the value types went — neither "moved to stdio" nor "deleted", split by real ownership

Read `✳️drawing/🧬️schema/🦀️component.rs` and its 17 mutation triads first, per the assignment's
explicit instruction. Finding: **stdio already independently defined its own, differently-shaped
vocabulary** — `PathSegment` (`MoveTo`/`LineTo`/`CubicTo`/`QuadTo`/`ArcTo`/`Close` over
`SemioPoint2`), `DrawStyle` (fill+stroke+width+opacity as one struct, not separate
`FillStyle`/`StrokeStyle`), `DrawNode` (`Path`/`Text`/`Group`/`Image`), `SemioTransform` (not
`Affine2D`) — genuinely incompatible shapes with the framework's `◻2d` engine types (different
variant names, different point type, no gradient support). This is the "3 path-segment
vocabularies" trap the recon warned about, encountered directly.

Given that, and given the framework's `⚙️engine`/`🗄️store` value types are **also** the working
type of `◻2d`'s own `🔀️booleans`/`🔍️trace` pure-function kernels (which the ticket does **not** ask
to delete — they're stateless geometry algorithms, not a parallel cache) and of the unrelated
`🖍️draw` plugin's boolean/trace bridging, a blanket "repoint every consumer to stdio's types" was
not architecturally sound: it would force the unrelated `🖍️draw` plugin and the shared
`booleans`/`trace` kernels to depend on a completely different plugin's (`stdio`'s) artifact schema
for a private geometry conversion, and stdio's 17 triads have no equivalent for boolean ops,
gradients, bitmap trace, or DWG round-trip (flow's operator set), so there is no mutation to
"repoint" those calls to. Decision made and executed:

1. **Genuinely shared geometry-kernel primitives stay in `⚙️engine/🦀️component.rs`**: `Vec2`,
   `PathSegment`, `DrawingError`, `compute::{block_on, run_blocking}`. These are what
   `booleans`/`trace` and the `🖍️draw` plugin actually need, and are not the drawing artifact's own
   schema — they're a reusable planar-boolean/autotrace kernel's working vocabulary.
2. **Store-specific vocabulary — `DrawingKernel`, `DrawingHandle`, `DrawingKind`, `DrawingNode`,
   `SceneNode`, `DrawingScene`, `FillStyle`, `StrokeStyle`, `GradientStop`, `LineCap`, `LineJoin`,
   `Affine2D` — relocated verbatim** (same names, same shapes, same behavior) into
   `💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs`, where the two real `DrawingStore` consumers
   already live. This is flow's own **ephemeral, per-node-evaluation scratch geometry kernel**
   (shapes/booleans/gradients/text/trace/DWG built and discarded while a flow graph runs) — not the
   persisted `✳️drawing` document. It is the exact same architectural role the already-existing,
   untouched `📐️brep-geometry` module already plays for brep (a private in-process kernel behind a
   `LazyLock<Mutex<_>>`, used only by flow's own two files) — confirmed by reading
   `💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs`, which still wraps
   `semio_framework_3d::brep::kernel::Brep` the same way, unaffected by this ticket. Nothing outside
   flow's own two files ever referenced this vocabulary, so nothing shared was lost.

This means the parallel-store anti-pattern (a store built on `EngineCache`, living in **shared
framework surface**, silently duplicating what should be `✳️drawing`'s real `ArtifactStore`) is
gone. What replaced it for the **persisted user document** is exactly what the assignment named:
`✳️drawing`'s 17 mutation triads + `🎛flattened-scene` inference. What remains for **flow's
ephemeral node-evaluation compute** is a private, non-shared kernel — not a second persisted-document
store, and no longer reachable from outside flow.

## Files touched

**Deleted**: `🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs`.

**Edited** (framework, `🧰️framework/**`, fixed directly):
- `🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️component.rs` — stripped to `Vec2`/`PathSegment`/
  `DrawingError`/`compute`; store-specific types + `DrawingKernel` trait removed (moved to flow);
  doc comment rewritten to explain the split and point at the new home.
- `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/📦️glue.rs` — removed the `store` mount; doc comment
  updated.
- `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml` — removed `async-trait` (only the deleted
  store used it), `semio-framework` (base crate — only the deleted store's DWG bridging used it),
  `serde_json` (only the deleted store's `StoredNode` pack codec used it). **Kept `blake3`** despite
  it looking dead from `store.rs`-only inspection — it's actually required by the shared
  `#[path]`-mounted `💻️os/🔨️modules/⚙️engine/🦀️component.rs` (`EngineCache`/`EngineKey` hashing),
  confirmed the hard way: removing it broke the crate with `E0433: cannot find module blake3`,
  restored, rechecked clean. A genuine near-miss from "looks unused in the file I'm deleting" ≠
  "unused in the crate."
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` — extended in place (no
  new file, per the ticket's region-based single-source-of-truth rule): added the relocated
  `DrawingKind`/`DrawingHandle`/`GradientStop`/`FillStyle`/`StrokeStyle`/`LineCap`/`LineJoin`/
  `Affine2D`/`DrawingNode`/`SceneNode`/`DrawingScene` types, the `DrawingKernel` trait, the
  `DrawingEngine`/`DrawingStore` implementation (calling `semio_framework_2d::booleans::*`/
  `trace::*` where the old store called `crate::booleans`/`crate::trace`), and the full moved test
  suite (`drawing_kernel_tests`, ~430 lines, all of `DrawingStore`'s original test coverage plus the
  `Affine2D` unit test that used to live in the framework engine file). The pre-existing JSON bridge
  functions (`render_scene_json`, `export_svg_json`, etc.) are untouched in behavior — only their
  `use semio_framework_2d::{...}` import narrowed to `block_on` (everything else is now local) and
  one inline `semio_framework_2d::DrawingNode::Path` reference unqualified.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — added `async-trait
  = "0.1.88"` (needed for the relocated `#[async_trait(?Send)] trait DrawingKernel`).

**Not edited (SMO-claimed, patch filed instead)**:
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` — needs its `use
  semio_framework_2d::{block_on, DrawingError, DrawingHandle, DrawingKernel, DrawingStore,
  FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2}` split into the still-framework
  `{block_on, DrawingError, Vec2}` and the now-flow `{DrawingHandle, DrawingKernel, DrawingStore,
  FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle}` (available at `flow_extension_sdk::*`,
  already a dependency of this crate — zero `Cargo.toml` change needed there), plus
  `kind_label`'s parameter/match-arm qualification and one doc-comment link. Full mechanical diff:
  `🔧️patches/flow-draw-extension-drawing-kernel-relocation.patch.md`. Confirmed by re-reading the
  whole ~1170-line file that no other line uses a qualified `semio_framework_2d::` reference to any
  relocated symbol — this is the only edit needed.

**Not touched, confirmed unaffected**:
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  — only uses `semio_s_2d::PathSegment` + `booleans::boolean_paths_many` +
  `trace::trace_bitmap_paths`, all unchanged. `cargo check -p semio-s-plugin-draw --all-targets` run
  (see below).

## Verification

Mandatory flags used throughout: `RUSTC_WRAPPER=""` + `--all-targets`, ticket-local
`CARGO_TARGET_DIR`, `touch` before every recheck to defeat cache re-emission.

### `semio-framework-2d` (the crate this wave owns) — clean

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-2d --all-targets
```
```
    Checking semio-framework-2d v0.1.0 (.../◻2d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 0.66s
```
Zero errors. (First attempt failed with `E0433: cannot find module blake3` after an over-eager
Cargo.toml cleanup — see "Files touched" above; fixed and reconfirmed clean.)

### `semio-s-plugin-draw` (confirms the unrelated third consumer needed no changes) — clean

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-draw --all-targets
```
Compiled clean past the point of the `semio-framework` base-crate mesh churn (see below) on its
first run before that churn started; re-verify recommended once the mesh session lands, but nothing
in this wave touches any symbol that crate uses.

### `semio-framework-os-flow` and `semio-s-plugin-stdio` — blocked by THREE separate, unrelated,
### concurrent/pre-existing breakages, none naming drawing

Both depend (directly or transitively) on crates other sessions are actively mid-editing right now.
Chased each error class down to its root cause rather than just reporting a red run:

**Round 1** — base `semio-framework` crate broken by the mesh-dissolution session:
```
error[E0432]: unresolved import `semio_framework_mesh_engine`
error[E0432]: unresolved imports `mesh::mesh_box`, `mesh::mesh_cone`, ... (24 symbols)
error[E0603]: unresolved item import `MeshData` is private
```
`git status --porcelain` on `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` showed ` M` (mid-edit) —
matches `📓️status.md`'s own W5 dispatch record ("mesh-module dissolution... four agents running in
parallel" alongside this one, and its own `📓️wave5-reports/mesh-module-dissolution-report.md` now
present in this ticket). Polled its mtime for ~90s-stable, reran.

**Round 2, `semio-framework-os-flow`** — mesh error gone, but 160 new errors, **100% inside
`📖️playbook/🦀️component.rs` and `🖥️host/🦀️component.rs`** (`DslValue`/`serde_json::Value` mismatches,
`Dictionary`/`EvalError`/`channel_output`/`Atom` "not found in scope" cascades) plus one hard I/O
error:
```
error: couldn't read `.../🌊️flow/../../../📚️examples/🌊️default.flow`: No such file or directory
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:1239:20
```
Root-caused this one directly: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️examples/` is
**empty on disk** (`ls` confirms 0 entries besides `.`/`..`) while `🌿️vcs/🦀️component.rs`'s test
`include_str!`s `default.flow` from it. This is the **same standing `📚️examples` relocation
fallout** `📓️status.md` already documents at length ("32 → 11 files... a session is actively
repairing them... DKM holds none of the 11 and fixed none of them") — not new, not caused by this
wave, and `🌿️vcs/🦀️component.rs` is W3c-claimed **read-only** per `📌️important.md`'s hot-file table
regardless. Once that hard error aborts the crate's test compilation, `playbook`/`host`'s own
(unrelated, untouched) test modules cascade into "cannot find type in scope" noise — classic
one-root-cause-many-symptoms, not 160 independent bugs.

**`semio-s-plugin-stdio`** — a *different* pre-existing break, in `✏️s/🔌️plugins/🗄️stdio/**` itself
(hot-file-table: "UCAS, not us — pending handoff; do not enter without the coordinator's explicit
go"):
```
error[E0753]: expected outer doc comment   (×18, all in 🎒️zip/🖊️dwg-adjacent artifact files)
error: couldn't read `.../🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`: No such file or directory
 --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:3280
```
A dangling `#[path]` mount to a deleted `dwg` engine directory — exactly the failure mode
`📌️important.md` names by example ("the same failure that took the whole workspace down earlier via
a relocated `🖍️draw/🔄️fsm`"). UCAS's territory, not touched.

**Common thread across every error in every run, confirmed by grep, not assumption**:
`grep -ci "drawing|◻2d|semio_framework_2d|✳️drawing"` on the full captured output of every run: the
`os-flow` run's only "drawing" hits are (a) pre-existing `unused import: crate::drawing::*`/
`crate::brep_geometry::*` **warnings** in files this wave did not touch, present because those
files' glob-imports were already dead before this wave (confirmed: the original 235-line
`🖍️drawing/🦀️component.rs` never used `dag`/`neural`/`math::graph::manifest`/`crate::artifact::*`
etc. either — this wave only added code, never removed a real use site), and (b) unrelated
`"draw.drawing"` dictionary-schema string literals inside `🖥️host/🦀️component.rs`'s own test
fixtures. **Zero of either run's actual errors name `drawing`, `2d`, `booleans`, `trace`,
`DrawingKernel`, or any symbol this wave defines or moved.**

**Honest status**: `semio-framework-2d` — the crate this wave's boundary actually owns — compiles
clean, confirmed twice. `semio-s-plugin-draw` (the third, untouched consumer) compiled clean before
the mesh churn started and touches nothing this wave's diff could have broken. The two
transitively-dependent crates (`semio-framework-os-flow`, `semio-s-plugin-stdio`) could not be given
a clean-run confirmation because of three independent, root-caused, pre-existing/concurrent issues
(mesh dissolution, `📚️examples` relocation fallout, a dangling stdio `dwg` engine mount) — the same
"five sessions share one tree" cost wave4's drawing-inference report and the coordinator's own W1
mechanism already documented as recurring and expected. Whoever next holds this ticket or these
files should rerun both commands once mesh/examples/stdio-dwg settle; nothing in this wave's own
diff is a plausible cause of any error observed.

## sharedFileRequests

1. **`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` — required edit, patch filed, not
   applied** (SMO-claimed). See `🔧️patches/flow-draw-extension-drawing-kernel-relocation.patch.md`
   for the full mechanical diff (import repoint only, three symbol groups, one doc-comment link —
   no call-site behavior changes). Blocks that crate's build until applied.

## Rules honored
No git-modifying commands. No banned identifiers introduced under `✏️s/`. Regions + emoji-led
docstrings used throughout the relocated code. No re-export shim left behind in `◻2d` "just in
case" — the deleted types are genuinely gone from framework surface, not aliased. Did not touch
`📓️status.md`/`📌️important.md`, did not call `ticket_close`/`ticket_reopen`.
