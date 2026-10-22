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

const reason = { en: "Tests keep direct spawns.", de: "Tests behalten direkte Starts." };
const empty: ToolSpawnLedgerV1 = { version: 1, reason, entries: [] };
const sources: Record<string, string> = {
  "gen/🏭️generator/🟦️.ts": "spawnSync(\"cargo\", [\"build\"]);\nspawnSync(\"cargo\", [\"test\"]);\nspawnSync(\"git\", []);\n",
  "gen/🏭️generator/🧪️tests/🟦️.ts": "spawnSync(\"bun\", []);\n",
  "svc/🟦️.ts": "await runToolV1(\"hdiutil\", []);\nawait runToolV1(\"nothing\", []);\nawait runToolV1(\"git\", []);\n",
  "svc/🐚️.sh": "cargo build\n",
  "svc/📋️project.json": "{\"targets\":{\"a\":{\"options\":{\"command\":\"bun x\"}}}}",
  "🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts": "spawn(\"cargo\", []);\n",
};
const read = (path: string): string | undefined => sources[path];
const scan = (): ReturnType<typeof scanRepositoryToolSpawnsV1> => scanRepositoryToolSpawnsV1("/r", Object.keys(sources), undefined, read);
const testLedger = (): ToolSpawnLedgerV1 => ({ version: 1, reason, entries: summarizeToolSpawnsV1(scan().sites).entries.filter(row => row.scope === "test").map(({ scope: _, ...row }) => row) });

describe("direct tool spawn law", () => {
  test("keeps its test ledger valid against the schema with a third-party validator", () => {
    const ajv = new (Ajv as any)({ strict: false, allErrors: true });
    expect(ajv.validate(ledgerSchema, ledgerDocument)).toBe(true);
    expect(() => readToolSpawnLedgerV1({ ...ledgerDocument, entries: [{ path: "", tool: "x", form: "spawn", count: 0 }] })).toThrow("invalid");
    expect(() => readToolSpawnLedgerV1({ ...ledgerDocument, reason: { en: "only English" } })).toThrow("invalid");
  });
  test("counts only spawns outside the adapter scope and ignores the tools module", () => {
    const summary = summarizeToolSpawnsV1(scan().sites);
    expect(summary.entries.map(row => `${row.path} ${row.tool} ${row.form} ${row.scope} ${row.count}`)).toEqual([
      "gen/🏭️generator/🟦️.ts cargo spawn prod 2", "gen/🏭️generator/🟦️.ts git spawn prod 1", "gen/🏭️generator/🧪️tests/🟦️.ts bun spawn test 1", "svc/🐚️.sh cargo shell prod 1",
    ]);
    expect(summary.declared).toEqual([{ tool: "bun", scope: "prod", count: 1 }]);
  });
  test("refuses every production spawn, tolerates ledgered test spawns and reports stale rows", () => {
    const result = toolSpawnFindingsV1(scan(), testLedger()).findings.filter(row => row.rule === "raw-spawn");
    expect(result.map(row => `${row.path} ${row.tool}`)).toEqual(["gen/🏭️generator/🟦️.ts cargo", "gen/🏭️generator/🟦️.ts git", "svc/🐚️.sh cargo"]);
    expect(toolSpawnFindingsV1(scan(), empty).findings.filter(row => row.rule === "raw-spawn").map(row => row.path)).toContain("gen/🏭️generator/🧪️tests/🟦️.ts");
    const gone: ToolSpawnLedgerV1 = { version: 1, reason, entries: [...testLedger().entries, { path: "gone.ts", tool: "go", form: "spawn", count: 2 }] };
    expect(toolSpawnFindingsV1(scan(), gone).stale).toEqual(["gone.ts go spawn 0/2"]);
  });
  test("checks calls of the tool interface against the registry", () => {
    const found = toolSpawnFindingsV1(scan(), testLedger()).findings;
    expect(found.filter(row => row.rule === "unknown-tool").map(row => row.tool)).toEqual(["nothing"]);
    expect(found.filter(row => row.rule === "caller-not-allowed").map(row => row.tool)).toEqual(["hdiutil"]);
  });
  test("prunes a ledger to the truth: counts fall, rows vanish, nothing is added unless accepted", () => {
    const actual = summarizeToolSpawnsV1(scan().sites);
    const old: ToolSpawnLedgerV1 = { version: 1, reason, entries: [{ path: "gen/🏭️generator/🧪️tests/🟦️.ts", tool: "bun", form: "spawn", count: 9 }, { path: "gone.ts", tool: "go", form: "spawn", count: 1 }] };
    expect(pruneToolSpawnLedgerV1(old, actual)).toEqual({ version: 1, reason, entries: [{ path: "gen/🏭️generator/🧪️tests/🟦️.ts", tool: "bun", form: "spawn", count: 1 }] });
    expect(pruneToolSpawnLedgerV1(empty, actual, true)).toEqual(testLedger());
  });
  test("keeps the reason of the ledger through render and prune", () => {
    const ledger = testLedger();
    const again = readToolSpawnLedgerV1(JSON.parse(renderToolSpawnLedgerV1(ledger)));
    expect(again.reason).toEqual(reason);
    expect(again.entries).toEqual(ledger.entries);
    expect(pruneToolSpawnLedgerV1(ledger, summarizeToolSpawnsV1(scan().sites)).entries).toEqual(ledger.entries);
  });
  test("accepts a registry that matches the repository and reports one that does not", () => {
    const problems = toolRegistryProblemsV1(process.cwd(), new Set(["x"]));
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
  test("fails on production offenders and names them", () => {
    const result = run([]);
    expect(result.status).toBe(1);
    expect(result.lines).toContain("finding(s)");
  });
  test("prints a report without failing and writes a pruned test ledger on request", () => {
    expect(run(["--report"]).lines).toContain("cargo prod");
    mkdirSync(join(root, TOOL_SPAWN_LEDGER_FILE_V1, ".."), { recursive: true });
    expect(run(["--prune", "--accept-new"]).status).toBe(0);
    const written = JSON.parse(readFileSync(join(root, TOOL_SPAWN_LEDGER_FILE_V1), "utf8"));
    expect(written.entries.length).toBe(1);
    expect(run([], readToolSpawnLedgerV1(written)).lines).toContain("finding(s)");
  });
});
