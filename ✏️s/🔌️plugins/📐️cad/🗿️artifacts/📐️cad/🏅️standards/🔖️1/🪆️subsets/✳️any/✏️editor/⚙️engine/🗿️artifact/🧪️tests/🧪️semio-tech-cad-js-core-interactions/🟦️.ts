/** 🧪️ The `@semio-tech/cad-js/core` interaction suite: bootstraps the cad modules once, then registers each interaction group in its original order. */
import type { ArtifactTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import { registerTransformInteractionTests } from "./🕹️transforms/🟦️.ts";
import { registerRegistryTests } from "./🗂️registries/🟦️.ts";
import { registerBoxLengthEntryTests } from "./📦️box-length/🟦️.ts";
import { registerMeasureTests } from "./📏️measure/🟦️.ts";
import { registerHistoryTests } from "./🕰️history/🟦️.ts";
import { registerInteractionE2eTests } from "./🎞️e2e/🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies, source: TestSource): Promise<void> {
  const { __artifactTestRuntime } = dependencies;

  __artifactTestRuntime!.bootstrapCadModules();
  await registerTransformInteractionTests(vitest, dependencies);
  await registerRegistryTests(vitest, dependencies);
  await registerBoxLengthEntryTests(vitest, dependencies);
  await registerMeasureTests(vitest, dependencies);
  await registerHistoryTests(vitest, dependencies);
  await registerInteractionE2eTests(vitest, dependencies);
}
