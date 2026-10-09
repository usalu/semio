/**
 * 🛝️ Wave W09: the ramp and railing-host cases of the EXISTING leaves, each derived from a committed case of its leaf (its `before` snapshot) with the hosts added:
 * `set-railing` (hosts, releases, refusals), `create-railing` (hosted by a ramp, refusals), `delete-stair` / `delete-slab` (the hosted railing leaves with its host),
 * `delete-material` (refused while a ramp names it), `move-elements` / `rotate-elements` (a ramp is a placed kind). Usage: `bun r10-w09-ramps-cases.ts`;
 * re-running refuses (a case of that name exists). Applied cases need `BIM_BLESS=1 cargo test` to write their `after` and `diff`.
 */
import { addCase, caseDirs, read, type Outcome } from "./r6-z-mutations-cases.ts";

const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";
const reject = (code: string, path: string[]): Outcome => ({ status: "rejected", code, path });
const ok: Outcome = { status: "applied" };

const dirOf = (kind: string, name: string) => {
  const found = caseDirs(kind).find((dir) => dir.endsWith(name));
  if (!found) throw new Error(`${kind} has no case ${name}`);
  return found;
};
const beforeOf = (kind: string, name: string) => JSON.parse(read(kind, dirOf(kind, name), "before"));
const payloadOf = (kind: string, name: string) => JSON.parse(read(kind, dirOf(kind, name), "mutation"));

const vertex = (x: number, y: number, bulge = 0) => ({ point: { x, y }, bulge });
const ramp = (storey: string, material: string, over: Record<string, unknown> = {}) => ({
  storey,
  path: [vertex(0, 0), vertex(10, 0)],
  width: 1.2,
  landing_start: 1.5,
  landing_end: 1.5,
  landing_turn: 1.5,
  max_slope: 0.0833333333333333,
  thickness: 0.2,
  material,
  base_offset: 0,
  top: { Unconnected: { height: 0.5 } },
  railing_left: false,
  railing_right: false,
  name: "Ramp",
  ...over,
});
const stair = (storey: string, over: Record<string, unknown> = {}) => ({
  storey,
  start: { x: 0, y: 0 },
  direction: 0,
  width: 1.2,
  flight: "Straight",
  top: { StoreyTop: { offset: 0 } },
  max_riser: 0.18,
  min_tread: 0.27,
  stringer: { kind: "None", width: 0.05, depth: 0.25 },
  nosing: 0,
  tread_thickness: 0.04,
  riser: "Closed",
  landing_depth: 1.2,
  phase: "New",
  name: "Stair",
  ...over,
});
const slabOf = (storey: string, over: Record<string, unknown> = {}) => ({ storey, slab_type: "slt-host", boundary: [vertex(0, 0), vertex(4, 0), vertex(4, 3), vertex(0, 3)], holes: [], offset: 0, phase: "New", name: "Host slab", ...over });
const host = (element: string, over: Record<string, unknown> = {}) => ({ element, side: "Left", edge: 0, inset: 0.05, ...over });
const hosted = (storey: string, material: string, element: string, over: Record<string, unknown> = {}) => ({
  storey,
  path: [],
  height: 1,
  post_spacing: 1.2,
  profile: { Rectangle: { width: 0.06, depth: 0.04 } },
  post_profile: { Rectangle: { width: 0.05, depth: 0.05 } },
  infill: "None",
  material,
  base_offset: 0,
  host: host(element),
  phase: "New",
  name: "Hosted guard",
  ...over,
});
const withSlabType = (before: any, material: string) => {
  before.slab_types = { ...(before.slab_types ?? {}), "slt-host": { name: "Host slab 200", layers: [{ material, thickness: 0.2, function: "Structure" }] } };
};
const only = (record: Record<string, unknown> | undefined) => Object.keys(record ?? {})[0];

//#region set-railing
{
  const kind = "set-railing";
  const make = (name: string, outcome: Outcome, edit: (before: any, material: string) => any) => {
    const before = beforeOf(kind, "reshapes");
    const material = only(before.materials);
    const payload = edit(before, "m-steel" in before.materials ? "m-steel" : material);
    addCase(kind, name, { beforeText: json(before), mutationText: json({ mutation: "setRailing", ...payload }), outcome });
  };
  make("hosts-on-a-stair", ok, (before, material) => ((before.stairs = { "s-host": stair("st-ground") }), { id: "rl-1", path: [], host: { value: host("s-host", { side: "Right" }) } }));
  make("hosts-on-a-ramp", ok, (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), { id: "rl-1", path: [], host: { value: host("rp-host") } }));
  make("hosts-on-a-slab-edge", ok, (before, material) => (withSlabType(before, material), (before.slabs = { "sl-host": slabOf("st-ground") }), { id: "rl-1", path: [], host: { value: host("sl-host", { edge: 2, inset: 0.1 }) } }));
  make("releases-the-host", ok, (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), (before.railings["rl-1"] = hosted("st-first", material, "rp-host")), { id: "rl-1", path: [{ x: 0, y: 0 }, { x: 5, y: 0 }], host: { value: null } }));
  make("moves-to-another-host", ok, (before, material) => ((before.stairs = { "s-host": stair("st-ground") }), (before.ramps = { "rp-host": ramp("st-ground", material) }), (before.railings["rl-1"] = hosted("st-first", material, "rp-host")), { id: "rl-1", host: { value: host("s-host", { side: "Right", inset: 0.1 }) } }));
  make("host-without-clearing-the-path", reject("mutation.invariant", ["path"]), (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), { id: "rl-1", host: { value: host("rp-host") } }));
  make("releasing-without-a-path", reject("mutation.invariant", ["path"]), (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), (before.railings["rl-1"] = hosted("st-first", material, "rp-host")), { id: "rl-1", host: { value: null } }));
  make("host-missing", reject("mutation.target-missing", ["host", "element"]), () => ({ id: "rl-1", path: [], host: { value: host("nothing") } }));
  make("host-is-no-stair-ramp-or-slab", reject("mutation.target-missing", ["host", "element"]), () => ({ id: "rl-1", path: [], host: { value: host("w-south") } }));
  make("slab-edge-missing", reject("mutation.invariant", ["host", "edge"]), (before, material) => (withSlabType(before, material), (before.slabs = { "sl-host": slabOf("st-ground") }), { id: "rl-1", path: [], host: { value: host("sl-host", { edge: 7 }) } }));
  make("slab-edge-curved", reject("mutation.invariant", ["host", "edge"]), (before, material) => (withSlabType(before, material), (before.slabs = { "sl-host": slabOf("st-ground", { boundary: [vertex(0, 0, 0.4), vertex(4, 0), vertex(4, 3), vertex(0, 3)] }) }), { id: "rl-1", path: [], host: { value: host("sl-host", { edge: 0 }) } }));
  make("stair-host-with-an-edge-index", reject("mutation.invariant", ["host"]), (before) => ((before.stairs = { "s-host": stair("st-ground") }), { id: "rl-1", path: [], host: { value: host("s-host", { edge: 3 }) } }));
  make("negative-host-inset", reject("mutation.invariant", ["host"]), (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), { id: "rl-1", path: [], host: { value: host("rp-host", { inset: -0.1 }) } }));
}
//#endregion

//#region create-railing
{
  const kind = "create-railing";
  const make = (name: string, outcome: Outcome, edit: (before: any, material: string) => any) => {
    const before = beforeOf(kind, "adds");
    const material = "m-steel" in before.materials ? "m-steel" : only(before.materials);
    const railing = edit(before, material);
    addCase(kind, name, { beforeText: json(before), mutationText: json({ mutation: "createRailing", id: "rl-9", railing }), outcome });
  };
  make("adds-a-railing-hosted-by-a-ramp", ok, (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), hosted("st-ground", material, "rp-host", { host: host("rp-host", { side: "Right" }) })));
  make("adds-a-railing-hosted-by-a-stair", ok, (before, material) => ((before.stairs = { "s-host": stair("st-ground") }), hosted("st-ground", material, "s-host")));
  make("host-missing", reject("mutation.target-missing", ["railing", "host", "element"]), (_, material) => hosted("st-ground", material, "nothing"));
  make("hosted-railing-with-a-path", reject("mutation.invariant", ["railing", "path"]), (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), hosted("st-ground", material, "rp-host", { path: [{ x: 0, y: 0 }, { x: 2, y: 0 }] })));
  make("negative-host-inset", reject("mutation.invariant", ["railing", "host"]), (before, material) => ((before.ramps = { "rp-host": ramp("st-ground", material) }), hosted("st-ground", material, "rp-host", { host: host("rp-host", { inset: -1 }) })));
  make("slab-edge-missing", reject("mutation.invariant", ["railing", "host", "edge"]), (before, material) => (withSlabType(before, material), (before.slabs = { "sl-host": slabOf("st-ground") }), hosted("st-ground", material, "sl-host", { host: host("sl-host", { edge: 9 }) })));
}
//#endregion

//#region deletes of a host
for (const [kind, collection, name, build] of [
  ["delete-stair", "stairs", "takes-its-hosted-railing-with-it", (storey: string, material: string, id: string) => hosted(storey, material, id)],
  ["delete-slab", "slabs", "takes-its-hosted-railing-with-it", (storey: string, material: string, id: string) => hosted(storey, material, id, { host: host(id, { edge: 1 }) })],
] as const) {
  const before = beforeOf(kind, "removes");
  const id = only(before[collection]);
  const record = before[collection][id];
  const material = only(before.materials);
  before.railings = { ...(before.railings ?? {}), "rl-host": build(record.storey, material, id) };
  addCase(kind, name, { beforeText: json(before), mutationText: json(payloadOf(kind, "removes")), outcome: ok });
}
//#endregion

//#region delete-material
{
  const kind = "delete-material";
  const before = beforeOf(kind, "used-by-a-railing");
  const material = payloadOf(kind, "used-by-a-railing").id;
  before.railings = {};
  const storey = only(before.storeys);
  before.ramps = { "rp-host": ramp(storey, material) };
  addCase(kind, "used-by-a-ramp", { beforeText: json(before), mutationText: json(payloadOf(kind, "used-by-a-railing")), outcome: reject("mutation.target-referenced", [material]) });
}
//#endregion

//#region placement
for (const [kind, base, name, edit] of [
  ["move-elements", "moves-a-wall-and-its-opening", "moves-a-ramp", (payload: any) => ({ ...payload, ids: ["rp-host"], vector: { x: 2, y: -1 } })],
  ["rotate-elements", "turns-a-wall-and-a-column", "turns-a-ramp", (payload: any) => ({ ...payload, ids: ["rp-host"], pivot: { x: 0, y: 0 }, angle: Math.PI / 2 })],
] as const) {
  const before = beforeOf(kind, base);
  const storey = only(before.storeys);
  const material = only(before.materials);
  before.ramps = { "rp-host": ramp(storey, material, { path: [vertex(0, 0), vertex(10, 0)] }) };
  addCase(kind, name, { beforeText: json(before), mutationText: json(edit(payloadOf(kind, base))), outcome: ok });
}
//#endregion

console.log("ramp and railing-host cases added");
