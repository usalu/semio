/**
 * 🧪️ Stand-alone mirror of the staged `sdk-foundation-admission` test against an overlay COPY (never the live tree): corpus verdicts directly and by the dependency-cruiser oracle, closedness errors, schema validity and workspace existence of the real declaration.
 *
 *   bun verify-staged.ts <overlay-dir>
 */
import Ajv from "ajv/dist/2020.js";
import { builtinModules, createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../..");
const overlay = resolve(process.argv[2]!);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const direction = `${library}/🕸️dependencies/🧭️direction`;
const require = createRequire(import.meta.url);
const { buildDependencyDirectionPolicy } = require(join(overlay, direction, "🏗️construction/🟨️.cjs"));
const { loadDependencyDirectionPolicy } = require(join(root, direction, "🚀️bootstrap/🟨️.cjs"));
const corpus = JSON.parse(readFileSync(join(import.meta.dir, "staged/60-sdk-foundation-admission.fixture.json"), "utf8"));
const taxonomy = JSON.parse(readFileSync(join(overlay, library, "🔣️taxonomy.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(overlay, library, "🧬️schema/🧱️dependency-direction/🔣️.json"), "utf8"));
const cruiser = pathToFileURL(Bun.resolveSync("dependency-cruiser", root));
const { validateDependency } = await import(new URL("../validate/index.mjs", cruiser).href);
const { default: normalizeRules } = await import(new URL("./rule-set/normalize.mjs", cruiser).href);
const failures: string[] = [];
const check = (ok: boolean, label: string): void => { if (!ok) failures.push(label); };
const list = (v: unknown): string[] => (v === undefined ? [] : typeof v === "string" ? [v] : (v as string[]));
const hit = (value: string, selector: unknown): boolean => list(selector).some(p => new RegExp(p, "u").test(value));
const selects = (value: string, s: any): boolean => (s.path === undefined || hit(value, s.path)) && !hit(value, s.pathNot);
const policyOf = (foundation: unknown, packages = corpus.packages): any => buildDependencyDirectionPolicy({ taxonomy: { ...taxonomy, dependencyDirections: { ...taxonomy.dependencyDirections, publicApi: { ...taxonomy.dependencyDirections.publicApi, sdkFoundationPackages: foundation } } }, plugins: [], workspacePackages: packages.map((row: any) => ({ ...row, exports: [...new Set(row.exportTargets.map((t: any) => t.subpath))] })), nodeBuiltins: builtinModules });

const policy = policyOf(corpus.foundationPackages);
for (const row of corpus.cases) {
  const rule = policy.forbidden.find((r: any) => r.name === row.rule);
  const direct = selects(row.from, rule.from) && selects(row.to, rule.to);
  const oracle = validateDependency(normalizeRules({ forbidden: [rule] }), { source: row.from }, { resolved: row.to, dependencyTypes: [], circular: false });
  check(direct === row.forbidden, `${row.id} direct`);
  check(!oracle.valid === row.forbidden, `${row.id} oracle`);
}
const bare = policyOf(["@semio-tech/framework-undeclared"]);
for (const row of corpus.cases.filter((r: any) => r.id.includes("foundation") && !r.forbidden)) {
  const rule = bare.forbidden.find((r: any) => r.name === row.rule);
  check(selects(row.from, rule.from) && selects(row.to, rule.to), `${row.id} bare policy must forbid`);
}
const { publicApi } = taxonomy.dependencyDirections;
const closed = (value: unknown): boolean => { try { policyOf(value, []); return false; } catch (error) { return /SDK foundation package|public API authority/u.test(String(error)); } };
for (const bad of [undefined, [], "@semio-tech/ui", ["@semio-tech/ui", "@semio-tech/ui"], ["ui"], [publicApi.sdkPackage], [1]]) check(closed(bad), `closedness ${JSON.stringify(bad)}`);
check(!closed(publicApi.sdkFoundationPackages), "real declaration admitted");
const validate = new Ajv({ strict: true }).compile(schema);
check(validate(taxonomy.dependencyDirections) === true, `schema ${JSON.stringify(validate.errors)}`);
const names = new Set(loadDependencyDirectionPolicy(root).workspacePackages.map((p: any) => p.name));
for (const name of publicApi.sdkFoundationPackages) check(names.has(name), `workspace package ${name}`);
const badSchema = new Ajv({ strict: true }).compile(schema);
check(badSchema({ ...taxonomy.dependencyDirections, publicApi: { ...publicApi, sdkFoundationPackages: [] } }) === false, "schema rejects empty list");
check(badSchema({ ...taxonomy.dependencyDirections, publicApi: { ...publicApi, sdkFoundationPackages: ["x"] } }) === false, "schema rejects unscoped name");
const { sdkFoundationPackages: _omitted, ...withoutList } = publicApi;
check(badSchema({ ...taxonomy.dependencyDirections, publicApi: withoutList }) === false, "schema requires the list");
console.log(`cases ${corpus.cases.length}, failures ${failures.length}`);
if (failures.length) { console.error(failures.join("\n")); process.exit(1); }
