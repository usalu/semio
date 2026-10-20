/** 📌️ Every pin of a native toolchain is written once and every other place that names it agrees: the contract policy, the bootstrap pins, the devcontainer, the manifests, the locks, the schemas, the cache policy, the project manifests and the hermetic class table. The manifests are read by two TOML parsers; the repository state is read from the checkout, so a drifting pin fails here before it fails on a machine. @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🧰️toolchains/🔣️.json */
import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {parse as parseIarna} from "@iarna/toml";
import * as SmolToml from "smol-toml";
import {repositoryCachePartitionRootV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {HERMETIC_TABLE_V1} from "../../🏗️build/🏃️execution/🔒️hermetic/🟦️.ts";
import {NATIVE_TOOLCHAINS_V1 as policy, dotnetDirectoryBuildPropsV1, dotnetGlobalJsonV1, goWorkVersionV1, pythonSeriesV1, toolchainRootsV1} from "../../../../../../🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts";

const libraryRoot = resolve(import.meta.dir, "../.."), repoRoot = resolve(libraryRoot, "../../../../..");
const read = (path: string) => readFileSync(join(repoRoot, path), "utf8"), json = (path: string) => JSON.parse(read(path));
const bootstrap = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap";
const pins = json(`${bootstrap}/🧬️schema/🔣️.json`).definitions.Pins.const.assets as {tool: string; version: string; platform: string; arch: string; url: string; sha256: string}[];
const devcontainer = Bun.JSONC.parse(read(".devcontainer/devcontainer.json")) as {features: Record<string, {version?: string; additionalVersions?: string}>};
const toml = (text: string) => {
  const bun = Bun.TOML.parse(text) as Record<string, any>;
  expect(bun).toEqual(parseIarna(text) as Record<string, any>);
  expect(bun).toEqual(SmolToml.parse(text) as Record<string, any>);
  return bun;
};
const tracked = (pattern: string) => spawnSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", pattern], {cwd: repoRoot, encoding: "utf8"}).stdout.split("\0").filter(Boolean);
const next = (series: string) => series.split(".")[0] + "." + (Number(series.split(".")[1]) + 1);

describe("Python", () => {
  const trackedManifests = tracked("*pyproject.toml").filter(path => !path.startsWith(".🧬semio/") && !path.includes("/🧫️fixtures/") && !path.includes("/🧪️tests/"));

  test("the manifests of the repository are exactly the projects of the policy, each on its interpreter series", () => {
    const roots = trackedManifests.map(path => path === "pyproject.toml" ? "." : path.slice(0, -"/pyproject.toml".length)).sort();
    expect(roots).toEqual(policy.python.projects.map(project => project.root).sort());
    for (const project of policy.python.projects) {
      const series = pythonSeriesV1(project.id), manifest = toml(read(join(project.root, "pyproject.toml")));
      expect({id: project.id, requires: manifest.project["requires-python"]}).toEqual({id: project.id, requires: `>=${series},<${next(series)}`});
      expect(read(join(project.root, "uv.lock"))).toContain(`requires-python = "==${series}.*"\n`);
      const declared = Object.keys(manifest["dependency-groups"] ?? {}).sort();
      for (const group of project.groups) expect({id: project.id, group, declared: declared.includes(group)}).toEqual({id: project.id, group, declared: true});
      if (project.id === "workspace") expect([...project.groups].sort()).toEqual(declared);
      if (project.id === "energy-oracle") expect(manifest.tool.uv["default-groups"]).toEqual(project.groups);
    }
  });

  test("the pinned uv is the one of the bootstrap pins, the devcontainer and the styling builder policy", () => {
    expect(new Set(pins.filter(row => row.tool === "uv").map(row => row.version))).toEqual(new Set([policy.python.uv]));
    expect(devcontainer.features["ghcr.io/jsburckhardt/devcontainer-features/uv:1"]!.version).toBe(policy.python.uv);
    const styling = json("🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/🧬️schema/🔣️.json").definitions.Policy.const;
    expect({uv: styling.uvVersion, python: styling.pythonVersion}).toEqual({uv: policy.python.uv, python: pythonSeriesV1("ui-styling")});
    expect(json("✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json").python.version).toBe(pythonSeriesV1("energy-oracle"));
    expect(devcontainer.features["ghcr.io/devcontainers/features/python:1"]!.version).toBe(pythonSeriesV1("workspace"));
    expect(read(`${bootstrap}/🔵️.ps1`)).toContain(`$script:PythonKind = "${pythonSeriesV1("workspace")}"`);
  });

  test("no environment lives inside a project and the planned roots are the partition roots of the cache policy", () => {
    expect(toolchainRootsV1(repoRoot)).toEqual({dependencies: repositoryCachePartitionRootV1(repoRoot, "dependencies"), tools: repositoryCachePartitionRootV1(repoRoot, "tools")});
    const gitignore = read(".gitignore");
    expect(gitignore).toMatch(/^\.venv$/mu);
    expect(tracked("*/.venv/*")).toEqual([]);
    for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts", "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts"]) {
      const text = read(path);
      expect({path, packages: text.includes("--all-packages"), planned: text.includes("pythonEnvironmentOfProjectV1")}).toEqual({path, packages: false, planned: true});
    }
    const builder = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎨️styling/🏗️builder/🐍️python/🟦️.ts");
    expect({system: builder.includes("--no-managed-python") || builder.includes("UV_NO_MANAGED_PYTHON"), downloads: builder.includes('UV_PYTHON_DOWNLOADS:"never"')}).toEqual({system: false, downloads: false});
  });
});

describe(".NET", () => {
  test("global.json and Directory.Build.props are the rendered contract and every project targets the pinned SDK major", () => {
    expect(read("global.json")).toBe(dotnetGlobalJsonV1());
    expect(read("Directory.Build.props")).toBe(dotnetDirectoryBuildPropsV1());
    const major = policy.dotnet.sdk.split(".")[0]!;
    for (const path of tracked("*.csproj")) if (!path.includes("/🧫️fixtures/")) expect({path, framework: /<TargetFramework>net(\d+)\.0<\/TargetFramework>/u.exec(read(path))?.[1]}).toEqual({path, framework: major});
    expect(devcontainer.features["ghcr.io/devcontainers/features/dotnet:2"]!.version).toBe(policy.dotnet.sdk);
  });

  test("the bootstrap installs exactly the pinned SDK from the pinned installers on every operating system", () => {
    const shell = read(`${bootstrap}/🐚️.sh`), windows = read(`${bootstrap}/🔵️.ps1`);
    expect({channel: shell.includes("--channel") || shell.includes("DOTNET_CHANNEL"), exact: shell.includes('--version "$sdk"'), windowsExact: windows.includes("-Version $sdk"), winget8: windows.includes("Microsoft.DotNet.SDK.8")}).toEqual({channel: false, exact: true, windowsExact: true, winget8: false});
    const sh = pins.find(row => row.tool === "dotnet-install")!, ps = pins.filter(row => row.tool === "dotnet-install-powershell");
    expect(ps.map(row => `${row.platform}-${row.arch}`).sort()).toEqual(["win32-arm64", "win32-x64"]);
    for (const row of ps) expect({version: row.version, url: row.url.replace("dotnet-install.ps1", "dotnet-install.sh")}).toEqual({version: sh.version, url: sh.url});
    expect(new Set(ps.map(row => row.sha256)).size).toBe(1);
  });

  test("every repository script puts the .NET outputs below an artifacts path outside the package", () => {
    for (const path of ["🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🔬️dotnet-package-commands/🟦️.ts"]) expect(read(path), path).toContain("--artifacts-path");
    expect(tracked("*/obj/*").filter(path => path.includes("🔷️dotnet"))).toEqual([]);
    expect(tracked("*/bin/*").filter(path => path.includes("🔷️dotnet"))).toEqual([]);
  });
});

describe("CMake and Ninja", () => {
  test("the host package managers no longer provide them and the report names only what the bootstrap provides", () => {
    const shell = read(`${bootstrap}/🐚️.sh`), windows = read(`${bootstrap}/🔵️.ps1`), image = read(".devcontainer/Dockerfile");
    const lists = shell.slice(shell.indexOf("linux_toolchain_packages()"), shell.indexOf("install_linux_packages()"));
    for (const name of ["cmake", "ninja"]) {
      expect(lists, name).not.toMatch(new RegExp(`\\b${name}(?:-build)?\\b`, "u"));
      expect(image, name).not.toMatch(new RegExp(`^\\s+${name}(?:-build)?\\s*\\\\$`, "mu"));
    }
    expect({brew: shell.includes("brew install cmake"), kitware: windows.includes("Kitware.CMake"), ninja: windows.includes("Ninja-build.Ninja"), compiler: windows.includes("Microsoft.VisualStudio.BuildTools"), report: shell.includes("report_missing_tools cc c++ make pkg-config curl git unzip")}).toEqual({brew: false, kitware: false, ninja: false, compiler: true, report: true});
  });

  test("the driver and the project keys select the pinned tools", () => {
    const driver = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🏃️execution/🟦️.ts");
    expect({selects: driver.includes("pinnedNativeEnvironmentV1(activated.environment,tools.root"), before: driver.indexOf("pinnedNativeEnvironmentV1(") < driver.indexOf('program("cmake",environment)')}).toEqual({selects: true, before: true});
    const root = json("📋️project.json");
    for (const target of ["cpp-custody", "cpp-test-toolchain"]) expect(root.targets[target].inputs, target).toEqual(["cmakeSources", "cmakeToolchain", "sharedGlobals"]);
    expect(root.namedInputs.cmakeSources).toContain("{workspaceRoot}/CMakeLists.txt");
    expect(root.namedInputs.cmakeToolchain.filter((row: {env?: string}) => row.env).map((row: {env: string}) => row.env)).toEqual(policy.cmake.environment);
  });
});

describe("Go", () => {
  test("go.work, the bootstrap pins and the devcontainer name one Go and the toolchain is the local one", () => {
    expect(goWorkVersionV1(read("go.work"))).toBe(policy.go.version);
    expect(new Set(pins.filter(row => row.tool === "go").map(row => row.version))).toEqual(new Set([policy.go.version]));
    expect(policy.go.tool.version).toBe(policy.go.version);
    for (const row of pins.filter(candidate => candidate.tool === "go")) {
      const host = `${row.platform}-${row.arch}`, distribution = policy.go.tool.distributions.find(candidate => candidate.host === host);
      expect({host, url: row.url, sha256: row.sha256}).toEqual({host, url: `${policy.go.tool.release}/${distribution?.archive}`, sha256: distribution?.sha256});
    }
    const shell = read(`${bootstrap}/🐚️.sh`), windows = read(`${bootstrap}/🔵️.ps1`);
    expect({shell: shell.includes("ensure_go") || shell.includes("/semio/go/"), windows: windows.includes('-Tool "go"')}).toEqual({shell: false, windows: false});
    expect(policy.go.version.startsWith(`${devcontainer.features["ghcr.io/devcontainers/features/go:1"]!.version}.`)).toBe(true);
    expect(policy.go.variables).toEqual({GOTOOLCHAIN: "local"});
    for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts"]) expect({path, pinned: read(path).includes("pinnedGoEnvironmentV1")}).toEqual({path, pinned: true});
    expect(read("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts")).toContain("NATIVE_TOOLCHAINS_V1.go.variables");
    for (const path of tracked("*go.mod").filter(path => !path.includes("/🧫️fixtures/"))) {
      const declared = /^go (\d+\.\d+(?:\.\d+)?)$/mu.exec(read(path))?.[1];
      expect({path, ok: declared !== undefined && Bun.semver.order(declared.split(".").length === 2 ? declared + ".0" : declared, policy.go.version) <= 0}).toEqual({path, ok: true});
    }
  });
});

describe("the keys", () => {
  test("the cache policy hashes the contract fingerprint and the declared variables of every toolchain and no root Python file", () => {
    const toolchains = json("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json").toolchains;
    const script = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts";
    for (const kind of ["python", "dotnet", "cmake", "go"] as const) {
      expect(toolchains[kind].commands, kind).toEqual([`bun "${script}" fingerprint ${kind}`]);
      expect([...toolchains[kind].environment].sort(), kind).toEqual([...policy[kind].environment].sort());
    }
    expect(toolchains.python.files).toEqual([]);
    for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json", "nx.json"]) expect(json(path).namedInputs.uv).not.toContain("{workspaceRoot}/uv.lock");
    expect(existsSync(join(repoRoot, script))).toBe(true);
  });

  test("the hermetic class table names a toolchain of the contract for every class and keeps unprovisioned ones on host state", () => {
    for (const row of HERMETIC_TABLE_V1.toolchains) expect(["bun", "go", "python", "dotnet", "cmake"]).toContain(row.identity);
    expect(HERMETIC_TABLE_V1.toolchains.filter(row => row.provisioned).map(row => row.id)).toEqual(["bun"]);
    const projects = new Set(HERMETIC_TABLE_V1.classes.filter(row => row.toolchain !== "bun").flatMap(row => row.select.projects));
    expect([...projects].sort()).toEqual(["@semio-tech/*-dotnet", "@semio-tech/*-go", "@semio-tech/*-py", "workspace"]);
  });
});
