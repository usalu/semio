/** 🔎️ Prints the world origin and tip of bones for a probe pose: bun probe.ts <slug|file> '<bones json>' bone,bone [contacts] */
import { probe, readAsset } from "./lib.ts";
const [src, bonesJson, list, contacts] = process.argv.slice(2);
const asset = await readAsset(src!);
const f = probe(asset, JSON.parse(bonesJson ?? "{}"), contacts ? contacts.split(",") : []);
const p = (v: { x: number; y: number; z: number }) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
for (const id of (list ?? Object.keys(f).join(",")).split(",")) console.log(`${id.padEnd(16)} o ${p(f[id]!.o)} tip ${p(f[id]!.tip)} u ${p(f[id]!.u)} v ${p(f[id]!.v)}`);
