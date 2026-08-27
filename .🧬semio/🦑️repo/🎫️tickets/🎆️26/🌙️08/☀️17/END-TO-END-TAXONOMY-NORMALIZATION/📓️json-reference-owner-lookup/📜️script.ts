import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";

const root = process.cwd(), report = import.meta.dir;
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const sourcePath = join(root, library, "🧹️normalization/🟦️.ts");
const vectorPath = join(root, library, "🧪️tests/🧪️json-reference-owner-lookup/🔣️.json");
const mode = process.argv[2], hash = (value: string | Uint8Array) => createHash("sha256").update(value).digest("hex");
if (mode !== "before" && mode !== "after" && mode !== "paired") throw new Error("Expected before, after or paired");
const bundlePath = join(report, "🟨️" + mode + ".generated.js"), outputPath = join(report, "📊️" + mode + ".generated.json");
if (existsSync(bundlePath) || existsSync(outputPath)) throw new Error("Exact diagnostic outputs already exist");
const vector = JSON.parse(readFileSync(vectorPath, "utf8")), source = readFileSync(sourcePath, "utf8");
const syntax = ts.createSourceFile("🟦️.ts", source, ts.ScriptTarget.Latest, true);
const parser = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "jsonTokens");
if (parser.length !== 1) throw new Error("Expected one jsonTokens declaration");
const inputs = vector.corpus.map((path: string) => ({ path, physicalPath: path }));
const energyPath = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🔣️component.json";
inputs.push({ path: energyPath, physicalPath: energyPath });
const noFollowRead = (path: string): Buffer => {
  let current = root;
  const parts = path.split("/");
  if (path.startsWith("/") || parts.some((part) => !part || part === "." || part === "..") || ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(opaque + "/"))) throw new Error("Unsafe corpus path");
  for (let index = 0; index < parts.length; index++) {
    current = join(current, parts[index]);
    const stat = lstatSync(current);
    if (stat.isSymbolicLink() || !(index === parts.length - 1 ? stat.isFile() : stat.isDirectory())) throw new Error("Unsafe corpus ancestor: " + path);
  }
  return readFileSync(current);
};
const contents = inputs.map((input: { path: string; physicalPath: string }) => ({ ...input, content: noFollowRead(input.physicalPath).toString("utf8") }));
if (mode === "paired") {
  const modes = ["before", "after"], records = modes.map((phase) => JSON.parse(readFileSync(join(report, "📊️" + phase + ".generated.json"), "utf8")));
  const modules = await Promise.all(modes.map(async (phase, index) => {
    const path = join(report, "🟨️" + phase + ".generated.js"), stat = lstatSync(path);
    if (!stat.isFile() || stat.isSymbolicLink() || hash(readFileSync(path)) !== records[index].bundleSha256) throw new Error("Bundle preimage drift");
    return import(path);
  }));
  const rows = [], started = performance.now();
  for (let index = 0; index < contents.length; index++) {
    const input = contents[index], rounds: number[][] = [[], []];
    for (const record of records) if (record.rows[index].path !== input.path || record.rows[index].sourceSha256 !== hash(input.content)) throw new Error("Paired corpus preimage drift");
    for (const module of modules) module.referenceTokens(input.path, input.content);
    for (let iteration = 0; iteration < 11; iteration++) for (const variant of iteration % 2 ? [1, 0] : [0, 1]) {
      const start = performance.now(), tokens = modules[variant].referenceTokens(input.path, input.content);
      rounds[variant].push(performance.now() - start);
      if (JSON.stringify(tokens) !== records[variant].rows[index].tokenJson) throw new Error("Paired token parity failed");
      if (performance.now() - started > 45_000) throw new Error("Paired timing exceeded 45 seconds");
    }
    if (hash(noFollowRead(input.physicalPath)) !== hash(input.content)) throw new Error("Paired corpus changed");
    rows.push({ path: input.path, sourceSha256: hash(input.content), tokenSha256: records[0].rows[index].tokenSha256, before: rounds[0], after: rounds[1], beforeMedian: [...rounds[0]].sort((a, b) => a - b)[5], afterMedian: [...rounds[1]].sort((a, b) => a - b)[5] });
  }
  console.log("[DEBUG] paired JSON owner corpus", JSON.stringify({ exactBeforeAfterParity: true, rows, milliseconds: performance.now() - started }));
  process.exit(0);
}
const built = await Bun.build({ entrypoints: [sourcePath], target: "bun", format: "esm", packages: "external", plugins: [{ name: "json-owner-corpus", setup(build) { build.onLoad({ filter: /\.ts$/u }, (args) => resolve(args.path) === sourcePath ? { contents: source + "\nexport { referenceTokens };\n", loader: "ts" } : undefined); } }] });
if (!built.success || built.outputs.length !== 1) throw new Error(JSON.stringify(built.logs));
await Bun.write(bundlePath, built.outputs[0]);
const { referenceTokens } = await import(bundlePath);
const started = performance.now(), rows = [];
for (const input of contents) {
  const tokenJson = JSON.stringify(referenceTokens(input.path, input.content)), rounds = [];
  for (let iteration = 0; iteration < 7; iteration++) {
    const start = performance.now(), tokens = referenceTokens(input.path, input.content);
    rounds.push(performance.now() - start);
    if (JSON.stringify(tokens) !== tokenJson) throw new Error("Unstable token output: " + input.path);
    if (performance.now() - started > 45_000) throw new Error("Bounded JSON comparison exceeded 45 seconds");
  }
  if (hash(noFollowRead(input.physicalPath)) !== hash(input.content)) throw new Error("Corpus input drift: " + input.path);
  rows.push({ path: input.path, bytes: Buffer.byteLength(input.content), sourceSha256: hash(input.content), tokenSha256: hash(tokenJson), tokenCount: JSON.parse(tokenJson).length, tokenJson, rounds });
}
const result = { schemaVersion: 1, mode, sourceSha256: hash(source), functionSha256: hash(parser[0].getText(syntax)), bundleSha256: hash(readFileSync(bundlePath)), rows };
if (mode === "after") {
  const previous = JSON.parse(readFileSync(join(report, "📊️before.generated.json"), "utf8"));
  if (previous.rows.length !== rows.length) throw new Error("Corpus membership changed");
  for (let index = 0; index < rows.length; index++) for (const key of ["path", "bytes", "sourceSha256", "tokenSha256", "tokenCount", "tokenJson"]) if (previous.rows[index][key] !== rows[index][key]) throw new Error("Before/after corpus changed at " + rows[index].path + ": " + key);
}
await Bun.write(outputPath, JSON.stringify(result) + "\n");
console.log("[DEBUG] JSON owner corpus", JSON.stringify({ ...result, rows: rows.map(({ tokenJson, ...row }) => row), milliseconds: performance.now() - started, exactBeforeAfterParity: mode === "after" }));
