import {mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, isAbsolute, join, resolve} from "node:path";
import {bazelUnitsV1, nxUnitTargetsV1, UNITS_BZL_V1, unitSpecPathV1, unitSpecsV1, unitTargetNameV1} from "../🏗️emit/🟦️.ts";
import {fileUnitsStoreV1, memoryUnitsStoreV1, runUnitsV1, unitsMessageV1, type ExecuteOutcomeV1, type ExecuteRequestV1, type UnitsRunResultV1} from "../🏃️engine/🟦️.ts";
import {applyGoOverlayV1, goUnitExecutorV1, planGoUnitsV1, type GoBuildContextV1, type GoWorkspaceSnapshotV1} from "../🐹️go/🟦️.ts";
import {planPythonUnitsV1, pythonUnitExecutorV1, type PythonModuleV1} from "../🐍️python/🟦️.ts";
import {canonicalJsonV1, interfaceClosureV1, outputClosureV1, sha256V1, unitInterfaceFileV1, unitKeyV1, type UnitResultFactsV1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, planUnitsV1, readUnitGraphV1, readUnitsPolicyV1, unitsSchemaFailuresV1, type UnitGraphV1, type UnitV1, type UnitsPolicyV1} from "../🕸️graph/🟦️.ts";
import policyDocument from "../🔣️.json" with {type: "json"};

const policy: UnitsPolicyV1 = readUnitsPolicyV1(policyDocument);
const both = (english: string, german: string): string => `${english}\n${german}`;
const operations = ["plan", "run", "emit", "exec"] as const;
const valuedOptions = Object.freeze(["--for", "--root", "--out", "--out-root", "--spec", "--iface", "--iface-dep", "--impl-dep", "--concurrency", "--only", "--receipts", "--file", "--lang"]);
const plainOptions = Object.freeze(["--json"]);
const repeatable = new Set(["--iface-dep", "--impl-dep", "--only"]);

/** 🚩️ The option names `repo units` accepts, so the operations catalogue can be checked against the parser. */
export function unitsOptionNamesV1(): {readonly valued: readonly string[]; readonly plain: readonly string[]} {return {valued: valuedOptions, plain: plainOptions};}

/** 📝️ The closed arguments of `repo units`. */
export type UnitsArgumentsV1 = Readonly<{
  operation: typeof operations[number]; target: "nx" | "bazel" | undefined; for: "go" | "python" | undefined; root: string | undefined; out: string | undefined; outRoot: string | undefined; spec: string | undefined; iface: string | undefined;
  ifaceDeps: readonly string[]; implDeps: readonly string[]; jobs: number; only: readonly string[]; store: string | undefined; file: string | undefined; json: boolean; language: "en" | "de" | undefined;
}>;

/** 🧷️ Reads `units plan|run|emit|exec` with its options; every unknown or repeated (non-repeatable) option is refused in both languages. */
export function unitsArgumentsV1(args: readonly string[]): UnitsArgumentsV1 {
  const usage = both("Expected repo units plan|run --for=go|python | emit nx|bazel --for=go|python --out=<dir> | exec --spec=<file> --out=<file> [options]", "Erwartet: repo units plan|run --for=go|python | emit nx|bazel --for=go|python --out=<Verzeichnis> | exec --spec=<Datei> --out=<Datei> [Optionen]");
  const operation = args[0] as typeof operations[number];
  if (!operations.includes(operation)) throw Error(usage);
  let index = 1, target: "nx" | "bazel" | undefined;
  if (operation === "emit") {
    if (args[1] !== "nx" && args[1] !== "bazel") throw Error(usage);
    target = args[1]; index = 2;
  }
  const values = new Map<string, string[]>(), flags = new Set<string>(), valued = new Set(valuedOptions), plain = new Set(plainOptions);
  for (; index < args.length; index++) {
    const argument = args[index]!, cut = argument.indexOf("="), name = cut < 0 ? argument : argument.slice(0, cut);
    if (valued.has(name)) {
      const value = cut < 0 ? args[++index] : argument.slice(cut + 1);
      if (!value || value.startsWith("--")) throw Error(both(`Option ${name} needs a value`, `Option ${name} braucht einen Wert`));
      if (values.has(name) && !repeatable.has(name)) throw Error(both(`Repeated option: ${name}`, `Wiederholte Option: ${name}`));
      values.set(name, [...(values.get(name) ?? []), value]);
    } else if (plain.has(name) && cut < 0) {
      if (flags.has(name)) throw Error(both(`Repeated option: ${name}`, `Wiederholte Option: ${name}`));
      flags.add(name);
    } else throw Error(both(`Unknown units option: ${argument}`, `Unbekannte Option für units: ${argument}`));
  }
  const one = (name: string): string | undefined => values.get(name)?.[0], language = one("--lang"), source = one("--for"), jobs = one("--concurrency") === undefined ? 1 : Number(one("--concurrency"));
  if (language !== undefined && language !== "en" && language !== "de") throw Error(both(`Unknown language: ${language} (en|de)`, `Unbekannte Sprache: ${language} (en|de)`));
  if (source !== undefined && source !== "go" && source !== "python") throw Error(both(`Unknown source language: ${source} (go|python)`, `Unbekannte Quellsprache: ${source} (go|python)`));
  if (!Number.isInteger(jobs) || jobs < 1 || jobs > 16) throw Error(both("--concurrency takes an integer from 1 to 16", "--concurrency nimmt eine ganze Zahl von 1 bis 16"));
  if ((operation === "plan" || operation === "run" || operation === "emit") && source === undefined) throw Error(both(`units ${operation} needs --for=go|python`, `units ${operation} braucht --for=go|python`));
  if (operation === "emit" && one("--out") === undefined) throw Error(both("units emit needs --out=<directory>", "units emit braucht --out=<Verzeichnis>"));
  if (operation === "exec" && (one("--spec") === undefined || (one("--out") === undefined && one("--out-root") === undefined))) throw Error(both("units exec needs --spec=<file> and --out=<file> or --out-root=<directory>", "units exec braucht --spec=<Datei> und --out=<Datei> oder --out-root=<Verzeichnis>"));
  return Object.freeze({
    operation, target, for: source as "go" | "python" | undefined, root: one("--root"), out: one("--out"), outRoot: one("--out-root"), spec: one("--spec"), iface: one("--iface"), ifaceDeps: values.get("--iface-dep") ?? [], implDeps: values.get("--impl-dep") ?? [],
    jobs, only: values.get("--only") ?? [], store: one("--receipts"), file: one("--file"), json: flags.has("--json"), language: language as "en" | "de" | undefined,
  });
}

/** 🎛️ Everything the command needs from the host process, so the command itself is testable without a build CLI: output, progress and cancellation of the surrounding invocation, a process runner, the tracked file inventory and (for the repository's canonical Go layout) the compiler overlay of the Go modules. */
export type UnitsCommandPortsV1 = Readonly<{
  write(line: string): void;
  progress(stage: string): Promise<void>;
  check(): void;
  signal: AbortSignal;
  language(requested: "en" | "de" | undefined): "en" | "de";
  run(argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}): Promise<{readonly code: number; readonly output: string}>;
  files(root: string, signal: AbortSignal): Promise<readonly string[]>;
  goOverlay?(root: string, modules: readonly string[]): Promise<Readonly<{replace: Readonly<Record<string, string>>; excludedDirectories: readonly string[]}>>;
}>;

const textOf = (path: string): string => readFileSync(path, "utf8");
const writeText = (path: string, text: string): void => {mkdirSync(dirname(path), {recursive: true}); writeFileSync(path, text);};
const absolute = (root: string, path: string): string => isAbsolute(path) ? path : resolve(root, path);
const wanted = {go: [/\.go$/, /(^|\/)go\.(mod|work|sum)$/, /\.(c|cc|cpp|cxx|h|hh|hpp|hxx|m|mm|s|S|syso)$/], python: [/\.pyi?$/]} as const;

type Planned = {readonly graph: UnitGraphV1; readonly overlay: Readonly<Record<string, string>> | undefined; readonly files: readonly string[]; readonly modules: readonly PythonModuleV1[]};
async function planFor(language: "go" | "python", root: string, ports: UnitsCommandPortsV1): Promise<Planned> {
  const tracked = (await ports.files(root, ports.signal)).filter(path => wanted[language].some(pattern => pattern.test(path)));
  const cache = new Map<string, string>(), read = (path: string): string => {let text = cache.get(path); if (text === undefined) {text = textOf(join(root, path)); cache.set(path, text);} return text;};
  if (language === "python") {
    const planned = planPythonUnitsV1({files: tracked, read}, {policy, roots: [""], toolchain: (await ports.run(["python3", "--version"], {cwd: root, env: {}, signal: ports.signal})).output.trim() || "python3", pins: {}});
    return {graph: planned.graph, overlay: undefined, files: tracked, modules: planned.modules};
  }
  const [os, arch, cgo] = (await ports.run(["go", "env", "GOOS", "GOARCH", "CGO_ENABLED"], {cwd: root, env: {}, signal: ports.signal})).output.trim().split("\n"), version = (await ports.run(["go", "version"], {cwd: root, env: {}, signal: ports.signal})).output.trim();
  const context: GoBuildContextV1 = {goos: os!, goarch: arch!, goMinor: Number(/go1\.(\d+)/.exec(version)?.[1] ?? 25), cgo: cgo === "1", tags: []};
  const moduleDirs = tracked.filter(path => path === "go.mod" || path.endsWith("/go.mod")).map(path => path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");
  const given = ports.goOverlay ? await ports.goOverlay(root, moduleDirs.map(dir => join(root, dir))) : undefined, overlay = given?.replace;
  const snapshot: GoWorkspaceSnapshotV1 = given ? applyGoOverlayV1({files: tracked, read}, root, given.replace, given.excludedDirectories) : {files: tracked, read};
  const planned = planGoUnitsV1(snapshot, {policy, context, toolchain: version, tests: true});
  return {graph: planned.graph, overlay, files: tracked, modules: []};
}

const summary = (graph: UnitGraphV1): {units: number; edges: number; layers: number} => {
  const plan = planUnitsV1(graph);
  return {units: graph.units.length, edges: graph.units.reduce((sum, unit) => sum + unit.dependencies.length, 0), layers: Math.max(-1, ...Object.values(plan.layer)) + 1};
};

/** 🔎️ Reads the dependency files of an `exec` into facts the key and the executor need. */
function dependencyFacts(unit: UnitV1, parsed: UnitsArgumentsV1, root: string): {facts: Map<string, UnitResultFactsV1>; artifacts: Map<string, string | null>} {
  const interfaces = new Map<string, {closure: string; artifact: string | null}>(), outputs = new Map<string, string>();
  const read = (path: string, name: "InterfaceFile" | "OutFile"): Record<string, any> => {
    const value = JSON.parse(textOf(absolute(root, path))) as Record<string, any>, failures = unitsSchemaFailuresV1(name, value);
    if (failures.length) throw Error(both(`Not a unit ${name} file: ${path} (${failures[0]})`, `Keine ${name}-Datei einer Einheit: ${path} (${failures[0]})`));
    return value;
  };
  if (parsed.outRoot) {
    for (const dependency of unit.dependencies) {
      const name = unitTargetNameV1(dependency.unit);
      if (dependency.use === "interface") {const file = read(join(parsed.outRoot, name, "iface.json"), "InterfaceFile"); interfaces.set(dependency.unit, {closure: file.closure, artifact: file.artifact});}
      if (dependency.use === "implementation") outputs.set(dependency.unit, read(join(parsed.outRoot, name, "out.json"), "OutFile").closure);
    }
  } else {
    for (const path of parsed.ifaceDeps) {const file = read(path, "InterfaceFile"); interfaces.set(file.unit, {closure: file.closure, artifact: file.artifact});}
    for (const path of parsed.implDeps) {const file = read(path, "OutFile"); outputs.set(file.unit, file.closure);}
  }
  const facts = new Map<string, UnitResultFactsV1>(), artifacts = new Map<string, string | null>();
  for (const dependency of unit.dependencies) {
    if (dependency.use === "declared") continue;
    const face = interfaces.get(dependency.unit), out = outputs.get(dependency.unit);
    if (dependency.use === "interface" && !face) throw Error(both(`Missing interface file of ${dependency.unit}`, `Schnittstellendatei von ${dependency.unit} fehlt`));
    if (dependency.use === "implementation" && out === undefined) throw Error(both(`Missing result file of ${dependency.unit}`, `Ergebnisdatei von ${dependency.unit} fehlt`));
    facts.set(dependency.unit, {interfaceClosure: face?.closure ?? null, outputClosure: out ?? ""});
    if (face) artifacts.set(dependency.unit, face.artifact);
  }
  return {facts, artifacts};
}

/** 🏭️ Runs the executor of a unit's language. Hermetic build tools run it inside the sandbox: only the unit's files and the published files of its dependencies exist. */
function executorFor(unit: UnitV1, root: string, scratch: string, ports: UnitsCommandPortsV1, overlay?: string, modules: readonly PythonModuleV1[] = []): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  const read = (path: string): string => textOf(join(root, path));
  if (unit.language === "go") return goUnitExecutorV1({workspace: root, outputRoot: scratch, run: ports.run, ...(overlay ? {overlay} : {})});
  if (unit.language === "python") return pythonUnitExecutorV1({workspace: root, outputRoot: scratch, policy, modules, read, run: ports.run});
  throw Error(both(`No units executor for ${unit.language}: the ${unit.language} units run through their own engine`, `Kein Units-Ausführer für ${unit.language}: Die ${unit.language}-Einheiten laufen über ihre eigene Engine`));
}

/** 🚀️ Runs `repo units`: plans the Go or Python units of the checkout, runs them with the early-cutoff engine, emits the Nx targets or the Bazel build files for them, or executes one unit (the action both build tools run). Progress is published per stage and unit, every stage checks for cancellation. */
export async function runUnitsCommandV1(live: string, args: readonly string[], ports: UnitsCommandPortsV1): Promise<void> {
  const parsed = unitsArgumentsV1(args.slice(args[0] === "units" ? 1 : 0)), lang = ports.language(parsed.language), root = parsed.root ? absolute(live, parsed.root) : live;
  const say = (name: Parameters<typeof unitsMessageV1>[2], values: Record<string, string | number> = {}): string => unitsMessageV1(policy, lang, name, values);
  if (parsed.operation === "exec") {
    ports.check();
    const unit = JSON.parse(textOf(absolute(live, parsed.spec!))) as UnitV1, failures = unitsSchemaFailuresV1("Unit", unit);
    if (failures.length) throw Error(both(`Not a unit spec: ${parsed.spec} (${failures[0]})`, `Keine Einheitenbeschreibung: ${parsed.spec} (${failures[0]})`));
    await ports.progress(`exec:${unit.id}`);
    const {facts, artifacts} = dependencyFacts(unit, parsed, live), scratch = mkdtempSync(join(tmpdir(), "unit-exec-"));
    try {
      const key = unitKeyV1(unit, {digest: path => {try {return sha256V1(readFileSync(join(root, path)));} catch {return "absent";}}, dependencies: facts});
      const outcome = await executorFor(unit, root, scratch, ports)({
        unit, key, signal: ports.signal,
        dependencies: unit.dependencies.map(dependency => ({unit: dependency.unit, use: dependency.use, interfaceClosure: facts.get(dependency.unit)?.interfaceClosure ?? null, outputClosure: facts.get(dependency.unit)?.outputClosure ?? "", artifact: artifacts.get(dependency.unit) ?? null})),
      });
      ports.check();
      const outFile = parsed.outRoot ? join(parsed.outRoot, unitTargetNameV1(unit.id), "out.json") : parsed.out!, ifaceFile = parsed.outRoot ? join(parsed.outRoot, unitTargetNameV1(unit.id), "iface.json") : parsed.iface;
      const closure = sha256V1(canonicalJsonV1({o: outcome.outputs, d: unit.dependencies.filter(dependency => dependency.use === "implementation").map(dependency => [dependency.unit, facts.get(dependency.unit)!.outputClosure]).sort((a, b) => compareCodePointsV1(a[0]!, b[0]!))}));
      writeText(absolute(live, outFile), JSON.stringify({version: 1, unit: unit.id, key, ok: outcome.ok, diagnostics: outcome.diagnostics, outputs: outcome.outputs, closure}) + "\n");
      if (unit.interface && ifaceFile && outcome.interface !== null) {
        const own = interfaceClosureV1(unit, outcome.interface, facts, policy, outcome.exposes ?? null)!;
        writeText(absolute(live, ifaceFile), JSON.stringify(unitInterfaceFileV1(unit, outcome.interface, own, outcome.artifact ?? null)) + "\n");
      }
      ports.write(outcome.ok ? say("ran", {unit: unit.id, milliseconds: outcome.milliseconds ?? 0}) : say("failed", {unit: unit.id}) + (outcome.diagnostics ? "\n" + outcome.diagnostics : ""));
      if (!outcome.ok) throw Error(say("failed", {unit: unit.id}));
    } finally {rmSync(scratch, {recursive: true, force: true});}
    return;
  }
  await ports.progress("plan");
  ports.check();
  const planned = await planFor(parsed.for!, root, ports), graph = readUnitGraphV1(planned.graph, policy), plan = planUnitsV1(graph);
  if (parsed.operation === "plan") {
    const numbers = summary(graph);
    if (parsed.file) writeText(absolute(live, parsed.file), JSON.stringify(graph));
    ports.write(parsed.json ? JSON.stringify(graph, null, 2) : `${say("planning", {units: numbers.units})}: ${numbers.edges} edges, ${numbers.layers} layers`);
    return;
  }
  if (parsed.operation === "emit") {
    const out = absolute(live, parsed.out!), specRoot = "specs";
    await ports.progress("emit");
    for (const [path, text] of Object.entries(unitSpecsV1(plan, specRoot))) writeText(join(out, path), text);
    if (parsed.target === "nx") writeText(join(out, "targets.json"), JSON.stringify(nxUnitTargetsV1(plan, {outputRoot: ".units", specRoot, command: ["repo", "units"]}), null, 2) + "\n");
    else {
      const tracked = new Set(planned.files);
      const extraInputs = (unit: UnitV1): readonly string[] => {
        if (unit.language !== "go") return [];
        const source = unit.sources[0] ?? "";
        for (let directory = source.includes("/") ? source.slice(0, source.lastIndexOf("/")) : ""; ; directory = directory.slice(0, Math.max(0, directory.lastIndexOf("/")))) {
          const mod = directory === "" ? "go.mod" : `${directory}/go.mod`;
          if (tracked.has(mod)) return [mod, ...(tracked.has(mod.replace(/go\.mod$/, "go.sum")) ? [mod.replace(/go\.mod$/, "go.sum")] : [])];
          if (directory === "") return [];
        }
      };
      writeText(join(out, "units.bzl"), UNITS_BZL_V1);
      writeText(join(out, "BUILD.bazel"), bazelUnitsV1(plan, {runner: "//:runner", load: "//:units.bzl", specRoot, policy, extraInputs, environment: {SEMIO_LANGUAGE: lang}}));
    }
    ports.write(`${say("done", {ran: plan.order.length, hit: 0, cutoff: 0, failed: 0})}: ${out}`);
    return;
  }
  const store = parsed.store ? fileUnitsStoreV1(absolute(live, parsed.store)) : memoryUnitsStoreV1();
  const scratch = mkdtempSync(join(tmpdir(), "units-run-"));
  let overlayFile: string | undefined;
  if (planned.overlay) {overlayFile = join(scratch, "overlay.json"); writeText(overlayFile, JSON.stringify({Replace: planned.overlay}));}
  try {
    const executors = new Map<string, (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1>>();
    const host = {
      digest: (path: string): string => {try {return sha256V1(readFileSync(join(root, path)));} catch {return "absent";}},
      execute: (request: ExecuteRequestV1): Promise<ExecuteOutcomeV1> => {
        let executor = executors.get(request.unit.language);
        if (!executor) {executor = executorFor(request.unit, root, scratch, ports, overlayFile, planned.modules); executors.set(request.unit.language, executor);}
        return executor(request);
      },
    };
    const result: UnitsRunResultV1 = await runUnitsV1(plan, policy, host, store, {
      concurrency: parsed.jobs, signal: ports.signal, ...(parsed.only.length ? {only: parsed.only} : {}),
      onProgress: row => {void ports.progress(`${row.status}:${row.unit}`); if (row.status !== "hit" || !parsed.json) ports.write(say(row.status === "hit" ? "hit" : row.status === "failed" ? "failed" : row.status === "blocked" ? "blocked" : row.cutoff ? "cutoff" : "ran", {unit: row.unit, milliseconds: row.milliseconds, dependency: ""}));},
    });
    if (parsed.file) writeText(absolute(live, parsed.file), JSON.stringify(result, null, 2));
    ports.write(parsed.json ? JSON.stringify(result, null, 2) : say("done", {ran: result.ran, hit: result.hit, cutoff: result.cutoff, failed: result.failed}));
    if (result.cancelled) throw Error(say("cancelled"));
    if (result.failed || result.blocked) throw Error(both(`${result.failed} units failed, ${result.blocked} blocked`, `${result.failed} Einheiten fehlgeschlagen, ${result.blocked} blockiert`));
  } finally {rmSync(scratch, {recursive: true, force: true});}
}

/** 🏗️ Binds the command to the running build CLI: its invocation for progress and cancellation, its owned process execution and (Go) the repository's canonical compiler overlay. */
export async function unitsCommandPortsV1(invocation: import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation): Promise<UnitsCommandPortsV1> {
  const routing = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"), execution = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts"), language = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts");
  return Object.freeze({
    write: (line: string) => console.log(line),
    progress: (stage: string) => routing.advanceScriptInvocation(invocation, "units:" + stage, "running"),
    check: () => routing.checkScriptInvocation(invocation),
    signal: invocation.control.signal,
    language: (requested: "en" | "de" | undefined) => requested ?? language.resolveLanguageV1(["en", "de"], [], process.env).language as "en" | "de",
    run: async (argv, options) => {try {return {code: 0, output: await execution.captureOwnedCommandV1(argv[0]!, argv.slice(1), options.cwd, {env: {...process.env, ...options.env}, signal: options.signal})};} catch (error) {return {code: 1, output: String(error instanceof Error ? error.message : error)};}},
    files: (directory: string, signal: AbortSignal) => trackedFiles(directory, signal),
    goOverlay: canonicalGoOverlayV1,
  });
}

/** 🧩️ The compiler overlay of the repository's canonical Go layout for the given module roots, from the library planner that `go-all` uses (member directories become sources, case leaves become tests of their parent package), and the directory names the layout keeps out of the compiler. */
export async function canonicalGoOverlayV1(root: string, modules: readonly string[]): Promise<Readonly<{replace: Readonly<Record<string, string>>; excludedDirectories: readonly string[]}>> {
  const goPlan: string = "../../../🧪️testing/🐹️go-plan/🟦️.ts", goLayout: string = "../../../🔍️discovery/🐹️go-layout/🟦️.ts";
  const {canonicalGoPlan} = await import(goPlan) as {canonicalGoPlan: (moduleRoot: string) => {replacements: Readonly<Record<string, string>>}};
  const {readCanonicalGoLayoutV1} = await import(goLayout) as {readCanonicalGoLayoutV1: () => {testsDirectory: string; sourceDirectoryNames: readonly string[]; opaqueDirectoryNames: readonly string[]}};
  const layout = readCanonicalGoLayoutV1(), replace: Record<string, string> = {};
  for (const module of modules) {
    if (!module.startsWith(root)) continue;
    for (const [virtual, source] of Object.entries(canonicalGoPlan(module).replacements)) {
      if (replace[virtual] !== undefined && replace[virtual] !== source) throw Error(both(`Go input overlay collision: ${virtual}`, `Kollision im Go-Eingabe-Overlay: ${virtual}`));
      replace[virtual] = source;
    }
  }
  return {replace, excludedDirectories: [layout.testsDirectory, ...layout.sourceDirectoryNames, ...layout.opaqueDirectoryNames]};
}

/** 📜️ Lists the tracked and untracked, non-ignored files of a checkout, read-only and cancellable. */
async function trackedFiles(directory: string, signal: AbortSignal): Promise<readonly string[]> {
  const {spawn} = await import("node:child_process");
  return new Promise((accept, reject) => {
    const child = spawn("git", ["-C", directory, "ls-files", "-z", "-co", "--exclude-standard"], {stdio: ["ignore", "pipe", "pipe"], windowsHide: true, signal}), chunks: Buffer[] = [];
    child.stdout.on("data", (chunk: Buffer) => chunks.push(chunk));
    child.on("error", reject);
    child.on("close", code => code === 0 ? accept(Buffer.concat(chunks).toString("utf8").split("\0").filter(Boolean)) : reject(Error(`git ls-files failed (${code})`)));
  });
}

/** 🚪️ The entry the build CLI calls for `repo units`. */
export async function runUnitsCliV1(root: string, args: readonly string[], invocation: import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation): Promise<void> {
  await runUnitsCommandV1(root, args, await unitsCommandPortsV1(invocation));
}
