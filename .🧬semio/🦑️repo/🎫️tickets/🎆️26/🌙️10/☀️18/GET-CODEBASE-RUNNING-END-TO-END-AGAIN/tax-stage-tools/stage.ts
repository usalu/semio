// 🏗️ Builds the anchored operation manifest of the tax-stage patch from the live tree without writing into it. usage: bun stage.ts [plan...]
import {mkdirSync, writeFileSync, rmSync} from "node:fs";
import {STAGED, Vfs} from "./vfs.ts";
import {planSimple} from "./plans/simple.ts";
import {planGhosts} from "./plans/ghosts.ts";
import {planLaws} from "./plans/laws.ts";
import {planInPlace, planLifts} from "./plans/lifts.ts";
import {planShims} from "./plans/shims.ts";

const v = new Vfs();
const log: Record<string, unknown>[] = [];
planSimple(v);
planGhosts(v, (row) => log.push(row));
planLifts(v, (row) => log.push(row));
planInPlace(v, (row) => log.push(row));
planLaws(v, (row) => log.push(row));
planShims(v, (row) => log.push(row));
const manifests = v.deferred();
if (manifests.length) throw new Error(`unexpected manifest changes (package.json edits belong to the exports-stage batch, not here): ${manifests.map((row) => row.path).join(", ")}`);
rmSync(`${STAGED}/blobs`, {recursive: true, force: true});
mkdirSync(STAGED, {recursive: true});
const ops = v.ops();
writeFileSync(`${STAGED}/ops.json`, JSON.stringify({version: 1, createdAt: new Date().toISOString(), ops}, null, 1));
writeFileSync(`${STAGED}/log.json`, JSON.stringify(log, null, 1));
console.log(`ops: ${ops.length} (${ops.filter((o) => o.kind === "write").length} write, ${ops.filter((o) => o.kind === "delete").length} delete)`);
for (const o of ops) console.log(o.kind.padEnd(6), o.path.replace(/^🧰️framework\/🛍️products\/🦑️repo\/🔨️modules\/📚️library/, "L"));
