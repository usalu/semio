/**
 * 🧪️ Adds the Wave Z cases to existing leaves, all derived from each leaf's first applied case:
 * - `id-taken-by-another-kind` (rejected, `mutation.duplicate-id`) for every create family and `split-wall`: the payload id is the id of a
 *   record of a different collection that the before snapshot already holds (a donor record is added when it holds none);
 * - `restates-an-unchanged-field` (applied) for every whole-payload `set-*` leaf: the payload also names one field at its current value,
 *   which the blessed diff must not carry;
 * - `blank-property` for `set-element-property` replaces the retired `mismatched-value` case (a kind mismatch is no longer representable).
 * Usage: `bun r6-z-mutations-new-cases.ts <duplicates|restates|property>`; the data cases of the delete leaves come from `r6-z-mutations-deletes.ts --cases`.
 */
import { addCase, appliedCases, caseDirs, read, removeCase } from "./r6-z-mutations-cases.ts";
import { field, object, parse, print, raw, string_, type Node } from "./r6-z-mutations-rawjson.ts";

const CREATES: [string, string][] = [
  ["create-site", "sites"],
  ["create-building", "buildings"],
  ["create-storey", "storeys"],
  ["create-grid-line", "grids"],
  ["create-wall", "walls"],
  ["create-curtain-wall", "curtain_walls"],
  ["create-column", "columns"],
  ["create-beam", "beams"],
  ["create-slab", "slabs"],
  ["create-roof", "roofs"],
  ["create-opening", "openings"],
  ["create-stair", "stairs"],
  ["create-railing", "railings"],
  ["create-space", "spaces"],
  ["create-material", "materials"],
  ["create-wall-type", "wall_types"],
  ["create-slab-type", "slab_types"],
  ["create-roof-type", "roof_types"],
  ["create-column-type", "column_types"],
  ["create-beam-type", "beam_types"],
  ["create-window-type", "window_types"],
  ["create-door-type", "door_types"],
  ["split-wall", "walls"],
];
const COLLECTIONS = ["storeys", "walls", "sites", "buildings", "grids", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types"];

const SETS: [string, string | null][] = [
  ["set-material", "materials"],
  ["set-wall-type", "wall_types"],
  ["set-slab-type", "slab_types"],
  ["set-roof-type", "roof_types"],
  ["set-column-type", "column_types"],
  ["set-beam-type", "beam_types"],
  ["set-window-type", "window_types"],
  ["set-door-type", "door_types"],
  ["set-building", "buildings"],
  ["set-site", "sites"],
  ["set-grid-line", "grids"],
  ["set-railing", "railings"],
  ["set-space", "spaces"],
  ["set-curtain-wall", "curtain_walls"],
  ["set-project-info", null],
];

const firstApplied = (kind: string) => {
  const dir = appliedCases(kind).sort()[0];
  if (!dir) throw new Error(`${kind} has no applied case`);
  return dir;
};

const DONOR: Record<string, Node> = {
  materials: object(["name", string_("Donor")], ["category", string_("Other")], ["color", object(["r", raw("0.5")], ["g", raw("0.5")], ["b", raw("0.5")])], ["density", raw("1000.0")], ["conductivity", raw("1.0")], ["specific_heat", raw("1000.0")]),
  sites: object(["name", string_("Donor")], ["latitude", raw("0.0")], ["longitude", raw("0.0")], ["elevation", raw("0.0")], ["true_north", raw("0.0")], ["boundary", { k: "arr", v: [] }]),
};

function duplicates() {
  for (const [kind, own] of CREATES) {
    const base = firstApplied(kind);
    const before = parse(read(kind, base, "before"));
    const mutation = parse(read(kind, base, "mutation"));
    if (before.k !== "obj" || mutation.k !== "obj") throw new Error("documents are no objects");
    const key = kind === "split-wall" ? "new_id" : "id";
    let donor: string | undefined;
    for (const name of COLLECTIONS.filter((name) => name !== own)) {
      const records = field(before, name);
      if (records?.k === "obj" && records.v.length > 0) {
        donor = records.v[0][0];
        break;
      }
    }
    if (!donor) {
      const name = own === "sites" ? "materials" : "sites";
      donor = "x-taken";
      const existing = before.v.find(([entry]) => entry === name);
      if (existing && existing[1].k === "obj") existing[1].v.push([donor, DONOR[name]]);
      else if (existing) existing[1] = object([donor, DONOR[name]]);
      else before.v.push([name, object([donor, DONOR[name]])]);
    }
    const slot = mutation.v.find(([entry]) => entry === key);
    if (!slot) throw new Error(`${kind} mutation has no ${key}`);
    slot[1] = string_(donor);
    addCase(kind, "id-taken-by-another-kind", { beforeText: print(before) + "\n", mutationText: print(mutation) + "\n", outcome: { status: "rejected", code: "mutation.duplicate-id", path: [donor] } });
    console.log(`${kind}: donor ${donor}`);
  }
}

function restates() {
  for (const [kind, collection] of SETS) {
    const base = firstApplied(kind);
    const before = parse(read(kind, base, "before"));
    const mutation = parse(read(kind, base, "mutation"));
    if (before.k !== "obj" || mutation.k !== "obj") throw new Error("documents are no objects");
    const id = field(mutation, "id");
    const record = collection ? field(field(before, collection)!, JSON.parse((id as { v: string }).v)) : field(before, "project");
    if (!record || record.k !== "obj") throw new Error(`${kind}: record not found`);
    const named = new Set(mutation.v.map(([key]) => key));
    const extra = record.v.find(([key]) => !named.has(key) && !["storey", "building", "site", "host"].includes(key));
    if (!extra) {
      console.log(`${kind}: no field left to restate`);
      continue;
    }
    mutation.v.push(extra);
    addCase(kind, "restates-an-unchanged-field", { beforeText: read(kind, base, "before"), mutationText: print(mutation) + "\n", outcome: { status: "applied" } });
    console.log(`${kind}: restates ${extra[0]}`);
  }
}

function property() {
  const kind = "set-element-property";
  const old = caseDirs(kind).find((dir) => dir.endsWith("mismatched-value"));
  if (old) removeCase(kind, old);
  const base = firstApplied(kind);
  const mutation = parse(read(kind, base, "mutation"));
  if (mutation.k !== "obj") throw new Error("mutation is no object");
  mutation.v.find(([key]) => key === "property")![1] = string_("");
  addCase(kind, "blank-property", { beforeText: read(kind, base, "before"), mutationText: print(mutation) + "\n", outcome: { status: "rejected", code: "mutation.invariant", path: ["property"] } });
}

const mode = process.argv[2];
if (mode === "duplicates") duplicates();
else if (mode === "restates") restates();
else if (mode === "property") property();
else throw new Error("usage: duplicates | restates | property");
