/** 🏛️ Feedback edges against an explicit package layering (rank 0 is the bottom; equal ranks form an allowed knot). */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const ranks: Record<string, number> = JSON.parse(readFileSync(resolve(import.meta.dir, process.argv[2] ?? "layers.json"), "utf8"));
const mode = process.argv[3] ?? "runtime";
const es = build(plan).filter(e => mode === "all" || !e.test);
const by = new Map<string, Set<string>>();
for (const e of es) { const a = ranks[e.from], b = ranks[e.to]; if (a === undefined || b === undefined) continue; if (a < b) (by.get(e.from + " (" + a + ") -> " + e.to + " (" + b + ")") ?? by.set(e.from + " (" + a + ") -> " + e.to + " (" + b + ")", new Set()).get(e.from + " (" + a + ") -> " + e.to + " (" + b + ")")!).add(e.file); }
let n = 0;
for (const [k, v] of [...by].sort((x, y) => y[1].size - x[1].size)) { console.log(String(v.size).padStart(4), "files", k); n += v.size; }
console.log("feedback files", n);
