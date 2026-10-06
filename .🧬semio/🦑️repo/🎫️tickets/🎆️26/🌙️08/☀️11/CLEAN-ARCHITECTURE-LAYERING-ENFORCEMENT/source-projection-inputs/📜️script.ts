import { existsSync, mkdirSync, readFileSync, writeFileSync, openSync, closeSync, writeSync, fsyncSync, ftruncateSync } from "node:fs";
import { dirname, resolve, win32, posix } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { createHash } from "node:crypto";
const ticket = resolve(import.meta.dir, "..");
let root = ticket;
while (!existsSync(resolve(root, "nx.json"))) root = dirname(root);
const command = process.argv[2], epoch = process.argv[3] ?? "1";
if (!/^[1-9]\d*$/.test(epoch)) throw Error("Invalid source projection epoch");
const templates = resolve(import.meta.dir, "📷️snapshot"), generated = resolve(ticket, "🗑️generated/source-projection");
mkdirSync(generated, { recursive: true });
const sha = (source: string) => createHash("sha256").update(source).digest("hex");
function save(path: string, value: unknown) {
  if (existsSync(path)) throw Error("Immutable projection authority exists: " + path);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(value, null, 2) + "\n", { flag: "wx" });
}
if (command === "contracts") {
  const change = (path: string, before: string | null, after: string | null) => ({ path, before, after });
  const cases: any[] = [];
  const add = (id: string, files: Record<string, string>, changes: any[], requests: string[], references: any[], expected: any) => cases.push({ id, files, changes, requests, references, expected });
  const accepted = (files: Record<string, string | null>, inverse: any[]) => ({ files, inverse });
  const refused = (code: string, path: string) => ({ refusal: { code, path } });
  add("untouched-source", { "a.rs": "original" }, [], ["a.rs"], [], accepted({ "a.rs": "original" }, []));
  add("addition", {}, [change("new.rs", null, "new")], ["new.rs"], [], accepted({ "new.rs": "new" }, [change("new.rs", "new", null)]));
  add("replacement", { "a.rs": "old" }, [change("a.rs", "old", "new")], ["a.rs"], [], accepted({ "a.rs": "new" }, [change("a.rs", "new", "old")]));
  add("unchanged-authority", { "a.rs": "same" }, [change("a.rs", "same", "same")], ["a.rs"], [], accepted({ "a.rs": "same" }, [change("a.rs", "same", "same")]));
  add("deletion", { "a.rs": "old" }, [change("a.rs", "old", null)], ["a.rs"], [], accepted({ "a.rs": null }, [change("a.rs", null, "old")]));
  add("repeated-retain-cannot-resurrect", { "a.rs": "old" }, [change("a.rs", "old", null)], ["a.rs", "a.rs", "a.rs"], [], accepted({ "a.rs": null }, [change("a.rs", null, "old")]));
  add("empty-source-is-present", { "a.rs": "old" }, [change("a.rs", "old", "")], ["a.rs"], [], accepted({ "a.rs": "" }, [change("a.rs", "", "old")]));
  add("empty-source-addition", {}, [change("a.rs", null, "")], ["a.rs"], [{ from: "root", target: "a.rs", kind: "root" }], accepted({ "a.rs": "" }, [change("a.rs", "", null)]));
  add("unicode-body-and-path", { "🧬️/ä.rs": "before\n" }, [change("🧬️/ä.rs", "before\n", "nachher\u0000")], ["🧬️/ä.rs"], [], accepted({ "🧬️/ä.rs": "nachher\u0000" }, [change("🧬️/ä.rs", "nachher\u0000", "before\n")]));
  add("missing-optional-source", {}, [], ["missing.rs"], [], accepted({ "missing.rs": null }, []));
  add("addition-over-existing", { "a.rs": "foreign" }, [change("a.rs", null, "new")], [], [], refused("predecessor-mismatch", "a.rs"));
  add("replacement-with-missing-predecessor", {}, [change("a.rs", "old", "new")], [], [], refused("predecessor-mismatch", "a.rs"));
  add("deletion-with-missing-predecessor", {}, [change("a.rs", "old", null)], [], [], refused("predecessor-mismatch", "a.rs"));
  add("changed-predecessor", { "a.rs": "foreign" }, [change("a.rs", "old", null)], [], [], refused("predecessor-mismatch", "a.rs"));
  add("absent-to-absent", {}, [change("a.rs", null, null)], [], [], refused("empty-change", "a.rs"));
  add("duplicate-change", { "a.rs": "old" }, [change("a.rs", "old", null), change("a.rs", "old", "new")], [], [], refused("duplicate-change", "a.rs"));
  for (const kind of ["manifest", "module", "include", "runtime", "root"]) add("deleted-" + kind + "-reference", { "a.rs": "old" }, [change("a.rs", "old", null)], [], [{ from: "consumer.rs", target: "a.rs", kind }], refused("deleted-reference", "a.rs"));
  add("missing-required-source", {}, [], [], [{ from: "root", target: "a.rs", kind: "root" }], refused("missing-reference", "a.rs"));
  add("reference-to-added-source", {}, [change("new.rs", null, "new")], ["new.rs"], [{ from: "consumer.rs", target: "new.rs", kind: "module" }], accepted({ "new.rs": "new" }, [change("new.rs", "new", null)]));
  add("ordered-inverse", { "z.rs": "old", "a.rs": "before" }, [change("z.rs", "old", null), change("a.rs", "before", "after")], ["z.rs", "a.rs"], [], accepted({ "z.rs": null, "a.rs": "after" }, [change("z.rs", null, "old"), change("a.rs", "after", "before")]));
  for (const path of ["", "/a.rs", "../a.rs", "a/../b.rs", "a//b.rs", "./a.rs", "a\\b.rs", "C:/a.rs", "a\u0000.rs"]) add("invalid-path-" + cases.length, {}, [change(path, null, "new")], [], [], refused("invalid-path", path));
  const nullable = { type: ["string", "null"] }, path = { type: "string" };
  const row = { type: "object", additionalProperties: false, required: ["path", "before", "after"], properties: { path, before: nullable, after: nullable } };
  const reference = { type: "object", additionalProperties: false, required: ["from", "target", "kind"], properties: { from: path, target: path, kind: { type: "string", minLength: 1 } } };
  const refusal = { type: "object", additionalProperties: false, required: ["code", "path"], properties: { code: { enum: ["invalid-path", "duplicate-change", "empty-change", "predecessor-mismatch", "deleted-reference", "missing-reference"] }, path } };
  const expected = { oneOf: [{ type: "object", additionalProperties: false, required: ["files", "inverse"], properties: { files: { type: "object", additionalProperties: nullable }, inverse: { type: "array", items: row } } }, { type: "object", additionalProperties: false, required: ["refusal"], properties: { refusal } }] };
  const schema = { $schema: "https://json-schema.org/draft/2020-12/schema", $id: "https://schemas.semio.tech/framework/filesystem/source-projection", type: "object", additionalProperties: false, required: ["cases"], properties: { cases: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "files", "changes", "requests", "references", "expected"], properties: { id: { type: "string", minLength: 1 }, files: { type: "object", additionalProperties: { type: "string" } }, changes: { type: "array", items: row }, requests: { type: "array", items: path }, references: { type: "array", items: reference }, expected } } } } };
  save(resolve(templates, "🧬️schema/🔣️.json"), schema);
  save(resolve(templates, "🧫️fixtures/🔣️.json"), { cases });
  const { default: Ajv } = await import("ajv/dist/2020.js");
  if (!new Ajv({ strict: true }).compile(schema)({ cases })) throw Error("Projection schema refused");
  save(resolve(generated, "contract-" + epoch + ".json"), { at: new Date().toISOString(), schemaHash: sha(JSON.stringify(schema, null, 2) + "\n"), cases: cases.length, implementationAuthored: false });
  console.log(JSON.stringify({ cases: cases.length, implementationAuthored: false }));
} else if (command === "stage") {
  const fixture = JSON.parse(readFileSync(resolve(templates, "🧫️fixtures/🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(resolve(templates, "🧬️schema/🔣️.json"), "utf8"));
  const redPath = resolve(generated, "red-1.log"), greenPath = resolve(generated, "green-2.log"), red = readFileSync(redPath, "utf8"), green = readFileSync(greenPath, "utf8");
  if (!red.includes("Cannot find module '../🟦️.ts'") || !green.includes("1 pass") || !green.includes("0 fail")) throw Error("Actual red/green source projection test evidence required");
  const observations = green.split("\n").filter(line => line.startsWith("[DEBUG] source projection case ")).map(line => JSON.parse(line.slice("[DEBUG] source projection case ".length)));
  if (observations.length !== fixture.cases.length || observations.some((row: any, index: number) => row.id !== fixture.cases[index].id || !isDeepStrictEqual(row.expected, fixture.cases[index].expected) || !isDeepStrictEqual(row.expected, row.own) || !isDeepStrictEqual(row.expected, row.independent))) throw Error("Exact language-neutral projection observations differ");
  const { default: Ajv } = await import("ajv/dist/2020.js");
  if (!new Ajv({ strict: true }).compile(schema)(fixture)) throw Error("Projection schema refused");
  const target = "🧰️framework/🔨️modules/📁️filesystem/📷️snapshot", names = ["🧬️schema/🔣️.json", "🧫️fixtures/🔣️.json", "🟦️.ts", "🦀️.rs", "🧪️tests/🟦️.ts", "🧪️tests/🦀️.rs"];
  const pair = (path: string, before: string | null, after: string) => ({ path, before, after, beforeHash: before === null ? null : sha(before), afterHash: sha(after), inverseBody: before, forward: { start: 0, removed: before ?? "", inserted: after }, inverse: { start: 0, removed: after, inserted: before ?? "" }, forwardExact: true, inverseExact: true });
  const rows = names.map(name => { const path = target + "/" + name; if (existsSync(resolve(root, path))) throw Error("Projection owner already exists: " + path); return pair(path, null, readFileSync(resolve(templates, name), "utf8")); });
  const mountPath = "🧰️framework/📦️packages/🦀️rust/🦀️.rs", before = readFileSync(resolve(root, mountPath), "utf8"), anchor = '#[path = "../../🔨️modules/🎯️action-bus/🦀️.rs"]';
  if (before.split(anchor).length !== 2 || before.includes("pub mod source_projection;")) throw Error("General source projection mount predecessor differs");
  const after = before.replace(anchor, '#[path = "../../🔨️modules/📁️filesystem/📷️snapshot/🦀️.rs"]\npub mod source_projection;\n\n' + anchor);
  rows.push(pair(mountPath, before, after));
  const ts = await import("typescript"), options = { noEmit: true, strict: true, skipLibCheck: true, allowImportingTsExtensions: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: ["bun"] };
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([resolve(templates, "🟦️.ts")], options)).map(row => ({ code: row.code, category: row.category, message: ts.flattenDiagnosticMessageText(row.messageText, "\n") }));
  if (diagnostics.some(row => row.category === ts.DiagnosticCategory.Error)) throw Error("Projection strict type check refused " + JSON.stringify(diagnostics));
  const { default: Parser } = await import("web-tree-sitter"); await Parser.init(); const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
  const rustGrammar = rows.filter(row => row.path.endsWith(".rs")).map(row => { const tree = parser.parse(row.after); const clean = !tree.rootNode.hasError(); tree.delete(); return { path: row.path, clean }; }); parser.delete();
  if (rustGrammar.some(row => !row.clean)) throw Error("Source projection Rust grammar refused");
  save(resolve(ticket, "🗑️generated/neutral-render-owner/source-projection-source-" + epoch + ".json"), { at: new Date().toISOString(), rows, observations, diagnostics, rustGrammar, evidence: [{ path: redPath, source: red, sha256: sha(red), exitCode: 1 }, { path: greenPath, source: green, sha256: sha(green), exitCode: 0 }], nativeExecuted: false, sourceWritesOutsideTicket: false, remaining: ["Independent source admission", "Guarded publication", "Owning published implementation verification", "Separate consumer planner continuation", "Native owning original law execution"] });
  console.log(JSON.stringify({ rows: rows.length, cases: observations.length, strictErrors: 0, rustGrammarClean: true, nativeExecuted: false, sourceWritesOutsideTicket: false }));
} else if (command === "stage-consumer") {
  const path = "consumer-os-native-inputs/📜️script.ts", before = readFileSync(resolve(ticket, path), "utf8"), start = before.indexOf('if(command==="plan"){'), end = before.indexOf('if(command==="metadata"||command==="metadata-selected"){', start);
  if (start < 0 || end < start) throw Error("Exact native plan branch required");
  let branch = before.slice(start, end);
  const edits: any[] = [];
  function replace(original: string, authored: string) {
    if (branch.split(original).length !== 2) throw Error("Exact consumer projection predecessor differs: " + original.slice(0, 100));
    branch = branch.replace(original, authored); edits.push({ original, authored });
  }
  replace('typeof row.path!=="string"||typeof row.after!=="string"', 'typeof row.path!=="string"||(row.before!==null&&typeof row.before!=="string")||(row.after!==null&&typeof row.after!=="string")');
  replace('const authored=new Map<string,string>();', 'const authored=new Map<string,string|null>();');
  replace('if(row.afterHash&&sha(row.after)!==row.afterHash)', 'if(row.beforeHash!==(row.before===null?null:sha(row.before))||row.afterHash!==(row.after===null?null:sha(row.after)))');
  replace('authored.set(path,row.after);}', 'authored.set(path,row.after);}const {SourceProjection}=await import(resolve(root,"🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts"));const projection=new SourceProjection(index.rows.map((row:any)=>({path:relative(root,resolve(root,row.path)),before:row.before,after:row.after})),(path:string)=>existsSync(resolve(root,path))?read(path):null);');
  replace('function source(path:string){return authored.get(path)??pinned.get(path)?.toString()??read(path);}', 'function source(path:string){if(authored.has(path))return projection.require(path);return pinned.get(path)?.toString()??read(path);}');
  replace('function retain(path:string){if(assets.has(path))return;', 'function retain(path:string,required=false){if(projection.isDeleted(path)){if(required)projection.require(path);return false;}if(assets.has(path))return true;');
  replace('proposed=authored.get(path),bytes=proposed===undefined?', 'proposed=authored.has(path)?projection.require(path):undefined,bytes=proposed===undefined?');
  replace('sha256:sha(before)});}}', 'sha256:sha(before)});}return true;}');
  replace('manifests.set(path,row);retain(path);retain(library);', 'manifests.set(path,row);retain(path,true);retain(library,true);');
  replace('retain(path);if(assets.get(path).sha256!==row.sha256)', 'retain(path,true);if(assets.get(path).sha256!==row.sha256)');
  replace('candidates.push(rel);retain(rel);', 'if(projection.isDeleted(rel)){if(candidates.length===0)projection.require(rel);}else{candidates.push(rel);retain(rel,true);}');
  replace('for(const row of runtime.declarations){const pinned=assets.get(row.path);', 'for(const row of runtime.declarations){if(projection.isDeleted(row.path))projection.require(row.path);const pinned=assets.get(row.path);');
  replace('routes,managedRunCommands,providerManifests:', 'routes,managedRunCommands,sourceProjection:{implementation:frame(relative(root,resolve(root,"🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts"))),changes:index.rows,inverse:projection.inverse(),deleted:[...authored].filter(([,after])=>after===null).map(([path])=>path)},providerManifests:');
  const ts = await import("typescript"), branchTree = ts.createSourceFile("📜️script.ts", branch, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), pathCanonicalization: { start: number; end: number; source: string }[] = [];
  function visit(node: import("typescript").Node) {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "relative" && node.arguments.length === 2 && node.arguments[0].getText(branchTree) === "root") pathCanonicalization.push({ start: node.getStart(branchTree), end: node.getEnd(), source: node.getText(branchTree) });
    ts.forEachChild(node, visit);
  }
  visit(branchTree);
  for (const row of [...pathCanonicalization].sort((a, b) => b.end - a.end)) branch = branch.slice(0, row.end) + '.split(sep).join("/")' + branch.slice(row.end);
  const canonicalPathGuards: { original: string; authored: string }[] = [];
  branch = branch.replace(/\b(path|dependency|rel)\.startsWith\("\.\."\+sep\)/g, (original: string, name: string) => {
    const authored = '(' + name + '===".."||' + name + '.startsWith("../"))';
    canonicalPathGuards.push({ original, authored }); return authored;
  });
  const after = before.slice(0, start) + branch + before.slice(end), parsed = ts.createSourceFile("📜️script.ts", after, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const parseDiagnostics = (parsed as any).parseDiagnostics.map((row: any) => ({ code: row.code, start: row.start, message: ts.flattenDiagnosticMessageText(row.messageText, "\n") }));
  if (parseDiagnostics.length) throw Error("Consumer projection syntax refused " + JSON.stringify(parseDiagnostics));
  const out = resolve(generated, "consumer-source-" + epoch + ".json");
  if (readFileSync(resolve(ticket, path), "utf8") !== before) throw Error("Pinned consumer helper advanced during projection staging");
  save(out, { at: new Date().toISOString(), path: resolve(ticket, path), before, after, beforeHash: sha(before), afterHash: sha(after), inverseBody: before, forward: { start: 0, removed: before, inserted: after }, inverse: { start: 0, removed: after, inserted: before }, edits, pathCanonicalization, canonicalPathGuards, parseDiagnostics, outsidePlanExact: after.slice(0, start) === before.slice(0, start) && after.slice(start + branch.length) === before.slice(end), scope: "Separate deletion-capable native planner proposal; active9/10 helpers are not written", sourceWritesOutsideTicket: false, nativeExecuted: false, published: false });
  console.log(JSON.stringify({ out, edits: edits.length, syntaxClean: true, outsidePlanExact: true, published: false }));
} else if (command === "test-consumer") {
  const authorityPath = resolve(generated, "consumer-source-2.json"), authoritySource = readFileSync(authorityPath, "utf8"), authority = JSON.parse(authoritySource);
  const fixturePath = resolve(import.meta.dir, "📷️consumer/🧫️fixtures/🔣️.json"), schemaPath = resolve(import.meta.dir, "📷️consumer/🧬️schema/🔣️.json"), fixtureSource = readFileSync(fixturePath, "utf8"), schemaSource = readFileSync(schemaPath, "utf8"), fixture = JSON.parse(fixtureSource);
  const { default: Ajv } = await import("ajv/dist/2020.js"), { omit } = await import("lodash"), { normalize } = await import("pathe"), ts = await import("typescript");
  if (!new Ajv({ strict: true }).compile(JSON.parse(schemaSource))(fixture) || !authority.outsidePlanExact || readFileSync(authority.path, "utf8") !== authority.before || sha(authority.before) !== authority.beforeHash || sha(authority.after) !== authority.afterHash) throw Error("Exact consumer source/schema authority refused");
  const projectionPath = resolve(root, "🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts"), projectionSource = readFileSync(projectionPath, "utf8"), { SourceProjection, SourceProjectionError } = await import(projectionPath);
  const tree = ts.createSourceFile("📜️script.ts", authority.after, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), functions = new Map<string, string>(); let candidateSource: string | null = null;
  function visit(node: import("typescript").Node) {
    if (ts.isFunctionDeclaration(node) && node.name && ["source", "retain"].includes(node.name.text)) { if (functions.has(node.name.text)) throw Error("Ambiguous consumer function"); functions.set(node.name.text, node.getText(tree)); }
    if (ts.isIfStatement(node) && node.expression.getText(tree) === "projection.isDeleted(rel)") { if (candidateSource !== null) throw Error("Ambiguous literal candidate guard"); candidateSource = node.getText(tree); }
    ts.forEachChild(node, visit);
  }
  visit(tree);
  if (functions.size !== 2 || candidateSource === null || !authority.canonicalPathGuards.length) throw Error("Actual staged collector functions/guards required");
  const emitted = ts.transpileModule([...functions.values()].join("\n") + "\nreturn {source,retain};", { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.None } }).outputText;
  const candidateBody = ts.transpileModule(candidateSource, { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.None } }).outputText;
  const rows: any[] = [], controls = resolve(generated, "controls-" + epoch);
  if (existsSync(controls)) throw Error("Immutable consumer controls exist"); mkdirSync(controls, { recursive: true });
  writeFileSync(resolve(controls, "retired.rs"), "current original\n", { flag: "wx" });
  for (const row of fixture.retention) {
    const changes = [{ path: "retired.rs", before: "current original\n", after: null }, { path: "survivor.rs", before: null, after: "survivor" }], assets = new Map(), retained = new Map(), authored = new Map(changes.map(change => [change.path, change.after])), pinned = new Map(), reads: string[] = [];
    const projection = new SourceProjection(changes, (path: string) => existsSync(resolve(controls, path)) ? readFileSync(resolve(controls, path), "utf8") : null);
    const trackedRead = (path: string) => { reads.push(path); return readFileSync(path); }, read = (path: string) => trackedRead(resolve(controls, path)).toString();
    const consumer = new Function("assets", "retained", "root", "projection", "authored", "pinned", "existsSync", "resolve", "readFileSync", "sha", "Buffer", "read", emitted)(assets, retained, controls, projection, authored, pinned, existsSync, resolve, trackedRead, sha, Buffer, read);
    let actual: string;
    try {
      if (row.operation === "retain") { consumer.retain("retired.rs", row.required); consumer.retain("survivor.rs", true); consumer.retain("retired.rs", row.required); actual = "omitted"; }
      else if (row.operation === "source") { consumer.source("retired.rs"); actual = "omitted"; }
      else if (row.operation === "require") { projection.require("retired.rs"); actual = "omitted"; }
      else { const candidates: string[] = [], candidate = new Function("projection", "candidates", "retain", "rel", candidateBody); candidate(projection, candidates, consumer.retain, "survivor.rs"); candidate(projection, candidates, consumer.retain, "retired.rs"); if (!isDeepStrictEqual(candidates, ["survivor.rs"])) throw Error("Ancestor candidate projection differs"); actual = "survivor"; }
    } catch (error) { if (!(error instanceof SourceProjectionError)) throw error; actual = error.code; }
    const oracleOverlay = Object.assign(Object.create(null), omit({ "retired.rs": "current original\n" }, ["retired.rs"]), { "survivor.rs": "survivor" });
    const independent = row.operation === "ancestor" && Object.hasOwn(oracleOverlay, "survivor.rs") ? "survivor" : row.required ? "deleted-reference" : "omitted";
    if (actual !== row.expected || independent !== row.expected || assets.has("retired.rs") || retained.has("retired.rs") || reads.length || readFileSync(resolve(controls, "retired.rs"), "utf8") !== "current original\n") throw Error("Consumer deletion control refused: " + row.id);
    rows.push({ ...row, actual, independent, retiredSnapshotPresent: assets.has("retired.rs"), retiredPhysicalFallbackReads: reads.length, actualPhysicalPredecessorPreserved: true });
  }
  const paths = fixture.paths.map((row: any) => {
    const platform = row.platform === "win32" ? win32 : posix, native = platform.relative(row.root, row.target), canonical = native.split(platform.sep).join("/"), independent = normalize(native);
    const guards = authority.canonicalPathGuards.map((guard: any) => new Function("path", "dependency", "rel", "return " + guard.authored)(canonical, canonical, canonical));
    if (canonical !== row.canonical || independent !== row.canonical || guards.some((escaped: boolean) => escaped !== row.escaped)) throw Error("Canonical escaped-path control refused: " + row.id);
    return { ...row, native, actual: canonical, independent, guards };
  });
  save(resolve(generated, "consumer-controls-" + epoch + ".json"), { at: new Date().toISOString(), authority: { path: authorityPath, sha256: sha(authoritySource) }, projection: { path: projectionPath, source: projectionSource, sha256: sha(projectionSource) }, fixture: { path: fixturePath, source: fixtureSource, sha256: sha(fixtureSource) }, schema: { path: schemaPath, source: schemaSource, sha256: sha(schemaSource) }, functions: [...functions].map(([name, source]) => ({ name, source })), candidateSource, rows, paths, nativeExecuted: false, wholePlannerExecuted: false, sourceWritesOutsideTicket: false });
  console.log("[DEBUG] consumer source projection: " + rows.length + " deletion controls and " + paths.length + " Windows/POSIX canonical path controls retained");
} else if (command === "publish-consumer") {
  const controller = new AbortController(), cancel = () => controller.abort(), io = { signal: controller.signal };
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  try {
  const publisherSource = readFileSync(import.meta.path, "utf8"), publisherProofPath = resolve(generated, "consumer-publisher-source-independent-2.json"), publisherProofSource = readFileSync(publisherProofPath, "utf8"), publisherProof = JSON.parse(publisherProofSource);
  if (publisherProof.admitted !== true || publisherProof.path !== import.meta.path || publisherProof.sha256 !== sha(publisherSource)) throw Error("Exact executing publisher admission required");
  const authorityPath = resolve(generated, "consumer-source-2.json"), proofPath = resolve(generated, "consumer-source-independent-2.json"), releasePath = resolve(ticket, "🗑️generated/consumer-os-native/replay-10/released-final-source-current-1.json");
  const authoritySource = readFileSync(authorityPath, "utf8"), proofSource = readFileSync(proofPath, "utf8"), authority = JSON.parse(authoritySource), proof = JSON.parse(proofSource);
  const { hashFileV1, readJsonFileV1 } = await import(resolve(ticket, "native-json-inputs/📜️script.ts")), releaseHash = await hashFileV1(releasePath, io), release = await readJsonFileV1(releasePath, io);
  if (await hashFileV1(releasePath, io) !== releaseHash) throw Error("Released compiler checkpoint advanced during bounded read");
  const releaseProofSource = readFileSync(release.proof.path, "utf8"), releaseProof = JSON.parse(releaseProofSource), controlsSource = readFileSync(proof.controls.path, "utf8"), controls = JSON.parse(controlsSource);
  if (proof.admitted !== true || proof.authorityPath !== authorityPath || proof.authorityHash !== sha(authoritySource) || proof.controls.sha256 !== sha(controlsSource) || proof.controls.rows !== 8 || proof.controls.platformPaths !== 6 || !proof.outsidePlanExact || !authority.outsidePlanExact || authority.inverseBody !== authority.before || authority.beforeHash !== sha(authority.before) || authority.afterHash !== sha(authority.after)) throw Error("Exact deletion source/control admission required");
  if (release.sourceCheckpointReleased !== true || release.terminal.exitCode !== 1 || release.runtime.selected !== null || release.runtime.completed !== 0 || release.originSupplement.currentGaps !== 0 || release.originSupplement.retainedPreGaps !== 0 || !release.snapshotChecks.length || release.snapshotChecks.some((row: any) => row.current !== row.expected) || releaseProof.ready !== true || sha(releaseProofSource) !== release.proof.sha256) throw Error("Honest held compiler checkpoint release required");
  if (!Array.isArray(releaseProof.bindings) || !releaseProof.bindings.length) throw Error("Exact released compiler bindings required");
  async function guardReleased() {
    if (await hashFileV1(releasePath, io) !== releaseHash) throw Error("Released compiler checkpoint advanced");
    for (const row of releaseProof.bindings) if (await hashFileV1(row.path, io) !== row.sha256) throw Error("Released compiler binding advanced: " + row.path);
    controller.signal.throwIfAborted();
  }
  await guardReleased();
  const projectionFrame = controls.projection;
  if (readFileSync(projectionFrame.path, "utf8") !== projectionFrame.source || sha(projectionFrame.source) !== projectionFrame.sha256) throw Error("Defining projection implementation advanced");
  const out = resolve(generated, "consumer-published-" + epoch + ".json"), journalPath = resolve(generated, "consumer-publication-journal-" + epoch + ".jsonl");
  if (existsSync(out) || existsSync(journalPath) || readFileSync(authority.path, "utf8") !== authority.before) throw Error("Consumer publication/current predecessor refused");
  const bindings = [{ path: import.meta.path, source: publisherSource }, { path: publisherProofPath, source: publisherProofSource }, { path: authorityPath, source: authoritySource }, { path: proofPath, source: proofSource }, { path: release.proof.path, source: releaseProofSource }, { path: proof.controls.path, source: controlsSource }, { path: projectionFrame.path, source: projectionFrame.source }];
  function guard() { controller.signal.throwIfAborted(); if (bindings.some(row => readFileSync(row.path, "utf8") !== row.source) || readFileSync(authority.path, "utf8") !== authority.before) throw Error("Consumer publication binding/predecessor advanced"); }
  const journal = openSync(journalPath, "wx");
  function append(value: unknown) { const bytes = Buffer.from(JSON.stringify(value) + "\n"); let offset = 0; while (offset < bytes.length) offset += writeSync(journal, bytes, offset, bytes.length - offset); fsyncSync(journal); }
  try {
    append({ at: new Date().toISOString(), event: "authority", publisher: { path: import.meta.path, source: publisherSource, sha256: sha(publisherSource) }, publisherIndependent: { path: publisherProofPath, source: publisherProofSource, sha256: sha(publisherProofSource) }, source: { path: authorityPath, sha256: sha(authoritySource) }, independent: { path: proofPath, sha256: sha(proofSource) }, controls: { path: proof.controls.path, sha256: sha(controlsSource) }, release: { path: releasePath, sha256: releaseHash }, row: { path: authority.path, before: authority.before, after: authority.after, inverseBody: authority.before, beforeHash: authority.beforeHash, afterHash: authority.afterHash, forward: authority.forward, inverse: authority.inverse } });
    guard(); append({ event: "begin", path: authority.path, beforeHash: authority.beforeHash, afterHash: authority.afterHash });
    const fd = openSync(authority.path, "r+");
    try { await guardReleased(); guard(); if (readFileSync(fd, "utf8") !== authority.before) throw Error("Consumer descriptor predecessor advanced"); controller.signal.throwIfAborted(); const bytes = Buffer.from(authority.after); let offset = 0; while (offset < bytes.length) offset += writeSync(fd, bytes, offset, bytes.length - offset, offset); ftruncateSync(fd, bytes.length); fsyncSync(fd); } finally { closeSync(fd); }
    if (readFileSync(authority.path, "utf8") !== authority.after) throw Error("Consumer immediate successor advanced");
    append({ event: "complete", path: authority.path, beforeHash: authority.beforeHash, afterHash: authority.afterHash });
    const result = { at: new Date().toISOString(), publisher: { path: import.meta.path, sha256: sha(publisherSource) }, publisherIndependent: { path: publisherProofPath, sha256: sha(publisherProofSource) }, authority: { path: authorityPath, sha256: sha(authoritySource) }, independent: { path: proofPath, sha256: sha(proofSource) }, release: { path: releasePath, sha256: releaseHash }, journalPath, path: authority.path, beforeHash: authority.beforeHash, afterHash: authority.afterHash, currentExact: true, wholePlannerExecuted: false, nativeExecuted: false, sourceWritesOutsideTicket: false };
    append({ event: "published", ...result }); save(out, result);
    console.log(JSON.stringify({ out, writes: 1, currentExact: true, wholePlannerExecuted: false, nativeExecuted: false }));
  } finally { closeSync(journal); }
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
} else if (command === "record-owning") {
  const authorityPath = resolve(ticket, "🗑️generated/neutral-render-owner/source-projection-source-1.json"), authoritySource = readFileSync(authorityPath, "utf8"), authority = JSON.parse(authoritySource), logPath = resolve(generated, "owning-bun-1.log"), raw = readFileSync(logPath, "utf8");
  const observations = raw.split("\n").filter(line => line.startsWith("[DEBUG] source projection case ")).map(line => JSON.parse(line.slice("[DEBUG] source projection case ".length)));
  if (!raw.includes("1 pass") || !raw.includes("0 fail") || !raw.includes("79 expect() calls") || observations.length !== 33 || !isDeepStrictEqual(observations, authority.observations)) throw Error("Owning published source projection runtime evidence differs");
  const checks = authority.rows.map((row: any) => { const source = existsSync(resolve(root, row.path)) ? readFileSync(resolve(root, row.path), "utf8") : null; return { path: row.path, source, expected: row.afterHash, actual: source === null ? null : sha(source), currentExact: source === row.after }; });
  if (checks.some((row: any) => !row.currentExact)) throw Error("Published source projection advanced during owning runtime verification");
  save(resolve(generated, "owning-runtime-" + epoch + ".json"), { at: new Date().toISOString(), authority: { path: authorityPath, sha256: sha(authoritySource) }, log: { path: logPath, source: raw, sha256: sha(raw) }, checks, observations, tests: 1, assertions: 79, passed: 1, failed: 0, runtimeExecuted: true, nativeExecuted: false, consumerWholeExecuted: false, sourceWritesOutsideTicket: false });
  console.log(JSON.stringify({ currentSources: checks.length, cases: observations.length, tests: 1, passed: 1, failed: 0, assertions: 79, nativeExecuted: false }));
} else if (command === "test-stage") {
  const { runBudgetedTestCommand } = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", resolve(templates, "🧪️tests/🟦️.ts")], { cwd: root, budgetMs: 120000 });
} else throw Error("Unknown source projection command: " + command);
