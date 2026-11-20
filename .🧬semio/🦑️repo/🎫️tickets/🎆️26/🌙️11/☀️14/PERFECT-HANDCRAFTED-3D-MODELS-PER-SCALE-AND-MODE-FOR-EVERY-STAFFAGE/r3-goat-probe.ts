/** 🔬️ Round 3 goat: which way `lateralScale` squeezes a revolve about the bone axis u, and what a swept ring measures. Usage: bun r3-goat-probe.ts */
import { Bench } from "./r3-goat-kit.ts";
const B = new Bench(50);
const base = { kind: "revolve", bone: "lumbar", material: "horn", role: "body", tone: 0.5, profile: [[-0.006, 0.094], [0.006, 0.094], [0.006, 0.106], [-0.006, 0.106]], axis: [1, 0, 0], origin: [0.1, 0, 0] };
B.lists.body.push({ id: "round", ...base }, { id: "squeezed", ...base, lateralScale: 0.5 });
for (const p of B.probe()) console.log(p.id, p.volume.toFixed(6), "size", p.max.map((v, i) => (v - p.min[i]!).toFixed(3)).join(" "));
