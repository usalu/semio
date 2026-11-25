/** 🐴️ `used-spring-rocker`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-spring-rocker.ts <asset document> [--force] */
import { type Script, box, ell, ext, main, of, rev } from "./lib.ts";

const mass = [0.042, 0.042, 0.042] as const;
const grip = [[-0.2, 0], [-0.2, 0.0375], [0.2, 0.0375], [0.2, 0]] as const;
const horse = [[-0.22, 0], [0.22, 0], [0.3, 0.06], [0.36, 0.26], [0.47, 0.28], [0.5, 0.335], [0.29, 0.455], [0.27, 0.5], [0.13, 0.22], [-0.13, 0.21], [-0.35, 0.24], [-0.45, 0.24], [-0.34, 0.17], [-0.3, 0.06]] as const;

const script: Script = {
  id: "used-spring-rocker",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the footing block as it is; the spring with its three plates as one envelope cylinder of 0.20 m from the footing to the seat (it stands for the three envelope pieces of 1:200 and no longer bends); the two panels keep their 19 mm as declared sheets with the horse outline reduced from 22 to 14 points; bottom board, spacer and seat board are one block between the panels; handle and footrest with their grips are round 75 mm bars across the panels.", de: "Körper für das 75-mm-Merkmal: der Fundamentblock wie er ist; die Feder mit ihren drei Platten als ein Hüllzylinder von 0,20 m vom Fundament bis zum Sitz (er steht für die drei Hüllstücke von 1:200 und biegt sich nicht mehr); die zwei Wangen behalten ihre 19 mm als erklärte Platten mit dem von 22 auf 14 Punkte reduzierten Pferdeumriss; Bodenbrett, Abstandhalter und Sitzbrett sind ein Block zwischen den Wangen; Griff und Fußstütze mit ihren Griffen sind runde 75-mm-Stäbe quer durch die Wangen." },
        { en: "Thermal: the body solids in their regions by reference (the envelope cylinder gives the spring its coverage and stays out of the area sums), the anchor plate and the handle tube as shells of their layer at true size.", de: "Thermik: die Körper in ihren Regionen als Verweis (der Hüllzylinder gibt der Feder ihre Abdeckung und bleibt aus den Flächensummen), die Ankerplatte und das Griffrohr als Schalen ihrer Schicht in wahrer Größe." },
        { en: "Structure: the members by reference, the cylinder spanning the three spring bones, the seat block as the body joint, two spring joints, the footing support and five mass markers.", de: "Tragwerk: die Stäbe als Verweis, der Zylinder überspannt die drei Federknochen, der Sitzblock als Gelenk des Körpers, zwei Federgelenke, das Fundamentauflager und fünf Massenmarken." },
      ],
      body: [
        box(["footing-block", "footing", "concrete-c20-25", 0.86], [0.4, 0.5, 0.5], [0.2, 0, 0]),
        rev(["coil", "spring-1", "spring-steel-painted", 0.27], [1, 0, 0], [0, 0, 0], [[-0.018, 0], [-0.018, 0.1], [0.302, 0.1], [0.302, 0]], { envelope: true }),
        ext(["panel-left", "body", "hdpe-red-faded", 0.56], "xy", horse, 0.08, 0.099, { sheet: true }),
        ext(["panel-right", "body", "hdpe-red-faded", 0.5], "xy", horse, -0.099, -0.08, { sheet: true }),
        box(["seat-block", "body", "hdpe-blue-faded", 0.42], [0.3, 0.21, 0.16], [0, 0.105, 0]),
        rev(["handle", "body", "hdpe-green-faded", 0.32], [0, 0, 1], [0.27, 0.34, 0], grip),
        rev(["footrest", "body", "hdpe-green-faded", 0.32], [0, 0, 1], [0.13, 0.025, 0], grip),
      ],
      thermal: [
        of("th-footing-block", "footing-block", { region: "footing-concrete" }),
        of("th-coil", "coil", { region: "spring-steel" }),
        of("th-panel-left", "panel-left", { region: "panels-hdpe" }),
        of("th-panel-right", "panel-right", { region: "panels-hdpe" }),
        of("th-seat-block", "seat-block", { region: "seat-hdpe" }),
        of("th-handle", "handle", { region: "grips-hdpe" }),
        of("th-footrest", "footrest", { region: "grips-hdpe" }),
        box(["th-anchor-plate", "footing", "steel-galvanised-weathered", 0.6], [0.01, 0.3, 0.3], [0.405, 0, 0], { region: "plates-steel", layer: "plate" }),
        rev(["th-handle-tube", "body", "steel-galvanised-weathered", 0.66], [0, 0, 1], [0.27, 0.34, 0], [[-0.099, 0], [-0.099, 0.017], [0.099, 0.017], [0.099, 0]], { region: "tubes-steel", layer: "tube" }),
      ],
      structure: [
        of("st-footing", "footing-block", { member: "footing", part: "member" }),
        of("st-coil", "coil", { member: "spring-1", part: "member", members: ["spring-2", "spring-3"] }),
        of("st-panel-left", "panel-left", { member: "body", part: "member" }),
        of("st-panel-right", "panel-right", { member: "body", part: "member" }),
        of("st-handle", "handle", { member: "body", part: "member" }),
        of("st-footrest", "footrest", { member: "body", part: "member" }),
        of("st-body-joint", "seat-block", { member: "body", part: "joint" }),
        ell(["st-joint-2", "spring-2", "spring-steel-painted", 0.2, "detail"], [0.048, 0.048, 0.048], [0, 0, 0], { member: "spring-2", part: "joint" }),
        ell(["st-joint-3", "spring-3", "spring-steel-painted", 0.2, "detail"], [0.048, 0.048, 0.048], [0, 0, 0], { member: "spring-3", part: "joint" }),
        box(["st-support-footing", "footing", "concrete-c20-25", 0.15, "ground"], [0.075, 0.12, 0.12], [0.3625, 0, 0], { member: "footing", part: "support", anchor: "footing-ground" }),
        ell(["st-mass-footing", "footing", "steel-galvanised-weathered", 0.2, "detail"], mass, [0.206, 0, 0], { member: "footing", part: "mass" }),
        ell(["st-mass-spring-1", "spring-1", "steel-galvanised-weathered", 0.2, "detail"], mass, [0.048, 0, 0], { member: "spring-1", part: "mass" }),
        ell(["st-mass-spring-2", "spring-2", "steel-galvanised-weathered", 0.2, "detail"], mass, [0.046, 0, 0], { member: "spring-2", part: "mass" }),
        ell(["st-mass-spring-3", "spring-3", "steel-galvanised-weathered", 0.2, "detail"], mass, [0.06, 0, 0], { member: "spring-3", part: "mass" }),
        ell(["st-mass-body", "body", "steel-galvanised-weathered", 0.2, "detail"], mass, [0.066, 0.157, 0], { member: "body", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
