/** 🏭️ W3-GLTF: writes every glTF mutation TypeScript twin from its committed JSON Schema (schema-first).
 * Leaf twins (`<entity>/<verb>/🟦️.ts`): the flat `Apply` payload, every local `$defs` record and the phase wire, each
 * with its `parse<Export>()`. The aggregate (`🧬️mutations/🟦️.ts`) and the eight subset views follow the aggregate
 * schemas' branches. `--check` fails when a twin on disk differs from what the schemas produce. */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

type Schema = Record<string, any>;

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets";
const ANY = `${ROOT}/♾️any/🧬️schema`;
const MUTATIONS = `${ANY}/🧬️mutations`;
const ID = "https://json.schemas.assets.semio-tech.com/s/stdio/gltf/2.0/any";
const VIEWS = ["🎞️animation", "🪪️asset", "💿️buffer", "🎥️camera", "💎️material", "🕸️mesh", "🎬️scene", "🦴️skin"];
const check = process.argv.includes("--check");
const read = (path: string): Schema => JSON.parse(readFileSync(path, "utf8"));
const stale: string[] = [];
const emit = (path: string, text: string): void => {
  if (existsSync(path) && readFileSync(path, "utf8") === text) return;
  stale.push(relative(ROOT, path));
  if (!check) writeFileSync(path, text);
};

interface Leaf { readonly dir: string; readonly domain: string; readonly verb: string; readonly schema: Schema; readonly kind: string }
const leaves: Leaf[] = readdirSync(MUTATIONS, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .flatMap((domain) => readdirSync(join(MUTATIONS, domain.name), { withFileTypes: true }).filter((entry) => entry.isDirectory() && existsSync(join(MUTATIONS, domain.name, entry.name, "🧬️schema/🔣️.json"))).map((verb) => ({ domain: domain.name, verb: verb.name })))
  .map(({ domain, verb }) => {
    const dir = join(MUTATIONS, domain, verb);
    const schema = read(join(dir, "🧬️schema/🔣️.json"));
    const kind = (schema.$id as string).replace(`${ID}/mutation/`, "").replace("/schema.json", "");
    return { dir, domain, verb, schema, kind };
  })
  .sort((left, right) => left.kind.localeCompare(right.kind));

//#region 🧬️Leaf
interface Shape { readonly type: string; readonly read: string }
const snapshotImports = new Set<string>();
const diffImports = new Set<string>();
const wire = (name: string): string => (snapshotImports.add(name), name);
const diffWire = (name: string): string => (diffImports.add(name), name);

const tagOf = (branches: Schema[]): string | null => {
  const keys = Object.keys(branches[0]?.properties ?? {}).filter((key) => branches.every((branch) => branch.properties?.[key]?.const !== undefined));
  return keys.length === 1 ? keys[0]! : null;
};

const shape = (node: Schema, context: string): Shape => {
  if (node.$ref) {
    const ref = node.$ref as string;
    if (ref.startsWith("#/$defs/")) {
      const name = ref.slice("#/$defs/".length);
      return { type: name, read: `parse${name}` };
    }
    if (ref === `${ID}/snapshot.json`) return { type: wire("GltfSnapshot"), read: wire("parseGltfSnapshot") };
    if (ref === `${ID}/diff.json`) return { type: diffWire("GltfDiff"), read: diffWire("parseGltfDiff") };
    const snapshotDef = ref.match(new RegExp(`^${ID.replaceAll(".", "\\.")}/snapshot\\.json#/\\$defs/(\\w+)$`, "u"));
    if (snapshotDef) return { type: wire(snapshotDef[1]!), read: wire(`parse${snapshotDef[1]}`) };
    throw new Error(`${context}: unsupported $ref ${ref}`);
  }
  if (node.anyOf) {
    const [value, empty] = node.anyOf as Schema[];
    if (node.anyOf.length !== 2 || empty?.type !== "null") throw new Error(`${context}: anyOf is not T | null`);
    const inner = shape(value!, context);
    return { type: `${inner.type} | null`, read: `${wire("gltfWireNullable")}(${inner.read})` };
  }
  if (node.const !== undefined) return { type: JSON.stringify(node.const), read: `${wire("gltfWireLiteral")}(${JSON.stringify(node.const)})` };
  if (node.enum) return { type: node.enum.map((member: unknown) => JSON.stringify(member)).join(" | "), read: `${wire("gltfWireLiteral")}(${node.enum.map((member: unknown) => JSON.stringify(member)).join(", ")})` };
  switch (node.type) {
    case "string":
      return { type: "string", read: wire("gltfWireString") };
    case "boolean":
      return { type: "boolean", read: wire("gltfWireBoolean") };
    case "number":
      return { type: "number", read: wire("gltfWireNumber") };
    case "integer": {
      if (node.minimum !== 0) throw new Error(`${context}: integer without a zero minimum`);
      if (node.maximum === undefined) return { type: "number", read: wire("gltfWireIndex") };
      if (node.maximum === 255) return { type: "number", read: wire("gltfWireByte") };
      return { type: "number", read: `${wire("gltfWireInteger")}(${node.maximum})` };
    }
    case "array": {
      const item = shape(node.items, `${context}[]`);
      const fixed = node.minItems !== undefined && node.minItems === node.maxItems ? (node.minItems as number) : undefined;
      if (fixed !== undefined && fixed <= 4) return { type: `[${Array(fixed).fill(item.type).join(", ")}]`, read: `${wire("gltfWireTuple")}(${Array(fixed).fill(item.read).join(", ")})` };
      if (node.minItems !== undefined && fixed === undefined) throw new Error(`${context}: unsupported array bounds`);
      return { type: `${item.type.includes(" ") ? `(${item.type})` : item.type}[]`, read: fixed === undefined ? `${wire("gltfWireArray")}(${item.read})` : `${wire("gltfWireArray")}(${item.read}, ${fixed})` };
    }
    default:
      throw new Error(`${context}: unsupported schema ${JSON.stringify(node).slice(0, 120)}`);
  }
};

const members = (node: Schema, context: string): { readonly fields: string; readonly reads: string } => {
  if (node.type !== "object" || node.additionalProperties !== false) throw new Error(`${context}: not a closed object`);
  const required = new Set<string>(node.required ?? []);
  const entries = Object.entries((node.properties ?? {}) as Record<string, Schema>).map(([key, member]) => ({ key, optional: !required.has(key), ...shape(member, `${context}.${key}`) }));
  return {
    fields: entries.map((entry) => `  ${entry.key}${entry.optional ? "?" : ""}: ${entry.type};`).join("\n"),
    reads: entries.map((entry) => `${entry.key}: ${wire(entry.optional ? "gltfWireOptional" : "gltfWireRequired")}(${entry.read})`).join(", "),
  };
};

const leafTwin = (leaf: Leaf): string => {
  snapshotImports.clear();
  diffImports.clear();
  const { schema } = leaf;
  const root = schema.title as string;
  const defs = Object.entries((schema.$defs ?? {}) as Record<string, Schema>);
  const phase = defs.find(([, node]) => node.oneOf && tagOf(node.oneOf) === "phase");
  const records = defs.filter(([name]) => name !== phase?.[0]);
  const types: string[] = [];
  const parsers: string[] = [];
  for (const [name, node] of records) {
    if (node.oneOf) {
      const tag = tagOf(node.oneOf);
      if (!tag) throw new Error(`${leaf.kind}: $defs/${name} is not a tagged union`);
      const variants = (node.oneOf as Schema[]).map((branch) => ({ value: branch.properties[tag].const as string, ...members(branch, `${leaf.kind}.${name}`) }));
      types.push(`export type ${name} =\n${variants.map((variant) => `  | { ${variant.fields.split("\n").map((field) => field.trim()).join(" ").replace(/;$/u, "")} }`).join("\n")};`);
      parsers.push(`export const parse${name} = ${wire("gltfWireTagged")}<${name}, ${JSON.stringify(tag)}>(${JSON.stringify(tag)}, {\n${variants.map((variant) => `  ${variant.value}: ${wire("gltfWireObject")}<Extract<${name}, { ${tag}: ${JSON.stringify(variant.value)} }>>({ ${variant.reads} }),`).join("\n")}\n});`);
    } else {
      const { fields, reads } = members(node, `${leaf.kind}.${name}`);
      types.push(`export interface ${name} {\n${fields}\n}`);
      parsers.push(`export const parse${name} = ${wire("gltfWireObject")}<${name}>({ ${reads} });`);
    }
  }
  const { fields, reads } = members(schema, leaf.kind);
  types.push(fields ? `export interface ${root} {\n${fields}\n}` : `export type ${root} = Record<string, never>;`);
  parsers.push(`export const parse${root} = ${wire("gltfWireObject")}<${root}>({${reads ? ` ${reads} ` : ""}});`);
  if (phase) {
    const [name, node] = phase;
    const restore = (node.oneOf as Schema[]).find((branch) => branch.properties.phase.const === "restore")!.properties.value;
    const apply = (node.oneOf as Schema[]).find((branch) => branch.properties.phase.const === "apply")!.properties.value;
    if (apply.$ref !== "#") throw new Error(`${leaf.kind}: the apply phase is not the document root`);
    const restored = shape(restore, `${leaf.kind}.${name}.restore`);
    types.push(`export type ${name} = ${diffWire("GltfPhase")}<${root}, ${restored.type}>;`);
    parsers.push(`export const parse${name} = ${diffWire("gltfWirePhase")}(parse${root}, ${restored.read});`);
  }
  const snapshotNames = [...snapshotImports].sort((left, right) => left.localeCompare(right));
  const diffNames = [...diffImports].sort((left, right) => left.localeCompare(right));
  const typeOnly = (name: string): boolean => /^[A-Z]/u.test(name);
  const importLine = (names: string[], from: string): string | null => (names.length === 0 ? null : `import { ${names.map((name) => (typeOnly(name) ? `type ${name}` : name)).join(", ")} } from "${from}";`);
  const emoji = leaf.verb.match(/^\p{Extended_Pictographic}\uFE0F?/u)?.[0] ?? "🦠️";
  const phaseName = phase?.[0];
  return [
    `/** ${emoji} \`${leaf.kind}\` wire twin: the flat \`Apply\` payload \`${root}\`${phaseName ? ` and the phase wire \`${phaseName}\`` : ""}, exactly as \`./🦀️.rs\` writes them.`,
    " * @see ./🧬️schema/🔣️.json */",
    importLine(snapshotNames, "../../../📸️snapshot/🟦️.ts"),
    importLine(diffNames, "../../../🔺️diff/🟦️.ts"),
    "",
    ...types.flatMap((type) => [type, ""]),
    ...parsers,
    "",
  ]
    .filter((line): line is string => line !== null)
    .join("\n");
};
//#endregion 🧬️Leaf

//#region 🧺️Aggregate
interface Branch { readonly mutation: string; readonly leaf: Leaf; readonly payload: string }
const branchesOf = (schema: Schema): Branch[] =>
  (schema.oneOf as Schema[]).map((branch) => {
    const mutation = branch.properties.mutation.const as string;
    const [id, pointer] = (branch.properties.payload.$ref as string).split("#");
    const leaf = leaves.find((candidate) => candidate.schema.$id === id);
    if (!leaf) throw new Error(`branch ${mutation}: no leaf owns ${id}`);
    const payload = pointer ? pointer.replace("/$defs/", "") : (leaf.schema.title as string);
    return { mutation, leaf, payload };
  });

const aggregateTwin = (): string => {
  const branches = branchesOf(read(join(MUTATIONS, "🔣️.json")));
  const imports = branches.map((branch) => `import { parse${branch.payload}, type ${branch.payload} } from "./${relative(MUTATIONS, branch.leaf.dir)}/🟦️.ts";`);
  return [
    "/** 🧬️ `GltfMutation` twin: the adjacently tagged (`mutation`/`payload`) aggregate over every glTF 2.0 leaf, branch for branch as",
    " * `./🔣️.json` and `./🦀️.rs` spell it; a wrapped leaf's payload is its whole phase wire, the set-snapshot leaf's its plain record.",
    " * @see ./🔣️.json */",
    'import { gltfWireLiteral, gltfWireObject, gltfWireRequired, type GltfWireReader } from "../📸️snapshot/🟦️.ts";',
    ...imports,
    "",
    "export type GltfMutation =",
    ...branches.map((branch, index) => `  | { readonly mutation: ${JSON.stringify(branch.mutation)}; readonly payload: ${branch.payload} }${index === branches.length - 1 ? ";" : ""}`),
    "",
    "const payloads: { readonly [K in GltfMutation[\"mutation\"]]: GltfWireReader<Extract<GltfMutation, { readonly mutation: K }>[\"payload\"]> } = {",
    ...branches.map((branch) => `  ${branch.mutation}: parse${branch.payload},`),
    "};",
    "const mutations = Object.keys(payloads) as GltfMutation[\"mutation\"][];",
    "",
    "/** 📥️ Reads one aggregate wire: the tag names the leaf, whose own reader decodes the payload. */",
    "export const parseGltfMutation: GltfWireReader<GltfMutation> = (value, at = \"$\") => {",
    "  const row = gltfWireObject<{ mutation: GltfMutation[\"mutation\"]; payload: unknown }>({ mutation: gltfWireRequired(gltfWireLiteral(...mutations)), payload: gltfWireRequired((payload) => payload) })(value, at);",
    "  return { mutation: row.mutation, payload: payloads[row.mutation](row.payload, `${at}.payload`) } as GltfMutation;",
    "};",
    "",
  ].join("\n");
};

const viewTwin = (view: string): { readonly schema: Schema; readonly twin: string } => {
  const path = join(ROOT, view, "🧬️schema/🧬️mutations/🔣️.json");
  const schema = read(path);
  const tags: string[] = [];
  for (const branch of schema.oneOf as Schema[]) {
    const mutation = branch.properties.mutation.const as string;
    const [target] = branchesOf({ oneOf: [branch] });
    const wrapped = Object.keys(target!.leaf.schema.$defs ?? {}).find((name) => target!.leaf.schema.$defs[name].oneOf && tagOf(target!.leaf.schema.$defs[name].oneOf) === "phase");
    branch.properties.payload.$ref = wrapped ? `${target!.leaf.schema.$id}#/$defs/${wrapped}` : target!.leaf.schema.$id;
    tags.push(mutation);
  }
  const noun = view.replace(/^\p{Extended_Pictographic}\uFE0F?/u, "");
  const title = `Gltf${noun[0]!.toUpperCase()}${noun.slice(1)}Mutation`;
  const emoji = view.match(/^\p{Extended_Pictographic}\uFE0F?/u)![0];
  const twin = [
    `/** ${emoji} \`${title}\` twin: the ${noun} slice of the glTF 2.0 mutation vocabulary, a view over the any subset's \`GltfMutation\``,
    " * that selects branches and never restates a payload.",
    " * @see ./🔣️.json */",
    'import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";',
    'import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";',
    "",
    `export type ${title} = Extract<GltfMutation, { readonly mutation: ${tags.map((tag) => JSON.stringify(tag)).join(" | ")} }>;`,
    "",
    `const members: readonly GltfMutation["mutation"][] = [${tags.map((tag) => JSON.stringify(tag)).join(", ")}];`,
    "",
    `export const parse${title}: GltfWireReader<${title}> = (value, at = "$") => {`,
    "  const mutation = parseGltfMutation(value, at);",
    `  return members.includes(mutation.mutation) ? (mutation as ${title}) : gltfWireRefuse(\`\${at}.mutation\`, \`value is not one of \${members.join(", ")}\`);`,
    "};",
    "",
  ].join("\n");
  return { schema, twin };
};
//#endregion 🧺️Aggregate

for (const leaf of leaves) emit(join(leaf.dir, "🟦️.ts"), leafTwin(leaf));
emit(join(MUTATIONS, "🟦️.ts"), aggregateTwin());
for (const view of VIEWS) {
  const { schema, twin } = viewTwin(view);
  emit(join(ROOT, view, "🧬️schema/🧬️mutations/🔣️.json"), `${JSON.stringify(schema, null, 2)}\n`);
  emit(join(ROOT, view, "🧬️schema/🧬️mutations/🟦️.ts"), twin);
}
console.log(`[w3-gltf-twins] leaves=${leaves.length} views=${VIEWS.length} ${check ? "stale" : "written"}=${stale.length}`);
for (const path of stale) console.log(`  ${path}`);
if (check && stale.length > 0) process.exit(1);
