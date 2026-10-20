import type { ArtifactTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import { createBoxLengthTestKit } from "./🧰️box-length-test-kit/🟦️.ts";
import { registerBoxInteractionTests } from "./📦️box-interaction/🟦️.ts";
import { registerLengthEntryAndStateEngineTests } from "./📏️length-entry-and-state-engine/🟦️.ts";

export async function registerBoxLengthEntryTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const kit = createBoxLengthTestKit(vitest, dependencies);
  registerBoxInteractionTests(kit);
  registerLengthEntryAndStateEngineTests(kit);
}
