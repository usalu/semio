/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only";
type Route = { id: string; lanes: Lane[]; frameworkInjected?: true };
type AppAuthority = { owner: string; toolIdsConstants: string[]; source: string; routes: Route[]; reservedToolIdsConstant:string; reservedRoutes:Route[]; reservedProducerSource:string; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
type Fixture = { schema: string; apps: AppAuthority[] };
const ANCHORS = [
  "semio_framework_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "ToolExecutionContract::bounded_first_step",
  "ToolExecutionContract::resumable",
  "build_artifact_store_one_item_preparation_factory",
  "fn register_window_config_owners(",
  "fn register_window_transient_owners(",
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
  ".default_layout(edit::layout())",
];

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

const contractPattern = (id: string): RegExp => new RegExp(`ArtifactToolPublicationContract \\{ tool_id: "${id}", lanes: &\\[[^\\]]*\\] \\},?`);

/** ⚖️ One app's routes must be the same set in its tool-id constants, its publication contracts and its
 * `Migrated` classifications — the exact three-way join `validate_tool_job_rows` demands. A route the
 * framework injects already classified is excluded from the classification side only: it still needs
 * its constant row and its publication contract (draw has none today — the shell's utility swap is a
 * window-transient edit, not a routed tool). */
function appOracle(app: AppAuthority, source: string): boolean {
  const ids = app.toolIdsConstants.flatMap((constant) => [...(source.match(new RegExp(`${constant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? [])].map((match) => match[1]!));
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => `${match[1]}:${[...match[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((lane) => lane[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", (?:semio_framework_plugin::)?InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  const authored = app.routes.filter((route) => route.frameworkInjected !== true).map(({ id }) => id);
  const reserved=[...(source.match(new RegExp(`${app.reservedToolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`,"s"))?.[1]?.matchAll(/"([^"]+)"/g)??[])].map(match=>match[1]!);
  return Object.values(app.laws).every(Boolean)
    && exact(reserved,app.reservedRoutes.map(route=>route.id))&&app.reservedRoutes.every(route=>!expected.includes(route.id))&&source.includes("fn build_reserved_tool_job(")&&source.includes(`${app.reservedToolIdsConstant}.contains(&request.tool_id.as_str())`)
    && app.ui.locales.join(",") === "en,de" && app.ui.accessibleLabels && app.ui.customizableUi
    && app.routes.every((route) => route.lanes.length > 0 && (!route.lanes.includes("host-only") || route.lanes.length === 1))
    && exact(ids, expected) && exact(contracts, app.routes.map(({ id, lanes }) => `${id}:${[...lanes].map(variant).sort().join("+")}`)) && exact(classifications, authored)
    && ANCHORS.every((anchor) => source.includes(anchor));
}

function clauses(app: AppAuthority, source: string): string {
  const ids = app.toolIdsConstants.flatMap((constant) => [...(source.match(new RegExp(`${constant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? [])].map((match) => match[1]!));
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => `${match[1]}:${[...match[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((lane) => lane[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", (?:semio_framework_plugin::)?InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  const authored = app.routes.filter((route) => route.frameworkInjected !== true).map(({ id }) => id);
  const reserved = [...(source.match(new RegExp(`${app.reservedToolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? [])].map((match) => match[1]!);
  return JSON.stringify({
    laws: Object.values(app.laws).every(Boolean),
    reserved: exact(reserved, app.reservedRoutes.map((route) => route.id)),
    reservedNotExpected: app.reservedRoutes.every((route) => !expected.includes(route.id)),
    buildReserved: source.includes("fn build_reserved_tool_job("),
    reservedContains: source.includes(`${app.reservedToolIdsConstant}.contains(&request.tool_id.as_str())`),
    ids: exact(ids, expected),
    contracts: exact(contracts, app.routes.map(({ id, lanes }) => `${id}:${[...lanes].map(variant).sort().join("+")}`)),
    classifications: exact(classifications, authored),
    anchorsMissing: ANCHORS.filter((anchor) => !source.includes(anchor)).length,
    counts: [ids.length, expected.length, contracts.length, classifications.length, authored.length, reserved.length],
  });
}

import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const plugin = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw";
const fixture = JSON.parse(readFileSync(join(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json"), "utf8")) as Fixture;
const orig = readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p30-tools/orig/draw-editor.rs.txt", "utf8");
const root = resolve(plugin, fixture.apps[0]!.source);
const base = root.slice(0, root.length - "/🦀️.rs".length);
const own = ["🖐️gesture-owners","⚙️gesture-jobs","🧾️bounded-commands","💾️store-preparation","🔏️proof-catalogs","🖥️play-app"];
const ownText = [readFileSync(root, "utf8"), ...own.map((c) => readFileSync(join(base, c, "🦀️.rs"), "utf8"))].join("\n");
for (const [mode, text] of [["orig", orig], ["file", readFileSync(root, "utf8")], ["own-children", ownText], ["tree", readModuleTreeSource(root)]] as const) {
  console.log(mode, text.length, fixture.apps.map((app) => app.owner + "=" + appOracle(app, text) + " " + clauses(app, text)).join(","));
}
