import { toolForCommandV1, toolGlobMatchesV1, toolLanguageV1, toolMessageV1, toolRawSpawnAllowedV1, TOOL_REGISTRY_V1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";
import { ToolErrorV1, toolCallerFileV1 } from "../🏃️run/🟦️.ts";

/** 🛡️ How the runtime guard answers a direct spawn of a registered tool: refuse it, warn once, or stay silent. */
export type ToolGuardModeV1 = "refuse" | "warn" | "off";
/** 🔌️ What the guard needs: the repository root, the mode and where warnings go. */
export type ToolGuardOptionsV1 = Readonly<{ root: string; mode?: ToolGuardModeV1; write?: (line: string) => void; registry?: ToolRegistryV1; environment?: Readonly<Record<string, string | undefined>> }>;

/** ⚖️ Decides one spawn: `allow` when the command is not registered or the caller is an adapter of the tool or test code, `refuse` or `warn` otherwise. */
export function decideToolSpawnV1(command: string, caller: string | undefined, options: Pick<ToolGuardOptionsV1, "mode" | "registry">): Readonly<{ action: "allow" | "warn" | "refuse"; tool: string | null }> {
  const registry = options.registry ?? TOOL_REGISTRY_V1, entry = toolForCommandV1(command, registry), mode = options.mode ?? "refuse";
  if (entry === undefined || caller === undefined || mode === "off" || toolRawSpawnAllowedV1(entry, caller) || (registry.testCallers ?? []).some(pattern => toolGlobMatchesV1(pattern, caller))) return { action: "allow", tool: entry?.id ?? null };
  return { action: mode, tool: entry.id };
}

const commandOf = (argument: unknown): string | undefined => {
  if (Array.isArray(argument)) return typeof argument[0] === "string" ? argument[0] : undefined;
  if (typeof argument === "object" && argument !== null && Array.isArray((argument as { cmd?: unknown }).cmd)) return commandOf((argument as { cmd: unknown[] }).cmd);
  return undefined;
};

/** 🔒️ Wraps `Bun.spawn`, `Bun.spawnSync` and `Bun.$` so a repository file that starts a registered tool outside its adapter is refused or warned about, naming `runToolV1`. Module imports of `node:child_process` and named Bun imports are linked before any hook can run, so those are covered by the static law, not here. Answers a function that removes the hooks. */
export function installToolGuardV1(options: ToolGuardOptionsV1): () => void {
  const bun = (globalThis as { Bun?: Record<string, unknown> }).Bun;
  if (bun === undefined) return () => undefined;
  const environment = options.environment ?? process.env, registry = options.registry ?? TOOL_REGISTRY_V1, language = toolLanguageV1(environment);
  const mode = options.mode ?? (environment.SEMIO_TOOL_GUARD as ToolGuardModeV1 | undefined) ?? "refuse", write = options.write ?? ((line: string) => { process.stderr.write(line + "\n"); }), warned = new Set<string>();
  const check = (command: string | undefined): void => {
    if (command === undefined) return;
    const caller = toolCallerFileV1(options.root), decision = decideToolSpawnV1(command, caller, { mode, registry });
    if (decision.action === "allow") return;
    const values = { command, caller: caller!, tool: decision.tool! };
    if (decision.action === "refuse") throw new ToolErrorV1("caller-refused", decision.tool!, toolMessageV1("guardRefused", values, language, registry));
    const key = `${caller}\0${decision.tool}`;
    if (!warned.has(key)) { warned.add(key); write(toolMessageV1("guardWarned", values, language, registry)); }
  };
  const originals: Array<[string, unknown]> = [];
  for (const name of ["spawn", "spawnSync"] as const) {
    const original = bun[name] as (...rest: unknown[]) => unknown;
    originals.push([name, original]);
    bun[name] = (...rest: unknown[]) => { check(commandOf(rest[0])); return original.apply(bun, rest); };
  }
  const shell = bun.$ as ((strings: TemplateStringsArray, ...values: unknown[]) => unknown) | undefined;
  if (shell !== undefined) {
    originals.push(["$", shell]);
    const wrapped = Object.assign((strings: TemplateStringsArray, ...values: unknown[]) => { check(String(strings[0] ?? "").trim().split(/\s+/u)[0]); return shell(strings, ...values); }, shell);
    bun.$ = wrapped;
  }
  return () => { for (const [name, original] of originals) bun[name] = original; };
}
