#!/usr/bin/env bun
/** 📐️ `@semio-tech/cad-js` task router: `bun ./📜️script.ts test|generate|fixture [args…]`. Folds the former cad-js-{core,renderer,kernel-brepjs,query,machine-stately,runtime} package scripts into one. */
import { join, resolve } from "node:path";
import type { BundleLinter } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { defineLint } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

/** 🔌️Dependency-boundary lint across all 6 folded domain files (former per-package `policyFile` checks merged: renderer + stately each carried their own single-file variant). */
export const policy = defineLint("@semio-tech/cad-js-modules", (_l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, "✏️s/🔌️plugins/📐️cad/🔨️modules");
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

class FixtureScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    process.env.CAD_GENERATE_STEP_FIXTURES = "1";
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

class GenerateScript extends BundleScript {
  async run(extra: string[]): Promise<void> {
    const { bootstrapCadModules } = await import("../../🔨️modules/🏃️runtime/🟦️component.ts");
    const { defaultModelDefinitionId } = await import("../../🔨️modules/🫀️core/🟦️component.ts");
    const { buildSpatialStatelyMachineCatalogView } = await import("../../🔨️modules/🎰️stately/🟦️component.ts");
    bootstrapCadModules();
    let outPath = join(this.root, "../../🔣️machine.json");
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

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("fixture", FixtureScript).register("generate", GenerateScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
