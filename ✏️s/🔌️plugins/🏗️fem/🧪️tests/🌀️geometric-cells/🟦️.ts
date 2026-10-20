import { test } from "vitest";
import { atTestLevel } from "@semio-tech/framework/process/test-budget";

atTestLevel(test, "quick")("geometric stiffness cells match independent matrix and quadrature oracles", async () => {
  const { verifyGeometricCellOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🌀️geometric-cells/🟦️.ts");
  verifyGeometricCellOracle();
});
