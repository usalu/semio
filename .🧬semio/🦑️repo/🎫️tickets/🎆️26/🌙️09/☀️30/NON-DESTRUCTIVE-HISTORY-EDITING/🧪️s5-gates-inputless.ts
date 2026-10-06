/** 📭️ S5-GATES: the `inputless` rule of the `schema-mutation-input-ui` gate (design §22.20) over EVERY mutation leaf on disk —
 * catalogued or not, so a stale central catalogue hides none — as the owner work list `📓️s5-gates-inputless.md`: every editable
 * leaf that shows no input (it has none, or every one is hidden), grouped by owner work package and plugin, with the descriptor
 * to mark (`"editable": false`) and whether the catalogue reads the leaf; then the leaves already declared withdraw-only.
 * A leaf is what the derive calls one: a directory below a `🧬️mutations` whose `🔣️.json` descriptor names an `aggregateVariant` and
 * a `payloadSchema`. The verdict is the gate's own (`mutationInputDeclarations` + `mutationInputDeclarationFindings`); `$ref`s
 * resolve through every schema document below a `🧬️schema` directory of the two source roots (first `$id` wins).
 *
 *   bun 🧪️s5-gates-inputless.ts [<output.md>] */
import { type Dirent, existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { mutationInputDeclarationFindings, mutationInputDeclarations, mutationInputWidgetVocabulary } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts";

type Json = Record<string, unknown>;
type Leaf = { readonly schema: string; readonly descriptor: string; readonly plugin: string; readonly artifact: string; readonly name: string };

const repoRoot = decodeURIComponent(new URL("../../../../../../../", import.meta.url).pathname).replace(/\/$/u, "");
const OWNERS: Readonly<Record<string, readonly string[]>> = {
  "S5-TEXT-STDIO": ["stdio", "writer", "trinity", "vcs"],
  "S5-GRAPHS-WIRES": ["dag", "sequence", "imperative", "space", "reasoning", "mathematical"],
  "S5-FLOWCAD": ["flow", "cad"],
  "S5-STROKES-NORM": ["raster", "remodel", "wfc", "process", "norm"],
  "S5-TOOLS": ["draw", "note", "layout", "fem", "lowpoly", "shooting", "energy", "forms", "gis", "procedural", "playbook", "block", "animate", "sourcing", "architect", "demonstrator"],
  "S5-PUZZLE": ["puzzle"],
};
const SKIPPED = new Set(["node_modules", "target", "dist", "🧫️fixtures", "🧪️tests", "🗑️generated"]);
const bare = (name: string): string => name.replace(/^[^\p{L}\p{N}]+/u, "");
const read = (path: string): Json | null => {
  try {
    const value: unknown = JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
    return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Json) : null;
  } catch {
    return null;
  }
};

const documents = new Map<string, Json>();
const leaves: Leaf[] = [];
const walk = (directory: string): void => {
  let entries: Dirent[];
  try {
    entries = readdirSync(join(repoRoot, directory), { withFileTypes: true });
  } catch {
    return;
  }
  const segments = directory.split("/");
  if (segments.includes("🧬️schema")) {
    for (const entry of entries) {
      const document = entry.isFile() && entry.name.endsWith(".json") ? read(`${directory}/${entry.name}`) : null;
      if (typeof document?.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document);
    }
  }
  const mutations = segments.lastIndexOf("🧬️mutations");
  const descriptor = mutations >= 0 && mutations < segments.length - 1 && segments.at(-1) !== "🧬️schema" && existsSync(join(repoRoot, directory, "🔣️.json")) ? read(`${directory}/🔣️.json`) : null;
  if (typeof descriptor?.aggregateVariant === "string" && typeof descriptor.payloadSchema === "string" && existsSync(join(repoRoot, directory, descriptor.payloadSchema))) {
    const artifacts = segments.indexOf("🗿️artifacts");
    leaves.push({ schema: `${directory}/${descriptor.payloadSchema}`, descriptor: `${directory}/🔣️.json`, plugin: segments[0] === "✏️s" ? bare(segments[2] ?? "") : `framework ${bare(segments.at(artifacts >= 0 ? artifacts - 1 : mutations - 2) ?? "")}`, artifact: artifacts >= 0 ? (segments[artifacts + 1] ?? "") : "", name: segments.slice(mutations + 1).join("/") });
  }
  for (const entry of entries) if (entry.isDirectory() && !SKIPPED.has(entry.name) && !entry.name.startsWith(".")) walk(`${directory}/${entry.name}`);
};
for (const root of ["✏️s", "🧰️framework"]) walk(root);

const catalogue = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json");
const catalogued = new Set(Object.values((catalogue?.scopes ?? {}) as Record<string, { readonly path: string }>).map((scope) => scope.path));
const widgets = mutationInputWidgetVocabulary(repoRoot, (id) => documents.get(id));
const owner = (plugin: string): string => Object.entries(OWNERS).find(([, plugins]) => plugins.includes(plugin))?.[0] ?? "coordinator";
const inputless: (Leaf & { readonly reason: string; readonly read: boolean })[] = [];
const withdrawn: Leaf[] = [];
const unions: Leaf[] = [];
let unreadable = 0;
for (const leaf of leaves.sort((left, right) => left.schema.localeCompare(right.schema))) {
  const schema = read(leaf.schema);
  if (schema === null) {
    unreadable += 1;
    continue;
  }
  if (read(leaf.descriptor)?.editable === false) {
    withdrawn.push(leaf);
    continue;
  }
  const finding = mutationInputDeclarationFindings(mutationInputDeclarations(schema, (id) => documents.get(id)), widgets, null).find((entry) => entry.code === "inputless");
  if (finding === undefined && schema.properties === undefined && (Array.isArray(schema.oneOf) || Array.isArray(schema.anyOf))) unions.push(leaf);
  if (finding !== undefined) inputless.push({ ...leaf, reason: finding.detail.includes("every one is hidden") ? "every input hidden" : "no input", read: catalogued.has(dirname(leaf.schema)) });
}

const lines: string[] = [];
const groups = (rows: readonly Leaf[]): Map<string, Map<string, Leaf[]>> => {
  const found = new Map<string, Map<string, Leaf[]>>();
  for (const row of rows) {
    const plugins = found.get(owner(row.plugin)) ?? new Map<string, Leaf[]>();
    found.set(owner(row.plugin), plugins);
    plugins.set(row.plugin, [...(plugins.get(row.plugin) ?? []), row]);
  }
  return found;
};
const stamp = new Date().toISOString().replace("T", " ").slice(0, 16);
lines.push("# 📭️ S5 gates — editable leaves that show no input (design §22.20)", "");
lines.push(`Generated ${stamp} UTC by \`bun T/🧪️s5-gates-inputless.ts\` (S5-GATES). Rule: the \`inputless\` finding of \`schema mutation-inputs\` — a leaf is EITHER editable and shows at least one input row, OR its descriptor declares it withdraw-only. This list applies the gate's rule to every leaf on disk (${leaves.length} leaves, ${unreadable} unreadable), so it also names leaves the stale central catalogue hides from the gate (column "Gate reads it" = no).`, "");
lines.push(`**${inputless.length} editable leaves show no input** (${inputless.filter((row) => row.read).length} of them are gate findings today, ${inputless.filter((row) => !row.read).length} are hidden by the catalogue); **${withdrawn.length} leaves are declared withdraw-only**.`, "");
lines.push("How to clear one: add `\"editable\": false` as a fifteenth key to the leaf descriptor named below (the derive then answers `input_schema() == None` and refuses `with_input_value`; no Rust attribute needed, and none of `mutation_leaf(payload = …)` / `mutation_leaf(input_schema = …)` may stand beside it) — or give the leaf an input its editor can show. A parameterless leaf (`delete-…`, `remove-…`, `clear-…`) is withdraw-only by nature. Then: `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --under \"✏️s/🔌️plugins/<plugin>\" --json` (repo root; no `inputless` row may remain) and the crate's own `cargo test --lib` round-trip law.", "");
lines.push(`Not on this list although their payload schema has no top-level \`properties\`: ${unions.length} root-union leaves — the editor shows their variant selector and every variant's fields (${unions.map((leaf) => `${leaf.plugin} \`${leaf.name}\``).join(", ") || "none"}). The goal audit's figure of 52 (\`📓️audit-s5-goal-2.md\` gap 8) counted two of them (procedural, forms) and predates the stdio marks.`, "");
lines.push("## Summary", "", "| Owner | Plugin | Editable, no input | of them hidden-only | Gate reads | Declared withdraw-only |", "| --- | --- | ---: | ---: | ---: | ---: |");
const marked = groups(withdrawn);
const open = groups(inputless);
for (const name of [...Object.keys(OWNERS), "coordinator"]) {
  const plugins = new Set([...(open.get(name)?.keys() ?? []), ...(marked.get(name)?.keys() ?? [])]);
  for (const plugin of [...plugins].sort()) {
    const rows = (open.get(name)?.get(plugin) ?? []) as (typeof inputless)[number][];
    lines.push(`| ${name} | ${plugin} | ${rows.length} | ${rows.filter((row) => row.reason !== "no input").length} | ${rows.filter((row) => row.read).length} | ${marked.get(name)?.get(plugin)?.length ?? 0} |`);
  }
}
lines.push("");
for (const name of [...Object.keys(OWNERS), "coordinator"]) {
  const plugins = open.get(name);
  if (plugins === undefined) continue;
  lines.push(`## ${name} — ${[...plugins.values()].reduce((sum, rows) => sum + rows.length, 0)} leaves to mark or to give an input`, "");
  for (const [plugin, rows] of [...plugins].sort(([left], [right]) => left.localeCompare(right))) {
    lines.push(`### ${plugin} (${rows.length})`, "", "| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |", "| --- | --- | --- | --- | --- |");
    for (const row of rows as (typeof inputless)[number][]) lines.push(`| ${row.artifact} | ${row.name} | ${row.reason} | ${row.read ? "yes" : "no"} | \`${row.descriptor}\` |`);
    lines.push("");
  }
}
lines.push(`## Declared withdraw-only (${withdrawn.length})`, "", "| Owner | Plugin | Artifact | Leaf |", "| --- | --- | --- | --- |");
for (const row of withdrawn) lines.push(`| ${owner(row.plugin)} | ${row.plugin} | ${row.artifact} | ${row.name} |`);
lines.push("");
const output = process.argv[2] ?? join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📓️s5-gates-inputless.md");
writeFileSync(output, `${lines.join("\n")}\n`);
console.log(`[s5-gates-inputless] ${leaves.length} leaves, ${inputless.length} editable without input (${inputless.filter((row) => row.read).length} read by the gate), ${withdrawn.length} withdraw-only, ${unreadable} unreadable → ${output}`);
