/** 🍲️ `used-microwave-oven`: the handcrafted volumes of 1:200 and 1:500, simplified by hand from its 1:100 volume. usage: used-microwave-oven.ts <asset document> [--force] */
import { type Script, box, ell, ext, main, of, rect, rev } from "./lib.ts";

const pad = box(["pad", "pad", "corrugated-board", 0.62], [0.48, 0.008, 0.395], [0.24, 0.004, 0.0075], { sheet: true });
const window = [[0.0465, -0.045], [0.0465, -0.265], [0.1985, -0.265], [0.1985, -0.045]] as const;
const door = [[0, 0], [0, -0.3185], [0.245, -0.3185], [0.245, 0]] as const;
const cavity = [[0.006, 0.024], [0.329, 0.024], [0.329, 0.244], [0.006, 0.244]] as const;
const lining = [[0.0005, 0.0005], [0.3145, 0.0005], [0.3145, 0.2115], [0.0005, 0.2115]] as const;
const vents = box(["th-exhaust-vents", "side-r", "painted-steel", 0.45], [0.08, 0.0005, 0.12], [0.179, -0.0005, 0], { region: "casing", tags: ["exhaust"], sheet: true });

const script: Script = {
  id: "used-microwave-oven",
  volumes: {
    "200": {
      status: "authored",
      denominator: 200,
      decisions: [
        { en: "Body for the 30 mm feature: the housing shell runs from the pad to the top (the 10 mm feet and the technical bay merge into it) and keeps the cavity as its opening, closed by a 30 mm back; the door keeps its 35 mm frame with the window and the 4 mm glass as a declared sheet, the fascia is its 35 mm block and the turntable one 30 mm disc; knobs, plug, cord, button and hinge pins are below the feature and left out.", de: "Körper für das 30-mm-Merkmal: die Gehäuseschale reicht von der Unterlage bis zur Oberseite (die 10-mm-Füße und der Technikraum gehen in ihr auf) und behält den Garraum als Öffnung, geschlossen von einer 30-mm-Rückwand; die Tür behält ihren 35-mm-Rahmen mit Fenster und das 4-mm-Glas als erklärte Platte, die Blende ist ihr 35-mm-Block und der Drehteller eine 30-mm-Scheibe; Knöpfe, Stecker, Kabel, Taste und Scharnierzapfen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference, the cavity lining with its back sheet, the exhaust patch and one iron block for the hidden electronics.", de: "Thermik: die Körper in ihren Regionen als Verweis, die Garraumauskleidung mit ihrer Rückwand, die Abluftfläche und ein Eisenblock für die verdeckte Elektronik." },
        { en: "Structure: the members by reference, the back also holding the cord bone that runs 20 mm behind it, a 30 mm pad support and the mass marker.", de: "Tragwerk: die Stäbe als Verweis, die Rückwand fasst auch den Kabelknochen, der 20 mm hinter ihr läuft, ein 30-mm-Auflager der Unterlage und die Massenmarke." },
      ],
      body: [
        pad,
        ext(["shell", "housing", "painted-steel", 0.86], "xy", rect(0, 0.44, 0, 0.258), -0.135, 0.135, { holes: [cavity] }),
        box(["back", "housing", "painted-steel", 0.8], [0.44, 0.258, 0.03], [0.22, 0.129, 0.15]),
        rev(["plate", "turntable", "borosilicate", 0.85], [1, 0, 0], [0, 0, 0], [[-0.004, 0], [-0.004, 0.1225], [0.026, 0.1225], [0.026, 0]]),
        ext(["door-frame", "door", "abs-aged", 0.92], "xy", door, -0.035, 0, { holes: [window] }),
        box(["glass", "door", "door-glass", 0.3], [0.152, 0.22, 0.004], [0.1225, -0.155, -0.023], { sheet: true }),
        box(["fascia", "panel", "abs-aged", 0.9], [0.247, 0.12, 0.035], [0.1235, 0, 0]),
      ],
      thermal: [
        of("th-pad", "pad", { region: "pad" }),
        of("th-shell", "shell", { region: "casing" }),
        of("th-back", "back", { region: "casing" }),
        ext(["th-cavity-lining", "cavity", "cavity-paint", 0.72], "xy", rect(0, 0.315, 0, 0.212), -0.135, 0.135, { holes: [lining], region: "cavity" }),
        box(["th-cavity-back", "cavity", "cavity-paint", 0.66], [0.315, 0.212, 0.0005], [0.1575, 0.106, 0.13475], { region: "cavity", sheet: true }),
        of("th-plate", "plate", { region: "turntable" }),
        of("th-door-frame", "door-frame", { region: "door" }),
        of("th-glass", "glass", { region: "window" }),
        of("th-fascia", "fascia", { region: "panel" }),
        vents,
        box(["th-electronics", "housing", "electrical-iron", 0.3], [0.09, 0.2, 0.1], [0.38, 0.13, 0.04], { region: "electronics" }),
      ],
      structure: [
        of("m-pad", "pad", { member: "pad", part: "member" }),
        of("m-shell", "shell", { member: "housing", part: "member", members: ["base", "side-l", "side-r", "top", "cavity"] }),
        of("m-back", "back", { member: "housing", part: "member", members: ["cord"] }),
        of("m-plate", "plate", { member: "turntable", part: "member" }),
        of("m-door-frame", "door-frame", { member: "door", part: "member" }),
        of("m-fascia", "fascia", { member: "panel", part: "member" }),
        box(["support-pad-floor", "pad", "corrugated-board", 0.2, "ground"], [0.4, 0.03, 0.3], [0.24, 0.015, 0.0075], { member: "pad", part: "support", anchor: "pad-floor" }),
        ell(["mass-housing", "housing", "electrical-iron", 0.3, "detail"], [0.02, 0.02, 0.02], [0.270328, 0.102376, 0.02], { member: "housing", part: "mass" }),
      ],
    },
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: on the pad as its declared 8 mm sheet the housing is one block from the pad to the top (feet, shell and back merged) behind a front of 75 mm parts: the door as its frame with the window as a glass block set 10 mm back, and the fascia block; turntable, knobs, plug and cord are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: auf der Unterlage als erklärter 8-mm-Platte ist das Gehäuse ein Block von der Unterlage bis zur Oberseite (Füße, Schale und Rückwand verschmolzen) hinter einer Front aus 75-mm-Teilen: die Tür als ihr Rahmen mit dem Fenster als 10 mm zurückgesetztem Glasblock und der Blendenblock; Drehteller, Knöpfe, Stecker und Kabel liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the block split into its regions at 75 mm: the casing as a tube around the cavity with a 75 mm back, door frame and fascia by reference with the glass as its declared 4 mm sheet, the lining as one tube closed by the casing back, the turntable as a 75 mm disc, the exhaust patch and one iron block for the hidden electronics.", de: "Thermik: der Block in seine Regionen zu 75 mm geteilt: das Gehäuse als Rohr um den Garraum mit 75-mm-Rückwand, Türrahmen und Blende als Verweis mit dem Glas als erklärter 4-mm-Platte, die Auskleidung als ein von der Gehäuserückwand geschlossenes Rohr, der Drehteller als 75-mm-Scheibe, die Abluftfläche und ein Eisenblock für die verdeckte Elektronik." },
        { en: "Structure: pad, block, door with glass and fascia by reference, the turntable as a 75 mm member, a 75 mm pad support and the mass marker.", de: "Tragwerk: Unterlage, Block, Tür mit Glas und Blende als Verweis, der Drehteller als 75-mm-Stab, ein 75-mm-Auflager der Unterlage und die Massenmarke." },
      ],
      body: [
        pad,
        box(["block", "housing", "painted-steel", 0.86], [0.44, 0.258, 0.26], [0.22, 0.129, 0.035]),
        ext(["door-frame", "door", "abs-aged", 0.92], "xy", door, -0.035, 0.04, { holes: [window] }),
        box(["glass", "door", "door-glass", 0.3], [0.152, 0.22, 0.065], [0.1225, -0.155, 0.0075]),
        box(["fascia", "panel", "abs-aged", 0.9], [0.247, 0.12, 0.075], [0.1235, 0, 0.02]),
      ],
      thermal: [
        of("th-pad", "pad", { region: "pad" }),
        ext(["th-casing", "housing", "painted-steel", 0.86], "xy", rect(0, 0.44, 0, 0.258), -0.095, 0.09, { holes: [cavity], region: "casing" }),
        box(["th-back", "housing", "painted-steel", 0.8], [0.44, 0.258, 0.075], [0.22, 0.129, 0.1275], { region: "casing" }),
        ext(["th-cavity-lining", "cavity", "cavity-paint", 0.72], "xy", rect(0, 0.315, 0, 0.212), -0.095, 0.09, { holes: [lining], region: "cavity" }),
        rev(["th-plate", "turntable", "borosilicate", 0.85], [1, 0, 0], [0, 0, 0], [[-0.004, 0], [-0.004, 0.1225], [0.071, 0.1225], [0.071, 0]], { region: "turntable" }),
        of("th-door-frame", "door-frame", { region: "door" }),
        box(["th-glass", "door", "door-glass", 0.3], [0.152, 0.22, 0.004], [0.1225, -0.155, -0.023], { region: "window", sheet: true }),
        of("th-fascia", "fascia", { region: "panel" }),
        vents,
        box(["th-electronics", "housing", "electrical-iron", 0.3], [0.09, 0.2, 0.1], [0.38, 0.13, 0.04], { region: "electronics" }),
      ],
      structure: [
        of("m-pad", "pad", { member: "pad", part: "member" }),
        of("m-block", "block", { member: "housing", part: "member", members: ["base", "side-l", "side-r", "top", "cavity", "cord"] }),
        rev(["m-plate", "turntable", "borosilicate", 0.85], [1, 0, 0], [0, 0, 0], [[-0.004, 0], [-0.004, 0.1225], [0.071, 0.1225], [0.071, 0]], { member: "turntable", part: "member" }),
        of("m-door-frame", "door-frame", { member: "door", part: "member" }),
        of("m-glass", "glass", { member: "door", part: "member" }),
        of("m-fascia", "fascia", { member: "panel", part: "member" }),
        box(["support-pad-floor", "pad", "corrugated-board", 0.2, "ground"], [0.4, 0.075, 0.3], [0.24, 0.0375, 0.0075], { member: "pad", part: "support", anchor: "pad-floor" }),
        box(["mass-housing", "housing", "electrical-iron", 0.3, "detail"], [0.075, 0.075, 0.075], [0.270328, 0.102376, 0.02], { member: "housing", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
