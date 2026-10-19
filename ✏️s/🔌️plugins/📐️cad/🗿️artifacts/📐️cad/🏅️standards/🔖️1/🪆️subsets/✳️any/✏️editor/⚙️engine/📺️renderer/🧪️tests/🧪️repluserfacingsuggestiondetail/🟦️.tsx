import type { RendererTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import { createReplTestKit } from "./🧰️repl-test-kit/🟦️.tsx";
import type { TestSource } from "./🧰️repl-test-kit/🟦️.tsx";
import { registerReplTextAndFilterTests } from "./🔤️repl-text-and-filters/🟦️.tsx";
import { registerReplEngagementAndSearchTests } from "./🔎️repl-engagement-and-search/🟦️.tsx";
import { registerRendererInteractionAdapterSuite } from "./🖼️adapter-suite/🟦️.tsx";
import { registerModelStatsPaneTests } from "./📊️model-stats-pane/🟦️.tsx";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: RendererTestDependencies, source: TestSource): Promise<void> {
  const kit = createReplTestKit(vitest, dependencies, source);
  registerReplTextAndFilterTests(kit);
  registerReplEngagementAndSearchTests(kit);
  registerRendererInteractionAdapterSuite(kit);
  registerModelStatsPaneTests(kit);
}
