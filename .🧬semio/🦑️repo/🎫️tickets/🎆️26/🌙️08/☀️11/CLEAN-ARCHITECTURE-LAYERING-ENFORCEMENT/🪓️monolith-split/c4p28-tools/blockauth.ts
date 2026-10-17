// 🧱️ c4p28 block publication-authority harness: the oracle and hostile-mutation functions of the block plugin script, copied verbatim, driven over the editor root read as its module tree (the script itself needs a parent Script process invocation).
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { resolve } from "node:path";
/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-transient" | "child" | "host-only";
type AppAuthority = { owner: string; toolIdsConstant: string; source: string; routes: { id: string; lanes: Lane[] }[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
type Fixture = { schema: string; apps: AppAuthority[] };

/** 🧱️ Every anchor an app's retained/bounded publication apparatus must carry verbatim. */
const ANCHORS = [
  "ToolExecutionContract::bounded_first_step",
  "semio_framework_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "build_artifact_store_one_item_preparation_factory",
  "register_tool_job_factories",
  "build_tool_job",
  "request.operation != request.authority.operation()",
  "request.generation != request.authority.generation()",
  "request.base_revision != request.authority.base_revision()",
  "authority.prepare_one_item",
  "fn cancel(&mut self)",
  "fn begin_close(&mut self)",
  "base.return_to_registry()",
  "fn terminal_is_empty(&self)",
  "LocalizedLabel::native",
  "SelectionMode::Multiple",
  ".default_layout(edit_mode::layout())",
];

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

/** ⚖️ One app's routes must be the same set in the Rust tool-id constant, the publication contracts and the `Migrated` classifications. */
function appOracle(app: AppAuthority, source: string): boolean {
  const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((match) => match[1]!);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => `${match[1]}:${[...match[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((lane) => lane[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  return Object.values(app.laws).every(Boolean)
    && app.ui.locales.join(",") === "en,de" && app.ui.accessibleLabels && app.ui.customizableUi
    && exact(ids, expected) && exact(contracts, app.routes.map(({ id, lanes }) => `${id}:${[...lanes].map(variant).sort().join("+")}`)) && exact(classifications, expected)
    && ANCHORS.every((anchor) => source.includes(anchor));
}

function oracle(fixture: Fixture, sources: Map<string, string>): boolean {
  return fixture.schema === "semio.app.publication-authority.v1" && fixture.apps.length > 0 && fixture.apps.every((app) => appOracle(app, sources.get(app.owner) ?? ""));
}

/** 🗡️ Three source mutations per app, derived from that app's own routes, that MUST break its oracle. */
function hostileSources(app: AppAuthority, source: string): string[] {
  const last = app.routes[app.routes.length - 1]!;
  const second = app.routes[Math.min(1, app.routes.length - 1)]!;
  return [
    source.replace(`ArtifactToolPublicationContract { tool_id: "${last.id}", lanes: &[${last.lanes.map((lane) => `ArtifactToolPublicationLane::${variant(lane)}`).join(", ")}] },`, ""),
    source.replaceAll("request.base_revision != request.authority.base_revision()", ""),
    source.replace(`.action_interactive_job("${second.id}", InteractiveJobClassification::Migrated)`, ""),
  ];
}

const plugin = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block";
const fixture = JSON.parse(await Bun.file(resolve(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json")).text()) as Fixture;
const only = fixture.apps.filter((app) => app.owner === "Block3dPlayApp");
const original = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/c4p28-orig/6.rs";
const source = process.argv[2] === "orig" ? await Bun.file(original).text() : readModuleTreeSource(resolve(plugin, only[0]!.source));
const result = { source: process.argv[2], bytes: source.length, accepts: appOracle(only[0]!, source), hostile: hostileSources(only[0]!, source).map((c) => ({ changed: c !== source, accepted: appOracle(only[0]!, c) })) };
console.log(JSON.stringify(result));
const app = only[0]!;
const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((m) => m[1]!);
const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((m) => `${m[1]}:${[...m[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((l) => l[1]).sort().join("+")}`);
const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((m) => m[1]!);
console.log(JSON.stringify({ ids: ids.length, contracts: contracts.length, classifications: classifications.length, expected: app.routes.length, lawsAll: Object.values(app.laws).every(Boolean), missingAnchors: ANCHORS.filter((a) => !source.includes(a)), idsHash: Bun.hash(ids.join()).toString(), contractsHash: Bun.hash(contracts.join()).toString(), classHash: Bun.hash(classifications.join()).toString() }));
