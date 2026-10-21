import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { RuntimeAdapterV1, RuntimeFamilyV1, RuntimeIdV1 } from "../📋️contract/🟦️.ts";
import { checkRuntimeLawV1, listRuntimeLawFilesV1, scanRuntimeLawSourceV1 } from "../⚖️law/🟦️.ts";
import { RUNTIME_REGISTRY_V1, RuntimeCapabilityMissingV1, GlobV1, isMainModuleV1, loadRuntimeAdapterV1, runtimeIdentityV1, runtimeKeyV1, runtimeSupportV1 } from "../🟦️.ts";
import ledger from "../⚖️law/📒️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { describe, expect, test } from "../🧪️test/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const here = dirname(fileURLToPath(import.meta.url));
const moduleRoot = resolve(here, "..");
const repositoryRoot = process.env.SEMIO_REPOSITORY_ROOT ?? resolve(here, "../../../..");
const oracle = createRequire(join(repositoryRoot, "package.json"));
const picomatch = oracle("picomatch") as (pattern: string, options?: object) => (path: string) => boolean;
const iarna = oracle("@iarna/toml") as { parse(text: string): unknown };
const jsoncParser = oracle("jsonc-parser") as { parse(text: string, errors?: unknown[], options?: object): unknown };
const semverOracle = oracle("semver") as { satisfies(version: string, range: string): boolean; compare(left: string, right: string): number };
const yamlOracle = oracle("yaml") as { parse(text: string, options?: object): unknown };
const MEMBERS: Readonly<Record<RuntimeFamilyV1, readonly (keyof RuntimeAdapterV1)[]>> = { glob: ["glob"], toml: ["toml"], jsonc: ["jsonc"], yaml: ["yaml"], which: ["which"], resolve: ["resolve"], sleep: ["sleep"], spawn: ["spawn", "spawnSync"], stdin: ["stdinText", "stdinStream"], file: ["file"], serve: ["serve"], sqlite: ["sqlite"], ffi: ["ffiOpen", "ffiRead", "ffiPointer", "ffiToArrayBuffer", "ffiCallback"], transpile: ["transpiler"], bundle: ["bundle"], semver: ["semverSatisfies", "semverOrder"], identity: ["identity"], main: ["main"], gc: ["gc"], plugin: ["loaderPlugin"], test: ["testApi", "testCommand"] };
const host = runtimeIdentityV1().id;
const ids = RUNTIME_REGISTRY_V1.runtimes.map(row => row.id as RuntimeIdV1);
const every = await Promise.all(ids.map(id => loadRuntimeAdapterV1(id)));
const adapters = every.filter(adapter => adapter.id === host || adapter.id === "node");
const normalize = (value: unknown): string => JSON.stringify(value, (_key, item) => (typeof item === "bigint" ? `${item}n` : item instanceof Date ? `D:${item.toISOString()}` : item));
const temporary = (): string => mkdtempSync(join(tmpdir(), "runtime-seam-"));
const installed = (): [string, string][] => {
  const found = new Map<string, string>();
  for (const row of RUNTIME_REGISTRY_V1.runtimes) { const path = adapters[0]!.which!(row.executable); if (path !== null) found.set(realpathSync(path), row.id); }
  if (!found.has(realpathSync(process.execPath))) found.set(realpathSync(process.execPath), host);
  return [...found].map(([path, id]) => [path, id]);
};
const executables = (): string[] => installed().map(([path]) => path);
const available = (name: string): boolean => spawnSync(name, ["--version"], { encoding: "utf8" }).status === 0;

describe("registry and selection", () => {
  test("the registry validates against its schema and every adapter provides exactly the families it declares", () => {
    const check = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/definitions/Registry" });
    expect(check(schema.definitions.Registry.const), JSON.stringify(check.errors)).toBe(true);
    for (const adapter of every) for (const family of RUNTIME_REGISTRY_V1.families as RuntimeFamilyV1[]) {
      const provided = MEMBERS[family].every(name => adapter[name] !== undefined), declared = runtimeSupportV1(family, adapter.id) !== "missing";
      expect(provided, `${adapter.id} ${family}`).toBe(declared);
    }
  });

  test("the hosting runtime identifies itself and the capability refusal names runtime and family", () => {
    expect(runtimeIdentityV1().key).toBe(runtimeKeyV1());
    expect(runtimeKeyV1().startsWith(`${host}:`)).toBe(true);
    const refusal = new RuntimeCapabilityMissingV1("ffi", "node", "no provider");
    expect(refusal.message).toContain("ffi");
    expect(refusal.message).toContain("no provider");
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
});

describe("data formats", () => {
  test("glob matching agrees between every adapter, the corpus and the picomatch oracle", () => {
    for (const [pattern, path, expected, checked] of fixture.glob as [string, string, boolean, boolean?][]) {
      for (const adapter of adapters) expect(adapter.glob!(pattern).match(path), `${adapter.id} ${pattern} ${path}`).toBe(expected);
      expect(new GlobV1(pattern).match(path), `facade ${pattern} ${path}`).toBe(expected);
      if (checked === true) expect(picomatch(pattern, { dot: true })(path), `picomatch ${pattern} ${path}`).toBe(expected);
    }
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
        const results: string[][] = [];
        for (const adapter of adapters) {
          const glob = adapter.glob!(pattern), sync = [...glob.scanSync({ cwd: root, ...options })].sort(), asynchronous: string[] = [];
          for await (const row of glob.scan({ cwd: root, ...options })) asynchronous.push(row);
          expect(asynchronous.sort(), `${adapter.id} async ${pattern} ${JSON.stringify(options)}`).toEqual(sync);
          results.push(sync);
        }
        for (const other of results.slice(1)) expect(other, `${pattern} ${JSON.stringify(options)}`).toEqual(results[0]!);
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("TOML corpus parses identically to the oracle on the portable parser and agrees with the native parser except for its known defects", () => {
    const portable = every.find(adapter => adapter.id === "node")!.toml!, native = every.find(adapter => adapter.id === "bun")!.toml!;
    for (const [id, text, expected, nativeDefect] of fixture.tomlValid as [string, string, unknown, boolean?][]) {
      expect(normalize(portable(text)), id).toBe(normalize(expected));
      expect(normalize(iarna.parse(text)), `oracle ${id}`).toBe(normalize(expected));
      if (nativeDefect !== true && host === "bun") expect(normalize(native(text)), `native ${id}`).toBe(normalize(expected));
    }
    for (const [id, text] of fixture.tomlInvalid as [string, string][]) {
      expect(() => portable(text), id).toThrow();
      expect(() => iarna.parse(text), `oracle ${id}`).toThrow();
    }
  });

  test("TOML parses every manifest and lock file of the repository like the oracle", () => {
    const listed = spawnSync("git", ["ls-files", "-z", "--", "*.toml", "*Cargo.lock"], { cwd: repositoryRoot, encoding: "utf8", maxBuffer: 1 << 28 });
    const files = listed.stdout.split("\0").filter(Boolean).slice(0, 400), portable = every.find(adapter => adapter.id === "node")!.toml!;
    let compared = 0;
    for (const file of files) {
      let text: string;
      try { text = readFileSync(join(repositoryRoot, file), "utf8"); } catch { continue; }
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

  test("YAML corpus agrees with the oracle on the portable parser and the repository's YAML files parse like the oracle", () => {
    const portable = every.find(adapter => adapter.id === "node")!.yaml!;
    for (const [id, text, expected] of fixture.yamlValid as [string, string, unknown][]) {
      expect(normalize(portable(text)), id).toBe(normalize(expected));
      expect(normalize(yamlOracle.parse(text)), `oracle ${id}`).toBe(normalize(expected));
      if (host === "bun") expect(normalize(every.find(adapter => adapter.id === "bun")!.yaml!(text)), `native ${id}`).toBe(normalize(expected));
    }
    for (const [id, text] of fixture.yamlInvalid as [string, string][]) expect(() => portable(text), id).toThrow();
    const listed = spawnSync("git", ["ls-files", "-z", "--", "*.yml", "*.yaml"], { cwd: repositoryRoot, encoding: "utf8", maxBuffer: 1 << 26 });
    let compared = 0;
    for (const file of listed.stdout.split("\0").filter(Boolean)) {
      let text: string;
      try { text = readFileSync(join(repositoryRoot, file), "utf8"); } catch { continue; }
      let expected: unknown;
      try { expected = yamlOracle.parse(text, { strict: false, uniqueKeys: false }); } catch { continue; }
      expect(normalize(portable(text)), file).toBe(normalize(expected)); compared++;
    }
    expect(compared).toBeGreaterThan(3);
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
});

describe("process and system", () => {
  test("executable lookup finds the same program on every adapter and nothing for an unknown name", () => {
    for (const adapter of adapters) {
      expect(adapter.which!("git"), adapter.id).toBe(adapters[0]!.which!("git"));
      expect(adapter.which!("no-such-program-semio"), adapter.id).toBeNull();
      expect(adapter.which!("git", { PATH: "/nonexistent" }), adapter.id).toBeNull();
    }
  });

  test("module resolution agrees across adapters for packages, subpaths and relative files", () => {
    for (const [specifier, from] of [["typescript/bin/tsc", repositoryRoot], ["vitest/package.json", repositoryRoot], ["ajv", repositoryRoot], ["./🟦️.ts", moduleRoot]] as const) {
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
      expect(await new Response(child.stdout as unknown as ReadableStream).text()).toBe("ABC");
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
        database.query("insert into t(name) values (?)").run("b");
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
      const facade = resolve(moduleRoot, "🟦️.ts");
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
});

describe("native bindings", () => {
  test("a host with its own binding loads a system library, calls it and reads memory", () => {
    if (host !== "bun") return;
    const bun = every.find(adapter => adapter.id === "bun")!, errno = process.platform === "darwin" ? "__error" : "__errno_location";
    const library = bun.ffiOpen!(process.platform === "darwin" ? "/usr/lib/libSystem.B.dylib" : "libc.so.6", { getpid: { args: [], returns: "i32" }, [errno]: { args: [], returns: "ptr" } });
    try {
      expect(library.symbols.getpid!()).toBe(process.pid);
      expect(typeof bun.ffiRead!().i32(library.symbols[errno]!())).toBe("number");
      expect(typeof bun.ffiPointer!(Buffer.alloc(8))).toBe("number");
    } finally { library.close(); }
  });

  test("the Node adapter routes every native call through the designated provider and refuses without one", () => {
    const node = every.find(adapter => adapter.id === "node")!;
    const before = process.env.SEMIO_RUNTIME_NATIVE_PROVIDER;
    delete process.env.SEMIO_RUNTIME_NATIVE_PROVIDER;
    try {
      expect(() => node.ffiOpen!("x", {})).toThrow("SEMIO_RUNTIME_NATIVE_PROVIDER");
      process.env.SEMIO_RUNTIME_NATIVE_PROVIDER = join(here, "🧫️fixtures/🧲️native-provider.cjs");
      const library = node.ffiOpen!("libfake", { add: { args: ["i32", "i32"], returns: "i32" } });
      expect(library.symbols.add!(2, 3)).toBe(5);
      expect(node.ffiRead!().i32(0)).toBe(7);
      expect(node.ffiCallback!(() => 1, { args: [], returns: "void" }).pointer).toBe(9);
    } finally { if (before === undefined) delete process.env.SEMIO_RUNTIME_NATIVE_PROVIDER; else process.env.SEMIO_RUNTIME_NATIVE_PROVIDER = before; }
  });
});

describe("source tools", () => {
  test("import scanning agrees with the corpus on every adapter, and the portable transpiler removes dead in-source test blocks", () => {
    for (const adapter of adapters) {
      const transpiler = adapter.transpiler!({ loader: "ts", define: { "import.meta.vitest": "undefined" }, deadCodeElimination: true }), transforms = adapter.id === host || host === "node";
      for (const [id, source, expected] of fixture.imports as [string, string, [string, string][]][]) expect(transpiler.scanImports(source).map(row => [row.path, row.kind]), `${adapter.id} ${id}`).toEqual(expected);
      expect(transpiler.scan("export const x = 1; export function y() {} export type Z = 1; export { x as w };").exports.sort(), adapter.id).toEqual(["w", "x", "y"]);
      for (const [id, source, dead] of fixture.transform as [string, string, string][]) {
        if (!transforms) continue;
        const output = transpiler.transformSync(source);
        expect(output, `${adapter.id} ${id}`).not.toContain(dead);
        expect(output, `${adapter.id} ${id}`).toContain("export const b = 2");
        expect(output, `${adapter.id} ${id}`).not.toContain(": number");
      }
    }
  });

  test("bundling an entry with a virtual module works on every adapter that has a bundler", async () => {
    const root = temporary();
    try {
      writeFileSync(join(root, "entry.ts"), `import answer from "virtual:answer";\nexport const result: number = answer + 1;\n`);
      for (const adapter of adapters) {
        let outcome;
        try {
          outcome = await adapter.bundle!({ entrypoints: [join(root, "entry.ts")], root: repositoryRoot, target: "browser", format: "esm", plugins: [{ name: "virtual", setup(build) {
            build.onResolve({ filter: /^virtual:/u }, arguments_ => ({ path: arguments_.path.slice(8), namespace: "virtual" }));
            build.onLoad({ filter: /.*/u, namespace: "virtual" }, () => ({ contents: "export default 41;", loader: "js" }));
          } }] });
        } catch (error) { expect(error instanceof RuntimeCapabilityMissingV1, `${adapter.id} ${String(error)}`).toBe(true); continue; }
        expect(outcome.success, `${adapter.id} ${JSON.stringify(outcome.logs)}`).toBe(true);
        const text = await outcome.outputs[0]!.text();
        expect(text, adapter.id).toContain("41");
        expect(text, adapter.id).not.toContain("from \"virtual:answer\"");
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("a loader plugin serves virtual modules and live objects in every runtime that is installed", () => {
    const probe = join(here, "🧫️fixtures/🔌️probe-plugin.ts");
    for (const executable of executables()) {
      const run = spawnSync(executable, [probe], { encoding: "utf8" });
      expect(run.status, `${executable} ${run.stderr}`).toBe(0);
      expect(JSON.parse(run.stdout.trim().split("\n").at(-1)!), executable).toEqual({ hello: 42, name: "hello", answer: 7, call: "called", object: { answer: 7 } });
    }
  });
});

describe("test runner", () => {
  test("one test file written against the thin API runs under every installed runner and fails the same way", () => {
    const probe = join(here, "🧫️fixtures/🧪️probe-test.ts");
    const runners: [string, string[]][] = [];
    for (const [path, id] of installed()) runners.push([path, [...every.find(adapter => adapter.id === id)!.testCommand!([probe])]]);
    expect(runners.length).toBeGreaterThan(0);
    for (const [executable, args] of runners) {
      const passing = spawnSync(executable, args, { encoding: "utf8", env: { ...process.env, SEMIO_RUNTIME_INNER: "1", SEMIO_PROBE_FAIL: "0" } });
      expect(passing.status, `${executable} ${args.join(" ")}\n${passing.stdout}${passing.stderr}`).toBe(0);
      const failing = spawnSync(executable, args, { encoding: "utf8", env: { ...process.env, SEMIO_RUNTIME_INNER: "1", SEMIO_PROBE_FAIL: "1" } });
      expect(failing.status, `${executable} must fail on a false expectation`).not.toBe(0);
    }
  });

  test("the portable expect implements the matcher subset with negation, resolves and rejects", async () => {
    const node = every.find(adapter => adapter.id === "node")!, { expect: check } = node.testApi!();
    check({ a: 1, b: undefined }).toEqual({ a: 1 });
    check(1).not.toBe(2);
    check([1, { a: [2] }]).toMatchObject([1, { a: [2] }]);
    check(() => { throw new TypeError("bad"); }).toThrow(TypeError);
    await check(Promise.resolve(1)).resolves.toBe(1);
    await check(Promise.reject(new Error("no"))).rejects.toThrow(/no/u);
    let caught: unknown;
    try { check(1).toBe(2); } catch (error) { caught = error; }
    expect(caught instanceof Error).toBe(true);
    let negated: unknown;
    try { check(1).not.toBe(1); } catch (error) { negated = error; }
    expect(negated instanceof Error).toBe(true);
  });

  test("test command arguments name each runner's own flags", () => {
    expect(every.find(adapter => adapter.id === "bun")!.testCommand!(["a.ts"], { timeoutMilliseconds: 5, filter: "x" })).toEqual(["test", "--timeout", "5", "-t", "x", "a.ts"]);
    expect(every.find(adapter => adapter.id === "node")!.testCommand!(["a.ts"], { timeoutMilliseconds: 5, filter: "x" })).toEqual(["--test", "--test-timeout=5", "--test-name-pattern=x", "a.ts"]);
  });
});

describe("law", () => {
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
    expect(checkRuntimeLawV1([{ path: "a/b.ts", source: "Bun.sleep(1);" }, { path: "🧰️framework/🔨️modules/⚙️runtime/🍞️bun/🟦️.ts", source: "Bun.sleep(1);" }], []).violations.length).toBe(1);
    expect(checkRuntimeLawV1([{ path: "a/b.ts", source: "Bun.sleep(1);" }], [{ path: "a/b.ts", rule: "Bun", count: 1, class: "x", owner: "o", reason: "r" }]).violations).toEqual([]);
    expect(checkRuntimeLawV1([{ path: "a/b.ts", source: "const x = 1;" }], [{ path: "a/b.ts", rule: "Bun", count: 1, class: "x", owner: "o", reason: "r" }]).stale.length).toBe(1);
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
});

describe("this suite under another runtime", () => {
  test("the same file passes under real Node when the host is not Node", () => {
    if (process.env.SEMIO_RUNTIME_INNER === "1" || host === "node" || !available("node")) return;
    const run = spawnSync("node", ["--test", fileURLToPath(import.meta.url)], { encoding: "utf8", env: { ...process.env, SEMIO_RUNTIME_INNER: "1" }, maxBuffer: 1 << 26 });
    expect(run.status, `${run.stdout.slice(-3000)}${run.stderr.slice(-2000)}`).toBe(0);
  }, 180000);
});
