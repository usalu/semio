import { describe, expect, it } from "vitest";
import { pluginHandleForBridge } from "@semio-tech/framework-core";

describe("framework renderer wgpu", () => {
  it("builds program bridge handles", () => {
    const handle = pluginHandleForBridge({
      programId: "draw",
      manifest: {
        programId: "draw",
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
    expect(JSON.parse(handle.manifest()).programId).toBe("draw");
  });
});
