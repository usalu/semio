import { test } from "vitest";
import { atTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

atTestLevel(test, "quick")("geometric stiffness cells match independent matrix and quadrature oracles", async () => {
  const { verifyGeometricCellOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🌀️geometric-cells/🟦️.ts");
  verifyGeometricCellOracle();
});
