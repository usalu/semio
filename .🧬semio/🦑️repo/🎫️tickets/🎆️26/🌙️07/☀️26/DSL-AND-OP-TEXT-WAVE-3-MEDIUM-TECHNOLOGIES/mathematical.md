# mathematical technology — DocumentDsl / OpText

Scope: `mathematical/plugin/rs/lib.rs` only.

## Shape investigated
- `MathProjection { graph: MathGraph, geometry: MathGeometry }`
- `MathGraph { directed: bool, nodes: Vec<MathNode>, edges: Vec<MathEdge>, camera: MathCamera, algorithm: String, algorithm_seed: Option<String> }`
- `MathNode { id, label, x, y }` / `MathEdge { id, source, target }` / `MathCamera { x, y, zoom }`
- `MathGeometry { points: Vec<(f64, f64)> }`
- `MathOperation { SetGraph { graph }, SetGeometry { geometry } }` — exactly 2 variants, coarse whole-slice replace.

## Wire-literal precedent (mathematical/graph/dsl/rs/lib.rs `wire` region)
`wire_literal_from_dag`/`dag_from_wire_literal` grammar is `id:kind@port -> id:kind@port {props}` — built for a
neutral typed-port DAG with a `PropertyBag`. `MathGraph`'s nodes/edges have no kind/port/property-bag concept
(just id/label/x/y and id/source/target), so the exact syntax doesn't map. Reused the *arrow* convention
(`source -> target` for directed edges) as the "analogous in spirit" nod to that precedent, and reused `note`
plugin's established `note_text` module conventions (key=value header tokens, comma `x,y` point-pair lists,
`pretty: bool` newline-vs-space printer parameter) since those are the closer structural match.

## Grammar
Document (`.mathematical` DSL, `print_dsl` pretty=true):
```
graph directed=true algorithm="topo" seed=-
camera x=0 y=0 zoom=1
node a "A" x=40 y=60
node b "B" x=240 y=20
node c "C" x=240 y=180
node d "D" x=440 y=100
edge e1 a -> b
edge e2 a -> c
edge e3 b -> d
edge e4 c -> d
points 40,220 260,40 360,140 300,260 140,300 180,160
```
- `graph` line + `camera` line required exactly once; `node`/`edge` lines zero or more; `points` line
  optional (omitted entirely when geometry has no points).
- `seed=-` for `None`, `seed="..."` for `Some`.

Op-text (single line, space-joined instead of newline-joined):
- `SetGraph`: `graph directed=... algorithm=... seed=... camera x=... y=... zoom=... node ... edge ...`
  (whole thing on one line — the shared `parse_graph`/`print_graph` functions take a `pretty: bool`).
- `SetGeometry`: `points x,y x,y ...` (or bare `points` when empty).

## Implementation
- `mod math_text` (private, hand-rolled lexer/parser/printer, no new deps) added inside `//#region 🔖️Dsl`,
  mirroring `note/plugin/rs/lib.rs`'s `note_text` module structure (Lexer/Parser/Printer sub-regions).
- `impl vcs::DocumentDsl for MathProjection` (`EXTENSION = "mathematical"`) inside `//#region 🔖️Dsl`.
- `impl vcs::OpText for MathOperation` inside its own `//#region 🔖️OpText`.
- `.example("demo", "Demo", ...)` switched from `serde_json::to_string(&MathProjection::default())` to
  `MathProjection::default().print_dsl()` (added `DocumentDsl` to the `use vcs::{...}` import).
- Tests added under new `//#region 🔖️DslTests` nested inside the existing `//#region 🧪️Tests` module:
  - `math_projection_dsl_round_trips_default`
  - `math_projection_dsl_round_trips_with_seed_and_empty_collections` (Some(seed), empty nodes/edges/points)
  - `math_set_graph_op_round_trips`
  - `math_set_geometry_op_round_trips`
  - `math_document_text_round_trips_through_store` (via `vcs::create_document_vcs_envelope` +
    `vcs::DocumentVcsStore` + `vcs::DocumentVcsCommand::Apply` + `vcs::test_support::assert_document_text_round_trip`)

## Verification
- `cargo test -p mathematical-plugin --lib` — run with `CARGO_TARGET_DIR` pointed at scratchpad to avoid
  shared target/ lock contention. See results appended below once green.
- `cargo check -p mathematical-plugin --target wasm32-unknown-unknown` — same isolated target dir.
