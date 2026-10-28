/** 🧪️ Portable graph mutation outputs agree with independent test-only oracles. */
import { test, expect } from "bun:test";
import corpus from "../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json";
import { verifySemanticGraph } from "./oracle.mjs";

test("semantic mutations preserve graph integrity and Unicode formatting", () => {
  expect(verifySemanticGraph(corpus)).toEqual({ cases: corpus.cases.length, accepted: corpus.cases.filter(row => row.accepted).length, refused: corpus.cases.filter(row => !row.accepted).length, wordCases: corpus.wordCases.length });
});
