import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fg from "fast-glob";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, delimiter, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { loadCatalogTaxonomy, semanticDirectoryKindId, implementationLeafBasenameFinding } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { buildCasePlan, loadOracleRegistry, oracleHostPackagesFor, parseFeature, testProjectName } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
import { oracleHostPython, pythonHostArguments } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts";
import control from "../../🧫️fixtures/🔮️oracle-source-ownership/🔣️.json";
import schema from "../../🧫️fixtures/🔮️oracle-source-ownership/🧬️schema/🔣️.json";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
const controlPath = join(root, control.plugin, "🧫️fixtures/🔮️oracle-source-ownership/🔣️.json");
const read = (path: string) => readFileSync(join(root, path), "utf8");
const normSources = [control.manifest, control.destination, ...control.adapters.map((row) => row.path)].map((path) => `{workspaceRoot}/${path}`).sort();

describe("Norm oracle source ownership", () => {
  test("portable data and third-party filesystem enumeration identify exactly fifteen adapters", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(control)).toBe(true);
    expect(validate({ ...control, module: "vocabulary" })).toBe(false);
    expect(validate({ ...control, adapters: control.adapters.slice(1) })).toBe(false);
    const actual = fg.sync(`${control.plugin}/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*/🐍️.py`, { cwd: root }).sort();
    expect(actual).toEqual(control.adapters.map((row) => row.path).sort());
    expect(control.adapters.every((row) => existsSync(join(root, row.feature)))).toBe(true);
  });

  test("the shared execution leaf has one semantic owner and no predecessor shim", () => {
    expect(existsSync(join(root, control.source))).toBe(false);
    expect(existsSync(join(root, control.destination))).toBe(true);
    const taxonomy = loadCatalogTaxonomy();
    expect(
      semanticDirectoryKindId("🏃️execution", taxonomy, {
        parentKindId: "oracles",
      }),
    ).toBe("oracle-execution");
    expect(implementationLeafBasenameFinding(control.destination, taxonomy)).toBeNull();
    expect(fg.sync(`${control.plugin}/🔮️oracles/**/*.py`, { cwd: root })).toEqual([control.destination]);
  }, 15000);

  test("ancestor contributions select the anonymous local module for all fifteen owners", () => {
    const registry = loadOracleRegistry(root);
    for (const row of control.adapters) {
      expect(
        oracleHostPackagesFor(registry, row.owner, "python").map(({ package: name, path, module }) => ({
          package: name,
          path,
          module,
        })),
      ).toEqual([
        {
          package: control.package,
          path: dirname(control.destination),
          module: control.module,
        },
      ]);
    }
  }, 60000);

  test("native Python imports one engine and registers every committed oracle scenario identity", async () => {
    const child = Bun.spawn([oracleHostPython(root), join(import.meta.dir, "🐍️.py"), root, controlPath], {
      cwd: root,
      env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1" },
      stdout: "pipe",
      stderr: "pipe",
      timeout: 25000,
    });
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    expect({ status, stderr }).toEqual({ status: 0, stderr: "" });
    const result = JSON.parse(stdout) as {
      declaration: { module: string; path: string };
      adapters: {
        path: string;
        selectors: string[];
        moduleFiles: string[];
        definitions: string[];
        handlers: string[];
      }[];
    };
    expect(result.declaration.module).toBe(control.module);
    expect(result.declaration.path).toBe(dirname(control.destination));
    expect(result.adapters.length).toBe(control.adapters.length);
    for (const [index, row] of result.adapters.entries()) {
      const fixture = control.adapters[index]!;
      const feature = parseFeature(read(fixture.feature));
      expect(feature.scenarios.length).toBeGreaterThan(0);
      expect(row.path).toBe(fixture.path);
      expect(row.selectors).toEqual([control.module]);
      expect(row.moduleFiles).toEqual([control.destination]);
      expect(row.definitions).toEqual(["adapter"]);
      expect(row.handlers).toEqual([...new Set(feature.scenarios.map((scenario) => `${scenario.id}::oracle`))].sort());
    }
  }, 30000);

  test("registered execution targets include the manifest, engine and every dynamic adapter", () => {
    const projectPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json";
    const project = JSON.parse(read(projectPath));
    expect(project.namedInputs.normOracleSources?.slice().sort()).toEqual(normSources);
    for (const target of ["test-oracle", "test-parity", "test"]) expect(project.targets[target].inputs).toContain("normOracleSources");
    const local = JSON.parse(read(`${control.plugin}/📦️packages/🟦️typescript/📋️project.json`));
    expect(local.namedInputs.normOracleSources?.slice().sort()).toEqual(normSources);
    expect(local.targets["test-oracle-source"].inputs).toContain("normOracleSources");
    expect(local.targets["test-oracle-source"].options.command).toBe("bun ./📜️script.ts oracle-source");
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launches = Bun.JSONC.parse(read(path)).configurations;
      expect(launches.filter((entry: { command: string }) => entry.command === "bun nx run @semio-tech/norm-js:test-oracle-source")).toHaveLength(1);
    }
  });

  test("the committed native host executes the selected EN 1991 case from the real plan builder", async () => {
    const selected = control.adapters.find((row) => row.path.includes("/🏋️mutate-en1991-1/"))!;
    const caseDir = dirname(selected.path),
      caseName = basename(caseDir);
    const discovered = {
      owner: selected.owner,
      ownerName: basename(selected.owner),
      case: caseName,
      caseDir,
      featurePath: selected.feature,
      adapters: { python: selected.path },
      sharedFixtureDir: `${selected.owner}/🧫️fixtures`,
      projectName: testProjectName(selected.owner, caseName),
    };
    const built = buildCasePlan(root, discovered, "exhaustive");
    expect(built.feature.errors).toEqual([]);
    expect(built.missingFixtures).toEqual([]);
    expect(built.plan.scenarios).toHaveLength(65);
    expect(built.plan.target).toEqual({
      artifact: "s.norm.en1991",
      standard: "1",
      subset: "any",
    });
    const directory = mkdtempSync(join(tmpdir(), "semio-norm-oracle-"));
    try {
      const planPath = join(directory, "🔣️.json"),
        resultsPath = join(directory, "results.jsonl");
      const plan = {
        ...built.plan,
        role: "oracle",
        implementation: "python",
        workDir: join(directory, "work"),
        outputDir: join(directory, "output"),
        artifactDir: join(directory, "artifacts"),
        resultsPath,
      };
      writeFileSync(planPath, JSON.stringify(plan));
      const declarations = oracleHostPackagesFor(loadOracleRegistry(root), selected.owner, "python");
      const localPaths = declarations.map((entry) => join(root, entry.path!));
      const pythonPath = localPaths.join(delimiter);
      const child = Bun.spawn([oracleHostPython(root), ...pythonHostArguments(join(root, control.host), planPath, resultsPath, join(root, selected.path), localPaths)], {
        cwd: root,
        env: {
          ...process.env,
          PYTHONDONTWRITEBYTECODE: "1",
          PYTHONPATH: pythonPath,
        },
        stdout: "pipe",
        stderr: "pipe",
        timeout: 25000,
      });
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      const results = existsSync(resultsPath)
        ? readFileSync(resultsPath, "utf8")
            .trim()
            .split("\n")
            .map((line) => JSON.parse(line))
        : [];
      const failures = results.filter((entry) => entry.status !== "passed").map(({ scenario, diagnostics }) => ({ scenario, diagnostics }));
      expect({ status, stderr, failures }).toEqual({
        status: 0,
        stderr: "",
        failures: [],
      });
      expect(results).toHaveLength(65);
      expect(results.map((entry) => entry.scenario).sort()).toEqual(built.plan.scenarios.map((entry) => entry.id).sort());
      expect(results.every((entry) => entry.status === "passed" && entry.role === "oracle" && entry.implementation === "python")).toBe(true);
      expect(stdout).toBe("");
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  }, 30000);
});
