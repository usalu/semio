import { createProjektetageDeckTestKit } from "./🧰️deck-test-kit/🟦️.ts";
import type { ProjektetageDeckDependencies, TestSource } from "./🧰️deck-test-kit/🟦️.ts";
import { registerDeckContractTests } from "./🎞️deck-contract-describe/🟦️.ts";

/** 🎞️ The projektetage deck contract: chapter order, German bookmarks, Zukunft Bau bookends and catalogue morphs. */
export async function registerProjektetageDeckTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ProjektetageDeckDependencies, source: TestSource): Promise<void> {
  const kit = createProjektetageDeckTestKit(vitest, dependencies, source);
  registerDeckContractTests(kit);
}
