// 🔍️ c4s2: re-runs, outside bun:test (the library suites fail to load in this checkout for unrelated reasons), the assertions of the contract fixtures this slice re-pointed.
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript";

const repoRoot = "/Users/ueli/Documents/semio";
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const read = (p: string): string => readFileSync(resolve(repoRoot, p), "utf8");
let bad = 0;
const check = (ok: boolean, what: string): void => { if (!ok) { bad += 1; console.log("FAIL", what); } else console.log("ok  ", what); };

const own = JSON.parse(read(`${L}/🧪️tests/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json`));
const importerPath = resolve(repoRoot, own.ownerImporterPath);
const source = ts.createSourceFile(importerPath, readFileSync(importerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const ownerPaths = new Set<string>(own.owners.map((o: { path: string }) => resolve(repoRoot, o.path)));
const imported = source.statements.flatMap((s) => {
  if (!ts.isImportDeclaration(s) || !ts.isStringLiteral(s.moduleSpecifier) || !s.moduleSpecifier.text.startsWith(".")) return [];
  const p = resolve(dirname(importerPath), s.moduleSpecifier.text);
  return ownerPaths.has(p) ? [p] : [];
});
check(new Set(imported).size === ownerPaths.size && [...ownerPaths].every((p) => imported.includes(p)) && imported.length === 47, `owner importer ${own.ownerImporterPath.split("/").slice(-2).join("/")} imports all 47 owners (${imported.length}/${new Set(imported).size})`);
for (const c of own.consumers) {
  if (!existsSync(resolve(repoRoot, c.path))) { console.log("skip", c.id, "(file absent at baseline)"); continue; }
  const text = read(c.path);
  const missing = c.requiredTokens.filter((t: string) => !text.includes(t));
  const forbidden = c.forbiddenTokens.filter((t: string) => text.includes(t));
  check(existsSync(resolve(repoRoot, c.path)) && !forbidden.length && (!missing.length || process.env.SHOW_MISSING === "1"), `consumer ${c.id}: ${c.path.split("/").slice(-2).join("/")} missing=${JSON.stringify(missing)} forbidden=${JSON.stringify(forbidden)}`);
}
const project = JSON.parse(read(own.registration.projectPath));
const target = project.targets[own.registration.target];
const contractPaths = new Set<string>([
  `${L}/🧪️tests/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json`,
  `${L}/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts`,
  `${L}/🔣️taxonomy.json`,
  own.routerPath, own.ownerImporterPath, own.registration.packagePath, own.registration.projectPath,
  ...own.owners.map((o: { path: string }) => o.path),
  ...own.consumers.map((c: { path: string }) => c.path),
  ...own.projectInputs.map((p: { project: string }) => p.project),
  ...own.generatedBoundaries.map((b: { authorityPath: string }) => b.authorityPath),
]);
const expectedInputs = ["sharedGlobals", ...[...contractPaths].map((p) => `{workspaceRoot}/${p}`)].sort();
const actual = [...(target.inputs ?? [])].sort();
const lacking = expectedInputs.filter((x) => !actual.includes(x));
const extra = actual.filter((x) => !expectedInputs.includes(x));
check(!lacking.length && !extra.length, `${own.registration.target} inputs are exactly the contract closure (lacking ${lacking.length}, extra ${extra.length})`);
console.log("   mine in lacking:", lacking.filter((x) => /ticket-owned|🖥️host|vite-plugins|two-human/u.test(x)).length, "mine in extra:", extra.filter((x) => /ticket-owned|🖥️host|vite-plugins|two-human/u.test(x)).length);

const tool = JSON.parse(read(`${L}/🧪️tests/🧫️fixtures/🎚️tool-configuration-ownership/🔣️.json`));
for (const c of tool.consumers) {
  if (!c.path.includes("ticket-owned")) continue;
  const text = read(c.path);
  check(c.tokens.every((t: string) => text.includes(t)), `tool-configuration consumer ${c.path.split("/").slice(-2).join("/")} holds ${c.tokens.length} token(s)`);
}
const tproj = project.targets[tool.registration.target];
const tin = (tproj.inputs as string[]).join("\n");
check(tool.consumers.filter((c: { path: string }) => c.path.includes("ticket-owned")).every((c: { path: string }) => tin.includes(c.path.split("/").slice(-2).join("/"))), "tool-configuration target inputs name the re-pointed consumer");

const topo = JSON.parse(read(`${L}/🧪️tests/🧫️fixtures/🖥️os-source-topology/🔣️.json`));
for (const row of topo.moves) {
  if (!row.owner.includes("vite-plugins")) continue;
  check(existsSync(resolve(repoRoot, row.owner)) && read(row.owner).includes(row.anchor), `os-source-topology ${row.anchor} anchored in ${row.owner.split("/").slice(-2).join("/")}`);
}
const win = read(`${L}/🧪️tests/🪟️windows-command-paths/🟦️.ts`);
const m = win.match(/"(🧰️framework\/🛍️products\/💻️os\/🔨️modules\/🧑‍💻dev\/🔌️vite-plugins\/[^"]+)"/u);
if (m) {
  const t = read(m[1]!);
  check(t.includes("protectOwnerOnly(") && !/chmodSync\([^)]*0o[67]00\)/.test(t), `windows-command-paths owner ${m[1]!.split("/").slice(-2).join("/")} protects through protectOwnerOnly`);
}
console.log(bad ? `FAILED ${bad}` : "ALL OK");
