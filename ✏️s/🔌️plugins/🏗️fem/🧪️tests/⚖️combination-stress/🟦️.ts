import { test } from "vitest";
import { atTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

atTestLevel(test, "quick")("portable combination stresses match an independent principal-stress oracle", async () => {
  const { verifyCombinationStressOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⚖️combination-stress/🟦️.ts");
  verifyCombinationStressOracle();
});
