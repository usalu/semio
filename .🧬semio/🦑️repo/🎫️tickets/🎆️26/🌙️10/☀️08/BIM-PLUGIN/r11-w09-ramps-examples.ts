/**
 * 🛝️ Wave R11 `w09-ramps`: the entrance ramp of the `house` example and the two railings it hosts, merged into the snapshot by `r4-x-examples-gen.ts`. Every value is authored (path, width, landings, slope limit,
 * thickness, rise, railing host, side and inset); the run, the slope, the flights and the code flags are inferred. `rampsFor(name, model)` returns the `ramps` and the `railings` to add and refuses a reference that
 * points at nothing, a ramp the slope limit forbids and a path that leaves the plot.
 */
import * as F from "./r3-f1-fixtures.ts";

type Json = Record<string, any>;
const P = F.P;
const vertex = (x: number, y: number, bulge = 0) => ({ point: P(x, y), bulge });

const house = () => ({
  ramps: {
    "rp-entry": {
      storey: "st-ground",
      path: [vertex(8.9, -5.2), vertex(2.9, -5.2), vertex(2.9, -0.2)],
      width: 1.2,
      landing_start: 1.5,
      landing_end: 1.5,
      landing_turn: 1.5,
      max_slope: 1 / 12,
      thickness: 0.2,
      material: "m-concrete",
      base_offset: -0.5,
      top: F.unconnected(0.5),
      railing_left: false,
      railing_right: false,
      name: "Entrance Ramp",
    },
  },
  railings: {
    "rl-ramp-left": hosted("rp-entry", "Left", "Entrance Ramp Railing Left"),
    "rl-ramp-right": hosted("rp-entry", "Right", "Entrance Ramp Railing Right"),
  },
});

function hosted(element: string, side: "Left" | "Right", name: string) {
  return {
    storey: "st-ground",
    path: [],
    height: 0.9,
    post_spacing: 1.5,
    profile: { Rectangle: { width: 0.05, depth: 0.04 } },
    post_profile: { Rectangle: { width: 0.05, depth: 0.05 } },
    infill: "None",
    material: "m-steel",
    base_offset: 0,
    host: { element, side, edge: 0, inset: 0.05 },
    name,
  };
}

const SLOPE_LIMIT_EPS = 1e-9;

const arc = (a: Json, b: Json, bulge: number) => {
  const chord = Math.hypot(b.x - a.x, b.y - a.y);
  if (bulge === 0) return chord;
  const sweep = 4 * Math.atan(Math.abs(bulge));
  return (chord / (2 * Math.sin(sweep / 2))) * sweep;
};

/** 📏️ The length of a ramp path, its landings (foot, head and one per right-angle corner) and its slope, mirroring `ramp-runs` for the straight, right-angled paths the examples use. */
function slope(ramp: Json): number {
  const points = ramp.path.map((v: Json) => v.point);
  const lengths = points.slice(1).map((p: Json, i: number) => arc(points[i], p, ramp.path[i].bulge));
  const total = lengths.reduce((a: number, b: number) => a + b, 0);
  const turns = lengths.length - 1;
  const run = Math.max(total - ramp.landing_start - ramp.landing_end - turns * ramp.landing_turn, 0);
  const top = ramp.top.Unconnected ? ramp.top.Unconnected.height : 0;
  const rise = Math.abs(top);
  if (rise <= SLOPE_LIMIT_EPS) return 0;
  if (run <= 0) throw new Error(`ramp has no sloped run left between its landings`);
  return rise / run;
}

export function rampsFor(name: string, model: Json): Json {
  const made: Json = name === "house" ? house() : { ramps: {}, railings: {} };
  for (const [id, ramp] of Object.entries<Json>(made.ramps)) {
    if (model.storeys[ramp.storey] === undefined) throw new Error(`ramps/${id}: storey ${ramp.storey}`);
    if (model.materials[ramp.material] === undefined) throw new Error(`ramps/${id}: material ${ramp.material}`);
    if (ramp.top.Storey && model.storeys[ramp.top.Storey.storey] === undefined) throw new Error(`ramps/${id}: top storey ${ramp.top.Storey.storey}`);
    const found = slope(ramp);
    if (found > ramp.max_slope + SLOPE_LIMIT_EPS) throw new Error(`ramps/${id}: slope ${found.toFixed(4)} is above its limit ${ramp.max_slope.toFixed(4)}`);
    const site = model.sites[model.buildings[model.storeys[ramp.storey].building].site];
    const building = model.buildings[model.storeys[ramp.storey].building];
    for (const { point } of ramp.path) {
      const [x, y] = [point.x + building.origin.x, point.y + building.origin.y];
      const xs = site.boundary.map((p: Json) => p.x);
      const ys = site.boundary.map((p: Json) => p.y);
      if (x < Math.min(...xs) || x > Math.max(...xs) || y < Math.min(...ys) || y > Math.max(...ys)) throw new Error(`ramps/${id}: the point ${point.x}, ${point.y} leaves the plot`);
    }
  }
  for (const [id, railing] of Object.entries<Json>(made.railings)) {
    if (model.storeys[railing.storey] === undefined) throw new Error(`railings/${id}: storey ${railing.storey}`);
    if (model.materials[railing.material] === undefined) throw new Error(`railings/${id}: material ${railing.material}`);
    if (made.ramps[railing.host.element] === undefined) throw new Error(`railings/${id}: host ${railing.host.element}`);
  }
  return made;
}
