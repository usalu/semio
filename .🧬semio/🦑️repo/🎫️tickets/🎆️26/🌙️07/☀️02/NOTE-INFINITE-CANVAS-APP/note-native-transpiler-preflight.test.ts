import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { resolve } from "node:path";
import ts from "typescript";
import fixture from "./note-native-transpiler-preflight.json";
import { nativeTranspilePublicationPathV1, nativeTranspileRequiresBuildV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🟦️.ts";

const driverPath = new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts", import.meta.url);
const source = ts.createSourceFile(driverPath.pathname, await Bun.file(driverPath).text(), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const driver = source.statements.find((statement) => ts.isFunctionDeclaration(statement) && statement.name?.text === "runRepositoryComponentDriverV1");
if (!driver || !ts.isFunctionDeclaration(driver) || !driver.body) throw new Error("Missing component driver");
const cache = driver.body.statements.find(ts.isReturnStatement);
if (!cache) throw new Error("Missing component compiler scope");
const beforeCompiler = source.text.slice(driver.body.getStart(source) + 1, cache.getStart(source));
const program = ts.transpileModule(`async function run(repoRoot: string, mode: string, plan: any, options: any) { ${beforeCompiler} return beginCompiler(); }`, {
  compilerOptions: { target: ts.ScriptTarget.ES2022 },
}).outputText;
const evaluate = new Function("resolve", "componentStageTargetV1", "ensureNativeTranspileV1", "beginCompiler", program + "\nreturn run;");
const repo = process.cwd();

test("the browser driver preflights precisely its deployable targets before compiler custody", async () => {
  const db = new Database(":memory:");
  const order = db.query<{ ordering: string }, [string, number, number]>(
    "SELECT CASE WHEN ?1 != 'browser' OR ?2 = 0 THEN 'compiler' WHEN ?3 = 1 THEN 'preflight' ELSE 'preflight,compiler' END AS ordering",
  );
  try {
    for (const row of fixture.cases) {
      const invocation = {}, control = {}, steps: string[] = [];
      const environment = row.existing === "override" ? { SEMIO_NATIVE_TRANSPILE_BIN: "owned-transpiler" } : {};
      const target = { manifestPath: resolve(repo, "Cargo.toml"), directoryName: row.deployable ? "fixture" : undefined };
      const expects = order.get(row.facet, Number(row.deployable), Number(row.fail === true))!.ordering;
      expect(row.steps.join(",")).toBe(expects);
      const run = evaluate(resolve, () => target, async (...args: unknown[]) => {
        expect(args).toEqual([repo, invocation, control, environment]);
        steps.push("preflight");
        if (row.fail) throw Error("native transpile refused");
      }, () => { steps.push("compiler"); });
      const outcome = run(repo, row.mode, { passes: [{ packages: [{ manifest: "Cargo.toml" }] }] }, { facet: row.facet, invocation, control, environment });
      if (row.fail) await expect(outcome).rejects.toThrow("native transpile refused");
      else await outcome;
      expect(steps).toEqual(row.steps);
    }
  } finally {
    db.close();
  }
});

test("an available opposite mode does not satisfy the selected platform publication", () => {
  for (const row of fixture.cases.filter(({ existing }) => existing !== "none")) {
    const mode = row.mode as "release" | "prototype";
    const override = row.existing === "override" ? "owned-transpiler" : undefined;
    const existing = override ? resolve(repo, override) : nativeTranspilePublicationPathV1(repo, row.existing as "release" | "prototype", row.platform);
    const saw: string[] = [];
    expect(nativeTranspileRequiresBuildV1(repo, mode, { SEMIO_NATIVE_TRANSPILE_BIN: override }, row.platform, path => {
      saw.push(path);
      return path === existing;
    })).toBe(!override);
    expect(saw).toEqual([override ? existing : nativeTranspilePublicationPathV1(repo, mode, row.platform)]);
  }
});
