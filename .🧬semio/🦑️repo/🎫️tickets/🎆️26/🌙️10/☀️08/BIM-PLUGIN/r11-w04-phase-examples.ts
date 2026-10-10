/**
 * 🎭️ Wave R11 `w04-phase`: the construction phases and the one storey move of the `house` and `office` examples, applied to the snapshot sources by `r4-x-examples-gen.ts`.
 * House: the basement is the existing building, the WC partition of the ground floor is demolished, everything else is new. Office: the ground-floor frame (columns, beams, slab) is existing and the north
 * curtain wall of the ground floor is demolished. `MOVES` names the elements an example authors on another storey than the one they end up on: the replay (`checks::replay_derived`) creates them on `from` and moves
 * them with `set-element-storey`, so the committed snapshot and the mutation both prove the storey move. `phasesFor` refuses an id that is no element and a move that would leave an opening behind.
 */
type Json = Record<string, any>;

const PHASES: Record<string, Record<string, Record<string, string[]>>> = {
  house: {
    Existing: {
      walls: ["w-b-south", "w-b-east-1", "w-b-bay", "w-b-east-2", "w-b-north", "w-b-west", "w-b-spine"],
      slabs: ["sl-b"],
      columns: ["c-b-support"],
      beams: ["bm-b-support"],
      stairs: ["sr-cellar"],
      spaces: ["sp-b1", "sp-b2"],
    },
    Demolished: { walls: ["w-g-wc"] },
  },
  office: {
    Existing: {
      columns: ["A", "B", "C", "D", "E", "F"].flatMap((letter) => [1, 2, 3, 4].map((row) => `c-0-${letter}${row}`)),
      slabs: ["sl-0"],
    },
    Demolished: { curtain_walls: ["cu-0-north"] },
  },
};

/** 🪜️ The elements an example authors on `from` and ends up with on their snapshot storey. */
export const MOVES: Record<string, { id: string; from: string }[]> = {
  house: [{ id: "w-u-bath-east", from: "st-ground" }],
  office: [],
};

const PHASED = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces"];

export function phasesFor(name: string, model: Json) {
  for (const collection of PHASED) {
    for (const record of Object.values<Json>(model[collection] ?? {})) record.phase ??= "New";
  }
  for (const [phase, collections] of Object.entries(PHASES[name] ?? {})) {
    for (const [collection, ids] of Object.entries(collections)) {
      for (const id of ids) {
        const record = model[collection]?.[id];
        if (!record) throw new Error(`${name}: no ${collection}/${id} to put in the ${phase} phase`);
        record.phase = phase;
      }
    }
  }
  for (const { id, from } of MOVES[name] ?? []) {
    const record = model.walls[id];
    if (!record) throw new Error(`${name}: no wall ${id} to move`);
    if (!model.storeys[from]) throw new Error(`${name}: no storey ${from}`);
    if (record.storey === from) throw new Error(`${name}: ${id} already stands on ${from}`);
    if (Object.values<Json>(model.openings).some((opening) => opening.host === id)) throw new Error(`${name}: ${id} hosts openings and cannot be authored on another storey first`);
  }
}
