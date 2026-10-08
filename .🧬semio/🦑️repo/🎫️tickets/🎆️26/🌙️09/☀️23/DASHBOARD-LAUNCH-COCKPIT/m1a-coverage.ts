#!/usr/bin/env bun
/**
 * 🧮️ M-1a coverage proof (ticket input, not codebase code).
 *
 * Maps every retained launch row of `🗑️generated/launch-inventory/inventory.json` (recommendation keep / axis /
 * compound / prompt) to a registry selection `{ id, parameters, extraArgs }`, resolves that selection with an
 * independent implementation of fleet-plan §2.2 against the declarations found in the LIVE owner manifests, and
 * compares the result with the row. Also validates every edited manifest with Ajv against the registry schema,
 * maps the four VS Code compounds and the 79 `.claude/launch.json` entries, and writes
 * `🗑️generated/declarations/coverage.json`. `--check` exits non-zero when a row is uncovered, a manifest is
 * invalid or a referenced target is missing.
 *
 * @see ./fleet-plan.md §2.2 §2.3
 * @see ./m1a-owner-declarations.md
 */
import Ajv2020 from "ajv/dist/2020.js";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { OWNER_EDITS } from "./m1a-declarations.ts";

type Json = Record<string, any>;
type Selection = { id: string; parameters: Record<string, string | boolean>; extraArgs: string[] };
type Resolved = { cmd: string; args: string[]; env: Record<string, string>; cwd: string; nxFlags: string[]; tail: string[]; ready?: Json; requires?: string[]; longRunning?: boolean; builtIn?: boolean; members?: Json[]; stop?: string };
type Difference = { kind: string; detail: string };

const ticket = import.meta.dir;
const root = resolve(ticket, "../../../../../../..");
const read = (path: string): Json => JSON.parse(readFileSync(join(root, path), "utf8"));
const inventory = JSON.parse(readFileSync(join(ticket, "🗑️generated/launch-inventory/inventory.json"), "utf8")) as Json;
const facts = JSON.parse(readFileSync(join(ticket, "🗑️generated/launch-inventory/snapshot/nx-facts.json"), "utf8")) as Json;
const claude = JSON.parse(readFileSync(join(ticket, "🗑️generated/launch-inventory/snapshot/claude-launch.json"), "utf8")) as Json;
const launch = JSON.parse(readFileSync(join(ticket, "🗑️generated/launch-inventory/snapshot/launch.json"), "utf8")) as Json;
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json";
const playgroundsPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json";
const playgrounds = read(playgroundsPath) as Json[];
const VERBS = ["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];
const LEVELS = ["fundamental", "quick", "long", "exhaustive"];

// #region 🔖️Manifests
const snapshotProjects = new Map<string, Json>((facts.main.projects as Json[]).map((project) => [project.name, project]));
const manifestPaths = new Map<string, string>([...snapshotProjects].filter(([, project]) => project.source === "manifest").map(([name, project]) => [name, project.manifestPath]));
for (const edit of OWNER_EDITS) manifestPaths.set(edit.name, edit.file);
const manifestCache = new Map<string, Json | null>();
function manifest(project: string): Json | null {
  if (!manifestCache.has(project)) {
    const path = manifestPaths.get(project);
    manifestCache.set(project, path && existsSync(join(root, path)) ? read(path) : null);
  }
  return manifestCache.get(project)!;
}
const rootDashboard = (manifest("workspace")?.metadata?.semio?.dashboard ?? {}) as Json;
const axes = (rootDashboard.parameters ?? []) as Json[];
const targetDeclaration = (project: string, target: string): Json => manifest(project)?.targets?.[target]?.metadata?.semio?.dashboard ?? {};
const projectDeclaration = (project: string): Json => manifest(project)?.metadata?.semio?.dashboard ?? {};
function targetSource(project: string, target: string): string {
  if (manifest(project)?.targets?.[target]) return "manifest";
  if ((snapshotProjects.get(project)?.targets as [string, Json][] | undefined)?.some(([name]) => name === target)) return "snapshot-manifest";
  if ((facts.graphCopies as Json[])[0].nodes.some((node: Json) => node.name === project && (node.targets as [string, Json][]).some(([name]) => name === target))) return "inferred";
  return "missing";
}
const verbOf = (name: string, declared?: string): string => declared ?? (VERBS.includes(name.split("-")[0]!) ? name.split("-")[0]! : "task");
// #endregion 🔖️Manifests

// #region 🔖️Resolver
function effect(parameter: Json, chosen: string | boolean | undefined, out: { env: Record<string, string>; nxFlags: string[]; args: string[]; values: Record<string, string> }): void {
  const value = chosen ?? parameter.default;
  if (value === undefined || value === false) return;
  if (parameter.kind === "choice") {
    const entry = (parameter.values as Json[]).find((candidate) => candidate.id === value);
    if (!entry) throw new Error(`parameter ${parameter.id}: ${String(value)} is not one of ${(parameter.values as Json[]).map((candidate) => candidate.id).join("|")}`);
    Object.assign(out.env, entry.env ?? {});
    out.nxFlags.push(...(entry.nxFlags ?? []));
    out.args.push(...(entry.args ?? []));
  } else if (parameter.kind === "flag") {
    Object.assign(out.env, parameter.env ?? {});
    out.nxFlags.push(...(parameter.nxFlags ?? []));
    out.args.push(...(parameter.args ?? []));
  } else {
    const text = String(value);
    if (parameter.valueEnv) out.env[parameter.valueEnv] = text;
    else if (parameter.valueFlag) out.args.push(...(parameter.valueFlag.endsWith("=") ? [`${parameter.valueFlag}${text}`] : [parameter.valueFlag, text]));
    else if (parameter.valuePositional) out.args.push(text);
  }
  out.values[parameter.id] = String(value);
}
const substitute = (word: string, values: Record<string, string>): string => word.replace(/\{([a-z0-9._-]+)\}/g, (token, id) => (id in values ? values[id]! : token));
const unique = (words: string[]): string[] => [...new Set(words)];

function applicableAxes(verb: string, owned: Json[]): Json[] {
  return axes.filter((axis) => !owned.some((parameter) => parameter.id === axis.id) && (!axis.appliesTo?.verbs || axis.appliesTo.verbs.includes(verb)));
}
function parametersOf(selection: Selection): Json[] {
  const [kind, rest] = [selection.id.slice(0, selection.id.indexOf(":")), selection.id.slice(selection.id.indexOf(":") + 1)];
  if (kind === "tool") return tool(rest).parameters ?? [];
  if (kind === "group") return applicableAxes(verbOf(group(rest).target), []);
  if (kind === "playground" || kind === "compound") return [];
  const { project, target } = nxId(selection.id);
  const declared = targetDeclaration(project, target);
  return [...(declared.parameters ?? []), ...applicableAxes(verbOf(target, declared.verb), declared.parameters ?? [])];
}
function nxId(id: string): { project: string; target: string } {
  const at = id.lastIndexOf(":");
  return { project: id.slice(0, at), target: id.slice(at + 1) };
}
function tool(reference: string): Json {
  const [project, id] = [reference.slice(0, reference.lastIndexOf("/")), reference.slice(reference.lastIndexOf("/") + 1)];
  const found = (projectDeclaration(project).tools as Json[] | undefined)?.find((candidate) => candidate.id === id);
  if (!found) throw new Error(`tool ${reference} is not declared`);
  return found;
}
function group(reference: string): Json {
  const [project, id] = [reference.slice(0, reference.lastIndexOf("/")), reference.slice(reference.lastIndexOf("/") + 1)];
  const found = (projectDeclaration(project).groups as Json[] | undefined)?.find((candidate) => candidate.id === id);
  if (!found) throw new Error(`group ${reference} is not declared`);
  return found;
}
function compound(reference: string): Json {
  const [project, id] = [reference.slice(0, reference.lastIndexOf("/")), reference.slice(reference.lastIndexOf("/") + 1)];
  const found = (projectDeclaration(project).compounds as Json[] | undefined)?.find((candidate) => candidate.id === id);
  if (!found) throw new Error(`compound ${reference} is not declared`);
  return found;
}
function readyOf(declared: Json | undefined, env: Record<string, string>): Json | undefined {
  if (!declared) return undefined;
  const port = declared.port ?? Number(env[declared.portEnv]);
  if (declared.port !== undefined && declared.portEnv) env[declared.portEnv] = String(declared.port);
  return { port, path: declared.path ?? "" };
}

/** 🧭️ Resolves one selection to the process the registry must start (fleet-plan §2.2). */
function resolveSelection(selection: Selection): Resolved {
  const colon = selection.id.indexOf(":");
  const [kind, rest] = [selection.id.slice(0, colon), selection.id.slice(colon + 1)];
  const out = { env: {} as Record<string, string>, nxFlags: [] as string[], args: [] as string[], values: {} as Record<string, string> };
  if (kind === "playground") return resolvePlayground(rest, selection);
  if (kind === "compound") {
    const declared = compound(rest);
    return { cmd: "", args: [], env: {}, cwd: "", nxFlags: [], tail: [], stop: declared.stop ?? "independent", members: (declared.members as Json[]).map((member) => ({ run: member.run, parameters: member.parameters ?? {}, resolved: resolveSelection({ id: member.run, parameters: Object.fromEntries(Object.entries(member.parameters ?? {}).map(([key, value]) => [key, typeof value === "boolean" ? value : String(value)])), extraArgs: [] }) })) };
  }
  const parameters = parametersOf(selection);
  for (const key of Object.keys(selection.parameters)) if (!parameters.some((parameter) => parameter.id === key)) throw new Error(`${selection.id}: parameter ${key} is not offered (offered: ${parameters.map((parameter) => parameter.id).join(", ")})`);
  for (const parameter of parameters) {
    if (parameter.required && selection.parameters[parameter.id] === undefined && parameter.default === undefined) throw new Error(`${selection.id}: required parameter ${parameter.id} has no value`);
    effect(parameter, selection.parameters[parameter.id], out);
  }
  const nxFlags = unique(out.nxFlags);
  if (kind === "tool") {
    const declared = tool(rest);
    const env = { ...(declared.env ?? {}), ...out.env };
    const command = (declared.command as string[]).map((word) => substitute(word, out.values));
    const ready = readyOf(declared.ready, env);
    return { cmd: command[0]!, args: [...command.slice(1), ...out.args, ...selection.extraArgs], env, cwd: declared.cwd ?? "", nxFlags: [], tail: [...out.args, ...selection.extraArgs], ready, requires: declared.requires, longRunning: declared.continuous === true || ready !== undefined };
  }
  const tail = [...out.args, ...selection.extraArgs];
  if (kind === "group") {
    const declared = group(rest);
    return { cmd: "bun", args: ["nx", "run-many", "-t", declared.target, `--projects=${(declared.projects as string[]).join(",")}`, ...nxFlags, ...(tail.length ? ["--", ...tail] : [])], env: out.env, cwd: "", nxFlags, tail };
  }
  const { project, target } = nxId(selection.id);
  const declared = targetDeclaration(project, target);
  const ready = readyOf(declared.ready, out.env);
  return { cmd: "bun", args: ["nx", "run", selection.id, ...nxFlags, ...(tail.length ? ["--", ...tail] : [])], env: out.env, cwd: "", nxFlags, tail, ready, requires: declared.requires };
}

/** 🛝️ Built-in playground resolution as the launch generator's `playgroundDevEnv` produced it (A-1 owns the real one). */
function resolvePlayground(variant: string, selection: Selection): Resolved {
  const entry = playgrounds.find((candidate) => candidate.variant === variant);
  if (!entry) throw new Error(`playground ${variant} is not in the catalog`);
  const renderer = String(selection.parameters.renderer ?? "react");
  if (renderer === "wgpu-native") return { cmd: "bun", args: ["nx", "run", "@semio-tech/framework-renderer-wgpu:native", "--", variant, ...selection.extraArgs], env: {}, cwd: "", nxFlags: [], tail: [variant, ...selection.extraArgs], builtIn: true, longRunning: true };
  const key = renderer === "react" ? "react" : "wgpu";
  const slot = selection.parameters["user-slot"] === undefined ? undefined : Number(selection.parameters["user-slot"]);
  const port = slot === undefined ? entry.ports[key] : entry.userPorts?.[key]?.[slot - 1];
  if (port === undefined) throw new Error(`playground ${variant}: no port for renderer ${renderer}${slot === undefined ? "" : ` user slot ${slot}`}`);
  const env: Record<string, string> = { S_OS_PORT: String(port), SEMIO_PLUGIN: variant, SEMIO_RENDERER: key, ...(entry.app ? { SEMIO_APP: entry.app } : {}) };
  if (selection.parameters.example !== undefined) {
    if (!(entry.examples as string[]).some((example) => example.replace(/^[^a-z0-9]+/u, "") === selection.parameters.example)) throw new Error(`playground ${variant}: example ${String(selection.parameters.example)} is not in the catalog`);
    env.SEMIO_DEFAULT_EXAMPLE = String(selection.parameters.example);
  }
  if (selection.parameters["app-role"] !== undefined) env.SEMIO_APP_ROLE = String(selection.parameters["app-role"]);
  return { cmd: "bun", args: ["nx", "run", "workspace:dev", "--", variant, ...selection.extraArgs], env, cwd: "", nxFlags: [], tail: [variant, ...selection.extraArgs], ready: { port, path: "" }, builtIn: true, longRunning: true };
}
function playgroundVariant(tokens: readonly string[]): { variant: string; rest: string[] } | null {
  for (let length = tokens.length; length >= 1; length--) {
    const alias = tokens.slice(0, length).join(" ");
    const entry = playgrounds.find((candidate) => candidate.variant === alias || (candidate.aliases as string[]).includes(alias));
    if (entry) return { variant: entry.variant, rest: tokens.slice(length) };
  }
  return null;
}
// #endregion 🔖️Resolver

// #region 🔖️Rows
const inputDefaults = new Map<string, string>((inventory.inputs.list as Json[]).map((input) => [input.id, input.default ?? ""]));
/** 🙋️ Inputs whose empty answer means "leave the option out" (documented on the input itself). */
const OPTIONAL_PROMPTS = new Set(["acceptanceUsers"]);
const literal = (word: string): string => word.replace(/\$\{workspaceFolder\}/g, "{workspace}").replace(/\$\{input:([A-Za-z0-9_.-]+)\}/g, (_token, id) => inputDefaults.get(id) || (OPTIONAL_PROMPTS.has(id) ? "" : `<prompt:${id}>`));
const RETARGET: Record<string, string> = {
  "@semio-tech/repo-cli-rs:daemon": "@semio-tech/repo-dashboard-rs:daemon",
  "@semio-tech/repo-cli-rs:run": "@semio-tech/repo-dashboard-rs:run",
  "@semio-tech/repo-cli-rs:preferences": "@semio-tech/repo-dashboard-rs:preferences",
  "@semio-tech/repo-cli-rs:install": "@semio-tech/repo-dashboard-rs:install",
};
const RUNNER_FLAGS = /^--(output-style|outputStyle|parallel)(=|$)/;
const CACHE_FLAGS = ["--skip-nx-cache", "--skip-remote-cache"];
/** 🧾️ Env a row sets that the canonical command leaves to its owner, with the reason. */
const OWNER_DEFAULT_ENV: Record<string, (row: Json, value: string) => string | null> = {
  OS_HUB_DATA: (_row, value) => (/\/hub-dev\/?$/.test(value) ? "owner default data root `.🧬semio/🌐hub/hub-dev` (hub `DevScript`)" : null),
  OS_HUB_URL: (_row, value) => (value === "http://127.0.0.1:8787" ? "owner default (`os-hub-admin` script)" : null),
  S_OS_PORT: (row, value) => (row.target === "dev-secure-suite" && value === "6066" ? "owner default UI port of `dev secure-suite`" : null),
  SEMIO_PLUGIN: (row, value) => (row.target === "dev-secure-native" && value === "s" ? "owner default plugin of `dev secure-native`" : null),
  PROCTOR_PORT: (row, value) => (value !== "8791" ? null : row.project === "@teaching/architecture-quiz" ? "owner default proctor port (quiz Vite config, `PROCTOR_DEV_PORT`)" : row.project === "@teaching/proctor" && row.target !== "dev" ? "owner default (`PROCTOR_DEV_PORT`)" : null),
  SEMIO_RENDERER: (row, value) => (["@semio-tech/mit-bestand-demonstrator", "@semio-tech/semio-tech-play"].includes(row.project) && value === "react" ? "set by the `bun nx` wrapper for this target (`resolveNxInvocation`)" : null),
  SERVER_PORT: (_row, value) => (value === "6277" ? "MCP inspector default proxy port" : null),
  SEMIO_DEBUG_CLOSE_PHASE: () => null,
  S_HUB_URL: (row, value) => (row.target === "local-hub-owner" && value === "http://127.0.0.1:8787" ? "owner default hub URL (`DEV_LOCAL_HUB_DEFAULT_URL`)" : null),
};

function axisParameters(row: Json, offered: Json[], differences: Difference[], tail: string[]): Record<string, string | boolean> {
  const chosen: Record<string, string | boolean> = {};
  const has = (id: string): boolean => offered.some((parameter) => parameter.id === id);
  const rowAxes = (row.axes ?? {}) as Record<string, string>;
  const via = (row.axisVia ?? {}) as Record<string, string[]>;
  if (rowAxes["build-mode"] === "ship") {
    if (has("build-mode")) chosen["build-mode"] = "ship";
    else differences.push({ kind: "not-offered", detail: "SEMIO_BUILD_MODE=ship: the build-mode axis is not offered on this command" });
  }
  if (rowAxes["cache-policy"]) {
    const want = rowAxes["cache-policy"] === "skip-local" ? "skip-local" : "skip-all";
    if (chosen["build-mode"] === "ship") {
      if (want === "skip-local") differences.push({ kind: "axis-spelling", detail: "row skips only the local cache; build-mode=ship always skips both caches" });
    } else if (has("cache")) chosen.cache = want;
    else if (row.runner === "nx-exec") differences.push({ kind: "runner-noise", detail: "cache flags on `nx exec` have no effect (nothing is cached)" });
    else differences.push({ kind: "not-offered", detail: `cache=${want}: the cache axis is not offered on this command` });
    if ((via["cache-policy"] ?? []).some((spelling) => spelling.startsWith("env:"))) differences.push({ kind: "axis-spelling", detail: "NX_SKIP_NX_CACHE / NX_SKIP_REMOTE_CACHE env pair is expressed by the cache flags" });
  }
  if (rowAxes["test-level"]) {
    const level = rowAxes["test-level"];
    const spellings = via["test-level"] ?? [];
    if (spellings.includes("positional")) {
      const at = tail.indexOf(level);
      if (at === 0) tail.splice(0, 1);
      differences.push({ kind: "axis-spelling", detail: `positional test level \`${level}\` is expressed as SEMIO_TEST_LEVEL (both reach \`resolveTestLevel\`)` });
    }
    if (spellings.length === 1 && spellings[0] === "target-name") {
      // the target command pins the level itself
    } else if (has("test-level")) chosen["test-level"] = level;
    else differences.push({ kind: "not-read", detail: `SEMIO_TEST_LEVEL=${level}: this target is no test (\`${row.target ?? "group"}\`); its script does not read the level` });
  }
  if (rowAxes["task-dependencies"]) {
    if (has("dependencies")) chosen.dependencies = true;
    else if (row.runner === "nx-exec") differences.push({ kind: "runner-noise", detail: "--excludeTaskDependencies on `nx exec` has no effect (no task graph)" });
    else differences.push({ kind: "not-offered", detail: "--excludeTaskDependencies: the dependencies axis is not offered on this command" });
  }
  for (const [axis, id] of [["nextest-output", "nextest-output"], ["cargo-jobs", "cargo-jobs"], ["build-budget", "build-budget"]] as const) {
    if (!rowAxes[axis]) continue;
    if (has(id)) chosen[id] = rowAxes[axis]!;
    else differences.push({ kind: "not-offered", detail: `${axis}=${rowAxes[axis]}: the axis is not offered on this command` });
  }
  if (rowAxes["output-style"]) differences.push({ kind: "runner-noise", detail: `--output-style=${rowAxes["output-style"]}: the dashboard owns output rendering` });
  return chosen;
}

/** 🎛️ Moves row arguments and env that a target-owned parameter expresses out of `tail`/`env` into `chosen`. */
function ownedParameters(owned: Json[], tail: string[], env: Record<string, string>, chosen: Record<string, string | boolean>, differences: Difference[] = []): void {
  for (const parameter of owned) {
    if (parameter.kind === "text") {
      if (parameter.valueEnv && env[parameter.valueEnv] !== undefined) {
        if (env[parameter.valueEnv] !== "") chosen[parameter.id] = env[parameter.valueEnv]!;
        delete env[parameter.valueEnv];
      } else if (parameter.valueFlag) {
        const at = tail.indexOf(parameter.valueFlag);
        if (at >= 0) {
          const value = tail[at + 1] ?? "";
          tail.splice(at, value === "" && at + 1 >= tail.length ? 1 : 2);
          if (value !== "") chosen[parameter.id] = value;
          else differences.push({ kind: "owner-default", detail: `${parameter.valueFlag} with an empty answer is left out; the owner then uses its own default` });
        }
      } else if (parameter.valuePositional && tail.length && !tail[0]!.startsWith("-")) chosen[parameter.id] = tail.shift()!;
      continue;
    }
    const candidates = parameter.kind === "flag" ? [{ ...parameter, id: true }] : (parameter.values as Json[]);
    for (const candidate of candidates) {
      const args = (candidate.args ?? []) as string[];
      const envPairs = Object.entries((candidate.env ?? {}) as Record<string, string>);
      if (!args.length && !envPairs.length) continue;
      const at = args.length ? (args[0]!.startsWith("-") ? tail.indexOf(args[0]!) : tail[0] === args[0] ? 0 : -1) : 0;
      const argsMatch = !args.length || (at >= 0 && args.every((word, offset) => tail[at + offset] === word));
      const envMatch = envPairs.every(([key, value]) => env[key] === literalEnv(value));
      if (!argsMatch || !envMatch) continue;
      if (args.length) tail.splice(at, args.length);
      for (const [key] of envPairs) delete env[key];
      chosen[parameter.id] = candidate.id;
      break;
    }
  }
}
const literalEnv = (value: string): string => value;

type Mapped = { selection: Selection; differences: Difference[]; rowEnv: Record<string, string>; rowTail: string[]; rowFlags: string[] };

function mapNxRun(row: Json): Mapped {
  const differences: Difference[] = [];
  const env: Record<string, string> = Object.fromEntries(Object.entries((row.env ?? {}) as Record<string, string>).map(([key, value]) => [key, literal(value)]));
  const rowEnv = { ...env };
  let tail = [...(row.targetArgs as string[]), ...(row.passthru as string[])].map(literal);
  if ((row.targetArgs as string[]).length) differences.push({ kind: "axis-spelling", detail: `\`${(row.targetArgs as string[]).join(" ")}\` stood before \`--\`; Nx forwards unknown options to the command, the registry places them after \`--\`` });
  const rowTail = [...tail];
  const rowFlags = (row.nxFlags as string[]).filter((flag) => !RUNNER_FLAGS.test(flag));
  let id = `${row.project}:${row.target}`;
  if (RETARGET[id]) {
    differences.push({ kind: "retargeted", detail: `${id} moved to ${RETARGET[id]} (the dashboard left the repo CLI package after the inventory snapshot)` });
    id = RETARGET[id]!;
  }
  const { project, target } = nxId(id);
  // playground entry points
  if (id === "workspace:dev" && tail[0] === "mcp") return mapMcp(row, tail, env, differences, rowTail, rowFlags);
  if (id === "workspace:dev" || id === "@semio-tech/framework-renderer-wgpu:native" || /^@semio-tech\/framework-os-dev:serve-.+-react-dev$/.test(id)) {
    const serve = /^@semio-tech\/framework-os-dev:serve-(.+)-react-dev$/.exec(id);
    const resolved = serve ? { variant: serve[1]!, rest: ["served"] } : playgroundVariant(tail);
    if (!resolved && id === "@semio-tech/framework-renderer-wgpu:native" && tail[0] === "trinity") {
      differences.push({ kind: "stale", detail: "`native -- trinity` names no catalog variant or alias; the row means the jack playground (`trinity-jack`)" });
      return finishPlayground(row, { variant: "trinity-jack", rest: tail.slice(1) }, "wgpu-native", env, differences, rowTail, rowFlags);
    }
    if (!resolved) throw new Error(`row ${row.index}: ${tail.join(" ")} names no playground`);
    const renderer = id.endsWith(":native") ? "wgpu-native" : serve ? "react" : env.SEMIO_RENDERER === "wgpu" ? "wgpu-wasm" : "react";
    if (serve) differences.push({ kind: "axis-spelling", detail: `inferred target ${id} is \`workspace:dev -- ${serve[1]} served\` under SEMIO_RENDERER=react (\`resolveNxInvocation\`)` });
    return finishPlayground(row, resolved, renderer, env, differences, rowTail, rowFlags);
  }
  const declared = targetDeclaration(project, target);
  const owned = (declared.parameters ?? []) as Json[];
  const offered = [...owned, ...applicableAxes(verbOf(target, declared.verb), owned)];
  const chosen: Record<string, string | boolean> = {};
  ownedParameters(owned, tail, env, chosen, differences);
  Object.assign(chosen, axisParameters(row, offered, differences, tail));
  if (chosen["test-level"] === undefined && ["quick", "long", "exhaustive"].includes(tail[0] ?? "") && offered.some((parameter) => parameter.id === "test-level") && !row.axes?.["test-level"]) {
    chosen["test-level"] = tail.shift()!;
    differences.push({ kind: "axis-spelling", detail: `positional test level \`${String(chosen["test-level"])}\` is expressed as SEMIO_TEST_LEVEL (both reach \`resolveTestLevel\`)` });
  }
  for (const key of ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"]) delete rowEnv[key];
  for (const [key, value] of Object.entries(rowEnv)) {
    const reason = OWNER_DEFAULT_ENV[key]?.(row, value);
    if (!reason) continue;
    differences.push({ kind: "owner-default", detail: `${key}=${value}: ${reason}` });
    delete rowEnv[key];
  }
  for (const key of ["SEMIO_DEBUG_CLOSE_PHASE", "SEMIO_CARGO_PREPARATION_TIMING"]) if (rowEnv[key] !== undefined) {
    differences.push({ kind: "launch-only", detail: `${key}=${rowEnv[key]}: one-off diagnostic switch of a single probe row; no parameter declares it (a launch accepts extra arguments, not extra environment)` });
    delete rowEnv[key];
  }
  if (rowEnv.SEMIO_TEST_LEVEL !== undefined && chosen["test-level"] === undefined && differences.some((difference) => difference.kind === "not-read")) delete rowEnv.SEMIO_TEST_LEVEL;
  if (target === "time-travel") {
    const drop = (flag: string, value: string, why: string): void => {
      const at = tail.indexOf(flag);
      if (at >= 0 && tail[at + 1] === value) {
        tail.splice(at, 2);
        differences.push({ kind: "owner-default", detail: `${flag} ${value}: ${why}` });
      }
    };
    drop("--serve", chosen.renderer === "wgpu" ? "http://127.0.0.1:6112/" : "http://127.0.0.1:6012/", "owner default serve of the renderer (`DEFAULT_SERVES`)");
    drop("--locales", "en,de", "owner default");
    drop("--chords", "en,de", "owner default");
    drop("--renderer", "react", "owner default");
  }
  if (target === "boot-watch") {
    const at = tail.indexOf("--restarts");
    if (at >= 0 && tail[at + 1] === "1") {
      tail.splice(at, 2);
      differences.push({ kind: "owner-default", detail: "--restarts 1: owner default" });
    }
  }
  return { selection: { id, parameters: chosen, extraArgs: tail }, differences, rowEnv, rowTail, rowFlags };
}

function finishPlayground(row: Json, resolved: { variant: string; rest: string[] }, renderer: string, env: Record<string, string>, differences: Difference[], rowTail: string[], rowFlags: string[]): Mapped {
  const entry = playgrounds.find((candidate) => candidate.variant === resolved.variant)!;
  const parameters: Record<string, string | boolean> = { renderer };
  const key = renderer === "react" ? "react" : "wgpu";
  let rest = [...resolved.rest];
  if (renderer !== "wgpu-native") {
    const port = env.S_OS_PORT;
    const slot = port ? ((entry.userPorts?.[key] ?? []) as number[]).indexOf(Number(port)) : -1;
    if (slot >= 0) parameters["user-slot"] = String(slot + 1);
    else if (port && Number(port) !== entry.ports[key]) differences.push({ kind: "conflict", detail: `row S_OS_PORT=${port} is not the catalog port ${entry.ports[key]}` });
    if (env.SEMIO_DEFAULT_EXAMPLE) parameters.example = env.SEMIO_DEFAULT_EXAMPLE;
    if (env.SEMIO_APP_ROLE) parameters["app-role"] = env.SEMIO_APP_ROLE;
    for (const knob of Object.keys(env).filter((name) => /_PLAY_PORT$/.test(name))) {
      differences.push({ kind: "dead-knob", detail: `${knob}=${env[knob]} has no reader; the server binds S_OS_PORT (catalog port ${entry.ports[key]})` });
      const pattern = /:(\d+)\)/.exec(row.ready?.pattern ?? "")?.[1];
      if (pattern && Number(pattern) !== entry.ports[key]) differences.push({ kind: "conflict", detail: `the row waits for port ${pattern}; the server binds ${entry.ports[key]}, so its ready action never fired` });
    }
    for (const name of ["S_HUB_URL", "S_DATA_DIR", "S_LOCAL_ONLY", "PLAYGROUND_LOCKED_EXAMPLE_ID"]) if (env[name] !== undefined) differences.push({ kind: "launch-only", detail: `${name}=${env[name]} exists only in the launch seed; the playground catalog has no field for it` });
    const suffix = (row.ready?.uriFormat ?? "%s").replace("%s", "");
    if (suffix) differences.push({ kind: "launch-only", detail: `ready URL suffix \`${suffix}\` exists only in the launch row` });
    if (rest[0] === "fixture") {
      differences.push({ kind: "not-read", detail: `\`${rest.join(" ")}\`: nothing in os-dev interprets a \`fixture\` segment (leftover segments only reach the dev server as server options); the row is the plain variant` });
      rest = [];
    }
  }
  if (Object.keys(row.axes ?? {}).includes("cache-policy")) differences.push({ kind: "not-offered", detail: "--skip-nx-cache: no cache axis on a playground (dev servers are never cached)" });
  return { selection: { id: `playground:${resolved.variant}`, parameters, extraArgs: rest }, differences, rowEnv: env, rowTail, rowFlags };
}

function mapMcp(row: Json, tail: string[], env: Record<string, string>, differences: Difference[], rowTail: string[], rowFlags: string[]): Mapped {
  const words = tail.join(" ");
  if (words === "mcp") {
    if (env.SERVER_PORT === "6277") differences.push({ kind: "owner-default", detail: "SERVER_PORT=6277: MCP inspector default proxy port" });
    differences.push({ kind: "axis-spelling", detail: "`workspace:dev -- mcp` is the declared target `workspace:dev-mcp`" });
    return { selection: { id: "workspace:dev-mcp", parameters: {}, extraArgs: [] }, differences, rowEnv: Object.fromEntries(Object.entries(env).filter(([key]) => key !== "SERVER_PORT")), rowTail: [], rowFlags };
  }
  if (words === "mcp repo") {
    differences.push({ kind: "axis-spelling", detail: "`workspace:dev -- mcp repo` is the declared target `workspace:dev-mcp-repo`" });
    differences.push({ kind: "literal-token", detail: "MCP_PROXY_AUTH_TOKEN is a literal token and is never stored; the inspector generates one per session" });
    differences.push({ kind: "launch-only", detail: "MCP_AUTO_OPEN_ENABLED=false and the token-carrying ready URL belong to the fixed token; the inspector opens its own tokenised URL" });
    return { selection: { id: "workspace:dev-mcp-repo", parameters: {}, extraArgs: [] }, differences, rowEnv: {}, rowTail: [], rowFlags };
  }
  if (words === "mcp stdio os" || words === "mcp http os") {
    const id = words === "mcp http os" ? "tool:workspace/os-mcp-http" : "tool:workspace/os-mcp-stdio";
    if (words === "mcp stdio os") differences.push({ kind: "direct", detail: "a stdio server is started without the Nx wrapper so nothing but the protocol reaches its stdout" });
    return { selection: { id, parameters: {}, extraArgs: [] }, differences, rowEnv: env, rowTail: [], rowFlags };
  }
  if (tail[1] === "stdio" && tail.length === 3) {
    differences.push({ kind: "direct", detail: "a stdio server is started without the Nx wrapper so nothing but the protocol reaches its stdout" });
    return { selection: { id: "tool:workspace/repo-mcp", parameters: { client: tail[2]! }, extraArgs: [] }, differences, rowEnv: env, rowTail: [tail[2]!], rowFlags };
  }
  throw new Error(`row ${row.index}: unmapped dev mcp form ${words}`);
}

const GROUP_ROWS: Record<number, { id: string; note?: Difference }> = {
  201: { id: "process-extension-catalogs" },
  202: { id: "sourcing-extension-catalogs" },
  203: { id: "extension-catalogs" },
  308: { id: "process-extension-tests" },
  312: { id: "sourcing-extension-tests" },
  704: { id: "stdio-artifact-tests", note: { kind: "axis-spelling", detail: "`--exclude=X` is the negated project pattern `!X` of the group" } },
  1009: { id: "snapshot-sqlite-parent-baselines" },
  1361: { id: "dag-actor-wasm" },
  2206: { id: "puzzle-spatial-tests" },
  2445: { id: "stdio-artifact-tests", note: { kind: "merged", detail: "row runs the filter on jpg, tiff and pdf only; the group runs it on every stdio artifact (superset)" } },
  2446: { id: "snapshot-sqlite-native", note: { kind: "merged", detail: "row lists 48 projects; the group runs every project that declares the target (101 at the snapshot)" } },
  2447: { id: "snapshot-sqlite-source", note: { kind: "merged", detail: "row lists 48 projects; the group runs every project that declares the target (101 at the snapshot)" } },
};
function shellWords(text: string): string[] {
  return [...text.matchAll(/"([^"]*)"|'([^']*)'|(\S+)/g)].map((match) => match[1] ?? match[2] ?? match[3]!);
}
function mapRunMany(row: Json): Mapped {
  const entry = GROUP_ROWS[row.index as number];
  if (!entry) throw new Error(`row ${row.index}: unmapped nx run-many row`);
  const differences: Difference[] = entry.note ? [entry.note] : [];
  const env: Record<string, string> = { ...(row.env ?? {}) };
  const argsFlag = (row.targetArgs as string[]).find((word) => word.startsWith("--args="));
  const tail = [...(argsFlag ? shellWords(argsFlag.slice("--args=".length)) : []), ...(row.passthru as string[]).map(literal)];
  const rowTail = [...tail];
  const id = `group:workspace/${entry.id}`;
  const offered = parametersOf({ id, parameters: {}, extraArgs: [] });
  const chosen = axisParameters(row, offered, differences, tail);
  for (const key of ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"]) delete env[key];
  if ((row.nxFlags as string[]).some((flag) => flag.startsWith("--parallel"))) differences.push({ kind: "runner-noise", detail: `${(row.nxFlags as string[]).find((flag) => flag.startsWith("--parallel"))}: Nx parallelism is a runner setting, not part of a group` });
  return { selection: { id, parameters: chosen, extraArgs: tail }, differences, rowEnv: env, rowTail, rowFlags: (row.nxFlags as string[]).filter((flag) => CACHE_FLAGS.includes(flag) || flag === "--excludeTaskDependencies") };
}

const EXEC_TARGETS: Record<number, string> = {
  232: "@semio-tech/cad-cad-rs:test-snapshot-sqlite-source",
  234: "@semio-tech/cad-cad-rs:verify-cad-document-contract",
  235: "@semio-tech/cad-cad-rs:verify-snapshot-sqlite-source",
};
/** 🧷️ Exec rows whose script command has no dedicated target: the base target plus extra arguments. */
const EXEC_TARGET_ARGS: Record<number, { id: string; extraArgs: string[] }> = { 2231: { id: "@semio-tech/stdio-docx-rs:test-snapshot-sqlite", extraArgs: ["source"] } };
function mapExec(row: Json): Mapped {
  const differences: Difference[] = [];
  const env: Record<string, string> = { ...(row.env ?? {}) };
  const command = (row.execCommand as string[]).map(literal);
  const rowFlags = (row.nxFlags as string[]).filter((flag) => !flag.startsWith("--projects="));
  const target = EXEC_TARGETS[row.index as number] ?? EXEC_TARGET_ARGS[row.index as number]?.id;
  const extraArgs = EXEC_TARGET_ARGS[row.index as number]?.extraArgs ?? [];
  if (target) {
    differences.push({ kind: "axis-spelling", detail: `\`nx exec … bun 📜️script.ts ${command.slice(2).join(" ")}\` is the declared target ${target}${extraArgs.length ? ` with extra arguments ${extraArgs.join(" ")}` : ""}` });
    const { project, target: name } = nxId(target);
    const offered = [...applicableAxes(verbOf(name), [])];
    const chosen = axisParameters({ ...row, runner: "nx-run", project, target: name }, offered, differences, []);
    for (const key of ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"]) delete env[key];
    return { selection: { id: target, parameters: chosen, extraArgs }, differences, rowEnv: env, rowTail: [...extraArgs], rowFlags };
  }
  for (const flag of rowFlags) differences.push({ kind: "runner-noise", detail: `${flag} on \`nx exec\` has no effect` });
  for (const key of ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"]) if (env[key] !== undefined) {
    delete env[key];
    differences.push({ kind: "runner-noise", detail: `${key} on \`nx exec\` has no effect` });
  }
  if (command[0] === "bun" && command[1] === "test") {
    const files = command.slice(2);
    const project = String(row.project);
    const file = files[0]!.startsWith("{workspace}/") && project === "workspace" ? `./${files[0]!.slice("{workspace}/".length)}` : files[0]!;
    if (file !== files[0]) differences.push({ kind: "axis-spelling", detail: "absolute test file path is given workspace-relative (`nx exec --projects=workspace` runs in the workspace root)" });
    const parameters: Record<string, string | boolean> = { file };
    if (project !== "workspace") parameters.project = project;
    if (env.SEMIO_BUILD_MODE === "ship") parameters["build-mode"] = "ship";
    if (env.NEXTEST_SUCCESS_OUTPUT !== undefined) {
      differences.push({ kind: "not-offered", detail: `NEXTEST_SUCCESS_OUTPUT=${env.NEXTEST_SUCCESS_OUTPUT}: global axes are offered on Nx commands only, not on a tool` });
      delete env.NEXTEST_SUCCESS_OUTPUT;
    }
    return { selection: { id: "tool:workspace/bun-test", parameters, extraArgs: files.slice(1) }, differences, rowEnv: env, rowTail: [file, ...files.slice(1)], rowFlags: [] };
  }
  const cargoScript = "{workspace}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
  if (command[0] === "bun" && command[1] === cargoScript && command[2] === "native" && command[3] === "cargo") {
    differences.push({ kind: "axis-spelling", detail: "the cargo wrapper script is addressed workspace-relative" });
    return { selection: { id: "tool:workspace/native-cargo", parameters: {}, extraArgs: command.slice(4) }, differences, rowEnv: env, rowTail: command.slice(4), rowFlags: [] };
  }
  throw new Error(`row ${row.index}: unmapped nx exec row ${command.join(" ")}`);
}

const TOOL_ROWS: Record<number, Selection & { note?: Difference }> = {
  32: { id: "tool:workspace/repo-mcp", parameters: { client: "client" }, extraArgs: [] },
  185: { id: "tool:workspace/gemini", parameters: {}, extraArgs: [] },
  186: { id: "tool:workspace/f3d", parameters: {}, extraArgs: [] },
  354: { id: "tool:workspace/kiro", parameters: {}, extraArgs: [] },
  355: { id: "tool:workspace/gitkraken", parameters: {}, extraArgs: [] },
  361: { id: "tool:workspace/mcp-inspector-os", parameters: {}, extraArgs: [] },
};

function mapRow(row: Json): Mapped {
  if (row.runner === "nx-run") return mapNxRun(row);
  if (row.runner === "nx-other") return mapRunMany(row);
  if (row.runner === "nx-exec") return mapExec(row);
  const fixed = TOOL_ROWS[row.index as number];
  if (!fixed) throw new Error(`row ${row.index}: unmapped ${row.runner} row`);
  return { selection: { id: fixed.id, parameters: fixed.parameters, extraArgs: fixed.extraArgs }, differences: fixed.note ? [fixed.note] : [], rowEnv: { ...(row.env ?? {}) }, rowTail: [], rowFlags: [] };
}

/** ⚖️ Compares the resolved selection with what is left of the row after its recorded differences. */
function compare(row: Json, mapped: Mapped, resolved: Resolved): Difference[] {
  const faults: Difference[] = [];
  const expectedEnv = { ...resolved.env };
  const rowEnv = { ...mapped.rowEnv };
  if (resolved.builtIn) {
    for (const key of ["S_OS_PORT", "SEMIO_PLUGIN", "SEMIO_RENDERER", "SEMIO_APP"]) {
      if (rowEnv[key] === undefined && expectedEnv[key] !== undefined) mapped.differences.push({ kind: "registry-derived", detail: `${key}=${expectedEnv[key]} comes from the playground catalog; the row left it out` });
      else if (rowEnv[key] !== undefined && rowEnv[key] !== expectedEnv[key]) faults.push({ kind: "MISMATCH", detail: `env ${key}: row ${rowEnv[key]} resolved ${expectedEnv[key] ?? "(unset)"}` });
      delete rowEnv[key];
      delete expectedEnv[key];
    }
    for (const key of ["S_HUB_URL", "S_DATA_DIR", "S_LOCAL_ONLY", "PLAYGROUND_LOCKED_EXAMPLE_ID", ...Object.keys(rowEnv).filter((name) => /_PLAY_PORT$/.test(name))]) delete rowEnv[key];
    if (row.runner === "nx-run" && row.target !== "dev" && row.target !== "native") return faults;
  }
  for (const key of new Set([...Object.keys(rowEnv), ...Object.keys(expectedEnv)])) {
    if (rowEnv[key] === expectedEnv[key]) continue;
    if (rowEnv[key] === undefined && resolved.ready && Object.values(resolved.env).includes(String(resolved.ready.port)) && expectedEnv[key] === String(resolved.ready.port)) {
      mapped.differences.push({ kind: "owner-default", detail: `${key}=${expectedEnv[key]} is exported from the declared ready port; the row relied on the owner default` });
      continue;
    }
    if (key === "SEMIO_TEST_LEVEL" && rowEnv[key] === undefined && mapped.differences.some((difference) => difference.detail.startsWith("positional test level")) && mapped.rowTail.includes(expectedEnv[key]!)) continue;
    if (rowEnv[key] === undefined && isDefaultOnly(mapped.selection, key)) {
      mapped.differences.push({ kind: "owner-default", detail: `${key}=${expectedEnv[key]} is the declared default; the row left it to the owner` });
      continue;
    }
    faults.push({ kind: "MISMATCH", detail: `env ${key}: row ${rowEnv[key] ?? "(unset)"} resolved ${expectedEnv[key] ?? "(unset)"}` });
  }
  if (row.runner === "nx-run" && !mapped.selection.id.startsWith("tool:") && !resolved.builtIn) {
    const rowFlags = new Set(mapped.rowFlags);
    const implied = new Set(resolved.nxFlags);
    for (const flag of rowFlags) if (!implied.has(flag)) faults.push({ kind: "MISMATCH", detail: `nx flag ${flag} of the row is not resolved` });
    for (const flag of implied) if (!rowFlags.has(flag) && !(CACHE_FLAGS.includes(flag) && ((row.axisVia?.["cache-policy"] ?? []) as string[]).some((spelling) => spelling.startsWith("env:")))) faults.push({ kind: "MISMATCH", detail: `resolved nx flag ${flag} is not on the row` });
  }
  return faults;
}
/** 🧷️ Words a selection resolves to only because a parameter declares a default. */
function defaultWords(selection: Selection): string[] {
  if (/^(playground|compound):/.test(selection.id)) return [];
  const out = { env: {} as Record<string, string>, nxFlags: [] as string[], args: [] as string[], values: {} as Record<string, string> };
  for (const parameter of parametersOf(selection)) if (selection.parameters[parameter.id] === undefined && parameter.default !== undefined) effect(parameter, undefined, out);
  return out.args;
}
function isDefaultOnly(selection: Selection, envKey: string): boolean {
  return parametersOf(selection).some((parameter) => parameter.default !== undefined && selection.parameters[parameter.id] === undefined && (parameter.valueEnv === envKey || (parameter.values as Json[] | undefined)?.some((value) => value.id === parameter.default && envKey in (value.env ?? {}))));
}
/** 🧵️ The row's words after `--`, normalised the way the selection expresses them, must equal the resolved tail as a multiset. */
function tailFault(row: Json, mapped: Mapped, resolved: Resolved): Difference | null {
  if (resolved.builtIn || row.runner === "external" || row.runner === "bun-x" || row.runner === "bun-script") return null;
  const dropped = mapped.differences.filter((difference) => ["owner-default", "axis-spelling", "not-read", "runner-noise"].includes(difference.kind)).length;
  const rowWords = [...mapped.rowTail].filter((word) => word !== "").sort();
  const resolvedWords = [...resolved.tail].sort();
  const missing = rowWords.filter((word, index) => rowWords.indexOf(word) === index && resolvedWords.filter((other) => other === word).length < rowWords.filter((other) => other === word).length);
  const explained = (word: string): boolean => LEVELS.includes(word) || mapped.differences.some((difference) => difference.detail.includes(word));
  const unexplained = missing.filter((word) => !explained(word));
  if (unexplained.length) return { kind: "MISMATCH", detail: `row words ${unexplained.join(" ")} are neither resolved nor explained (${dropped} recorded differences)` };
  const extra = resolvedWords.filter((word, index) => resolvedWords.indexOf(word) === index && rowWords.filter((other) => other === word).length < resolvedWords.filter((other) => other === word).length);
  const defaults = defaultWords(mapped.selection);
  const surplus = extra.filter((word) => !defaults.includes(word));
  if (surplus.length) return { kind: "MISMATCH", detail: `resolved words ${surplus.join(" ")} are not on the row` };
  for (const word of extra) mapped.differences.push({ kind: "owner-default", detail: `\`${word}\` comes from a declared parameter default; the row left it to the owner` });
  return null;
}
// #endregion 🔖️Rows

// #region 🔖️Validation
const schema = read(schemaPath);
const ajv = new (Ajv2020 as any)({ allErrors: true, strict: false });
ajv.addSchema(schema);
const validateManifest = ajv.getSchema(`${schema.$id}#/$defs/ProjectManifest`);
const validation = OWNER_EDITS.map((edit) => {
  const document = read(edit.file);
  const valid = validateManifest(document) as boolean;
  const declared = [document.metadata?.semio?.dashboard ? 1 : 0, Object.values(document.targets ?? {}).filter((target: any) => target.metadata?.semio?.dashboard).length];
  return { file: edit.file, project: edit.name, valid, projectDeclaration: declared[0] === 1, targetDeclarations: declared[1], errors: valid ? [] : (validateManifest.errors ?? []).map((error: Json) => `${error.instancePath} ${error.message}`) };
});
const references: { from: string; reference: string; status: string }[] = [];
function checkReference(from: string, reference: string): void {
  const kind = reference.slice(0, reference.indexOf(":"));
  let status = "ok";
  try {
    if (kind === "playground") {
      if (!playgrounds.some((entry) => entry.variant === reference.slice("playground:".length))) status = "missing playground";
    } else if (kind === "tool") tool(reference.slice("tool:".length));
    else {
      const { project, target } = nxId(reference);
      status = targetSource(project, target) === "missing" ? "missing target" : "ok";
    }
  } catch (error) {
    status = error instanceof Error ? error.message : String(error);
  }
  references.push({ from, reference, status });
}
for (const edit of OWNER_EDITS) {
  const document = read(edit.file);
  for (const [name, target] of Object.entries((document.targets ?? {}) as Record<string, Json>)) for (const reference of target.metadata?.semio?.dashboard?.requires ?? []) checkReference(`${edit.name}:${name}`, reference);
  for (const entry of document.metadata?.semio?.dashboard?.compounds ?? []) for (const member of entry.members) checkReference(`compound:${edit.name}/${entry.id}`, member.run);
  for (const entry of document.metadata?.semio?.dashboard?.tools ?? []) for (const reference of entry.requires ?? []) checkReference(`tool:${edit.name}/${entry.id}`, reference);
  for (const entry of document.metadata?.semio?.dashboard?.groups ?? []) for (const project of entry.projects as string[]) if (!/[*!]/.test(project) && !snapshotProjects.has(project)) references.push({ from: `group:${edit.name}/${entry.id}`, reference: project, status: "missing project" });
}
// #endregion 🔖️Validation

// #region 🔖️Coverage
const retained = (inventory.records as Json[]).filter((row) => !String(row.recommendation).startsWith("drop"));
const rows: Json[] = [];
const uncovered: Json[] = [];
for (const row of retained) {
  try {
    const mapped = mapRow(row);
    const resolved = resolveSelection(mapped.selection);
    const faults = compare(row, mapped, resolved);
    const tail = tailFault(row, mapped, resolved);
    if (tail) faults.push(tail);
    const colon = mapped.selection.id.indexOf(":");
    const kind = ["playground", "tool", "group", "compound"].includes(mapped.selection.id.slice(0, colon)) ? mapped.selection.id.slice(0, colon) : "target";
    const source = kind === "target" ? targetSource(nxId(mapped.selection.id).project, nxId(mapped.selection.id).target) : kind;
    if (source === "missing") faults.push({ kind: "MISMATCH", detail: `${mapped.selection.id} is declared nowhere (manifests, snapshot graph)` });
    const semantic = mapped.differences.filter((difference) => ["launch-only", "merged", "conflict", "stale", "not-offered", "dead-knob", "literal-token"].includes(difference.kind));
    rows.push({
      index: row.index,
      name: row.name,
      family: `${row.class} / ${row.family}`,
      recommendation: row.recommendation,
      row: { command: row.command, env: row.env, ready: row.ready ? { pattern: row.ready.pattern, uriFormat: row.ready.uriFormat } : null },
      id: mapped.selection.id,
      parameters: mapped.selection.parameters,
      extraArgs: mapped.selection.extraArgs,
      kind,
      source,
      expected: { cmd: resolved.cmd, args: resolved.args, env: resolved.env, cwd: resolved.cwd, ...(resolved.ready ? { ready: resolved.ready } : {}), ...(resolved.requires ? { requires: resolved.requires } : {}), ...(resolved.builtIn ? { builtIn: true } : {}) },
      match: faults.length ? "MISMATCH" : mapped.differences.length === 0 ? "exact" : semantic.length ? "covered-with-deviation" : "equivalent",
      differences: [...mapped.differences, ...faults],
    });
  } catch (error) {
    uncovered.push({ index: row.index, name: row.name, family: `${row.class} / ${row.family}`, command: row.command, reason: error instanceof Error ? error.message : String(error) });
  }
}

const COMPOUNDS: Record<string, { id: string; note: string }> = {
  "🧭️compound🖥️s⚛️react🗄️os-hub": { id: "compound:workspace/s-with-hub", note: "member rows set S_HUB_URL and S_DATA_DIR on the shell; a compound member carries parameters only (launch-only facts, see report)" },
  "🧭️compound🖥️s⚛️react🌉️os-mcp": { id: "compound:workspace/s-with-os-mcp", note: "the os-mcp member declares no ready, so the shell starts without waiting, as in VS Code" },
  "🧭️compound🖥️s👥️users🗄️os-hub": { id: "compound:workspace/s-users-with-hub", note: "user slots 1 and 2 take their ports from the catalog `userPorts`; S_DATA_DIR per slot is a launch-only fact" },
  "🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor": { id: "compound:@teaching/architecture-quiz/quiz-with-proctor", note: "the quiz `dev` reuses the proctor that already answers, so waiting for the proctor's ready line makes the order deterministic" },
};
const compounds = (launch.compounds as Json[]).map((entry) => {
  const mapping = COMPOUNDS[entry.name];
  if (!mapping) return { name: entry.name, match: "UNCOVERED" };
  const resolved = resolveSelection({ id: mapping.id, parameters: {}, extraArgs: [] });
  return { name: entry.name, members: entry.configurations, stopAll: entry.stopAll, id: mapping.id, expected: { stop: resolved.stop, members: resolved.members!.map((member) => ({ run: member.run, parameters: member.parameters, cmd: member.resolved.cmd, args: member.resolved.args, env: member.resolved.env, ready: member.resolved.ready ?? null })) }, match: resolved.members!.length === entry.configurations.length && (resolved.stop === "together") === (entry.stopAll === true) ? "covered" : "MISMATCH", note: mapping.note };
});

function mapClaude(entry: Json, index: number): Json {
  const base = { index, name: entry.name, port: entry.port ?? null, url: entry.url ?? null };
  if (!entry.runtimeExecutable) {
    const port = entry.port ?? Number(new URL(entry.url).port);
    const owner = playgrounds.find((candidate) => candidate.ports.react === port || candidate.ports.wgpu === port);
    return { ...base, kind: "attach-only", id: owner ? `playground:${owner.variant}` : null, parameters: owner ? { renderer: owner.ports.react === port ? "react" : "wgpu-wasm" } : {}, verdict: owner ? "attach to the running playground (`semio open`); no command" : `port ${port} belongs to no catalog playground (ticket lane); drop` };
  }
  const args = (entry.runtimeArgs ?? []) as string[];
  const text = `${entry.runtimeExecutable} ${args.join(" ")}`;
  if (/🎫️tickets\//.test(text)) return { ...base, kind: "ticket-script", id: null, verdict: "ticket-scoped script; belongs in the ticket's 🎮️commands.json (M-1b) or drops" };
  if (args[0] !== "nx") {
    if (/❓️quiz\/📦️packages\/🟦️typescript\/📜️script\.ts dev$/.test(text)) return { ...base, kind: "script", id: "@teaching/architecture-quiz:dev", verdict: "second quiz stack beside the first (ports 6063/8793, own PROCTOR_DATA): the Nx target with environment overrides the registry does not declare" };
    return { ...base, kind: "script", id: null, verdict: existsSync(join(root, args[0] ?? "")) ? "script outside Nx without a declared owner" : "script file no longer exists; drop" };
  }
  const reference = args[2]!;
  const tail = args.includes("--") ? args.slice(args.indexOf("--") + 1) : [];
  const flags = args.slice(3, args.includes("--") ? args.indexOf("--") : undefined);
  const env = Object.fromEntries(Object.entries((entry.env ?? {}) as Record<string, string>).filter(([key]) => !/^NX_(DAEMON|ISOLATE_PLUGINS|CACHE_PROJECT_GRAPH|PLUGIN_NO_TIMEOUTS)$/.test(key)));
  const notes: string[] = [];
  const playground = (variant: string, renderer: string, extraArgs: string[]): Json => {
    const resolved = resolveSelection({ id: `playground:${variant}`, parameters: { renderer }, extraArgs });
    if (resolved.ready && entry.port !== undefined && resolved.ready.port !== entry.port) notes.push(`entry port ${entry.port} differs from the catalog port ${resolved.ready.port}`);
    return { ...base, kind: "playground", id: `playground:${variant}`, parameters: { renderer }, extraArgs, expected: { cmd: resolved.cmd, args: resolved.args, env: resolved.env, ready: resolved.ready ?? null }, verdict: notes.length ? notes.join("; ") : "covered" };
  };
  if (reference === "workspace:dev" || reference === "@semio-tech/framework-os-dev:dev") {
    if (tail[0] === "storybook" || tail[0] === "storybook-static") {
      const target = `workspace:dev-${tail.join("-")}`;
      const declared = targetSource("workspace", target.slice("workspace:".length)) !== "missing";
      if (entry.port !== undefined && entry.port !== 6010 && env.STORYBOOK_PORT === undefined) notes.push(`entry expects port ${entry.port} but sets no STORYBOOK_PORT; the server binds 6010`);
      return { ...base, kind: "target", id: declared ? target : "workspace:dev", parameters: {}, extraArgs: declared ? [] : tail, verdict: [declared ? "covered" : "no dedicated target; `workspace:dev` with extra arguments", ...notes].join("; ") };
    }
    const resolved = playgroundVariant(tail);
    if (!resolved) return { ...base, kind: "target", id: reference, extraArgs: tail, verdict: "names no playground" };
    const explicit = env.SEMIO_RENDERER;
    const served = resolved.rest.includes("served");
    const entryRenderer = explicit === "wgpu" ? "wgpu-wasm" : explicit === "react" || served ? "react" : playgrounds.find((candidate) => candidate.variant === resolved.variant)!.ports.react === entry.port ? "react" : "wgpu-wasm";
    if (!explicit && !served && entryRenderer === "react") notes.push("entry sets no SEMIO_RENDERER: the owner default is wgpu (`frameworkOsPlaygroundDevEnv`), so the command binds the wgpu port, not the React port the entry names");
    return playground(resolved.variant, entryRenderer, resolved.rest);
  }
  const lane = /^@semio-tech\/framework-os-dev:(serve|dev)-(.+)-(react|wgpu)-dev$/.exec(reference);
  if (lane) return playground(lane[2]!, lane[3] === "react" ? "react" : "wgpu-wasm", lane[1] === "serve" ? ["served"] : []);
  if (reference === "@semio-tech/framework-renderer-wgpu:native") return playground(tail[0]!, "wgpu-native", tail.slice(1));
  const { project, target } = nxId(reference);
  const source = targetSource(project, target);
  if (source === "missing") return { ...base, kind: "target", id: reference, verdict: "target no longer exists; drop" };
  const selection: Selection = { id: reference, parameters: {}, extraArgs: tail };
  const offered = parametersOf(selection);
  const chosen = selection.parameters;
  ownedParameters((targetDeclaration(project, target).parameters ?? []) as Json[], selection.extraArgs, env, chosen);
  if (flags.includes("--skip-nx-cache") && offered.some((parameter) => parameter.id === "cache")) chosen.cache = "skip-local";
  if (env.SEMIO_TEST_LEVEL && offered.some((parameter) => parameter.id === "test-level")) {
    chosen["test-level"] = env.SEMIO_TEST_LEVEL;
    delete env.SEMIO_TEST_LEVEL;
  }
  const resolved = resolveSelection(selection);
  for (const [key, value] of Object.entries(env)) {
    if (resolved.env[key] === value) continue;
    if (key === "SEMIO_TEST_ARTIFACT_DIR") notes.push("SEMIO_TEST_ARTIFACT_DIR is an output location (ticket or 🗑️generated path), not stored");
    else if (key === "PROCTOR_PORT" && value === "8791") notes.push("PROCTOR_PORT=8791 is the owner default");
    else notes.push(`${key}=${value} is not declared (entry-only override)`);
  }
  return { ...base, kind: "target", id: reference, parameters: chosen, extraArgs: selection.extraArgs, expected: { cmd: resolved.cmd, args: resolved.args, env: resolved.env, ready: resolved.ready ?? null }, verdict: notes.length ? notes.join("; ") : "covered" };
}
const claudeEntries = (claude.configurations as Json[]).map(mapClaude);
// #endregion 🔖️Coverage

// #region 🔖️Output
const tally = (items: Json[], key: (item: Json) => string): Record<string, number> => items.reduce((counts: Record<string, number>, item) => ({ ...counts, [key(item)]: (counts[key(item)] ?? 0) + 1 }), {});
const sha = (path: string): string => createHash("sha256").update(readFileSync(path)).digest("hex").slice(0, 16);
const declarations = OWNER_EDITS.flatMap((edit) => {
  const document = read(edit.file);
  const project = document.metadata?.semio?.dashboard ?? {};
  return [
    ...(project.parameters ?? []).map((entry: Json) => ({ file: edit.file, project: edit.name, kind: "axis", id: entry.id })),
    ...(project.tools ?? []).map((entry: Json) => ({ file: edit.file, project: edit.name, kind: "tool", id: `tool:${edit.name}/${entry.id}` })),
    ...(project.compounds ?? []).map((entry: Json) => ({ file: edit.file, project: edit.name, kind: "compound", id: `compound:${edit.name}/${entry.id}` })),
    ...(project.groups ?? []).map((entry: Json) => ({ file: edit.file, project: edit.name, kind: "group", id: `group:${edit.name}/${entry.id}` })),
    ...Object.entries((document.targets ?? {}) as Record<string, Json>).filter(([, target]) => target.metadata?.semio?.dashboard).map(([name, target]) => ({ file: edit.file, project: edit.name, kind: "target", id: `${edit.name}:${name}`, declares: Object.keys(target.metadata.semio.dashboard), parameters: (target.metadata.semio.dashboard.parameters ?? []).map((parameter: Json) => parameter.id) })),
  ];
}).map((declaration) => ({ ...declaration, rows: rows.filter((row) => row.id === declaration.id || (declaration.kind === "axis" && declaration.id in row.parameters && !(parametersOwn(row.id) as string[]).includes(declaration.id))).length }));
function parametersOwn(id: string): string[] {
  if (/^(playground|group|compound):/.test(id)) return [];
  if (id.startsWith("tool:")) return ((tool(id.slice("tool:".length)).parameters ?? []) as Json[]).map((parameter) => parameter.id);
  const { project, target } = nxId(id);
  return ((targetDeclaration(project, target).parameters ?? []) as Json[]).map((parameter) => parameter.id);
}
const output = {
  generatedBy: ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/m1a-coverage.ts",
  inputs: { inventory: sha(join(ticket, "🗑️generated/launch-inventory/inventory.json")), registrySchema: sha(join(root, schemaPath)), playgroundCatalog: sha(join(root, playgroundsPath)) },
  resolution: {
    target: "cmd `bun`, args `nx run <id> <nxFlags…> [-- <args…> <extraArgs…>]`; parameters apply in the order target-owned (declaration order) then global axes (declaration order); nxFlags are deduplicated; a declared ready with `port` and `portEnv` exports `<portEnv>=<port>`",
    group: "cmd `bun`, args `nx run-many -t <target> --projects=<a,b,…> <nxFlags…> [-- <extraArgs…>]`; axes apply by the verb of the group target",
    tool: "cmd `command[0]`, args `command[1…]` with `{<parameter id>}` substituted, then parameter args, then extraArgs; global axes do not apply",
    playground: "built in (A-1): `nx run workspace:dev -- <variant> <extraArgs…>` with S_OS_PORT / SEMIO_PLUGIN / SEMIO_RENDERER / SEMIO_APP from the catalog; renderer wgpu-native is `nx run @semio-tech/framework-renderer-wgpu:native -- <variant>`",
    tokens: "`{workspace}` stays a token in this file; the runner substitutes the workspace root",
    match: { exact: "selection resolves to the row", equivalent: "same process and meaningful environment; recorded differences are spelling, runner noise or repeated owner defaults", "covered-with-deviation": "covered, but a fact of the row is deliberately not reproduced (see differences: launch-only, merged, conflict, stale, not-offered, dead-knob, literal-token)" },
  },
  totals: {
    retainedRows: retained.length,
    covered: rows.filter((row) => row.match !== "MISMATCH").length,
    byMatch: tally(rows, (row) => row.match),
    byKind: tally(rows, (row) => row.kind),
    byRecommendation: tally(rows, (row) => `${row.recommendation} → ${row.match}`),
    differenceKinds: tally(rows.flatMap((row) => row.differences as Json[]), (difference) => difference.kind),
    uncovered: uncovered.length,
    distinctCommandIds: new Set(rows.map((row) => row.id)).size,
    compounds: tally(compounds, (entry) => entry.match),
    claude: tally(claudeEntries, (entry) => `${entry.kind}${entry.id ? "" : " (no command)"}`),
  },
  validation: { schema: schemaPath, manifests: validation, references },
  declarations,
  rows,
  uncovered,
  compounds,
  claude: claudeEntries,
};
const outDir = join(ticket, "🗑️generated/declarations");
mkdirSync(outDir, { recursive: true });
writeFileSync(join(outDir, "coverage.json"), `${JSON.stringify(output, null, 1)}\n`);
console.log(JSON.stringify(output.totals, null, 1));
console.log(`manifests valid: ${validation.filter((entry) => entry.valid).length}/${validation.length}; references not ok: ${references.filter((entry) => entry.status !== "ok").length}`);
for (const entry of validation.filter((candidate) => !candidate.valid)) console.log("INVALID", entry.file, entry.errors.join(" | "));
for (const entry of references.filter((candidate) => candidate.status !== "ok")) console.log("REFERENCE", JSON.stringify(entry));
for (const row of rows.filter((candidate) => candidate.match === "MISMATCH").slice(0, 60)) console.log("MISMATCH", row.index, row.id, JSON.stringify(row.parameters), row.differences.filter((difference: Json) => difference.kind === "MISMATCH").map((difference: Json) => difference.detail).join(" || "), "<<", row.row.command.slice(0, 140));
for (const row of uncovered.slice(0, 60)) console.log("UNCOVERED", row.index, row.reason, "<<", row.command.slice(0, 140));
if (process.argv.includes("--check") && (uncovered.length || rows.some((row) => row.match === "MISMATCH") || validation.some((entry) => !entry.valid) || references.some((entry) => entry.status !== "ok") || compounds.some((entry) => entry.match !== "covered"))) process.exit(1);
// #endregion 🔖️Output
