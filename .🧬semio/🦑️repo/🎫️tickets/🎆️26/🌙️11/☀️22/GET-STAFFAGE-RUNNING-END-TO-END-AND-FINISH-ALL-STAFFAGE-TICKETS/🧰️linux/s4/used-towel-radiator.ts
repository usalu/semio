/** 🪜️ `used-towel-radiator`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-towel-radiator.ts <asset document> [--force] */
import { type Script, box, ext, main, of } from "./lib.ts";

const cube = [0.075, 0.075, 0.075] as const;
const frame = [[-0.45, 0], [0.45, 0], [0.45, 0.075], [0.26, 0.075], [0.0375, 1.3], [-0.0375, 1.3], [-0.26, 0.075], [-0.45, 0.075]] as const;
const opening = [[-0.15, 0.1], [0.15, 0.1], [0, 0.95]] as const;
const rail = [[0.0375, 0.0305], [0.0155, 0.0525], [-0.0155, 0.0525], [-0.0375, 0.0305], [-0.0375, -0.0005], [-0.0155, -0.0225], [0.0155, -0.0225], [0.0375, -0.0005]] as const;
const across = { repeat: { count: 2, translation: [0, 0, 0.465] } };

const script: Script = {
  id: "used-towel-radiator",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: each end of the A-frame rack is one 75 mm plate of its outline (foot bar and both legs) with the opening between the legs kept as a hole, joined by a 75 mm ridge; the sill of each face is one 75 mm block of its true length (the cross tube is left out), each edge strip a 75 mm bar from frame to frame; the radiator keeps its two uprights at their true section as slender parts and its rails as five octagonal 75 mm bars between them, the typified rails of 1:200 thickened about their axes and declared as the envelope of the row; the valve is a 75 mm block; brackets are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: jedes Ende des A-Bock-Gestells ist eine 75-mm-Platte seines Umrisses (Fußholm und beide Beine) mit der Öffnung zwischen den Beinen als Loch, verbunden durch einen 75-mm-First; die Schwelle jeder Seite ist ein 75-mm-Block ihrer wahren Länge (das Querrohr entfällt), jede Kantenleiste ein 75-mm-Stab von Rahmen zu Rahmen; der Heizkörper behält seine zwei Holme im wahren Querschnitt als schlanke Teile und seine Sprossen als fünf achteckige 75-mm-Stäbe dazwischen, die typisierten Sprossen von 1:200, um ihre Achsen verdickt und als Hülle der Reihe erklärt; das Ventil ist ein 75-mm-Block; Konsolen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: sills, uprights, rails and valve by reference; the rack and the edge strips as shells of their layer at the true 40 mm tube and strip section, so the outer area stays that of the tubes; the four brackets as shells of their layer at true size and a 75 mm plume patch at the top rail.", de: "Thermik: Schwellen, Holme, Sprossen und Ventil als Verweis; das Gestell und die Kantenleisten als Schalen ihrer Schicht im wahren 40-mm-Rohr- und Leistenquerschnitt, damit die Außenfläche die der Rohre bleibt; die vier Konsolen als Schalen ihrer Schicht in wahrer Größe und eine 75-mm-Auftriebsfläche an der obersten Sprosse." },
        { en: "Structure: the members by reference, the sill bearing as a 75 mm joint and two mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Schwellenlager als 75-mm-Gelenk und zwei Massenmarken." },
      ],
      body: [
        ext(["end-frames", "rack", "painted-steel", 0.45], "xy", frame, -0.42, -0.345, { holes: [opening], repeat: { count: 2, translation: [0, 0, 0.765] } }),
        box(["ridge", "rack", "painted-steel", 0.45], [0.075, 0.075, 0.69], [0, 1.2625, 0]),
        box(["sills", "rack", "softwood", 0.62], [0.075, 0.2, 0.36], [0.2375, 0.1, 0], { repeat: { count: 2, translation: [-0.475, 0, 0] } }),
        box(["edge-strips", "rack", "epdm", 0.15], [0.075, 0.075, 0.69], [0.089, 0.8, 0], { repeat: { count: 2, translation: [-0.178, 0, 0] } }),
        box(["upright-right", "radiator", "chrome-steel", 0.86], [0.03, 1.2, 0.035], [0, 0.55, -0.2325], { slender: true }),
        box(["upright-left", "radiator", "chrome-steel", 0.86], [0.03, 1.2, 0.035], [0, 0.55, 0.2325], { slender: true }),
        ext(["rails", "rails", "chrome-steel", 0.82], "xy", rail, -0.215, 0.215, { envelope: true, repeat: { count: 5, translation: [0, 0.26925, 0] } }),
        box(["valve-body", "radiator", "chrome-brass", 0.78], cube, [-0.015, -0.0875, 0.2495]),
      ],
      thermal: [
        ext(["th-end-frames", "rack", "painted-steel", 0.45], "xy", frame, -0.42, -0.38, { holes: [opening], repeat: { count: 2, translation: [0, 0, 0.8] }, region: "rack", layer: "steel" }),
        box(["th-ridge", "rack", "painted-steel", 0.45], [0.04, 0.04, 0.76], [0, 1.28, 0], { region: "rack", layer: "steel" }),
        of("th-sills", "sills", { region: "sills" }),
        box(["th-edge-strips", "rack", "epdm", 0.15], [0.062, 0.04, 0.76], [0.089, 0.8, 0], { repeat: { count: 2, translation: [-0.178, 0, 0] }, region: "edge-strips", layer: "rubber" }),
        of("th-upright-right", "upright-right", { region: "upright-return" }),
        of("th-upright-left", "upright-left", { region: "upright-flow" }),
        of("th-rails", "rails", { region: "rails" }),
        of("th-valve-body", "valve-body", { region: "valve" }),
        box(["th-brackets-lower", "radiator", "chrome-steel", 0.74], [0.05, 0.03, 0.045], [-0.04, 0.15, -0.2325], { ...across, region: "brackets", layer: "zinc" }),
        box(["th-brackets-upper", "radiator", "chrome-steel", 0.74], [0.05, 0.03, 0.045], [-0.04, 1.014, -0.2325], { ...across, region: "brackets", layer: "zinc" }),
        box(["th-radiator-plume", "rails", "chrome-steel", 0.82, "detail"], [0.075, 0.075, 0.43], [0, 1.1445, 0], { region: "rails", tags: ["radiator-plume"] }),
      ],
      structure: [
        of("member-end-frames", "end-frames", { member: "rack", part: "member" }),
        of("member-ridge", "ridge", { member: "rack", part: "member" }),
        of("member-sills", "sills", { member: "rack", part: "member" }),
        of("member-edge-strips", "edge-strips", { member: "rack", part: "member" }),
        of("member-upright-right", "upright-right", { member: "radiator", part: "member" }),
        of("member-upright-left", "upright-left", { member: "radiator", part: "member" }),
        of("member-rails", "rails", { member: "rails", part: "member" }),
        box(["joint-sill-bearing", "radiator", "chrome-steel", 0.5, "detail"], cube, [0, 0.0375, 0], { member: "radiator", part: "joint" }),
        box(["mass-rack", "rack", "painted-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0, 1.2625, 0], { member: "rack", part: "mass" }),
        box(["mass-radiator", "radiator", "chrome-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0, 0.55, 0], { member: "radiator", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
