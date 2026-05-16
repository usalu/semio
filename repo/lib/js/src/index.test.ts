import { describe, expect, test } from "bun:test";
import {
  NEO4J_GRAPH_DATABASE_NAMES,
  joinNeo4jGraphDatabaseName,
  partitionNeo4jGraphCliArgv,
} from "../../../../generate.neo4j.gen.ts";
import { defineLint } from "./script.ts";
import type { FileLinter } from "./linter.ts";

describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["semio", "metabolism"])).toBe("semio-metabolism");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["semio", "metabolism", "--verbose"])).toEqual({
      nameParts: ["semio", "metabolism"],
      passthrough: ["--verbose"],
    });
  });

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
