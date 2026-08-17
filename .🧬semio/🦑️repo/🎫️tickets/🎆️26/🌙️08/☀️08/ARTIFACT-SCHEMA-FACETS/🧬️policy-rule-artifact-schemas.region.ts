//#region 🔧️PolicyRuleArtifactSchemas
/**
 * 🧬️Wave W2 artifact-schema facet scanners (ARTIFACT-SCHEMA-FACETS).
 * Three facets × five `schemaFormats` leaves must agree on canonical camelCase fields; extractors are the
 * compiler this design deliberately does not have. Nested `📸️snapshot` / `🔺️diff` children are recognized
 * via taxonomy `snapshotChildDirs` / `diffChildDirs` in `policyTaxonomyDirsBreaches`.
 */

/** 🪪Canonical field shape shared by every schemaFormats extractor. */
export type PolicySchemaFieldCardinality = "scalar" | "list" | "fixedList" | "map";

/** 🧬️One normalised field from a schema leaf. */
export type PolicySchemaFieldShape = {
  name: string;
  optional: boolean;
  cardinality: PolicySchemaFieldCardinality;
  scalar: string;
  state: string;
};

/** 📦Extracted top-level type + fields from one leaf. */
export type PolicySchemaLeafExtract = {
  typeName: string;
  fields: PolicySchemaFieldShape[];
};

/** 🧭️§2 facet paths relative to an artifact root. */
const POLICY_SCHEMA_FACET_RELS = ["🧬️schema", "📸️snapshot/🧬️schema", "🔺️diff/🧬️schema"] as const;

/** 🏷️§10 prefix table keyed by `policyStripEmoji(plugin)/policyStripEmoji(artifact)`. */
const POLICY_ARTIFACT_SCHEMA_PREFIXES: Readonly<Record<string, string>> = {
  "writer/writer": "Writer",
  "mathematical/mathematical": "Mathematical",
  "procedural/procedural2d": "Procedural2d",
  "procedural/procedural3d": "Procedural3d",
  "flow/flow": "Flow",
  "gis/gisterrain": "GisTerrain",
  "gis/gismap": "GisMap",
  "vcs/vcs": "Vcs",
  "animate/present": "Present",
  "shooting/shooting": "Shooting",
  "demonstrator/playground": "Playground",
  "sequence/sequence": "Sequence",
  "fem/2d": "Fem2d",
  "fem/3d": "Fem3d",
  "architect/program": "Program",
  "process/process3d": "Process3d",
  "lowpoly/lowpoly": "Lowpoly",
  "reasoning/wires": "Wires",
  "forms/forms": "Forms",
  "layout/layout": "Layout",
  "cad/cad": "Cad",
  "norm/iso16757": "Iso16757",
  "norm/vdi3805": "Vdi3805",
  "norm/din4108": "Din4108",
  "norm/din16798": "Din16798",
  "norm/en1990": "En1990",
  "norm/en1991": "En1991",
  "norm/en1992": "En1992",
  "norm/en1993": "En1993",
  "norm/en1994": "En1994",
  "norm/en1995": "En1995",
  "norm/en1996": "En1996",
  "norm/en1997": "En1997",
  "norm/en1998": "En1998",
  "norm/en1999": "En1999",
  "norm/din18599": "Din18599",
  "playbook/playbook": "Playbook",
  "imperative/imperative": "Imperative",
  "remodel/remodel": "Remodel",
  "energy/model": "EnergyModel",
  "trinity/rewrite": "Rewrite",
  "trinity/jack": "Jack",
  "dag/dag": "Dag",
  "draw/draw": "Draw",
  "raster/raster": "Raster",
  "note/note": "Note",
  "puzzle/2d": "Puzzle2d",
  "puzzle/5d": "Puzzle5d",
  "puzzle/3d": "Puzzle3d",
  "block/2d": "Block2d",
  "block/5d": "Block5d",
  "block/3d": "Block3d",
  "space/home": "SHome",
  "sourcing/curate": "Curate",
};

/** 🔤snake_case → camelCase canonical field name. */
function policySnakeToCamel(name: string): string {
  return name.replace(/_([a-z0-9])/g, (_, c: string) => c.toUpperCase());
}

/** 🔤Normalize state-class tokens to kebab (persistent / shared-ui / …). */
function policyCanonicalState(raw: string): string {
  return raw.trim().toLowerCase().replace(/_/g, "-");
}

/** 🔤Map language type tokens onto §6 canonical scalar ids when exact. */
function policyCanonicalScalar(raw: string): string {
  const t = raw.replace(/\s+/g, "").trim();
  const table: Record<string, string> = {
    String: "string",
    string: "string",
    bool: "bool",
    boolean: "bool",
    Boolean: "bool",
    i32: "int32",
    u32: "uint32",
    i64: "int64",
    f32: "float32",
    f64: "float64",
    Int: "int32",
    Float: "float64",
    bytes: "bytes",
    "Vec<u8>": "bytes",
  };
  return table[t] ?? t;
}

/** 🏷️§10 prefix for an artifact rel path, or null when the artifact is absent from the table. */
function policyArtifactSchemaPrefix(artRel: string): string | null {
  const parts = artRel.replaceAll("\\", "/").split("/");
  const artifactsIdx = parts.indexOf("🗿️artifacts");
  if (artifactsIdx < 1 || artifactsIdx + 1 >= parts.length) return null;
  const plugin = policyStripEmoji(parts[artifactsIdx - 1] ?? "");
  const artifact = policyStripEmoji(parts[artifactsIdx + 1] ?? "");
  return POLICY_ARTIFACT_SCHEMA_PREFIXES[`${plugin}/${artifact}`] ?? null;
}

/** 🏷️Expected type name for a facet path given prefix X. */
function policyExpectedSchemaTypeName(prefix: string, facetRel: string): string {
  if (facetRel === "🧬️schema") return `${prefix}Artifact`;
  if (facetRel === "📸️snapshot/🧬️schema") return `${prefix}Snapshot`;
  return `${prefix}Diff`;
}

/** 🧩Parse Rust type into optional/cardinality/scalar. */
function policyParseRustFieldType(typeText: string): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let t = typeText.replace(/\s+/g, " ").trim();
  let optional = false;
  if (/^Option\s*</.test(t)) {
    optional = true;
    t = t.replace(/^Option\s*<\s*/, "").replace(/\s*>\s*$/, "");
  }
  const mapMatch = /^(?:BTreeMap|HashMap)\s*<\s*String\s*,\s*(.+)\s*>$/.exec(t);
  if (mapMatch) {
    return { optional, cardinality: "map", scalar: policyCanonicalScalar(mapMatch[1]!.trim()) };
  }
  const fixedMatch = /^\[\s*(.+?)\s*;\s*\d+\s*\]$/.exec(t);
  if (fixedMatch) {
    return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(fixedMatch[1]!.trim()) };
  }
  if (/^Vec\s*<\s*u8\s*>$/.test(t)) {
    return { optional, cardinality: "scalar", scalar: "bytes" };
  }
  const vecMatch = /^Vec\s*<\s*(.+)\s*>$/.exec(t);
  if (vecMatch) {
    return { optional, cardinality: "list", scalar: policyCanonicalScalar(vecMatch[1]!.trim()) };
  }
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(t) };
}

/**
 * 🦀️Extract `pub` fields of the single top-level `pub struct`, including `#[state(…)]`.
 */
export function policyExtractRustSchemaFields(text: string): PolicySchemaLeafExtract {
  const structMatch = /\bpub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/.exec(text);
  if (!structMatch) return { typeName: "", fields: [] };
  const typeName = structMatch[1]!;
  const bodyStart = structMatch.index! + structMatch[0].length;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /(?:#\[state\(([^\)]*)\)\]\s*)?pub\s+([a-z][a-z0-9_]*)\s*:\s*([^,{]+)/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const stateRaw = (m[1] ?? "").trim();
    const snake = m[2]!;
    const parsed = policyParseRustFieldType(m[3]!.trim());
    fields.push({
      name: policySnakeToCamel(snake),
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Parse a TypeScript property type into optional/cardinality/scalar. */
function policyParseTsFieldType(typeText: string, optionalMark: boolean): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let t = typeText.replace(/\s+/g, " ").trim().replace(/;$/, "");
  const optional = optionalMark || t.endsWith("| undefined") || t.endsWith("| null");
  t = t.replace(/\s*\|\s*undefined$/, "").replace(/\s*\|\s*null$/, "").trim();
  const recordMatch = /^Record\s*<\s*string\s*,\s*(.+)\s*>$/.exec(t);
  if (recordMatch) {
    return { optional, cardinality: "map", scalar: policyCanonicalScalar(recordMatch[1]!.trim()) };
  }
  const tupleMatch = /^\[\s*(.+?)\s*(?:,\s*\1\s*)+\]$/.exec(t);
  if (tupleMatch && t.includes(",")) {
    const inner = tupleMatch[1]!.trim();
    return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(inner) };
  }
  const arrMatch = /^(?:Array\s*<\s*(.+)\s*>|(.+)\[\])$/.exec(t);
  if (arrMatch) {
    return { optional, cardinality: "list", scalar: policyCanonicalScalar((arrMatch[1] ?? arrMatch[2]!).trim()) };
  }
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(t) };
}

/**
 * 🟦️Extract members of the single exported `interface`, including `/** @state … */` JSDoc.
 */
export function policyExtractTypescriptSchemaFields(text: string): PolicySchemaLeafExtract {
  const ifaceMatch = /\bexport\s+interface\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/.exec(text);
  if (!ifaceMatch) return { typeName: "", fields: [] };
  const typeName = ifaceMatch[1]!;
  const bodyStart = ifaceMatch.index! + ifaceMatch[0].length;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /(?:\/\*\*\s*@state\s+([a-z0-9_-]+)\s*\*\/\s*)?([A-Za-z_][A-Za-z0-9_]*)(\?)?\s*:\s*([^;]+);/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const parsed = policyParseTsFieldType(m[4]!.trim(), Boolean(m[3]));
    fields.push({
      name: m[2]!,
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: m[1] ? policyCanonicalState(m[1]) : "",
    });
  }
  return { typeName, fields };
}

/**
 * 🔗️Extract fields of the single `type`, including `@state(class: …)`.
 */
export function policyExtractGraphqlSchemaFields(text: string): PolicySchemaLeafExtract {
  const typeMatch = /\btype\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/.exec(text);
  if (!typeMatch) return { typeName: "", fields: [] };
  const typeName = typeMatch[1]!;
  const bodyStart = typeMatch.index! + typeMatch[0].length;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(\[[^\]]+\]|[A-Za-z_][A-Za-z0-9_]*)(!)?(?:\s*@state\s*\(\s*class\s*:\s*([A-Z_]+)\s*\))?/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const name = m[1]!;
    const typeTok = m[2]!;
    const required = Boolean(m[3]);
    const stateRaw = m[4] ?? "";
    let cardinality: PolicySchemaFieldCardinality = "scalar";
    let scalar = typeTok;
    const listMatch = /^\[\s*(.+?)\s*!?\s*\]$/.exec(typeTok);
    if (listMatch) {
      const inner = listMatch[1]!.replace(/!$/, "").trim();
      if (/Entry$/.test(inner)) {
        cardinality = "map";
        scalar = inner.replace(/Entry$/, "");
      } else {
        cardinality = "list";
        scalar = inner;
      }
    }
    fields.push({
      name,
      optional: !required,
      cardinality,
      scalar: policyCanonicalScalar(scalar),
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Walk a JSON Schema property schema into cardinality + scalar. */
function policyParseJsonSchemaProperty(prop: Record<string, unknown>): Pick<PolicySchemaFieldShape, "cardinality" | "scalar"> {
  const typ = prop.type;
  if (typ === "array") {
    const minItems = prop.minItems;
    const maxItems = prop.maxItems;
    const items = prop.items as Record<string, unknown> | undefined;
    const scalar = items ? policyJsonSchemaScalar(items) : "unknown";
    if (typeof minItems === "number" && minItems === maxItems) {
      return { cardinality: "fixedList", scalar };
    }
    return { cardinality: "list", scalar };
  }
  if (typ === "object" && prop.additionalProperties != null && prop.additionalProperties !== false) {
    const add = prop.additionalProperties;
    const scalar = typeof add === "object" && add ? policyJsonSchemaScalar(add as Record<string, unknown>) : "unknown";
    return { cardinality: "map", scalar };
  }
  return { cardinality: "scalar", scalar: policyJsonSchemaScalar(prop) };
}

/** 🔤Canonical scalar id from a JSON Schema schema object. */
function policyJsonSchemaScalar(schema: Record<string, unknown>): string {
  if (schema.contentEncoding === "base64") return "bytes";
  if (schema.contentMediaType === "application/json") return "string";
  const typ = schema.type;
  const format = schema.format;
  if (typ === "string") return "string";
  if (typ === "boolean") return "bool";
  if (typ === "integer") {
    if (format === "int32") return "int32";
    if (format === "uint32") return "uint32";
    if (format === "int64") return "int64";
    return "int32";
  }
  if (typ === "number") {
    if (format === "float") return "float32";
    if (format === "double") return "float64";
    return "float64";
  }
  if (typeof schema.$ref === "string") {
    const ref = schema.$ref as string;
    return ref.split("/").pop() ?? ref;
  }
  if (typeof schema.title === "string") return schema.title;
  return typeof typ === "string" ? typ : "unknown";
}

/**
 * 🔣️Extract `properties` + `required` + `x-semio-state` from the normative JSON Schema leaf.
 */
export function policyExtractJsonSchemaFields(text: string): PolicySchemaLeafExtract {
  let doc: Record<string, unknown>;
  try {
    doc = JSON.parse(text) as Record<string, unknown>;
  } catch {
    return { typeName: "", fields: [] };
  }
  const typeName = typeof doc.title === "string" ? doc.title : "";
  const properties = (doc.properties ?? {}) as Record<string, Record<string, unknown>>;
  const required = new Set<string>(Array.isArray(doc.required) ? (doc.required as string[]) : []);
  const fields: PolicySchemaFieldShape[] = [];
  for (const [name, prop] of Object.entries(properties)) {
    const parsed = policyParseJsonSchemaProperty(prop ?? {});
    const stateRaw = typeof prop?.["x-semio-state"] === "string" ? (prop["x-semio-state"] as string) : "";
    fields.push({
      name,
      optional: !required.has(name),
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Parse a protobuf field type into optional/cardinality/scalar. */
function policyParseProtoFieldType(
  typeText: string,
  optionalKw: boolean,
  repeatedKw: boolean,
): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  const mapMatch = /^map\s*<\s*string\s*,\s*(.+)\s*>$/.exec(typeText.trim());
  if (mapMatch) {
    return { optional: optionalKw, cardinality: "map", scalar: policyCanonicalScalar(mapMatch[1]!.trim()) };
  }
  if (repeatedKw) {
    return { optional: optionalKw, cardinality: "list", scalar: policyCanonicalScalar(typeText.trim()) };
  }
  return { optional: optionalKw, cardinality: "scalar", scalar: policyCanonicalScalar(typeText.trim()) };
}

/**
 * 🛰️Extract fields of the single `message`, including `// @state …` leading comments.
 */
export function policyExtractProtobufSchemaFields(text: string): PolicySchemaLeafExtract {
  const msgMatch = /\bmessage\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/.exec(text);
  if (!msgMatch) return { typeName: "", fields: [] };
  const typeName = msgMatch[1]!;
  const bodyStart = msgMatch.index! + msgMatch[0].length;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /(?:\/\/\s*@state\s+([a-z0-9_-]+)\s*\n\s*)?(optional\s+)?(repeated\s+)?(map\s*<\s*string\s*,\s*[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const parsed = policyParseProtoFieldType(m[4]!, Boolean(m[2]), Boolean(m[3]));
    fields.push({
      name: policySnakeToCamel(m[5]!),
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: m[1] ? policyCanonicalState(m[1]) : "",
    });
  }
  return { typeName, fields };
}

/** 🗂️Load every schemaFormats leaf for one facet; returns null entries for missing files. */
function policyLoadSchemaFacetLeaves(
  repoRoot: string,
  facetRel: string,
): { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] {
  const taxonomy = loadTaxonomy();
  const formats = taxonomy.schemaFormats ?? {};
  const out: { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] = [];
  for (const [formatId, format] of Object.entries(formats)) {
    const leafFilename = format.leafFilename;
    const relPath = `${facetRel}/${leafFilename}`;
    const abs = join(repoRoot, relPath);
    if (!existsSync(abs)) {
      out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract: null });
      continue;
    }
    const text = readFileSync(abs, "utf8");
    let extract: PolicySchemaLeafExtract;
    switch (formatId) {
      case "🦀️rust":
        extract = policyExtractRustSchemaFields(text);
        break;
      case "🟦️typescript":
        extract = policyExtractTypescriptSchemaFields(text);
        break;
      case "🔗️graphql":
        extract = policyExtractGraphqlSchemaFields(text);
        break;
      case "🔣️jsonschema":
        extract = policyExtractJsonSchemaFields(text);
        break;
      case "🛰️protobuf":
        extract = policyExtractProtobufSchemaFields(text);
        break;
      default:
        extract = { typeName: "", fields: [] };
        break;
    }
    out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract });
  }
  return out;
}

/**
 * 📏️Facet completeness + normative leaf: all three facet dirs, each with every schemaFormats leaf
 * and the `artifactSchemaSpecFilenames` normative JSON Schema leaf.
 */
function policyArtifactSchemaFacetCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const formats = Object.entries(taxonomy.schemaFormats ?? {});
  const normativeByFacet = taxonomy.artifactSchemaSpecFilenames ?? {};
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) {
        breaches.push({
          id: `artifact-schema-facet-missing-${facetAbs}`,
          summary: `"${artRel}" is missing required schema facet ${facetRel}/`,
          kind: "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: "Every artifact must expose 🧬️schema, 📸️snapshot/🧬️schema, and 🔺️diff/🧬️schema facets.",
          solution: `Create ${facetAbs}/ with all five schemaFormats leaves (and the normative 🔣️component.json).`,
        });
        continue;
      }
      for (const [formatId, format] of formats) {
        const leafRel = `${facetAbs}/${format.leafFilename}`;
        if (existsSync(join(repoRoot, leafRel))) continue;
        breaches.push({
          id: `artifact-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
          kind: "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf filename from 🔣️taxonomy.json.",
          solution: `Add handcrafted ${leafRel}.`,
        });
      }
      const normative = normativeByFacet[facetRel];
      if (normative) {
        const normativeRel = `${facetAbs}/${normative}`;
        if (!existsSync(join(repoRoot, normativeRel))) {
          breaches.push({
            id: `artifact-schema-normative-missing-${normativeRel}`,
            summary: `"${facetAbs}" is missing normative artifactSchemaSpecFilenames leaf ${normative}`,
            kind: "artifact-schema/normative-leaf",
            scope: artRel,
            priority: "high",
            reason: "Within a facet the 🔣️component.json JSON Schema leaf is normative; the other four mirror it.",
            solution: `Add ${normativeRel} as the source of truth for this facet's fields.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Field parity: all five leaves of one facet declare the identical canonical field set with identical
 * optionality and cardinality; JSON Schema is the truth when others disagree.
 */
function policyArtifactSchemaFieldParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const leaves = policyLoadSchemaFacetLeaves(repoRoot, facetAbs);
      if (leaves.some((l) => l.extract === null)) continue;
      const jsonLeaf = leaves.find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      const truth = new Map(jsonLeaf.extract.fields.map((f) => [f.name, f]));
      for (const leaf of leaves) {
        if (leaf.formatId === "🔣️jsonschema" || !leaf.extract) continue;
        const seen = new Map(leaf.extract.fields.map((f) => [f.name, f]));
        for (const [name, truthField] of truth) {
          const other = seen.get(name);
          if (!other) {
            breaches.push({
              id: `artifact-schema-field-parity-missing-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" is missing field "${name}" present in normative JSON Schema`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Field parity requires identical canonical fields across all five leaves; JSON Schema is normative (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
              solution: `Add field "${name}" to ${leaf.relPath} matching ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}, scalar=${truthField.scalar}).`,
            });
            continue;
          }
          if (other.optional !== truthField.optional || other.cardinality !== truthField.cardinality) {
            breaches.push({
              id: `artifact-schema-field-parity-shape-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" field "${name}" disagrees with normative JSON Schema optionality/cardinality`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Normative ${jsonLeaf.relPath} declares "${name}" as optional=${truthField.optional}, cardinality=${truthField.cardinality}; ${leaf.formatId} has optional=${other.optional}, cardinality=${other.cardinality}.`,
              solution: `Change "${name}" in ${leaf.relPath} to match ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
            });
          }
        }
        for (const name of seen.keys()) {
          if (truth.has(name)) continue;
          breaches.push({
            id: `artifact-schema-field-parity-extra-${leaf.relPath}-${name}`,
            summary: `"${leaf.relPath}" declares extra field "${name}" absent from normative JSON Schema`,
            kind: "artifact-schema/field-parity",
            scope: artRel,
            priority: "high",
            reason: `JSON Schema at ${jsonLeaf.relPath} is normative; extra fields in other formats break cross-format identity.`,
            solution: `Remove "${name}" from ${leaf.relPath}, or add it to ${jsonLeaf.relPath} if it is a real artifact field.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️State-class parity: snapshot facet fields equal exactly the persistent fields of the artifact facet.
 */
function policyArtifactSchemaStateParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const artifactFacet = `${artRel}/🧬️schema`;
    const snapshotFacet = `${artRel}/📸️snapshot/🧬️schema`;
    if (!existsSync(join(repoRoot, artifactFacet)) || !existsSync(join(repoRoot, snapshotFacet))) continue;
    const artLeaves = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet);
    const snapLeaves = policyLoadSchemaFacetLeaves(repoRoot, snapshotFacet);
    const artJson = artLeaves.find((l) => l.formatId === "🔣️jsonschema")?.extract;
    const snapJson = snapLeaves.find((l) => l.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !snapJson) continue;
    const persistent = artJson.fields.filter((f) => f.state === "persistent");
    const snapMap = new Map(snapJson.fields.map((f) => [f.name, f]));
    const persMap = new Map(persistent.map((f) => [f.name, f]));
    for (const f of persistent) {
      const s = snapMap.get(f.name);
      if (!s) {
        breaches.push({
          id: `artifact-schema-state-parity-missing-${artRel}-${f.name}`,
          summary: `Snapshot facet is missing persistent artifact field "${f.name}"`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: "XSnapshot must equal exactly the persistent fields of XArtifact (equality, not subset).",
          solution: `Add "${f.name}" to ${snapshotFacet}/🔣️component.json (and the other four leaves) matching the artifact facet.`,
        });
        continue;
      }
      if (s.optional !== f.optional || s.cardinality !== f.cardinality) {
        breaches.push({
          id: `artifact-schema-state-parity-shape-${artRel}-${f.name}`,
          summary: `Snapshot field "${f.name}" shape differs from persistent artifact field`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: `Persistent artifact field "${f.name}" is optional=${f.optional}, cardinality=${f.cardinality}; snapshot has optional=${s.optional}, cardinality=${s.cardinality}.`,
          solution: `Align "${f.name}" in ${snapshotFacet}/🔣️component.json with ${artifactFacet}/🔣️component.json.`,
        });
      }
    }
    for (const name of snapMap.keys()) {
      if (persMap.has(name)) continue;
      breaches.push({
        id: `artifact-schema-state-parity-extra-${artRel}-${name}`,
        summary: `Snapshot facet has non-persistent field "${name}"`,
        kind: "artifact-schema/state-parity",
        scope: artRel,
        priority: "high",
        reason: "XSnapshot may only contain the persistent fields of XArtifact.",
        solution: `Remove "${name}" from ${snapshotFacet}/🔣️component.json, or mark it persistent on the artifact facet if it belongs there.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Diff coverage: every non-effect artifact field has a diff entry; no effect field does; `artifact` exists.
 */
function policyArtifactSchemaDiffCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const artifactFacet = `${artRel}/🧬️schema`;
    const diffFacet = `${artRel}/🔺️diff/🧬️schema`;
    if (!existsSync(join(repoRoot, artifactFacet)) || !existsSync(join(repoRoot, diffFacet))) continue;
    const artJson = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet).find((l) => l.formatId === "🔣️jsonschema")?.extract;
    const diffJson = policyLoadSchemaFacetLeaves(repoRoot, diffFacet).find((l) => l.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !diffJson) continue;
    const diffNames = new Set(diffJson.fields.map((f) => f.name));
    if (!diffNames.has("artifact")) {
      breaches.push({
        id: `artifact-schema-diff-artifact-entry-${artRel}`,
        summary: `Diff facet is missing whole-replacement field "artifact"`,
        kind: "artifact-schema/diff-coverage",
        scope: artRel,
        priority: "high",
        reason: "XDiff must include `artifact: Option<Box<XArtifact>>` for whole-artifact replacement.",
        solution: `Add field "artifact" to ${diffFacet}/🔣️component.json (and the other four leaves).`,
      });
    }
    for (const f of artJson.fields) {
      if (f.state === "effect") {
        if (diffNames.has(f.name)) {
          breaches.push({
            id: `artifact-schema-diff-effect-${artRel}-${f.name}`,
            summary: `Diff facet must not cover effect field "${f.name}"`,
            kind: "artifact-schema/diff-coverage",
            scope: artRel,
            priority: "high",
            reason: "Effect fields are fire-and-forget and must not appear in XDiff.",
            solution: `Remove "${f.name}" from ${diffFacet}/🔣️component.json.`,
          });
        }
        continue;
      }
      if (!diffNames.has(f.name)) {
        breaches.push({
          id: `artifact-schema-diff-coverage-${artRel}-${f.name}`,
          summary: `Diff facet is missing entry for non-effect artifact field "${f.name}"`,
          kind: "artifact-schema/diff-coverage",
          scope: artRel,
          priority: "high",
          reason: "Every artifact field whose state class is not effect must have a same-named diff entry.",
          solution: `Add sparse diff entry "${f.name}" to ${diffFacet}/🔣️component.json matching §7.3 cardinality rules.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Type-name parity: XArtifact / XSnapshot / XDiff spelled identically across all five leaves of their facet.
 */
function policyArtifactSchemaTypeNameParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const prefix = policyArtifactSchemaPrefix(artRel);
    if (!prefix) {
      breaches.push({
        id: `artifact-schema-prefix-unknown-${artRel}`,
        summary: `"${artRel}" has no §10 schema type prefix mapping`,
        kind: "artifact-schema/type-name-parity",
        scope: artRel,
        priority: "high",
        reason: "Type-name parity derives the expected XArtifact/XSnapshot/XDiff names from the normative §10 prefix table — never by guessing.",
        solution: `Add a prefix entry for this artifact to POLICY_ARTIFACT_SCHEMA_PREFIXES in 📜️script.ts (see normative-spec §10).`,
      });
      continue;
    }
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const expected = policyExpectedSchemaTypeName(prefix, facetRel);
      const leaves = policyLoadSchemaFacetLeaves(repoRoot, facetAbs);
      for (const leaf of leaves) {
        if (!leaf.extract) continue;
        if (!leaf.extract.typeName) {
          breaches.push({
            id: `artifact-schema-type-name-missing-${leaf.relPath}`,
            summary: `"${leaf.relPath}" does not declare top-level type ${expected}`,
            kind: "artifact-schema/type-name-parity",
            scope: artRel,
            priority: "high",
            reason: `Every leaf of facet ${facetRel} must declare the same top-level type name ${expected}.`,
            solution: `Declare ${expected} as the top-level type in ${leaf.relPath}.`,
          });
          continue;
        }
        if (leaf.extract.typeName !== expected) {
          breaches.push({
            id: `artifact-schema-type-name-${leaf.relPath}`,
            summary: `"${leaf.relPath}" declares ${leaf.extract.typeName} but §10 expects ${expected}`,
            kind: "artifact-schema/type-name-parity",
            scope: artRel,
            priority: "high",
            reason: `Type-name parity requires ${expected} in all five leaves of ${facetRel} (prefix ${prefix} from §10).`,
            solution: `Rename the top-level type in ${leaf.relPath} to ${expected}.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Pack relocation: no `🎒️pack` may sit directly under an artifact root (it lives under 📸️snapshot).
 */
function policyArtifactSchemaPackRelocationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const packRel = `${artRel}/🎒️pack`;
    if (!existsSync(join(repoRoot, packRel))) continue;
    breaches.push({
      id: `artifact-schema-pack-root-${artRel}`,
      summary: `"${packRel}" must move under 📸️snapshot/🎒️pack`,
      kind: "artifact-schema/pack-relocation",
      scope: artRel,
      priority: "high",
      reason: "A pack encodes exactly the snapshot; bare 🎒️pack on the artifact root is forbidden.",
      solution: `Move ${packRel}/ to ${artRel}/📸️snapshot/🎒️pack/ and update glue #[path] mounts.`,
    });
  }
  return breaches;
}

/** ⚖️Aggregates artifact-schema facet scanners (completeness, parity, coverage, pack relocation). */
export function policyArtifactSchemaBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyArtifactSchemaFacetCompletenessBreaches(repoRoot),
    ...policyArtifactSchemaFieldParityBreaches(repoRoot),
    ...policyArtifactSchemaStateParityBreaches(repoRoot),
    ...policyArtifactSchemaDiffCoverageBreaches(repoRoot),
    ...policyArtifactSchemaTypeNameParityBreaches(repoRoot),
    ...policyArtifactSchemaPackRelocationBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleArtifactSchemas
