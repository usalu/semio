import { runInterfaceImportCensus } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🔌️interface-import-census/🟦️.ts";
const r = runInterfaceImportCensus(process.cwd(), new AbortController().signal, () => {});
console.log("files", r.files, "hits", r.hits.length, "disagreements", r.disagreements.length);
for (const h of r.hits.slice(0, 40)) console.log(h.path, h.line, h.specifier);
for (const d of r.disagreements.slice(0, 10)) console.log("DISAGREE", d);
