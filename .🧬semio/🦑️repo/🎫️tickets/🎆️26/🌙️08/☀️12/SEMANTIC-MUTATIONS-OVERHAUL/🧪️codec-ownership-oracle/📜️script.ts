import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";

const workspace = process.cwd();
const fixturePath = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-codec-ownership/🧫️fixtures/🔣️.json");
const discoveryPath = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts");
const policyPath = join(workspace, "📜️script.ts");
const ticketRoot = join(workspace, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️codec-ownership-oracle");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { mutationRoot: string; aggregate: string; cases: { name: string; source: string; expected: string[] }[] };
const { canonicalPrimaryFilenameForKind, createRustMutationCodecOwnershipInspector, loadTaxonomy } = await import(discoveryPath);
const { policyMutationStructuralBreaches } = await import(policyPath);
const inspect = createRustMutationCodecOwnershipInspector(fixture.aggregate);
const taxonomy = loadTaxonomy();
const rustComponent = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
const root = mkdtempSync(join(ticketRoot, "🧫️fixture-root-"));
const compilerOutput = join(root, "📦️compiler-output");
let failures = 0;
console.log(`[DEBUG] canonical mutation Rust filename: ${rustComponent}`);
console.log(`[DEBUG] retained fixture root: ${root}`);
console.log(`[DEBUG] compiler output directory: ${compilerOutput}`);
{
  const aggregate = join(root, fixture.mutationRoot, rustComponent);
  const codec = join(root, fixture.mutationRoot, "📝️text", rustComponent);
  mkdirSync(dirname(codec), { recursive: true });
  mkdirSync(compilerOutput, { recursive: true });
  writeFileSync(aggregate, fixture.aggregate);
  for (const vector of fixture.cases) {
    const source = `${fixture.aggregate}\n${vector.source}`;
    const parser = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "-"], { encoding: "utf8", input: source });
    const compiler = spawnSync("rustc", ["--crate-type=lib", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "--out-dir", compilerOutput, "-"], { encoding: "utf8", input: source });
    writeFileSync(codec, vector.source);
    const facts = inspect(vector.source).map(({ kind }) => kind);
    const expectedBreach = vector.expected.length > 0;
    const policyBreach = policyMutationStructuralBreaches(root, [fixture.mutationRoot]).some((breach) => breach.kind === "mutation/codec-ownership");
    const valid = parser.status === 0 && (compiler.status === 0) === !expectedBreach && JSON.stringify(facts) === JSON.stringify(vector.expected) && policyBreach === expectedBreach;
    console.log(`[DEBUG] ${vector.name} parser=${parser.status} compiler=${compiler.status} inspector=${JSON.stringify(facts)} policy=${policyBreach} ${valid ? "pass" : "FAIL"}`);
    if (!valid) {
      failures += 1;
      if (parser.stderr) console.error(`[DEBUG] parser stderr ${vector.name}\n${parser.stderr}`);
      if (compiler.stderr) console.error(`[DEBUG] compiler stderr ${vector.name}\n${compiler.stderr}`);
    }
  }
}
if (failures > 0) throw new Error(`codec ownership oracle mismatches: ${failures}`);
console.log(`[DEBUG] codec ownership oracle passed ${fixture.cases.length}/${fixture.cases.length} vectors`);
