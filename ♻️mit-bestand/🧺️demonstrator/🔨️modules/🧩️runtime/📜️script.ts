#!/usr/bin/env bun
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { DEMONSTRATOR_RUNTIME_TARGETS, demonstratorRuntimeModuleLayout } from "./🟦️.ts";

/** 🎪️ Verifies the runtime union completed by the outer Nx preparation graph. */
class PreparationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const profile = args[0];
    if (args.length !== 1 || !["dev", "release"].includes(profile)) throw new Error("prepare <dev|release>");
    const plugin = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
    const moduleRoot = join(plugin, "📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
    const { pluginModuleDirNames, extensionModuleDirNames } = demonstratorRuntimeModuleLayout(DEMONSTRATOR_RUNTIME_TARGETS.map(row => row.pluginId));
    for (const name of [...pluginModuleDirNames.slice(2), ...extensionModuleDirNames]) {
      const directory = join(moduleRoot, name), marker = JSON.parse(readFileSync(join(directory, ".nx-artifact.json"), "utf8"));
      if (!Array.isArray(marker.files) || !marker.files.includes("🌉️bridge.js") || !marker.files.includes("🔣️.json") || marker.files.some((file: string) => !existsSync(join(directory, file)))) throw new Error(`Incomplete Demonstrator runtime component: ${name}`);
    }
    for (const target of DEMONSTRATOR_RUNTIME_TARGETS) {
      const file = join(plugin, "📇️registry/dist/sessions", target.variant, "🟦️session.ts");
      const session = (await import(pathToFileURL(file).href)).PLAYGROUND_SESSION;
      if (session.variant !== target.variant || session.registryPluginId !== target.pluginId) throw new Error(`Mismatched Demonstrator session: ${target.variant}`);
    }
    console.log(`Prepared Demonstrator ${profile}: ${DEMONSTRATOR_RUNTIME_TARGETS.length} runtime variants`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PreparationScript);
if (import.meta.main) await router.run(process.argv.slice(2));
