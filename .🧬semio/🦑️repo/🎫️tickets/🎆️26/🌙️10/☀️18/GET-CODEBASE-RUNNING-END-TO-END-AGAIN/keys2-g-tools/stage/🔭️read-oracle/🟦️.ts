import {lstatSync, mkdtempSync, readdirSync, realpathSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import document from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import {LandlockRefusedV1, linuxAvailableV1, runLinuxV1, type LinuxToolsV1} from "./🐧️linux/🟦️.ts";
import {runSeatbeltV1, SeatbeltProfileRefusedV1, seatbeltAvailableV1} from "./🍎️seatbelt/🟦️.ts";
import {planV1, undeclaredV1, type ReadPlanV1, type ReadRoundV1, type ReadRunV1} from "./🪢️policy/🟦️.ts";
import {runMirrorV1} from "./🪞️mirror/🟦️.ts";
import {captureV1, type CapturedV1} from "./🏃️run/🟦️.ts";

/** 🔭️ The backends behind the one interface: `seatbelt` (macOS kernel enforcement), `landlock` (Linux kernel enforcement, strace names the refusals) and `mirror` (every platform, output equivalence). */
export type ReadOracleBackendV1 = "seatbelt" | "landlock" | "mirror";
/** 💪️ How strong the evidence of a backend is. */
export type ReadOracleStrengthV1 = "kernel-enforced" | "output-equivalence";
/** 📨️ The backend neutral request: which command, which files it may read, how often to learn the missing ones; the language is explicit, none is assumed. */
export type ReadOracleRequestV1 = {readonly version: 1; readonly language: "en" | "de"; readonly root: string; readonly cwd: string; readonly allowed: readonly string[]; readonly ambient: readonly string[]; readonly names: readonly string[]; readonly command: readonly string[]; readonly environment: Readonly<Record<string, string>>; readonly rounds: number; readonly timeoutMs: number; readonly backend: "auto" | ReadOracleBackendV1; readonly compare: "output" | "exit"};
/** 🔄️ One round of the report. */
export type ReadOracleRoundV1 = ReadRoundV1 & {readonly round: number; readonly backend: ReadOracleBackendV1};
/** 📊️ What the oracle found: the backend and its strength, every undeclared read it learned, every round, whether the command passed and whether a final round read nothing undeclared. */
export type ReadOracleReportV1 = {readonly version: 1; readonly backend: ReadOracleBackendV1; readonly strength: ReadOracleStrengthV1; readonly allowed: number; readonly learned: readonly string[]; readonly rounds: readonly ReadOracleRoundV1[]; readonly passed: boolean; readonly clean: boolean};
/** 🔌️ The outside world of a run, injectable for tests: platform, tools, progress and cancellation. */
export type ReadOraclePortsV1 = {readonly platform?: NodeJS.Platform; readonly python?: string; readonly progress?: (text: string) => void; readonly signal?: AbortSignal; readonly memoryLimitMegabytes?: number};

const STRENGTH: Readonly<Record<ReadOracleBackendV1, ReadOracleStrengthV1>> = {seatbelt: "kernel-enforced", landlock: "kernel-enforced", mirror: "output-equivalence"};
const ORDER: Readonly<Partial<Record<NodeJS.Platform, readonly ReadOracleBackendV1[]>>> = {darwin: ["seatbelt", "mirror"], linux: ["landlock", "mirror"]};

/** 💬️ Renders a message of the document in the requested language. */
export function readOracleTextV1(language: "en" | "de", key: keyof typeof document.messages, values: Readonly<Record<string, string | number>> = {}): string {
  return document.messages[key][language].replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name] ?? `{${name}}`));
}

/** 🪟️ The Windows plan of the document in a language: the summary and one line per option with its strength and whether it needs no privileges. */
export function windowsPlanV1(language: "en" | "de"): string[] {
  return [document.windows.summary[language], ...document.windows.options.map((option) => `- ${option.id} (${option.strength}, ${option.userland ? "userland" : "privileged"}): ${option.text[language]}`)];
}

/** 🛂️ Refuses a request that does not satisfy the schema. */
export function readOracleRequestV1(value: unknown): ReadOracleRequestV1 {
  const failures = validateJsonSchemaSubset({$ref: "#/definitions/Request"}, value, schema as never);
  if (failures.length) throw new Error(`Read oracle request refused: ${failures.slice(0, 3).join("; ")}`);
  return value as ReadOracleRequestV1;
}

/** 🧭️ The backends that can run on a platform, best first; the mirror is always last. */
export function backendOrderV1(platform: NodeJS.Platform): readonly ReadOracleBackendV1[] {
  return ORDER[platform] ?? ["mirror"];
}

const launcher = join(import.meta.dirname, "🐧️linux/🐍️.py");
const lister = (root: string) => (directory: string) => { try { return readdirSync(directory ? join(root, directory) : root); } catch { return undefined; } };
const linuxFileSystem = {list: (absolute: string) => { try { return readdirSync(absolute); } catch { return undefined; } }, isDirectory: (absolute: string) => { try { return lstatSync(absolute).isDirectory(); } catch { return false; } }, real: (absolute: string) => { try { return realpathSync(absolute); } catch { return absolute; } }};

/** 🏁️ Runs the command under the best available backend, learns the undeclared reads round by round and reports them. A backend that cannot run here (profile too large, no Landlock) hands over to the next one; progress is reported per round and a signal cancels between rounds and kills the running command. */
export async function runReadOracleV1(input: unknown, ports: ReadOraclePortsV1 = {}): Promise<ReadOracleReportV1> {
  const request = readOracleRequestV1(input), platform = ports.platform ?? process.platform, root = realpathSync(request.root), language = request.language;
  const python = ports.python ?? process.env.SEMIO_PYTHON ?? "python3", limit = ports.memoryLimitMegabytes ?? 500;
  const available = new Map<ReadOracleBackendV1, {available: boolean; reason?: string; trace?: boolean}>([["seatbelt", seatbeltAvailableV1(platform)], ["landlock", linuxAvailableV1(launcher, python, platform)], ["mirror", {available: true}]]);
  const candidates = (request.backend === "auto" ? backendOrderV1(platform) : [request.backend]).filter((backend) => available.get(backend)!.available);
  if (!candidates.length) throw new Error(readOracleTextV1(language, "unavailable", {platform, reasons: (request.backend === "auto" ? backendOrderV1(platform) : [request.backend]).map((backend) => `${backend}: ${available.get(backend)!.reason ?? "unavailable"}`).join("; ")}));
  const owned = mkdtempSync(join(tmpdir(), "ro-"));
  const tools: LinuxToolsV1 = {python, launcher, trace: available.get("landlock")!.trace ?? false, fs: linuxFileSystem};
  const learned: string[] = [], rounds: ReadOracleRoundV1[] = [], base = {command: request.command, cwd: request.cwd, env: {...process.env, ...request.environment}, timeoutMs: request.timeoutMs, scratch: owned, signal: ports.signal};
  let baseline: CapturedV1 | undefined, index = 0, round = 0;
  try {
    while (round < request.rounds) {
      if (ports.signal?.aborted) throw new Error(readOracleTextV1(language, "cancelled"));
      if (process.memoryUsage().rss > limit * 1048576) throw new Error(readOracleTextV1(language, "memory", {limit}));
      const backend = candidates[index]!, allowed = [...request.allowed, ...learned], plan: ReadPlanV1 = planV1(root, {allowed, ambient: request.ambient, names: request.names}, lister(root)), run: ReadRunV1 = {...base, round};
      let result: ReadRoundV1;
      try {
        if (backend === "seatbelt") result = await runSeatbeltV1(plan, run);
        else if (backend === "landlock") result = await runLinuxV1(plan, run, tools);
        else {
          if (request.compare && baseline === undefined) baseline = await captureV1(request.command, {cwd: join(root, request.cwd), env: run.env, timeoutMs: request.timeoutMs, signal: ports.signal});
          result = await runMirrorV1(plan, allowed, run, baseline && {...baseline, output: request.compare === "output" ? baseline.output : undefined});
        }
      } catch (error) {
        if (!(error instanceof SeatbeltProfileRefusedV1 || error instanceof LandlockRefusedV1) || index + 1 >= candidates.length) throw error;
        ports.progress?.(readOracleTextV1(language, "fallback", {from: backend, reason: error.message.split("\n")[0]!.slice(0, 120), to: candidates[index + 1]!}));
        index++;
        continue;
      }
      const fresh = undeclaredV1(result.denied, plan);
      rounds.push({...result, round, backend, denied: fresh});
      ports.progress?.(readOracleTextV1(language, "round", {round, exit: result.exit, denied: fresh.length, seconds: result.seconds, backend}));
      round++;
      learned.push(...fresh);
      if (!fresh.length) break;
    }
  } finally {
    rmSync(owned, {recursive: true, force: true});
  }
  const last = rounds.at(-1)!, backend = last.backend;
  return {version: 1, backend, strength: STRENGTH[backend], allowed: request.allowed.length, learned, rounds, passed: last.exit === 0, clean: last.exit === 0 && last.denied.length === 0};
}

