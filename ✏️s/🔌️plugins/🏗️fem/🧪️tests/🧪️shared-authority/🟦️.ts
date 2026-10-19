import { expect, test } from "vitest";
import { sha256 } from "@noble/hashes/sha2.js";
import { operationEntityId } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🟦️.ts";
import { colorInputSelfTests } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧪️color-input/🟦️.ts";
import fixture from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️entity-id/🔣️.json";
import Ajv from "ajv";
import { compare } from "fast-json-patch";
import { renderRevisionMatches } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🟦️.ts";
import renderSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🔣️.json";
import renderFixture from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️render-revision/🔣️.json";
import Decimal from "decimal.js";
import { parseResultMode, resultModeUsesShape, resultModeEigenvalue } from "../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
import projectionSchema from "../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-projection/🔣️.json";
import projectionFixture from "../../⚙️engine/🖥️app-surface/🧫️fixtures/👁️result-projection/🔣️.json";

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

test("mounted render identity refuses reused counters from another canonical document", () => {
  const validate = new Ajv({ strict: true }).compile(renderSchema);
  for (const row of renderFixture.cases) {
    const left = { ...renderFixture.base, ...row.left };
    const right = { ...renderFixture.base, ...row.right };
    expect(validate(left)).toBe(true);
    expect(validate(right)).toBe(true);
    expect(renderRevisionMatches(left, right), row.id).toBe(row.expected);
    expect(renderRevisionMatches(right, left), row.id).toBe(row.expected);
    expect(compare(left, right).length === 0, row.id).toBe(row.expected);
  }
  for (const counter of ["00", "-1", "18446744073709551616", "99999999999999999999"]) {
    expect(validate({ ...renderFixture.base, baseRevision: counter })).toBe(false);
    expect(validate({ ...renderFixture.base, generation: counter })).toBe(false);
  }
  console.log("[DEBUG] mounted render identity eight vectors match independent Ajv and RFC6902, including full canonical hash and lossless u64 counters");
});

test("shared entity authority distinguishes writers and replays the same admission", () => {
  const ids = new Set<string>();
  const encoder = new TextEncoder();
  for (const row of fixture.rows) {
    const authority = { parentDocumentId: row.documentId, authoringSeed: row.authoringSeed };
    const id = operationEntityId(authority, row.prefix, row.localKey);
    const field = (value: string) => encoder.encode(value).length + ":" + value;
    const bytes = encoder.encode(field(row.documentId) + field(row.authoringSeed) + field(row.localKey));
    const oracle = row.prefix + "-" + Buffer.from(sha256(bytes)).toString("hex").slice(0, 16);
    expect(id).toBe(row.expected);
    expect(id).toBe(oracle);
    expect(operationEntityId(authority, row.prefix, row.localKey)).toBe(id);
    expect(ids.has(id)).toBe(false);
    ids.add(id);
  }
  expect(() => operationEntityId({ parentDocumentId: "doc", authoringSeed: "" }, "n", "nodes")).toThrow("admitted authoring seed");
  console.log("[DEBUG] entity authority vectors=" + fixture.rows.length + " independent SHA-256 oracle passed");
});

test("FEM color projection shares the framework color-string corpus", () => {
  const assertions = colorInputSelfTests();
  expect(assertions).toBeGreaterThan(0);
  console.log("[DEBUG] FEM shared color-input/color-string assertions=" + assertions);
});

test("shared retained index sorting agrees with the portable independent order oracle", async () => {
  const { verifyRetainedIndexSortOracle } = await import("../../../../../🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts");
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
