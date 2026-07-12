import { describe, expect, it } from "vitest";
import { pluginHandleForBridge } from "@semio-tech/framework-core";

describe("framework renderer wgpu", () => {
  it("exports boot entry", async () => {
    const mod = await import("./index.ts");
    expect(typeof mod.bootFrameworkOsWgpu).toBe("function");
  });

  it("builds plugin bridge handles", () => {
    const handle = pluginHandleForBridge({
      pluginId: "draw",
      manifest: {
        pluginId: "draw",
        label: "Draw",
        version: "0.1.0",
        apps: [],
        programs: [],
        examples: [],
      },
      createApp: async () => 1,
      destroyApp: async () => {},
      handleAction: async () => [],
      render: async () => ({ type: "text", value: "hello" }),
      tools: async () => [],
      windowEngagements: async () => ({}),
      windowMeasures: async () => ({}),
      appLabels: async () => ({ windowKindLabels: {}, panelTabLabels: {}, modeLabels: {} }),
      dispose: () => {},
    });
    expect(JSON.parse(handle.manifest()).pluginId).toBe("draw");
  });
});
