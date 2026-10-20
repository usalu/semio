import registryData from "./🔣️.json" with { type: "json" };
import registrySchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { resolveBuildHelpLanguageV1 } from "../../🏗️build/⚙️operations/📖️help/🟦️.ts";
import type { ScriptCommand, ScriptInvocation, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🛗️ What a command's process needs before it can start: `none` runs on a fresh clone with only Bun, `dependencies` needs the locked dependencies, which the router provisions first. */
export type RootCommandRequiresV1 = "none" | "dependencies";

/** 🌐️ A text in every language the registry declares. */
export type RootCommandTextV1 = Readonly<Record<string, string>>;

/** 🧭️ One command of the root script as the registry declares it. */
export type RootCommandV1 = Readonly<{ id: string; module: string; export: string; requires: RootCommandRequiresV1; reason: string; synopsis?: string; summary: RootCommandTextV1 }>;

/** 🗂️ The authored registry of every root command. */
export type RootCommandRegistryV1 = Readonly<{ version: 1; languages: readonly string[]; labels: Readonly<Record<string, RootCommandTextV1>>; commands: readonly RootCommandV1[] }>;

/** 🚦️ Provisions the dependencies of one command before its module is loaded. */
export type RootCommandGateV1 = (command: RootCommandV1, invocation: ScriptInvocation) => Promise<void>;

/** 💤️ Loads the implementing class of one command. */
export type RootCommandLoaderV1 = () => Promise<ScriptCommand>;

/** 🎯️ What a help request names: the whole router, or one command. */
export type RootCommandHelpRequestV1 = Readonly<{ command?: string }>;

const contract = compileJsonSchemaSubset(registrySchema), frontend = "bun ./📜️script.ts";
let cached: RootCommandRegistryV1 | undefined;

/** 📋️ Admits a registry that satisfies its schema, with unique command ids and the provisioning owner `repo`. */
export function readRootCommandRegistryV1(value: unknown): RootCommandRegistryV1 {
  const problems = contract.validate(value);
  if (problems.length) throw Error(`Closed root command registry required: ${problems[0]}`);
  const registry = value as RootCommandRegistryV1, ids = new Set<string>();
  for (const command of registry.commands) {
    if (ids.has(command.id)) throw Error(`Root command ${command.id} is declared twice`);
    ids.add(command.id);
  }
  if (!ids.has("repo")) throw Error("The root command registry must declare repo, the owner of dependency provisioning");
  return registry;
}

/** 🗂️ The registry authored next to this module. */
export function rootCommandRegistryV1(): RootCommandRegistryV1 {
  return cached ??= readRootCommandRegistryV1(registryData);
}

function table(rows: readonly (readonly [string, string])[]): string[] {
  const width = Math.max(...rows.map(([left]) => left.length));
  return rows.map(([left, right]) => `  ${left.padEnd(width)}  ${right}`);
}

/** 🖨️ Renders the help of the whole router or of one command in the chosen language. */
export function renderRootCommandHelpV1(registry: RootCommandRegistryV1, id: string | undefined, language: string): string {
  const label = (name: string): string => registry.labels[name]![language]!;
  if (id === undefined) {
    const choice = registry.languages.join("|");
    return [`${frontend} - ${label("tagline")}`, "", `${label("usage")}:`, `  ${frontend} <command> [arguments]`, `  ${frontend} <command> --help [--lang=${choice}]`, "", `${label("commands")}:`, ...table(registry.commands.map((command) => [command.requires === "dependencies" ? `${command.id} *` : command.id, command.summary[language]!] as const)), "", label("legend")].join("\n");
  }
  const command = registry.commands.find((row) => row.id === id);
  if (command === undefined) throw Error(`${label("unknownCommand").replaceAll("{command}", id)} (${registry.commands.map((row) => row.id).join("|")})`);
  return [`${command.id} - ${command.summary[language]!}`, "", `${label("usage")}:`, `  ${frontend} ${command.synopsis ?? `${command.id} [arguments]`}`, "", `${label("requires")}: ${label(command.requires === "none" ? "requiresNone" : "requiresDependencies")}`].join("\n");
}

/** 🔎️ Reads whether the arguments ask for help and for which command; arguments after `--` belong to the command, and arguments without a command ask for the router help. */
export function readRootCommandHelpRequestV1(argv: readonly string[]): RootCommandHelpRequestV1 | undefined {
  const boundary = argv.indexOf("--"), head = argv.slice(0, boundary < 0 ? undefined : boundary), positional: string[] = [];
  for (let index = 0; index < head.length; index++) {
    const argument = head[index]!;
    if (argument === "--lang") index++;
    else if (!argument.startsWith("-")) positional.push(argument);
  }
  if (positional.length > 0 && !head.some((argument) => argument === "--help" || argument === "-h")) return undefined;
  return positional.length === 0 ? {} : { command: positional[0]! };
}

/** 📖️ Renders the help the arguments ask for, or nothing when they ask for none; never loads a command. */
export function rootCommandHelpV1(registry: RootCommandRegistryV1, argv: readonly string[], environment: Readonly<Record<string, string | undefined>>): string | undefined {
  const request = readRootCommandHelpRequestV1(argv);
  return request === undefined ? undefined : renderRootCommandHelpV1(registry, request.command, resolveBuildHelpLanguageV1(registry.languages, argv, environment));
}

/** 🔌️ Registers every command of the registry lazily: the gate runs before the module is loaded, and the loaders must match the registry exactly. */
export function registerRootCommandsV1(router: ScriptRouter, registry: RootCommandRegistryV1, loaders: Readonly<Record<string, RootCommandLoaderV1>>, gate: RootCommandGateV1): ScriptRouter {
  const declared = registry.commands.map((command) => command.id), supplied = Object.keys(loaders);
  const missing = declared.filter((id) => !supplied.includes(id)), extra = supplied.filter((id) => !declared.includes(id));
  if (missing.length || extra.length) throw Error(`Root command loaders must match the registry (missing: ${missing.join(",") || "-"}; unknown: ${extra.join(",") || "-"})`);
  for (const command of registry.commands) router.registerLazy(command.id, async (invocation) => { await gate(command, invocation); return loaders[command.id]!(); });
  return router;
}
