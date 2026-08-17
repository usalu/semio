# Normative Spec — Puzzle Design Parity

Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Goal: `R26-02`  
Every agent MUST read this file before editing.

## Terminology contract

| Compose | Puzzle 2d | Puzzle 3d | Puzzle 5d |
|---------|-----------|-----------|-----------|
| Piece | Node | Object | Part |
| Type | NodeKind | ObjectKind | PartKind |
| Port | HandleKind | VortexKind | GripKind |
| Connector | Handle | Vortex | Grip |
| Connection | Edge | Attraction | Fastener |
| Connection.u/v | Edge.x/y | Attraction.x/y | Fastener.x/y |
| PieceConnectionKind Fixed/Connected | Node.anchor Fixed/Derived | Object.anchor Fixed/Derived | Part.anchor Fixed/Derived |

## Units and axis decisions (pinned Wave 0)

1. **Rotation / turn / tilt are degrees** (compose stores degrees, converts via `deg_to_rad` in `geom::flatten`). Puzzle 3d brush pose does not yet consume these fields; the flatten port MUST treat stored values as degrees.
2. **Diagram axis mapping:** `node.x = center.u`, `node.y = center.v` (matches sketchpad `sketchpadPieceDiagramUv`). Fastener/Attraction/Edge `x`/`y` map from compose Connection `u`/`v`.
3. **Default anchor:** `Fixed` (compose `PieceConnectionKind::Fixed` is `#[default]`).
4. **Pose tolerance for golden parity:** `1e-6` after `round_f`.

## Connection analogue — identical 8 parameters

On `Puzzle2dEdge`, `Puzzle3dAttraction`, `Puzzle5dFastener`:

```rust
pub gap: f64,      // translation along parent-rotated Y
pub shift: f64,    // translation along parent-rotated X
pub rise: f64,     // translation along parent-rotated Z
pub rotation: f64, // degrees about parent connector direction (applied as -rotation_rad)
pub turn: f64,     // degrees about turn axis (parent-rotated Z, after rotation)
pub tilt: f64,     // degrees about tilt axis (parent-rotated X, after rotation)
pub x: f64,        // diagram offset (compose u)
pub y: f64,        // diagram offset (compose v)
```

Defaults: all `0.0`. Serde: camelCase. DSL keys: kebab-case (`fastener-kind`, etc.).

## Anchor

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle5dPartAnchor { // and Puzzle2dNodeAnchor / Puzzle3dObjectAnchor
    #[default]
    Fixed,
    Derived,
}
```

- `Fixed` root: keep stored plane + center.
- `Derived` root: keep center if present; reset plane to default XY.

## Kind catalogs (type-like)

Replace thin catalogs. 5d names shown; 2d/3d rename Part→Node/Object, Grip→Handle/Vortex, Fastener→Edge/Attraction, Rope→Wire/Cable.

### PartKind (Type)

```rust
pub struct Puzzle5dCatalogPartKind {
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub image: String,
    pub unit: String,
    pub abstract_: bool,          // serde/json: "abstract"
    pub base_kinds: Vec<String>,
    pub representations: Vec<Puzzle5dRepresentation>,
    pub grips: Vec<Puzzle5dGripTemplate>,
    pub attributes: Vec<Puzzle5dAttribute>,
    pub authors: Vec<Puzzle5dAuthor>,
}
```

### Representation

```rust
pub struct Puzzle5dRepresentation {
    pub id: String,
    pub name: String,
    pub url: String,
    pub mime: String,
    pub tags: Vec<String>,
    pub lod: Option<String>,
    pub description: String,
}
```

### GripTemplate (Connector template on a kind)

```rust
pub struct Puzzle5dGripTemplate {
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub grip_kind: Option<String>,
    pub point: [f64; 3],
    pub direction: [f64; 3],      // default [0,0,1]
    pub t: Option<f64>,
    pub mandatory: Option<bool>,
    pub radius: Option<f64>,
    // 2d templates use `angle: f64` instead of point/direction
}
```

### GripKind (Port)

```rust
pub struct Puzzle5dCatalogGripKind {
    pub id: String,
    pub code: Option<String>,
    pub label: Option<String>,
    pub order: Option<i32>,
    pub compatible_with: Vec<String>,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub default_rope_kind: String,
}
```

### Compatibility (unified)

```rust
pub struct Puzzle5dKindCompatibility {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: Puzzle5dCompatSpecificity,
}
```

2d: `Puzzle2dMeta.kind_catalogs` MUST become the typed struct (no more `Option<dsl::DslValue>`).

Out of scope: 3d `target_volumes` / `references` do **not** get 5d twins.

## Schema leaves (all five formats × three facets)

Every new field MUST land in:

- `🧬️schema/` — full artifact
- `📸️snapshot/🧬️schema/` — persistent subset
- `🔺️diff/🧬️schema/` — sparse deltas

Formats: `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`.

Also update: `🧬️mutations`, `🗣️dsl`, `🔧️op`, `📡️spr`, `📸️snapshot/🎒️pack`, existing examples' DSL if fields become required.

Enforced by `policyArtifactSchemaBreaches` and `catalog-integration` tests.

## Flatten constants (literal from compose)

```rust
const TOLERANCE: f64 = 0.01;
const DIAGRAM_RADIUS: f64 = 2.697;
const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;
const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633;

fn deg_to_rad(deg: f64) -> f64 { deg * std::f64::consts::PI / 180.0 }
fn round_f(v: f64) -> f64 { (v * 1_000_000.0).round() / 1_000_000.0 }
```

## Flatten algorithm (byte-exact)

Source of truth: `compose/client/lib/rs/lib.rs` `geom::flatten` lines 1189–1503.

### Placement of modules

- 3d: `🗿️artifacts/🧊️3d/⚙️engine/📐️geometry/` — submodule `flatten` — absolute planes from attraction graph
- 5d: `🗿️artifacts/🖐️5d/⚙️engine/📐️flatten/` — wraps 3d planes + diagram centers
- 2d: `🗿️artifacts/◻2d/⚙️engine/📐️layout/` — new `fastened` layout mode (same 3-branch center rule)

### Absolute plane (`compute_child_plane`)

1. Build undirected adjacency from attractions/fasteners/edges (parent/child via endpoint piece/object/part ids).
2. BFS per component in document order.
3. Fixed root: stored plane + center. Derived root: keep center; plane = default XY.
4. For each neighbor:
   - Align reverse-child direction onto parent direction via quaternion (antiparallel fallback with TOLERANCE).
   - Apply `-rotation` about parent direction; then `turn` about turn axis; then `tilt` about tilt axis.
   - Translate by `(shift, gap, rise)` in parent-rotated basis `(X,Y,Z)`.
   - Translate by parent connector point; compose with parent plane matrix.
5. Round plane axes/origin with `round_f`.

### Diagram center (3-branch rule)

```text
connection_x = fastener.x (default 0)
connection_y = fastener.y (default 0)
parent_t / angle from parent grip (t or angle/2π)

if parent_center == (0,0):
  angle = 2π * parent_t
  child = (DIAGRAM_RADIUS * sin(angle), DIAGRAM_RADIUS * cos(angle))
else if |parent_direction.z| > 0.5:
  child = parent + (connection_x, connection_y + DIAGRAM_VERTICAL_V_EXTRA)
else:
  child = parent + (connection_x, connection_y) * DIAGRAM_HORIZONTAL_SCALE

round_f both components
```

## Legacy bridge removal

- DELETE runtime `importComposeKit` action and `🗿️artifacts/🖐️5d/⚙️engine/🌉️compose/`.
- Compose fixtures appear ONLY inside test-only parity harness (Wave 5).

## Example asset coherence

- `.dsl` is the authoring source of truth.
- `.op` / `.pack` / `.spr` MUST equal codec output for the parsed DSL (not stubs with `len() > 64`).
- Extend `ExamplesScript` with `examples emit`; tighten `policySemioArtifactExamplesBreaches`.

## Capsule Dream acceptance

- Source: `compose/fixture/kit/dev/metabolism/wip/initialKit/design/capsule-dream.design.compose.json` (2880 pieces / 2864 connections).
- Golden: sibling `flat.design.compose.json` (2880 posed pieces).
- Puzzle 5d flatten of transferred asset MUST match golden for all 2880 parts: `origin`, plane axes, `center.x/y` to `1e-6`.
- Same for nakagin + slanted/twisted/dancing variants.
- 3d flatten and 2d `fastened` layout MUST agree with the 5d projection.

## Workforce hard rules

- Scratch only inside this ticket folder.
- Never mutate git (no commit/stash/checkout/worktree).
- Never touch a file owned by another agent in the same wave.
- Write a `🧪`-prefixed markdown report into this ticket folder.
- Run the scoped test command before reporting.
- Models: `cursor-grok-4.5-high` or `composer-2.5` only (no fast variants).

## Single-owner files (never edit in parallel)

- `📦️packages/🦀️rust/📦️glue.rs`
- `📦️packages/🟦️typescript/📦️index.ts`
- `.vscode/launch.json`
- `.storybook/stories/puzzle/**`
- `🎛️apps/🖐️5d/🦀️component.rs` (Wave 3 integrator D7 only)

## Wave ownership (quick ref)

| Wave | Agent | Owns |
|------|-------|------|
| 1 | A1-2d | `🗿️artifacts/◻2d/**` |
| 1 | A2-3d | `🗿️artifacts/🧊️3d/**` except `⚙️engine/📐️geometry` |
| 1 | A3-5d | `🗿️artifacts/🖐️5d/**` except new `⚙️engine/📐️flatten` |
| 1b | B1 | glue.rs + TS index |
| 2 | C1 | 3d geometry flatten |
| 2 | C2 | 5d flatten |
| 2 | C3 | 2d fastened layout |
| 3 | D1–D6 | leaf folders under `🎛️apps/🖐️5d/` |
| 3 | D7 | app root + edit mode |
| 4+ | see plan | |
