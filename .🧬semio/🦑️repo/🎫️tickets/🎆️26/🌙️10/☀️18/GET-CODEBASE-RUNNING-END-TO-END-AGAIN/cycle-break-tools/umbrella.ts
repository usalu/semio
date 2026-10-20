/** 🌂 Which packages still import the framework umbrella after the carves (runtime), and which of its pass-through keys point at them. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
import { model } from "./lib.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const es = build(plan).filter(e => !e.test);
const importers = new Map<string, number>();
for (const e of es) if (e.to === "@semio-tech/framework" && e.from !== "@semio-tech/framework") importers.set(e.from, (importers.get(e.from) ?? 0) + 1);
console.log("packages importing the umbrella (runtime):", importers.size);
console.log([...importers].sort((a, b) => b[1] - a[1]).map(([k, v]) => `${v} ${k}`).join("\n"));
const U = new Set(importers.keys());
const fw = model.byName.get("@semio-tech/framework")!;
console.log("\nframework depends on (declared):", Object.keys(fw.deps).filter(k => U.has(k)).join(", "));
