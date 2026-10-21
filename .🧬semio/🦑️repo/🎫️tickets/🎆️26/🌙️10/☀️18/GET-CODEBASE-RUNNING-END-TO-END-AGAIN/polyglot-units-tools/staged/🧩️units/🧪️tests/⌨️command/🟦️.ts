import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import {applyGoOverlayV1, planGoPackagesV1, readUnitsPolicyV1, runUnitsCommandV1, sha256V1, unitsArgumentsV1, unitsOptionNamesV1, unitsSchemaFailuresV1, type UnitsCommandPortsV1} from "../../🟦️.ts";
import policyDocument from "../../🔣️.json" with {type: "json"};

const policy = readUnitsPolicyV1(policyDocument);
const have = (command: string): boolean => spawnSync(command, [command === "go" ? "version" : "--version"], {encoding: "utf8"}).status === 0;
const temporary = (): string => mkdtempSync(join(tmpdir(), "units-command-"));
const write = (root: string, files: Readonly<Record<string, string>>): void => {for (const [path, text] of Object.entries(files)) {mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text);}};

const portsFor = (files: readonly string[], options: {signal?: AbortSignal; lines?: string[]; progress?: string[]} = {}): UnitsCommandPortsV1 => {
  const signal = options.signal ?? new AbortController().signal;
  return {
    write: line => {options.lines?.push(line);},
    progress: async stage => {options.progress?.push(stage);},
    check: () => {if (signal.aborted) throw Error("cancelled");},
    signal,
    language: requested => requested ?? "en",
    run: async (argv, run) => {const result = spawnSync(argv[0]!, argv.slice(1), {cwd: run.cwd, env: {...process.env, ...run.env}, encoding: "utf8"}); return {code: result.status ?? 1, output: (result.stdout ?? "") + (result.stderr ?? "")};},
    files: async () => files,
  };
};

describe("repo units: arguments", () => {
  test("every operation parses with its options, long and `--name value` spellings", () => {
    expect(unitsArgumentsV1(["plan", "--for=go", "--json"])).toMatchObject({operation: "plan", for: "go", json: true});
    expect(unitsArgumentsV1(["run", "--for", "python", "--concurrency=4", "--only=python:a", "--only", "python:b", "--receipts=r"])).toMatchObject({operation: "run", for: "python", jobs: 4, only: ["python:a", "python:b"], store: "r"});
    expect(unitsArgumentsV1(["emit", "bazel", "--for=go", "--out=d", "--lang=de"])).toMatchObject({operation: "emit", target: "bazel", out: "d", language: "de"});
    expect(unitsArgumentsV1(["exec", "--spec=s.json", "--out=o.json", "--iface-dep=a", "--iface-dep=b", "--impl-dep=c"])).toMatchObject({operation: "exec", ifaceDeps: ["a", "b"], implDeps: ["c"]});
    expect(unitsArgumentsV1(["exec", "--spec=s.json", "--out-root=out"])).toMatchObject({outRoot: "out"});
  });
  test("refusals are written in both languages", () => {
    for (const bad of [[], ["frob"], ["plan"], ["plan", "--for=cobol"], ["plan", "--for=go", "--for=python"], ["plan", "--for=go", "--nope"], ["plan", "--for=go", "--concurrency=99"], ["emit", "docker", "--for=go", "--out=x"], ["emit", "nx", "--for=go"], ["exec", "--spec=s"], ["plan", "--for=go", "--lang=fr"], ["plan", "--for=go", "--root"]]) {
      let message = "";
      try {unitsArgumentsV1(bad);} catch (error) {message = String((error as Error).message);}
      expect(message.split("\n").length, JSON.stringify(bad)).toBe(2);
    }
  });
  test("the operations catalogue (once wired) declares exactly the options the parser accepts", async () => {
    const path = join(import.meta.dir, "../../../⚙️operations/🔣️catalog.json");
    if (!existsSync(path)) return;
    const catalog = JSON.parse(readFileSync(path, "utf8")) as {operations: {name: string; flags: string[]; subcommands?: {flags: string[]}[]}[]; flags: {name: string; value: string; operations?: string[]}[]};
    const operation = catalog.operations.find(row => row.name === "units");
    if (!operation) return;
    const names = unitsOptionNamesV1(), accepted = new Set([...names.valued, ...names.plain].filter(name => name !== "--lang"));
    expect(new Set(operation.flags)).toEqual(accepted);
    for (const flag of operation.flags) {const row = catalog.flags.find(candidate => candidate.name === flag)!; expect(row.operations).toContain("units"); expect(row.value === "none").toBe(names.plain.includes(flag));}
    for (const subcommand of operation.subcommands ?? []) for (const flag of subcommand.flags) expect(accepted.has(flag)).toBe(true);
  });
});

describe("repo units: plan, run, emit, exec with real python", () => {
  const files = {
    "lab/__init__.py": "",
    "lab/a.py": "def big(x):\n    return x + 1\n\nCONSTANT = 3\n",
    "lab/b.py": "from lab.a import big, CONSTANT\n\ndef use():\n    return big(CONSTANT)\n",
    "lab/c.py": "from lab.b import use\n\nRESULT = use()\n",
  };
  test("plan prints numbers in the requested language and writes the graph", async () => {
    if (!have("python3")) return;
    const root = temporary();
    try {
      write(root, files);
      for (const [lang, expected] of [["en", "Planning 4 compilation units: 2 edges, 3 layers"], ["de", "Plane 4 Kompilationseinheiten: 2 edges, 3 layers"]] as const) {
        const lines: string[] = [];
        await runUnitsCommandV1(root, ["units", "plan", "--for=python", `--lang=${lang}`, "--file=graph.json"], portsFor(Object.keys(files), {lines}));
        expect(lines).toEqual([expected]);
      }
      expect(unitsSchemaFailuresV1("Graph", JSON.parse(readFileSync(join(root, "graph.json"), "utf8")))).toEqual([]);
    } finally {rmSync(root, {recursive: true, force: true});}
  });
  test("run shares receipts: cold, hot, then a body edit is cut off at the edited unit", async () => {
    if (!have("python3")) return;
    const root = temporary();
    try {
      write(root, files);
      const run = async (): Promise<string[]> => {const lines: string[] = []; await runUnitsCommandV1(root, ["run", "--for=python", "--receipts=.receipts", "--concurrency=2"], portsFor(Object.keys(files), {lines})); return lines;};
      const cold = await run();
      expect(cold.at(-1)).toBe("4 built, 0 cached, 0 cut off, 0 failed");
      expect((await run()).at(-1)).toBe("0 built, 4 cached, 0 cut off, 0 failed");
      writeFileSync(join(root, "lab/a.py"), files["lab/a.py"].replace("x + 1", "x + 2"));
      const edited = await run();
      expect(edited.at(-1)).toBe("1 built, 3 cached, 1 cut off, 0 failed");
      expect(edited.some(line => line.startsWith("python:lab.a: rebuilt, interface unchanged"))).toBe(true);
      writeFileSync(join(root, "lab/a.py"), "CONSTANT = 3\n");
      let failure = "";
      try {await run();} catch (error) {failure = String((error as Error).message);}
      expect(failure).toContain("1 units failed");
    } finally {rmSync(root, {recursive: true, force: true});}
  }, 120000);
  test("emit writes spec files, Nx targets or the Bazel rule and BUILD file", async () => {
    if (!have("python3")) return;
    const root = temporary();
    try {
      write(root, files);
      await runUnitsCommandV1(root, ["emit", "nx", "--for=python", "--out=nx"], portsFor(Object.keys(files)));
      const targets = JSON.parse(readFileSync(join(root, "nx/targets.json"), "utf8")) as {name: string; command: string[]}[];
      expect(targets.length).toBe(4);
      for (const target of targets) {expect(existsSync(join(root, "nx/specs", target.name + ".json"))).toBe(true); expect(unitsSchemaFailuresV1("Unit", JSON.parse(readFileSync(join(root, "nx/specs", target.name + ".json"), "utf8")))).toEqual([]);}
      await runUnitsCommandV1(root, ["emit", "bazel", "--for=python", "--out=bz"], portsFor(Object.keys(files)));
      const build = readFileSync(join(root, "bz/BUILD.bazel"), "utf8");
      expect(build.split("repo_unit(").length - 1).toBe(4);
      expect(build).toContain('SEMIO_LANGUAGE": "en"');
      expect(readFileSync(join(root, "bz/units.bzl"), "utf8")).toContain("repo_unit = rule(");
    } finally {rmSync(root, {recursive: true, force: true});}
  });
  test("exec reads dependency files, writes out.json and iface.json and fails with the diagnostics of a bad import", async () => {
    if (!have("python3")) return;
    const root = temporary();
    try {
      write(root, files);
      await runUnitsCommandV1(root, ["emit", "bazel", "--for=python", "--out=bz"], portsFor(Object.keys(files)));
      const specOf = (id: string): string => {const dir = join(root, "bz/specs"); for (const name of require("node:fs").readdirSync(dir) as string[]) if ((JSON.parse(readFileSync(join(dir, name), "utf8")) as {id: string}).id === id) return join("bz/specs", name); throw Error(id);};
      const exec = (id: string, extra: string[]): Promise<void> => runUnitsCommandV1(root, ["exec", `--spec=${specOf(id)}`, ...extra], portsFor([]));
      await exec("python:lab.a", ["--out=r/a/out.json", "--iface=r/a/iface.json"]);
      const iface = JSON.parse(readFileSync(join(root, "r/a/iface.json"), "utf8"));
      expect(unitsSchemaFailuresV1("InterfaceFile", iface)).toEqual([]);
      expect(unitsSchemaFailuresV1("OutFile", JSON.parse(readFileSync(join(root, "r/a/out.json"), "utf8")))).toEqual([]);
      expect((JSON.parse(iface.artifact) as {modules: Record<string, {names: string[]}>}).modules["lab.a"]!.names).toEqual(["CONSTANT", "big"]);
      await exec("python:lab.b", ["--out=r/b/out.json", "--iface=r/b/iface.json", "--iface-dep=r/a/iface.json"]);
      writeFileSync(join(root, "lab/a.py"), "CONSTANT = 3\n");
      await exec("python:lab.a", ["--out=r/a2/out.json", "--iface=r/a2/iface.json"]);
      let failure = "";
      try {await exec("python:lab.b", ["--out=r/b2/out.json", "--iface=r/b2/iface.json", "--iface-dep=r/a2/iface.json"]);} catch (error) {failure = String((error as Error).message);}
      expect(failure).toBe("python:lab.b: failed");
      expect(JSON.parse(readFileSync(join(root, "r/b2/out.json"), "utf8")).diagnostics).toContain("cannot import name 'big' from 'lab.a'");
      let missing = "";
      try {await exec("python:lab.b", ["--out=r/b3/out.json", "--iface=r/b3/iface.json"]);} catch (error) {missing = String((error as Error).message);}
      expect(missing.split("\n").length).toBe(2);
    } finally {rmSync(root, {recursive: true, force: true});}
  }, 120000);
  test("cancellation stops before anything is executed or written", async () => {
    if (!have("python3")) return;
    const root = temporary(), controller = new AbortController();
    try {
      write(root, files);
      controller.abort();
      let message = "";
      try {await runUnitsCommandV1(root, ["run", "--for=python"], portsFor(Object.keys(files), {signal: controller.signal}));} catch (error) {message = String((error as Error).message);}
      expect(message).toBe("cancelled");
    } finally {rmSync(root, {recursive: true, force: true});}
  });
});

describe("go compiler overlay", () => {
  const modulePath = "example.com/m";
  test("member directories merge into their parent package, unmapped cases are not packages, go list agrees", () => {
    if (!have("go")) return;
    const directory = temporary();
    try {
      const real: Record<string, string> = {
        "m/go.mod": `module ${modulePath}\n\ngo 1.25\n`,
        "m/impl.go": "package m\n\nfunc One() int { return 1 }\n",
        "m/🔧️member/🐹️.go": "package m\n\nfunc Two() int { return One() + 1 }\n",
        "m/🧪️tests/🅰️case/🐹️.go": "package m\n\nimport \"testing\"\n\nfunc TestOne(t *testing.T) { if One() != 1 { t.Fatal() } }\n",
        "m/🧪️tests/🅱️adapter/🐹️.go": "package adapter\n\nfunc Host() {}\n",
      };
      write(directory, real);
      const replace = {[join(directory, "m/zz_semio_aaaa.go")]: join(directory, "m/🔧️member/🐹️.go"), [join(directory, "m/zz_semio_bbbb_test.go")]: join(directory, "m/🧪️tests/🅰️case/🐹️.go")};
      const overlayFile = join(directory, "overlay.json");
      writeFileSync(overlayFile, JSON.stringify({Replace: replace}));
      const snapshot = applyGoOverlayV1({files: Object.keys(real), read: path => readFileSync(join(directory, path), "utf8")}, directory, Object.fromEntries(Object.entries(replace).map(([virtual, source]) => [virtual, source])), ["🧪️tests", "🔧️member"]);
      const {packages} = planGoPackagesV1({files: snapshot.files, read: snapshot.read}, {policy, context: {goos: "darwin", goarch: "arm64", goMinor: 25, cgo: false, tags: []}});
      expect(packages.map(row => row.importPath)).toEqual([modulePath]);
      expect([...packages[0]!.goFiles].sort()).toEqual(["m/impl.go", "m/zz_semio_aaaa.go"]);
      expect([...packages[0]!.testGoFiles]).toEqual(["m/zz_semio_bbbb_test.go"]);
      expect(snapshot.realPath!("m/zz_semio_aaaa.go")).toBe("m/🔧️member/🐹️.go");
      const list = spawnSync("go", ["list", `-overlay=${overlayFile}`, "-json", "-e", "."], {cwd: join(directory, "m"), env: {...process.env, GOFLAGS: "", GOPROXY: "off", GOWORK: "off", GOCACHE: join(directory, ".gocache")}, encoding: "utf8"});
      const item = JSON.parse(list.stdout) as {GoFiles: string[]; TestGoFiles: string[]};
      expect([...item.GoFiles].sort()).toEqual(["impl.go", "zz_semio_aaaa.go"]);
      expect(item.TestGoFiles).toEqual(["zz_semio_bbbb_test.go"]);
      void sha256V1;
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 60000);
});

describe("repo units exec with go: interface files instead of dependency sources", () => {
  const files = {
    "go.mod": "module example.com/lab\n\ngo 1.25\n",
    "a/a.go": "package a\n\nimport \"strings\"\n\nfunc Big(s string) string {\n\tvar out []string\n\tfor i := 0; i < 10; i++ {\n\t\tout = append(out, strings.ToUpper(s))\n\t}\n\treturn strings.Join(out, \",\")\n}\n\nfunc Small(x int) int { return x + 1 }\n",
    "b/b.go": "package b\n\nimport \"example.com/lab/a\"\n\nfunc Use() string { return a.Big(\"q\") }\n",
    "c/c.go": "package main\n\nimport \"example.com/lab/b\"\n\nfunc main() { println(b.Use()) }\n",
  };
  test("pure-Go libraries compile with go tool compile against dependency export data; a body edit leaves iface.json unchanged, a signature edit moves it", async () => {
    if (!have("go")) return;
    const root = temporary();
    try {
      write(root, files);
      const env = {GOFLAGS: "-mod=mod", GOPROXY: "off", GOCACHE: join(root, ".gocache"), GOPATH: join(root, ".gopath"), GOWORK: "off"};
      const ports = {...portsFor(Object.keys(files)), run: async (argv: readonly string[], run: {cwd: string; env: Readonly<Record<string, string>>}) => {const result = spawnSync(argv[0]!, argv.slice(1), {cwd: run.cwd, env: {...process.env, ...env, ...run.env}, encoding: "utf8"}); return {code: result.status ?? 1, output: (result.stdout ?? "") + (result.stderr ?? "")};}};
      await runUnitsCommandV1(root, ["emit", "bazel", "--for=go", "--out=bz"], ports);
      const specs = Object.fromEntries((require("node:fs").readdirSync(join(root, "bz/specs")) as string[]).map(name => [(JSON.parse(readFileSync(join(root, "bz/specs", name), "utf8")) as {id: string}).id, join("bz/specs", name)]));
      const spec = JSON.parse(readFileSync(join(root, specs["go:example.com/lab/b"]!), "utf8")) as {action: {env: Record<string, string>}};
      expect(spec.action.env.SEMIO_GO_MODE).toBe("compile");
      expect(JSON.parse(readFileSync(join(root, specs["go:example.com/lab/c"]!), "utf8")).action.env.SEMIO_BAZEL_INPUTS).toBe("sources");
      const exec = (id: string, tag: string, extra: string[] = []): Promise<void> => runUnitsCommandV1(root, ["exec", `--spec=${specs[id]}`, `--out=r/${tag}/out.json`, `--iface=r/${tag}/iface.json`, ...extra], ports);
      const read = (tag: string): {interface: string; closure: string; artifact: string} => JSON.parse(readFileSync(join(root, `r/${tag}/iface.json`), "utf8"));
      await exec("go:example.com/lab/a", "a1");
      await exec("go:example.com/lab/b", "b1", ["--iface-dep=r/a1/iface.json"]);
      expect(Object.keys((JSON.parse(read("b1").artifact) as {files: Record<string, string>}).files)).toEqual(["example.com/lab/a", "example.com/lab/b"]);
      writeFileSync(join(root, "a/a.go"), files["a/a.go"].replace("ToUpper", "ToLower"));
      await exec("go:example.com/lab/a", "a2");
      expect(read("a2").closure).toBe(read("a1").closure);
      expect(read("a2").artifact).toBe(read("a1").artifact);
      writeFileSync(join(root, "a/a.go"), files["a/a.go"].replace("func Small(x int) int", "func Small(x int64) int64"));
      await exec("go:example.com/lab/a", "a3");
      expect(read("a3").closure).not.toBe(read("a1").closure);
      writeFileSync(join(root, "a/a.go"), "package a\n\nfunc Big(s string) string {\n");
      let failure = "";
      try {await exec("go:example.com/lab/a", "a4");} catch (error) {failure = String((error as Error).message);}
      expect(failure).toBe("go:example.com/lab/a: failed");
    } finally {rmSync(root, {recursive: true, force: true});}
  }, 180000);
});
