/** 🪗️ `used-accordion`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-accordion.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect } from "./lib.ts";


const script: Script = {
  id: "used-accordion",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the case keeps its frame of 18 mm boards as one hollow part judged by its overall extents, so the instrument still shows in the open case, with the back as a declared 18 mm sheet; the lid lying before it is thickened from 29 to 75 mm, the handle is one 75 mm block that keeps the height; the accordion is three blocks: treble box with the keyboard, the four bellows segments as one, bass box with button field and strap; the grille is below the feature and left out.", de: "Körper für das 75-mm-Merkmal: der Koffer behält seinen Rahmen aus 18-mm-Brettern als ein Hohlteil, nach seinen Gesamtmaßen beurteilt, damit das Instrument im offenen Koffer sichtbar bleibt, mit der Rückwand als erklärter 18-mm-Platte; der davor liegende Deckel ist von 29 auf 75 mm verdickt, der Griff ein 75-mm-Block, der die Höhe hält; das Akkordeon sind drei Blöcke: Diskantgehäuse mit der Tastatur, die vier Balgsegmente als eines, Bassgehäuse mit Knopffeld und Riemen; das Gitter liegt unter dem Merkmal und entfällt." },
        { en: "Thermal: the body solids in their regions by reference; grille, keyboard, bass buttons and bass strap as shells of their layer at true size.", de: "Thermik: die Körper in ihren Regionen als Verweis; Gitter, Tastatur, Bassknöpfe und Bassriemen als Schalen ihrer Schicht in wahrer Größe." },
        { en: "Structure: the members by reference, the bellows block spanning the four bellows bones, the lid hinge as a 75 mm joint and five mass markers.", de: "Tragwerk: die Stäbe als Verweis, der Balgblock überspannt die vier Balgknochen, das Deckelscharnier als 75-mm-Gelenk und fünf Massenmarken." },
      ],
      body: [
        ext(["case-frame", "case", "case-tolex", 0.22], "xy", rect(0, 0.5, 0, 0.51), -0.11, 0.135, { holes: [rect(0.018, 0.482, 0.018, 0.492)] }),
        box(["case-back", "case", "case-tolex", 0.22], [0.464, 0.474, 0.018], [0.25, 0.255, 0.126], { sheet: true }),
        box(["lid-panel", "lid", "case-tolex", 0.24], [0.5, 0.075, 0.511], [0.25, 0.0255, -0.2425]),
        box(["handle-block", "handle", "leather", 0.3], [0.13, 0.075, 0.075], [0.065, -0.0025, 0]),
        box(["treble-box", "treble", "celluloid", 0.38], [0.14, 0.466, 0.19], [-0.07, -0.005, 0]),
        box(["bellows", "bellows-1", "bellows-card", 0.13], [0.2, 0.44, 0.175], [0.1, 0, 0]),
        box(["bass-box", "bass", "celluloid", 0.38], [0.168, 0.466, 0.19], [0.084, -0.005, 0]),
      ],
      thermal: [
        of("th-case-frame", "case-frame", { region: "case-shell" }),
        of("th-handle-block", "handle-block", { region: "case-shell" }),
        of("th-case-back", "case-back", { region: "case-lining" }),
        of("th-lid-panel", "lid-panel", { region: "case-lid" }),
        of("th-treble-box", "treble-box", { region: "treble-casing" }),
        of("th-bellows", "bellows", { region: "bellows" }),
        of("th-bass-box", "bass-box", { region: "bass-casing" }),
        box(["th-grille", "treble", "plated-steel", 0.78], [0.073, 0.366, 0.003], [-0.0585, 0, -0.0965], { region: "grille", layer: "grille-sheet", sheet: true }),
        box(["th-keyboard", "treble", "key-plastic", 0.8], [0.035, 0.43, 0.125], [-0.1225, 0, -0.0275], { region: "keyboard", layer: "key-tops" }),
        box(["th-bass-buttons", "bass", "key-plastic", 0.72], [0.028, 0.384, 0.094], [0.154, -0.003, -0.018], { region: "bass-buttons", layer: "button" }),
        box(["th-bass-strap", "bass", "leather", 0.3], [0.028, 0.384, 0.051], [0.154, -0.003, 0.0545], { region: "straps", layer: "strap-leather" }),
      ],
      structure: [
        of("st-case-frame", "case-frame", { member: "case", part: "member" }),
        of("st-case-back", "case-back", { member: "case", part: "member" }),
        of("st-lid-panel", "lid-panel", { member: "lid", part: "member" }),
        of("st-handle-block", "handle-block", { member: "handle", part: "member" }),
        of("st-treble-box", "treble-box", { member: "treble", part: "member" }),
        of("st-bellows", "bellows", { member: "bellows-1", part: "member", members: ["bellows-2", "bellows-3", "bellows-4"] }),
        of("st-bass-box", "bass-box", { member: "bass", part: "member" }),
        box(["j-lid-hinge", "lid", "corner-steel", 0.3, "detail"], [0.5, 0.075, 0.075], [0.25, 0.0255, 0], { member: "lid", part: "joint" }),
        box(["mass-case", "case", "case-tolex", 0.3, "detail"], [0.075, 0.075, 0.075], [0.25, 0.255, 0.0975], { member: "case", part: "mass" }),
        box(["mass-lid", "lid", "case-tolex", 0.3, "detail"], [0.075, 0.075, 0.075], [0.25, 0.0255, -0.24], { member: "lid", part: "mass" }),
        box(["mass-treble", "treble", "celluloid", 0.3, "detail"], [0.075, 0.075, 0.075], [-0.05, 0, 0], { member: "treble", part: "mass" }),
        box(["mass-bellows", "bellows-1", "bellows-card", 0.3, "detail"], [0.075, 0.075, 0.075], [0.1, 0, 0], { member: "bellows-1", part: "mass" }),
        box(["mass-bass", "bass", "celluloid", 0.3, "detail"], [0.075, 0.075, 0.075], [0.07, 0, 0], { member: "bass", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
