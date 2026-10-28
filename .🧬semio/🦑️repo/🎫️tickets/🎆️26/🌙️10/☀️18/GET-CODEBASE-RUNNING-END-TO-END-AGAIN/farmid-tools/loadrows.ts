// [farmid] Which module loads change the sealed record authority: <root> [extra module paths relative to 🏗️build...]; imports the extras first (as a graph child's command body does), then opens the farm service session exactly as a worker does, and prints the sealed load rows (kind, path, meaning) as JSON. Physical mode, read-only. Not a library file.
const [root, ...extras] = process.argv.slice(2), B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
Bun.plugin({ name: "fi-rows", setup(b) { b.onLoad({ filter: /observation\/[^/]*\.mjs$/ }, async args => { if (!args.path.endsWith("🏗️build/👁️observation/🟨️.mjs")) return undefined; const t = await Bun.file(args.path).text(); return { contents: t + "\nglobalThis.__fiRows=()=>[...loadObservations.values()].filter(row=>!(row.kind==='file'&&stampedFragments.some(f=>row.path.includes(f)))).map(row=>[row.kind,row.path,JSON.stringify(observationMeaningV1(row))]);\n", loader: "js" }; }); } });
for (const extra of extras) await import(B + extra);
const service = await import(B + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs");
const { mkdtempSync } = await import("node:fs"), { tmpdir } = await import("node:os"), { join } = await import("node:path");
const identity = await service.open({ version: 2, workspaceRoot: root, registry: null, commandInputs: null, lateClosures: false, projectsByRoot: [], componentRoots: [], stagingBase: mkdtempSync(join(tmpdir(), "fi-rows-")) }, { slot: 0, generation: 1 });
console.log(JSON.stringify({ identity, rows: (globalThis as any).__fiRows().sort() }));
process.exit(0);
