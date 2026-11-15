import { Kit, ivalsAll } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
const ids = process.argv[4]!.split(",");
const E = k.E((s) => ids.includes(s.id) && s.role === "body"), F = k.F((s) => ["bole-lower", "bole-upper"].includes(s.id));
for (const y of [0.1, 0.5, 1, 3, 7, 7.6, 10, 13, 15, 18, 20]) { const e = ivalsAll(E, "y", y), f = ivalsAll(F, "y", y); console.log(y, "E", JSON.stringify(e.map((v) => [v[0].toFixed(3), v[1].toFixed(3)])), "F", JSON.stringify(f.map((v) => [v[0].toFixed(3), v[1].toFixed(3)]))); }
