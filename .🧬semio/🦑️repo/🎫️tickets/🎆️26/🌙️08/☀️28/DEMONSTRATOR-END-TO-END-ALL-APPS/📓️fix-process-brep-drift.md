# Fix: `semio-s-plugin-process` brep struct-literal drift (15× E0063)

## Scope

One file: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs`

A peer added four fields to the b-rep snapshot types in `semio-s-plugin-stdio`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🦀️.rs`).
`process3d`'s parametric box/cylinder/sphere brep synthesizers (and the empty-topology
placeholder for imported meshes/solids) construct these structs by full literal and went
stale, producing 15× `error[E0063]` (missing field).

## What each new field means (from the stdio source's own doc comments)

- **`BrepVertex::tol: f64`** (line ~170) — native `Vertex::tol`, the containment-ball radius
  in model units. `#[value(default)]`; `0.0` (Rust default) means "unspecified" —
  `Body::from_snapshot` treats `<= 0.0` as "use the kernel default", not a literal zero
  tolerance.
- **`BrepEdge::tol: f64`** (line ~184) — native `Edge::tol`, tube radius in model units. Same
  "`<= 0.0` = unspecified" convention.
- **`BrepFace::tol: f64`** (line ~243) — native `Face::tol`, shell thickness in model units.
  Same convention.
- **`SemioBrepSnapshot::coedges: Vec<BrepCoedge>`** (line ~313) — first-class coedges (one
  face's directed use of one edge within one loop, matching the native kernel's `Coedge`).
  Empty is valid for any snapshot produced before this field existed: `Body::from_snapshot`
  falls back to reconstructing coedges from `BrepLoop.edges` when this collection is empty.
- **`SemioBrepSnapshot::next_label: u64`** (line ~322) — the native `Body::labels`/
  `LabelSource` high-water mark, carried across `to_snapshot`/`from_snapshot` round trips so
  independent mutations never mint colliding persistent labels. `0` means "no persistent-label
  history yet, mint fresh labels for everything" — safe for any snapshot with no prior
  mutation history.

## Value threaded in, and the evidence for it

- **`tol: 0.0`** on every synthesized `BrepVertex`/`BrepEdge`/`BrepFace` literal.
  `process3d`'s box/cylinder/sphere functions synthesize brand-new parametric geometry with no
  source tolerance in scope to thread through (unlike, say, a kernel session that already
  tracks a working tolerance) — `0.0` is exactly the documented "unspecified, use kernel
  default" sentinel, not an invented default. Confirmed against already-compiling code using
  the identical literal: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/🦀️.rs:178-181`
  builds `BrepVertex { .., tol: 0.0 }`, `BrepEdge { .., tol: 0.0 }`, `BrepFace { .., tol: 0.0 }`
  test fixtures the same way.
- **`coedges: Vec::new()`, `next_label: 0`** on every `SemioBrepSnapshot` literal. Matches the
  exact pattern used by stdio's own STEP importer — the closest analogue to `process3d`'s
  synthesizers, since both mint a brep from scratch with no prior mutation/label history:
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs:411`:
  `SemioBrepSnapshot { .., coedges: Vec::new(), next_label: 0 }`.

## Sites fixed (all in `process3d/🦀️.rs`)

- `empty_brep_snapshot()` — 1× `SemioBrepSnapshot`
- `brep_snapshot_for_box()` — vertex map closure (`BrepVertex`), `line_edge` closure
  (`BrepEdge`), `plane_face` closure (`BrepFace`), final `SemioBrepSnapshot`
- `brep_snapshot_for_cylinder()` — 2× `BrepVertex`, 2× `BrepEdge`, 3× `BrepFace`, final
  `SemioBrepSnapshot`
- `brep_snapshot_for_sphere()` — 1× `BrepFace`, final `SemioBrepSnapshot`

Total: 5× `BrepFace`, 3× `BrepVertex`, 3× `BrepEdge`, 4× `SemioBrepSnapshot` — matches the
15 reported errors exactly.

## Verification (mandatory command, run to completion in the foreground/background — not killed)

```
cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-process --lib --target wasm32-wasip2 --keep-going
```

Verbatim final lines:

```
warning: constant `PROCESS_3D_PLAY_SURFACE_MAIN` is never used
  --> ✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/././././././../../🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️.rs:18:7
   |
18 | const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `semio-s-plugin-process` (lib) generated 112 warnings (run `cargo fix --lib -p semio-s-plugin-process` to apply 84 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 16m 08s
```

`grep -cE '^error'` on the full log: **0**. Cargo exit code: **0**. All 112 warnings are
pre-existing and unrelated to this crate's `process3d` brep code (dead-code/unused lints
elsewhere in the plugin).

## Files touched

- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` (edited — 15 struct-literal sites)
