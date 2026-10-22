import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { createDirectToolLawV1, DIRECT_TOOL_PATTERNS_V1, directToolGlobV1, listDirectToolFilesV1, scanDirectToolDeclarationsV1 } from "../🟦️.ts";
import { runDirectToolDeclarationLawV1 } from "../⌨️command/🟦️.ts";
import patterns from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "./🧫️fixtures/🔣️.json" with { type: "json" };

const here = dirname(fileURLToPath(import.meta.url));
const root = (() => { for (let directory = here; ; directory = dirname(directory)) { if (existsSync(join(directory, "MODULE.bazel"))) return directory; if (dirname(directory) === directory) return process.cwd(); } })();
const oracle = join(here, "🐍️oracle.py");
const law = createDirectToolLawV1();

describe("direct tool declarations: pattern document", () => {
  test("validates against its schema with a third-party validator", () => {
    const ajv = new (Ajv as any)({ strict: false, allErrors: true });
    expect(ajv.validate(schema, patterns)).toBe(true);
    expect(DIRECT_TOOL_PATTERNS_V1.enforced.length).toBe(patterns.enforced.length);
  });
  test("names the tools of the user's vision as enforced", () => {
    const names = new Set(patterns.enforced.flatMap(row => row.commands));
    for (const name of ["bun", "bunx", "node", "npx", "cargo", "bazel", "nx", "go", "python", "python3", "uv", "dotnet", "cmake"]) expect(names.has(name)).toBe(true);
  });
  test("gives every exclusion and allowance a reason in both languages", () => {
    for (const row of [...patterns.exclude, ...patterns.allow]) { expect(row.reason.en.length).toBeGreaterThan(10); expect(row.reason.de.length).toBeGreaterThan(10); }
  });
  test("matches globs the way the other implementation does", () => {
    expect(directToolGlobV1("**/*.md").test("a/b/c.md")).toBe(true);
    expect(directToolGlobV1("**/*.md").test("c.md")).toBe(true);
    expect(directToolGlobV1("a/*/c").test("a/b/d/c")).toBe(false);
  });
});

describe("direct tool declarations: shared fixture rows", () => {
  test("every command row finds exactly its tools", () => {
    for (const row of fixtures.cases) {
      const got = [...new Set(law.commands(row.command).filter(([, cls]) => cls === "enforced").map(([tool]) => tool))].sort();
      expect({ id: row.id, got }).toEqual({ id: row.id, got: [...new Set(row.tools)].sort() });
    }
  });
  test("every file row finds its findings at their lines", () => {
    for (const row of fixtures.files) {
      const got = law.scanText(row.path, row.text).filter(finding => finding.allowed === null).map(finding => [finding.tool, finding.line]);
      expect({ id: row.id, got }).toEqual({ id: row.id, got: row.findings });
    }
  });
  test("every path row is excluded, typed and allowed as declared", () => {
    for (const row of fixtures.paths) {
      const got = { excluded: law.excluded(row.path), kind: law.kindOf(row.path)?.id ?? null, allowed: law.allowed(row.path, (row as any).tool ?? "node", (row as any).context ?? "line") };
      for (const [key, value] of Object.entries(row.expect)) expect({ id: row.id, key, value: (got as any)[key] }).toEqual({ id: row.id, key, value });
    }
  });
});

describe("direct tool declarations: independent Python implementation", () => {
  const python = (args: string[], cwd?: string) => runToolSyncV1("python", ["-I", oracle, ...args], { cwd, check: false, encoding: "utf8", caller: import.meta.url });
  test("passes the same fixture rows", () => {
    const result = python(["--patterns", join(here, "..", "🔣️.json"), "--cases", join(here, "🧫️fixtures", "🔣️.json")]);
    expect({ status: result.status, out: result.stdout.trim() }).toEqual({ status: 0, out: "cases failed: 0" });
  });
  test("reports the same findings on a scratch repository", () => {
    const directory = mkdtempSync(join(tmpdir(), "tool-declarations-"));
    try {
      const files: Record<string, string> = { "README.md": "Run `bun install`.\n\n```sh\ncargo test\nrepo run a:b\n```\n", ".github/workflows/x.yml": "jobs:\n  a:\n    steps:\n      - run: |\n          node -e 1\n          repo preflight\n", "package.json": "{\"scripts\":{\"a\":\"bun x.ts\",\"b\":\"repo run c\"}}\n", "ok.sh": "repo --help\n", "tools/🧪️tests/x.md": "```sh\nbun x\n```\n" };
      for (const [path, text] of Object.entries(files)) { const full = join(directory, path); mkdirSync(dirname(full), { recursive: true }); writeFileSync(full, text); }
      const mine = law.scanText("README.md", files["README.md"]!).concat(law.scanText(".github/workflows/x.yml", files[".github/workflows/x.yml"]!), law.scanText("package.json", files["package.json"]!)).map(finding => `${finding.path}:${finding.line}:${finding.tool}`).sort();
      const list = join(directory, "list.txt");
      writeFileSync(list, Object.keys(files).join("\n"));
      const result = python(["--patterns", join(here, "..", "🔣️.json"), "--root", directory, "--files", list, "--json"]);
      const theirs = (JSON.parse(result.stdout).findings as Array<{ path: string; line: number; tool: string }>).map(finding => `${finding.path}:${finding.line}:${finding.tool}`).sort();
      expect(theirs).toEqual(mine);
      expect(mine).toEqual(["README.md:1:bun", "README.md:4:cargo", ".github/workflows/x.yml:5:node", "package.json:1:bun"].sort());
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });
  test("refuses an empty scope", () => {
    const result = python(["--patterns", join(here, "..", "🔣️.json"), "--root", root, "--files", "/dev/null"]);
    expect(result.status).toBe(2);
  });
});

describe("direct tool declarations: the repository", () => {
  test("has no direct tool invocation outside the allowed paths", () => {
    const scan = scanDirectToolDeclarationsV1(root, listDirectToolFilesV1(root));
    const live = scan.findings.filter(finding => finding.allowed === null).map(finding => `${finding.path}:${finding.line} ${finding.tool} ${finding.command.slice(0, 60)}`);
    expect(live).toEqual([]);
    expect(scan.scanned).toBeGreaterThan(1000);
  }, 900000);
  test("refuses an empty scope", () => {
    expect(() => scanDirectToolDeclarationsV1(root, [])).toThrow("empty-scope");
  });
  test("the command answers 0 when clean and prints the German text on request", () => {
    const lines: string[] = [];
    const status = runDirectToolDeclarationLawV1(root, [], line => lines.push(line), { SEMIO_LANGUAGE: "de" });
    expect(status).toBe(0);
    expect(lines.join("\n")).toContain("bestanden");
  }, 900000);
  test("every pinned fixture file is read as JSON", () => {
    expect(JSON.parse(readFileSync(join(here, "🧫️fixtures", "🔣️.json"), "utf8")).version).toBe(1);
  });
});
