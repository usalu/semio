#!/usr/bin/env bun
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import ts from "typescript";
import { createPatch, applyPatch } from "diff";
import { expect } from "bun:test";

const epoch = process.argv[2];
if (!epoch || !/^[1-9][0-9]*$/u.test(epoch)) throw Error("Expected a fresh epoch");
let root = import.meta.dir;
while (!existsSync(resolve(root, "nx.json"))) {
  const parent = dirname(root);
  if (parent === root) throw Error("Missing workspace");
  root = parent;
}
const ticket = resolve(import.meta.dir, ".."), out = resolve(ticket, "🗑️generated/cargo-preparation-nul-test-first", epoch);
if (existsSync(out)) throw Error("Epoch already exists");
mkdirSync(out, { recursive: true });
process.env.SEMIO_TEST_ARTIFACT_DIR = out;
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo";
const parserPath = base + "/🟦️.ts", schemaPath = base + "/🧬️schema/🛠️preparation/🔣️.json", fixturePath = base + "/🧫️fixtures/🔣️.json", lawPath = base + "/🧪️tests/🟦️.ts";
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const frames = [parserPath, schemaPath, fixturePath, lawPath].map(path => {
  const source = readFileSync(resolve(root, path), "utf8");
  return { path, source, sha256: hash(source), observedAt: new Date().toISOString() };
});
const before = new Map(frames.map(frame => [resolve(root, frame.path), frame.source]));
const additions = ["\u0000dir/📜️script.ts", "dir/\u0000child/📜️script.ts", "dir\u0000/📜️script.ts"].map(script => ({ script, command: ["build", "0"] }));
const fixture = JSON.parse(before.get(resolve(root, fixturePath))!);
if (!Array.isArray(fixture.preparation?.rejected)) throw Error("Original preparation corpus is missing");
fixture.preparation.rejected.push(...additions);
const fixtureAfter = JSON.stringify(fixture, null, 2) + "\n";
const needle = String.raw`|| row.script.includes("\\"))`, replacement = String.raw`|| row.script.includes("\\") || row.script.includes("\0"))`;
const parserBefore = before.get(resolve(root, parserPath))!;
if (parserBefore.split(needle).length !== 2) throw Error("Non-unique preparation script guard");
const parserAfter = parserBefore.replace(needle, replacement), schema = JSON.parse(before.get(resolve(root, schemaPath))!);
const originalPattern = schema.properties.script.pattern;
if (typeof originalPattern !== "string" || !originalPattern.startsWith("^")) throw Error("Missing original script pattern");
schema.properties.script.pattern = originalPattern.replace("^", "^(?![\\s\\S]*\\u0000)");
const schemaAfter = JSON.stringify(schema, null, 2) + "\n";
const rows = [[fixturePath, fixtureAfter, "test-only"], [schemaPath, schemaAfter, "production"], [parserPath, parserAfter, "production"]].map(([path, authored, phase]) => {
  const source = before.get(resolve(root, path!))!, forward = createPatch(path!, source, authored!), inverseEdit = createPatch(path!, authored!, source);
  if (applyPatch(source, forward, { fuzzFactor: 0 }) !== authored || applyPatch(authored!, inverseEdit, { fuzzFactor: 0 }) !== source) throw Error("Inverse failed");
  return { path, phase, before: source, authored, beforeHash: hash(source), authoredHash: hash(authored!), inverse: source, forward, inverseEdit };
});
function loadModule(path: string, overlay: Map<string, string>, dependencies: Record<string, any>) {
  const source = (overlay.get(resolve(root, path)) ?? readFileSync(resolve(root, path), "utf8")).replaceAll("import.meta.url", JSON.stringify(pathToFileURL(resolve(root, path)).href));
  const transformed = ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS, esModuleInterop: true } }).outputText;
  const local = createRequire(resolve(root, path)), exports: Record<string, any> = {};
  const fs = local("node:fs");
  const require = (name: string) => name in dependencies ? dependencies[name] : name === "node:fs" ? { ...fs, readFileSync: (input: any, encoding: any) => overlay.get(input instanceof URL ? fileURLToPath(input) : resolve(String(input))) ?? fs.readFileSync(input, encoding) } : local(name);
  new Function("require", "exports", transformed)(require, exports);
  return exports;
}
const phases: any[] = [];
for (const phase of ["test-only", "production"]) {
  const overlay = new Map(before);
  for (const row of rows.filter(row => phase === "production" || row.phase === "test-only")) overlay.set(resolve(root, row.path!), row.authored!);
  const parser = loadModule(parserPath, overlay, {}), callbacks: { name: string; run: () => unknown }[] = [];
  loadModule(lawPath, overlay, { "../🟦️.ts": parser, "bun:test": { expect, test: (name: string, run: () => unknown) => callbacks.push({ name, run }) } });
  const selected = callbacks.filter(callback => callback.name === "portable preparation recipes match the independent closed schema");
  if (selected.length !== 1) throw Error("Original recipe law is not unique");
  let failure: string | null = null;
  try { await selected[0]!.run(); } catch (error) { failure = String(error); }
  const observations = additions.map(recipe => { try { parser.parseCargoPreparation(recipe); return { recipe, accepted: true }; } catch { return { recipe, accepted: false }; } });
  phases.push({ phase, originalRegisteredLaws: callbacks.length, selectedLaw: selected[0]!.name, passed: failure === null, failure, observations, nativeExecuted: 0 });
}
const oracle = Bun.spawnSync(["node", "-e", "const fs=require('node:fs');const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>{try{fs.statSync(row.script);return false;}catch(e){return e.code==='ERR_INVALID_ARG_VALUE';}})));", JSON.stringify(additions)], { timeout: 2_000, stdout: "pipe", stderr: "pipe" });
if (oracle.exitCode !== 0) throw Error("Bounded Node path oracle failed: " + oracle.stderr.toString());
const nodeRefused = JSON.parse(oracle.stdout.toString());
const { validateJsonSchemaSubset } = await import(resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
const firstPartySchemaRefused = additions.map(row => validateJsonSchemaSubset(schema, row).length > 0);
const firstPartySchemaCases = [...fixture.preparation.accepted.map((recipe: any) => ({ recipe, expected: true })), ...fixture.preparation.rejected.map((recipe: any) => ({ recipe, expected: false }))].map(row => ({ ...row, errors: validateJsonSchemaSubset(schema, row.recipe) }));
const guards = frames.map(frame => ({ path: frame.path, exact: readFileSync(resolve(root, frame.path), "utf8") === frame.source }));
const proof = { schemaVersion: 1, frames, rows, phases, additions, nodeRefused, firstPartySchemaRefused, firstPartySchemaCases, guards, sourceWrites: 0, published: false, nativeExecuted: 0, scope: "Actual original pure recipe-law callback only; all other original callbacks conserved but unexecuted, full native workspace cohort required during owner transfer" };
writeFileSync(resolve(import.meta.dir, `root-staged-test-first-${epoch}.json`), JSON.stringify(proof, null, 2) + "\n");
writeFileSync(resolve(out, "proof.json"), JSON.stringify(proof, null, 2) + "\n");
if (phases[0].passed || !phases[1].passed || phases[0].observations.some((row: any) => !row.accepted) || phases[1].observations.some((row: any) => row.accepted) || nodeRefused.some((value: boolean) => !value) || firstPartySchemaRefused.some(value => !value) || firstPartySchemaCases.some(row => (row.errors.length === 0) !== row.expected) || guards.some(row => !row.exact)) throw Error("Actual original-law RED/GREEN or path admission proof failed");
console.log(JSON.stringify({ output: resolve(out, "proof.json"), actualOriginalLawRed: true, actualOriginalLawGreen: true, addedNulPaths: additions.length, firstPartySchemaCases: firstPartySchemaCases.length, originalRegisteredLaws: phases[0].originalRegisteredLaws, sourceWrites: 0, nativeExecuted: 0 }));
