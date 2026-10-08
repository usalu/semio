# 🪜️ API: `stair-runs` (Wave I, label `i-spaces-quantities`)

For `i-solids-rest` (stair mesh) and `i-plan-diagnostics` (plan linework, stair rule violations). Module path
`semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::stair_runs`, file
`S/🧬️schema/💡️inferences/🪜️stair-runs/🦀️.rs` (`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`).
Field on the aggregate: `ModelInference.stair_runs: BTreeMap<String /*stair id*/, StairRun>`.
Stairs on a storey that does not exist are not planned, so they have no entry. Pure entry point for one stair when you
already hold the storey levels: `run_of(&Stair, &StoreyLevel /*own*/, Option<&StoreyLevel> /*target of TopConstraint::Storey*/) -> StairRun`.

## Placement conventions (the whole contract)

- `Stair.start` is the middle of the foot edge of the first riser; `Stair.direction` the travel angle (radians, counter-clockwise
  from +X); `Stair.width` the clear width across the direction of travel (symmetric about the walking line).
- A flight with first riser `first_riser` (1-based over the whole stair) stands on `start`. Riser `k` of the flight (0-based)
  has its foot edge centred at `start + k * tread * (cos direction, sin direction)`, spans `width` across, and its foot is at
  `base_z + k * riser_height` (`flight.base_z` already includes the risers of earlier flights). The tread surface behind riser `k`
  is at `flight.base_z + (k + 1) * riser_height` and reaches to riser `k + 1`.
- The last riser of the last flight arrives on the upper floor (`top_z`): a stair of `n` risers has `n - 1` treads; a landing counts
  as one tread. There is no tread surface after the last riser (it is the floor of the storey above).
- All risers have the same height; all plain treads of non-spiral stairs have the same going.

## Values (all `f64` metres/radians unless noted; derive `ToValue/FromValue`, `Clone`, `Default`, `PartialEq`)

```rust
pub struct StairRun {
    pub base_z: f64,          // own storey elevation (building-relative, like WallLayout.base_z)
    pub top_z: f64,           // Unconnected: base + height | StoreyTop: storey top + offset | Storey: target elevation + offset
    pub rise: f64,            // top_z - base_z
    pub riser_count: u32,     // ceil(rise / max_riser), >= 1 when rise > 0, 0 when rise <= 0 or max_riser invalid; capped at MAX_RISERS = 512
    pub riser_height: f64,    // rise / riser_count (0 when riser_count == 0)
    pub tread_count: u32,     // riser_count - 1 (landings included as one tread)
    pub tread: f64,           // going per plain tread; non-spiral: max(min_tread, 0.62 - 2 * riser_height); spiral: arc on the walking line
    pub stride: f64,          // 2 * riser_height + tread (Blondel)
    pub width: f64,
    pub run_length: f64,      // sum of flight lengths
    pub flights: Vec<StairFlightRun>,
    pub landings: Vec<StairLanding>,
    pub compliance: StairCompliance,
}
pub struct StairFlightRun {
    pub first_riser: u32, pub risers: u32, pub treads: u32,   // treads = risers - 1
    pub start: Point2,        // centre of the foot edge of riser `first_riser`
    pub direction: f64,       // travel angle of this flight (tangent at `start` for a winder)
    pub tread: f64,           // going per tread (arc length on the walking line for a winder)
    pub base_z: f64,          // z of the foot of riser `first_riser`
    pub length: f64,          // treads * tread: horizontal distance first riser -> last riser
    pub winder: Option<StairWinder>,
}
pub struct StairWinder { pub centre: Point2, pub inner_radius: f64, pub outer_radius: f64, pub start_angle: f64, pub sweep: f64 }
pub struct StairLanding {
    pub after_flight: u32,    // index into `flights` of the arriving flight
    pub z: f64,               // landing surface = base_z + risers-so-far * riser_height
    pub centre: Point2,       // centre of the rectangle
    pub direction: f64,       // angle of the arriving flight; `depth` is measured along it
    pub width: f64,           // across `direction`
    pub depth: f64,           // along `direction`
}
pub struct StairCompliance { pub rise_positive: bool, pub riser_ok: bool, pub tread_ok: bool, pub blondel_ok: bool, pub compliant: bool }
```

`Point2` is the snapshot's `{x, y}`. Constants: `BLONDEL_MIN = 0.59`, `BLONDEL_MAX = 0.65`, `BLONDEL_TARGET = 0.62`, `MAX_RISERS = 512`.

## Flight kinds (how the numbers above are produced)

| `StairFlight` | Flights | Landings | Geometry |
|---|---|---|---|
| `Straight` | 1 (`riser_count` risers) | none | along `direction` from `start` |
| `LTurn { split, turn }` | `n1 = clamp(round(split * count), 1, count - 1)` and `count - n1` | one square landing `width x width` | flight 1 from `start`; landing starts at the last riser face of flight 1 and extends `width` along `direction`; flight 2 turns by `+90 deg` (`Left`) or `-90 deg` (`Right`) and starts at the landing edge centre `landing.centre + (width / 2) * dir2` |
| `UTurn { gap }` | `ceil(count / 2)` and the rest | one landing `depth = width`, `width' = 2 * width + gap` | flight 2 runs back (`direction + pi`), starts `width + gap` to the left of flight 1's last riser foot; the landing covers both flights' ends |
| `Spiral { radius, sweep }` | 1 winder flight | none | `radius` = outer radius, walking line radius `radius - width / 2`, signed `sweep` (positive = counter-clockwise, centre on the left of `direction`), `start_angle = direction -/+ pi/2`; `tread` = `|sweep| * walking_radius / treads` |
| LTurn/UTurn with 1 riser | 1 | none | cannot turn |

Compliance: `riser_ok = riser_height <= max_riser && count within cap`, `tread_ok = tread >= min_tread` (when more than one
riser), `blondel_ok = 0.59 <= 2R + T <= 0.65` (when more than one riser), `compliant` = all four flags.

## Parametric law

`stair-runs` is an `InferredField<ModelSnapshot>` (`FIELD_ID s.bim.model.inference.stair-runs`, reads `stairs, storeys, buildings, sites`),
key `Rooted` (`storey_levels::{Rooted, RootedValue, rooted_plan, rooted_dependency, rooted_value, rooted_elements}` is the
reusable storey-rooted pattern added to the `storey-levels` leaf): storey nodes are roots, a stair has its own storey and the
storey of a `TopConstraint::Storey` as parents. Editing one storey height re-infers exactly the stairs that stand on or reach it.
Tests: storey height 3.0 -> 3.4 gives 16 -> 19 risers; an `Unconnected` stair keeps its rise.
