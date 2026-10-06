/** 🧷️ Binds registered generation, compiler owners and paired publications to physical bytes. */
import { mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
const workspace = "C:/git/semio", ticket = process.env.SEMIO_TICKET_DIR!, output = join(ticket, "🗑️generated/final-catalogue-revalidation"), mode = process.argv[2], phase = process.argv[3] ?? "before";
const hash = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex"), read = (path: string) => JSON.parse(readFileSync(path, "utf8")), write = (name: string, value: unknown) => writeFileSync(join(output, name + ".json"), JSON.stringify(value, null, 2));
mkdirSync(output, { recursive: true });
if (mode === "generation") {
 const { vizGeneratedFiles } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts");
 const files = vizGeneratedFiles().map(file => ({ path: file.path, sha256: hash(readFileSync(join(workspace, file.path))) }));
 if (files.length !== 83) throw Error("83 generated files required");
 write("generation-" + phase, files);
 if (phase === "after") { const prior = read(join(output, "generation-before.json")), changed = files.filter(file => prior.find((entry: any) => entry.path === file.path)?.sha256 !== file.sha256); write("generation-deltas", changed); console.log("[DEBUG] Generation83 changed=" + changed.length); }
 else console.log("[DEBUG] Generation83 physical bytes captured");
} else if (mode === "compiler") {
 const graphPath = join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "project-graph.json"), graph = read(graphPath), node = (graph.graph?.nodes ?? graph.nodes)["@semio-tech/print"], targets = node.data.targets, names = Object.keys(targets).filter(name => /^build-viz-(?:[0-9]+|api)$/.test(name)).sort();
 if (names.length !== 81 || JSON.stringify([...targets["build-viz"].dependsOn].sort()) !== JSON.stringify(names)) throw Error("81 exact aggregate owners required");
 if (!targets["test-viz-full"].dependsOn.includes("build-viz") || !targets["test-viz-full"].inputs.some((input: any) => input.transitive && input.dependentTasksOutputFiles === "**/*.pdf")) throw Error("full gate PDF input contract");
 const expand = (inputs: any[], seen = new Set<string>()): string[] => inputs.flatMap(input => {
  if (typeof input !== "string" || input.startsWith("!")) return [];
  if (input.startsWith("{workspaceRoot}/")) return [input.slice(16)];
  if (input.startsWith("{projectRoot}/")) return [node.data.root + "/" + input.slice(14)];
  if (seen.has(input) || !node.data.namedInputs?.[input]) throw Error("unresolved/cyclic input " + input);
  return expand(node.data.namedInputs[input], new Set([...seen, input]));
 });
 const paths = new Set<string>(), patterns = new Map<string, string[]>(), owners = names.map(name => {
  const target = targets[name], selected = new Set<string>();
  if (target.cache !== true || target.parallelism === false) throw Error("owner cache/parallelism " + name);
  for (const pattern of expand(target.inputs)) {
   if (!patterns.has(pattern)) patterns.set(pattern, pattern.includes("*") ? [...new Bun.Glob(pattern).scanSync({ cwd: workspace, onlyFiles: true, dot: true })] : [pattern]);
   for (const raw of patterns.get(pattern)!) { const path = raw.replaceAll("\\", "/"); if (!statSync(join(workspace, path)).isFile()) throw Error("input missing " + path); selected.add(path); paths.add(path); }
  }
  return { name, files: [...selected].sort(), target };
 });
 const files = [...paths].sort().map(path => ({ path, sha256: hash(readFileSync(join(workspace, path))) }));
 write("compiler-" + phase, files); write("owner-" + phase, owners); write("compiler-binding-" + phase, { graphPath, graphSha256: hash(readFileSync(graphPath)), owners: owners.length, files: files.length, sourceSha256: hash(JSON.stringify(files)) });
 if (phase === "after") { const prior = read(join(output, "compiler-before.json")), changed = files.filter(file => prior.find((entry: any) => entry.path === file.path)?.sha256 !== file.sha256), removed = prior.filter((file: any) => !paths.has(file.path)); write("compiler-drift", { changed, removed }); if (changed.length || removed.length) throw Error("source drift " + changed.length + "/" + removed.length); }
 console.log("[DEBUG] Compiler owners=" + owners.length + " inputs=" + files.length + " sha256=" + hash(JSON.stringify(files)));
} else if (mode === "paint-red") {
 const { visualizationTemplates } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts"), { printDocumentOutputDirectory } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"), { verifyPrintGalleryPaintOwnership } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts"), template = visualizationTemplates().find(template => template.id === "viz-0")!, directory = printDocumentOutputDirectory("viz-0"), receipt = read(join(directory, ".nx-artifact.json")), path = join(directory, receipt.files.find((name: string) => !name.includes("-dark"))), source = join(workspace, "🧰️framework/🛍️products/📓️print", template.texPath), leaves = [...readFileSync(source, "utf8").matchAll(/% viz-covers: ([^\r\n]+)[\s\S]*?\\SemioVizChart\{([^}]+)\}/g)].map(match => ({ leafId: match[1]!, slug: match[2]! }));
 if (!leaves.length || receipt.owner !== "@semio-tech/print:build-viz-0") throw Error("actual baseline owner");
 write("paint-red-input", { path, pdfSha256: hash(readFileSync(path)), receiptSha256: hash(readFileSync(join(directory, ".nx-artifact.json"))), source, sourceSha256: hash(readFileSync(source)), leaves });
 await verifyPrintGalleryPaintOwnership(path, leaves);
 throw Error("Required untagged-PDF RED unexpectedly passed");
} else if (mode === "paint-neutral") {
 const { verifyPrintKindPaintFixtures } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts"); await verifyPrintKindPaintFixtures();
} else if (mode === "paint-green") {
 const { visualizationTemplates } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts"), { printDocumentOutputDirectory } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"), { verifyPrintGalleryPaintOwnership } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts");
 const id = phase === "before" ? "viz-0" : phase, directory = printDocumentOutputDirectory(id), receiptPath = join(directory, ".nx-artifact.json"), receipt = read(receiptPath), template = visualizationTemplates().find(template => template.id === id)!, source = join(workspace, "🧰️framework/🛍️products/📓️print", template.texPath), text = readFileSync(source, "utf8"), leaves = [...text.matchAll(/% viz-covers: ([^\r\n]+)\r?\n[\s\S]*?\\SemioVizChart\{([^}]+)\}/g)].map(match => ({ leafId: match[1]!, slug: match[2]! }));
 const results = []; for (const name of receipt.files) { const path = join(directory, name), paints = await verifyPrintGalleryPaintOwnership(path, leaves); results.push({ path, pdfSha256: hash(readFileSync(path)), sourceSha256: hash(text), receiptSha256: hash(readFileSync(receiptPath)), leaves, paints }); console.log(`[DEBUG] Actual ${id}/${name}: ${paints.length} exact owned bodies PASS`); } write("paint-green-" + id, results);
} else if (mode === "paint-groups") {
 const results = read(join(output, "paint-green-" + (phase === "before" ? "viz-0" : phase) + ".json")), groups = new Map<string, any[]>();
 for (const result of results) for (const paint of result.paints) { const leaf = result.leaves.find((leaf: any) => "SemioVizLeaf" + Buffer.from(leaf.leafId).toString("hex") === paint.marker), key = result.path + "/" + paint.graphicSignature; groups.set(key, [...(groups.get(key) ?? []), { leaf, graphicSignature: paint.graphicSignature, paths: paint.paths.length, images: paint.images.length, glyphs: paint.glyphs.map((glyph: any) => glyph.text) }]); }
 const duplicates = [...groups].filter(([, rows]) => rows.length > 1).map(([key, rows]) => ({ key, rows })); write("primitive-equal-geometry", duplicates); console.log("[DEBUG] Actual primitive collision diagnostics " + JSON.stringify(duplicates));
} else if (mode === "tephi-oracle") {
 const urls = ["https://raw.githubusercontent.com/SciTools/tephi/main/tephi/transforms.py", "https://raw.githubusercontent.com/SciTools/tephi/main/tephi/constants.py"], sources = await Promise.all(urls.map(async url => { const result = await fetch(url); if (!result.ok) throw Error("primary oracle source " + result.status); return await result.text(); })), profile = [[1000, 20, 10], [850, 10, 5], [700, 0, -5], [500, -15, -25]];
 const python = "import ast,json,sys,types,numpy as np\na=json.load(sys.stdin); c={}; exec(compile(ast.Module(body=[n for n in ast.parse(a['constants']).body if isinstance(n,ast.Assign)],type_ignores=[]),'primary-constants','exec'),c); e={'np':np,'constants':types.SimpleNamespace(**c)}; names={'convert_pT2Tt','convert_Tt2xy','convert_xy2Tt','convert_Tt2pT'}; exec(compile(ast.Module(body=[n for n in ast.parse(a['transforms']).body if isinstance(n,ast.FunctionDef) and n.name in names],type_ignores=[]),'primary-transforms','exec'),e); p=np.asarray(a['profile']); out=[]\nfor column in [1,2]:\n t,theta=e['convert_pT2Tt'](p[:,0],p[:,column]); x,y=e['convert_Tt2xy'](t,theta); ti,thi=e['convert_xy2Tt'](x,y); pi,tpi=e['convert_Tt2pT'](ti,thi); out.append({'column':column,'points':np.column_stack([x,y]).tolist(),'roundtrip':np.column_stack([pi,tpi]).tolist()})\nprint(json.dumps({'numpy':np.__version__,'outputs':out}))";
 const child = Bun.spawn(["C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe", "-c", python], { stdin: "pipe", stdout: "pipe", stderr: "pipe" }); child.stdin.write(JSON.stringify({ transforms: sources[0], constants: sources[1], profile })); child.stdin.end(); const stdout = await new Response(child.stdout).text(), stderr = await new Response(child.stderr).text(); if (await child.exited) throw Error(stderr); const result = JSON.parse(stdout); write("tephi-oracle", { sources: urls.map((url, index) => ({ url, sha256: hash(sources[index]!) })), functions: ["convert_pT2Tt", "convert_Tt2xy", "convert_xy2Tt", "convert_Tt2pT"], profile, ...result }); console.log("[DEBUG] Actual SciTools Tephi functions with existing NumPy: " + JSON.stringify(result));
} else if (mode === "thermodynamic-controls") {
 const { compileNativeThermodynamicControlEffects } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeThermodynamicControlEffects(join(output, "thermodynamic-controls-" + phase));
} else if (mode === "specialized-placement-controls") {
 const { compileNativeSpecializedPlacementControls } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeSpecializedPlacementControls(join(output, "specialized-placement-controls-" + phase));} else if (mode === "biofabric-matrix-controls") {
 const { compileNativeBiofabricMatrixControls } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeBiofabricMatrixControls(join(output, "biofabric-matrix-controls-" + phase));} else if (mode === "inherited-caption-controls") {
 const { compileNativeInheritedCaptionControls } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeInheritedCaptionControls(join(output, "inherited-caption-controls-" + phase));} else if (mode === "list-seating-controls") {
 const { compileNativeListSeatingControlEffects } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeListSeatingControlEffects(join(output, "list-seating-controls-" + phase));
} else if (mode === "hull-controls") {
 const { compileNativePrimitiveHullEffects } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativePrimitiveHullEffects(join(output, "hull-controls-" + phase));
} else if (mode === "timeline-controls") {
 const { compileNativeTimelineControlEffects } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeTimelineControlEffects(join(output, "timeline-controls-" + phase));
} else if (mode === "process-controls") {
 const { compileNativeProcessControlEffects } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts"); await compileNativeProcessControlEffects(join(output, "process-controls-" + phase));
} else if (mode === "stage-watch") {
 const { stagePrintSources } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/📥️source-staging/🟦️.ts"), { printDocument, printLibrarySources } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"), product = join(workspace, "🧰️framework/🛍️products/📓️print"), start = Date.now(), owners = read(join(output, "owner-before.json")), expected = new Map<string, any[]>(), observed = new Map<string, any>();
 const libraryRoot = join(output, "stage-expectations/library"), library = [...stagePrintSources(product, printLibrarySources(), libraryRoot)].map(([source, path]) => ({ source, relative: "library/" + path.slice(libraryRoot.length + 1).replaceAll("\\", "/"), sha256: hash(readFileSync(path)) }));
 for (const owner of owners) { const id = owner.name.slice(6), document = printDocument(id), root = join(output, "stage-expectations", id); expected.set(id, [...library, ...[...stagePrintSources(product, document.sources, root)].map(([source, path]) => ({ source, relative: "source/" + path.slice(root.length + 1).replaceAll("\\", "/"), sha256: hash(readFileSync(path)) }))]); }
 write("stage-expected", [...expected].map(([id, files]) => ({ id, files })));
 console.log("[DEBUG] Stage watcher81 expected input closures ready start=" + start);
 let lastProgress = start;
 while (observed.size < 81) {
  const root = join(ticket, "🗑️generated/print");
  for (const name of readdirSync(root)) {
   const match = /^(viz-(?:[0-9]+|api))-/.exec(name), id = match?.[1], directory = join(root, name);
   if (!id || observed.has(id) || !expected.has(id) || statSync(directory).birthtimeMs < start) continue;
   try {
    const files = expected.get(id)!.map(file => ({ ...file, actualPath: join(directory, file.relative), actualSha256: hash(readFileSync(join(directory, file.relative))) }));
    if (files.some(file => file.actualSha256 !== file.sha256)) throw Error("staged source bytes differ " + id);
    observed.set(id, { id, directory, observedAt: new Date().toISOString(), files }); write("stages", [...observed.values()]); console.log("[DEBUG] Actual stage " + id + " " + observed.size + "/81");
   } catch (error) { if (!/ENOENT/.test(String(error))) throw error; }
  }
  if (Date.now() - lastProgress > 30000) { console.log("[DEBUG] Stage watcher progress=" + observed.size + "/81"); lastProgress = Date.now(); }
  if (Date.now() - start > 7200000) throw Error("stage watcher timeout " + observed.size + "/81");
  if (observed.size < 81) await new Promise(resolve => setTimeout(resolve, 1000));
 }
 console.log("[DEBUG] All81 actual paired publisher input stages byte-bound");
} else if (mode === "publication") {
 const { printDocumentOutputDirectory } = await import(workspace + "/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"), owners = read(join(output, "owner-before.json")), logPath = join(ticket, "🗑️generated/catalogue-current-full-publication.log"), log = readFileSync(logPath, "utf8"), ids = [...new Set([...log.matchAll(/Published (viz-(?:[0-9]+|api)): 2 PDFs/g)].map(match => match[1]))].sort();
 if (ids.length !== 81) throw Error("actual publication markers " + ids.length);
 const files = owners.flatMap((owner: any) => {
  const id = owner.name.slice(6), directory = printDocumentOutputDirectory(id), receipt = read(join(directory, ".nx-artifact.json"));
  if (!ids.includes(id) || receipt.version !== 1 || receipt.owner !== "@semio-tech/print:" + owner.name || receipt.files.length !== 2 || JSON.stringify(readdirSync(directory).sort()) !== JSON.stringify([".nx-artifact.json", ...receipt.files].sort())) throw Error("receipt owner " + id);
  return receipt.files.map((name: string) => { const path = join(directory, name), bytes = readFileSync(path); if (bytes.subarray(0, 5).toString() !== "%PDF-") throw Error("not PDF " + path); return { id, name, path, sha256: hash(bytes), bytes: bytes.length, receiptSha256: hash(readFileSync(join(directory, ".nx-artifact.json"))) }; });
 });
 const consumed = [...log.matchAll(/Consumed visualizations (viz-(?:[0-9]+|api))\/([^:\r\n]+): (\d+) pages PASS/g)].map(match => ({ id: match[1], name: match[2], pages: Number(match[3]) }));
 if (consumed.length !== 162 || files.length !== 162 || !files.every((file: any) => consumed.filter(row => row.id === file.id && row.name === file.name).length === 1) || !log.includes("PDF.js consumption visualizations: 162 PDFs PASS")) throw Error("exact162 actual consumers required");
 write("publications", files); write("consumers", consumed); write("publication-binding", { owners: ids.length, files: files.length, consumers: consumed.length, logPath, logSha256: hash(readFileSync(logPath)) }); console.log("[DEBUG] Publication81 paired162 exact consumers162 zero disk drift");
} else throw Error("Expected generation|compiler|publication");
