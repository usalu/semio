/** 🧪️ AJV and RFC 6902 independently admit the shared native graph witnesses. */
import { test, expect } from "bun:test";
import corpus from "../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json";
import { verifyGraphValidation } from "./oracle.mjs";

test("Layout graph validation agrees with independent schema and topological oracles", () => {
  expect(verifyGraphValidation(corpus)).toEqual({ documents: corpus.documents.length, parents: corpus.parents.length });
});
