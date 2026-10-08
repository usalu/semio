import { test } from "bun:test";
import { join } from "node:path";

test("actual retained Hub WGPU consumer executes current source ownership laws", async () => {
  const root = process.env.NX_WORKSPACE_ROOT;
  if (!root) throw Error("Explicit repository root required");
  const { proveHubRendererCommandOwnershipV1 } = await import(join(root, "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🧪️tests/🟦️.ts"));
  await proveHubRendererCommandOwnershipV1(root);
});
