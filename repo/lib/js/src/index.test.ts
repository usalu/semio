import { describe, expect, test } from "bun:test";
import { existsSync } from "node:fs";
import { join } from "node:path";
import {
  NEO4J_GRAPH_DATABASE_NAMES,
  getAllNeo4jGraphExportSpecs,
  joinNeo4jGraphDatabaseName,
  parseExtraNeo4jGraphDatabaseNamesFromEnv,
  partitionNeo4jGraphCliArgv,
} from "../../../../generate.neo4j.gen.ts";
import { BundleScript, ScriptRouter, dispatchSubcommand, findRepoRoot } from "./bundle-script.ts";
import { defineLint } from "./script.ts";
import type { FileLinter } from "./linter.ts";
import {
  dependencyBoundaryBreachesForFile,
  isAdapterBoundaryFile,
  parseTsImportSpecs,
} from "./dependency-boundary.ts";

describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["semio", "kit"])).toBe("semio-kit");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["metabolism", "--verbose"])).toEqual({
      nameParts: ["metabolism"],
      passthrough: ["--verbose"],
    });
  });

  test("product graphs are fixed four joined names", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toEqual(["semio", "elements", "coda", "reuse"]);
  });

  test("NEO4J_EXTRA_GRAPH_DATABASES extends export specs", () => {
    const env = { NEO4J_EXTRA_GRAPH_DATABASES: " foo , bar-baz " };
    expect(parseExtraNeo4jGraphDatabaseNamesFromEnv(env)).toEqual(["foo", "bar-baz"]);
    const names = getAllNeo4jGraphExportSpecs(env).map((s) => joinNeo4jGraphDatabaseName(s));
    expect(names).toContain("foo");
    expect(names).toContain("bar-baz");
  });
});

describe("bundle-script", () => {
  test("ScriptRouter usage lists registered commands", () => {
    class A extends BundleScript {
      run(): void {}
    }
    const router = new ScriptRouter(import.meta.dir).register("a", A).register("b", A);
    expect(router.hasCommands()).toBe(true);
    expect(router.usage()).toContain("a");
    expect(router.usage()).toContain("b");
  });

  test("findRepoRoot reaches monorepo from repo/lib/js/src", () => {
    const root = findRepoRoot(import.meta.dir);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      { go: (rest) => {
        ran = rest.join(",");
      } },
      "unused",
    );
    expect(ran).toBe("x");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});

describe("dependency-boundary", () => {
  test("detects adapter region marker", () => {
    expect(isAdapterBoundaryFile("pkg/foo.ts", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/main.py", "# #region 🔌Adapters\nimport fastapi")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import { z } from "zod";\nexport const a = 1;\n`;
    const file = "semio/client/lib/js/boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(
      new URL("../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"),
      file,
      content,
      file,
    );
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(
      new URL("../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"),
      file,
      content,
      file,
    );
    expect(breachs).toEqual([]);
  });
});
