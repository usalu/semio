import { describe, expect, test } from "bun:test";
import { NEO4J_GRAPH_DATABASE_NAMES } from "../../../../generate.neo4j.gen.ts";
import { defineLint } from "./script.ts";
import type { FileLinter } from "./linter.ts";

describe("Neo4j graph database registry", () => {
  test("includes semio-metabolism for MCP and generate export", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toContain("semio-metabolism");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});
