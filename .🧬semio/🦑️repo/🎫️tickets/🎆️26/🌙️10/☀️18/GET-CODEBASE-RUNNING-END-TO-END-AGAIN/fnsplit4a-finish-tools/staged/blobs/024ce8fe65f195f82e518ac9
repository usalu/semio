/** 🔸️ The plugin source phase of registerPlaygroundActionAndPluginSourceTests. */
import { loadKernelSourceWatchFixture, loadKernelSchema } from "@semio-tech/framework-kernel/testing/fixtures";
import type { PluginCatalog, PluginRegistryEntry, PluginSourceEvent } from "@semio-tech/framework";
import type { registerPlaygroundResolutionTests } from "../🔶️playground-resolution/🟦️.ts";

/** 🔹 Registers the plugin source tests of registerPlaygroundActionAndPluginSourceTests. */
export function registerPluginSourceTests({ describe, it, createDevPluginSource, createExtensionSource, SYNTHETIC_PLUGIN_CATALOG, expect, vi, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, multiplexPluginSources }: Pick<ReturnType<typeof registerPlaygroundResolutionTests>, "describe" | "it" | "createDevPluginSource" | "createExtensionSource" | "SYNTHETIC_PLUGIN_CATALOG" | "expect" | "vi" | "PluginModuleUnavailableError" | "extensionSourceEventToPluginSourceEvent" | "multiplexPluginSources">) {
  describe("PluginSource", () => {
    const registry: readonly PluginRegistryEntry[] = [
      { pluginId: "note", moduleUrl: "/🔌️plugin-modules/note/note_plugin.js" },
      { pluginId: "s", moduleUrl: "/🔌️plugin-modules/s/s_plugin.js" },
    ];
    const silentWatch = () => () => {};

    it("opens and closes exactly its owner's watch streams without a product route dependency", async () => {
      const fixture = await loadKernelSourceWatchFixture();
      const schema = await loadKernelSchema();
      const { default: Ajv } = await import("ajv");
      
      const opened: string[] = [], closed: string[] = [];
      const watchFor = (label: string) => () => {
        opened.push(label);
        return () => void closed.push(label);
      };
      for (const stream of fixture.streams) {
        const source = stream.source === "dev" ? createDevPluginSource(registry, watchFor(stream.watch)) : createExtensionSource(SYNTHETIC_PLUGIN_CATALOG, watchFor(stream.watch));
        const unsubscribe = source.subscribe(() => {});
        expect(opened.at(-1)).toBe(stream.watch);
        unsubscribe();
        expect(closed.at(-1)).toBe(stream.watch);
      }
      expect(opened).toEqual(fixture.streams.map((stream) => stream.watch));
      expect(closed).toEqual(opened);
    });

    it("list() returns the registry it was created with", async () => {
      const source = createDevPluginSource(registry, silentWatch);
      expect(source.id).toBe("dev");
      await expect(source.list()).resolves.toEqual(registry);
    });

    /** 🔎️ Answers the HEAD probe of a locally served module's descriptor: `served` paths answer 200. */
    const servedModules = (served: readonly string[]) => {
      const probes: string[] = [];
      vi.stubGlobal("fetch", async (url: string, init?: RequestInit) => {
        probes.push(`${init?.method ?? "GET"} ${decodeURIComponent(url)}`);
        return new Response(null, { status: served.includes(decodeURIComponent(url)) ? 200 : 404 });
      });
      return probes;
    };
    const acquisition = () => ({ signal: new AbortController().signal });

    it("acquireModule() cache-busts a cold page load even before the build snapshot arrives", async () => {
      const probes = servedModules(["/🔌️plugin-modules/note/🔣️.json", "/🔌️plugin-modules/s/🔣️.json"]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        const firstAcquired = await source.acquireModule("note", undefined, acquisition());
        const first = new URL(firstAcquired.moduleUrl, "http://semio.test");
        const second = new URL((await source.acquireModule("s", undefined, acquisition())).moduleUrl, "http://semio.test");
        expect(decodeURIComponent(first.pathname)).toBe("/🔌️plugin-modules/note/note_plugin.js");
        expect(first.searchParams.get("v")).toMatch(/^\d+$/);
        expect(second.searchParams.get("v")).toBe(first.searchParams.get("v"));
        expect(firstAcquired.rebuiltAt).toBe(Number(first.searchParams.get("v")));
        expect(probes).toEqual(["HEAD /🔌️plugin-modules/note/🔣️.json", "HEAD /🔌️plugin-modules/s/🔣️.json"]);
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("acquireModule() cache-busts with a rebuiltAt query param", async () => {
      servedModules(["/🔌️plugin-modules/note/🔣️.json"]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        await expect(source.acquireModule("note", 1785789943669, acquisition())).resolves.toEqual({ moduleUrl: "/🔌️plugin-modules/note/note_plugin.js?v=1785789943669", rebuiltAt: 1785789943669 });
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("acquireModule() answers unavailable for an unknown pluginId and for a module this device does not serve", async () => {
      servedModules([]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        await expect(source.acquireModule("missing", undefined, acquisition())).rejects.toBeInstanceOf(PluginModuleUnavailableError);
        await expect(source.acquireModule("note", undefined, acquisition())).rejects.toThrow(/HTTP 404/);
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("subscribe() through a watch that never delivers yields nothing and unsubscribes cleanly", () => {
      const source = createDevPluginSource(registry, silentWatch);
      const events: PluginSourceEvent[] = [];
      const unsubscribe = source.subscribe((event) => events.push(event));
      expect(() => unsubscribe()).not.toThrow();
      expect(events).toEqual([]);
    });

    it("normalizes extension snapshots and installs into plugin availability events", () => {
      expect(extensionSourceEventToPluginSourceEvent({ kind: "snapshot", extensions: [{ extensionId: "gamma-extension", installedAt: 1785789943669 }] })).toEqual({
        kind: "snapshot",
        plugins: [{ pluginId: "gamma-extension", rebuiltAt: 1785789943669 }],
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "installed", extensionId: "gamma-extension", installedAt: 1785789943670 })).toEqual({
        kind: "built",
        pluginId: "gamma-extension",
        rebuiltAt: 1785789943670,
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "uninstalled", extensionId: "gamma-extension" })).toBeUndefined();
    });

    it("multiplexPluginSources() merges list() and acquires from the first child that serves the module", async () => {
      const catalog: PluginCatalog = {
        plugins: [],
        extensions: [{ pluginId: "gamma-extension", wasmOut: "gamma.wasm", role: "extension", contributes: [], consumes: [] }],
        hosts: [],
        playgrounds: [],
        moduleUrl: (pluginId) => `/🔌️plugin-modules/${pluginId}/🌉️bridge.js`,
        extensionModuleUrl: (pluginId) => `/🧩️extension-modules/${pluginId}/🌉️bridge.js`,
      };
      const dev = createDevPluginSource(registry, silentWatch);
      const extensions = createExtensionSource(catalog, silentWatch);
      const multiplexed = multiplexPluginSources(dev, extensions);
      expect(multiplexed.id).toBe("dev+extensions");
      const listed = await multiplexed.list();
      expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...catalog.extensions.map((entry) => entry.pluginId)].sort());
      servedModules(["/🔌️plugin-modules/note/🔣️.json"]);
      try {
        expect(decodeURIComponent(new URL((await multiplexed.acquireModule("note", undefined, acquisition())).moduleUrl, "http://semio.test").pathname)).toBe("/🔌️plugin-modules/note/note_plugin.js");
        await expect(multiplexed.acquireModule("gamma-extension", 7, acquisition())).resolves.toEqual({ moduleUrl: "/🧩️extension-modules/gamma-extension/🌉️bridge.js?v=7", rebuiltAt: 7 });
        await expect(multiplexed.acquireModule("missing", undefined, acquisition())).rejects.toBeInstanceOf(PluginModuleUnavailableError);
        const remote = { id: "remote", list: async () => [], subscribe: () => () => {}, acquireModule: async (pluginId: string) => ({ moduleUrl: `/remote/${pluginId}/🌉️bridge.js`, rebuiltAt: 9 }) };
        await expect(multiplexPluginSources(dev, remote).acquireModule("s", undefined, acquisition())).resolves.toEqual({ moduleUrl: "/remote/s/🌉️bridge.js", rebuiltAt: 9 });
        const failing = { ...remote, acquireModule: async () => Promise.reject(new Error("transport down")) };
        await expect(multiplexPluginSources(failing, dev).acquireModule("note", undefined, acquisition())).rejects.toThrow("transport down");
      } finally {
        vi.unstubAllGlobals();
      }
    });
  });
}
