import { createToolAdmissionV1, findToolV1, probeToolV1, resolveRegisteredToolV1, runToolV1, setDefaultToolAdmissionV1, toolLanguageV1, toolMessageV1, toolRootV1, ToolErrorV1, TOOL_REGISTRY_V1, type ToolEntryV1, type ToolResolutionV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 } from "../../🌡️host-capacity/🟦️.ts";

/** 🔌️ What the tools command touches: output, environment, root and the cancellation signal; tests replace them. */
export type ToolsCommandContextV1 = Readonly<{ environment: NodeJS.ProcessEnv; write: (line: string) => void; root?: string; signal?: AbortSignal }>;

const flag = (head: readonly string[], name: string): string | undefined => {
  for (let index = 0; index < head.length; index++) { const argument = head[index]!; if (argument === `--${name}`) return head[index + 1]; if (argument.startsWith(`--${name}=`)) return argument.slice(name.length + 3); }
  return undefined;
};

const describe = (entry: ToolEntryV1, resolution: ToolResolutionV1): string => `${entry.id.padEnd(11)} ${(resolution.pin.version ?? resolution.pin.minimum ?? entry.pin.state).padEnd(26)} ${resolution.source.padEnd(15)} ${resolution.command === null ? "-" : [resolution.command, ...resolution.prefix].join(" ")}`;

/** 📋️ Prints the registered tools with their pin, where each was found on this host and the command that runs it. */
function list(head: readonly string[], context: ToolsCommandContextV1, root: string): number {
  const language = toolLanguageV1(context.environment), rows = TOOL_REGISTRY_V1.tools.map(entry => ({ entry, resolution: resolveRegisteredToolV1(entry, root, { environment: context.environment }) }));
  if (head.includes("--json")) { context.write(JSON.stringify(rows.map(({ entry, resolution }) => ({ id: entry.id, kind: entry.kind, summary: entry.summary, capabilities: entry.capabilities, adapter: entry.adapter, allowedCallers: entry.allowedCallers, pin: resolution.pin, source: resolution.source, command: resolution.command, prefix: resolution.prefix, tried: resolution.tried })), null, 1)); return 0; }
  context.write(toolMessageV1("listHeader", {}, language));
  for (const { entry, resolution } of rows) context.write(describe(entry, resolution));
  return 0;
}

/** 🩺️ Resolves every registered tool (or those named by `--only=a,b`) on this host, asks each for its version and reports a tool that is missing or differs from its pin. */
function verify(head: readonly string[], context: ToolsCommandContextV1, root: string): number {
  const language = toolLanguageV1(context.environment), only = flag(head, "only")?.split(","), problems: string[] = [];
  const entries = TOOL_REGISTRY_V1.tools.filter(entry => (only === undefined || only.includes(entry.id)) && entry.platforms.includes(process.platform as never));
  for (const entry of entries) {
    const resolution = resolveRegisteredToolV1(entry, root, { environment: context.environment });
    if (resolution.command === null) { problems.push(toolMessageV1("verifyProblem", { tool: entry.id, problem: toolMessageV1("toolMissing", { tool: entry.id, tried: resolution.tried.join(", "), command: entry.provision.hints?.[process.platform as "darwin"] ?? entry.provision.command }, language) }, language)); continue; }
    const found = probeToolV1(entry, resolution, context.environment, root), pinned = resolution.pin.version;
    if (entry.pin.state === "pinned" && found !== null && pinned !== null && /^\d+(?:\.\d+){0,2}$/u.test(pinned) && !pinned.startsWith(found) && !found.startsWith(pinned)) problems.push(toolMessageV1("verifyProblem", { tool: entry.id, problem: toolMessageV1("versionMismatch", { tool: entry.id, found, pinned }, language) }, language));
  }
  for (const line of problems) context.write(line);
  if (problems.length === 0) context.write(toolMessageV1("verifyOk", { count: entries.length }, language));
  return problems.length === 0 ? 0 : 1;
}

/** ▶️ Runs a registered tool under machine capacity with the sanitized environment of the registry: `repo tools exec [--cwd=<path>] [--light] <tool> -- <arguments>`. */
async function exec(head: readonly string[], tail: readonly string[], context: ToolsCommandContextV1, root: string): Promise<number> {
  const language = toolLanguageV1(context.environment), id = head.find((word, index) => !word.startsWith("--") && head[index - 1] !== "--cwd"), entry = id === undefined ? undefined : findToolV1(id);
  if (id === undefined) throw new Error(toolMessageV1("execUsage", {}, language));
  if (entry === undefined) throw new ToolErrorV1("unknown-tool", id, toolMessageV1("unknownTool", { tool: id, known: TOOL_REGISTRY_V1.tools.map(tool => tool.id).join(", ") }, language));
  const controller = new AbortController(), stop = (): void => controller.abort(), cwd = flag(head, "cwd") ?? process.cwd();
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  context.signal?.addEventListener("abort", stop, { once: true });
  setDefaultToolAdmissionV1(createToolAdmissionV1(BUILD_HOST_CAPACITY_POLICY_V1, { environment: context.environment, write: line => process.stderr.write(line + "\n") }));
  try {
    const result = await runToolV1(entry.id, tail, { root, cwd, environment: context.environment, signal: controller.signal, check: false, heavy: !head.includes("--light"), stdout: "inherit", trusted: true, onProgress: event => { if (event.phase === "admitting") process.stderr.write(toolMessageV1("admitting", { tool: event.tool }, language) + "\n"); } });
    return result.status ?? (result.signal === null ? 1 : 128 + (result.signal === "SIGINT" ? 2 : 15));
  } finally {
    process.removeListener("SIGINT", stop);
    process.removeListener("SIGTERM", stop);
  }
}

/** 🚪️ Dispatches `repo tools list [--json]`, `repo tools verify [--only=<ids>]` and `repo tools exec [--cwd=<path>] <tool> -- <arguments>`; the exit code of `exec` is the tool's. */
export async function runToolsCommandV1(args: readonly string[], context: ToolsCommandContextV1): Promise<number> {
  const boundary = args.indexOf("--"), head = args.slice(1, boundary < 0 ? undefined : boundary), tail = boundary < 0 ? [] : args.slice(boundary + 1), root = context.root ?? toolRootV1(process.cwd(), context.environment);
  if (args[0] === "list") return list(head, context, root);
  if (args[0] === "verify") return verify(head, context, root);
  if (args[0] === "exec") return exec(head, tail, context, root);
  throw new Error(toolMessageV1("execUsage", {}, toolLanguageV1(context.environment)));
}
