import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { TestCasePlan, TestRole } from "@semio-tech/framework/test/adapter";
import { evaluateParity, loadOracleRegistry, profileTable, readResults } from "@semio-tech/repo-test";

const repository = resolve(import.meta.dir, "../../../../../../..");
const retained = join(repository, ".cache/b/o/m/tests/e2885f2bbe3d7a8b08e328b9/9f6c907fd51368ab");
const subject = "test-s-plugins-stdio-artifacts-xlsx-standards-ecma376-subsets-base-bbaa39-🔀️mutate-xlsx-ecma-376-subject-rust";
const manifest = join(retained, "hosts", subject, "Cargo.toml");
const originalPath = join(retained, "work", subject, "📋️plan.json");
const oraclePath = join(retained, "reports", "latest", "📤️results.jsonl");
const variant = process.argv[3];
const generated = join(import.meta.dir, "🗑️generated", variant === "isolated" ? "retained-iso" : variant === "fix" ? "retained-fix" : "retained-j1");
const store = variant === "isolated" ? "isolated" : "shared";
const planPath = join(generated, "plan.json");
const argumentPath = join(generated, "cargo-arguments.json");
const destinations = {
  workDir: join(generated, "work"),
  outputDir: join(generated, "output"),
  artifactDir: join(generated, "artifacts"),
  resultsPath: join(generated, "results.jsonl"),
};
const cargoArguments = [
  "cargo", `--store=${store}`, "--configuration=prototype", "--parallelism=1", "--",
  "run", "--locked", "--manifest-path", manifest, "-p", "semio-test-host--mutate-xlsx-ecma-376",
  "--bin", "host", "--features", "sut", "--", "--plan", planPath, "--out", destinations.resultsPath,
];

const readPlan = (path: string): TestCasePlan => JSON.parse(readFileSync(path, "utf8"));
const original = readPlan(originalPath);
if (original.schemaVersion !== 2 || original.case !== "🔀️mutate-xlsx-ecma-376" || original.level !== "exhaustive" ||
    original.role !== "subject" || original.implementation !== "rust" || original.comparisonPipeline !== null ||
    original.oracleInput !== null || original.scenarios.length !== 18) {
  throw Error("Expected the retained exhaustive XLSX Rust subject plan");
}

/** 🧪️ Reads the production result protocol and requires coverage of every authentic scenario. */
function phase(path: string, role: TestRole) {
  const { results, problems } = readResults(path);
  if (problems.length > 0) throw Error(problems.join("\n"));
  const expected = new Map(original.scenarios.map((scenario) => [scenario.id, scenario.level]));
  for (const row of results) {
    if (row.owner !== original.owner || row.case !== original.case || row.level !== expected.get(row.scenario) ||
        row.implementation !== original.implementation || row.role !== role || !expected.delete(row.scenario)) {
      throw Error(`Unexpected ${role} result ${row.testId}`);
    }
  }
  if (expected.size > 0) throw Error(`Missing ${role} results for ${[...expected.keys()].join(", ")}`);
  return results;
}

const expectedPlan = JSON.stringify({ ...original, ...destinations }, null, 2) + "\n";
const expectedArguments = JSON.stringify(cargoArguments, null, 2) + "\n";
const oracle = phase(oraclePath, "oracle");
if (oracle.some((row) => row.status !== "passed")) throw Error("The retained oracle did not pass every scenario");

/** 🔏️ Verifies exact scenario inputs and allows changes only to the four output destinations. */
function verifyPlan() {
  if (readFileSync(planPath, "utf8") !== expectedPlan ||
      readFileSync(argumentPath, "utf8") !== expectedArguments) {
    throw Error("The supplemental plan or Cargo arguments have changed");
  }
}

switch (process.argv[2]) {
  case "prepare":
    if (!existsSync(manifest) || !existsSync(join(retained, "hosts", subject, "Cargo.lock"))) {
      throw Error("The retained subject workspace or lockfile has expired");
    }
    mkdirSync(generated);
    writeFileSync(planPath, expectedPlan, { flag: "wx" });
    writeFileSync(argumentPath, expectedArguments, { flag: "wx" });
    verifyPlan();
    console.log(JSON.stringify({ supplemental: true, command: "repo", cargoArguments, planPath }));
    break;
  case "verify":
    verifyPlan();
    if (existsSync(destinations.resultsPath)) throw Error("A supplemental subject result already exists");
    console.log(JSON.stringify({ supplemental: true, command: "repo", cargoArguments, planPath }));
    break;
  case "compare": {
    verifyPlan();
    const subjectRows = phase(destinations.resultsPath, "subject");
    const { verdicts, unmatched } = evaluateParity(original.comparison, [...oracle, ...subjectRows], profileTable(loadOracleRegistry(repository)));
    const passed = subjectRows.filter((row) => row.status === "passed").length;
    const equal = verdicts.filter((row) => row.equal).length;
    const report = { supplemental: true, oracle: oracle.length, subject: subjectRows.length, passed, parity: { equal, comparisons: verdicts.length }, verdicts, unmatched };
    writeFileSync(join(generated, "comparison.json"), JSON.stringify(report, null, 2) + "\n", { flag: "wx" });
    console.log(JSON.stringify({ ...report, verdicts: undefined }));
    process.exitCode = passed === subjectRows.length && equal === subjectRows.length && unmatched.length === 0 ? 0 : 1;
    break;
  }
  default:
    throw Error("Expected prepare, verify, or compare");
}
