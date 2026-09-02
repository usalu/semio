#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCmd, runCmdStatus, runViteBunxDev, runVitest, spawnDaemon, waitForHttpUrl, withViteConfigLoader } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildEngineWasm, buildPlugins, ensurePluginRegistry } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts";
import { DEMONSTRATOR_PANES, demonstratorPaneRuntimeVariant } from "./🟦️brand.ts";

const demonstratorRoot = import.meta.dir;

//#region 🎪️DemonstratorPluginBuild
/** 🆕 Builds current plugin inputs unless the caller explicitly requests staged artifacts. */
function demonstratorShouldBuildPlugins(skipPluginBuild: string | undefined): boolean {
  return skipPluginBuild !== "1";
}

/** @emoji 🎯️ Builds only the primary crate behind a runtime variant; its contributed extensions are
 * already included by the demonstrator crate's own consumer closure. */
async function buildRuntimePlugin(variant: string): Promise<void> {
  const pluginId = runtimePluginId(variant);
  const previousPluginOnly = process.env.SEMIO_PLUGIN_ONLY;
  process.env.SEMIO_PLUGIN_ONLY = pluginId;
  try {
    await buildPlugins(variant);
  } finally {
    if (previousPluginOnly === undefined) delete process.env.SEMIO_PLUGIN_ONLY;
    else process.env.SEMIO_PLUGIN_ONLY = previousPluginOnly;
  }
}

/** @emoji 🪪️ Resolves a playground variant to the plugin artifact it builds. */
function runtimePluginId(variant: string): string {
  const pluginId = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === variant)?.pluginId;
  if (!pluginId) throw new Error(`unknown demonstrator runtime variant: ${variant}`);
  return pluginId;
}

/** @emoji 🧮️ Returns one representative runtime variant for each additional plugin artifact. */
export function demonstratorRuntimeBuildVariants(primaryVariant: string): readonly string[] {
  const seenPluginIds = new Set([runtimePluginId(primaryVariant)]);
  const variants: string[] = [];
  for (const pane of DEMONSTRATOR_PANES) {
    const variant = demonstratorPaneRuntimeVariant(pane.variant);
    const pluginId = runtimePluginId(variant);
    if (seenPluginIds.has(pluginId)) continue;
    seenPluginIds.add(pluginId);
    variants.push(variant);
  }
  return variants;
}

export type DemonstratorRuntimeModuleLayout = {
  readonly pluginModuleDirNames: readonly string[];
  readonly extensionModuleDirNames: readonly string[];
};

/** @emoji 🛣️ Separates runtime plugin and extension directories by their public catalog roots while preserving the full transitive dependency and contribution closure. */
export function demonstratorRuntimeModuleLayout(rootPluginIds: readonly string[]): DemonstratorRuntimeModuleLayout {
  const catalog = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  const byId = new Map(catalog.map((target) => [target.pluginId, target] as const));
  const selected = new Set(rootPluginIds);
  const queue = [...rootPluginIds];
  for (let index = 0; index < queue.length; index++) {
    const target = byId.get(queue[index]!);
    if (!target) continue;
    for (const dependency of target.dependsOn ?? []) {
      if (selected.has(dependency)) continue;
      selected.add(dependency);
      queue.push(dependency);
    }
    const consumes = new Set(target.consumes ?? []);
    if (consumes.size === 0) continue;
    for (const extension of EXTENSION_TARGETS) {
      if (selected.has(extension.pluginId) || !(extension.contributes ?? []).some((tag) => consumes.has(tag))) continue;
      selected.add(extension.pluginId);
      queue.push(extension.pluginId);
    }
  }
  const pluginIds: string[] = [];
  const extensionIds: string[] = [];
  for (const id of selected) {
    if (byId.get(id)?.role === "extension") extensionIds.push(id);
    else pluginIds.push(id);
  }
  return { pluginModuleDirNames: ["_vendor", "_shard", ...pluginIds], extensionModuleDirNames: extensionIds };
}

/** @emoji 🎪️ Builds every pane's runtime plugin crate + declared engines into the shared
 * `🧧framework/os/dev` `🔌️plugin-modules/` dir this page's own `⚙️vite.config.ts` static-serves from.
 *
 * Five panes share the demonstrator crate. Generator deliberately boots the procedural crate, so it
 * must be built separately rather than consuming whichever procedural artifact happens to be staged.
 * The primary variant is restored after both builds so registry session generation stays deterministic.
 * Engines still need a per-pane pass: only some panes declare one (e.g. only `verfolgen` needs
 * tiled-map), and each variant's own registry row carries its own `engines` list independently even
 * though they now share a `pluginId`. */
async function buildDemonstratorPlugins(): Promise<void> {
  const primaryVariant = DEMONSTRATOR_PANES[0]?.variant;
  const buildCurrentPlugins = demonstratorShouldBuildPlugins(process.env.SKIP_PLUGIN_BUILD);
  if (primaryVariant) {
    if (buildCurrentPlugins) await buildPlugins(primaryVariant);
    else await ensurePluginRegistry(primaryVariant);
  }
  if (buildCurrentPlugins) {
    for (const variant of demonstratorRuntimeBuildVariants(primaryVariant ?? "generator")) await buildRuntimePlugin(variant);
  }
  if (primaryVariant) await ensurePluginRegistry(primaryVariant);
  for (const pane of DEMONSTRATOR_PANES) {
    await buildEngineWasm(pane.variant, "react", join(demonstratorRoot, "package.json"));
  }
}
//#endregion 🎪️DemonstratorPluginBuild

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDemonstratorPlugins();
    runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "MIT_BESTAND_DEMONSTRATOR_PORT",
      defaultPort: "6029",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    process.env.SEMIO_BUILD_MODE = "ship";
    await buildDemonstratorPlugins();
    if (runCmdStatus("bun", withViteConfigLoader(["run", "vite", "build", "--config", "⚙️vite.config.ts", ...segments]), { cwd: this.root, env: process.env }) !== 0) {
      throw new Error("demonstrator landing build failed");
    }
    console.log(`[build] demonstrator built at ${join(demonstratorRoot, "dist")}`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest[0] === "e2e") {
      await this.runAcceptancePlaywright();
      return;
    }
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }

  /** 🎪️ Demonstrator-local analog of root `📜️script.ts`'s `runStorybookPlaywright()` — the demonstrator is
   * a live Vite dev server (not a prebuilt static bundle like Storybook), so this spawns `DevScript`'s own
   * `dev` command as a daemon instead of a static file server. `runViteBunxDev`'s `fixedPort: true` reuse
   * path (see `DevScript.run`) means this harmlessly no-ops the spawn (and the later `kill()`) if a
   * developer already has the demonstrator running on this port — it never tears down someone else's
   * session. */
  private async runAcceptancePlaywright(): Promise<void> {
    const port = process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? "6029";
    const baseUrl = `http://127.0.0.1:${port}/`;
    const server = spawnDaemon("bun", [join(this.root, "📜️script.ts"), "dev"], {
      cwd: this.root,
      env: { ...process.env, MIT_BESTAND_DEMONSTRATOR_PORT: port },
    });
    try {
      await waitForHttpUrl(baseUrl, 180_000);
      runCmd("bunx", ["playwright", "test", "--config", join(this.root, "🧪️playwright.config.ts")], {
        cwd: this.repoRoot,
        env: {
          ...process.env,
          PLAYWRIGHT_BASE_URL: baseUrl,
          PLAYWRIGHT_BROWSERS_PATH: process.env.PLAYWRIGHT_BROWSERS_PATH ?? `${this.repoRoot}/node_modules/.cache/ms-playwright`,
          MIT_BESTAND_DEMONSTRATOR_PORT: port,
        },
      });
    } finally {
      server.kill();
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  //#region 🧪️DemonstratorPluginBuildTests
  describe("demonstratorRuntimeBuildVariants", () => {
    it("requires fresh plugin builds unless explicitly skipped", () => {
      expect(demonstratorShouldBuildPlugins(undefined)).toBe(true);
      expect(demonstratorShouldBuildPlugins("0")).toBe(true);
      expect(demonstratorShouldBuildPlugins("1")).toBe(false);
    });

    it("builds one additional artifact for six pane runtime variants", () => {
      expect(demonstratorRuntimeBuildVariants("generator")).toEqual(["procedural3d"]);
    });

    it("publishes plugins, the shared shard, and consumed extensions at their catalog URL roots", () => {
      const layout = demonstratorRuntimeModuleLayout(["demonstrator", "procedural"]);
      expect(layout.pluginModuleDirNames.slice(0, 2)).toEqual(["_vendor", "_shard"]);
      expect(new Set(layout.pluginModuleDirNames.slice(2))).toEqual(new Set(["demonstrator", "procedural", "cad", "gis", "process", "puzzle", "sourcing", "stdio", "flow"]));
      expect(new Set(layout.extensionModuleDirNames)).toEqual(
        new Set([
          "flow-extension-bim",
          "flow-extension-brep",
          "flow-extension-dictionary",
          "flow-extension-draw",
          "flow-extension-list",
          "flow-extension-logic",
          "flow-extension-math",
          "flow-extension-primitive",
          "flow-extension-text",
          "process-extension-concrete",
          "process-extension-metal",
          "process-extension-robotic",
          "process-extension-wood",
        ]),
      );
      expect(layout.extensionModuleDirNames.every((id) => !layout.pluginModuleDirNames.includes(id))).toBe(true);
    });
  });
  //#endregion 🧪️DemonstratorPluginBuildTests
}
