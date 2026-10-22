import {createHash} from "node:crypto";
import {chmodSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, renameSync, rmSync, writeFileSync} from "node:fs";
import {open} from "node:fs/promises";
import {delimiter, dirname, join, relative, sep} from "node:path";
import {
  NATIVE_TOOLCHAINS_V1,
  extractedLinkEscapesV1,
  nativeDistributionV1,
  nativeToolBinaryDirectoryV1,
  nativeToolDirectoryV1,
  nativeToolIdentityV1,
  nativeToolSpecV1,
  nativeToolPathV1,
  nativeToolUrlV1,
  toolchainHostV1,
  toolchainMessageV1,
  type NativeToolV1,
  type ToolchainHostV1,
} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";

/** 🔌️ Everything provisioning touches outside the file system: the network, child programs, the platform, the offline flag, the language environment and the log; replaceable so every host and every failure is testable on any host. */
export type NativeToolPortV1 = Readonly<{
  platform: NodeJS.Platform;
  offline: boolean;
  environment: Readonly<Record<string, string | undefined>>;
  fetch(url: string, signal: AbortSignal): Promise<Response>;
  run(command: string, args: readonly string[], cwd: string, signal: AbortSignal): Promise<void>;
  capture(command: string, args: readonly string[], cwd: string, signal: AbortSignal): Promise<string>;
  log(text: string): void;
}>;
/** 🧾️ The receipt published next to an installed tool: who owns it, which pin it is and the digest of every executable. */
export type NativeToolReceiptV1 = Readonly<{owner: string; identity: string; files: Readonly<Record<string, string>>}>;

const owner = "workspace:native-tools", ceiling = Object.freeze({entries: 200000, depth: 32});
const digest = (path: string): string => createHash("sha256").update(readFileSync(path)).digest("hex");
const regular = (path: string): boolean => {
  const stat = lstatSync(path);
  return stat.isFile() && !stat.isSymbolicLink();
};

/** 🛣️ Rejects a substituted directory between a path and the tools root; the ancestors of the root belong to the host, whose own links (`/var`, `/home`) are legitimate. */
function plainDirectoryChain(path: string, root: string): void {
  for (let current = path; current.length >= root.length && (current === root || current.startsWith(root + sep)); current = dirname(current)) {
    try {
      const stat = lstatSync(current);
      if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(toolchainMessageV1("directory", {path: current}));
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    if (current === root) return;
  }
}

/** 🔐️ Checks the executables of one installed pin against its receipt and returns the directory that holds them. */
export function preparedNativeToolV1(tool: NativeToolV1, host: ToolchainHostV1, toolsRoot: string, environment: Readonly<Record<string, string | undefined>> = process.env): string {
  const directory = nativeToolDirectoryV1(tool, host, toolsRoot), row = nativeDistributionV1(tool, host), version = nativeToolSpecV1(tool).version;
  plainDirectoryChain(directory, toolsRoot);
  let receipt: NativeToolReceiptV1;
  try {
    receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") throw new Error(toolchainMessageV1("missing", {tool, version, target: tool === "go" ? "deps-go" : "deps-cpp"}, environment));
    throw error;
  }
  const base = row.binaryDirectory === "." ? "" : row.binaryDirectory + "/";
  if (receipt.owner !== owner || receipt.identity !== nativeToolIdentityV1(tool, host)) throw new Error(toolchainMessageV1("receipt", {tool, version, path: ".toolchain.json"}, environment));
  for (const name of row.binaries) {
    const path = join(directory, ...(base + name).split("/")), recorded = receipt.files[base + name];
    if (recorded === undefined || !regular(path) || digest(path) !== recorded) throw new Error(toolchainMessageV1("receipt", {tool, version, path: base + name}, environment));
  }
  return nativeToolBinaryDirectoryV1(tool, host, toolsRoot);
}

/** 🧪️ Walks an extracted tree under a finite ceiling and refuses a link that leaves it. */
export function verifyExtractedTreeV1(root: string): number {
  let count = 0;
  const visit = (directory: string, depth: number): void => {
    if (depth > ceiling.depth) throw new Error(toolchainMessageV1("depth", {}));
    for (const entry of readdirSync(directory, {withFileTypes: true})) {
      const path = join(directory, entry.name);
      if (++count > ceiling.entries) throw new Error(toolchainMessageV1("size", {}));
      const stat = lstatSync(path);
      if (stat.isSymbolicLink()) {
        const target = readlinkSync(path);
        if (extractedLinkEscapesV1(relative(root, path), target)) throw new Error(toolchainMessageV1("link", {path: relative(root, path), target}));
      } else if (stat.isDirectory()) visit(path, depth + 1);
      else if (!stat.isFile()) throw new Error(toolchainMessageV1("special", {path: relative(root, path)}));
    }
  };
  visit(root, 0);
  return count;
}

/** 📥️ Downloads one pinned archive into a private directory under its pinned size and SHA-256 and returns its path. */
async function download(tool: NativeToolV1, host: ToolchainHostV1, directory: string, port: NativeToolPortV1, signal: AbortSignal): Promise<string> {
  const row = nativeDistributionV1(tool, host), version = nativeToolSpecV1(tool).version, values = {tool, version, host, bytes: row.bytes};
  if (port.offline) throw new Error(toolchainMessageV1("offline", values, port.environment));
  port.log(toolchainMessageV1("download", values, port.environment));
  const response = await port.fetch(nativeToolUrlV1(tool, host), signal);
  if (!response.ok || !response.body) throw new Error(toolchainMessageV1("http", {status: response.status, url: nativeToolUrlV1(tool, host)}, port.environment));
  const archive = join(directory, row.archive), file = await open(archive, "wx"), hash = createHash("sha256"), reader = response.body.getReader();
  let bytes = 0;
  try {
    for (let read = await reader.read(); !read.done; read = await reader.read()) {
      signal.throwIfAborted();
      bytes += read.value.byteLength;
      if (bytes > row.bytes) throw new Error(toolchainMessageV1("checksum", values, port.environment));
      hash.update(read.value);
      await file.writeFile(read.value);
    }
  } catch (error) {
    await reader.cancel().catch(() => undefined);
    throw error;
  } finally {
    await file.close();
  }
  if (bytes !== row.bytes || hash.digest("hex") !== row.sha256) throw new Error(toolchainMessageV1("checksum", values, port.environment));
  return archive;
}

/** 📦️ Provisions one pinned tool: download under the pinned checksum, unpack with the program every operating system ships, verify the tree, record the executables in a receipt and publish the directory by one rename; a second call finds the receipt and does nothing. */
export async function prepareNativeToolV1(tool: NativeToolV1, host: ToolchainHostV1, toolsRoot: string, port: NativeToolPortV1, signal: AbortSignal): Promise<string> {
  const directory = nativeToolDirectoryV1(tool, host, toolsRoot), parent = dirname(directory), row = nativeDistributionV1(tool, host), version = nativeToolSpecV1(tool).version, values = {tool, version, host};
  plainDirectoryChain(parent, toolsRoot);
  try {
    lstatSync(directory);
    return preparedNativeToolV1(tool, host, toolsRoot, port.environment);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
  }
  mkdirSync(parent, {recursive: true});
  const temporary = mkdtempSync(join(parent, ".prepare-")), progress = setInterval(() => port.log(toolchainMessageV1("download", {...values, bytes: row.bytes}, port.environment)), 10000);
  try {
    const archive = await download(tool, host, temporary, port, signal), extracted = join(temporary, "extracted");
    mkdirSync(extracted);
    const [program, ...template] = (NATIVE_TOOLCHAINS_V1.extract[row.format] as Readonly<Record<string, readonly string[]>>)[port.platform]!;
    await port.run(program!, template.map(part => part.replace("{archive}", archive).replace("{destination}", extracted)), temporary, signal);
    const payload = row.prefix === "" ? extracted : join(extracted, row.prefix);
    if (!lstatSync(payload).isDirectory()) throw new Error(toolchainMessageV1("member", {...values, member: row.prefix}, port.environment));
    verifyExtractedTreeV1(payload);
    const base = row.binaryDirectory === "." ? "" : row.binaryDirectory + "/", files: Record<string, string> = {};
    for (const name of row.binaries) {
      const path = join(payload, ...(base + name).split("/"));
      let present = true;
      try {
        present = regular(path);
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
        present = false;
      }
      if (!present) throw new Error(toolchainMessageV1("binary", {...values, binary: name}, port.environment));
      if (port.platform !== "win32") chmodSync(path, 0o755);
      files[base + name] = digest(path);
    }
    const main = join(payload, ...(base + row.binaries[0]!).split("/")), reported = await port.capture(main, [...nativeToolSpecV1(tool).versionArguments], temporary, signal);
    if (!reported.includes(version)) throw new Error(toolchainMessageV1("pin", {tool, actual: reported.trim().split(/\r?\n/u)[0] ?? "", expected: version, source: nativeToolSpecV1(tool).versionArguments.join(" ")}, port.environment));
    writeFileSync(join(payload, ".toolchain.json"), JSON.stringify({owner, identity: nativeToolIdentityV1(tool, host), files}) + "\n");
    signal.throwIfAborted();
    try {
      renameSync(payload, directory);
    } catch (error) {
      if (!["EEXIST", "ENOTEMPTY", "EPERM", "EACCES"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error;
      try {
        lstatSync(directory);
      } catch {
        throw error;
      }
    }
    return preparedNativeToolV1(tool, host, toolsRoot, port.environment);
  } finally {
    clearInterval(progress);
    rmSync(temporary, {recursive: true, force: true});
  }
}

/** 🧭️ Puts the pinned CMake and Ninja first on the `PATH` of an environment, under the spelling of the variable that environment already uses. */
export function withPinnedNativePathV1(environment: Readonly<Record<string, string | undefined>>, host: ToolchainHostV1, toolsRoot: string): Readonly<Record<string, string | undefined>> {
  const key = Object.keys(environment).find(name => name.toLowerCase() === "path") ?? "PATH";
  return Object.freeze({...environment, [key]: [...nativeToolPathV1(host, toolsRoot), ...(environment[key] ?? "").split(delimiter).filter(Boolean)].join(delimiter)});
}

/** 🏁️ Provisions the pinned CMake and Ninja of the running host (or finds them) and returns the environment that selects them. */
export async function pinnedNativeEnvironmentV1(environment: Readonly<Record<string, string | undefined>>, toolsRoot: string, port: NativeToolPortV1, signal: AbortSignal, provision: boolean, host: ToolchainHostV1 = toolchainHostV1()): Promise<Readonly<Record<string, string | undefined>>> {
  for (const tool of ["cmake", "ninja"] as const) {
    if (provision) await prepareNativeToolV1(tool, host, toolsRoot, port, signal);
    else preparedNativeToolV1(tool, host, toolsRoot, port.environment);
  }
  return withPinnedNativePathV1(environment, host, toolsRoot);
}

/** 🐹️ Provisions the pinned Go of the running host (or finds it) and returns the environment every Go command of the repository runs under: the pinned `bin` first on `PATH`, `GOROOT` the pinned directory and the local toolchain only, so nothing depends on a Go found on the machine. */
export async function pinnedGoEnvironmentV1(environment: Readonly<Record<string, string | undefined>>, toolsRoot: string, port: NativeToolPortV1, signal: AbortSignal, provision: boolean = true, host: ToolchainHostV1 = toolchainHostV1()): Promise<Readonly<Record<string, string | undefined>>> {
  if (provision) await prepareNativeToolV1("go", host, toolsRoot, port, signal);
  else preparedNativeToolV1("go", host, toolsRoot, port.environment);
  const key = Object.keys(environment).find(name => name.toLowerCase() === "path") ?? "PATH";
  return Object.freeze({...environment, ...NATIVE_TOOLCHAINS_V1.go.variables, GOROOT: nativeToolDirectoryV1("go", host, toolsRoot), [key]: [nativeToolBinaryDirectoryV1("go", host, toolsRoot), ...(environment[key] ?? "").split(delimiter).filter(Boolean)].join(delimiter)});
}
