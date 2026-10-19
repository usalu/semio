/** 🎞️ The projektetage deck describe block: its wrapper and parts. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";
import { registerStructureAndBookendTests } from "../🧭️structure-and-bookends/🟦️.ts";
import { registerCatalogueMorphTests } from "../🖼️catalogue-morphs/🟦️.ts";
import { registerToolSlideTests } from "../🧱️tool-slides/🟦️.ts";
import { registerInputAndCheckSlideTests } from "../⌨️input-and-check-slides/🟦️.ts";
import { registerImportAndTypologySlideTests } from "../📥️import-and-typology-slides/🟦️.ts";
import { registerResearchAndComponentSlideTests } from "../🔎️research-and-component-slides/🟦️.ts";

/** 🪄 Registers the projektetage deck describe block from its parts. */
export function registerDeckContractTests(kit: ProjektetageDeckTestKit): void {
  const { describe } = kit;
  describe("projektetage deck", () => {
    registerStructureAndBookendTests(kit);
    registerCatalogueMorphTests(kit);
    registerToolSlideTests(kit);
    registerInputAndCheckSlideTests(kit);
    registerImportAndTypologySlideTests(kit);
    registerResearchAndComponentSlideTests(kit);
  });
}
