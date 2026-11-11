import { test } from "vitest";
import { atTestLevel } from "@semio-tech/process/test-budget";

atTestLevel(test, "quick")("portable combination stresses match an independent principal-stress oracle", async () => {
  const { verifyCombinationStressOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⚖️combination-stress/🟦️.ts");
  verifyCombinationStressOracle();
});
