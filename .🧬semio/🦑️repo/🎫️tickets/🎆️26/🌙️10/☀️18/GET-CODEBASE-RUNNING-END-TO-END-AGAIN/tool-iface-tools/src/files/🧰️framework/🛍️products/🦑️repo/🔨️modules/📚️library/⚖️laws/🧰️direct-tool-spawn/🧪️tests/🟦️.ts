import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import ledgerDocument from "../🧾️ledger/🔣️.json" with { type: "json" };
import ledgerSchema from "../🧬️schema/🔣️.json" with { type: "json" };
import { runToolSpawnLawV1 } from "../⌨️command/🟦️.ts";
import { pruneToolSpawnLedgerV1, readToolSpawnLedgerV1, renderToolSpawnLedgerV1, scanRepositoryToolSpawnsV1, summarizeToolSpawnsV1, toolRegistryProblemsV1, toolSpawnFindingsV1, TOOL_SPAWN_LEDGER_FILE_V1, type ToolSpawnLedgerV1 } from "../🟦️.ts";

const empty: ToolSpawnLedgerV1 = { version: 1, entries: [], declared: [] };
const sources: Record<string, string> = {
  "gen/🏭️generator/🟦️.ts": "spawnSync(\"cargo\", [\"build\"]);\nspawnSync(\"cargo\", [\"test\"]);\nspawnSync(\"git\", []);\n",
  "gen/🏭️generator/🧪️tests/🟦️.ts": "spawnSync(\"bun\", []);\n",
  "svc/🟦️.ts": "await runToolV1(\"cargo\", []);\nawait runToolV1(\"nothing\", []);\nawait runToolV1(\"git\", []);\n",
  "svc/🐚️.sh": "cargo build\n",
  "svc/📋️project.json": "{\"targets\":{\"a\":{\"options\":{\"command\":\"bun x\"}}}}",
  "🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts": "spawn(\"cargo\", []);\n",
};
const read = (path: string): string | undefined => sources[path];

describe("direct tool spawn law", () => {
  test("keeps its ledger valid against the schema with a third-party validator", () => {
    const ajv = new (Ajv as any)({ strict: false, allErrors: true });
    expect(ajv.validate(ledgerSchema, ledgerDocument)).toBe(true);
    expect(() => readToolSpawnLedgerV1({ ...ledgerDocument, entries: [{ path: "", tool: "x", form: "spawn", scope: "prod", count: 0 }] })).toThrow("invalid");
  });
  test("counts only spawns outside the adapter scope and ignores the tools module", () => {
    const scan = scanRepositoryToolSpawnsV1("/r", Object.keys(sources), undefined, read), summary = summarizeToolSpawnsV1(scan.sites);
    expect(summary.entries.map(row => `${row.path} ${row.tool} ${row.form} ${row.scope} ${row.count}`)).toEqual([
      "gen/🏭️generator/🟦️.ts cargo spawn prod 2", "gen/🏭️generator/🟦️.ts git spawn prod 1", "gen/🏭️generator/🧪️tests/🟦️.ts bun spawn test 1", "svc/🐚️.sh cargo shell prod 1",
    ]);
    expect(summary.declared).toEqual([{ tool: "bun", scope: "prod", count: 1 }]);
  });
  test("refuses new and larger offenders, tolerates the ledgered ones and reports stale rows", () => {
    const scan = scanRepositoryToolSpawnsV1("/r", Object.keys(sources), undefined, read), actual = summarizeToolSpawnsV1(scan.sites);
    expect(toolSpawnFindingsV1(scan, actual).findings.filter(row => row.rule === "raw-spawn" || row.rule === "declared-grew")).toEqual([]);
    const smaller = { ...actual, entries: actual.entries.map(row => row.tool === "cargo" && row.path.endsWith("🟦️.ts") ? { ...row, count: 1 } : row) };
    expect(toolSpawnFindingsV1(scan, smaller).findings.filter(row => row.rule === "raw-spawn").map(row => row.detail)).toEqual(["1 spawn of cargo outside its adapter (1 tolerated)"]);
    const larger = { ...actual, entries: [...actual.entries, { path: "gone.ts", tool: "go", form: "spawn" as const, scope: "prod" as const, count: 2 }] };
    expect(toolSpawnFindingsV1(scan, larger).stale).toEqual(["gone.ts go spawn 0/2"]);
    expect(toolSpawnFindingsV1(scan, empty).findings.filter(row => row.rule === "declared-grew").length).toBe(1);
  });
  test("checks calls of the tool interface against the registry", () => {
    const scan = scanRepositoryToolSpawnsV1("/r", Object.keys(sources), undefined, read), found = toolSpawnFindingsV1(scan, summarizeToolSpawnsV1(scan.sites)).findings;
    expect(found.filter(row => row.rule === "unknown-tool").map(row => row.tool)).toEqual(["nothing"]);
    expect(found.filter(row => row.rule === "caller-not-allowed").map(row => row.tool)).toEqual(["cargo"]);
  });
  test("prunes a ledger to the truth: counts fall, rows vanish, nothing is added unless accepted", () => {
    const scan = scanRepositoryToolSpawnsV1("/r", Object.keys(sources), undefined, read), actual = summarizeToolSpawnsV1(scan.sites);
    const old: ToolSpawnLedgerV1 = { version: 1, entries: [{ path: "gen/🏭️generator/🟦️.ts", tool: "cargo", form: "spawn", scope: "prod", count: 9 }, { path: "gone.ts", tool: "go", form: "spawn", scope: "prod", count: 1 }], declared: [{ tool: "bun", scope: "prod", count: 5 }] };
    expect(pruneToolSpawnLedgerV1(old, actual)).toEqual({ version: 1, entries: [{ path: "gen/🏭️generator/🟦️.ts", tool: "cargo", form: "spawn", scope: "prod", count: 2 }], declared: [{ tool: "bun", scope: "prod", count: 1 }] });
    expect(pruneToolSpawnLedgerV1(old, actual, true)).toEqual(actual);
  });
  test("keeps the reasons of a ledger through render and prune", () => {
    const ledger: ToolSpawnLedgerV1 = { version: 1, reasons: { test: { en: "Tests keep direct spawns.", de: "Tests behalten direkte Starts." } }, entries: [{ path: "a/🧪️tests/b.ts", tool: "git", form: "spawn", scope: "test", count: 1, reason: "moves with its owner" }], declared: [] };
    const again = readToolSpawnLedgerV1(JSON.parse(renderToolSpawnLedgerV1(ledger)));
    expect(again.reasons).toEqual(ledger.reasons);
    expect(again.entries).toEqual(ledger.entries);
    expect(pruneToolSpawnLedgerV1(ledger, ledger)).toEqual(ledger);
    expect(() => readToolSpawnLedgerV1({ ...ledger, reasons: { test: { en: "only English" } } })).toThrow("invalid");
  });
  test("accepts a registry that matches the repository and reports one that does not", () => {
    const files = new Set(["x"]);
    const problems = toolRegistryProblemsV1(process.cwd(), files);
    expect(problems.length).toBeGreaterThan(0);
    expect(problems.every(row => row.rule === "registry")).toBe(true);
  });
});

describe("repo verify tool-spawns", () => {
  const root = mkdtempSync(join(tmpdir(), "tool-spawn-law-"));
  const error = console.error;
  beforeAll(() => { console.error = () => undefined; });
  afterAll(() => { console.error = error; rmSync(root, { recursive: true, force: true }); });
  for (const [path, text] of Object.entries(sources)) { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), text); }
  spawnSync("git", ["init", "-q"], { cwd: root });
  const run = (args: string[], ledger = empty) => { const lines: string[] = []; const status = runToolSpawnLawV1(root, args, line => lines.push(line), ledger); return { status, lines: lines.join("\n") }; };
  test("fails on offenders missing from the ledger and names them", () => {
    const result = run([]);
    expect(result.status).toBe(1);
    expect(result.lines).toContain("finding(s)");
  });
  test("prints a report without failing and writes a pruned ledger on request", () => {
    expect(run(["--report"]).lines).toContain("cargo prod");
    mkdirSync(join(root, TOOL_SPAWN_LEDGER_FILE_V1, ".."), { recursive: true });
    expect(run(["--prune", "--accept-new"]).status).toBe(0);
    const written = JSON.parse(readFileSync(join(root, TOOL_SPAWN_LEDGER_FILE_V1), "utf8"));
    expect(written.entries.length).toBe(4);
    expect(run([], readToolSpawnLedgerV1(written)).lines).not.toContain("raw-spawn");
  });
});
