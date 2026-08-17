# Wave PEEL3 — topology SCC + the forced `📄️step` dependency landed; foundations batch NOT attempted

## Scope executed vs scope deferred

Landed the **topology SCC batch** (`🏟️arena`, `🕸️topology`, `🔺️euler`, `📜️history`, `🧱️primitives`,
`✅️validate` — 2,516 LOC) **plus `📄️step` (1,035 LOC)**, which the brief did not name but which a
real, measured dependency forced into this same batch (see below). **The "foundations" batch**
(~5,143 LOC, two further cycles) **was NOT attempted** — a deliberate scope cut after landing one
large, fully-verified batch, per the standing rule from wave PEEL's polling-loop postmortem: one
clean batch beats several attempted-and-unproven ones.

## Cycle re-verification (measured, not trusted from the brief)

Grepped every `crate::brep::X` cross-reference inside the six named files before touching anything:

```
arena       -> arena (self, define_id! macro only)
topo        -> arena, curve, euler, history, mat, primitives(test-only), surface, tolerance, vec
euler       -> arena, curve, history, tolerance, topo, vec
history     -> (none — self-contained)
primitives  -> arena, curve, error, euler, history, mat, surface, tolerance, topo, validate
validate    -> arena, error, topo, tolerance (+ euler/history/curve/mat/surface/tolerance/vec in test-only code)
```

Confirms the brief's claimed mutual recursion (`primitives→validate→topo→euler→history→topo`) —
**all six move as one unit, confirmed real, not merely asserted.**

### 🔑 A dependency the brief didn't name: `📄️step` forces itself into this batch

Grepped every *remaining* `📐️brep` subdir (the 13 that stay behind: predicates, engine,
curve-ops, vector, curve, polynomial, bezier, surface, tolerance, matrix, error, surface-ops,
bspline) for references back into the six SCC modules. **Thirteen of fourteen came back clean —
`📄️step` did not**:

```
📄️step imports (production code, not test-only):
  crate::brep::arena::{ArenaId, Curve3Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId}
  crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex}
  crate::brep::history::OpRecorder
  crate::brep::topo::Body
```

`write_step`/`read_step` build/walk a real `Body` through the checked Euler editors — not test
fixtures. Since `semio-framework-3d` may never name a stdio symbol (the binding crate-direction
law), leaving `step` behind while arena/euler/history/topo move into stdio would have broken
framework-3d's build the instant the topology SCC left. **The only two lawful options were: move
`step` in the same change, or don't move the topology SCC at all this wave.** I moved it. This is
new information this wave discovered, not something the brief anticipated — flagging it because
"a proven structural impossibility... is a blocker" per the ticket's binding ruling, and I want
the reasoning on record for why `step` (not part of either of the "foundations" batch's two named
cycles) landed early instead of waiting for that batch.

`step` is *also* a **known pre-existing duplicate**: stdio already has a separately-complete,
tested AP214 STEP↔SemioBrep walk under `✳️brep/🚪️io` (`SemioBrepToStep`/`SemioBrepFromStep` +
`artifacts::step`'s generic Part-21 tokenizer). An earlier wave (G4 job 2) found this and declined
to reconcile the two because doing so requires rewiring `⚙️engine`'s `BrepKernel` impl — out of
scope per `📌️important.md`'s explicit "BrepKernel — do NOT attempt." **I relocated `step`
verbatim, duplicate-and-all, to satisfy the crate-direction law — I did not touch the duplication
itself.** That reconciliation stays exactly where G4 left it: deferred to the (separate, named)
BrepKernel wave.

## What moved, and where

| Source (deleted) | LOC | #test | Destination | Shape |
|---|---|---|---|---|
| `📐️brep/🏟️arena` | 260 | 8 | `✳️brep/🧬️schema/📸️snapshot/🏟️arena/component.rs` | flat, verbatim (only the `define_id!` macro's `$crate::brep::arena::ArenaId` self-reference needed repointing) |
| `📐️brep/🕸️topology` | 668 | 11 | `✳️brep/🧬️schema/📸️snapshot/🕸️topology/component.rs` | flat, own regions preserved |
| `📐️brep/📜️history` | 179 | 7 | same file, nested `pub mod history { … }` | no pre-mounted facet existed for it; nested inside topology (its only real co-dependent) rather than inventing a new top-level mount |
| `📐️brep/🔺️euler` | 277 | 6 | `✳️brep/🧬️schema/🔺️diff/🔺️euler/component.rs` | **merged** alongside the already-resident imprint code (wave PEEL), per the brief's explicit instruction — Make/SplitJoin regions inserted before the existing Api/UvArrange regions; existing file had 4 tests, combined file now has 10 |
| `📐️brep/🧱️primitives` | 889 | 11 | `✳️brep/🧬️schema/🔺️diff/🧱️primitives/component.rs` | flat, replaced a 4-line placeholder stub |
| `📐️brep/✅️validate` | 243 | 5 | `✳️brep/🧬️schema/💡️inferences/✅validation-report/component.rs` | **merged** alongside the already-resident `BrepValidationReport` inference — kept as a plain `pub fn validate_body`, not wired as its own `InferredField` (it checks the *ephemeral* `Body` mid-construction, a genuinely different, complementary check from the existing whole-snapshot referential-integrity inference); existing file had 5 tests, combined file now has 10 |
| `📐️brep/📄️step` | 1,035 | 3 | `✳️brep/🧬️schema/⚙️engine/📄️step/component.rs` | flat, new subdirectory nested inside `⚙️engine` (mirroring the pre-existing `📦️mesh-io` sibling), mounted via a **local** `#[path]`+`mod step;` line inside `⚙️engine/component.rs` itself — **not** a new entry in stdio's 9,400-line `📦️glue.rs` |

**Total moved: 3,551 LOC, 51 tests.**

Every `crate::brep::X` reference in the moved files was mechanically redirected: the six SCC names
(+`step`) to their new intra-crate `crate::artifacts::semio::standards::v1::subsets::brep::schema::…`
paths, everything else (`curve`, `mat`, `surface`, `tolerance`, `vec`, `error`, `bspline`) left as
`semio_framework_3d::brep::…` forward edges (foundations, not moving this wave). Done via a
Python regex substitution script (not hand-retyping), then `grep -c 'crate::brep::'` verified 0
residual on every transformed file before assembly.

## Consumer repoint (11 files, all inside stdio, all mechanical)

`🔺️diff/🎨️blend`, `🔺️diff/↔️offset`, `🔺️diff/🔀️boolean`, `🔺️diff/➡️sweep`, `🔺️diff/🧵️sew`,
`⚙️engine` (+ its `📦️mesh-io` sibling), `💡️inferences/🌳bounding-volume`,
`💡️inferences/📏mass-properties`, `💡️inferences/🏷classification`, `💡️inferences/🧩tessellation` —
every `semio_framework_3d::brep::(arena|topo|euler|history|primitives|validate)` forward-edge
import (including inline fully-qualified references buried in function bodies, not just `use`
lines — caught by the same regex pass) repointed to the new intra-crate paths. `⚙️engine`'s own
self-referential `semio_framework_3d::brep::step::{read_step, write_step}` was simplified to the
local `use step::{read_step, write_step};` form already established by its `mesh_io` sibling.

Verified **zero** residual `semio_framework_3d::brep::(arena|topo|euler|history|primitives|validate)`
references anywhere in the repo (one repo-wide grep, clean) after the edit.

## Framework-3d glue.rs surgery

Removed 7 `#[path]` mount lines (`arena`, `history`, `topo`, `euler`, `validate`, `primitives`,
`step`) from `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` in the same change as the
source deletion. Checked all 7 deleted directories for non-`.rs` files first (the `🔺️mesh`
DWG-codec near-miss precedent) — **none found**, `rm -rf` was safe. Kept: `error`, `vec`, `mat`,
`tolerance`, `predicates`, `poly`, `bezier`, `bspline`, `curve`, `curve_ops`, `surface`,
`surface_ops`, `engine` (the foundations batch + framework-3d's own 67-LOC leaf, both untouched).

`📐️brep` now has **13 subdirs / 4,175 LOC** (was 20 subdirs / 7,726 LOC at wave start).
`✳️brep` (stdio) is now 17,961 LOC.

## No code lost or duplicated — symbol proof, both directions

Distinctive symbol, grepped across `stdio`'s tree vs framework-3d's `📐️brep` remainder:

```
pub fn make_box          : stdio=1  fw3d(remainder)=0
pub struct Body           : stdio=1  fw3d(remainder)=0
pub fn validate_body      : stdio=1  fw3d(remainder)=0
pub struct PersistentLabel: stdio=1  fw3d(remainder)=0
pub fn make_vertex        : stdio=1  fw3d(remainder)=0
pub trait ArenaId         : stdio=1  fw3d(remainder)=0
pub struct Store          : stdio=1  fw3d(remainder)=0
pub fn write_step         : stdio=1  fw3d(remainder)=0
pub fn read_step          : stdio=1  fw3d(remainder)=0
pub fn split_edge         : stdio=1  fw3d(remainder)=0
pub struct OpRecorder     : stdio=1  fw3d(remainder)=0
```

Every symbol present exactly once in the new home, zero times in the old location. No duplication
window was left open (unlike wave M3a's math/cas move, which deliberately opened one for a
different reason) — this was a straight cut-and-repoint, done in one change.

## Test arithmetic — exact, both directions checked

```
semio-framework-3d --lib:  233 passed / 0 failed  →  182 passed / 0 failed   (−51, 0 failed both sides)
```

`233 − 182 = 51`, matching **exactly** the per-file `#[test]` count I recorded before deletion:
`arena 8 + topology 11 + euler 6 + history 7 + primitives 11 + validate 5 + step 3 = 51`. Zero
tests lost, zero silently dropped.

```
semio-s-plugin-stdio --lib: 3259 passed / 5 failed / 4 ignored
```

The **5 failing tests are byte-identical by name** to `scratch-w0-baseline-failures-sorted.txt`'s
stdio section (`binary::extent`, `dwg::fixture_honesty_law`, `dxf::bounds`,
`ifc::fixture_honesty_law`, `zip::entries`) — all pre-existing, all unrelated to brep, zero new
failures introduced. I could **not** do a clean "baseline + 51 forward" pairwise diff on the stdio
side — I did not capture stdio's exact test count immediately before this wave's edits (avoided a
second full `cargo test` run per the "verify ONCE, no polling" rule), and per PEEL2's own report
this same number moves under other concurrent waves independent of this one, so an exact delta
claim would not be trustworthy even if I had it. What I *did* verify cleanly: the failure set is
unchanged, and none of the 51 moved tests appear in it (all 51 pass in their new home) — spot
confirmed by `artifacts::semio::…::brep::schema::engine::tests::sphere_torus_cut_produces_preview_mesh`
passing, which exercises the newly-nested `step` module's neighbor code in the same file.

## Verification output (mandatory form, run once each)

```
touch 🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs
cargo check -p semio-framework-3d --all-targets   → 0 errors (warnings only, from an upstream crate)
cargo test  -p semio-framework-3d --lib           → 182 passed; 0 failed

touch ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs
cargo check -p semio-s-plugin-stdio --all-targets → 0 errors (698+802 warnings, all pre-existing dead-code/style, none new)
cargo test  -p semio-s-plugin-stdio --lib         → 3259 passed; 5 failed (pre-existing, see above); 4 ignored
```

`df -h /`: 121Gi free, 9% used — checked before and after the long stdio compile, not a
disk-space mirage.

## `⚙️engine` / `BrepKernel` — status update, not touched further

Same two things as PEEL2 flagged, now updated:

1. **`📐️brep/⚙️engine`** (framework-3d, 67 LOC) — still untouched. Zero references to any of the
   7 modules moved this wave (checked). Survives until the foundations batch lands.
2. **`✳️brep/🧬️schema/⚙️engine`** (stdio, now ~1,715 LOC facade + 451 LOC `📦️mesh-io` + 1,045 LOC
   `📄️step`, newly nested) — NOT dissolved. `📄️step` moved *into* it (as a plain relocation, not a
   BrepKernel rewrite) because it was the only consumer; the trait/impl collapse itself remains
   untouched and out of scope, per `📌️important.md`.

## `📦️mesh-io` — imports repointed, NOT relocated to `🚪️io`

Per the brief: "only move it if you can do so without adding a new mount to a file another session
is editing." `📦️mesh-io`'s directory itself was **not** moved this wave — only its `use
semio_framework_3d::brep::(arena|euler|history|primitives)` imports were repointed (it's a
consumer of the topology SCC, same mechanical fix as the other 10 consumer files). Relocating its
*directory* to `🚪️io/` would mean removing its `#[path]` mount from `⚙️engine/component.rs` — a
file I already heavily edited this wave (step's new mount + the consumer repoint) — and adding a
new one wherever it lands in `🚪️io/`. Given the size of what already landed this wave and the
"verify once, then stop" budget rule, I judged doing a second structural move inside the same
already-large diff added more risk than the remaining budget could safely absorb. Deferred, not
declined — the next wave can do this cleanly against a smaller diff.

## Honest remainders (what the next PEEL wave needs)

**Foundations batch (~5,143 LOC), NOT started.** Two further cycles inside it, per PEEL2's
measurement (re-verify before trusting):
- `{〰️polynomial, 🪢️bspline, ➰️curve, 🏄️surface}` mutually recursive through bspline
- `{➡️vector, 📏️tolerance, ⚖️predicates}` (`vec→tolerance→predicates→vec`)
- plus standalone `🔢️matrix`, `✂️curve-ops`, `🪡️surface-ops`, `🚨️error` → their pre-allocated
  `📸️snapshot/*` compute dirs (already confirmed to exist: `snapshot/{vector,curve,polynomial,
  surface,tolerance,error}` are pre-mounted; `matrix`, `bezier`, `bspline`, `predicates`,
  `curve-ops`, `surface-ops` are **not** pre-mounted — the next wave will need to either add new
  mounts or find/confirm a nesting precedent, same as `step` needed this wave).

**`📐️brep` is not yet empty or deletable** — 13 subdirs, 4,175 LOC remain (the foundations batch
+ the tiny `⚙️engine`), plus `🥽️mesh` (2,769 LOC, explicitly out of scope — live MESH wave).

**`📦️mesh-io`** — imports fixed, directory not relocated (see above); a clean rider on the next
wave once `⚙️engine/component.rs` isn't also being edited for something else.

**`BrepKernel`/stdio's `⚙️engine` facade** — deliberately not dissolved; the STEP duplication
between framework-3d's hand-rolled Part-21 codec (now relocated, not fixed) and stdio's own AP214
walk is still open, same as G4 left it.

**framework-3d's own `⚙️engine` (67 LOC)** — survives until foundations lands, unchanged this wave.

## Files touched this wave

**Deleted** (7 dirs, all `.rs`-only, checked first):
`🧰️framework/🔨️modules/🧊️3d/📐️brep/{🏟️arena,🕸️topology,🔺️euler,📜️history,🧱️primitives,✅️validate,📄️step}/`

**Created**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📄️step/🦀️component.rs`

**Rewritten** (destination content, previously stub placeholders or pre-existing files needing a
merge): `…/📸️snapshot/🏟️arena/component.rs`, `…/📸️snapshot/🕸️topology/component.rs` (topology +
nested history), `…/🔺️diff/🔺️euler/component.rs` (euler editors + existing imprint, merged),
`…/🔺️diff/🧱️primitives/component.rs`, `…/💡️inferences/✅validation-report/component.rs`
(validate_body + existing BrepValidationReport, merged), `…/⚙️engine/component.rs` (step mount
added + consumer repoint)

**Edited** (import repoint only): `…/🔺️diff/🎨️blend/component.rs`, `…/🔺️diff/↔️offset/component.rs`,
`…/🔺️diff/🔀️boolean/component.rs`, `…/🔺️diff/➡️sweep/component.rs`, `…/🔺️diff/🧵️sew/component.rs`,
`…/⚙️engine/📦️mesh-io/component.rs`, `…/💡️inferences/🌳bounding-volume/component.rs`,
`…/💡️inferences/📏mass-properties/component.rs`, `…/💡️inferences/🏷classification/component.rs`,
`…/💡️inferences/🧩tessellation/component.rs`

**Edited** (mount removal): `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs`

Scratch files: all intermediate transform copies and the substitution scripts live in this
session's scratchpad, not the ticket folder or the repo — none are part of the deliverable.

## sharedFileRequests

None — all files touched this wave are inside `📐️brep`/`✳️brep`, both explicitly this ticket's
territory per the ownership table.

## Concurrent-churn observations

None hit. Both verification runs came back clean on the first try; no foreign errors encountered
in either crate.

## Honest pass/fail

**Pass.** Both batches (topology SCC + the forced `step` addition) landed atomically — code moved,
consumers repointed, mounts removed, all in one change per destination; both crates verified once
each with 0 compile errors and 0 new test failures. Foundations batch honestly not started —
flagged with the specific missing pre-mounts the next wave will hit.
