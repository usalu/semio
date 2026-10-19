import { toolJobProductionSourcesOf, toolJobProofCatalogFailures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobProductionProtocolSource, toolJobProofs, toolJobStaticRows, toolJobDispositions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { listSourceFamily } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { relative, sep } from "node:path";
import { policyAllRustFiles } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const P = process.env.P!.split(",");
const ROOT = /\/(?:✏️editor|👁️viewer)\/🦀️\.rs$/u;
import { existsSync, readFileSync, writeFileSync } from "node:fs";
const LST = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json";
const every: string[] = existsSync(LST) ? JSON.parse(readFileSync(LST, "utf8")) : policyAllRustFiles(R);
if (!existsSync(LST)) writeFileSync(LST, JSON.stringify(every));
const all = every.filter((f) => P.some((p) => f.startsWith(p + "/")));
const files = toolJobProductionSourcesOf(R, all);
const proofs = toolJobProofs(files);
const rows = toolJobStaticRows(files), disp = toolJobDispositions(files);
console.log("files", files.size, "proofs", proofs.length, "rows", rows.length, "disp", disp.size);
const failures = toolJobProofCatalogFailures(files, rows, disp, proofs);
const cat = new Map<string, number>(); for (const x of failures) { const k = x.split(" ").slice(0, 5).join(" ") + " | " + x.split("\0")[0].split(" ").pop()!.slice(-60); cat.set(k, (cat.get(k) ?? 0) + 1); }
console.log([...cat].map(([k,v])=>v+" "+k).join("\n"));
if (process.env.SHOW) for (const x of failures.slice(0, Number(process.env.SHOW))) console.log(JSON.stringify(x).slice(0, 700));
if (process.env.ROWS) { console.log("ROWS", rows.map((r) => r.file.slice(-40) + ":" + r.id).join("\n")); console.log("DISP", [...disp].map(([k, v]) => k.slice(-50) + "=" + v).join("\n")); console.log("PROOFOWNERS", [...new Set(proofs.map((p) => p.ownerFile.slice(-40) + " " + p.ownerTypeName))].join("\n")); }
if (process.env.DBG) { const p0 = proofs.find((p) => p.ownerFile.includes(process.env.DBG!)); console.log("DBG proof", p0?.documentSchema, p0?.ownerTypeName); const t = files.get(p0!.ownerFile) ?? ""; const m = t.match(/impl\s+(?:semio_framework_plugin::)?ArtifactEditor\s+for\s+\w+\s*\{/g); console.log("owner impls", m, "len", t.length); for (const mm of t.matchAll(/const\s+DOCUMENT_SCHEMA[^;]*;/g)) console.log(mm[0].slice(0, 160)); }
if (process.env.DBG2) { const nm = process.env.DBG2!; for (const [f, t] of files) for (const m of t.matchAll(new RegExp(`const\\s+${nm}\\s*:[^;]*;`, "g"))) console.log("DEF", f.slice(-60), JSON.stringify(m[0].slice(0, 200))); }
