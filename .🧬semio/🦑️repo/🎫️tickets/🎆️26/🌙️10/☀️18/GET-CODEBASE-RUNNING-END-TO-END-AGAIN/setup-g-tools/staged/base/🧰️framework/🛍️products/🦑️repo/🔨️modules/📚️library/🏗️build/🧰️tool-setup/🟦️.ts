import {existsSync, realpathSync} from "node:fs";
import {homedir} from "node:os";
import {delimiter, dirname, join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import type {NativeToolV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {pythonInterpreterDirectoryV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import document from "./🔣️.json" with {type: "json"};

if (validateJsonSchemaSubset(schema, document, schema).length) throw new Error("Closed pinned tool table required / Geschlossene Tabelle der gepinnten Werkzeuge erforderlich");

/** 🧷️ How a pin is written: the value itself, or a pointer, pattern or row filter into the file that owns it. */
export type ToolPinV1 =
  | Readonly<{format: "value"; value: string}>
  | Readonly<{format: "json"; file: string; pointer: string}>
  | Readonly<{format: "regex"; file: string; pattern: string; flags?: string}>
  | Readonly<{format: "rows"; file: string; pointer: string; where: Readonly<Record<string, string>>; field: string; package?: string; from?: string}>;
/** 📍️ How a provisioned or installed copy is found. */
export type ToolLocateV1 =
  | Readonly<{kind: "provisioned"; family: "bun" | "bazel" | "cargo" | "binaryen" | "dotnet"; executable: string}>
  | Readonly<{kind: "provisioned"; family: "native"; tool: NativeToolV1; executable: string}>
  | Readonly<{kind: "path"; names: readonly string[]; directories: readonly string[]}>
  | Readonly<{kind: "python"; interpreter: string}>
  | Readonly<{kind: "hook"}>;
/** 🧰️ How a missing or drifted tool is provisioned. */
export type ToolSetupKindV1 =
  | Readonly<{kind: "bun" | "bootstrap" | "rust" | "dotnet" | "bazel" | "binaryen"}>
  | Readonly<{kind: "native"; tool: NativeToolV1}>
  | Readonly<{kind: "python"; interpreter: string}>
  | Readonly<{kind: "script"; script: string; prepare: readonly string[]; verify?: readonly string[]}>;
/** 🏓️ One row of the table. */
export type ToolRowV1 = Omit<(typeof document.tools)[number], "pin" | "locate" | "setup"> & Readonly<{pin: ToolPinV1; locate: readonly ToolLocateV1[]; setup: ToolSetupKindV1}>;
/** 🧾️ The authored table of every pinned tool: pins as pointers, probes, locations and provisioners. */
export const TOOL_SETUP_V1: Omit<typeof document, "tools"> & Readonly<{tools: readonly ToolRowV1[]}> = document as unknown as Omit<typeof document, "tools"> & Readonly<{tools: readonly ToolRowV1[]}>;
/** 🩺️ The verdict on one tool. */
export type ToolStatusV1 = "ok" | "missing" | "differs" | "failed" | "deferred";
/** 📊️ What verification found for one tool on this machine. */
export type ToolReportV1 = Readonly<{id: string; pin: string; found: string | null; status: ToolStatusV1; location: string | null; message?: string}>;
/** 🔌️ Everything verification touches outside the pure table: files, child programs, the platform and the provisioned layouts; replaceable so every host and every failure is testable on any host. */
export type ToolContextV1 = Readonly<{
  root: string;
  toolsRoot: string;
  platform: NodeJS.Platform;
  environment: Readonly<Record<string, string | undefined>>;
  home: string;
  read(path: string): string;
  exists(path: string): boolean;
  list(directory: string): readonly string[];
  resolvePackage(name: string, from: string): string;
  capture(command: string, args: readonly string[], cwd: string, signal: AbortSignal): Promise<string>;
  provisioned(locate: Extract<ToolLocateV1, {kind: "provisioned"}>, pin: string): string | undefined;
  hook(row: ToolRowV1, pin: string, signal: AbortSignal): Promise<Readonly<{found: string | null; location: string | null}>>;
}>;

/** 🗣️ Renders one text of the table in the resolved language; without a resolvable language it renders every language, so no language is ever assumed. */
export function toolSetupMessageV1(key: keyof typeof document.messages.texts, values: Readonly<Record<string, string | number>> = {}, environment: Readonly<Record<string, string | undefined>> = process.env, args: readonly string[] = []): string {
  const text = document.messages.texts[key] as Readonly<Record<string, string>>, fill = (template: string) => template.replace(/\{([a-z]+)\}/gu, (_all, name: string) => String(values[name] ?? "{" + name + "}"));
  try {
    return fill(text[resolveLanguageV1(document.messages.languages, args, environment, () => []).language]!);
  } catch {
    return document.messages.languages.map(language => fill(text[language]!)).join(" / ");
  }
}

/** 🔎️ Finds one row by its id. */
export function toolRowV1(id: string, environment: Readonly<Record<string, string | undefined>> = process.env): ToolRowV1 {
  const row = TOOL_SETUP_V1.tools.find(candidate => candidate.id === id);
  if (!row) throw new Error(toolSetupMessageV1("unknown", {tool: id}, environment));
  return row;
}

/** 📋️ The rows of a selection in table order: `all` or the ids named. */
export function toolRowsV1(selection: "all" | readonly string[], environment: Readonly<Record<string, string | undefined>> = process.env): readonly ToolRowV1[] {
  const ids = selection === "all" ? document.groups.all : selection;
  return ids.map(id => toolRowV1(id, environment));
}

/** 📌️ The version of a tool whose pin is the value the table itself holds. */
export function toolPinValueV1(id: string, environment: Readonly<Record<string, string | undefined>> = process.env): string {
  const pin = toolRowV1(id, environment).pin;
  if (pin.format !== "value") throw new Error(toolSetupMessageV1("pinfile", {tool: id, file: "tool table"}, environment));
  return pin.value;
}

const at = (value: unknown, pointer: string): unknown => pointer.split("/").slice(1).reduce<unknown>((current, key) => (current as Record<string, unknown> | undefined)?.[key], value);

/** 📖️ Reads the pin a row names from the pin file that owns it; the pin is never copied. */
export function resolvePinV1(row: ToolRowV1, context: Pick<ToolContextV1, "root" | "read" | "resolvePackage" | "environment">): string {
  const pin = row.pin, fail = (file: string): Error => new Error(toolSetupMessageV1("pinfile", {tool: row.id, file}, context.environment));
  if (pin.format === "value") return pin.value;
  const file = "package" in pin && pin.package !== undefined ? join(dirname(context.resolvePackage(pin.package, "from" in pin && pin.from !== undefined ? pin.from : pin.package)), ...pin.file.split("/")) : join(context.root, ...pin.file.split("/"));
  let value: unknown;
  try {
    const text = context.read(file);
    if (pin.format === "regex") return new RegExp(pin.pattern, "flags" in pin ? pin.flags : "").exec(text)?.[1] ?? (() => { throw fail(file); })();
    value = at(JSON.parse(text), pin.pointer);
  } catch (error) {
    throw error instanceof Error && error.message.includes(row.id) ? error : fail(file);
  }
  if (pin.format === "json") {
    if (typeof value !== "string") throw fail(file);
    return value;
  }
  const match = (Array.isArray(value) ? value : []).find(candidate => Object.entries(pin.where).every(([key, expected]) => String((candidate as Record<string, unknown>)[key]) === expected)) as Record<string, unknown> | undefined;
  if (typeof match?.[pin.field] !== "string") throw fail(file);
  return match[pin.field] as string;
}

/** ⚖️ Decides whether a found version satisfies a pin: equal, or the pin names a series (`3.14`) the found version belongs to. */
export function toolVersionMatchesV1(found: string, pin: string): boolean {
  return found === pin || found.startsWith(pin + ".");
}

const expand = (entry: string, context: ToolContextV1): string | undefined => entry.startsWith("$") ? context.environment[entry.slice(1)] : entry.startsWith("~") ? join(context.home, entry.slice(1)) : entry;

function pythonInterpreter(context: ToolContextV1, series: string): string | undefined {
  const directory = pythonInterpreterDirectoryV1(context.toolsRoot), windows = context.platform === "win32";
  const matches = context.list(directory).filter(name => name.startsWith("cpython-" + series + ".")).sort().reverse();
  for (const name of matches) {
    const candidate = windows ? join(directory, name, "python.exe") : join(directory, name, "bin", "python3");
    if (context.exists(candidate)) return candidate;
  }
  return undefined;
}

/** 📦️ Finds the `package.json` of a package the way Node resolves it from an installed package: in the `node_modules` directories above the real directory of `from`. */
export function resolveSiblingPackageV1(root: string, name: string, from: string): string {
  for (let current = dirname(realpathSync(join(root, "node_modules", from))); ; current = dirname(current)) {
    const candidate = join(current, name, "package.json");
    if (existsSync(candidate)) return candidate;
    if (dirname(current) === current) throw new Error(name);
  }
}

/** ⏮️ Finds the first copy of a tool the row's locators name: provisioned in the tools partition first, then installed on the machine. */
export function locateToolV1(row: ToolRowV1, pin: string, context: ToolContextV1): string | undefined {
  const suffix = context.platform === "win32" ? ".exe" : "";
  for (const locate of row.locate) {
    if (locate.kind === "provisioned") {
      const found = context.provisioned(locate, pin);
      if (found !== undefined && context.exists(found)) return found;
    } else if (locate.kind === "python") {
      const found = pythonInterpreter(context, pin);
      if (found !== undefined) return found;
    } else if (locate.kind === "path") {
      const directories = [...locate.directories.map(entry => expand(entry, context)).filter((entry): entry is string => entry !== undefined), ...(context.environment[Object.keys(context.environment).find(name => name.toLowerCase() === "path") ?? "PATH"] ?? "").split(delimiter).filter(Boolean)];
      for (const directory of directories) for (const name of locate.names) {
        const candidate = join(directory, name.endsWith(suffix) ? name : name + suffix);
        if (context.exists(candidate)) return candidate;
      }
    }
  }
  return undefined;
}

/** 🆚️ Compares one tool with its pin: resolves the pin, finds a copy, asks it for its version and decides. */
export async function verifyToolV1(row: ToolRowV1, context: ToolContextV1, signal: AbortSignal): Promise<ToolReportV1> {
  signal.throwIfAborted();
  let pin: string;
  try {
    pin = resolvePinV1(row, context);
  } catch (error) {
    return Object.freeze({id: row.id, pin: "-", found: null, status: "failed", location: null, message: (error as Error).message});
  }
  try {
    if (row.probe.mode === "hook") {
      const result = await context.hook(row, pin, signal);
      return Object.freeze({id: row.id, pin, found: result.found, location: result.location, status: result.found === null ? "missing" : toolVersionMatchesV1(result.found, pin) ? "ok" : "differs"});
    }
    const location = locateToolV1(row, pin, context);
    if (location === undefined) return Object.freeze({id: row.id, pin, found: null, location: null, status: "missing"});
    const output = await context.capture(location, row.probe.arguments.map(argument => argument.replace("{pin}", pin)), context.root, signal);
    if (row.probe.mode === "listed") {
      const listed = new RegExp(row.probe.pattern.replace("{pin}", pin.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")), "mu").exec(output)?.[1] ?? null;
      return Object.freeze({id: row.id, pin, found: listed, location, status: listed === null ? "differs" : "ok"});
    }
    const found = new RegExp(row.probe.pattern, "mu").exec(output)?.[1] ?? null;
    return Object.freeze({id: row.id, pin, found, location, status: found === null ? "failed" : toolVersionMatchesV1(found, pin) ? "ok" : "differs"});
  } catch (error) {
    signal.throwIfAborted();
    return Object.freeze({id: row.id, pin, found: null, location: null, status: "failed", message: (error as Error).message});
  }
}

/** ✅️ Verifies every row of a selection one after the other; a cancelled signal stops between tools. */
export async function verifyToolsV1(rows: readonly ToolRowV1[], context: ToolContextV1, signal: AbortSignal): Promise<readonly ToolReportV1[]> {
  const reports: ToolReportV1[] = [];
  for (const row of rows) reports.push(await verifyToolV1(row, context, signal));
  return Object.freeze(reports);
}

/** 🖨️ Renders the verification report as aligned lines in the resolved language, the verdict last. */
export function renderToolReportV1(reports: readonly ToolReportV1[], environment: Readonly<Record<string, string | undefined>> = process.env): readonly string[] {
  const lines = [toolSetupMessageV1("header", {}, environment)];
  for (const report of reports) lines.push([report.id.padEnd(14), report.pin.padEnd(21), (report.found ?? "-").padEnd(21), toolSetupMessageV1(report.status, {}, environment).padEnd(9), report.location ?? report.message ?? "-"].join(" "));
  const problems = reports.filter(report => report.status !== "ok").length;
  lines.push(problems === 0 ? toolSetupMessageV1("clean", {count: reports.length}, environment) : toolSetupMessageV1("drift", {count: problems, total: reports.length}, environment));
  return Object.freeze(lines);
}

/** ⚠️ Warns when the Bun that runs the tools is not the pinned one, which the verification of the provisioned copy cannot show. */
export function runtimeWarningV1(reports: readonly ToolReportV1[], running: string | undefined, environment: Readonly<Record<string, string | undefined>> = process.env): string | undefined {
  const bun = reports.find(report => report.id === "bun");
  return bun === undefined || running === undefined || toolVersionMatchesV1(running, bun.pin) ? undefined : toolSetupMessageV1("runtime", {found: running, pin: bun.pin}, environment);
}

/** 🏃️ Names the Bun that runs the tools, where it lives and the pinned one, so a report always says which runtime judged it. */
export function runningRuntimeLineV1(reports: readonly ToolReportV1[], running: string, executable: string, environment: Readonly<Record<string, string | undefined>> = process.env): string {
  return toolSetupMessageV1("running", {found: running, path: executable, pin: reports.find(report => report.id === "bun")?.pin ?? "-"}, environment);
}

/** 🖥️ What setting up one tool needs from the machine: the provisioner of its kind. */
export type ToolProvisionerV1 = (row: ToolRowV1, pin: string, signal: AbortSignal) => Promise<void>;

/** ⛓️ Whether the pin of a row is written in a package of the installed dependencies and those are not installed yet. */
export function toolPinAwaitsDependenciesV1(row: ToolRowV1, context: Pick<ToolContextV1, "root" | "exists">): boolean {
  return row.pin.format === "rows" && row.pin.package !== undefined && !context.exists(join(context.root, "node_modules", row.pin.from ?? row.pin.package));
}

/** 📥️ Provisions every tool of a selection that does not match its pin, then verifies it again; a tool installed by the bootstrap script is reported, not installed, and a provisioned tool that still differs fails the whole setup. */
export async function setupToolsV1(rows: readonly ToolRowV1[], context: ToolContextV1, provision: ToolProvisionerV1, signal: AbortSignal, log: (line: string) => void): Promise<readonly ToolReportV1[]> {
  const reports: ToolReportV1[] = [];
  for (const row of rows) {
    signal.throwIfAborted();
    const before = await verifyToolV1(row, context, signal);
    if (before.status === "ok") {
      log(toolSetupMessageV1("skipped", {tool: row.id, found: before.found ?? before.pin}, context.environment));
      reports.push(before);
      continue;
    }
    if (before.status === "failed" && before.pin === "-") {
      if (!toolPinAwaitsDependenciesV1(row, context)) throw new Error(before.message);
      const pin = row.pin as Extract<ToolPinV1, {format: "rows"}>, message = toolSetupMessageV1("postponed", {tool: row.id, package: pin.from ?? pin.package ?? row.id}, context.environment);
      log(message);
      reports.push(Object.freeze({...before, status: "deferred" as const, message}));
      continue;
    }
    if (row.setup.kind === "bootstrap") {
      log(toolSetupMessageV1("manual", {tool: row.id, pin: before.pin}, context.environment));
      reports.push(before);
      continue;
    }
    log(toolSetupMessageV1("step", {tool: row.id, pin: before.pin}, context.environment));
    await provision(row, before.pin, signal);
    const after = await verifyToolV1(row, context, signal);
    if (after.status !== "ok") throw new Error(toolSetupMessageV1("mismatch", {tool: row.id, found: after.found ?? "-", pin: after.pin}, context.environment));
    log(toolSetupMessageV1("done", {tool: row.id, found: after.found ?? after.pin}, context.environment));
    reports.push(after);
  }
  return Object.freeze(reports);
}

/** 🏠️ The default home directory of a context. */
export const toolHomeV1 = (): string => homedir();
