import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { stageArtifacts } from "../📤️publication/🟦️.ts";
import { terminateOwnedChildTree } from "../../🪓️termination/🟦️.ts";
import { startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";
import { chmodSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep, isAbsolute, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { createInterface } from "node:readline";

import { acquireCargoBuildLeaseV1 } from "./🔒️lease/🟦️.ts";


/** ✍️ Re-signs a native executable on macOS, ad-hoc, and does nothing anywhere else.
 *
 * A Mach-O binary carries its code signature inside the file, and the kernel validates it against
 * the bytes on disk. A copied or rewritten executable whose signature no longer matches is not
 * rejected with a message — it is `SIGKILL`ed without one. Signing every distribution copy ad-hoc
 * (`--sign -`) is what keeps that from happening. This is *not* Developer ID signing or
 * notarization: a binary signed this way still trips Gatekeeper when it arrives from the internet
 * on someone else's Mac, which needs a signing identity this repository does not have. */
export function signExecutableForDistribution(binary: string): void {
  if (process.platform !== "darwin") return;
  const signed = spawnSync("codesign", ["--force", "--sign", "-", binary], { stdio: ["ignore", "inherit", "inherit"] });
  if (signed.status !== 0) throw new Error(`codesign refused ${binary} (status ${signed.status ?? "unknown"})`);
}

/** 📥️ Places an executable at `destination`: remove, copy, then sign — never an overwrite in place.
 *
 * Writing over a running or previously-signed Mach-O binary is the exact shape that produces a
 * silent `SIGKILL` on the next launch, so the old inode is unlinked first and the fresh copy is
 * signed afterwards. */
export function installExecutable(source: string, destination: string): void {
  mkdirSync(dirname(destination), { recursive: true });
  rmSync(destination, { force: true });
  copyFileSync(source, destination);
  chmodSync(destination, lstatSync(source).mode & 0o777);
  signExecutableForDistribution(destination);
}

/** 🚚️ Packages one already-built native executable as a versioned, checksummed local tarball.
 *
 * This is the last mile a release build was missing: a compiled binary sitting in a project's
 * `dist/` is not something an operator can be handed. It produces
 * `<output>/<name>-<version>-<platform>-<arch>.tar.gz` plus a `.sha256` next to it, and **uploads
 * nothing anywhere** — where the artifact goes afterwards is a deployment decision this repository
 * does not take. Returns the tarball path. */
export function packageNativeRelease(options: { readonly binary: string; readonly name: string; readonly version: string; readonly output: string; readonly platform?: string; readonly arch?: string }): string {
  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  if (!lstatSync(options.binary).isFile()) throw new Error(`no release binary at ${options.binary}`);
  mkdirSync(options.output, { recursive: true });
  const filename = `${options.name}-${options.version}-${platform}-${arch}.tar.gz`;
  const tarball = join(options.output, filename);
  const payload = mkdtempSync(join(options.output, "payload-"));
  try {
    installExecutable(options.binary, join(payload, `${options.name}-${options.version}`, options.name));
    rmSync(tarball, { force: true });
    const archived = spawnSync("tar", ["-czf", tarball, "-C", payload, `${options.name}-${options.version}`], { stdio: ["ignore", "inherit", "inherit"] });
    if (archived.status !== 0) throw new Error(`tar refused ${tarball} (status ${archived.status ?? "unknown"})`);
  } finally {
    rmSync(payload, { recursive: true, force: true });
  }
  const digest = createHash("sha256").update(readFileSync(tarball)).digest("hex");
  writeFileSync(`${tarball}.sha256`, `${digest}  ${filename}\n`);
  console.log(`[publish] ${tarball}\n[publish] sha256 ${digest}`);
  return tarball;
}

/** 🪢️ The process a Cargo build belongs to, by pid. A launcher that builds on someone's behalf (the
 * MCP gateway staging its own binary inside a client's `initialize`) names itself here, and the build
 * stops the moment that owner is gone — killed by its client, timed out by a gate — instead of
 * outliving it as an orphan that holds the shared build-dir locks. */
export const CARGO_BUILD_OWNER_PID_ENV = "SEMIO_BUILD_OWNER_PID";

/** 🪢️ Whether the owner named by {@link CARGO_BUILD_OWNER_PID_ENV} is still alive; `true` when none is named. */
export function cargoBuildOwnerAliveV1(env: Readonly<Record<string,string|undefined>> = process.env): boolean {
  const owner = Number(env[CARGO_BUILD_OWNER_PID_ENV] ?? "");
  if (!Number.isSafeInteger(owner) || owner <= 0) return true;
  try {
    process.kill(owner, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🧾️ Schema id of the sources record a staged Cargo executable can carry: every source file Cargo
 * compiled it from (its own dep-info) and when the build that produced it started. */
export const CARGO_BINARY_SOURCES_SCHEMA_V1 = "semio.cargo.binary-sources/v1";

/** 🧾️ One staged executable's sources record. */
export interface CargoBinarySourcesV1 {
  readonly schema: typeof CARGO_BINARY_SOURCES_SCHEMA_V1;
  readonly builtAtMs: number;
  readonly sources: readonly string[];
}

/** 📜️ The source files a Cargo dep-info file (`<artifact>.d`) names for its target, with Cargo's
 * `\ ` space escape undone. Only the first `target: deps` rule counts; the empty per-dependency
 * rules Cargo appends after it carry nothing. */
export function cargoDepInfoSourcesV1(text: string): readonly string[] {
  const rule = text.split(/\r?\n/u).find((line) => line.includes(": ")) ?? "";
  return rule
    .slice(rule.indexOf(": ") + 2)
    .split(/(?<!\\) /u)
    .filter((entry) => entry.length > 0)
    .map((entry) => entry.replace(/\\ /gu, " "));
}

/** 🧾️ Reads a sources record, answering `null` for anything that is not exactly one. */
export function parseCargoBinarySourcesV1(text: string): CargoBinarySourcesV1 | null {
  try {
    const value = JSON.parse(text) as { schema?: unknown; builtAtMs?: unknown; sources?: unknown };
    if (value.schema !== CARGO_BINARY_SOURCES_SCHEMA_V1 || typeof value.builtAtMs !== "number" || !Number.isFinite(value.builtAtMs) || !Array.isArray(value.sources) || value.sources.length === 0 || !value.sources.every((source) => typeof source === "string" && source.length > 0)) return null;
    return { schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs: value.builtAtMs, sources: value.sources as string[] };
  } catch {
    return null;
  }
}

/** 🕰️ Whether a staged executable is still what its sources build: every source it was compiled from
 * still exists and none was modified after its build started — Cargo's own freshness rule, over the
 * whole dependency closure rather than one crate. `modifiedAtMs` answers `null` for a missing file. A
 * touched but unchanged file reads as `changed`, which costs a no-op Cargo build and never serves a
 * stale binary. */
export function cargoBinarySourcesFreshnessV1(record: CargoBinarySourcesV1, modifiedAtMs: (path: string) => number | null): { readonly fresh: boolean; readonly changed: string | null } {
  for (const source of record.sources) {
    const modified = modifiedAtMs(source);
    if (modified === null || modified > record.builtAtMs) return { fresh: false, changed: source };
  }
  return { fresh: true, changed: null };
}

/** 🏗️ Supplies exact caller-owned compiler storage and execution limits. */
export type CargoArtifactBuildPolicyV1 = Readonly<{ version: 1; cwd: string; buildDirectory: string; leaseDirectory: string; captureDirectory: string; budgetMs: number }>;
const artifactPolicySchema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
function admitArtifactPolicy(value: unknown): CargoArtifactBuildPolicyV1 {
  const errors = validateJsonSchemaSubset(artifactPolicySchema, value);
  if (errors.length) throw Error(`Invalid Cargo artifact policy: ${errors.join("; ")}`);
  return value as CargoArtifactBuildPolicyV1;
}
/** 🔐️ Admits only an explicit compiler policy for the selected source owner. */
export function readCargoArtifactBuildPolicyV1(environment: Readonly<Record<string,string|undefined>>, cwd: string): CargoArtifactBuildPolicyV1 {
  if (!environment.SEMIO_CARGO_ARTIFACT_POLICY) throw Error("Explicit Cargo artifact policy required");
  const policy = admitArtifactPolicy(JSON.parse(environment.SEMIO_CARGO_ARTIFACT_POLICY));
  if (resolve(policy.cwd) !== resolve(cwd)) throw Error("Cargo artifact policy belongs to a different owner");
  return policy;
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state.
 * `sourcesRecord` names a {@link CargoBinarySourcesV1} file staged beside the selected executable, in
 * the same atomic publication, so a consumer can tell a stale executable from a fresh one without
 * running Cargo. Cargo's stderr is piped and forwarded, never inherited: Bun marks its own stderr
 * `O_NONBLOCK` once written, an inherited pipe shares that flag, and a Cargo burst (replayed warnings
 * of fresh units) then fails with `EAGAIN` as soon as a slow reader lets the 64 KiB pipe fill — the
 * build dies with its diagnostics cut mid-line (ticket 26/09/23 W4, `wp-w4/w4-nonblock-probe.ts`). */
export type CargoArtifactBuildOptionsV1 = { readonly command?: "build" | "rustc"; readonly output?: string; readonly sourcesRecord?: string; readonly validate?: (files: ReadonlyMap<string, string>) => void; readonly signal?: AbortSignal; readonly environment?: Readonly<Record<string,string|undefined>>; readonly commandPort?: Readonly<{ command: string; args: readonly string[] }> };

/** 🚦️ Holds the shared profile lease through compiler shutdown and artifact capture, including queue cancellation. */
export async function buildCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1 = {}): Promise<void> {
  policy = admitArtifactPolicy(policy);
  if (!isAbsolute(manifest)) throw Error("Cargo artifact manifest must be absolute");
  const budget = policy.budgetMs;
  if (!Number.isFinite(budget) || budget < 0) throw Error("Invalid Cargo build budget");
  const controller = new AbortController();
  const abort = () => controller.abort();
  options.signal?.addEventListener("abort", abort, { once: true });
  process.once("SIGINT", abort);
  process.once("SIGTERM", abort);
  const observeOwner = () => { if (!cargoBuildOwnerAliveV1(options.environment ?? process.env)) controller.abort(); };
  observeOwner();
  const ownerWatch = setInterval(observeOwner, 1000);
  const expiry = budget > 0 ? setTimeout(abort, budget) : undefined;
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    if (options.signal?.aborted) abort();
    lease = await acquireCargoBuildLeaseV1({ directory: policy.leaseDirectory, buildDirectory: policy.buildDirectory, args, signal: controller.signal });
    controller.signal.throwIfAborted();
    await captureCargoArtifacts(manifest, args, policy, { ...options, signal: controller.signal });
  } finally {
    try { lease?.release(); }
    finally {
      clearInterval(ownerWatch);
      if (expiry) clearTimeout(expiry);
      options.signal?.removeEventListener("abort", abort);
      process.removeListener("SIGINT", abort);
      process.removeListener("SIGTERM", abort);
    }
  }
}

/** 📋️ Retains actual completed Cargo compiler, dep-info and resource observations without changing build or staging selection. */
export function writeCompletedCargoInvocationProvenanceV1(receiptPath: string, invocation: { manifest: string; cwd: string; command: string; args: string[]; buildDirectory: string | null; builtAtMs: number; status: number; cancelled: boolean; units: any[]; buildScripts: any[] }, stagedPaths: ReadonlyMap<string, string> = new Map(), cargoHome: string = join(homedir(), ".cargo")): void {
  const sha256 = (file: string): string | null => existsSync(file) && lstatSync(file).isFile() ? createHash("sha256").update(readFileSync(file)).digest("hex") : null;
        for (const unit of invocation.units) {
          const candidates = [...new Set((unit.message.filenames ?? []).flatMap((file: string) => [file.replace(/\.(?:rlib|rmeta|so|dylib|dll|lib|wasm|exe)$/u, "") + ".d", join(dirname(file), basename(file).replace(/^lib/u, "").replace(/\.(?:rlib|rmeta|so|dylib|dll|lib|wasm|exe)$/u, "") + ".d")]))] as string[];
          unit.depInfo = candidates.filter(file => existsSync(file) && lstatSync(file).isFile()).map(path => {
            const text = readFileSync(path, "utf8"), sources = cargoDepInfoSourcesV1(text), relativeSources = sources.filter(source => !isAbsolute(source)).map(source => normalize(source)), targetSource = resolve(unit.message.target.src_path);
            const owners = new Set<string>();
            for (const manifest of [invocation.manifest, unit.message.manifest_path].filter((path): path is string => typeof path === "string")) for (let owner = dirname(resolve(manifest));;) {
              if (existsSync(join(owner, "Cargo.toml"))) owners.add(owner);
              const parent = dirname(owner); if (parent === owner) break; owner = parent;
            }
            const bases = [...owners].filter(base => relativeSources.some(source => resolve(base, source) === targetSource) && sources.every(source => { const path = resolve(base, source); return existsSync(path) && lstatSync(path).isFile(); }));
            return { path, text, baseDirectory: relativeSources.length === 0 ? invocation.cwd : bases.length === 1 ? bases[0] : null };
          });
          unit.inputs = [...new Set(unit.depInfo.flatMap((row: { text: string; baseDirectory: string | null }) => cargoDepInfoSourcesV1(row.text).filter(path => isAbsolute(path) || row.baseDirectory !== null).map(path => resolve(row.baseDirectory ?? invocation.cwd, path))))].map(path => ({ path, sha256: sha256(path as string) }));
          unit.observedAtMs = Date.now();
          unit.artifacts = (unit.message.filenames ?? []).map((path: string) => {
            const stagedPath = stagedPaths.get(path);
            return { path, sha256: sha256(path), ...(stagedPath ? { stagedPath, stagedSha256: sha256(stagedPath) } : {}) };
          });
        }
        const buildResources = invocation.buildScripts.flatMap(build => {
          const path = join(build.out_dir, "semio-runtime-resource-inputs.jsonl");
          if (!existsSync(path)) return [];
          const text = readFileSync(path, "utf8"), resources = text.split(/\r?\n/u).filter(Boolean).map(line => {
            const input = JSON.parse(line);
            return { input, sha256: sha256(input.path), outputSha256: typeof input.output === "string" ? sha256(input.output) : null, ...(input.kind === "directory" ? { observedEntries: readdirSync(input.path).map(entry => join(input.path, entry)).sort() } : {}) };
          });
          return [{ package_id: build.package_id, out_dir: build.out_dir, path, text, sha256: sha256(path), observedAtMs: Date.now(), resources }];
        });
  const manifests = new Set<string>([invocation.manifest, ...invocation.units.map(unit => unit.message.manifest_path).filter((path): path is string => typeof path === "string")]), inputPaths = new Set<string>(manifests), visited = new Set<string>();
  for (let path of [invocation.cwd, ...[...manifests].map(path => dirname(path))]) for (;;) {
    if (visited.has(path)) break;
    visited.add(path);
    for (const name of ["Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".cargo/config", "rust-toolchain.toml", "rust-toolchain"]) inputPaths.add(join(path, name));
    const parent = dirname(path); if (parent === path) break; path = parent;
  }
  for (const name of ["config.toml", "config"]) inputPaths.add(join(cargoHome, name));
  const invocationInputs = [...inputPaths].sort().map(path => ({ path, sha256: sha256(path) })), text = JSON.stringify({ version: 1, ...invocation, observedAtMs: Date.now(), invocationInputs, buildResources }) + "\n";
  for (const path of new Set([receiptPath, ...(invocation.buildDirectory ? [join(invocation.buildDirectory, "semio-cargo-provenance", basename(receiptPath))] : [])])) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, text); }
}

async function captureCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1): Promise<void> {
  options.signal?.throwIfAborted();
  const path = resolve(manifest);
  const sourceRoot = dirname(path);
  const staging = resolve(sourceRoot, options.output ?? "dist/build");
  if (!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
  const captureParent = policy.captureDirectory;
  mkdirSync(captureParent, { recursive: true });
  const capture = mkdtempSync(join(captureParent, "cargo-artifacts-"));
  const owner = relative(policy.cwd, path).split(sep).join("/");
  const files = new Map<string, string>();
  const dependencies = new Map<string, string>();
  const provenanceRoot = (options.environment ?? process.env).SEMIO_TEST_ARTIFACT_DIR;
  const units: any[] = [], buildScripts: any[] = [], stagedNames = new Map<string, string>();
  let hasLibrary = false;
  let primaryExecutable: string | undefined;
  let cancelled = false;
  let forceKill: ReturnType<typeof setTimeout> | undefined;
  const delimiter = args.indexOf("--"),
    compilerArgs = delimiter < 0 ? [] : args.slice(delimiter),
    cargoArgs = delimiter < 0 ? args : args.slice(0, delimiter);
  const builtAtMs = Date.now();
  const commandPort = options.commandPort ?? { command: "cargo", args: [] };
  const child = spawn(commandPort.command, [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], {
    cwd: policy.cwd,
    env: { ...(options.environment ?? process.env), CARGO_TARGET_DIR: join(capture, "target"), CARGO_BUILD_BUILD_DIR: policy.buildDirectory },
    detached: process.platform !== "win32",
    stdio: ["inherit", "pipe", "pipe"],
  });
  child.stderr!.pipe(process.stderr, { end: false });
  const cancel = (): void => {
    cancelled = true;
    terminateOwnedChildTree(child);
  };
  options.signal?.addEventListener("abort", cancel, { once: true });
  if (options.signal?.aborted) cancel();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const stopProgress = startNativeProgress(`artifact-rust:${owner}:build`);
  const status = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(error.message);
      accept(1);
    });
    child.once("close", (code) => accept(code ?? 1));
  });
  try {
    try {
      try {
        for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
          let message;
          try {
            message = JSON.parse(line);
          } catch {
            process.stdout.write(line + "\n");
            continue;
          }
          if ((message.reason === "compiler-artifact" || message.reason === "build-script-executed") && (options.environment ?? process.env).SEMIO_TEST_ARTIFACT_DIR) process.stdout.write(line + "\n");
          if (message.reason === "build-script-executed") buildScripts.push(message);
          if (message.reason === "compiler-artifact") units.push({ message });
          if (message.reason !== "compiler-artifact" || message.target?.kind?.includes("custom-build")) continue;
          const packageUrl = message.package_id?.split("#")[0]?.replace(/^path\+/, "");
          const bin = args.indexOf("--bin"),
            example = args.indexOf("--example");
          const selected =
            bin >= 0
              ? message.target?.kind?.includes("bin") && message.target.name === args[bin + 1]
              : example >= 0
                ? message.target?.kind?.includes("example") && message.target.name === args[example + 1]
                : args.includes("--bins")
                  ? message.target?.kind?.includes("bin")
                  : true;
          const primary = selected && packageUrl !== undefined && packageUrl.startsWith("file:") && resolve(fileURLToPath(packageUrl)) === sourceRoot;
          if (primary && typeof message.executable === "string") primaryExecutable = message.executable;
          for (const file of message.filenames ?? []) {
            if (file.endsWith(".d")) continue;
            const library = primary && file.endsWith(".rmeta") ? message.filenames.find((candidate: string) => candidate.endsWith(".rlib")) : undefined;
            const name = (library ? library.replace(/\.rlib$/, ".rmeta") : file).split(/[\\/]/).at(-1)!;
            const key = primary ? name : /\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file) ? `deps/${name}` : undefined;
            if (!key) continue;
            stagedNames.set(file, key);
            if (!primary) {
              dependencies.set(key, file);
              continue;
            }
            const captured = join(capture, key);
            mkdirSync(dirname(captured), { recursive: true });
            copyFileSync(file, captured);
            chmodSync(captured, lstatSync(file).mode & 0o777);
            files.set(key, captured);
            hasLibrary ||= file.endsWith(".rlib");
          }
        }
      } catch (error) {
        cancel();
        await status;
        throw error;
      }
      if ((await status) !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
    } finally {
      stopProgress();
      if (forceKill) clearTimeout(forceKill);
      options.signal?.removeEventListener("abort", cancel);
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
    if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
    if (hasLibrary)
      for (const [name, file] of dependencies) {
        const captured = join(capture, name);
        mkdirSync(dirname(captured), { recursive: true });
        copyFileSync(file, captured);
        chmodSync(captured, lstatSync(file).mode & 0o777);
        files.set(name, captured);
      }
    if (options.sourcesRecord !== undefined) {
      if (primaryExecutable === undefined) throw new Error(`Cargo emitted no executable to record the sources of: ${owner}`);
      const record: CargoBinarySourcesV1 = { schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs, sources: cargoDepInfoSourcesV1(readFileSync(`${primaryExecutable.replace(/\.exe$/u, "")}.d`, "utf8")) };
      if (record.sources.length === 0) throw new Error(`Cargo dep-info names no sources for ${owner}`);
      const captured = join(capture, options.sourcesRecord);
      writeFileSync(captured, `${JSON.stringify(record)}\n`);
      files.set(options.sourcesRecord, captured);
    }
    options.validate?.(files);
    await stageArtifacts(staging, owner, files, { signal: options.signal, leaseDirectory: policy.leaseDirectory });
    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(policy.cwd, staging).split(sep).join("/")}`);
  } finally {
    try {
      {
        writeCompletedCargoInvocationProvenanceV1(join(provenanceRoot ?? join(policy.buildDirectory, "semio-cargo-provenance"), `cargo-unit-provenance-${basename(capture)}.json`), { manifest: path, cwd: policy.cwd, command: commandPort.command, args: [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], buildDirectory: policy.buildDirectory, builtAtMs, status: await status, cancelled, units, buildScripts }, new Map([...stagedNames].filter(([, key]) => files.has(key)).map(([path, key]) => [path, join(staging, key)])), (options.environment ?? process.env).CARGO_HOME);
      }
    } finally {
      rmSync(capture, { recursive: true, force: true });
    }
  }
}
