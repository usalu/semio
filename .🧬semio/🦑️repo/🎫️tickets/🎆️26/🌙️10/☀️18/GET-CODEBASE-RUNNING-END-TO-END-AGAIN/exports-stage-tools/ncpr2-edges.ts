/**
 * 🪢 Emits the current `no-cross-package-relative` violations in the edge format of `lay15-tools/ncpr2.ts` (cruise-like rows), so the
 * importer rewrite can run right after the batch is applied:
 *
 *   bun ncpr2-edges.ts <out.json>
 *   bun lay15-tools/ncpr2.ts <out.json> <plan.json> apply
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "./lib.ts";

const out = process.argv[2];
if (!out) throw Error("usage: bun ncpr2-edges.ts <out.json>");
const viol = JSON.parse(readFileSync(resolve(GENERATED, "ncpr-now.json"), "utf8")) as { file: string; spec: string; target: string; pkgs: string[] }[];
const rows = viol.flatMap(v => v.pkgs.map(p => ({ rule: "no-cross-package-relative-" + p, from: v.file, to: v.target, specifier: v.spec })));
writeFileSync(out, JSON.stringify(rows));
console.log(`${rows.length} rows (${viol.length} edges) -> ${out}`);
