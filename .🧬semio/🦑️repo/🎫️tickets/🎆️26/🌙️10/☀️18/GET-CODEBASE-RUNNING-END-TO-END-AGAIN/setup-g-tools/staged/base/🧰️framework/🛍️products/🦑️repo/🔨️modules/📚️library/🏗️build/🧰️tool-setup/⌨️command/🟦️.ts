import {createHash} from "node:crypto";
import {chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync} from "node:fs";
import {homedir} from "node:os";
import {delimiter, dirname, join} from "node:path";
import {captureOwnedCommandV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {createIssuedResourceControlV1, scriptResourceProcessEnvironmentV1, type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {NATIVE_TOOLCHAINS_V1, nativeToolBinaryDirectoryV1, pythonInterpreterDirectoryV1, toolchainHostV1, type ToolchainHostV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import bootstrapPins from "../../../../🔩️native/🥾️bootstrap/🧬️schema/🔣️.json" with {type: "json"};
import {repoCacheDirectory} from "../../../⚡️caching/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {withRepositoryToolingStoreV1, type RepositoryToolingStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {runTool} from "../../../⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts";
import {provisionNativeToolsV1, withPinnedNativePathV1, type NativeToolPortV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";
import {systemNativeToolPortV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts";
import {bunDistribution, prepareBun} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🟦️.ts";
import {binaryenDirectory, prepareBinaryen} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts";
import {bazelDistribution} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts";
import {locateToolV1, renderToolReportV1, resolveSiblingPackageV1, setupToolsV1, runtimeWarningV1, runningRuntimeLineV1, toolRowV1, toolRowsV1, toolSetupMessageV1, verifyToolsV1, type ToolContextV1, type ToolProvisionerV1, type ToolReportV1, type ToolRowV1} from "../🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

const executableSuffix = (platform: NodeJS.Platform): string => platform === "win32" ? ".exe" : "";

/** 🧭️ Reads `tools verify|setup` arguments into the rows they select; no argument selects the whole table. */
export function selectToolRowsV1(names: readonly string[], excluded: readonly string[] = [], environment: Readonly<Record<string, string | undefined>> = process.env): readonly ToolRowV1[] {
  return toolRowsV1(names.length === 0 ? "all" : names, environment).filter(row => !excluded.includes(row.setup.kind));
}

/** 📍️ The directory a dotnet SDK is installed to below the tools partition: named by SDK, host and the digest of the pinned installer script. */
export function dotnetSdkDirectoryV1(toolsRoot: string, sdk: string, host: ToolchainHostV1, scriptSha256: string): string {
  return join(toolsRoot, "native", "dotnet", sdk, host + "-" + scriptSha256.slice(0, 16));
}

/** 📌️ One pinned row of the bootstrap pins. */
export type DotnetInstallerRowV1 = Readonly<{tool: string; version: string; platform: string; arch: string; url: string; sha256: string}>;

/** 📜️ Selects the pinned .NET installer script of a host from the bootstrap pins: PowerShell on Windows, the shell script elsewhere. */
export function dotnetInstallerRowV1(platform: NodeJS.Platform, architecture: string): DotnetInstallerRowV1 | undefined {
  const rows = bootstrapPins.definitions.Pins.const.assets as readonly DotnetInstallerRowV1[];
  return platform === "win32" ? rows.find(row => row.tool === "dotnet-install-powershell" && row.arch === architecture) : rows.find(row => row.tool === "dotnet-install");
}

function provisionedLocation(root: string, toolsRoot: string, platform: NodeJS.Platform, architecture: string, host: ToolchainHostV1, locate: Extract<ToolRowV1["locate"][number], {kind: "provisioned"}>, pin: string): string | undefined {
  const suffix = executableSuffix(platform);
  try {
    if (locate.family === "native") return join(nativeToolBinaryDirectoryV1(locate.tool as "go", host, toolsRoot), locate.executable + suffix);
    if (locate.family === "bun") {
      const musl = platform === "linux" && !(process.report?.getReport?.() as {header?: {glibcVersionRuntime?: string}} | undefined)?.header?.glibcVersionRuntime;
      return join(toolsRoot, "bun", bunDistribution(pin, platform, architecture, musl).sha256, locate.executable + suffix);
    }
    if (locate.family === "bazel") return join(toolsRoot, "bazel", "bin", bazelDistribution(platform, architecture).sha256.slice(0, 16), locate.executable + suffix);
    if (locate.family === "cargo") return join(repositoryCachePartitionRootV1(root, "rust"), "cargo", "home", "bin", locate.executable + suffix);
    if (locate.family === "binaryen") return join(binaryenDirectory(root), "bin", locate.executable + suffix);
    const installer = dotnetInstallerRowV1(platform, architecture);
    return installer === undefined ? undefined : join(dotnetSdkDirectoryV1(toolsRoot, pin, host, installer.sha256), locate.executable + suffix);
  } catch {
    return undefined;
  }
}

/** 🔌️ The context of the running machine: real files, real child programs under the caller's control, and the layouts the repository provisions into. */
export function systemToolContextV1(root: string, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>> = process.env): ToolContextV1 {
  const toolsRoot = repositoryCachePartitionRootV1(root, "tools"), host = toolchainHostV1();
  const hooks = async (row: ToolRowV1, pin: string, signal: AbortSignal): Promise<Readonly<{found: string | null; location: string | null}>> => {
    if (row.setup.kind === "script" && row.setup.verify !== undefined) {
      try {
        await captureOwnedCommandV1(process.execPath, [join(root, ...row.setup.script.split("/")), ...row.setup.verify], root, {control, signal, env: scriptResourceProcessEnvironmentV1(control, environment), output: {maximumBytes: 65536, maximumLines: 256}});
        return {found: pin, location: row.setup.script};
      } catch {
        signal.throwIfAborted();
        return {found: null, location: null};
      }
    }
    const browsers = environment.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(root, "tools", "ms-playwright"), directory = join(browsers, "chromium-" + pin);
    return existsSync(directory) ? {found: pin, location: directory} : {found: null, location: null};
  };
  return Object.freeze({
    root, toolsRoot, platform: process.platform, environment, home: homedir(),
    read: (path: string) => readFileSync(path, "utf8"),
    exists: (path: string) => existsSync(path),
    list: (directory: string) => { try { return readdirSync(directory); } catch { return []; } },
    resolvePackage: (name: string, from: string) => resolveSiblingPackageV1(root, name, from),
    capture: (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => captureOwnedCommandV1(command, args, cwd, {control, signal, env: withPinnedNativePathV1(environment, host, toolsRoot, ["go", "uv", "cmake", "ninja"]), output: {maximumBytes: 65536, maximumLines: 256}}),
    provisioned: (locate, pin) => provisionedLocation(root, toolsRoot, process.platform, process.arch, host, locate, pin),
    hook: (row, pin, signal) => hooks(row, pin, signal),
  });
}

/** 🩺️ Runs `repo tools verify [tool…]`: prints the report and returns the exit code, 1 when any tool differs from its pin. */
export async function runToolsVerifyV1(root: string, control: ResourceLeaseControl, names: readonly string[], write: (line: string) => void = writeResultLineV1): Promise<number> {
  const context = systemToolContextV1(root, control), reports = await verifyToolsV1(selectToolRowsV1(names, [], context.environment), context, control.signal);
  for (const line of renderToolReportV1(reports, context.environment)) write(line);
  const running = (process.versions as {bun?: string}).bun, warning = runtimeWarningV1(reports, running, context.environment);
  if (running !== undefined) write(runningRuntimeLineV1(reports, running, process.execPath, context.environment));
  if (warning !== undefined) write(warning);
  return reports.every(report => report.status === "ok") ? 0 : 1;
}

/** 📥️ Installs the pinned .NET SDK with the pinned installer script (checked against its SHA-256) into a private directory and publishes it by one rename together with a receipt. */
export async function installDotnetSdkV1(sdk: string, directory: string, scriptSha256: string, installer: DotnetInstallerRowV1, host: ToolchainHostV1, environment: Readonly<Record<string, string | undefined>>, port: NativeToolPortV1, signal: AbortSignal, run: (command: string, args: string[], env: NodeJS.ProcessEnv) => Promise<void>): Promise<void> {
  const parent = dirname(directory);
  mkdirSync(parent, {recursive: true});
  const temporary = mkdtempSync(join(parent, ".prepare-"));
  try {
    port.log(toolSetupMessageV1("step", {tool: "dotnet", pin: sdk}, environment));
    const response = await port.fetch(installer.url, signal);
    if (!response.ok) throw new Error(toolSetupMessageV1("installer", {tool: "dotnet", host}, environment));
    const bytes = Buffer.from(await response.arrayBuffer());
    if (createHash("sha256").update(bytes).digest("hex") !== installer.sha256) throw new Error(toolSetupMessageV1("installer", {tool: "dotnet", host}, environment));
    const windows = installer.url.endsWith(".ps1"), script = join(temporary, windows ? "dotnet-install.ps1" : "dotnet-install.sh"), payload = join(temporary, "sdk");
    writeFileSync(script, bytes);
    mkdirSync(payload);
    const base = {...environment, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1"} as NodeJS.ProcessEnv;
    if (windows) await run("powershell", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", script, "-Version", sdk, "-InstallDir", payload, "-NoPath"], base);
    else await run("bash", [script, "--version", sdk, "--install-dir", payload, "--no-path"], base);
    const executable = join(payload, windows ? "dotnet.exe" : "dotnet");
    if (!existsSync(executable)) throw new Error(toolSetupMessageV1("installer", {tool: "dotnet", host}, environment));
    if (!windows) chmodSync(executable, 0o755);
    writeFileSync(join(payload, ".toolchain.json"), JSON.stringify({owner: "workspace:native-tools", identity: "dotnet:" + sdk + ":" + host + ":" + scriptSha256, files: {[windows ? "dotnet.exe" : "dotnet"]: createHash("sha256").update(readFileSync(executable)).digest("hex")}}) + "\n");
    signal.throwIfAborted();
    try {
      renameSync(payload, directory);
    } catch (error) {
      if (!existsSync(directory)) throw error;
    }
  } finally {
    rmSync(temporary, {recursive: true, force: true});
  }
}

/** 🔧️ Runs `repo tools setup toolchains|all`: provisions every pinned tool of the selection that differs from its pin below the tools partition (or through its owning installer), then verifies it again; returns the final report. */
export async function runToolchainsSetupV1(root: string, invocation: ScriptInvocation, control: ResourceLeaseControl, names: readonly string[], excluded: readonly string[], offline: boolean): Promise<readonly ToolReportV1[]> {
  const context = systemToolContextV1(root, control), host = toolchainHostV1(), rows = selectToolRowsV1(names, excluded, context.environment), port = systemNativeToolPortV1(root, offline);
  const entry = (label: string): ResourceLeaseControl => createIssuedResourceControlV1(invocation, label, 0);
  const inStore = <T>(body: (store: RepositoryToolingStoreV1, actual: ResourceLeaseControl) => Promise<T>): Promise<T> => withRepositoryToolingStoreV1(root, invocation, entry("cache:entry"), async (store, actual) => {
    requireNativeStorageReservationV1(store.storage, store.control);
    await store.recheck();
    return body(store, actual);
  });
  const pinned = (extra: Record<string, string | undefined> = {}): NodeJS.ProcessEnv => withPinnedNativePathV1({...scriptResourceProcessEnvironmentV1(control, process.env), ...extra}, host, context.toolsRoot, ["go", "uv", "cmake", "ninja"]) as NodeJS.ProcessEnv;
  const execute = (command: string, args: string[], env: NodeJS.ProcessEnv, signal: AbortSignal): Promise<void> => runTool(command, args, root, signal, false, env, repositoryCargoPreparationStorageV1(root)).then(() => undefined);
  const provision: ToolProvisionerV1 = async (row, pin, signal) => {
    const setup = row.setup;
    if (setup.kind === "native") await inStore(store => provisionNativeToolsV1([setup.tool], store.root, port, signal));
    else if (setup.kind === "bun") await inStore((store, actual) => prepareBun(root, pin, store, actual));
    else if (setup.kind === "binaryen") await inStore(store => prepareBinaryen(root, store));
    else if (setup.kind === "script") await execute(process.execPath, [join(root, ...setup.script.split("/")), ...setup.prepare], pinned(), signal);
    else if (setup.kind === "python") {
      const uv = locateToolV1(toolRowV1("uv"), NATIVE_TOOLCHAINS_V1.python.uv, context)!, python = NATIVE_TOOLCHAINS_V1.python;
      await execute(uv, ["python", "install", pin], pinned({...python.variables, UV_PYTHON_INSTALL_DIR: pythonInterpreterDirectoryV1(context.toolsRoot), UV_CACHE_DIR: join(repositoryCachePartitionRootV1(root, "dependencies"), ...python.layout.cache.split("/"))}), signal);
    } else if (setup.kind === "dotnet") {
      const installer = dotnetInstallerRowV1(process.platform, process.arch);
      if (installer === undefined) throw new Error(toolSetupMessageV1("installer", {tool: "dotnet", host}, context.environment));
      await installDotnetSdkV1(pin, dotnetSdkDirectoryV1(context.toolsRoot, pin, host, installer.sha256), installer.sha256, installer, host, context.environment, port, signal, (command, args, env) => execute(command, args, env, signal));
    } else if (setup.kind === "rust") await execute("rustup", ["toolchain", "install"], pinned(), signal);
  };
  return setupToolsV1(rows, context, provision, control.signal, line => console.log(line));
}
