import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, posix } from "node:path";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";

/** 🔒️ The closed set of cache-correctness rules; every finding names exactly one. */
export const CACHE_RULES_V1 = ["unclassified-target", "authored-cache-overridden", "empty-inputs", "producer-without-outputs", "env-unhashed", "named-input-undeclared", "output-escapes", "output-overlap", "command-mutating", "command-network", "uncached-without-reason"] as const;
export type CacheRuleV1 = (typeof CACHE_RULES_V1)[number];

/** 🎯️ The resolved target facts a rule reads: the shape the inference gives a target after its cache policy. */
export interface CacheTargetV1 {
  cache?: boolean;
  executor?: string;
  inputs?: unknown[];
  outputs?: string[];
  options?: { command?: string; commands?: unknown[]; env?: Record<string, string> };
  configurations?: Record<string, { env?: Record<string, string>; command?: string }>;
  metadata?: { semio?: Record<string, any> };
}

/** 🧾️ One resolved target together with the names its inputs may reference. */
export interface CacheRecordV1 { id: string; root: string; authoredCache: boolean | null; target: CacheTargetV1; namedInputs: Record<string, unknown[]> }

/** 🚩️ One violated rule on one target (or, for overlaps, on one pair). */
export interface CacheFindingV1 { id: string; rule: CacheRuleV1; detail: string }

const MANIFEST = "📋️project.json";
const MUTATING = /(?:--fix(?![\w-])|--write(?![\w-])|\bpublish\b|\bdeploy\b|\bgit\s+(?:commit|push|add|checkout|stash|reset)\b)/u;
const MUTATING_EXEMPT = /(?:--check\b|--dry-run\b|\bverify\b)/u;
const NETWORK = /\b(?:curl|wget|git clone|git fetch|npm install|bun install|bun add|bunx|npx|cargo fetch|cargo install|pip install|uv sync|uv pip|apt-get|brew install|go get|go mod download)(?![-\w])/u;

/** 🧷️ Matches a workspace-relative path against one input glob (`*`, `**`, `?`, `{a,b}`); the independent `minimatch` oracle checks it. */
export function globMatchesV1(pattern: string, path: string): boolean {
  const source = pattern.replace(/[.+^$()|\\[\]]/gu, "\\$&").replace(/\{([^{}]*)\}/gu, (_, alternatives: string) => `(?:${alternatives.split(",").join("|")})`).replace(/\*\*\/|\*\*|\*|\?/gu, (token) => token === "**/" ? "(?:.*/)?" : token === "**" ? ".*" : token === "*" ? "[^/]*" : "[^/]");
  return new RegExp(`^${source}$`, "u").test(path);
}

const ALWAYS_HASHED_ENV = new Set(["SEMIO_BUILD_MODE"]);
const WORD = /^[A-Za-z][A-Za-z0-9_-]*$/u;
const expandInputs = (inputs: readonly unknown[], names: Record<string, unknown[]>, seen: ReadonlySet<string> = new Set()): unknown[] => inputs.flatMap((input) => {
  if (typeof input !== "string") return [input];
  const word = input.startsWith("^") ? input.slice(1) : input;
  if (!WORD.test(word)) return [input];
  if (input.startsWith("^") || seen.has(word) || !Object.hasOwn(names, word)) return [];
  return expandInputs(names[word]!, names, new Set([...seen, word]));
});

const projectPath = (pattern: string, root: string): string => pattern.replace(/^\{workspaceRoot\}\//u, "").replaceAll("{projectRoot}/", root === "." ? "" : `${root}/`).replaceAll("{projectRoot}", root);
const coversManifest = (leaves: readonly unknown[], root: string): boolean => {
  const path = root === "." ? MANIFEST : `${root}/${MANIFEST}`, patterns = leaves.filter((leaf): leaf is string => typeof leaf === "string");
  return patterns.some((pattern) => !pattern.startsWith("!") && globMatchesV1(projectPath(pattern, root), path)) && !patterns.some((pattern) => pattern.startsWith("!") && globMatchesV1(projectPath(pattern.slice(1), root), path));
};

const commandOf = (target: CacheTargetV1): string => target.options?.command ?? (Array.isArray(target.options?.commands) ? target.options.commands.map((operation) => typeof operation === "string" ? operation : (operation as { command?: string })?.command ?? "").join(" && ") : "");
const outputPath = (output: string, root: string): string | undefined => {
  const tokens = output.replaceAll(/\{options\.env\.(\w+)\}/gu, "<$1>");
  if (tokens.includes("{options.")) return undefined;
  return tokens.startsWith("/") ? tokens : posix.normalize(projectPath(tokens, root) || ".");
};

/** 🔎️ Runs every per-target rule (the inference appends `{ env: SEMIO_BUILD_MODE }` to every replayed target, so the build mode is always hashed) on one resolved target; a target that never replays only answers for its classification and its authored cache. */
export function cacheCorrectnessFindingsV1(record: CacheRecordV1): CacheFindingV1[] {
  const { id, root, target } = record, findings: CacheFindingV1[] = [], found = (rule: CacheRuleV1, detail: string) => findings.push({ id, rule, detail });
  const decision = target.metadata?.semio?.cache, semio = target.metadata?.semio ?? {};
  if (decision?.class === "unclassified") found("unclassified-target", "no policy row, declaration or verifying command classifies this target");
  if (target.cache === false && decision === undefined) found("uncached-without-reason", "cache false records no decision: declare metadata.semio.cache with the class and reason that forbid replay");
  if (record.authoredCache === true && target.cache === false && decision?.class !== "continuous") found("authored-cache-overridden", `authored cache true, resolved ${decision ? `${decision.class}/${decision.reason}` : "uncached"}`);
  if (target.cache !== true) return findings;
  const leaves = expandInputs(target.inputs ?? ["default", "^default"], record.namedInputs);
  if (Array.isArray(target.inputs) && target.inputs.length === 0 && target.executor !== "repo:noop") found("empty-inputs", "inputs [] hashes nothing but the results of dependencies");
  const declaredOutputs = Array.isArray(target.outputs) && target.outputs.length > 0;
  if (decision?.class === "deterministic" && decision.reason === "pure-function-of-inputs" && !declaredOutputs && semio.rustBuildIntent === undefined && semio.rustUnitAggregate === undefined && semio.hostPlatformOutputsV1 === undefined && target.executor !== "repo:noop") found("producer-without-outputs", "a replayed producer declares no output, so a hit restores nothing");
  const environment = new Set([...Object.keys(target.options?.env ?? {}), ...Object.values(target.configurations ?? {}).flatMap((configuration) => Object.keys(configuration.env ?? {}))]), hashed = new Set(leaves.flatMap((leaf) => typeof leaf === "object" && leaf !== null && typeof (leaf as { env?: unknown }).env === "string" ? [(leaf as { env: string }).env] : []));
  if (environment.size && !coversManifest(leaves, root)) for (const key of [...environment].sort()) if (!hashed.has(key) && !ALWAYS_HASHED_ENV.has(key)) found("env-unhashed", `options.env.${key} is neither an {env} input nor covered by hashing the declaring manifest`);
  for (const input of target.inputs ?? []) if (typeof input === "string") { const word = input.startsWith("^") ? input.slice(1) : input; if (WORD.test(word) && !Object.hasOwn(record.namedInputs, word)) found("named-input-undeclared", `input ${input} names no declared named input`); }
  for (const output of target.outputs ?? []) { const path = outputPath(output, root); if (path !== undefined && (path.startsWith("/") || path === ".." || path.startsWith("../"))) found("output-escapes", `${output} resolves outside the workspace (${path})`); }
  const command = commandOf(target);
  if (MUTATING.test(command) && !MUTATING_EXEMPT.test(command)) found("command-mutating", command.slice(0, 160));
  if (NETWORK.test(command)) found("command-network", command.slice(0, 160));
  return findings;
}

const segments = (path: string) => path.replace(/\/(?:\*\*(?:\/\*)?)?$/u, "").split("/").filter(Boolean);
const nests = (left: string[], right: string[]) => left.length <= right.length ? left.every((part, index) => part === right[index]) : right.every((part, index) => part === left[index]);

/** 🧱️ Two replayed targets that declare the same output tree (or one inside the other) overwrite each other's restore. */
export function cacheOutputOverlapFindingsV1(records: readonly CacheRecordV1[]): CacheFindingV1[] {
  const owners: { id: string; path: string; parts: string[] }[] = [], findings: CacheFindingV1[] = [];
  for (const record of records) if (record.target.cache === true) for (const output of new Set(record.target.outputs ?? [])) { const path = outputPath(output, record.root); if (path !== undefined && !path.startsWith("/") && !path.startsWith("..")) owners.push({ id: record.id, path, parts: segments(path) }); }
  const byHead = new Map<string, typeof owners>();
  for (const owner of owners) { const head = owner.parts[0] ?? ""; (byHead.get(head) ?? byHead.set(head, []).get(head)!).push(owner); }
  for (const group of byHead.values()) for (let left = 0; left < group.length; left++) for (let right = left + 1; right < group.length; right++) {
    const a = group[left]!, b = group[right]!;
    if (a.id !== b.id && a.parts.length && b.parts.length && nests(a.parts, b.parts)) findings.push({ id: a.id, rule: "output-overlap", detail: `${a.path} overlaps ${b.id}: ${b.path}` });
  }
  return findings;
}

/** 🧮️ Every finding of every rule over a set of resolved targets. */
export function cacheCorrectnessRepositoryFindingsV1(records: readonly CacheRecordV1[]): CacheFindingV1[] {
  return [...records.flatMap(cacheCorrectnessFindingsV1), ...cacheOutputOverlapFindingsV1(records)].sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : a.rule < b.rule ? -1 : a.rule > b.rule ? 1 : a.detail < b.detail ? -1 : 1);
}

type PolicyV1 = { targetDefaults?: Record<string, Record<string, unknown>> };
export interface CachePolicyInternalsV1 { targetPolicy(name: string, target: any, policy?: any): any }

/** 📚️ Resolves every authored target of the tracked manifests through the production cache policy (targetDefaults, generator-contract outputs, classification) into the records the rules read. Inference-only projects have no manifest target and are covered by the production outputs gate. */
export function repositoryCacheRecordsV1(repoRoot: string, internals: CachePolicyInternalsV1, tracked?: readonly string[]): CacheRecordV1[] {
  const read = (path: string) => JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", policy: PolicyV1 = read(`${library}/⚡️caching/🔣️policy.json`), nx = read("nx.json").namedInputs ?? {}, contracts: Record<string, any> = read(`${library}/🔣️taxonomy.json`).generatorContracts;
  const injected = new Map<string, string[]>();
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned") continue;
    if (!injected.has(contract.target)) injected.set(contract.target, []);
    for (const output of contract.outputRoots) { const target = output.producer?.target ?? contract.target; (injected.get(target) ?? injected.set(target, []).get(target)!).push(`{workspaceRoot}/${output.path}`); }
  }
  const manifests = (tracked ?? execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", `*${MANIFEST}`], { cwd: repoRoot, maxBuffer: 1 << 28 }).toString("utf8").split("\0")).filter((path) => path && !path.startsWith(".🧬semio/") && existsSync(join(repoRoot, path)));
  const records: CacheRecordV1[] = [];
  for (const manifest of manifests.sort()) {
    let json: any;
    try { json = read(manifest); } catch { continue; }
    const root = posix.dirname(manifest), names: Record<string, unknown[]> = { ...nx, ...json.namedInputs, ...Object.fromEntries(Object.keys(json.metadata?.sourceInputs ?? {}).map((key) => [key, []])) };
    for (const [name, authored] of Object.entries<any>(json.targets ?? {})) {
      const id = `${json.name ?? root}:${name}`, base = { ...(policy.targetDefaults?.[name] ?? {}), ...authored, ...(authored.cache === false && authored.inputs === undefined ? { inputs: ["default"] } : {}), options: { ...authored.options, ...(typeof authored.command === "string" && authored.options?.command === undefined ? { command: authored.command } : {}) } } as any;
      const outputs = injected.get(id);
      if (outputs?.length) base.outputs = outputs;
      records.push({ id, root, authoredCache: typeof authored.cache === "boolean" ? authored.cache : null, target: internals.targetPolicy(name, structuredClone(base)), namedInputs: names });
    }
  }
  return records;
}

/** 🚨️ Turns the findings into policy breaches the verification lanes print. */
export function policyCacheCorrectnessBreaches(findings: readonly CacheFindingV1[]): BreachRecord[] {
  return findings.map((finding) => ({
    id: `cache-correctness-${finding.rule}-${finding.id}`,
    summary: `${finding.rule} ${finding.id}: ${finding.detail}`,
    kind: "repo/cache-correctness",
    scope: finding.id,
    priority: "high",
    autofixable: false,
    reason: "A replayed task whose key omits an input, an environment value or an output restores a stale or empty result for every later build, and a shared remote cache would spread it to every machine.",
    solution: "Declare the missing inputs or outputs on the target, classify it in `⚡️caching/🔣️policy.json` `targetClasses`, or set `cache: false` with the decision that names why it cannot replay.",
  }));
}
