/** 📎 Relative `?raw`/`?url` imports that cross a package boundary under the carved ownership (they bypass exports and need a typed asset module). */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { build } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const raw = new Set(scan().edges.filter(e => /\?(raw|url)/u.test(e.spec)).map(e => e.file + "\0" + e.target));
const cross = build(plan, false).filter(e => e.kind === "rel" && raw.has(e.file + "\0" + e.target));
console.log("raw/url edges", raw.size, "crossing", cross.length);
for (const e of cross) console.log(" ", e.from, "->", e.to, e.file.replace("🧰️framework/", "FW/").slice(-110), "=>", e.target.replace("🧰️framework/", "FW/").slice(-90));
const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), "utf8")) as { edits: { file: string; from: string }[] };
const fixed = new Set(delta.edits.filter(e => /\?(raw|url)/u.test(e.from)).map(e => e.file));
const spec = new Map(scan().edges.filter(e => /\?(raw|url)/u.test(e.spec)).map(e => [e.file + "\0" + e.target, e.spec]));
const open = cross.filter(e => !fixed.has(e.file));
console.log("not covered by a staged ?raw/?url edit:", open.length);
for (const e of open) console.log(" ", e.from, "->", e.to, e.file.replace("🧰️framework/", "FW/").slice(-100), "::", spec.get(e.file + "\0" + e.target));
