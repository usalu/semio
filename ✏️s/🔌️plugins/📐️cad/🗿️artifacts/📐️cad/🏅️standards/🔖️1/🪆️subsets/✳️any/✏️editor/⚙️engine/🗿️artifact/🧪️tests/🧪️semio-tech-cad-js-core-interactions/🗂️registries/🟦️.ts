/** 🗂️ Action and interaction registries: declarative specs, typology construction, nested interaction calls, selection actions. */
import type { ArtifactTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import { createRegistriesTestKit } from "./🧰️kit/🟦️.ts";
import { registerActionAndInteractionRegistrySuite } from "./🗂️suite/🟦️.ts";

export async function registerRegistryTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const kit = createRegistriesTestKit(vitest, dependencies);
  registerActionAndInteractionRegistrySuite(kit);
}
