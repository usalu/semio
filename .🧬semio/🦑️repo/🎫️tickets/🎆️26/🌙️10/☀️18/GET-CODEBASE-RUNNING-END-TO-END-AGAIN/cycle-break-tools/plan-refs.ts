/** 🔗 Stages the edits of the non-source references (taxonomy policy and wgpu worker profile, fixtures, project inputs) to every path and specifier the delta moves, re-keys or deletes. */
import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { Plan, type JsonOp } from "./plan.ts";
import { finish } from "./direct.ts";
import { ROOT, posix } from "./lib.ts";
import { entryOfSpec, pkgDirOf } from "./refs-map.ts";
import { FILES, sites, type Site } from "./ref-sites.ts";
const FW = "🧰️framework/";
const M = FW + "🔨️modules/";
const LIB = FW + "🛍️products/🦑️repo/🔨️modules/📚️library/";
const [TAX, PHYS, BOUNDARY, WSEXPORTS, OWNERSHIP, CLOSURE, WGPU] = FILES as [string, string, string, string, string, string, string];
const plan = new Plan();
const all = sites().filter(s => s.kind === "entry" ? s.to !== s.value.replace(/^\{workspaceRoot\}\//u, "") : true);
const of = (file: string): Site[] => all.filter(s => s.file === file);
const read = (file: string): string => readFileSync(resolve(ROOT, file), "utf8");
const pathMap = (file: string): Record<string, string> => Object.fromEntries(of(file).filter(s => s.kind !== "key" && s.to).map(s => [s.value, s.to!]));

const jsonEdit = (file: string, ops: JsonOp[], why: string): void => plan.edit({ file, mode: "json", from: ops.map(o => o.kind).join("+"), to: JSON.stringify(ops), ops, wave: "refs", why });

const imports = ["generatorContracts", "wgpu-frame-worker", "packageGeneration", "browserProfile", "workspaceImports"];
const keyRenames: Record<string, { to: string; set: Record<string, string> }> = {};
for (const s of of(TAX).filter(x => x.kind === "key")) keyRenames[s.value] = { to: s.to!, set: { manifestPath: pkgDirOf(s.to!) + "/package.json" } };
const roles = ["dependencyDirections", "roles"];
const taxonomy = JSON.parse(read(TAX));
const foundation: string[] = taxonomy.dependencyDirections.roles["ui-foundation"].ownerPaths;
const staleGlue = foundation.filter(p => p.startsWith(FW + "📦️packages/🟦️typescript/") && foundation.includes(M + p.slice((FW + "📦️packages/🟦️typescript/").length)));
const P = FW + "🔨️modules/🏃️process/📦️packages/🟦️typescript/";
const taxOps: JsonOp[] = [
  { kind: "renameKeys", pointer: imports, map: keyRenames },
  { kind: "retarget", map: pathMap(TAX) },
  { kind: "arrayRemove", pointer: [...roles, "ui-foundation", "ownerPaths"], values: staleGlue },
  { kind: "arrayInsert", pointer: [...roles, "ui-foundation", "ownerPaths"], after: M + "🛂️manifest/🏷️ui-contract-and-canvas-types", values: [M + "🛂️manifest/📋️context-menu"] },
  { kind: "arrayInsert", pointer: [...roles, "ui-foundation", "ownerPaths"], after: M + "🧩️action-argument-resolution", values: [M + "🔏️hash"] },
  { kind: "arrayInsert", pointer: [...roles, "ui-test-support", "ownerPaths"], after: P + "🧭️routing", values: [P + "🏗️build/🎟️capacity", P + "🎛️execution"] },
];
jsonEdit(TAX, taxOps, "wgpu worker profile follows the carved packages; ui-foundation admits the carved module paths and drops the glue forms; ui-test-support admits the process helpers the ui tests use");
jsonEdit(PHYS, [{ kind: "retarget", map: pathMap(PHYS) }], "retention reader sources point at the carved filesystem module");
jsonEdit(BOUNDARY, [{ kind: "retarget", map: pathMap(BOUNDARY) }, { kind: "dropRows", pointer: ["cases"], where: { id: "ui-foundation-transient-package-entry-is-allowed" } }], "the glue form of the transient entry no longer exists; the module-root entry row stays");
jsonEdit(OWNERSHIP, [{ kind: "retarget", map: pathMap(OWNERSHIP) }], "the coordinator ports moved into the server library");

const swap = (file: string, from: string, to: string, why: string): void => plan.edit({ file, mode: "text", from, to, wave: "refs", why });
{
  const text = read(WSEXPORTS);
  const spec = "@semio-tech/framework/manifest/generated/ui-axes", next = "@semio-tech/framework-manifest/generated/ui-axes";
  const dir = pkgDirOf(next)!, entry = entryOfSpec(next)!;
  const manifest = `${FW}📦️packages/🟦️typescript/package.json`;
  const target = "./🛂️manifest/🎚️ui-axes/🟦️.ts";
  const row = `{ "manifest": "${manifest}", "specifier": "${spec}", "target": "${target}", "source": "`;
  if (!text.includes(row)) throw Error("workspace-exports row changed");
  swap(WSEXPORTS, row, `{ "manifest": "${dir}/package.json", "specifier": "${next}", "target": "./${posix(relative(dir, entry))}", "source": "`, "ui-axes moved to the manifest package");
}
{
  const text = read(CLOSURE);
  const config = `${FW}🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json`;
  for (const s of of(CLOSURE).filter(x => x.kind === "key")) {
    const i = text.split("\n")[s.line - 1]!, target = text.split("\n")[s.line]!;
    const entry = entryOfSpec(s.to!)!;
    const rel = posix(relative(dirname(resolve(ROOT, config)), resolve(ROOT, entry)));
    const next = target.replace(/"target": "[^"]*"/u, `"target": "${rel}"`);
    swap(CLOSURE, i + "\n" + target, i.replace(s.value, s.to!) + "\n" + next, "server library keys moved to @semio-tech/repo-server-library");
  }
}
{
  const text = read(WGPU);
  for (const s of of(WGPU)) {
    const from = `"${s.value}"`;
    if (text.split(from).length !== 2) throw Error("project input not unique: " + s.value);
    swap(WGPU, from, `"{workspaceRoot}/${s.to}"`, "wgpu inputs follow the moved sources");
  }
}
finish(plan, "refs", process.argv);
