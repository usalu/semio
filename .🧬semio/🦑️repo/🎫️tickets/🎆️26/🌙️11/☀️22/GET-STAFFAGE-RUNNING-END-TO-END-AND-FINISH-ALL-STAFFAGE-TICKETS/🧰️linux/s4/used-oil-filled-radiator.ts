/** ♨️ `used-oil-filled-radiator`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-oil-filled-radiator.ts <asset document> [--force] */
import { type Script, box, ext, main, of } from "./lib.ts";

const fin = [[0, -0.055], [0.02, -0.075], [0.548, -0.075], [0.568, -0.055], [0.568, 0.055], [0.548, 0.075], [0.02, 0.075], [0, 0.055]] as const;
const twice = { repeat: { count: 2, translation: [0, 0, 0.252] } };

const script: Script = {
  id: "used-oil-filled-radiator",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the fin row is one envelope prism of the fin outline over the true stack length (it stands for the seven typified fins of 1:200, whose 25 mm gaps no longer print), the control housing takes the cord plate in and is one 75 mm slab on the stack end, each foot bar with its two castors is one 75 mm rail from the pad to the fins, and the pad is its declared 12 mm sheet; knob, hubs and castor wheels are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Rippenreihe ist ein Hüllprisma des Rippenumrisses über die wahre Stapellänge (es steht für die sieben typisierten Rippen von 1:200, deren 25-mm-Spalte nicht mehr drucken), das Bediengehäuse nimmt die Kabelplatte auf und ist eine 75-mm-Scheibe am Stapelende, jede Fußschiene mit ihren zwei Rollen ist eine 75-mm-Kufe von der Unterlage bis zu den Rippen, und die Unterlage ist ihre erklärte 12-mm-Platte; Knopf, Naben und Rollenräder liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: pad, fins, housing and rails by reference, the castors as four 75 mm blocks, knob and cord as 75 mm blocks on the housing and a 75 mm plume patch in the top of the stack.", de: "Thermik: Unterlage, Rippen, Gehäuse und Kufen als Verweis, die Rollen als vier 75-mm-Blöcke, Knopf und Kabel als 75-mm-Blöcke am Gehäuse und eine 75-mm-Auftriebsfläche oben im Stapel." },
        { en: "Structure: the members by reference, castor bars and the knob as 75 mm members, the clamps, the housing screws and the knob spindle as 75 mm joints, four 75 mm pad supports and three mass markers.", de: "Tragwerk: die Stäbe als Verweis, Rollenleisten und der Knopf als 75-mm-Stäbe, die Klemmen, die Gehäuseschrauben und die Knopfachse als 75-mm-Gelenke, vier 75-mm-Auflager der Unterlage und drei Massenmarken." },
      ],
      body: [
        box(["pad", "blanket", "felt-pad", 0.3], [0.45, 0.012, 0.6], [0.225, 0, 0], { sheet: true }),
        ext(["fins", "fins", "enamelled-steel", 0.82], "xy", fin, -0.179, 0.179, { envelope: true }),
        box(["housing", "housing", "housing-plastic", 0.88], [0.525, 0.12, 0.075], [0.2625, 0, -0.005]),
        box(["rails", "feet", "painted-steel", 0.45], [0.25, 0.075, 0.075], [0.125, -0.0245, -0.126], twice),
      ],
      thermal: [
        of("th-pad", "pad", { region: "pad" }),
        of("th-fins", "fins", { region: "fins" }),
        of("th-housing", "housing", { region: "housing" }),
        of("th-rails", "rails", { region: "feet" }),
        box(["th-castors-right", "castors", "castor-polyamide", 0.55], [0.075, 0.075, 0.075], [0, 0.0175, -0.126], { repeat: { count: 2, translation: [0.2, 0, 0] }, region: "castors" }),
        box(["th-castors-left", "castors", "castor-polyamide", 0.55], [0.075, 0.075, 0.075], [0, 0.0175, 0.126], { repeat: { count: 2, translation: [0.2, 0, 0] }, region: "castors" }),
        box(["th-knob", "knob", "knob-plastic", 0.3], [0.075, 0.075, 0.075], [0, 0, 0], { region: "knob" }),
        box(["th-cord", "housing", "pvc-cord", 0.25], [0.19, 0.075, 0.075], [0.155, 0, -0.043], { region: "cord" }),
        box(["th-plume", "fins", "enamelled-steel", 0.6, "detail"], [0.075, 0.075, 0.3], [0.5305, 0, 0], { region: "fins", tags: ["plume"] }),
      ],
      structure: [
        of("m-pad", "pad", { member: "blanket", part: "member" }),
        of("m-fins", "fins", { member: "fins", part: "member" }),
        of("m-housing", "housing", { member: "housing", part: "member" }),
        of("m-rails", "rails", { member: "feet", part: "member" }),
        box(["m-castors", "castors", "castor-polyamide", 0.55], [0.275, 0.075, 0.075], [0.1, 0.0175, -0.126], { ...twice, member: "castors", part: "member" }),
        box(["m-knob", "knob", "knob-plastic", 0.3], [0.075, 0.075, 0.075], [0, 0, 0], { member: "knob", part: "member" }),
        box(["j-clamps", "feet", "painted-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.125, 0.021, -0.126], { ...twice, member: "feet", part: "joint" }),
        box(["j-housing-screws", "housing", "painted-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.25, 0, 0.033], { member: "housing", part: "joint" }),
        box(["j-knob-spindle", "knob", "painted-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0, 0, 0.0375], { member: "knob", part: "joint" }),
        box(["support-pad-floor-fl", "blanket", "felt-pad", 0.2, "ground"], [0.075, 0.075, 0.075], [0.405, 0.0315, 0.25], { member: "blanket", part: "support", anchor: "pad-floor-fl" }),
        box(["support-pad-floor-fr", "blanket", "felt-pad", 0.2, "ground"], [0.075, 0.075, 0.075], [0.405, 0.0315, -0.25], { member: "blanket", part: "support", anchor: "pad-floor-fr" }),
        box(["support-pad-floor-rl", "blanket", "felt-pad", 0.2, "ground"], [0.075, 0.075, 0.075], [0.045, 0.0315, 0.25], { member: "blanket", part: "support", anchor: "pad-floor-rl" }),
        box(["support-pad-floor-rr", "blanket", "felt-pad", 0.2, "ground"], [0.075, 0.075, 0.075], [0.045, 0.0315, -0.25], { member: "blanket", part: "support", anchor: "pad-floor-rr" }),
        box(["mass-fins", "fins", "enamelled-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.269, 0, 0], { member: "fins", part: "mass" }),
        box(["mass-housing", "housing", "housing-plastic", 0.3, "detail"], [0.075, 0.075, 0.075], [0.268, 0, 0], { member: "housing", part: "mass" }),
        box(["mass-feet", "feet", "enamelled-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.125, 0.042, 0], { member: "feet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
