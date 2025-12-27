#!/usr/bin/env tsx
// #region Header

// repo.tests.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { spawnSync } from "child_process";
import { existsSync, readFileSync } from "fs";
import { dirname, join } from "path";
import { describe, expect, test } from "vitest";
import { fileURLToPath } from "url";

// #endregion Imports

// #region Constants

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT_DIR = __dirname;
const ASSETS_REPO_DIR = join(ROOT_DIR, "assets", "repo");
const CLI_TIMEOUT = 5000;

// #endregion Constants

// #region Utilities

function runCli(args: string, timeout = CLI_TIMEOUT): { stdout: string; stderr: string; exitCode: number } {
  const result = spawnSync("npx", ["tsx", "repo.tsx", ...args.split(" ").filter(Boolean)], { cwd: ROOT_DIR, encoding: "utf-8", shell: true, timeout });
  return { stdout: result.stdout ?? "", stderr: result.stderr ?? "", exitCode: result.status ?? 1 };
}

// #endregion Utilities

// #region Help Command Tests

describe("help command", () => {
  test("shows help with no arguments", () => {
    const result = runCli("");
    expect(result.stdout).toContain("repo - Monorepo CLI for Semio");
    expect(result.stdout).toContain("Commands:");
    expect(result.stdout).toContain("analyze");
    expect(result.stdout).toContain("fix");
    expect(result.stdout).toContain("rule");
    expect(result.stdout).toContain("ticket");
    expect(result.stdout).toContain("project");
    expect(result.stdout).toContain("folder");
    expect(result.stdout).toContain("file");
    expect(result.stdout).toContain("region");
    expect(result.stdout).toContain("definition");
    expect(result.stdout).toContain("tool");
  });

  test("shows help with help command", () => {
    const result = runCli("help");
    expect(result.stdout).toContain("repo - Monorepo CLI for Semio");
    expect(result.exitCode).toBe(0);
  });

  test("shows help with --help flag", () => {
    const result = runCli("--help");
    expect(result.stdout).toContain("repo - Monorepo CLI for Semio");
    expect(result.exitCode).toBe(0);
  });

  test("shows help with -h flag", () => {
    const result = runCli("-h");
    expect(result.stdout).toContain("repo - Monorepo CLI for Semio");
    expect(result.exitCode).toBe(0);
  });
});

// #endregion Help Command Tests

// #region Rule Command Tests

describe("rule command", () => {
  test("rule list shows all rules", () => {
    const result = runCli("rule list");
    expect(result.stdout).toContain("Registered rules");
    expect(result.stdout).toContain("header");
    expect(result.stdout).toContain("region");
    expect(result.stdout).toContain("comment");
  });

  test("rule usage shown without subcommand", () => {
    const result = runCli("rule");
    expect(result.stdout).toContain("Usage:");
  });
});

// #endregion Rule Command Tests

// #region Ticket Command Tests

describe("ticket command", () => {
  test("ticket list runs", () => {
    const result = runCli("ticket list");
    expect(result.stdout).toContain("Found");
  });

  test("ticket usage shown without subcommand", () => {
    const result = runCli("ticket");
    expect(result.stdout).toContain("Usage:");
  });
});

// #endregion Ticket Command Tests

// #region Folder Command Tests

describe("folder command", () => {
  test("folder tree shows folder structure", () => {
    const result = runCli("folder tree assets/repo");
    expect(result.stdout).toContain("Folder tree");
    expect(result.stdout).toContain("some");
    expect(result.stdout).toContain("reports");
  });

  test("folder tree with non-existent path shows error", () => {
    const result = runCli("folder tree nonexistent/path");
    expect(result.stderr + result.stdout).toContain("not found");
  });

  test("folder usage shown without subcommand", () => {
    const result = runCli("folder");
    expect(result.stdout).toContain("Usage:");
  });
});

// #endregion Folder Command Tests

// #region Region Command Tests

describe("region command", () => {
  test("region tree shows regions in TypeScript file", () => {
    const result = runCli("region tree assets/repo/some/folder/file.tsx");
    expect(result.stdout).toContain("Regions in");
    expect(result.stdout).toContain("Header");
    expect(result.stdout).toContain("Types");
    expect(result.stdout).toContain("Components");
    expect(result.stdout).toContain("Constants");
  });

  test("region tree shows regions in Python file", () => {
    const result = runCli("region tree assets/repo/some/folder/file.py");
    expect(result.stdout).toContain("Regions in");
    expect(result.stdout).toContain("Header");
    expect(result.stdout).toContain("Functions");
  });

  test("region tree shows regions in C# file", () => {
    const result = runCli("region tree assets/repo/some/folder/file.cs");
    expect(result.stdout).toContain("Regions in");
    expect(result.stdout).toContain("Header");
    expect(result.stdout).toContain("Classes");
  });

  test("region tree with non-existent file shows error", () => {
    const result = runCli("region tree nonexistent.tsx");
    expect(result.stderr + result.stdout).toContain("not found");
  });

  test("region tree without file shows error", () => {
    const result = runCli("region tree");
    expect(result.stderr + result.stdout).toContain("required");
  });
});

// #endregion Region Command Tests

// #region Definition Command Tests

describe("definition command", () => {
  test("definition list shows definitions in TypeScript file", () => {
    const result = runCli("definition list assets/repo/some/folder/file.tsx");
    expect(result.stdout).toContain("Definitions in");
    expect(result.stdout).toContain("TestType");
    expect(result.stdout).toContain("TestKind");
    expect(result.stdout).toContain("TestComponent");
    expect(result.stdout).toContain("TestClass");
    expect(result.stdout).toContain("TEST_CONSTANT");
    expect(result.stdout).toContain("TestEnum");
  });

  test("definition list shows definition kinds", () => {
    const result = runCli("definition list assets/repo/some/folder/file.tsx");
    expect(result.stdout).toContain("interface");
    expect(result.stdout).toContain("type");
    expect(result.stdout).toContain("function");
    expect(result.stdout).toContain("class");
    expect(result.stdout).toContain("variable");
    expect(result.stdout).toContain("enum");
  });

  test("definition list with non-existent file shows error", () => {
    const result = runCli("definition list nonexistent.tsx");
    expect(result.stderr + result.stdout).toContain("not found");
  });

  test("definition list without file shows error", () => {
    const result = runCli("definition list");
    expect(result.stderr + result.stdout).toContain("required");
  });
});

// #endregion Definition Command Tests

// #region Tool Command Tests

describe("tool command", () => {
  test("tool without target shows error", () => {
    const result = runCli("tool");
    expect(result.stderr + result.stdout).toContain("required");
  });
});

// #endregion Tool Command Tests

// #region Violations Fixture Tests

describe("violations fixture", () => {
  test("violations.json fixture has correct structure", () => {
    const fixturePath = join(ASSETS_REPO_DIR, "reports", "violations.json");
    expect(existsSync(fixturePath)).toBe(true);
    const content = JSON.parse(readFileSync(fixturePath, "utf-8"));
    expect(content.timestamp).toBeDefined();
    expect(content.status).toBe("warning");
    expect(content.scope).toBe("@semio");
    expect(content.summary.total).toBe(2);
    expect(content.violations).toHaveLength(2);
  });

  test("violations fixture contains expected violation kinds", () => {
    const fixturePath = join(ASSETS_REPO_DIR, "reports", "violations.json");
    const content = JSON.parse(readFileSync(fixturePath, "utf-8"));
    const kinds = content.violations.map((v: { kind: string }) => v.kind);
    expect(kinds).toContain("header:missing-region");
    expect(kinds).toContain("region:empty");
  });

  test("fixture violations have correct structure", () => {
    const fixturePath = join(ASSETS_REPO_DIR, "reports", "violations.json");
    const content = JSON.parse(readFileSync(fixturePath, "utf-8"));
    for (const violation of content.violations) {
      expect(violation).toHaveProperty("id");
      expect(violation).toHaveProperty("summary");
      expect(violation).toHaveProperty("kind");
      expect(violation).toHaveProperty("priority");
      expect(violation).toHaveProperty("severity");
      expect(violation).toHaveProperty("autofixable");
      expect(violation).toHaveProperty("solution");
      expect(violation).toHaveProperty("reason");
      expect(violation).toHaveProperty("scope");
    }
  });
});

// #endregion Violations Fixture Tests

// #region Test Fixtures Tests

describe("test fixtures", () => {
  test("file.tsx exists and has valid header", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.tsx");
    expect(existsSync(filePath)).toBe(true);
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Header");
    expect(content).toContain("file.tsx");
    expect(content).toContain("GNU Affero General Public License");
    expect(content).toContain("#endregion Header");
  });

  test("file.tsx has Types region with definitions", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.tsx");
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Types");
    expect(content).toContain("interface TestType");
    expect(content).toContain("type TestKind");
    expect(content).toContain("#endregion Types");
  });

  test("file.tsx has Components region with function", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.tsx");
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Components");
    expect(content).toContain("function TestComponent");
    expect(content).toContain("class TestClass");
    expect(content).toContain("#endregion Components");
  });

  test("file.tsx has Constants region", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.tsx");
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Constants");
    expect(content).toContain("TEST_CONSTANT");
    expect(content).toContain("TestEnum");
    expect(content).toContain("#endregion Constants");
  });

  test("file.py exists and has valid header", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.py");
    expect(existsSync(filePath)).toBe(true);
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("# region Header");
    expect(content).toContain("file.py");
    expect(content).toContain("GNU Affero General Public License");
    expect(content).toContain("# endregion Header");
  });

  test("file.cs exists and has header region", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file.cs");
    expect(existsSync(filePath)).toBe(true);
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Header");
    expect(content).toContain("#endregion");
  });

  test("file_invalid.tsx has no header region", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file_invalid.tsx");
    expect(existsSync(filePath)).toBe(true);
    const content = readFileSync(filePath, "utf-8");
    expect(content).not.toContain("#region Header");
  });

  test("file_empty_region.tsx has empty region", () => {
    const filePath = join(ASSETS_REPO_DIR, "some", "folder", "file_empty_region.tsx");
    expect(existsSync(filePath)).toBe(true);
    const content = readFileSync(filePath, "utf-8");
    expect(content).toContain("#region Empty");
    expect(content).toContain("#endregion Empty");
  });
});

// #endregion Test Fixtures Tests

// #endregion
