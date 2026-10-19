/** 🗂️ The cad-js core action and interaction registries describe block: its wrapper and parts. */
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";
import { registerActionDocumentAndTypologyKitTests } from "../📜️docs/🟦️.ts";
import { registerCurveInterpolationStepTests } from "../➰️curves/🟦️.ts";
import { registerModelDiffAndTypologyModeTests } from "../🔧️diffs/🟦️.ts";
import { registerNestedInteractionCallTests } from "../📞️nested/🟦️.ts";
import { registerRegistryLoadingAndBoxFootprintTests } from "../📚️loading/🟦️.ts";
import { registerSelectionCommandTests } from "../🎯️select/🟦️.ts";

/** 🏛 Registers the cad-js core action and interaction registries describe block from its parts. */
export function registerActionAndInteractionRegistrySuite(kit: RegistriesTestKit): void {
  const { describe } = kit;
  describe("@semio-tech/cad-js/core action and interaction registries", () => {
    registerActionDocumentAndTypologyKitTests(kit);
    registerCurveInterpolationStepTests(kit);
    registerModelDiffAndTypologyModeTests(kit);
    registerNestedInteractionCallTests(kit);
    registerRegistryLoadingAndBoxFootprintTests(kit);
    registerSelectionCommandTests(kit);
  });
}
