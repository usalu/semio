#!/usr/bin/env bun
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { createHash } from "node:crypto";
import * as toml from "@iarna/toml";
import JSON5 from "json5";

const epoch = process.argv[2];
if (!epoch || !/^[1-9][0-9]*$/u.test(epoch)) throw Error("Expected a fresh positive epoch");
let root = import.meta.dir;
while (!existsSync(resolve(root, "nx.json"))) {
  const parent = dirname(root);
  if (parent === root) throw Error("Missing workspace");
  root = parent;
}
const ticket = resolve(import.meta.dir, ".."), out = resolve(ticket, "🗑️generated/current-native-prerequisites");
mkdirSync(out, { recursive: true });
const resultPath = resolve(out, `root-current-${epoch}.json`);
if (existsSync(resultPath)) throw Error("Epoch already exists");
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const taxonomyPath = library + "/🔣️taxonomy.json", discoveryPath = library + "/🔍️discovery/🟦️.ts";
const registryPath = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml";
const frames = new Map<string, any>();
function capture(path: string) {
  if (frames.has(path)) return frames.get(path).source as string;
  const source = readFileSync(resolve(root, path), "utf8");
  frames.set(path, { path, source, sha256: hash(source), observedAt: new Date().toISOString() });
  return source;
}
const taxonomySource = capture(taxonomyPath), taxonomy = JSON.parse(taxonomySource);
if (JSON.stringify(taxonomy) !== JSON.stringify(JSON5.parse(taxonomySource))) throw Error("Taxonomy parser disagreement");
capture(discoveryPath);
const registrySource = capture(registryPath), registry = Bun.TOML.parse(registrySource), independentRegistry = toml.parse(registrySource);
if (JSON.stringify(registry) !== JSON.stringify(independentRegistry)) throw Error("Registry manifest parser disagreement");
const recordDeclarations = registrySource.split("\n").filter(line => /^semio-framework-dsl-record\s*=/u.test(line));
const record = (registry as any).dependencies?.["semio-framework-dsl-record"];
if (recordDeclarations.length !== 1 || typeof record?.path !== "string") throw Error("Canonical Record declaration must exist exactly once");
const providerPath = resolve(dirname(resolve(root, registryPath)), record.path, "Cargo.toml");
const providerSource = capture(providerPath), provider = Bun.TOML.parse(providerSource);
if (JSON.stringify(provider) !== JSON.stringify(toml.parse(providerSource)) || (provider as any).package?.name !== "semio-framework-dsl-record") throw Error("Record provider identity differs");
const outputMatches: any[] = [];
for (const [contractId, contract] of Object.entries(taxonomy.generatorContracts) as [string, any][]) {
  for (const output of contract.outputRoots ?? []) {
    if (!output.path.endsWith("/🎞️frame-worker/🤖️generated/🟨️.js")) continue;
    const producer = output.producer;
    if (!producer) throw Error("Worker output has no producer");
    const projectPath = producer.ownerPath + "/📋️project.json", projectSource = capture(projectPath);
    const project = JSON.parse(projectSource);
    if (JSON.stringify(project) !== JSON.stringify(JSON5.parse(projectSource))) throw Error("Project parser disagreement");
    const separator = producer.target.lastIndexOf(":"), targetName = producer.target.slice(separator + 1);
    outputMatches.push({ contractId, output, projectName: project.name, targetName, target: project.targets?.[targetName], producerPresent: project.name === producer.target.slice(0, separator) && Boolean(project.targets?.[targetName]) });
  }
}
if (!outputMatches.length) throw Error("Actual taxonomy contains no frame worker output");
const { validateGeneratorContractsAgainstWorkspace } = await import(resolve(root, discoveryPath));
const problems = validateGeneratorContractsAgainstWorkspace(root, taxonomy);
const workerProblems = problems.filter((problem: string) => problem.includes("/🎞️frame-worker/🤖️generated/🟨️.js") && problem.includes("no declared Nx producer"));
const immediateGuards = [...frames.values()].map(frame => ({ path: frame.path, exact: readFileSync(resolve(root, frame.path), "utf8") === frame.source }));
const result = { schemaVersion: 1, observedAt: new Date().toISOString(), sourceWrites: 0, provenance: "Fresh current observations; no claim about historical Nx consumption or concurrent authorship", registry: { path: registryPath, declarations: recordDeclarations.length, parserParity: true, providerPath, providerIdentity: (provider as any).package.name }, outputMatches, actualValidatorProblems: problems, workerProblems, frames: [...frames.values()], immediateGuards };
writeFileSync(resultPath, JSON.stringify(result, null, 2) + "\n");
console.log(JSON.stringify({ resultPath, sourceWrites: 0, recordDeclarations: recordDeclarations.length, outputMatches: outputMatches.length, producersPresent: outputMatches.every(row => row.producerPresent), workerProblems: workerProblems.length, allGeneratorProblems: problems.length, immediateGuardGaps: immediateGuards.filter(row => !row.exact).length }));
if (workerProblems.length || outputMatches.some(row => !row.producerPresent) || immediateGuards.some(row => !row.exact)) process.exitCode = 1;
