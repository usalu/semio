// ⚖️ Runs the cache-correctness law and the closed project schema over the demonstrator and report manifests of an overlay root (default /private/tmp/dbz/ov).
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
const live = "/Users/ueli/Documents/semio", root = process.argv[2] ?? "/private/tmp/dbz/ov";
const L = live + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { cacheCorrectnessFindingsV1 } = await import(L + "/⚖️laws/🔒️cache-correctness/🟦️.ts");
const Ajv = createRequire(live + "/package.json")("ajv").default;
const schema = JSON.parse(readFileSync(L + "/🏗️build/🧬️schema/📋️project/🔣️.json", "utf8"));
const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
let bad = 0;
const rows: [string, string, string[]][] = [
  ["♻️mit-bestand/🧺️demonstrator", "♻️mit-bestand/🧺️demonstrator", ["prepare-prototype", "prepare-release", "activate-prototype", "activate-lanes-prototype", "serve", "dev", "prepare-e2e", "serve-e2e", "test-e2e", "build"]],
  ["♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript", "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript", ["build-zwischenbericht", "build-forschungsbericht", "build-kompaktbericht", "build", "generate-actor-network", "akteursnetz-validate"]],
  ["♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript", "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript", ["build", "test", "canonical-architecture", "dev"]],
];
for (const [dir, projectRoot, targets] of rows) {
  const project = JSON.parse(readFileSync(join(root, dir, "📋️project.json"), "utf8"));
  const ok = validate(project);
  console.log(project.name, "schema", ok ? "valid" : JSON.stringify(validate.errors?.slice(0, 3)));
  if (!ok) bad++;
  for (const name of targets) {
    const authored = project.targets[name];
    const target = { ...authored, inputs: [...(authored.inputs ?? ["default"]), { env: "SEMIO_BUILD_MODE" }] };
    const findings = cacheCorrectnessFindingsV1({ id: `${project.name}:${name}`, root: projectRoot, authoredCache: authored.cache ?? null, target, namedInputs: { sharedGlobals: [{ env: "SEMIO_BUILD_MODE" }], ...(project.namedInputs ?? {}), default: ["{projectRoot}/**/*"] } });
    console.log(`  ${name}`, JSON.stringify(findings));
    bad += findings.length;
  }
}
process.exit(bad ? 1 : 0);
