import { vi } from "vitest";
vi.mock("../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts", async (original) => ({
  ...await original(),
  useSharedBorderFrameRef: (ref: unknown) => ref,
  useSharedWindowBorder: () => null,
}));
