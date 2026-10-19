import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { canonicalGoPlan } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";

const library = "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, library, "🧪️tests/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json"), "utf8"));
const twin = join(import.meta.dir, "plan.py");

const put = (file: string, text: string): void => { mkdirSync(join(file, ".."), { recursive: true }); writeFileSync(file, text); };

function materialize(root: string): void {
  put(join(root, "go.mod"), `module ${vector.module}\n\ngo 1.24\n\nrequire ${vector.dependency.module} v0.0.0\n\nreplace ${vector.dependency.module} => ./${vector.dependency.directory}\n`);
  const dependencyRoot = join(root, vector.dependency.directory);
  put(join(dependencyRoot, "go.mod"), `module ${vector.dependency.module}\n\ngo 1.24\n`);
  put(join(dependencyRoot, vector.dependency.domain, "🐹️.go"), `package ${vector.dependency.package}\n${vector.dependency.production}\n`);
  for (const row of vector.nativePackages) put(join(root, row.owner, "🐹️.go"), `package ${row.package}\n${row.production}\n`);
  for (const row of vector.oraclePrograms) put(join(root, row.directory, "🐹️.go"), `package main\n${row.production}\n`);
  for (const row of vector.hostAdapterCases) put(join(root, row.owner, "🧪️tests", row.case, "🐹️.go"), `package adapter\n${row.test}\n`);
  for (const row of [...vector.cases, ...vector.opaqueCases]) {
    const owner = row.owner === "." ? root : join(root, row.owner);
    const imports = row.productionImports?.length ? `import (${row.productionImports.map((name: string) => `"${name}"`).join("\n")})\n` : "";
    put(join(owner, row.domain, "🐹️.go"), `package ${row.package}\n${imports}${row.production}\n`);
    put(join(owner, "🧪️tests", row.case, "🐹️.go"), `package ${row.package}\nimport "testing"\n${row.test}\n`);
  }
}

function materializePackageModule(root: string): string {
  const row = vector.packageModule, owner = join(root, row.owner), module = join(owner, "📦️packages", "🐹️go");
  put(join(module, "go.mod"), `module ${row.module}\n\ngo 1.24\n`);
  put(join(module, "🐹️.go"), `package ${row.package}\n${row.production}\n`);
  for (const member of row.members) put(join(module, member.domain, "🐹️.go"), `package ${row.package}\n${member.production}\n`);
  const nested = join(module, row.nestedModule.domain);
  put(join(nested, "go.mod"), `module ${row.nestedModule.module}\n\ngo 1.24\n`);
  put(join(nested, "🐹️.go"), `package ${row.nestedModule.package}\n${row.nestedModule.production}\n`);
  for (const member of row.nestedModule.members) put(join(nested, member.domain, "🐹️.go"), `package ${row.nestedModule.package}\n${member.production}\n`);
  for (const testCase of row.cases) put(join(owner, "🧪️tests", testCase.case, "🐹️.go"), `package ${testCase.package}\nimport (${["testing", ...(testCase.imports ?? [])].map((name: string) => `"${name}"`).join("\n")})\n${testCase.test}\n`);
  return module;
}

function both(module: string, directory: string, layout?: string): void {
  const ts = canonicalGoPlan(module, layout ? vector.layout : undefined);
  const overlay = join(directory, "py-overlay.json");
  const args = ["-I", twin, module, overlay, ...(layout ? [layout] : [])];
  const py = spawnSync("python3", args, { encoding: "utf8" });
  expect(py.status, py.stderr).toBe(0);
  const twinReplace = JSON.parse(readFileSync(overlay, "utf8")).Replace as Record<string, string>;
  expect(twinReplace).toEqual({ ...ts.sourceReplacements, ...ts.testReplacements });
  expect(py.stdout.split("\n").filter(Boolean).sort()).toEqual([...ts.packages].sort());
}

test("TS and Python planners agree on the shared discovery vector, host adapters excluded", () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "semio-go-parity-")));
  try {
    const layout = join(root, "layout.json");
    const work = join(root, "m");
    mkdirSync(work, { recursive: true });
    writeFileSync(layout, JSON.stringify(vector.layout));
    materialize(work);
    both(work, root, layout);
    const plan = canonicalGoPlan(work, vector.layout);
    expect(Object.values(plan.replacements).some((path) => vector.hostAdapterCases.some((row: { case: string }) => path.includes(row.case)))).toBe(false);
    const packaged = join(root, "p");
    both(materializePackageModule(packaged), root, layout);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("TS and Python planners agree on every real module with host adapters", () => {
  const server = join(import.meta.dir, "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator");
  const scratch = realpathSync(mkdtempSync(join(tmpdir(), "semio-go-parity-real-")));
  try {
    both(realpathSync(server), scratch);
    both(realpathSync(join(import.meta.dir, library)), scratch);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});
