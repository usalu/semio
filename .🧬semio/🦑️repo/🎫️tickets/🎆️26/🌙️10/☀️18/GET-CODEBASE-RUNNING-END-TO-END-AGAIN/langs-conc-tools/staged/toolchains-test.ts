/** 🧰️ The native toolchain contract agrees with its language-agnostic fixture, an independent Python implementation, two JSON Schema validators, the publishers' own checksum listings and a real uv: pins identify toolchains, never machines, and every Python project owns an environment outside the checkout. @see ./🔣️.json */
import {describe, expect, setDefaultTimeout, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {constants, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import * as toolchains from "../🟦️.ts";
import policy from "../🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type: "json"};

setDefaultTimeout(120000);
const here = import.meta.dir, posix = (path: string) => path.replaceAll("\\", "/");
const python = testPythonV1();
const oracle = (extra: Record<string, unknown> = {}) => {
  if (python === null) throw new Error("A Python 3 interpreter is required for the independent implementation");
  const run = spawnSync(python, ["-I", join(here, "🐍️oracle/🐍️.py")], {input: JSON.stringify({policy, environments: fixture.environments, identities: fixture.identities, tools: fixture.tools, links: fixture.links, ...extra}), encoding: "utf8", maxBuffer: 1 << 26});
  if (run.status !== 0) throw new Error(run.stderr);
  return JSON.parse(run.stdout) as {environments: any[]; identities: any[]; tools: any[]; links: boolean[]; files: {globalJson: string; artifacts: string}; props: any};
};
const props = toolchains.dotnetDirectoryBuildPropsV1();
const expected = oracle({props});
const hostOf = (name: string) => name as toolchains.ToolchainHostV1;

describe("the policy", () => {
  test("is closed under two JSON Schema validators and rejects every mutation of its pins", () => {
    const ajv = new Ajv({strict: false, allErrors: true}), validate = ajv.compile(schema);
    expect(validate(policy), JSON.stringify(validate.errors)).toBe(true);
    expect(validateJsonSchemaSubset(schema, policy, schema)).toEqual([]);
    const mutate = (change: (copy: any) => void) => {
      const copy = structuredClone(policy) as any;
      change(copy);
      return copy;
    };
    const mutations: Record<string, (copy: any) => void> = {
      "unknown host": copy => copy.hosts.push("freebsd-x64"),
      "short checksum": copy => { copy.cmake.cmake.distributions[0].sha256 = "abc"; },
      "upper checksum": copy => { copy.cmake.ninja.distributions[0].sha256 = copy.cmake.ninja.distributions[0].sha256.toUpperCase(); },
      "parent segment in a layout": copy => { copy.python.layout.environments = "../outside"; },
      "absolute layout": copy => { copy.python.layout.cache = "/abs"; },
      "unknown archive format": copy => { copy.cmake.cmake.distributions[0].format = "rar"; },
      "foreign release host": copy => { copy.cmake.cmake.release = "https://example.com/cmake/releases/download/v4.4.4"; },
      "missing German text": copy => { delete copy.messages.texts.missing.de; },
      "extra key": copy => { copy.python.surprise = true; },
      "wild sdk": copy => { copy.dotnet.sdk = "8.0"; },
      "roll forward": copy => { copy.dotnet.rollForward = "latestMajor"; },
    };
    for (const [name, change] of Object.entries(mutations)) {
      const copy = mutate(change);
      expect({name, ajv: validate(copy), subset: validateJsonSchemaSubset(schema, copy, schema).length === 0}, name).toEqual({name, ajv: false, subset: false});
    }
  });

  test("covers every host once per tool, in sorted order, with a unique environment name list", () => {
    expect([...policy.hosts]).toEqual([...policy.hosts].sort());
    for (const tool of ["cmake", "ninja", "go"] as const) {
      const rows = (tool === "go" ? policy.go.tool : policy.cmake[tool]).distributions;
      expect(rows.map(row => row.host).sort()).toEqual([...policy.hosts].sort());
      for (const row of rows) {
        expect(row.format).toBe(row.archive.endsWith(".zip") ? "zip" : "tar.gz");
        expect(row.binaries.length).toBeGreaterThan(0);
        expect(row.host.startsWith("win32") ? row.binaries.every(name => name.endsWith(".exe")) : row.binaries.every(name => !name.endsWith(".exe"))).toBe(true);
      }
    }
    for (const kind of ["python", "dotnet", "cmake", "go"] as const) {
      const names = policy[kind].environment;
      expect([...names]).toEqual([...names].sort());
      expect(new Set(names).size).toBe(names.length);
    }
    expect(Object.values(policy.python.interpreters).map(row => row.series).sort()).toEqual(["3.12", "3.14"]);
  });

  test("agrees with the publishers' own checksum listings", () => {
    const rows = new Map(Object.values(fixture.publisher).flatMap(listing => listing.lines.map(line => [line.split("  ")[1]!, line.split("  ")[0]!] as const)));
    const specs = {cmake: policy.cmake.cmake, ninja: policy.cmake.ninja, go: policy.go.tool};
    for (const tool of ["cmake", "ninja", "go"] as const) for (const row of specs[tool].distributions) expect({tool, archive: row.archive, sha256: rows.get(row.archive)}).toEqual({tool, archive: row.archive, sha256: row.sha256});
    expect(Object.values(fixture.publisher).reduce((sum, listing) => sum + listing.lines.length, 0)).toBe(rows.size);
  });

  test("keeps every German text next to an English one with the same placeholders", () => {
    for (const [key, text] of Object.entries(policy.messages.texts)) {
      const holes = (value: string) => [...value.matchAll(/\{([a-z]+)\}/gu)].map(match => match[1]).sort();
      expect({key, holes: holes(text.de)}).toEqual({key, holes: holes(text.en)});
    }
  });
});

describe("the fixture", () => {
  test("is valid under its strict schema", () => {
    const validate = new Ajv({strict: true}).compile(fixtureSchema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });
});

describe("the Python environments", () => {
  test("equal the independent implementation for every project, host, root and lock", () => {
    expect(expected.environments.length).toBe(fixture.environments.length);
    for (const [index, row] of fixture.environments.entries()) {
      const plan = toolchains.pythonEnvironmentV1({project: row.project, host: hostOf(row.host), roots: row.roots, manifestSha256: row.manifest, lockSha256: row.lock}), want = expected.environments[index];
      expect({name: row.name, key: plan.key, directory: posix(plan.directory), interpreters: posix(plan.interpreters), cache: posix(plan.cache), series: plan.series}, row.name).toEqual({name: row.name, key: want.key, directory: want.directory, interpreters: want.interpreters, cache: want.cache, series: want.series});
      const paths = new Set(["UV_PROJECT_ENVIRONMENT", "UV_PYTHON_INSTALL_DIR", "UV_CACHE_DIR"]);
      expect(Object.fromEntries(Object.entries(plan.variables).map(([name, value]) => [name, paths.has(name) ? posix(value) : value])), row.name).toEqual(want.variables);
      expect([...plan.installArguments], row.name).toEqual(want.installArguments);
      expect([...plan.syncArguments], row.name).toEqual(want.syncArguments);
      expect([...plan.runArguments], row.name).toEqual(want.runArguments);
    }
  });

  test("never share a directory between projects, never depend on the checkout path and move with the lock", () => {
    const plans = fixture.environments.map(row => ({row, plan: toolchains.pythonEnvironmentV1({project: row.project, host: hostOf(row.host), roots: row.roots, manifestSha256: row.manifest, lockSha256: row.lock})}));
    const byKey = new Map<string, Set<string>>();
    for (const {row, plan} of plans) (byKey.get(plan.key) ?? byKey.set(plan.key, new Set()).get(plan.key)!).add(row.project);
    for (const [key, projects] of byKey) expect({key, projects: projects.size}).toEqual({key, projects: 1});
    const same = (a: string, b: string) => plans.find(entry => entry.row.name === a)!.plan.key === plans.find(entry => entry.row.name === b)!.plan.key;
    for (const project of policy.python.projects) {
      expect(same(`${project.id} darwin-arm64 root0 a`, `${project.id} darwin-arm64 root1 a`), "another checkout path").toBe(true);
      expect(same(`${project.id} darwin-arm64 root0 a`, `${project.id} darwin-arm64 root0 b`), "another lock").toBe(false);
      expect(same(`${project.id} darwin-arm64 root0 a`, `${project.id} win32-x64 root0 a`), "another host").toBe(false);
    }
  });

  test("keep the planned directory below the Windows path budget for a long checkout path", () => {
    const roots = {dependencies: "C:\\Users\\someone-with-a-long-name\\Documents\\work\\semio-checkout\\.cache\\b\\d", tools: "C:\\Users\\someone-with-a-long-name\\Documents\\work\\semio-checkout\\.cache\\b\\t"};
    for (const project of policy.python.projects) {
      const plan = toolchains.pythonEnvironmentV1({project: project.id, host: "win32-x64", roots, manifestSha256: "0".repeat(64), lockSha256: "1".repeat(64)});
      expect(plan.directory.length + "\\Lib\\site-packages\\some_package\\subpackage\\module_name.py".length).toBeLessThan(256);
    }
  });

  test("refuse unknown projects and malformed digests", () => {
    expect(() => toolchains.pythonEnvironmentV1({project: "nope", host: "linux-x64", roots: fixture.roots[0]!, manifestSha256: "0".repeat(64), lockSha256: "1".repeat(64)})).toThrow();
    expect(() => toolchains.pythonEnvironmentV1({project: "ui-styling", host: "linux-x64", roots: fixture.roots[0]!, manifestSha256: "x", lockSha256: "1".repeat(64)})).toThrow();
    expect(toolchains.pythonProjectForRootV1("./")?.id).toBe("workspace");
    expect(toolchains.pythonProjectForRootV1(policy.python.projects[1]!.root + "/")?.id).toBe("ui-styling");
    expect(toolchains.pythonProjectForRootV1("elsewhere")).toBeUndefined();
  });
});

describe("the toolchain identities", () => {
  test("equal the independent implementation and ignore every variable the policy does not declare", () => {
    for (const [index, row] of fixture.identities.entries()) {
      const identity = toolchains.toolchainIdentityV1(row.kind, {host: hostOf(row.host), ...("project" in row ? {project: row.project} : {}), environment: row.environment}), want = expected.identities[index];
      expect({name: row.name, digest: identity.digest, pins: identity.pins, environment: identity.environment}, row.name).toEqual({name: row.name, digest: want.digest, pins: want.pins, environment: want.environment});
      expect(Object.keys(identity.environment).every(name => (policy[row.kind].environment as readonly string[]).includes(name))).toBe(true);
      expect(JSON.stringify(identity)).not.toContain("/not/declared");
    }
  });

  test("differ by kind, host and every declared variable and never by an installation path", () => {
    const digests = new Set(fixture.identities.filter(row => Object.keys(row.environment).length === 0 && !("project" in row)).map(row => toolchains.toolchainIdentityV1(row.kind, {host: hostOf(row.host)}).digest));
    expect(digests.size).toBe(4 * policy.hosts.length);
    const a = toolchains.toolchainIdentityV1("go", {host: "linux-x64", environment: {GOFLAGS: "-a"}}).digest, b = toolchains.toolchainIdentityV1("go", {host: "linux-x64", environment: {GOFLAGS: "-b"}}).digest;
    expect(a).not.toBe(b);
    expect(toolchains.toolchainIdentityV1("go", {host: "linux-x64", environment: {GOPATH: "/anywhere"}}).digest).toBe(toolchains.toolchainIdentityV1("go", {host: "linux-x64"}).digest);
    expect(toolchains.toolchainIdentityV1("python", {host: "linux-x64", project: "energy-oracle"}).digest).not.toBe(toolchains.toolchainIdentityV1("python", {host: "linux-x64", project: "ui-styling"}).digest);
    expect(toolchains.toolchainIdentityV1("python", {host: "linux-x64", project: "ui-styling"}).digest).toBe(toolchains.toolchainIdentityV1("python", {host: "linux-x64", project: "workspace"}).digest);
  });

  test("name the host from the platform and refuse the unknown", () => {
    expect(toolchains.toolchainHostV1("darwin", "arm64")).toBe("darwin-arm64");
    expect(toolchains.toolchainHostV1("win32", "x64")).toBe("win32-x64");
    expect(() => toolchains.toolchainHostV1("freebsd", "x64")).toThrow(/freebsd-x64/u);
  });
});

describe("the pinned archive tools", () => {
  test("equal the independent implementation for every host", () => {
    for (const [index, row] of fixture.tools.entries()) {
      const tool = row.tool, host = hostOf(row.host), want = expected.tools[index];
      expect({name: row.name, directory: posix(toolchains.nativeToolDirectoryV1(tool, host, row.root)), binaries: posix(toolchains.nativeToolBinaryDirectoryV1(tool, host, row.root)), url: toolchains.nativeToolUrlV1(tool, host), identity: toolchains.nativeToolIdentityV1(tool, host)}, row.name).toEqual({name: row.name, directory: want.directory, binaries: want.binaryDirectory, url: want.url, identity: want.identity});
    }
    expect(toolchains.nativeToolPathV1("linux-x64", "/t").map(posix)).toEqual(["/t/native/cmake/4.4.4/linux-x64-e5bb807f7728cb60/bin", "/t/native/ninja/1.13.2/linux-x64-5749cbc4e6682735"]);
  });

  test("download only from the publishers' release addresses and name no installation state in the identity", () => {
    for (const row of fixture.tools) {
      const url = new URL(toolchains.nativeToolUrlV1(row.tool, hostOf(row.host)));
      expect({host: url.hostname, https: url.protocol}).toEqual({host: row.tool === "go" ? "dl.google.com" : "github.com", https: "https:"});
      expect(toolchains.nativeToolIdentityV1(row.tool, hostOf(row.host))).not.toContain("/");
    }
  });
});

describe("the extracted links", () => {
  test("agree with the independent implementation", () => {
    expect(fixture.links.map(row => toolchains.extractedLinkEscapesV1(row.path, row.target))).toEqual(expected.links);
    expect(expected.links).toContain(true);
    expect(expected.links).toContain(false);
  });
});

describe("the rendered repository files", () => {
  test("render the global.json the independent implementation renders and the SDK the policy pins", () => {
    expect(toolchains.dotnetGlobalJsonV1()).toBe(expected.files.globalJson);
    expect(JSON.parse(toolchains.dotnetGlobalJsonV1())).toEqual({sdk: {version: policy.dotnet.sdk, rollForward: policy.dotnet.rollForward}});
  });

  test("render a Directory.Build.props the independent XML parser reads as the policy says", () => {
    expect(expected.props).toEqual({root: "Project", use: "true", condition: "'$(ArtifactsPath)' == ''", path: `$(MSBuildThisFileDirectory)${policy.dotnet.artifacts}/$(MSBuildProjectName)`});
  });

  test("read the pinned Go version of a go.work text", () => {
    expect(toolchains.goWorkVersionV1("go 1.26.9\n\nuse (\n\t./a\n)\n")).toBe("1.26.9");
    expect(toolchains.goWorkVersionV1("go 1.25\n")).toBeUndefined();
    expect(toolchains.goWorkVersionV1("toolchain go1.26.9\n")).toBeUndefined();
  });
});

describe("the texts", () => {
  test("render in the selected language and show every language when none is selected", () => {
    const [english, german, both] = fixture.messages.map(row => toolchains.toolchainMessageV1(row.key as keyof typeof policy.messages.texts, row.values, row.environment));
    expect(english).toBe(policy.messages.texts.missing.en.replace("{tool}", "cmake").replace("{version}", "4.4.4"));
    expect(german).toBe(policy.messages.texts.missing.de.replace("{tool}", "cmake").replace("{version}", "4.4.4"));
    expect(both).toBe(policy.messages.texts.host.en.replace("{host}", "freebsd-x64") + " / " + policy.messages.texts.host.de.replace("{host}", "freebsd-x64"));
  });
});

const uv = Bun.which("uv"), dotnet = Bun.which("dotnet"), interpreterHome = join(process.env.HOME ?? "", ".local/share/uv/python");
const managed = existsSync(interpreterHome) ? readdirSync(interpreterHome, {withFileTypes: true}).filter(entry => /^cpython-3\.14\.\d+-/u.test(entry.name))[0]?.name : undefined;

describe.skipIf(dotnet === null)("a real MSBuild", () => {
  const sandbox = mkdtempSync(join(tmpdir(), "tc-dotnet-")), packageRoot = join(sandbox, "pkg");
  mkdirSync(packageRoot, {recursive: true});
  writeFileSync(join(sandbox, "Directory.Build.props"), props);
  writeFileSync(join(sandbox, "global.json"), toolchains.dotnetGlobalJsonV1());
  writeFileSync(join(packageRoot, "probe.csproj"), '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><EnableDefaultCompileItems>false</EnableDefaultCompileItems></PropertyGroup></Project>');
  const environment = {...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1", DOTNET_CLI_USE_MSBUILD_SERVER: "0", MSBUILDDISABLENODEREUSE: "1", DOTNET_CLI_HOME: join(sandbox, "home"), NUGET_PACKAGES: join(sandbox, "packages")};
  const msbuild = (args: readonly string[]) => spawnSync(dotnet!, ["msbuild", join(packageRoot, "probe.csproj"), "-nodeReuse:false", ...args], {cwd: packageRoot, env: environment, encoding: "utf8"});

  test("puts every raw build output below the artifacts path of the repository and lets an explicit path win", () => {
    const wanted = `${join(sandbox, ...policy.dotnet.artifacts.split("/"), "probe")}`;
    const raw = msbuild(["-getProperty:ArtifactsPath", "-getProperty:UseArtifactsOutput"]);
    expect(raw.status, raw.stderr + raw.stdout).toBe(0);
    expect(JSON.parse(raw.stdout).Properties).toEqual({ArtifactsPath: wanted, UseArtifactsOutput: "true"});
    expect(msbuild(["-p:ArtifactsPath=" + join(sandbox, "session"), "-getProperty:ArtifactsPath"]).stdout.trim()).toBe(join(sandbox, "session"));
  });

  test("pins the SDK: an exact sdk resolves, a different pinned version is refused", () => {
    const selected = spawnSync(dotnet!, ["--version"], {cwd: packageRoot, env: environment, encoding: "utf8"});
    expect(selected.status, selected.stderr).toBe(0);
    expect(selected.stdout.trim()).toBe(policy.dotnet.sdk);
    writeFileSync(join(sandbox, "global.json"), toolchains.dotnetGlobalJsonV1().replace(policy.dotnet.sdk, "8.0.999"));
    const refused = spawnSync(dotnet!, ["--version"], {cwd: packageRoot, env: environment, encoding: "utf8"});
    expect(refused.status).not.toBe(0);
    writeFileSync(join(sandbox, "global.json"), toolchains.dotnetGlobalJsonV1());
  });

  test("builds without writing obj or bin into the package", {timeout: 180000}, () => {
    const built = spawnSync(dotnet!, ["build", join(packageRoot, "probe.csproj"), "-nodeReuse:false", "-p:UseSharedCompilation=false", "-v:q"], {cwd: packageRoot, env: environment, encoding: "utf8"});
    expect(built.status, built.stderr + built.stdout).toBe(0);
    expect(readdirSync(packageRoot).sort()).toEqual(["probe.csproj"]);
    expect(existsSync(join(sandbox, ...policy.dotnet.artifacts.split("/"), "probe", "bin"))).toBe(true);
    expect(existsSync(join(sandbox, ...policy.dotnet.artifacts.split("/"), "probe", "obj"))).toBe(true);
    spawnSync(dotnet!, ["build-server", "shutdown"], {env: environment, encoding: "utf8"});
    rmSync(sandbox, {recursive: true, force: true});
  });
});

describe.skipIf(uv === null || managed === undefined)("a real uv", () => {
  const sandbox = mkdtempSync(join(tmpdir(), "tc-uv-"));
  const sha = (text: string) => createHash("sha256").update(text).digest("hex");
  const roots = {dependencies: join(sandbox, "deps"), tools: join(sandbox, "tools")};
  const project = join(sandbox, "project");
  mkdirSync(project, {recursive: true});
  mkdirSync(join(roots.tools, ...policy.python.layout.interpreters.split("/")), {recursive: true});
  if (managed !== undefined) cpSync(join(interpreterHome, managed), join(roots.tools, ...policy.python.layout.interpreters.split("/"), managed), {recursive: true, mode: constants.COPYFILE_FICLONE});
  const manifest = '[project]\nname = "probe"\nversion = "0.0.0"\nrequires-python = ">=3.14,<3.15"\ndependencies = []\n\n[dependency-groups]\nbuild = []\ntest = []\n';
  writeFileSync(join(project, "pyproject.toml"), manifest);
  const environmentOf = (variables: Readonly<Record<string, string>>, cleanPath = false) => ({HOME: process.env.HOME ?? "", PATH: cleanPath ? "/usr/bin:/bin" : process.env.PATH ?? "", ...variables});
  const plan = () => toolchains.pythonEnvironmentV1({project: "ui-styling", host: toolchains.toolchainHostV1(), roots, manifestSha256: sha(manifest), lockSha256: sha("lock")});
  const run = (args: readonly string[], variables: Readonly<Record<string, string>>, cleanPath = false) => spawnSync(uv!, [...args], {cwd: project, env: environmentOf(variables, cleanPath), encoding: "utf8"});
  const projectArgs = (args: readonly string[]) => args.map(argument => argument === policy.python.projects[1]!.root ? project : argument);

  test("syncs the planned environment outside the project and leaves no .venv behind", () => {
    const planned = plan();
    expect(run(["lock"], planned.variables).status).toBe(0);
    const synced = run(projectArgs(planned.syncArguments), planned.variables);
    expect(synced.stderr + synced.stdout).not.toMatch(/error/iu);
    expect(synced.status).toBe(0);
    expect(existsSync(join(planned.directory, process.platform === "win32" ? "Scripts/python.exe" : "bin/python"))).toBe(true);
    expect(existsSync(join(project, ".venv"))).toBe(false);
    expect(readdirSync(project).sort()).toEqual(["pyproject.toml", "uv.lock"]);
  });

  test("runs on the pinned series from the managed interpreters only", () => {
    const planned = plan(), ran = run([...projectArgs(planned.runArguments), "python", "-c", "import sys;print('.'.join(map(str,sys.version_info[:2])), sys.prefix)"], planned.variables);
    expect(ran.status, ran.stderr).toBe(0);
    const [series, prefix] = ran.stdout.trim().split(" ");
    expect(series).toBe(planned.series);
    expect(realpathSync(prefix!)).toBe(realpathSync(planned.directory));
    const home = /^home = (.*)$/mu.exec(readFileSync(join(planned.directory, "pyvenv.cfg"), "utf8"))![1]!;
    expect(realpathSync(home).startsWith(realpathSync(join(roots.tools, ...policy.python.layout.interpreters.split("/"), managed!)))).toBe(true);
  });

  test("refuses a system interpreter when the managed one is absent", () => {
    const empty = mkdtempSync(join(tmpdir(), "tc-empty-")), planned = toolchains.pythonEnvironmentV1({project: "ui-styling", host: toolchains.toolchainHostV1(), roots: {dependencies: join(empty, "deps"), tools: join(empty, "tools")}, manifestSha256: sha(manifest), lockSha256: sha("lock")});
    const refused = run(projectArgs(planned.syncArguments), planned.variables, true);
    expect(refused.status).not.toBe(0);
    expect(refused.stderr).toMatch(/interpreter|Python/u);
    expect(existsSync(planned.directory)).toBe(false);
    rmSync(empty, {recursive: true, force: true});
  });

  test("shares one environment between concurrent identical syncs and gives a new lock its own", async () => {
    const planned = plan(), env = environmentOf(planned.variables);
    const runs = await Promise.all([1, 2, 3].map(() => new Promise<number | null>(resolve => { const child = Bun.spawn([uv!, ...projectArgs(planned.syncArguments)], {cwd: project, env, stdout: "ignore", stderr: "ignore"}); child.exited.then(resolve); })));
    expect(runs).toEqual([0, 0, 0]);
    const other = toolchains.pythonEnvironmentV1({project: "ui-styling", host: toolchains.toolchainHostV1(), roots, manifestSha256: sha(manifest), lockSha256: sha("lock-2")});
    expect(other.directory).not.toBe(planned.directory);
    expect(dirname(other.directory)).toBe(dirname(planned.directory));
    rmSync(sandbox, {recursive: true, force: true});
  });
});
