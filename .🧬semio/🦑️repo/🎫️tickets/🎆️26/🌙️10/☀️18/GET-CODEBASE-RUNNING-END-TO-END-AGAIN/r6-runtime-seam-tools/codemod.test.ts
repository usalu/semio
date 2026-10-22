import { expect, test } from "bun:test";
import { transformSource } from "./codemod.ts";

const FROM = "../runtime/🟦️.ts";
const run = (source: string): string => transformSource("x.ts", source, () => FROM).text;

test("calls on the Bun global become facade calls and gain one merged import", () => {
  const out = run(`const a = Bun.TOML.parse(t); const b = new Bun.Glob("*.ts"); const c = Bun.which("git", { PATH: p });\n`);
  expect(out).toContain("runtimeParseTomlV1(t)");
  expect(out).toContain('new GlobV1("*.ts")');
  expect(out).toContain('runtimeWhichV1("git", { PATH: p })');
  expect(out).toContain(`import { GlobV1, runtimeParseTomlV1, runtimeWhichV1 } from "${FROM}";`);
  expect(out).not.toContain("Bun.");
});

test("globalThis casts reach the same rewrite", () => {
  expect(run(`(globalThis as any).Bun.spawnSync(["a"], {});\n`)).toContain('runtimeSpawnSyncV1(["a"], {})');
});

test("import.meta members: dir and path become dirname and filename, main becomes the entry-point test", () => {
  const out = run(`const d = import.meta.dir, p = import.meta.path; if (import.meta.main) go();\n`);
  expect(out).toContain("import.meta.dirname");
  expect(out).toContain("import.meta.filename");
  expect(out).toContain("if (isMainModuleV1(import.meta.url)) go();");
});

test("builtin prefix tests and sqlite imports go through the owned names", () => {
  expect(run(`if (s.startsWith("bun:")) skip();\n`)).toContain("isRuntimeBuiltinSpecifierV1(s)");
  expect(run(`const { Database } = await import("bun:sqlite");\n`)).toContain("const { Database } = { Database: SqliteDatabaseV1 };");
  expect(run(`import { Database } from "bun:sqlite";\n`)).toContain(`import { SqliteDatabaseV1 as Database } from "${FROM}";`);
  expect(run(`type T = typeof import("bun:sqlite").Database;\n`)).toContain("type T = typeof SqliteDatabaseV1;");
});

test("tool commands: a script run keeps working through the executable name, a subcommand stays for its owner", () => {
  expect(run(`runCmd("bun", [join(root, "tsc"), "--noEmit", "--types", "bun"]);\n`)).toContain('runCmd(runtimeExecutableV1(), [join(root, "tsc"), "--noEmit", "--types", runtimeTypeEnvironmentV1()]);');
  expect(run(`runCmd("bun", ["test", file]);\n`)).toContain("runCmd(runtimeExecutableV1(), runtimeTestArgumentsV1([file]));");
  const kept = transformSource("x.ts", `runCmd("bun", ["install", file]);\n`, () => FROM);
  expect(kept.text).toContain('runCmd("bun", ["install", file]);');
  expect(kept.manual.some(row => row.rule === "specifier")).toBe(true);
});

test("strings, comments and a second run change nothing", () => {
  const source = `// Bun.file is mentioned\nconst s = "uses Bun.file here";\nconst r = /Bun/u;\n`;
  expect(run(source)).toBe(source);
  const once = run(`const a = Bun.sleep(1);\n`);
  expect(run(once)).toBe(once);
});
