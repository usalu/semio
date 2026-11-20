import { setTimeout as delay } from "node:timers/promises";
import { chmodSync, closeSync, constants, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readdirSync, readSync, realpathSync, rmdirSync, statfsSync, unlinkSync, writeFileSync } from "node:fs";
import { userInfo } from "node:os";
import { join, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import schema from "./constrained.json";
import readySchema from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import capacitySchema from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json";
import { hostCapacityDirectoryV1, newHostCapacityIdV1, readLiveHostCapacityReservationV1, withHostCapacityV1, type HostCapacityHandleV1, type HostCapacityOptionsV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1, createCapacityControlV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";
import { casPartitionBudgetV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🌡️partition/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { nativeImageHdiutilCommandV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🛠️hdiutil/🟦️.ts";
import { nativeImageInventoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📥️information/🟦️.ts";
import { readSliceLabOptionsV1, SLICE_LAB_POLICY_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🟦️.ts";

const policy = schema["x-policy"];
const GiB = 1024 ** 3;
const ticket = resolve(fileURLToPath(new URL(".", import.meta.url)));
const live = resolve(ticket, "../".repeat(7));
const generated = join(ticket, "🗑️generated");
const imageBase = "lab.sparseimage";
const lab = fileURLToPath(new URL("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/📜️script.ts", import.meta.url));
const mountTemplate = "/private/tmp/lv-XXXXXX";
const cloneSuffix = `${SLICE_LAB_POLICY_V1.prefix}-XXXXXX/m1/${SLICE_LAB_POLICY_V1.frozen.directory}`;
const scope = { maximumBytes: 64 * 1024 ** 2, maximumLines: 2000000 };
const informationScope = { maximumBytes: 1024 ** 2, maximumLines: 20000 };
type Options = { mode: "preflight" | "run"; imageGiB: number; timeoutSeconds: number };
type Identity = Readonly<{ dev: bigint; ino: bigint }>;
type Environment = Readonly<Record<string, string | undefined>>;
type ObjectRecord = Record<string, unknown>;

export function constrainedOptions(argv: readonly string[]): Options {
  const [mode, ...flags] = argv;
  if (mode !== "preflight" && mode !== "run") throw Error("Expected preflight or run");
  const values = { imageGiB: schema.properties.imageGiB.default, timeoutSeconds: schema.properties.timeoutSeconds.default };
  const seen = new Set<string>();
  for (const flag of flags) {
    const option = /^--(image-gib|timeout)=([1-9][0-9]*)$/u.exec(flag);
    if (!option || seen.has(option[1]!)) throw Error("Unknown or repeated constrained lab option");
    seen.add(option[1]!);
    const key = option[1] === "image-gib" ? "imageGiB" : "timeoutSeconds";
    const value = Number(option[2]);
    const bound = schema.properties[key];
    if (!Number.isSafeInteger(value) || value < bound.minimum || value > bound.maximum) throw Error("Constrained lab value is outside its range");
    values[key] = value;
  }
  return { mode, ...values };
}

export function constrainedDisk(imageGiB: number, allocatedBytes: number, reserveBytes: number, freeBytes: number) {
  const maximumBackingBytes = (imageGiB + policy.backingOverheadGiB) * GiB;
  const requiredFreeBytes = Math.max(reserveBytes, policy.minimumHostReserveGiB * GiB) + policy.freeMarginGiB * GiB + Math.max(0, maximumBackingBytes - allocatedBytes);
  const admitted = allocatedBytes <= maximumBackingBytes && freeBytes >= requiredFreeBytes;
  return { allocatedBytes, freeBytes, requiredFreeBytes, maximumBackingBytes, admitted };
}

/** 💽 Retains the exact backing and free-space measurements from the failing sample. */
export function requireConstrainedDisk(disk: ReturnType<typeof constrainedDisk>): void {
  if (!disk.admitted) throw Error(`Private lab physical growth admission was lost: ${JSON.stringify(disk)}`);
}

export function constrainedPaths(tracked: readonly string[], untracked: readonly string[]) {
  let trackedCount = 0;
  let maximumTrackedUnits = 0;
  let maximumUntrackedUnits = 0;
  let maximumCloneUnits = 0;
  for (const [paths, isTracked] of [[tracked, true], [untracked, false]] as const) {
    for (const name of paths) {
      if (SLICE_LAB_POLICY_V1.frozen.exclude.some(prefix => name === prefix || name.startsWith(prefix + "/"))) continue;
      if (name.split("/").some(part => !part || part === "." || part === "..")) throw Error("Invalid frozen checkout input path");
      if (isTracked) {
        trackedCount++;
        maximumTrackedUnits = Math.max(maximumTrackedUnits, name.length);
      } else maximumUntrackedUnits = Math.max(maximumUntrackedUnits, name.length);
      maximumCloneUnits = Math.max(maximumCloneUnits, join(mountTemplate, cloneSuffix, name).length);
    }
  }
  return { trackedCount, maximumTrackedUnits, maximumUntrackedUnits, maximumCloneUnits, admitted: trackedCount > 0 && maximumCloneUnits <= policy.maximumRuntimePathUnits };
}

const equalIdentity = (a: Identity, b: Identity) => a.dev === b.dev && a.ino === b.ino;
const lookup = (path: string) => lstatSync(path, { bigint: true });
function exactDirectory(path: string) {
  const current = lookup(path);
  if (!current.isDirectory() || realpathSync(path) !== path || path.length > policy.maximumRuntimePathUnits) throw Error("Exact bounded lab directory required");
  return current;
}
function exactPrivateDirectory(path: string) {
  const current = exactDirectory(path);
  if (current.uid !== BigInt(process.getuid!()) || (current.mode & 0o077n) !== 0n) throw Error("Exact private lab directory required");
  return current;
}
function exactBacking(path: string, identity?: Identity) {
  const current = lookup(path);
  if (!current.isFile() || current.nlink !== 1n || current.uid !== BigInt(process.getuid!()) || realpathSync(path) !== path || identity && !equalIdentity(current, identity))
    throw Error("Exact owned sparseimage required");
  return current;
}
function hostDisk(path: string, imageGiB: number, allocated = 0) {
  const stat = statfsSync(path, { bigint: true });
  const totalBytes = Number(stat.blocks * stat.bsize), freeBytes = Number(stat.bavail * stat.bsize);
  if (!Number.isSafeInteger(totalBytes) || !Number.isSafeInteger(freeBytes) || totalBytes <= 0 || freeBytes < 0 || freeBytes > totalBytes) throw Error("Exact host partition availability required");
  const budget = casPartitionBudgetV1(totalBytes);
  if (!budget.derivable) throw Error("Physical partition reserve is not derivable");
  return { totalBytes, hostReserveBytes: Math.max(budget.reserveBytes, policy.minimumHostReserveGiB * GiB), ...constrainedDisk(imageGiB, allocated, budget.reserveBytes, freeBytes) };
}
async function exactOutput(program: string, args: string[], cwd: string, env: Environment, signal: AbortSignal, output = informationScope, timeoutMilliseconds = policy.toolTimeoutMilliseconds) {
  const chunks: Buffer[] = [];
  await runOwnedCommand(program, args, cwd, program, timeoutMilliseconds, {
    env, signal, output, stdout: "ignore",
    outputConsumer: (bytes, stream) => { if (stream === "stdout") chunks.push(Buffer.from(bytes)); },
  });
  return new TextDecoder("utf-8", { fatal: true }).decode(Buffer.concat(chunks));
}
async function pathPlan(signal: AbortSignal) {
  const inputs = [];
  for (const selector of [["--cached"], ["--others", "--exclude-standard"]]) {
    const text = await exactOutput("git", ["-c", "core.fsmonitor=false", "ls-files", "-z", ...selector], live, { ...process.env, GIT_OPTIONAL_LOCKS: "0" }, signal, scope, policy.pathScanTimeoutMilliseconds);
    if (text && !text.endsWith("\0")) throw Error("Incomplete frozen checkout roster");
    inputs.push(text ? text.slice(0, -1).split("\0") : []);
  }
  return constrainedPaths(inputs[0]!, inputs[1]!);
}
function generatedParent() {
  return existsSync(generated) ? (exactDirectory(generated), generated) : (exactDirectory(ticket), ticket);
}
function requireConstrainedHost(options: Options) {
  if (process.platform !== "darwin") throw Error("This lab wrapper requires local Darwin sparseimage ownership");
  constrainedLabArguments(mountTemplate, options);
}
async function preflight(options: Options, signal: AbortSignal) {
  const paths = await pathPlan(signal);
  const disk = hostDisk(generatedParent(), options.imageGiB);
  return { version: 1, paths, disk, memoryMegabytes: policy.memoryMegabytes, cores: policy.cores, imageGiB: options.imageGiB, timeoutSeconds: options.timeoutSeconds, admitted: paths.admitted && disk.admitted };
}
export function constrainedLabArguments(mount: string, options: Options) {
  const out = join(mount, "report");
  if (join(mount, `${SLICE_LAB_POLICY_V1.prefix}-XXXXXX`).length > SLICE_LAB_POLICY_V1.maximumWorkDirectoryCharacters) throw Error("Short private lab work path required");
  readSliceLabOptionsV1({ machines: 1, memoryMegabytes: policy.memoryMegabytes, cores: policy.cores, stages: "all", frozen: true, slice: [...SLICE_LAB_POLICY_V1.slice], work: mount, out, timeoutSeconds: options.timeoutSeconds });
  return ["run", "--machines=1", `--memory=${policy.memoryMegabytes}`, `--cores=${policy.cores}`, "--stages=all", "--frozen", `--timeout=${options.timeoutSeconds}`, `--work=${mount}`, `--out=${out}`, "--keep"];
}
async function imageTool(args: string[], cwd: string, signal: AbortSignal) {
  const command = nativeImageHdiutilCommandV1(args, cwd);
  return exactOutput(command.program, [...command.args], command.cwd, command.env, signal);
}
function record(value: unknown): ObjectRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Expected image inventory object");
  return value as ObjectRecord;
}
function records(value: unknown): ObjectRecord[] {
  if (!Array.isArray(value)) throw Error("Expected image inventory list");
  return value.map(record);
}
export function constrainedMount(inventory: ObjectRecord, backing: string, mount: string): string | undefined {
  const images = records(inventory.images);
  const owned = images.filter(image => image["image-path"] === backing);
  if (owned.length > 1) throw Error("Ambiguous private backing attachment");
  const occupants = images.flatMap(image => records(image["system-entities"]).filter(entry => entry["mount-point"] === mount).map(entry => ({ image, entry })));
  if (!owned.length && !occupants.length) return undefined;
  if (owned.length !== 1 || occupants.length !== 1 || occupants[0]!.image !== owned[0]) throw Error("Unproved private mount relationship");
  const mounted = records(owned[0]!["system-entities"]).filter(entry => entry["mount-point"] !== undefined);
  const device = occupants[0]!.entry["dev-entry"];
  if (mounted.length !== 1 || typeof device !== "string" || !/^\/dev\/disk[0-9]+(?:s[0-9]+)*$/u.test(device)) throw Error("Exact private mounted device required");
  return device;
}
function copyReceipt(source: string, destination: string, device: bigint, accept: (value: ObjectRecord) => boolean) {
  if (!lstatSync(source, { throwIfNoEntry: false })) return false;
  const parent = exactDirectory(resolve(source, ".."));
  if (parent.dev !== device || source.length > policy.maximumRuntimePathUnits) throw Error("Lab receipt escaped the private volume");
  const fd = openSync(source, constants.O_RDONLY | constants.O_NOFOLLOW | constants.O_NONBLOCK);
  try {
    const before = fstatSync(fd, { bigint: true });
    if (!before.isFile() || before.dev !== device || before.nlink !== 1n || before.size <= 0n || before.size > BigInt(policy.maximumReceiptBytes)) throw Error("Bounded private receipt required");
    const bytes = Buffer.alloc(Number(before.size) + 1);
    let length = 0;
    while (length < bytes.length) {
      const got = readSync(fd, bytes, length, bytes.length - length, null);
      if (!got) break;
      length += got;
    }
    const after = fstatSync(fd, { bigint: true });
    if (before.size !== BigInt(length) || before.mtimeNs !== after.mtimeNs || before.ctimeNs !== after.ctimeNs || !equalIdentity(before, lookup(source))) throw Error("Private receipt changed during export");
    const content = bytes.subarray(0, length);
    if (!accept(record(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(content))))) throw Error("Invalid lab receipt");
    writeFileSync(destination, content, { flag: "wx", mode: 0o600 });
    return true;
  } finally {
    closeSync(fd);
  }
}
export function exportReceipts(mount: string, dir: string, device: bigint) {
  const runs = readdirSync(mount).filter(name => /^slw-[A-Za-z0-9]{6}$/u.test(name));
  if (runs.length > 1) throw Error("Ambiguous machine result root");
  const root = runs.length === 1 ? join(mount, runs[0]!, "m1") : undefined;
  const machine = root !== undefined && copyReceipt(join(root, "result.json"), join(dir, "m1.json"), device, value => typeof value.ready === "boolean" && Object.hasOwn(value, "failure") && Array.isArray(value.timings));
  const verdict = copyReceipt(join(mount, "report", "verdict.json"), join(dir, "verdict.json"), device, value => Object.hasOwn(value, "verdict"));
  const timings = root !== undefined && copyReceipt(join(root, "s", "f", "ready", "ready-timings.json"), join(dir, "m1-timings.json"), device,
    value => validateJsonSchemaSubset(readySchema.definitions.Timings, value, readySchema).length === 0);
  return { machine, verdict, timings };
}

type LabDiskAdmission = Pick<ReturnType<typeof constrainedDisk>, "admitted" | "freeBytes" | "requiredFreeBytes">;
type LabDiskRetry = Readonly<{stage: "while-queued" | "after-grant"; disk: LabDiskAdmission}>;
type LabDiskWait = Readonly<Omit<LabDiskAdmission, "admitted"> & {stage: "before-grant" | LabDiskRetry["stage"]}>;

/** 🚦 Withdraws an ungranted lab request when the gate observes a wait and physical disk no longer admits it. */
export async function withConstrainedCapacity(options: HostCapacityOptionsV1, measure: () => LabDiskAdmission, run: (grant: HostCapacityHandleV1) => Promise<LabDiskAdmission | null>): Promise<LabDiskRetry | null> {
  const parent = options.control.signal, stop = new AbortController();
  const control = { ...options.control, signal: AbortSignal.any([parent, stop.signal]) };
  let withdrawal: {disk: LabDiskAdmission} | {error: unknown} | undefined;
  try {
    const disk = await withHostCapacityV1({ ...options, control, onWait: facts => {
      try {
        const measured = measure();
        if (!measured.admitted) withdrawal = { disk: measured };
      } catch (error) {
        withdrawal = { error };
      }
      if (withdrawal !== undefined) stop.abort(withdrawal);
      else options.onWait?.(facts);
    } }, grant => {
      parent.throwIfAborted();
      return run(grant);
    });
    return disk === null ? null : { stage: "after-grant", disk };
  } catch (error) {
    parent.throwIfAborted();
    if (withdrawal === undefined || error !== withdrawal) throw error;
    if ("error" in withdrawal) throw withdrawal.error;
    return { stage: "while-queued", disk: withdrawal.disk };
  }
}

export async function runWhenDiskAdmitted(signal: AbortSignal, measure: () => LabDiskAdmission, runUnderGrant: () => Promise<LabDiskRetry | null>, wait: () => Promise<void>, report: (row: LabDiskWait) => void) {
  for (;;) {
    signal.throwIfAborted();
    const disk = measure();
    const shortage = disk.admitted ? await runUnderGrant() : { stage: "before-grant" as const, disk };
    if (shortage === null) return;
    signal.throwIfAborted();
    report({ stage: shortage.stage, freeBytes: shortage.disk.freeBytes, requiredFreeBytes: shortage.disk.requiredFreeBytes });
    try { await wait(); }
    catch (error) { signal.throwIfAborted(); throw error; }
  }
}

async function runConstrained(options: Options) {
  const wait = createCapacityControlV1();
  const environment = { ...process.env };
  const protocol = capacitySchema.definitions.Protocol.const;
  for (const key of [protocol.grantEnvironment, protocol.planEnvironment, protocol.handoverEnvironment, protocol.marksEnvironment, protocol.totalEnvironment, protocol.coresEnvironment])
    delete environment[key];
  const id = newHostCapacityIdV1();
  const maximumMilliseconds = (options.timeoutSeconds + policy.capacityExtraSeconds) * 1000;
  const control = wait.control;
  try {
    requireConstrainedHost(options);
    const directory = hostCapacityDirectoryV1({}, "darwin", userInfo().homedir);
    if (environment[protocol.directoryEnvironment] && hostCapacityDirectoryV1(environment) !== directory) throw Error("Canonical machine-wide capacity ledger required");
    environment[protocol.directoryEnvironment] = directory;
    const request = { id, operation: "command", priority: "batch", minimumJobs: 1, preferredJobs: 1, fixedMegabytes: policy.memoryMegabytes, perJobMegabytes: 0, cpuPerJob: policy.cores } as const;
    const measureDisk = () => hostDisk(generatedParent(), options.imageGiB);
    const waitForDisk = () => delay(policy.diskAdmissionRetryMilliseconds, undefined, { signal: control.signal });
    const runUnderGrant = () => withConstrainedCapacity({ directory, policy: BUILD_HOST_CAPACITY_POLICY_V1, request, control, environment }, measureDisk, async handle => {
      const signal = AbortSignal.any([control.signal, AbortSignal.timeout(maximumMilliseconds)]);
      if (handle.id !== id || handle.bypassed || handle.inherited || handle.adopted) throw Error("Fresh machine-wide exact lab grant required");
      const granted = { ...environment, ...handle.environment, SEMIO_WORKSPACE_ROOT: live };
      function requireGrant() {
        const grant = readLiveHostCapacityReservationV1(granted, BUILD_HOST_CAPACITY_POLICY_V1);
        if (!grant || grant.id !== id || grant.jobs !== 1 || grant.cpu !== policy.cores || grant.reservedMegabytes !== policy.memoryMegabytes) throw Error("Exact machine-wide lab grant lost");
      }
      requireGrant();
      const checked = await preflight(options, signal);
      if (!checked.paths.admitted) throw Error(`Physical/path preflight under grant refused: ${JSON.stringify(checked)}`);
      signal.throwIfAborted();
      requireGrant();
      if (!checked.disk.admitted) return checked.disk;
      if (!existsSync(generated)) mkdirSync(generated, { mode: 0o700 });
      exactDirectory(generated);
      const dir = mkdtempSync(join(generated, "l-"));
      exactPrivateDirectory(dir);
      const backing = join(dir, imageBase);
      const identityPath = join(dir, "owner.json");
      const inventory = async (control: AbortSignal) => nativeImageInventoryV1(await imageTool(["info", "-plist"], dir, control));
      const mount = mkdtempSync("/private/tmp/lv-");
      const bare = exactPrivateDirectory(mount);
      const started = new Date().toISOString();
      const state = { version: 1, started, id, mount, backing, admission: checked };
      try {
        writeFileSync(identityPath, JSON.stringify({ ...state, bare: [String(bare.dev), String(bare.ino)] }), { flag: "wx", mode: 0o600 });
      } catch (error) {
        if (equalIdentity(exactPrivateDirectory(mount), bare)) rmdirSync(mount);
        throw error;
      }
      let image: Identity | undefined;
      let mounted: Identity | undefined;
      let primary: unknown, cleanup: unknown, receiptFailure: unknown;
      let receipts = { machine: false, verdict: false, timings: false };
      try {
        if (readdirSync(mount).length) throw Error("Empty private lab mount required");
        const planned = constrainedLabArguments(mount, options);
        await imageTool(["create", "-type", "SPARSE", "-fs", "Case-sensitive APFS", "-size", `${options.imageGiB}g`, "-volname", "Semio lab", "-nospotlight", backing], dir, signal);
        image = exactBacking(backing);
        writeFileSync(join(dir, "image.json"), JSON.stringify({ device: String(image.dev), inode: String(image.ino) }), { flag: "wx", mode: 0o600 });
        const created = exactBacking(backing, image);
        requireConstrainedDisk(hostDisk(dir, options.imageGiB, Number(created.blocks * 512n)));
        signal.throwIfAborted();
        requireGrant();
        if (!equalIdentity(exactPrivateDirectory(mount), bare) || constrainedMount(await inventory(signal), backing, mount)) throw Error("Unused private mount required");
        await imageTool(["attach", "-plist", "-mount", "required", "-nobrowse", "-mountpoint", mount, backing], dir, signal);
        if (!constrainedMount(await inventory(signal), backing, mount)) throw Error("Private image did not attach at its owned mount");
        mounted = exactDirectory(mount);
        if (mounted.dev === bare.dev || !readdirSync(mount).every(name => [".fseventsd", ".Spotlight-V100", ".Trashes"].includes(name))) throw Error("Fresh private APFS root required");
        chmodSync(mount, 0o700);
        if (!equalIdentity(exactPrivateDirectory(mount), mounted)) throw Error("Private mounted root changed");
        const virtual = statfsSync(mount, { bigint: true });
        if (virtual.bavail * virtual.bsize < BigInt(policy.minimumVirtualGiB * GiB)) throw Error("Formatted private APFS volume is below the 40 GiB lab floor");
        const stop = new AbortController();
        const childSignal = AbortSignal.any([signal, stop.signal]);
        function verifyRunning() {
          const current = exactBacking(backing, image);
          if (!equalIdentity(exactDirectory(mount), mounted!)) throw Error("Private lab mount identity changed");
          requireConstrainedDisk(hostDisk(dir, options.imageGiB, Number(current.blocks * 512n)));
          requireGrant();
        }
        verifyRunning();
        const timer = setInterval(() => {
          try { verifyRunning(); } catch (error) { stop.abort(error); }
        }, policy.diskSampleMilliseconds);
        try {
          await runOwnedCommand("repo", ["tools", "exec", "--light", "bun", "--", lab, ...planned], live, "constrained-slice-lab", maximumMilliseconds, { env: granted, signal: childSignal });
          childSignal.throwIfAborted();
          verifyRunning();
        } catch (error) {
          throw stop.signal.reason ?? error;
        } finally {
          clearInterval(timer);
        }
      } catch (error) {
        primary = error;
      }
      try {
        const drain = AbortSignal.timeout(3 * policy.toolTimeoutMilliseconds);
        if (image) exactBacking(backing, image);
        const observed = await inventory(drain);
        const device = constrainedMount(observed, backing, mount);
        if (device) {
          if (!image) throw Error("Private image identity was never acquired");
          const current = exactDirectory(mount);
          if (current.dev === bare.dev || mounted && !equalIdentity(current, mounted)) throw Error("Private mount identity changed");
          if (mounted) {
            try { receipts = exportReceipts(mount, dir, current.dev); } catch (error) { receiptFailure = error; primary ??= error; }
          }
          if (records(observed.images).some(row => typeof row["image-path"] === "string" && row["image-path"].startsWith(mount + sep))) throw Error("Nested lab image remains attached");
          exactBacking(backing, image);
          if (!equalIdentity(exactDirectory(mount), current)) throw Error("Private mount identity changed before detach");
          await imageTool(["detach", mount], dir, drain);
          if (constrainedMount(await inventory(drain), backing, mount)) throw Error("Private lab mount remains attached");
        }
        if (!equalIdentity(exactPrivateDirectory(mount), bare)) throw Error("Bare mount identity changed");
        rmdirSync(mount);
        if (image && !receiptFailure) {
          exactBacking(backing, image);
          unlinkSync(backing);
        } else if (!image && existsSync(backing)) throw Error("Unproved sparseimage retained");
      } catch (error) {
        cleanup = error;
      }
      if (!primary && !receipts.machine) primary = Error("Lab process completed without its machine receipt");
      const note = (error: unknown) => error instanceof Error ? error.message.slice(-4000) : error === undefined ? null : String(error).slice(-4000);
      writeFileSync(join(dir, "launch.json"), JSON.stringify({ ...state, ended: new Date().toISOString(), receipts, primary: note(primary), receiptFailure: note(receiptFailure), cleanup: note(cleanup) }, null, 2), { flag: "wx", mode: 0o600 });
      process.stdout.write(`[${dir}]\n`);
      if (primary) throw primary;
      if (cleanup) throw cleanup;
      return null;
    });
    await runWhenDiskAdmitted(control.signal, measureDisk, runUnderGrant, waitForDisk,
      row => process.stderr.write(`[lab] disk-wait ${JSON.stringify(row)}\n`));
  } finally {
    wait.dispose();
  }
}

if (import.meta.main) {
  try {
    if (process.argv.length === 3 && process.argv[2] === "--help") process.stdout.write(`Usage: constrained.ts preflight|run [--image-gib=41..48] [--timeout=30..14400]\nDefault timeout ${schema.properties.timeoutSeconds.default} seconds\n`);
    else {
      const options = constrainedOptions(process.argv.slice(2));
      if (options.mode === "run") await runConstrained(options);
      else {
        requireConstrainedHost(options);
        const checked = await preflight(options, AbortSignal.timeout(2 * policy.pathScanTimeoutMilliseconds));
        process.stdout.write(JSON.stringify(checked, null, 2) + "\n");
        if (!checked.admitted) process.exitCode = 2;
      }
    }
  } catch (error) {
    process.stderr.write((error instanceof Error ? error.message : String(error)) + "\n");
    process.exitCode = 1;
  }
}
