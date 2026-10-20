// ⚖️ Runs the cache-correctness law over the staged play manifests (targets as authored; inference adds the build-mode input to every replayed target).
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔒️cache-correctness/🟦️.ts";
const { cacheCorrectnessFindingsV1 } = await import(L);
const staged = join(import.meta.dir, "staged");
let bad = 0;
const caching = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching";
for (const [source, root, targets] of [["🏢️semio-tech/🎡️play/📋️project.json", "🏢️semio-tech/🎡️play", ["prepare-prototype", "activate-prototype"]], ["✏️s/🧑‍💻dev/📋️project.json", "✏️s/🧑‍💻dev", ["generate"]], [caching + "/📋️project.json", caching, ["generator-inputs"]]] as const) {
  const dir = createHash("sha256").update(source).digest("hex").slice(0, 8);
  const project = JSON.parse(readFileSync(join(staged, dir, "📋️project.json"), "utf8"));
  for (const name of targets) {
    const target = { ...project.targets[name], inputs: [...(project.targets[name].inputs ?? ["default"]), { env: "SEMIO_BUILD_MODE" }] };
    const findings = cacheCorrectnessFindingsV1({ id: `${project.name}:${name}`, root, authoredCache: project.targets[name].cache ?? null, target, namedInputs: { sharedGlobals: [{ env: "SEMIO_BUILD_MODE" }], ...(project.namedInputs ?? {}), default: ["{projectRoot}/**/*"] } });
    console.log(`${project.name}:${name}`, JSON.stringify(findings));
    bad += findings.length;
  }
}
process.exit(bad ? 1 : 0);
