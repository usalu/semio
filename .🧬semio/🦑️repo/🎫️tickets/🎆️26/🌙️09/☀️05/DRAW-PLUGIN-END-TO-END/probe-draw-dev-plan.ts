import { readFileSync } from "node:fs";
import { join } from "node:path";

const [fixturePath] = Bun.argv.slice(2);
const root = process.cwd();
const read = (file: string) => readFileSync(join(root, file), "utf8");
const fixture = JSON.parse(read(fixturePath));
const lib = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { cacheInternals } = await import(join(root, lib, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
const { catalogBuildContributionV1 } = await import(join(root, "✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"));
const { createScriptProcessEnvelope, withScriptProcessEnvelope } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const contribution = catalogBuildContributionV1({ readText: read });
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const policy = { version: 1 as const, owner: "draw:verify-playground-plan", maximumElapsedMilliseconds: 180_000 };
await withScriptProcessEnvelope(createScriptProcessEnvelope(policy, { workspaceRoot: root }, Date.now()), async invocation => {
  const session = await cacheInternals.playgroundSessionTargets([fixture.manifest], root, [contribution]);
  if (!session["session-draw"]) throw Error("Draw session owner was not inferred");
  const targets = await cacheInternals.playgroundPreparationTargets([fixture.manifest], root, osDev, JSON.parse(read(lib + "/🔣️taxonomy.json")).generatorContracts, [contribution], { workspaceRoot: root, invocation });
  for (const operation of ["dev", "activate", "prepare", "serve"]) {
    const target = targets[operation + "-draw-react-prototype"];
    if (!target || target.metadata?.semio?.runtimeInputAdmission?.status === "refused") throw Error("Draw " + operation + " was refused " + JSON.stringify(target));
    console.log("[DEBUG]", operation, JSON.stringify({ command: target.options.command, dependsOn: target.dependsOn }));
  }
  if (!targets["prepare-draw-react-prototype"].dependsOn.includes("@semio-tech/plugin-registry:session-draw") ||
      !targets["prepare-draw-react-prototype"].dependsOn.includes("semio-hub-draw-component:component")) throw Error("Draw lost a session or component prerequisite");
});
