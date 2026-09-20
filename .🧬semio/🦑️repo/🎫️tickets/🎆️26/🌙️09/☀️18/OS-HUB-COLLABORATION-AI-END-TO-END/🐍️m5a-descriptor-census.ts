/**
 * 🔬️ M5a descriptor census — reads the generated plugin registry and every committed
 * `🔣️.json` package descriptor exactly the way `semio-framework-os-mcp`'s
 * `RegistryDiscovery::scan` does, then projects, per plugin, what an agent would see in the
 * MCP capability catalog: audience (declared vs derived), description coverage, argument
 * coverage, destructive marking, and every `Mutation`-kind capability whose id looks like a
 * raw gesture route (the §7.4 misclassification risk).
 *
 * Run: `bun .🧬semio/.../🐍️m5a-descriptor-census.ts [--json]`
 */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const registryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");

type Semantics = {
  audience?: "agent" | "input" | "chrome";
  description?: unknown;
  useWhen?: string[];
  effects?: { destructive?: boolean; writes?: unknown[]; reversible?: boolean };
  policy?: { approval?: string; scopes?: string[] };
};
type Action = { id: string; kind?: string; inPalette?: boolean; args?: unknown[]; semantics?: Semantics; title?: unknown };

/** 🧭️ The derivation `manifest::derive_audience` applies when nothing is declared. */
function deriveAudience(kind: string, inPalette: boolean): string {
  if (kind === "interaction") return "input";
  if (kind === "view" && !inPalette) return "chrome";
  return "agent";
}

/** 🖱️ Lexicon of raw-gesture-route id fragments — a capability id containing one of these is a
 * live-surface event, never an intent verb an agent can reach for. */
export const GESTURE_FRAGMENTS = [
  "pointerdown",
  "pointerup",
  "pointermove",
  "pointercancel",
  "pointerenter",
  "pointerleave",
  "pointerhover",
  "doubleclick",
  "dblclick",
  "mousedown",
  "mouseup",
  "mousemove",
  "dragstart",
  "dragmove",
  "dragend",
  "dragover",
  "dragenter",
  "dragleave",
  "drop",
  "wheel",
  "keydown",
  "keyup",
  "keypress",
  "escape",
  "engagementinput",
  "engagementsubmit",
  "engagementcancel",
  "engagementcommit",
  "commitdraft",
  "canceldraft",
  "updatedraft",
  "gesture",
  "touchstart",
  "touchmove",
  "touchend",
  "hover",
  "brushstroke",
  "strokepoint",
  "applyevents",
];

function looksLikeGestureRoute(id: string): string | undefined {
  const lowered = id.toLowerCase();
  return GESTURE_FRAGMENTS.find((fragment) => lowered.includes(fragment));
}

/** 🗣️ Unwraps the `LocalizedLabel` wire envelope. A described capability serializes as
 * `{"native": {"en": "…", "de": "…"}}` — the locale map is one level down, never at the top.
 * Measured by A3 on 2026-09-20 against the re-described `🗒️note/🔣️.json`: without this unwrap the
 * census reported `described = 0` / `en+de = 0` for a descriptor carrying 40 bilingual descriptions,
 * i.e. it under-reported exactly the plugins a regeneration has already fixed. */
function localeMap(value: unknown): Record<string, unknown> | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Record<string, unknown>;
  const native = record.native;
  if (native && typeof native === "object" && !Array.isArray(native)) return native as Record<string, unknown>;
  return record;
}

function hasText(value: unknown): boolean {
  if (typeof value === "string") return value.trim().length > 0;
  const map = localeMap(value);
  if (!map) return false;
  return Object.values(map).some((entry) => typeof entry === "string" && entry.trim().length > 0);
}

function localeKeys(value: unknown): string[] {
  const map = localeMap(value);
  if (!map) return [];
  return Object.entries(map)
    .filter(([, text]) => typeof text === "string" && text.trim().length > 0)
    .map(([locale]) => locale);
}

function collectActions(descriptor: any): Action[] {
  const out: Action[] = [];
  const seen = new Set<unknown>();
  const walk = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    if (seen.has(node)) return;
    seen.add(node);
    if (Array.isArray(node)) {
      for (const child of node) walk(child);
      return;
    }
    const record = node as Record<string, unknown>;
    if (typeof record.id === "string" && (typeof record.kind === "string" || record.semantics)) {
      if (record.semantics || record.kind) out.push(record as Action);
    }
    for (const [key, child] of Object.entries(record)) {
      if (key === "semantics") continue;
      walk(child);
    }
  };
  walk(descriptor);
  return out;
}

const registry = JSON.parse(readFileSync(registryPath, "utf8")) as Array<{ pluginId: string; cratePath: string }>;
const rows: any[] = [];
const skips: string[] = [];

for (const entry of registry) {
  const ownerRoot = dirname(dirname(join(repoRoot, entry.cratePath)));
  let descriptor: any;
  try {
    descriptor = JSON.parse(readFileSync(join(ownerRoot, "🔣️.json"), "utf8"));
  } catch (error) {
    skips.push(`${entry.pluginId}: ${(error as Error).message.slice(0, 120)}`);
    continue;
  }
  const actions = collectActions(descriptor);
  const unique = new Map<string, Action>();
  for (const action of actions) if (!unique.has(action.id)) unique.set(action.id, action);
  let agent = 0;
  let declared = 0;
  let described = 0;
  let bilingual = 0;
  let withArgs = 0;
  let destructive = 0;
  const gestureRisks: string[] = [];
  const destructiveCandidates: string[] = [];
  for (const action of unique.values()) {
    const semantics = action.semantics ?? {};
    const kind = (action.kind ?? "mutation").toLowerCase();
    const audience = semantics.audience ?? deriveAudience(kind, action.inPalette === true);
    if (semantics.audience) declared += 1;
    if (audience !== "agent") continue;
    agent += 1;
    if (hasText(semantics.description)) described += 1;
    if (localeKeys(semantics.description).length >= 2) bilingual += 1;
    if (Array.isArray(action.args) && action.args.length > 0) withArgs += 1;
    if (semantics.effects?.destructive) destructive += 1;
    const fragment = looksLikeGestureRoute(action.id);
    if (fragment && !semantics.audience) gestureRisks.push(`${action.id} (~${fragment}, kind=${kind})`);
    if (/delete|clear|remove|reset|replace|discard|purge|wipe|drop|truncate|overwrite|setsnapshot|loaddocument|setdocument/i.test(action.id) && !semantics.effects?.destructive) {
      destructiveCandidates.push(`${action.id}`);
    }
  }
  rows.push({
    plugin: entry.pluginId,
    total: unique.size,
    agent,
    declared,
    described,
    bilingual,
    withArgs,
    destructive,
    gestureRisks,
    destructiveCandidates,
  });
}

if (process.argv.includes("--json")) {
  console.log(JSON.stringify({ rows, skips }, null, 2));
} else {
  console.log(`# m5a descriptor census — ${registry.length} registry rows, ${rows.length} decoded, ${skips.length} skipped`);
  console.log(`# utc: ${new Date().toISOString()}`);
  console.log("");
  console.log("| plugin | caps | agent | declared | described | en+de | args | destructive | gesture-risk |");
  console.log("|---|---|---|---|---|---|---|---|---|");
  for (const row of rows.sort((a, b) => b.gestureRisks.length - a.gestureRisks.length || a.plugin.localeCompare(b.plugin))) {
    console.log(`| ${row.plugin} | ${row.total} | ${row.agent} | ${row.declared} | ${row.described} | ${row.bilingual} | ${row.withArgs} | ${row.destructive} | ${row.gestureRisks.length} |`);
  }
  const totals = rows.reduce(
    (acc, row) => ({
      total: acc.total + row.total,
      agent: acc.agent + row.agent,
      declared: acc.declared + row.declared,
      described: acc.described + row.described,
      bilingual: acc.bilingual + row.bilingual,
      withArgs: acc.withArgs + row.withArgs,
      destructive: acc.destructive + row.destructive,
      risks: acc.risks + row.gestureRisks.length,
    }),
    { total: 0, agent: 0, declared: 0, described: 0, bilingual: 0, withArgs: 0, destructive: 0, risks: 0 },
  );
  console.log(`| **TOTAL** | ${totals.total} | ${totals.agent} | ${totals.declared} | ${totals.described} | ${totals.bilingual} | ${totals.withArgs} | ${totals.destructive} | ${totals.risks} |`);
  console.log("");
  console.log("## gesture routes published to agents with no declaration");
  for (const row of rows) {
    if (row.gestureRisks.length === 0) continue;
    console.log(`- **${row.plugin}**: ${row.gestureRisks.join(", ")}`);
  }
  console.log("");
  console.log("## destructive-looking verbs with effects.destructive = false");
  for (const row of rows) {
    if (row.destructiveCandidates.length === 0) continue;
    console.log(`- **${row.plugin}**: ${row.destructiveCandidates.join(", ")}`);
  }
  console.log("");
  console.log(`## skipped descriptors (${skips.length})`);
  for (const skip of skips) console.log(`- ${skip}`);
}
