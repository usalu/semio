import { expect, mock, test } from "bun:test";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import * as runtime from "@semio-tech/repo-test";

const repository = resolve(import.meta.dir, "../../../../../../..");
const framework = join(repository, "🧰️framework");
const domain = join(framework, "🛍️products/🦑️repo/🔨️modules/🧪️test");
const { declareBuildTestEnvelopeV1, withBuildTestInvocationV1 } = await import(join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts"));
declareBuildTestEnvelopeV1({ owner: "test:xlsx-parity-coordinator", maximumElapsedMilliseconds: 45_000, capabilities: {} });
const generated = join(import.meta.dir, "🗑️generated");
const retained = join(generated, "retained-iso");
const phasePaths = {
  oracle: join(repository, ".cache/b/o/m/tests/e2885f2bbe3d7a8b08e328b9/9f6c907fd51368ab/reports/latest/📤️results.jsonl"),
  subject: join(retained, "results.jsonl"),
} as const;
const sha256 = {
  oracle: "669552e56aad183265df18f815b8407e19847c31fa844455b09444f8588d63ac",
  subject: "70b3c23100037643d3ebf677e38a025088c4dd058835e13a05a9f5829af5f661",
  plan: "884255f4f445dfea777fbd59982a2349c9e59078e2d074afbaef9bb692f43d5c",
} as const;

/** 🔏️ Reads only the retained evidence that matches the recorded integrity fingerprints. */
function retainedBytes(path: string, identity: keyof typeof sha256) {
  const bytes = readFileSync(path);
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(sha256[identity]);
  return bytes.toString("utf8");
}

test("production XLSX parity coordinator pairs both retained native phases", async () => {
  const output = mkdtempSync(join(generated, "pair-coordinator-"));
  const original = JSON.parse(retainedBytes(join(retained, "plan.json"), "plan"));
  const { role, implementation, workDir, outputDir, artifactDir, resultsPath, ...expectedPlan } = original;
  const observedPhases: string[] = [];
  const execution = join(domain, "🏃️execution/🎬️scenario/🟦️.ts");
  const reporting = join(domain, "📊️reporting/📋️orchestration/🟦️.ts");
  const barrel = join(domain, "📦️packages/🟦️typescript/🟦️.ts");
  mock.module(execution, () => ({
    executeOne: async (root, discovered, level, currentRole: keyof typeof phasePaths, currentImplementation) => {
      expect(root).toBe(repository);
      expect(currentImplementation).toBe("rust");
      const { plan, missingFixtures } = runtime.buildCasePlan(root, discovered, level);
      expect(JSON.parse(JSON.stringify(plan))).toEqual(expectedPlan);
      expect(missingFixtures).toEqual([]);
      retainedBytes(phasePaths[currentRole], currentRole);
      const outcome = runtime.readResults(phasePaths[currentRole]);
      expect(outcome.problems).toEqual([]);
      expect(outcome.results.map((row) => [row.owner, row.case, row.role, row.status])).toEqual(
        original.scenarios.map(() => [original.owner, original.case, currentRole, "passed"]),
      );
      expect(outcome.results.map((row) => row.scenario).sort()).toEqual(original.scenarios.map((scenario) => scenario.id).sort());
      observedPhases.push(currentRole);
      return outcome;
    },
  }));
  const reportingExports = { ...await import(reporting) };
  const runtimeExports = { ...runtime };
  mock.module(reporting, () => ({ ...reportingExports, reportsDir: () => output }));
  mock.module(barrel, () => ({ ...runtimeExports, testCacheDir: (_root, child) => join(output, child) }));

  const { ParityScript } = await import(join(domain, "⚖️parity/📋️orchestration/🟦️.ts"));
  const result = await withBuildTestInvocationV1(async (invocation) => {
    const savedExit = process.exitCode;
    try {
      await new ParityScript(domain, repository, invocation).run(["exhaustive", "--owner-root", original.owner, "--case", original.case]);
      return process.exitCode;
    } finally {
      process.exitCode = savedExit;
    }
  });
  expect(result).toBe(0);
  expect(observedPhases).toEqual(["oracle", "subject"]);
  const summary = JSON.parse(readFileSync(join(output, "📊️summary.json"), "utf8"));
  expect(summary).toMatchObject({ cases: 1, scenarios: 18, executed: 36, passed: 36, failed: 0, errored: 0, problems: [], oracleUnavailable: [] });
  expect(summary.parity).toHaveLength(18);
  expect(summary.parity.every((row) => row.equal && row.diffs === 0)).toBe(true);
}, 45_000);
