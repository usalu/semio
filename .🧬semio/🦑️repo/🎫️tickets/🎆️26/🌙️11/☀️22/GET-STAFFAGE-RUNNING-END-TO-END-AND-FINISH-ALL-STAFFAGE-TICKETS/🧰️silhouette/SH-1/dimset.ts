/** 📐️ Declares dimensions of a person by hand: keeps one `dims` op at the head of `ops/<slug>.json` (merged with the one already there); apply.ts writes it to the catalog file.
 *  bun dimset.ts <slug> widthM=0.477 [heightM=…] [depthM=…] */
import { readOps, writeOps } from "./ops.ts";
import { opsFile } from "./people.ts";

const [slug, ...pairs] = process.argv.slice(2);
const set = Object.fromEntries(pairs.map((p) => { const [k, v] = p.split("="); if (!["widthM", "heightM", "depthM"].includes(k!) || !Number.isFinite(Number(v))) throw new Error(`bad dimension ${p}`); return [k!, Number(v)]; }));
const existing = await readOps(opsFile(slug!));
const merged = { ...Object.assign({}, ...existing.filter((o) => o.op === "dims").map((o) => o.set)), ...set };
await writeOps(opsFile(slug!), [{ op: "dims", set: merged }, ...existing.filter((o) => o.op !== "dims")]);
console.log(`${slug}: dims ${JSON.stringify(merged)}`);
