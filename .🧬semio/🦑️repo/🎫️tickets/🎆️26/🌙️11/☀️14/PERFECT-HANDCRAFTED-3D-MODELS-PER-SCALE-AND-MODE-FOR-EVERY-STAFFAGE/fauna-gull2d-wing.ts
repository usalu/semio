/** 🐦️ Fauna: the folded gull hand (primaries) is a thin sheet, about 18 mm across, not as thick as the arm: sets the extrusion z-range of the hand shapes (front and plan derive from it) at every scale. */
const out: any[] = [];
for (const sc of ["50", "100"]) for (const [id, z0, z1] of [["near-hand", -0.09, -0.072], ["far-hand", 0.072, 0.09]] as const) for (const view of ["elevation", "front", "plan"]) out.push({ scale: sc, view, id, set: { solid: { kind: "extruded", sweep: "w", z0, z1 } } });
out.push({ scale: "200", view: "elevation", id: "hand", set: { solid: { kind: "extruded", sweep: "w", z0: -0.09, z1: -0.072 } } });
for (const view of ["front", "plan"]) for (const [id, z0, z1] of [["near-hand", -0.09, -0.072], ["far-hand", 0.072, 0.09]] as const) out.push({ scale: "200", view, id, set: { solid: { kind: "extruded", sweep: "w", z0, z1 } } });
console.log(JSON.stringify(out));
