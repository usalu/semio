import { expect, test } from "vitest";
import { colorInputSelfTests } from "@semio-tech/ui-contract/testing/color-input";
import Ajv from "ajv";
import Decimal from "decimal.js";
import { parseResultMode, resultModeUsesShape, resultModeEigenvalue } from "../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
import projectionSchema from "../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-projection/🔣️.json";
import projectionFixture from "../../⚙️engine/🖥️app-surface/🧪️tests/🧫️fixtures/👁️result-projection/🔣️.json";

test("shared FEM result modes project eigenvectors and the declared physical units", () => {
  const validate = new Ajv({ strict: true }).compile(projectionSchema);
  expect(validate(projectionFixture)).toBe(true);
  for (const row of projectionFixture.cases) {
    const mode = parseResultMode(row.mode);
    expect(resultModeUsesShape(mode), row.id).toBe(row.modeShape);
    const oracle = mode === "static" ? null : mode === "buckling" ? new Decimal(row.eigenvalue).toNumber() : new Decimal(row.eigenvalue).sqrt().div(new Decimal(Math.PI).mul(2)).toNumber();
    const actual = resultModeEigenvalue(mode, row.eigenvalue);
    if (row.displayValue === null) expect(actual).toBeNull();
    else {
      expect(actual, row.id).toBeCloseTo(row.displayValue, 12);
      expect(actual, row.id).toBeCloseTo(oracle!, 12);
    }
  }
  console.log("[DEBUG] FEM result projection five vectors agree with independent Decimal square-root/unit conversion");
});

test("FEM color projection shares the framework color-string corpus", () => {
  const assertions = colorInputSelfTests();
  expect(assertions).toBeGreaterThan(0);
  console.log("[DEBUG] FEM shared color-input/color-string assertions=" + assertions);
});

test("shared retained index sorting agrees with the portable independent order oracle", async () => {
  const { verifyRetainedIndexSortOracle } = await import("@semio-tech/framework/value/sort/testing");
  verifyRetainedIndexSortOracle();
});

test("shared sparse triplets preserve original accumulation order", async () => {
  const { verifyTripletCsrOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧮️triplet-csr/🟦️.ts");
  verifyTripletCsrOracle();
});

test("shared factor oracle preserves every original sparse equation", async () => {
  const { verifyPagedLdltOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-ldlt/🟦️.ts");
  verifyPagedLdltOracle();
});

test("shared signed pencil corpus matches independent compressed sparse matrices", async () => {
  const { verifySymmetricPencilOracle } = await import("../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎼️pencil/🟦️.ts");
  verifySymmetricPencilOracle();
});
