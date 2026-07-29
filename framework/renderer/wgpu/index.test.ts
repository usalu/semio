import { describe, expect, it } from "vitest";
import { pluginHandleForBridge } from "@semio-tech/framework-core";

describe("framework renderer wgpu", () => {
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
      refreshUi: async () => ({}),
      dispose: () => {},
    });
    expect(JSON.parse(handle.manifest()).pluginId).toBe("draw");
  });
});
