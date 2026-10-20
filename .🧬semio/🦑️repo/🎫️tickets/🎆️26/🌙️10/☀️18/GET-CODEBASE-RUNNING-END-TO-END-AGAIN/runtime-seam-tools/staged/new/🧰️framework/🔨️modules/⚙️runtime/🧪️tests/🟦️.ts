import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { RuntimeCapabilityMissingV1, type RuntimeAdapterV1, type RuntimeFamilyV1, type RuntimeIdV1 } from "../📋️contract/🟦️.ts";
import { checkRuntimeLawV1, listRuntimeLawFilesV1, scanRuntimeLawSourceV1 } from "../⚖️law/🟦️.ts";
import ledger from "../⚖️law/📒️.json" with { type: "json" };
import { GlobV1, isMainModuleV1, runtimeIdentityV1, runtimeKeyV1, runtimeSupportV1, selectRuntimeAdapterV1, RUNTIME_REGISTRY_V1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const repositoryRoot = process.env.SEMIO_REPOSITORY_ROOT ?? resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
const oracle = createRequire(join(repositoryRoot, "package.json"));
const Ajv = oracle("ajv") as typeof import("ajv").default;
const picomatch = oracle("picomatch") as (pattern: string, options?: object) => (path: string) => boolean;
const iarna = oracle("@iarna/toml") as { parse(text: string): unknown };
const jsoncParser = oracle("jsonc-parser") as { parse(text: string, errors?: unknown[], options?: object): unknown };
const semverOracle = oracle("semver") as { satisfies(version: string, range: string): boolean; compare(left: string, right: string): number };
const MEMBERS: Readonly<Record<RuntimeFamilyV1, readonly (keyof RuntimeAdapterV1)[]>> = { glob: ["glob"], toml: ["toml"], jsonc: ["jsonc"], yaml: ["yaml"], which: ["which"], resolve: ["resolve"], sleep: ["sleep"], spawn: ["spawn", "spawnSync"], stdin: ["stdinText", "stdinStream"], file: ["file"], serve: ["serve"], sqlite: ["sqlite"], ffi: ["ffiOpen", "ffiRead", "ffiPointer", "ffiCallback"], transpile: ["transpiler"], bundle: ["bundle"], semver: ["semverSatisfies", "semverOrder"], identity: ["identity"], main: ["main"], gc: ["gc"] };
const ids = RUNTIME_REGISTRY_V1.runtimes.map(row => row.id as RuntimeIdV1);
const adapters = ids.map(id => selectRuntimeAdapterV1(id));
const normalize = (value: unknown): string => JSON.stringify(value, (_key, item) => (typeof item === "bigint" ? `${item}n` : item instanceof Date ? `D:${item.toISOString()}` : item));
const temporary = (): string => mkdtempSync(join(tmpdir(), "runtime-seam-"));

test("the registry validates against its schema and every adapter provides exactly the families it declares", () => {
  const check = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/definitions/Registry" });
  expect(check(schema.definitions.Registry.const), JSON.stringify(check.errors)).toBe(true);
  for (const adapter of adapters) for (const family of RUNTIME_REGISTRY_V1.families as RuntimeFamilyV1[]) {
    const provided = MEMBERS[family].every(name => adapter[name] !== undefined), declared = runtimeSupportV1(family, adapter.id) !== "missing";
    expect(provided, `${adapter.id} ${family}`).toBe(declared);
  }
  expect(RUNTIME_REGISTRY_V1.runtimes.map(row => row.id)).toEqual([...ids]);
});

test("the hosting runtime identifies itself and the facade refuses families the adapter lacks", () => {
  expect(runtimeIdentityV1().key).toBe(runtimeKeyV1());
  expect(runtimeKeyV1().startsWith(`${runtimeIdentityV1().id}:`)).toBe(true);
  const node = selectRuntimeAdapterV1("node");
  expect(node.ffiOpen).toBeUndefined();
  expect(runtimeSupportV1("ffi", "node")).toBe("missing");
  expect(new RuntimeCapabilityMissingV1("ffi", "node").message).toContain("ffi");
});

test("glob matching agrees between every adapter, the corpus and the picomatch oracle", () => {
  for (const [pattern, path, expected] of fixture.glob as [string, string, boolean][]) {
    for (const adapter of adapters) expect(adapter.glob!(pattern).match(path), `${adapter.id} ${pattern} ${path}`).toBe(expected);
    expect(new GlobV1(pattern).match(path), `facade ${pattern} ${path}`).toBe(expected);
  }
  for (const [pattern, path, expected, checked] of fixture.glob as [string, string, boolean, boolean?][]) if (checked === true) expect(picomatch(pattern, { dot: true })(path), `picomatch ${pattern} ${path}`).toBe(expected);
});

test("glob scanning lists the same entries on every adapter for a tree with dot entries, symlinks and nesting", async () => {
  const root = temporary();
  try {
    for (const directory of ["a/b", ".hid/x", "a/.dot", "node_modules/m", "ln"]) mkdirSync(join(root, directory), { recursive: true });
    for (const file of ["a/f.ts", "a/b/g.ts", ".hid/x/h.ts", "a/.dot/i.ts", "a/.j.ts", "top.ts", "node_modules/m/k.ts"]) writeFileSync(join(root, file), "");
    symlinkSync("../a", join(root, "ln/link"));
    symlinkSync("nowhere", join(root, "broken"));
    const patterns = ["**/*.ts", "*.ts", "a/*", "a/**", "**", "*", "{a,.hid}/**/*.ts", ".hid/**", "a/f.ts", "a", "ln/**", "*/*.ts", "**/b/*", "a/?.ts", "**/*.{ts,js}"];
    const optionSets = [{}, { dot: true }, { onlyFiles: false }, { onlyFiles: false, dot: true }, { followSymlinks: true }, { absolute: true }];
    for (const pattern of patterns) for (const options of optionSets) {
      const results = new Map<string, string[]>();
      for (const adapter of adapters) {
        const glob = adapter.glob!(pattern), sync = [...glob.scanSync({ cwd: root, ...options })].sort(), asynchronous: string[] = [];
        for await (const row of glob.scan({ cwd: root, ...options })) asynchronous.push(row);
        expect(asynchronous.sort(), `${adapter.id} async ${pattern} ${JSON.stringify(options)}`).toEqual(sync);
        results.set(adapter.id, sync);
      }
      const [first, ...rest] = [...results.values()];
      for (const other of rest) expect(other, `${pattern} ${JSON.stringify(options)}`).toEqual(first!);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("TOML corpus parses identically to the oracle on the portable parser and agrees with the native parser except for its known defects", () => {
  const portable = selectRuntimeAdapterV1("node").toml!, native = selectRuntimeAdapterV1("bun").toml!;
  for (const [id, text, expected, nativeDefect] of fixture.tomlValid as [string, string, unknown, boolean?][]) {
    expect(normalize(portable(text)), id).toBe(normalize(expected));
    expect(normalize(iarna.parse(text)), `oracle ${id}`).toBe(normalize(expected));
    if (nativeDefect !== true) expect(normalize(native(text)), `native ${id}`).toBe(normalize(expected));
  }
  for (const [id, text] of fixture.tomlInvalid as [string, string][]) {
    expect(() => portable(text), id).toThrow();
    expect(() => iarna.parse(text), `oracle ${id}`).toThrow();
  }
});

test("TOML parses every manifest and lock file of the repository like the oracle", () => {
  const listed = spawnSync("git", ["ls-files", "-z", "--", "*.toml", "*Cargo.lock"], { cwd: repositoryRoot, encoding: "utf8", maxBuffer: 1 << 28 });
  const files = listed.stdout.split("\0").filter(Boolean).slice(0, 400), portable = selectRuntimeAdapterV1("node").toml!;
  let compared = 0;
  for (const file of files) {
    let text: string;
    try { text = require("node:fs").readFileSync(join(repositoryRoot, file), "utf8"); } catch { continue; }
    let expected: unknown;
    try { expected = iarna.parse(text); } catch { expect(() => portable(text), file).toThrow(); continue; }
    expect(normalize(portable(text)), file).toBe(normalize(expected)); compared++;
  }
  expect(compared).toBeGreaterThan(50);
});

test("JSONC corpus agrees with the oracle on every adapter", () => {
  for (const [id, text, expected] of fixture.jsonc as [string, string, unknown][]) {
    expect(normalize(jsoncParser.parse(text, [], { allowTrailingComma: true })), `oracle ${id}`).toBe(normalize(expected));
    for (const adapter of adapters) expect(normalize(adapter.jsonc!(text)), `${adapter.id} ${id}`).toBe(normalize(expected));
  }
});

test("semantic version ranges and ordering agree with the semver oracle on every adapter", () => {
  for (const adapter of adapters) {
    for (const version of fixture.semverVersions) for (const range of fixture.semverRanges) {
      let expected = false;
      try { expected = semverOracle.satisfies(version, range); } catch { expected = false; }
      expect(adapter.semverSatisfies!(version, range), `${adapter.id} ${version} ${range}`).toBe(expected);
    }
    for (const left of fixture.semverVersions) for (const right of fixture.semverVersions) expect(adapter.semverOrder!(left, right), `${adapter.id} ${left} ${right}`).toBe(semverOracle.compare(left, right) as -1 | 0 | 1);
  }
});

test("executable lookup finds the same program on every adapter and nothing for an unknown name", () => {
  for (const adapter of adapters) {
    expect(adapter.which!("git"), adapter.id).toBe(selectRuntimeAdapterV1("bun").which!("git"));
    expect(adapter.which!("no-such-program-semio"), adapter.id).toBeNull();
    expect(adapter.which!("git", { PATH: "/nonexistent" }), adapter.id).toBeNull();
  }
});

test("module resolution agrees across adapters for packages, subpaths and relative files", () => {
  for (const [specifier, from] of [["typescript/bin/tsc", repositoryRoot], ["vitest/package.json", repositoryRoot], ["ajv", repositoryRoot], ["./🟦️.ts", resolve(dirname(fileURLToPath(import.meta.url)), "..")]] as const) {
    const results = adapters.map(adapter => adapter.resolve!(specifier, from));
    expect(new Set(results).size, `${specifier} ${JSON.stringify(results)}`).toBe(1);
  }
  for (const adapter of adapters) expect(() => adapter.resolve!("no-such-package-semio", repositoryRoot)).toThrow();
});

test("child processes report output, exit codes, timeouts and signals the same on every adapter", async () => {
  for (const adapter of adapters) {
    const sync = adapter.spawnSync!([process.execPath, "-e", "process.stdout.write('out');process.stderr.write('err');process.exit(3)"]);
    expect([sync.exitCode, sync.stdout.toString(), sync.stderr.toString(), sync.success]).toEqual([3, "out", "err", false]);
    const child = adapter.spawn!([process.execPath, "-e", "process.stdout.write(require('fs').readFileSync(0,'utf8').toUpperCase())"], { stdin: "abc", stderr: "ignore" });
    expect(await new Response(child.stdout).text()).toBe("ABC");
    expect(await child.exited).toBe(0);
    const slow = adapter.spawn!([process.execPath, "-e", "setTimeout(()=>{},60000)"], { timeout: 200, stderr: "ignore" });
    expect(await slow.exited).not.toBe(0);
    expect(() => adapter.spawnSync!(["no-such-program-semio"])).toThrow();
  }
});

test("HTTP serving round-trips methods, headers, bodies, file bodies and the body limit on every adapter", async () => {
  const root = temporary();
  try {
    writeFileSync(join(root, "data.txt"), "file-body");
    for (const adapter of adapters) {
      const server = await adapter.serve!({ hostname: "127.0.0.1", port: 0, maximumRequestBodyBytes: 64, fetch: async request => {
        const url = new URL(request.url);
        if (url.pathname === "/file") return new Response(adapter.file!(join(root, "data.txt")).body(), { headers: { "x-kind": "file" } });
        if (url.pathname === "/missing") return new Response("Not Found", { status: 404 });
        return new Response(JSON.stringify({ method: request.method, body: request.method === "POST" ? await request.text() : null, header: request.headers.get("x-probe") }), { headers: { "content-type": "application/json" } });
      } });
      try {
        const base = `http://127.0.0.1:${server.port}`;
        expect(await (await fetch(`${base}/echo`, { headers: { "x-probe": "1" } })).json(), adapter.id).toEqual({ method: "GET", body: null, header: "1" });
        expect(await (await fetch(`${base}/echo`, { method: "POST", body: "payload" })).json(), adapter.id).toEqual({ method: "POST", body: "payload", header: null });
        const file = await fetch(`${base}/file`);
        expect([await file.text(), file.headers.get("x-kind")], adapter.id).toEqual(["file-body", "file"]);
        expect((await fetch(`${base}/missing`)).status, adapter.id).toBe(404);
      } finally { await server.stop(true); }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("SQLite connections give the same answers on every adapter, including null rows, value arrays and transactions", () => {
  for (const adapter of adapters) {
    let Database: ReturnType<NonNullable<RuntimeAdapterV1["sqlite"]>>;
    try { Database = adapter.sqlite!(); } catch { expect(adapter.id).toBe("node"); continue; }
    const database = new Database(":memory:");
    try {
      database.exec("create table t(id integer primary key, name text)");
      database.run("insert into t(name) values (?)", "a");
      const insert = database.query("insert into t(name) values (?)");
      insert.run("b");
      expect(database.query("select count(*) as c from t").get(), adapter.id).toEqual({ c: 2 });
      expect(database.query("select name from t order by id").all(), adapter.id).toEqual([{ name: "a" }, { name: "b" }]);
      expect(database.query("select id, name from t order by id").values(), adapter.id).toEqual([[1, "a"], [2, "b"]]);
      expect(database.query("select name from t where id = ?").get(99), adapter.id).toBeNull();
      const write = database.transaction((name: string) => { database.run("insert into t(name) values (?)", name); throw new Error("rollback"); });
      expect(() => write("c")).toThrow("rollback");
      expect(database.query("select count(*) as c from t").get(), adapter.id).toEqual({ c: 2 });
    } finally { database.close(); }
  }
});

test("the entry-point test is true for the launched module only, through a symlink too, and false for an importer", () => {
  const root = temporary();
  try {
    const facade = resolve(dirname(fileURLToPath(import.meta.url)), "..", "🟦️.ts");
    writeFileSync(join(root, "probe.ts"), `import { isMainModuleV1 } from ${JSON.stringify(facade)};\nconsole.log(JSON.stringify(isMainModuleV1(import.meta.url)));\n`);
    writeFileSync(join(root, "importer.ts"), `import "./probe.ts";\n`);
    symlinkSync(join(root, "probe.ts"), join(root, "link.ts"));
    const run = (file: string): string => spawnSync(process.execPath, [join(root, file)], { encoding: "utf8" }).stdout.trim();
    expect(run("probe.ts")).toBe("true");
    expect(run("link.ts")).toBe("true");
    expect(run("importer.ts")).toBe("false");
    expect(typeof isMainModuleV1(import.meta.url)).toBe("boolean");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("the native binding family loads a system library, calls it and reads memory on Bun and is refused elsewhere", () => {
  const bun = selectRuntimeAdapterV1("bun"), library = bun.ffiOpen!(process.platform === "darwin" ? "/usr/lib/libSystem.B.dylib" : "libc.so.6", { getpid: { args: [], returns: "i32" }, [process.platform === "darwin" ? "__error" : "__errno_location"]: { args: [], returns: "ptr" } });
  try {
    expect(library.symbols.getpid!()).toBe(process.pid);
    const pointer = library.symbols[process.platform === "darwin" ? "__error" : "__errno_location"]!();
    expect(typeof bun.ffiRead!().i32(pointer)).toBe("number");
    expect(typeof bun.ffiPointer!(Buffer.alloc(8))).toBe("number");
  } finally { library.close(); }
  expect(() => selectRuntimeAdapterV1("node").ffiOpen!("x", {})).toThrow();
});

test("the transpiler scans imports, removes dead in-source test blocks and the bundler builds an entry on Bun", async () => {
  const bun = selectRuntimeAdapterV1("bun"), transpiler = bun.transpiler!({ loader: "ts", define: { "import.meta.vitest": "undefined" }, deadCodeElimination: true });
  expect(transpiler.scanImports(`import a from "./a.ts"; const b = require("node:fs"); await import("./c.ts");`).map(row => [row.path, row.kind])).toEqual([["./a.ts", "import-statement"], ["node:fs", "require-call"], ["./c.ts", "dynamic-import"]]);
  expect(transpiler.scan(`export const x = 1; export function y() {}`).exports.sort()).toEqual(["x", "y"]);
  expect(transpiler.transformSync(`export const a: number = 1; if (import.meta.vitest) { console.log("dead"); }`)).not.toContain("dead");
  const root = temporary();
  try {
    writeFileSync(join(root, "entry.ts"), `export const answer: number = 42;\n`);
    const result = await bun.bundle!({ entrypoints: [join(root, "entry.ts")], target: "browser", format: "esm" });
    expect(result.success).toBe(true);
    expect(await result.outputs[0]!.text()).toContain("42");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("the portable stack runs under real Node and reports the node adapter", () => {
  const probe = join(dirname(fileURLToPath(import.meta.url)), "🟢️node/🟦️.ts");
  const node = spawnSync("node", [probe], { encoding: "utf8" });
  if (node.error !== undefined) return;
  expect(node.status, node.stderr).toBe(0);
  const report = JSON.parse(node.stdout) as { runtime: string; failures: string[] };
  expect(report.runtime).toBe("node");
  expect(report.failures).toEqual([]);
});

test("the law finds runtime names in code, specifiers, templates and meta members and ignores comments, strings and regexes", () => {
  const rules = (source: string): string[] => scanRuntimeLawSourceV1(source).map(row => row.rule);
  expect(rules(`const a = Bun.file("x");`)).toEqual(["Bun"]);
  expect(rules(`if (typeof Bun === "undefined") {}`)).toEqual(["Bun"]);
  expect(rules(`const v = process.versions.bun;`)).toEqual(["process.versions.bun"]);
  expect(rules(`const d = import.meta.dir + import.meta.main;`)).toEqual(["import.meta", "import.meta"]);
  expect(rules(`const m = await import("bun:sqlite");`)).toEqual(["specifier"]);
  expect(rules("const p = String.raw`const x = Bun.stdin;`;")).toEqual(["embedded"]);
  expect(rules(`// Bun.file is mentioned\n/* import.meta.main */ const s = "uses Bun.file in text"; const r = /Bun\\.file/u;`)).toEqual([]);
  expect(rules(`const ok = import.meta.dirname + import.meta.filename + import.meta.url;`)).toEqual([]);
  const report = checkRuntimeLawV1([{ path: "a/b.ts", source: "Bun.sleep(1);" }, { path: "🧰️framework/🔨️modules/⚙️runtime/🍞️bun/🟦️.ts", source: "Bun.sleep(1);" }], []);
  expect(report.violations.length).toBe(1);
  expect(checkRuntimeLawV1([{ path: "a/b.ts", source: "Bun.sleep(1);" }], [{ path: "a/b.ts", rule: "Bun", count: 1, class: "x", owner: "o", reason: "r" }]).violations).toEqual([]);
  expect(checkRuntimeLawV1([{ path: "a/b.ts", source: "const x = 1;" }], [{ path: "a/b.ts", rule: "Bun", count: 1, class: "x", owner: "o", reason: "r" }]).stale.length).toBe(1);
});

test("every runtime the launcher registry can provision is a runtime this registry can host, with the same executable name", () => {
  const launcher = join(repositoryRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json");
  if (!existsSync(launcher)) return;
  const declared = JSON.parse(readFileSync(launcher, "utf8")).runtimes as Record<string, { executable: { unix: string } }>;
  for (const [id, row] of Object.entries(declared)) {
    const hosted = RUNTIME_REGISTRY_V1.runtimes.find(candidate => candidate.id === id);
    expect(hosted, `launcher runtime ${id} is hostable`).toBeDefined();
    expect(hosted!.executable, id).toBe(row.executable.unix);
  }
});

test("no production source of the framework names a specific runtime outside the adapter and the ledger", () => {
  const probe = spawnSync("git", ["rev-parse", "--is-inside-work-tree"], { cwd: repositoryRoot, encoding: "utf8" });
  if (probe.status !== 0) return;
  const files = listRuntimeLawFilesV1(repositoryRoot, "🧰️framework").map(path => ({ path, source: readFileSync(join(repositoryRoot, path), "utf8") }));
  const report = checkRuntimeLawV1(files, ledger.rows as never);
  expect(report.violations.slice(0, 20)).toEqual([]);
  expect(report.stale.slice(0, 20)).toEqual([]);
  expect(files.length).toBeGreaterThan(1000);
}, 120000);
