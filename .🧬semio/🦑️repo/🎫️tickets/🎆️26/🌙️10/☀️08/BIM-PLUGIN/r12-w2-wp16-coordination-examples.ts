/**
 * 🤝️ WP-16 `w2-wp16-coordination`: the coordination set of the `house` and `office` examples, merged into the snapshot by `r4-x-examples-gen.ts` after the sheets. `coordinationFor(name, model)` writes the
 * authored records a team keeps for coordination: clash sets (the structure against the walls, the services against the structure with a clearance), the rule set of the typical German limits and
 * one issue raised from the model with a comment. Nothing derived is written: the clashes and the rule findings are the inferences `clash-sets` and `rule-results`.
 *
 * Limits (rule of the closest standard, metres unless noted): riser at most 0.19 (DIN 18065, residential stairs), tread at least 0.26, stair width at least 1.00 (MBO section 34), door clear width at
 * least 0.90 (DIN 18040-2), ramp slope at most 6 % (DIN 18040-1), clear height of rooms at least 2.40 (MBO, residential) and corridors at least 1.20 wide (ASR A1.8), a compartment of at most 400 square
 * metres net floor area (MBO use units).
 *
 * DIN 18065: <https://www.din.de>; MBO: <https://www.bauministerkonferenz.de>.
 */
type Json = Record<string, any>;

const sel = (over: Json = {}) => ({ classes: [], storeys: [], phases: [], ids: [], ...over });
const scope = (over: Json = {}) => ({ storeys: [], phases: [], ids: [], filter: "", ...over });
const rule = (name: string, kind: string, limit: number, severity: string, over: Json = {}) => ({ name, kind, limit, severity, scope: scope(over) });
const DATE = "2026-10-09T08:00:00Z";

const rulesFor = (name: string) => ({
  "rl-riser": rule("Riser height (DIN 18065)", "MaxRiser", 0.19, "Error"),
  "rl-tread": rule("Tread depth (DIN 18065)", "MinTread", 0.26, "Error"),
  "rl-stair-width": rule("Stair width (MBO 34)", "MinStairWidth", 1.0, "Warning"),
  "rl-door-width": rule("Door clear width (DIN 18040-2)", "MinDoorWidth", 0.9, "Warning"),
  "rl-clear-height": rule("Clear height of rooms (MBO)", "MinClearHeight", 2.4, "Error"),
  ...(name === "house" ? { "rl-ramp": rule("Ramp slope 6 % (DIN 18040-1)", "MaxRampSlope", 0.06, "Error") } : {}),
  ...(name === "office"
    ? {
        "rl-corridor": rule("Corridor width (ASR A1.8)", "MinCorridorWidth", 1.2, "Warning", { filter: "Circulation" }),
        "rl-compartment": rule("Use unit area (MBO)", "MaxCompartmentArea", 400, "Note"),
      }
    : {}),
});

const clashSetsFor = (name: string, model: Json) => ({
  "cs-structure": { name: "Structure against walls", a: sel({ classes: ["Column", "Beam", "Slab"] }), b: sel({ classes: ["Wall"] }), tolerance: 0.002, clearance: 0 },
  ...(name === "house"
    ? { "cs-stairs": { name: "Stairs against structure", a: sel({ classes: ["Stair"] }), b: sel({ classes: ["Wall", "Column", "Beam", "Slab"] }), tolerance: 0.005, clearance: 0 } }
    : {
        "cs-services": { name: "Services against structure", a: sel({ classes: ["Mep"] }), b: sel({ classes: ["Beam", "Column", "Slab", "Wall"] }), tolerance: 0.005, clearance: 0.05 },
        "cs-frame": { name: "Beams against columns", a: sel({ classes: ["Beam"] }), b: sel({ classes: ["Column"] }), tolerance: 0.002, clearance: 0 },
        "cs-facade": { name: "Curtain walls against structure", a: sel({ classes: ["CurtainWall"] }), b: sel({ classes: ["Column", "Beam", "Slab"] }), tolerance: 0.005, clearance: 0.01 },
      }),
});

/** 🚩️ The issue of an example: raised on one stair or beam with a viewpoint on the building and one comment. */
function issuesFor(name: string, model: Json) {
  const wallId = Object.keys(model.walls)[0];
  const subject = name === "house" ? Object.keys(model.stairs)[0] : Object.keys(model.beams)[0];
  const storeyIds = Object.keys(model.storeys);
  const camera = name === "house" ? { target: { x: 5, y: 4 }, target_height: 3, azimuth: 0.9, pitch: 0.55, distance: 22 } : { target: { x: 9, y: 6 }, target_height: 6, azimuth: 0.9, pitch: 0.5, distance: 40 };
  const set = name === "house" ? "cs-stairs" : "cs-structure";
  return {
    issues: {
      "is-1": {
        title: name === "house" ? "Check the headroom above the main stair" : "Check the beam bearing at the south wall",
        description: name === "house" ? "The stair rises towards the slab above; confirm 2.00 m of headroom before the stair is built." : "The beam is modelled to bear on the wall; confirm the bearing length with the structural engineer.",
        status: "Open",
        priority: "High",
        assignee: "structure",
        author: "semio",
        created: DATE,
        labels: ["Coordination", name === "house" ? "Stair" : "Structure"],
        elements: [subject, wallId],
        clash: { set, first: [subject, wallId].sort()[0], second: [subject, wallId].sort()[1] },
        viewpoint: { camera, section: { min: { x: -1, y: -1, z: -3.5 }, max: { x: 16, y: 13, z: 9 } }, isolate: [subject, wallId] },
      },
    },
    issue_comments: {
      "ic-1": { issue: "is-1", author: "structure", date: "2026-10-09T09:30:00Z", text: "Will check with the next design iteration." },
      "ic-2": { issue: "is-1", author: "semio", date: "2026-10-09T10:15:00Z", text: "Thanks. Section box and isolation are saved with the viewpoint." },
    },
    storeyIds,
  };
}

export function coordinationFor(name: string, model: Json) {
  const { storeyIds: _storeys, ...raised } = issuesFor(name, model);
  return { clash_sets: clashSetsFor(name, model), rules: rulesFor(name), ...raised };
}
