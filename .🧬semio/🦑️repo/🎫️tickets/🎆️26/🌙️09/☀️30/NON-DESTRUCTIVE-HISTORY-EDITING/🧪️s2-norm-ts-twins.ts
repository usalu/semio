/** 🏭️ S2-NORM WP-6: writes every norm schema TypeScript twin from its committed JSON Schema (schema-first), on the pattern
 * of `🧪️w3-gltf-twins.ts`. Per artifact subset: the snapshot, diff and artifact twins (root and every `$defs` record, each
 * with its `parse<Export>()`), one wire twin per mutation leaf, and the aggregate `<A>Mutation` twin, externally or
 * internally tagged exactly as its schema spells it. Readers come from the norm wire contract
 * `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🟦️.ts`. A `$defs` record its schema keeps out of TypeScript
 * (`x-semio-formats` without `🟦️typescript`) is emitted under the artifact-prefixed name (`Din4108ThermalZone`), so the
 * twin never declares an export the contract withholds. `--check` fails when a twin on disk differs from what the schemas
 * produce, or when a peer module's import from a twin is no longer exported.
 *
 *   bun ./🧪️s2-norm-ts-twins.ts [--check] [--only <artifact-dir>]
 */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

type Schema = Record<string, any>;

const REPO = "/Users/ueli/Documents/semio";
const NORM = `${REPO}/✏️s/🔌️plugins/📕️norm`;
const ARTIFACTS = `${NORM}/🗿️artifacts`;
const CONTRACT = `${NORM}/📇️registry/🧬️contract/🟦️.ts`;
const CATALOG = `${REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`;
const TS = "🟦️typescript";
const check = process.argv.includes("--check");
const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : undefined;
const read = (path: string): Schema => JSON.parse(readFileSync(path, "utf8"));
const stale: string[] = [];
const broken: string[] = [];
const emit = (path: string, text: string): void => {
  if (existsSync(path) && readFileSync(path, "utf8") === text) return;
  stale.push(relative(REPO, path));
  if (!check) writeFileSync(path, text);
};
const specifier = (from: string, to: string): string => {
  const path = relative(dirname(from), to);
  return path.startsWith(".") ? path : `./${path}`;
};
const pascal = (text: string): string => text.replace(/(^|-)([a-z0-9])/gu, (_, __, letter: string) => letter.toUpperCase());

//#region 🗂️Modules
type Facet = "snapshot" | "diff" | "artifact" | "inference" | "text" | "aggregate" | "leaf" | "report";

/** 📛️ TypeScript names a twin already publishes to its peers (the `🪶️sqlite` companions import them): a withheld `$defs`
 * record kept under this name instead of the prefixed default, and an alias for a schema position that has no record of its
 * own, both resolved from the schema, never hand-typed. */
const PUBLISHED: Readonly<Record<string, { readonly names?: Readonly<Record<string, string>>; readonly aliases?: Partial<Record<Facet, Readonly<Record<string, string>>>> }>> = {
  "🌬️din16798": { names: { ZoneDocument: "Din16798Zone", VentSystemDocument: "Din16798VentSystem" } },
  "🏭️vdi3805": { names: { VdiUnit: "VdiUnit" } },
  "🌍️en1997": { names: { SpreadFoundation: "SpreadFoundation", SoilLayer: "SoilLayer", AnnexChoice: "AnnexChoice", Pile: "Pile", FoundationLoadCase: "FoundationLoadCase", PileTestProfile: "PileTestProfile", RetainingWall: "RetainingWall", Slope: "Slope", UpliftCase: "UpliftCase" } },
  "🏛️en1992": { aliases: { snapshot: { DuctilityClass: "/$defs/ReinforcementGrade/properties/ductility" } } },
};
interface Module {
  readonly facet: Facet;
  readonly json: string;
  readonly twin: string;
  readonly schema: Schema;
  readonly root: string;
  readonly names: ReadonlyMap<string, string>;
  readonly emoji: string;
  readonly kind?: string;
  readonly aliases: Readonly<Record<string, string>>;
}
const modules = new Map<string, Module>();
const defsOf = (schema: Schema): Readonly<Record<string, Schema>> => ({ ...(schema.definitions ?? {}), ...(schema.$defs ?? {}) });
const withheld = (node: Schema): boolean => Array.isArray(node["x-semio-formats"]) && !node["x-semio-formats"].includes(TS);
const register = (artifact: string, facet: Facet, json: string, prefix: string, fallback: string, emoji: string, kind?: string): Module => {
  const schema = read(json);
  const published = PUBLISHED[artifact];
  const names = new Map(Object.entries(defsOf(schema)).map(([key, node]) => [key, published?.names?.[key] ?? (withheld(node) ? `${prefix}${key}` : key)]));
  const module: Module = { facet, json, twin: join(dirname(json), "🟦️.ts"), schema, root: (schema.title as string | undefined) ?? fallback, names, emoji, kind, aliases: published?.aliases?.[facet] ?? {} };
  if (!modules.has(schema.$id as string)) modules.set(schema.$id as string, module);
  return module;
};

interface Subset { readonly artifact: string; readonly dir: string; readonly prefix: string; readonly facets: readonly Module[]; readonly leaves: readonly Module[]; readonly aggregate: Module }
const subsets: Subset[] = readdirSync(ARTIFACTS, { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && existsSync(join(ARTIFACTS, entry.name, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json")))
  .map((entry) => {
    const dir = join(ARTIFACTS, entry.name, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    const prefix = (read(join(dir, "🧬️mutations/🔣️.json")).title as string).replace(/Mutation$/u, "");
    const facets = [
      register(entry.name, "snapshot", join(dir, "📸️snapshot/🔣️.json"), prefix, `${prefix}Snapshot`, "📸️"),
      register(entry.name, "diff", join(dir, "🔺️diff/🔣️.json"), prefix, `${prefix}Diff`, "🔺️"),
      register(entry.name, "artifact", join(dir, "🔣️.json"), prefix, `${prefix}Artifact`, "🧬️"),
      register(entry.name, "inference", join(dir, "💡️inferences/🔣️.json"), prefix, `${prefix}Inference`, "💡️"),
      ...[["📸️snapshot", "Snapshot"], ["🔺️diff", "Diff"], ["🧬️mutations", "Mutations"], ["💡️inferences", "Inference"]].map(([facet, noun]) => register(entry.name, "text", join(dir, facet!, "📝️text/🔣️.json"), prefix, `${prefix}${noun}Text`, "📝️")),
    ];
    const leaves = readdirSync(join(dir, "🧬️mutations"), { withFileTypes: true })
      .filter((leaf) => leaf.isDirectory() && existsSync(join(dir, "🧬️mutations", leaf.name, "🧬️schema/🔣️.json")) && existsSync(join(dir, "🧬️mutations", leaf.name, "🔣️.json")))
      .map((leaf) => {
        const descriptor = read(join(dir, "🧬️mutations", leaf.name, "🔣️.json"));
        const emoji = leaf.name.match(/^\p{Extended_Pictographic}️?/u)?.[0] ?? "🦠️";
        return register(entry.name, "leaf", join(dir, "🧬️mutations", leaf.name, "🧬️schema/🔣️.json"), prefix, pascal(descriptor.semanticKind as string), emoji, descriptor.semanticKind as string);
      });
    const aggregate = register(entry.name, "aggregate", join(dir, "🧬️mutations/🔣️.json"), prefix, `${prefix}Mutation`, "🧺️");
    return { artifact: entry.name, dir, prefix, facets, leaves, aggregate };
  })
  .filter((subset) => only === undefined || subset.artifact === only);
/** 📊️ The plugin-level modules every artifact shares, twinned like a subset facet. */
const shared: readonly Module[] = only === undefined ? [register("⚖️compliance", "report", `${NORM}/⚖️compliance/🧬️schema/🔣️.json`, "", "CheckReport", "⚖️")] : [];

interface Foreign { readonly twin: string; readonly root: string }
const foreign = new Map<string, Foreign>();
const scopes = Object.values(read(CATALOG).scopes as Record<string, { path: string; formats: Record<string, string> }>);
const foreignModule = (id: string): Foreign => {
  if (foreign.has(id)) return foreign.get(id)!;
  for (const scope of scopes) {
    const json = join(REPO, scope.path, scope.formats["🔣️jsonschema"] ?? "");
    if (!scope.formats[TS] || scope.path.startsWith("✏️s/🔌️plugins/📕️norm") || !existsSync(json) || !json.endsWith(".json")) continue;
    const schema = read(json);
    if (schema.$id !== id) continue;
    const found = { twin: join(REPO, scope.path, scope.formats[TS]!), root: schema.title as string };
    foreign.set(id, found);
    return found;
  }
  throw new Error(`no TypeScript twin owns ${id}`);
};
//#endregion 🗂️Modules

//#region 🧬️Shapes
interface Shape { readonly type: string; readonly read: string }
class Uses {
  readonly contract = new Set<string>(["type NormWireReader"]);
  readonly imports = new Map<string, Set<string>>();
  wire(name: string): string {
    this.contract.add(name);
    return name;
  }
  bring(twin: string, name: string): void {
    const names = this.imports.get(twin) ?? new Set<string>();
    names.add(`type ${name}`).add(`parse${name}`);
    this.imports.set(twin, names);
  }
}
const group = (type: string): string => (/[|&]/u.test(type) ? `(${type})` : type);
const literal = (values: readonly unknown[], uses: Uses): Shape => ({ type: values.map((value) => JSON.stringify(value)).join(" | "), read: `${uses.wire("normWireLiteral")}(${values.map((value) => JSON.stringify(value)).join(", ")})` });

const reference = (ref: string, module: Module, uses: Uses, self: Module): Shape => {
  const [id, pointer] = ref.split("#") as [string, string | undefined];
  const key = pointer?.replace(/^\/(?:\$defs|definitions)\//u, "");
  if (id === "") {
    const name = module.names.get(key!);
    if (!name) throw new Error(`${relative(REPO, module.json)}: unresolved local ${ref}`);
    return { type: name, read: `${uses.wire("normWireRef")}(() => parse${name})` };
  }
  const target = modules.get(id);
  if (target) {
    const name = key ? target.names.get(key) : target.root;
    if (!name) throw new Error(`${relative(REPO, module.json)}: unresolved ${ref}`);
    if (target.twin !== self.twin) uses.bring(target.twin, name);
    return { type: name, read: target.twin === self.twin ? `${uses.wire("normWireRef")}(() => parse${name})` : `parse${name}` };
  }
  if (key) throw new Error(`${relative(REPO, module.json)}: foreign pointer ${ref}`);
  const outside = foreignModule(id);
  uses.bring(outside.twin, outside.root);
  return { type: outside.root, read: `parse${outside.root}` };
};

const resolveLocal = (node: Schema, module: Module): Schema => {
  if (typeof node.$ref !== "string") return node;
  const [id, pointer] = node.$ref.split("#");
  const owner = id === "" ? module : modules.get(id);
  const key = pointer?.replace(/^\/(?:\$defs|definitions)\//u, "");
  return owner ? (key ? defsOf(owner.schema)[key]! : owner.schema) : node;
};
const tagOf = (branches: readonly Schema[], module: Module): string | null => {
  const resolved = branches.map((branch) => resolveLocal(branch, module));
  const keys = Object.keys(resolved[0]?.properties ?? {}).filter((key) => resolved.every((branch) => branch.properties?.[key]?.const !== undefined));
  return keys.length >= 1 ? keys[0]! : null;
};

const shape = (node: Schema, module: Module, uses: Uses, context: string, named?: string): Shape => {
  if (typeof node.$ref === "string") return reference(node.$ref, module, uses, module);
  const branches = (node.oneOf ?? node.anyOf) as Schema[] | undefined;
  if (branches) {
    const nulls = branches.filter((branch) => branch.type === "null");
    const rest = branches.filter((branch) => branch.type !== "null");
    if (nulls.length === 1 && rest.length === 1) {
      const inner = shape(rest[0]!, module, uses, context);
      return { type: `${group(inner.type)} | null`, read: `${uses.wire("normWireNullable")}(${inner.read})` };
    }
    const parts = branches.map((branch, index) => shape(branch, module, uses, `${context}|${index}`));
    const union = named ?? parts.map((part) => group(part.type)).join(" | ");
    const tag = nulls.length === 0 ? tagOf(branches, module) : null;
    if (tag) {
      const values = branches.map((branch) => resolveLocal(branch, module).properties[tag].const as string);
      return { type: parts.map((part) => group(part.type)).join(" | "), read: `${uses.wire("normWireTagged")}<${group(union)}, ${JSON.stringify(tag)}>(${JSON.stringify(tag)}, {\n${values.map((value, index) => `  ${JSON.stringify(value)}: ${parts[index]!.read},`).join("\n")}\n})` };
    }
    return { type: parts.map((part) => group(part.type)).join(" | "), read: `${uses.wire("normWireAny")}<[${parts.map((part) => part.type).join(", ")}]>(${parts.map((part) => part.read).join(", ")})` };
  }
  if (node.const !== undefined) return literal([node.const], uses);
  if (Array.isArray(node.enum)) return literal(node.enum, uses);
  const types: string[] = Array.isArray(node.type) ? node.type : node.type !== undefined ? [node.type] : node.properties ? ["object"] : [];
  if (types.length === 2 && types.includes("null")) {
    const inner = shape({ ...node, type: types.find((type) => type !== "null") }, module, uses, context);
    return { type: `${group(inner.type)} | null`, read: `${uses.wire("normWireNullable")}(${inner.read})` };
  }
  if (types.length !== 1) return { type: uses.wire("type NormJson").replace("type ", ""), read: uses.wire("normWireJson") };
  switch (types[0]) {
    case "null":
      return literal([null], uses);
    case "string":
      return { type: "string", read: uses.wire("normWireString") };
    case "boolean":
      return { type: "boolean", read: uses.wire("normWireBoolean") };
    case "number":
    case "integer": {
      const reader = uses.wire(types[0] === "number" ? "normWireNumber" : "normWireInteger");
      const bounds = Object.fromEntries(["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"].filter((key) => typeof node[key] === "number").map((key) => [key, node[key]]));
      return { type: "number", read: Object.keys(bounds).length ? `${uses.wire("normWireRange")}(${reader}, ${JSON.stringify(bounds)})` : reader };
    }
    case "array": {
      const item = node.items ? shape(node.items, module, uses, `${context}[]`) : { type: uses.wire("type NormJson").replace("type ", ""), read: uses.wire("normWireJson") };
      const bounds = node.minItems !== undefined || node.maxItems !== undefined ? `, ${node.minItems ?? 0}, ${node.maxItems ?? "Number.POSITIVE_INFINITY"}` : "";
      return { type: `${group(item.type)}[]`, read: `${uses.wire("normWireArray")}(${item.read}${bounds})` };
    }
    case "object": {
      if (!node.properties) {
        const value = node.additionalProperties && typeof node.additionalProperties === "object" ? shape(node.additionalProperties, module, uses, `${context}{}`) : { type: uses.wire("type NormJson").replace("type ", ""), read: uses.wire("normWireJson") };
        return { type: `{ [key: string]: ${value.type} }`, read: `${uses.wire("normWireMap")}(${value.read})` };
      }
      const object = members(node, module, uses, context);
      return { type: `{ ${object.fields.map((field) => field.text).join(" ")} }`, read: `${uses.wire("normWireObject")}<${named ?? `{ ${object.fields.map((field) => field.text).join(" ")} }`}>({ ${object.reads} }${node.additionalProperties === false ? "" : ", false"})` };
    }
    default:
      throw new Error(`${context}: unsupported type ${types[0]}`);
  }
};

interface Field { readonly text: string; readonly line: string }
const members = (node: Schema, module: Module, uses: Uses, context: string): { readonly fields: readonly Field[]; readonly reads: string } => {
  const required = new Set<string>(node.required ?? []);
  const entries = Object.entries((node.properties ?? {}) as Record<string, Schema>).map(([key, member]) => {
    const inner = shape(member, module, uses, `${context}.${key}`);
    const defaulted = !required.has(key) && / \| null$/u.test(inner.type);
    const optional = !required.has(key) && !defaulted;
    const property = /^[A-Za-z_$][A-Za-z0-9_$]*$/u.test(key) ? key : JSON.stringify(key);
    const state = typeof member["x-semio-state"] === "string" ? `  /** @state ${member["x-semio-state"]} */\n` : "";
    return { key: property, optional, defaulted, inner, state };
  });
  return {
    fields: entries.map((entry) => ({ text: `${entry.key}${entry.optional ? "?" : ""}: ${entry.inner.type};`, line: `${entry.state}  ${entry.key}${entry.optional ? "?" : ""}: ${entry.inner.type};` })),
    reads: entries.map((entry) => `${entry.key}: ${entry.defaulted ? `${uses.wire("normWireDefault")}(${entry.inner.read}, () => null)` : `${uses.wire(entry.optional ? "normWireOptional" : "normWireRequired")}(${entry.inner.read})`}`).join(", "),
  };
};

const declare = (name: string, node: Schema, module: Module, uses: Uses): { readonly type: string; readonly parser: string } => {
  if (node.properties && (node.type === "object" || node.type === undefined)) {
    const object = members(node, module, uses, name);
    return {
      type: object.fields.length ? `export interface ${name} {\n${object.fields.map((field) => field.line).join("\n")}\n}` : `export type ${name} = Record<string, never>;`,
      parser: `export const parse${name}: NormWireReader<${name}> = ${uses.wire("normWireObject")}<${name}>({ ${object.reads} }${node.additionalProperties === false ? "" : ", false"});`,
    };
  }
  const inner = shape(node, module, uses, name, name);
  return { type: `export type ${name} = ${inner.type};`, parser: `export const parse${name}: NormWireReader<${name}> = ${inner.read};` };
};
//#endregion 🧬️Shapes

//#region 📝️Twins
const DESCRIPTIONS: Readonly<Record<Facet, (module: Module) => string>> = {
  snapshot: (module) => `\`${module.root}\` wire twin: the persisted snapshot and every record it holds`,
  diff: (module) => `\`${module.root}\` wire twin: the sparse field delta a mutation raises`,
  artifact: (module) => `\`${module.root}\` wire twin: the artifact document across its state lanes`,
  inference: (module) => `\`${module.root}\` wire twin: the derived reading of the document`,
  text: (module) => `\`${module.root}\` wire twin: the serialized text form this facet's grammar and codec speak`,
  report: (module) => `\`${module.root}\` wire twin: the compliance report every norm artifact's checks produce`,
  aggregate: (module) => `\`${module.root}\` wire twin: the mutation aggregate, branch for branch as \`./🔣️.json\` spells it`,
  leaf: (module) => `\`${module.kind}\` wire twin: the leaf payload \`${module.root}\``,
};

const twin = (module: Module, body: { readonly types: readonly string[]; readonly parsers: readonly string[] }, uses: Uses, tail: readonly string[] = []): string => {
  const contract = [...uses.contract].sort((left, right) => left.replace("type ", "").localeCompare(right.replace("type ", "")));
  const imports = [...uses.imports.entries()]
    .map(([path, names]) => ({ from: specifier(module.twin, path), names: [...names].sort((left, right) => left.replace("type ", "").localeCompare(right.replace("type ", ""))) }))
    .sort((left, right) => left.from.localeCompare(right.from));
  return [
    `/** ${module.emoji} ${DESCRIPTIONS[module.facet](module)}, exactly as \`./🦀️.rs\` writes it. Generated from \`./🔣️.json\``,
    " * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.",
    " * @see ./🔣️.json */",
    `import { ${contract.join(", ")} } from "${specifier(module.twin, CONTRACT)}";`,
    ...imports.map((entry) => `import { ${entry.names.join(", ")} } from "${entry.from}";`),
    "",
    ...body.types.flatMap((type) => [type, ""]),
    ...body.parsers,
    ...(tail.length ? ["", ...tail] : []),
    "",
  ].join("\n");
};

const records = (module: Module): string => {
  const uses = new Uses();
  const at = (pointer: string): Schema => pointer.split("/").slice(1).reduce<Schema>((node, key) => node?.[key.replaceAll("~1", "/").replaceAll("~0", "~")], module.schema) ?? (() => { throw new Error(`${relative(REPO, module.json)}: no position ${pointer}`); })();
  const declarations = [
    declare(module.root, module.schema, module, uses),
    ...Object.entries(defsOf(module.schema)).map(([key, node]) => declare(module.names.get(key)!, node, module, uses)),
    ...Object.entries(module.aliases).map(([name, pointer]) => declare(name, at(pointer), module, uses)),
  ];
  const sqlite = join(dirname(module.twin), "🪶️sqlite/🟦️.ts");
  const tail = module.facet === "snapshot" && existsSync(sqlite) ? ['export * from "./🪶️sqlite/🟦️.ts";'] : [];
  return twin(module, { types: declarations.map((entry) => entry.type), parsers: declarations.map((entry) => entry.parser) }, uses, tail);
};

const aggregateTwin = (subset: Subset): string => {
  const module = subset.aggregate;
  const uses = new Uses();
  const branches = module.schema.oneOf as Schema[];
  const external = branches.every((branch) => branch.properties && Object.keys(branch.properties).length === 1 && branch.properties[Object.keys(branch.properties)[0]!].$ref);
  if (external) {
    const rows = branches.map((branch) => {
      const [variant] = Object.keys(branch.properties) as [string];
      const payload = reference(branch.properties[variant].$ref, module, uses, module);
      return { variant, payload };
    });
    const type = `export type ${module.root} =\n${rows.map((row) => `  | { ${row.variant}: ${row.payload.type} }`).join("\n")};`;
    const parser = `export const parse${module.root}: NormWireReader<${module.root}> = ${uses.wire("normWireExternal")}<${module.root}>({\n${rows.map((row) => `  ${row.variant}: ${row.payload.read},`).join("\n")}\n});`;
    return twin(module, { types: [type], parsers: [parser] }, uses);
  }
  const rows = branches.map((branch) => {
    const leaf = modules.get((branch.$ref as string).split("#")[0]!)!;
    const tag = leaf.schema.properties?.mutation?.const as string | undefined;
    if (!tag) throw new Error(`${relative(REPO, leaf.json)}: an internally tagged leaf without a \`mutation\` const`);
    return { tag, payload: reference(branch.$ref, module, uses, module) };
  });
  const type = `export type ${module.root} =\n${rows.map((row) => `  | ${row.payload.type}`).join("\n")};`;
  const parser = `export const parse${module.root}: NormWireReader<${module.root}> = ${uses.wire("normWireTagged")}<${module.root}, "mutation">("mutation", {\n${rows.map((row) => `  ${row.tag}: ${row.payload.read},`).join("\n")}\n});`;
  return twin(module, { types: [type], parsers: [parser] }, uses);
};
//#endregion 📝️Twins

//#region 🛂️Consumers
/** 🛂️ Every name a peer module beside the twins imports from a regenerated twin must still be exported by it. */
const consumers = (subset: Subset, written: ReadonlyMap<string, string>): void => {
  const sqlite = join(subset.dir, "📸️snapshot/🪶️sqlite/🟦️.ts");
  for (const path of [sqlite]) {
    if (!existsSync(path)) continue;
    const source = readFileSync(path, "utf8");
    for (const match of source.matchAll(/import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*"([^"]+)"/gu)) {
      const target = join(dirname(path), match[2]!);
      const text = written.get(target);
      if (text === undefined) continue;
      for (const raw of match[1]!.split(",")) {
        const name = raw.trim().replace(/^type\s+/u, "").split(/\s+as\s+/u)[0]!;
        if (name && !new RegExp(`^export\\s+(?:interface|type|const|function|class)\\s+${name}\\b`, "mu").test(text)) broken.push(`${relative(REPO, path)} imports ${name} from ${match[2]}, which the regenerated twin no longer exports`);
      }
    }
  }
};
//#endregion 🛂️Consumers

/** 🪢️ A split-layout leaf (`<leaf>/🦠️mutation/🦀️.rs`) keeps its TypeScript payload twin beside the Rust one: that file is
 * the generated leaf wire twin, re-exported, so the payload has one schema-derived definition. */
const splitTwin = (module: Module): string => {
  const path = join(dirname(dirname(module.json)), "🦠️mutation/🟦️.ts");
  return [
    `/** ${module.emoji} \`${module.kind}\` payload twin of the split leaf layout: the generated wire twin, re-exported.`,
    " * @see ../🧬️schema/🔣️.json */",
    `export { parse${module.root}, type ${module.root} } from "${specifier(path, module.twin)}";`,
    "",
  ].join("\n");
};

for (const subset of subsets) {
  const written = new Map<string, string>();
  for (const module of [...subset.facets, ...subset.leaves]) written.set(module.twin, records(module));
  for (const module of subset.leaves) if (existsSync(join(dirname(dirname(module.json)), "🦠️mutation/🦀️.rs"))) written.set(join(dirname(dirname(module.json)), "🦠️mutation/🟦️.ts"), splitTwin(module));
  written.set(subset.aggregate.twin, aggregateTwin(subset));
  consumers(subset, written);
  for (const [path, text] of written) emit(path, text);
}
for (const module of shared) emit(module.twin, records(module));
console.log(`[s2-norm-ts-twins] subsets=${subsets.length} twins=${shared.length + subsets.reduce((count, subset) => count + subset.facets.length + subset.leaves.length + 1, 0)} ${check ? "stale" : "written"}=${stale.length} broken=${broken.length}`);
for (const path of stale.slice(0, 40)) console.log(`  ${path}`);
for (const line of broken) console.log(`  BROKEN ${line}`);
if ((check && stale.length > 0) || broken.length > 0) process.exit(1);
