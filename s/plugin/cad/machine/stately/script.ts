#!/usr/bin/env bun
/** 🧭 `@semio-tech/cad-js-machine-stately` task router: `generate` | `test` | `policy`. */
import { join, resolve } from "node:path";
import type { FileLinter } from "../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel } from "../../../repo/lib/js/index.ts";
import { defineLint } from "../../../repo/lib/js/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio-tech/cad-js-machine-stately-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

class GenerateScript extends BundleScript {
  async run(extra: string[]): Promise<void> {
    const { bootstrapCadModules } = await import("@semio-tech/cad-js-runtime");
    const { defaultModelDefinitionId } = await import("@semio-tech/cad-js-core");
    const { buildSpatialStatelyMachineCatalogView } = await import("./index.ts");
    bootstrapCadModules();
    let outPath = join(this.root, "machine.json");
    let modelDefinitionId = defaultModelDefinitionId();
    const interactionIds: string[] = [];
    for (let i = 0; i < extra.length; i++) {
      const a = extra[i]!;
      if (a === "--out" && extra[i + 1]) {
        outPath = resolve(this.root, extra[i + 1]!);
        i++;
        continue;
      }
      if (a === "--model-definition" && extra[i + 1]) {
        modelDefinitionId = extra[i + 1]!;
        i++;
        continue;
      }
      if (!a.startsWith("-")) interactionIds.push(a);
    }
    const doc = buildSpatialStatelyMachineCatalogView({
      modelDefinitionId,
      interactionIds: interactionIds.length > 0 ? interactionIds : undefined,
    });
    await Bun.write(outPath, `${JSON.stringify(doc, null, 2)}\n`);
    console.error(`wrote ${outPath} (${doc.machines.length} machine(s))`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("generate", GenerateScript);

await runBundleScriptMain(router, import.meta.url);
