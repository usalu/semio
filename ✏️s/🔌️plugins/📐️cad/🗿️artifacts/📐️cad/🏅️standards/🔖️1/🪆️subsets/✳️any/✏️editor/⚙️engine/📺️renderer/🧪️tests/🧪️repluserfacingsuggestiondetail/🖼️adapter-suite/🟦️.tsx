/** 🖼️ The cad-js renderer interaction adapter describe block: its wrapper and parts. */
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";
import { registerRayProjectionAndPickingTests } from "../🎯️ray-projection-and-picking/🟦️.tsx";
import { registerNurbsPolesAndSceneVisibilityTests } from "../〰️nurbs-and-visibility/🟦️.tsx";
import { registerChromeTogglesAndPickFilterTests } from "../🎚️toggles-and-filters/🟦️.tsx";
import { registerHoverMaterialsAndIsolationTests } from "../🖱️hover-materials-and-isolation/🟦️.tsx";
import { registerPickRenderingAndTypologyStyleTests } from "../🎨️pick-render-and-styles/🟦️.tsx";

/** 🪟 Registers the cad-js renderer interaction adapter describe block from its parts. */
export function registerRendererInteractionAdapterSuite(kit: ReplTestKit): void {
  const { describe } = kit;
  describe("@semio-tech/cad-js/renderer interaction adapter", () => {
    registerRayProjectionAndPickingTests(kit);
    registerNurbsPolesAndSceneVisibilityTests(kit);
    registerChromeTogglesAndPickFilterTests(kit);
    registerHoverMaterialsAndIsolationTests(kit);
    registerPickRenderingAndTypologyStyleTests(kit);
  });
}
