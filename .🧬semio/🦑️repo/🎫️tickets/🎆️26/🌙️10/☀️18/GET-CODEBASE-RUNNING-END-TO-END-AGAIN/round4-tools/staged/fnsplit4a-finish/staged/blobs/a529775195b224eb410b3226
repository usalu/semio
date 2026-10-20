/** 🔶️ The playground resolution phase of registerPlaygroundActionAndPluginSourceTests. */
import type { ActionArgDef, ArgSchema, ContextMenuItemSpec, PluginCatalog } from "@semio-tech/framework";

/** 🔷 Registers the playground resolution tests of registerPlaygroundActionAndPluginSourceTests. */
export function registerPlaygroundResolutionTests({ dependencies, vitest }: { readonly dependencies: typeof import("@semio-tech/framework"); readonly vitest: NonNullable<ImportMeta["vitest"]> }) {
  const { createDevPluginSource, createExtensionSource, createLeasePool, effectiveActionArgs, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, missingRequiredArgs, multiplexPluginSources, organizeContextMenu, resolvePlaygroundBoot, resolvePluginHostConfig, resolvePluginRegistryId } = dependencies;
  const { describe, expect, it, vi } = vitest;

  const SYNTHETIC_PLUGIN_CATALOG: PluginCatalog = {
    plugins: [
      { pluginId: "alpha", wasmOut: "alpha.wasm", role: "plugin", contributes: [], consumes: [] },
      { pluginId: "beta", wasmOut: "beta.wasm", role: "plugin", contributes: [], consumes: [] },
    ],
    extensions: [{ pluginId: "beta-extension-gamma", wasmOut: "beta_gamma.wasm", role: "extension", contributes: ["beta.module"], consumes: [] }],
    hosts: [{ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" }],
    playgrounds: [
      { variant: "alpha", pluginId: "alpha", aliases: [] },
      { variant: "beta-play", pluginId: "beta", app: "beta-play-app", aliases: ["b", "beta play"] },
    ],
    moduleUrl: (pluginId) => `/🔌️plugin-modules/${pluginId}/🌉️bridge.js`,
    extensionModuleUrl: (pluginId) => `/🧩️extension-modules/${pluginId}/🌉️bridge.js`,
  };

  describe("PlaygroundResolution", () => {
    it("resolves host config from the injected catalog", () => {
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "alpha")).toEqual({ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "beta-play")).toBeUndefined();
    });

    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "b")).toBe("beta");
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "beta play")).toBe("beta");
    });

    it("rebuilds program rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot(SYNTHETIC_PLUGIN_CATALOG, "beta-play", {
        variant: "alpha",
        defaultAppId: "alpha-app",
        plugins: [{ pluginId: "alpha", moduleUrl: "/🔌️plugin-modules/alpha/alpha_plugin.js" }],
      });
      expect(boot.variant).toBe("beta-play");
      expect(boot.defaultAppId).toBe("beta-play-app");
      expect(boot.plugins).toEqual([{ pluginId: "beta", moduleUrl: "/🔌️plugin-modules/beta/🌉️bridge.js", contributes: [], consumes: [] }]);
    });
  });

  describe("effectiveActionArgs", () => {
    const TEXT_SCHEMA: ArgSchema = { kind: "string", options: [] };
    const textArg = (id: string, extra: Partial<ActionArgDef> = {}): ActionArgDef => ({
      id,
      label: id,
      schema: TEXT_SCHEMA,
      required: false,
      ...extra,
    });

    it("keeps a seeded arg that is not a declared form field, alongside the form's own staged fields (26/08/16 HUB-SPACES shareSpace regression: spaceId must reach the dispatched descriptor)", () => {
      const defs = [textArg("email")];
      const effective = effectiveActionArgs(defs, { email: "user2@semio.dev" }, { spaceId: "sp-1" });
      expect(effective).toEqual({ spaceId: "sp-1", email: "user2@semio.dev" });
    });

    it("a seed value for a declared field pre-fills it until the form stages its own value (renameSpace's current-name prefill)", () => {
      const defs = [textArg("name")];
      expect(effectiveActionArgs(defs, {}, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "Old Name" });
      expect(effectiveActionArgs(defs, { name: "New Name" }, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "New Name" });
    });

    it("a zero-declared-field confirm dialog passes seed+staged through wholesale (deleteSpace's confirm/cancel shape)", () => {
      expect(effectiveActionArgs([], {}, { spaceId: "sp-1", confirmed: true })).toEqual({ spaceId: "sp-1", confirmed: true });
    });

    it("missingRequiredArgs is unaffected by extra seed keys", () => {
      const defs = [textArg("email", { required: true })];
      const effective = effectiveActionArgs(defs, {}, { spaceId: "sp-1" });
      expect(missingRequiredArgs(defs, effective)).toEqual(["email"]);
    });
  });

  describe("organizeContextMenu", () => {
    const menuLeaf = (id: string): ContextMenuItemSpec => ({ id, label: id, action: id });
    const menuDestructive = (id: string): ContextMenuItemSpec => ({ ...menuLeaf(id), destructive: true });

    it("keeps a flat within-budget menu as-is, with groups sorted after leaves", () => {
      const items = [menuLeaf("a"), menuLeaf("b"), { id: "menu.group.view", children: [menuLeaf("c")] }];
      expect(organizeContextMenu(items, () => undefined)).toEqual(items);
    });

    it("shares the Rust fixture's grouped structure for a flat 12-item over-budget menu", () => {
      const items: ContextMenuItemSpec[] = [
        menuLeaf("primary0"),
        menuLeaf("primary1"),
        menuLeaf("primary2"),
        menuLeaf("primary3"),
        menuLeaf("primary4"),
        menuLeaf("overflow0"),
        menuLeaf("overflow1"),
        menuLeaf("overflow2"),
        menuLeaf("overflow3"),
        menuLeaf("overflow4"),
        menuLeaf("overflow5"),
        menuDestructive("delete"),
      ];
      const categoryOf = (id: string): string | undefined => (id.startsWith("overflow") ? "view" : undefined);
      const organized = organizeContextMenu(items, categoryOf);

      expect(organized.map((item) => item.id)).toEqual([
        "primary0",
        "primary1",
        "primary2",
        "primary3",
        "primary4",
        "menu.group.view",
        "separator-organized-6",
        "delete",
      ]);
      expect(organized[5]!.children?.map((child) => child.id)).toEqual([
        "overflow0",
        "overflow1",
        "overflow2",
        "overflow3",
        "overflow4",
        "overflow5",
      ]);
      expect(organized[6]!.separator).toBe(true);
      expect(organized[6]!.label).toBeUndefined();
      expect(organized[7]!.destructive).toBe(true);
    });
  });
  return { describe, it, createDevPluginSource, createExtensionSource, SYNTHETIC_PLUGIN_CATALOG, expect, vi, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, multiplexPluginSources, createLeasePool };
}
