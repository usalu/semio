import ts from "/Users/ueli/.cache/c5c8/nm/node_modules/typescript/lib/typescript.js";
const L = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff";
const files = [`${L}/🟦️.ts`, ...["📋️deltas-requirements", "🗃️deltas-records", "🧱️deltas-program-structure", "🩹️patch-types"].map((d) => `${L}/${d}/🚪️parsers/🟦️.ts`), `${L}/../🧪️tests/🔬️document-contract/🟦️.ts`];
const program = ts.createProgram(files, { strict: true, noUnusedLocals: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
const all = ts.getPreEmitDiagnostics(program);
const mine = all.filter((d) => d.file?.fileName.includes("🔺️diff"));
console.log("total diagnostics", all.length, "in diff folder", mine.length);
for (const d of mine.slice(0, 20)) console.log(d.file!.fileName.slice(L.length), ts.flattenDiagnosticMessageText(d.messageText, "\n").slice(0, 200));
for (const d of all.filter((x) => !x.file?.fileName.includes("🔺️diff")).slice(0, 5)) console.log("other", d.file?.fileName.slice(-60), ts.flattenDiagnosticMessageText(d.messageText, "\n").slice(0, 120));
