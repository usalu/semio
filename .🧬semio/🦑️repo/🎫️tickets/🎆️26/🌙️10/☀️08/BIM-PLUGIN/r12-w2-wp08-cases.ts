#!/usr/bin/env bun
/**
 * 🧗️ Wave W2 `w2-wp08-walldepth`: the cases of the wall depth package on EXISTING leaves (attach on `create-wall` and `set-wall-top`, the authored reveal on `create-opening` and `set-opening`, the attach refusals of the
 * three deletes, the cascade of the sweeps of `delete-wall`, `delete-elements` and `split-wall`, the copy of the sweeps by `copy-elements`). Each case is added with its own `before` snapshot built from the shared fixtures of
 * `r12-w2-wp08-leaves.ts`. Usage: `bun r12-w2-wp08-cases.ts`; re-running skips a case that exists. Applied cases need `BIM_BLESS=1 cargo test` to write `after` and `diff`.
 */
import * as F from "./r3-f1-fixtures.ts";
import { addCase, caseDirs, type Outcome } from "./r6-z-mutations-cases.ts";
import { baseboard, library, ok, rect, reject, roof, withSweep, withTwoBuildings } from "./r12-w2-wp08-leaves.ts";

const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";
const add = (kind: string, name: string, before: unknown, mutation: Record<string, unknown>, outcome: Outcome) => {
  if (caseDirs(kind).some((dir) => dir.endsWith(name))) return console.log(`skip ${kind}/${name}`);
  const tag = kind.split("-").map((word, index) => (index === 0 ? word : word[0].toUpperCase() + word.slice(1))).join("");
  addCase(kind, name, { beforeText: json(before), mutationText: json({ mutation: tag, ...mutation }), outcome });
};

const toRoof = (roofId: string, offset = 0) => ({ Roof: { roof: roofId, offset } });
const toSlab = (slabId: string, offset = 0) => ({ Slab: { slab: slabId, offset } });
const toCeiling = (ceilingId: string, offset = 0) => ({ Ceiling: { ceiling: ceilingId, offset } });
const attic = (top: unknown, extra: Record<string, unknown> = {}) => ({ ...F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), top, "Attic gable"), ...extra });
const windowType = { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 1, material: "m-paint" };
const windowed = (opening: Record<string, unknown> = {}) => library({ window_types: { "win-1": windowType }, openings: Object.keys(opening).length ? { "o-1": { host: "w-south", kind: { Window: { window_type: "win-1" } }, offset: 4, flip_hand: false, flip_facing: false, name: "Window", ...opening } } : {} });
const newWindow = (extra: Record<string, unknown> = {}) => ({ id: "o-1", opening: { host: "w-south", kind: { Window: { window_type: "win-1" } }, offset: 4, flip_hand: false, flip_facing: false, name: "Window", ...extra } });
const ceilingLibrary = () => ({ ...library(), ceiling_types: { "cet-board": { name: "Board", layers: [F.layer("m-paint", 0.0125, "Finish")] } }, ceilings: { "ce-hall": { storey: "st-ground", ceiling_type: "cet-board", boundary: rect(0, 0, 8, 6), holes: [], offset: 0.3, name: "Hall ceiling" } } });
const attached = (model: any, wall: string, patch: Record<string, unknown>) => ({ ...model, walls: { ...model.walls, [wall]: { ...model.walls[wall], ...patch } } });

//#region 🔖️Attach
add("set-wall-top", "attaches-to-a-roof", library(), { id: "w-east", top: toRoof("r-main", -0.05) }, ok);
add("set-wall-top", "attaches-to-a-slab", library(), { id: "w-east", top: toSlab("sl-ground", 0.1) }, ok);
add("set-wall-top", "roof-missing", library(), { id: "w-east", top: toRoof("r-gone") }, reject("mutation.target-missing", ["top", "roof"]));
add("set-wall-top", "ceiling-missing", library(), { id: "w-east", top: toCeiling("ce-gone") }, reject("mutation.target-missing", ["top", "ceiling"]));
add("set-wall-top", "frees-an-attached-top", attached(library(), "w-east", { top: toRoof("r-main") }), { id: "w-east", top: F.unconnected(2.4) }, ok);

add("create-wall", "adds-a-wall-under-the-roof", library(), { id: "w-attic", wall: attic(toRoof("r-main")) }, ok);
add("create-wall", "adds-a-wall-standing-on-a-slab", library(), { id: "w-hall", wall: { ...F.wall("st-ground", "wt-300", F.line([0, 6], [8, 6]), F.storeyTop(0), "Hall"), base_slab: "sl-ground" } }, ok);
add("create-wall", "roof-missing", library(), { id: "w-attic", wall: attic(toRoof("r-gone")) }, reject("mutation.target-missing", ["wall", "top", "roof"]));
add("create-wall", "base-slab-missing", library(), { id: "w-hall", wall: attic(F.storeyTop(0), { base_slab: "sl-gone" }) }, reject("mutation.target-missing", ["wall", "base_slab"]));
add("create-wall", "base-slab-in-another-building", withTwoBuildings(), { id: "w-hall", wall: attic(F.storeyTop(0), { base_slab: "sl-b2" }) }, reject("mutation.invariant", ["wall", "base_slab"]));

const raftered = (model: any) => ({ ...model, roofs: { ...model.roofs, "r-b2": roof("st-b2", "Annex roof") } });
add("set-wall-top", "roof-in-another-building", raftered(withTwoBuildings()), { id: "w-east", top: toRoof("r-b2") }, reject("mutation.invariant", ["top", "roof"]));
//#endregion 🔖️Attach

//#region 🔖️Deletes
add("delete-roof", "attached-by-a-wall", attached(library(), "w-east", { top: toRoof("r-main") }), { id: "r-main" }, reject("mutation.target-referenced", ["r-main"]));
add("delete-slab", "attached-by-the-top-of-a-wall", attached(library(), "w-east", { top: toSlab("sl-ground") }), { id: "sl-ground" }, reject("mutation.target-referenced", ["sl-ground"]));
add("delete-slab", "attached-by-the-base-of-a-wall", attached(library(), "w-south", { base_slab: "sl-ground" }), { id: "sl-ground" }, reject("mutation.target-referenced", ["sl-ground"]));
add("delete-ceiling", "attached-by-a-wall", attached(ceilingLibrary(), "w-east", { top: toCeiling("ce-hall") }), { id: "ce-hall" }, reject("mutation.target-referenced", ["ce-hall"]));
add("delete-wall", "cascades-its-sweeps", withSweep(), { id: "w-south" }, ok);
add("delete-elements", "removes-a-wall-with-its-sweeps", withSweep(), { ids: ["w-south"] }, ok);
add("split-wall", "repeats-the-sweeps-on-the-new-wall", withSweep(), { id: "w-south", t: 0.5, new_id: "w-south-2" }, ok);
add("split-wall", "sweep-copy-id-taken", withSweep({ wall_sweeps: { "ws-base": baseboard(), "ws-base-w-south-2": baseboard("w-east") } }), { id: "w-south", t: 0.5, new_id: "w-south-2" }, reject("mutation.duplicate-id", ["ws-base-w-south-2"]));
add("copy-elements", "copies-a-wall-with-its-sweeps", withSweep(), { ids: ["w-south"], vector: { x: 0, y: 3 }, prefix: "cp" }, ok);
//#endregion 🔖️Deletes

//#region 🔖️Reveal
add("create-opening", "adds-a-window-with-a-reveal", windowed(), newWindow({ reveal_depth: 0.1, reveal_material: "m-paint" }), ok);
add("create-opening", "negative-reveal-depth", windowed(), newWindow({ reveal_depth: -0.1 }), reject("mutation.invariant", ["opening", "reveal_depth"]));
add("create-opening", "reveal-material-missing", windowed(), newWindow({ reveal_material: "m-ghost" }), reject("mutation.target-missing", ["opening", "reveal_material"]));
add("set-opening", "sets-a-reveal", windowed({ name: "Window" }), { id: "o-1", reveal_depth: { value: 0.1 }, reveal_material: { value: "m-paint" } }, ok);
add("set-opening", "clears-the-reveal", windowed({ reveal_depth: 0.1, reveal_material: "m-paint" }), { id: "o-1", reveal_depth: { value: null }, reveal_material: { value: null } }, ok);
add("set-opening", "negative-reveal-depth", windowed({ name: "Window" }), { id: "o-1", reveal_depth: { value: -0.1 } }, reject("mutation.invariant", ["reveal_depth"]));
add("set-opening", "reveal-material-missing", windowed({ name: "Window" }), { id: "o-1", reveal_material: { value: "m-ghost" } }, reject("mutation.target-missing", ["reveal_material"]));
//#endregion 🔖️Reveal

console.log("w2-wp08-walldepth: cases added");
