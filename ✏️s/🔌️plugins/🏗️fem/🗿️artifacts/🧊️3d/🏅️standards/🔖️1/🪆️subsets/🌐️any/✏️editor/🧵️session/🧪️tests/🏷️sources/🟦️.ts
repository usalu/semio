import assert from "node:assert/strict";
import { lusolve } from "mathjs";
import Ajv from "ajv";
import corpus from "../../🧫️fixtures/🏷️sources/🔣️.json" with { type: "json" };

/** 🧮️ Independent LU solutions reproduce PyNite translations for every selected source and order. */
export function testFem3dMountedSourceOracle(): void {
  const { elasticModulus: e, inertia: i, length: l, tipLoad: p } = corpus.benchmark;
  const stiffness = [[12 * e * i / l ** 3, -6 * e * i / l ** 2], [-6 * e * i / l ** 2, 4 * e * i / l]];
  for (const order of corpus.orders) {
    for (const source of corpus.cases) {
      const loads = order.map(id => corpus.cases.find(row => row.kind === "loadCase" && row.id === id)!);
      const factor = source.kind === "loadCase" ? loads.find(row => row.id === source.id)!.factor : loads.reduce((sum, row) => sum + row.factor * (corpus.combinationTerms[row.id as keyof typeof corpus.combinationTerms] ?? 0), 0);
      assert.equal(factor, source.factor);
      const displacement = lusolve(stiffness, [p * factor, 0]) as number[][];
      assert(Math.abs(displacement[0]![0]! - factor * corpus.expected.tipTy) < 1e-12);
      assert(Math.abs(displacement[1]![0]! - factor * corpus.expected.tipRz) < 1e-12);
      assert.equal(-p * factor, factor * corpus.expected.rootTyReaction);
      assert.equal(-p * l * factor, factor * corpus.expected.rootRzReaction);
    }
  }
  assert.equal(corpus.cases.filter(row => row.id === "tip").length, 2);
  const validate = new Ajv({ strict: true }).compile({
    type: "object", required: ["cases", "combinations", "terms", "factors"], additionalProperties: false,
    properties: {
      cases: { type: "array", items: { type: "string" }, uniqueItems: true, maxItems: corpus.maximumSources },
      combinations: { type: "array", items: { type: "string" }, uniqueItems: true, maxItems: corpus.maximumSources },
      terms: { type: "array", items: { type: "string", enum: ["tip"] }, maxItems: corpus.maximumSources },
      factors: { type: "array", items: { type: "number" }, maxItems: corpus.maximumSources },
    },
  });
  assert(validate({ cases: ["tip"], combinations: ["tip"], terms: ["tip"], factors: [1] }));
  for (const refusal of corpus.refusals) {
    const source = { cases: ["tip"], combinations: ["tip"], terms: ["tip"], factors: [1] };
    switch (refusal) {
      case "duplicate-case": source.cases.push("tip"); break;
      case "duplicate-combination": source.combinations.push("tip"); break;
      case "missing-case": source.terms.push("missing"); break;
      case "nonfinite-factor": source.factors[0] = Infinity; break;
      case "case-maximum-plus-one": source.cases = Array.from({ length: corpus.maximumSources + 1 }, (_, index) => `case${index}`); break;
      case "combination-maximum-plus-one": source.combinations = Array.from({ length: corpus.maximumSources + 1 }, (_, index) => `combination${index}`); break;
      case "term-maximum-plus-one": source.terms = Array(corpus.maximumSources + 1).fill("tip"); break;
      default: throw Error(`Unknown refusal ${refusal}`);
    }
    assert(!validate(source), `independent admission ${refusal}`);
  }
}

