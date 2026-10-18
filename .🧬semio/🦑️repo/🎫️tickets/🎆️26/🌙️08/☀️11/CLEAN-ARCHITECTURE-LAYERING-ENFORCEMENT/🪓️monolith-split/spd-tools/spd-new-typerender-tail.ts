//#region 🔖️Auxiliaries
/** 🏷️ True when a name is already declared in the leaf (or is an export of the document) and must not be taken by an auxiliary declaration. */
export type Taken = (name: string) => boolean;

/** 📦️ The named declarations one rendering adds in front of its export (`<Parent><Field>` for a nested object, the `JSON` scalar), and the names it has claimed. */
class Auxiliaries {
  private readonly claimed = new Set<string>();
  private readonly texts = new Map<string, string>();

  constructor(private readonly taken: Taken, root: string) {
    this.claimed.add(root);
  }

  claim(name: string): boolean {
    if (this.claimed.has(name) || this.taken(name)) return false;
    this.claimed.add(name);
    return true;
  }

  put(name: string, text: string): void {
    this.texts.set(name, text);
  }

  json(): void {
    if (this.claim("JSON")) this.put("JSON", "scalar JSON\n");
  }

  all(): string[] {
    return [...this.texts.values()];
  }
}
//#endregion 🔖️Auxiliaries

//#region 🔖️GraphQL
interface GraphqlType {
  readonly text: string;
  readonly nullable: boolean;
}

const GRAPHQL_SCALARS = { string: "String", boolean: "Boolean", integer: "Int", number: "Float" } as const;

/** 🔗️ A `type <name>` of a nested object, or `undefined` when a property has no GraphQL name. */
function graphqlObject(name: string, node: Dict, aux: Auxiliaries): string | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  for (const [field, shape] of entries) {
    if (!IDENTIFIER_NAME.test(field)) return undefined;
    const rendered = graphqlType(asDict(shape), name, field, aux);
    lines.push(`  ${field}: ${rendered.text}${required.has(field) && !rendered.nullable ? "!" : ""}`);
  }
  return `type ${name} {\n${lines.join("\n")}\n}\n`;
}

/** 🔗️ The GraphQL type of one schema node and whether it admits `null`. Every node has one: a nested object becomes the type `<Parent><Field>`, an enum or constant its scalar, and a form GraphQL cannot express (a map, a union, any value, a tuple) the scalar `JSON`. */
function graphqlType(node: Dict, parent: string, field: string, aux: Auxiliaries): GraphqlType {
  const json = (): GraphqlType => {
    aux.json();
    return { text: "JSON", nullable: false };
  };
  if (has(node, "$ref")) {
    const target = referenced(node);
    return target === undefined ? json() : { text: target, nullable: false };
  }
  const present = nullableOf(node);
  if (present) return { ...graphqlType(present, parent, field, aux), nullable: true };
  const pinned = pinnedKind(node);
  const kind = pinned ?? (typeof node.type === "string" ? node.type : undefined);
  if (kind === "string" || kind === "boolean" || kind === "integer" || kind === "number") return { text: GRAPHQL_SCALARS[kind], nullable: false };
  if (kind === "array") {
    if (!isDict(node.items)) return json();
    const inner = graphqlType(node.items, parent, field, aux);
    return { text: `[${inner.text}${inner.nullable ? "" : "!"}]`, nullable: false };
  }
  if (kind === "object" && objectEntries(node) !== undefined) {
    const name = `${parent}${pascal(field)}`;
    if (pascal(field) === "" || !aux.claim(name)) return json();
    const text = graphqlObject(name, node, aux);
    if (text === undefined) return json();
    aux.put(name, text);
    return { text: name, nullable: false };
  }
  return json();
}

/** 🔗️ The GraphQL declarations of one export: an enum, an object type (with the types of its nested objects in front) or a scalar. `undefined` only when a property has no GraphQL name. */
export function renderGraphql(name: string, definition: Dict, taken: Taken = () => false): string | undefined {
  const members = stringEnum(definition);
  if (members) {
    if (members.every((entry) => IDENTIFIER_NAME.test(entry))) return `enum ${name} {\n${members.map((entry) => `  ${entry}`).join("\n")}\n}\n`;
    return `# 🔤️ ${name} is a string domain whose members are not GraphQL names; the JSON Schema enumerates them.\nscalar ${name}\n`;
  }
  const aux = new Auxiliaries(taken, name);
  const entries = objectEntries(definition);
  if (entries !== undefined) {
    const text = graphqlObject(name, definition, aux);
    return text === undefined ? undefined : [...aux.all(), text].join("\n");
  }
  if (isDict(definition.properties) && truthy(definition.properties)) return undefined;
  return `scalar ${name}\n`;
}
//#endregion 🔖️GraphQL

//#region 🔖️Protobuf
interface ProtoType {
  readonly text: string;
  readonly repeated: boolean;
  readonly map: boolean;
}

const BYTES: ProtoType = { text: "bytes", repeated: false, map: false };

const single = (text: string): ProtoType => ({ text, repeated: false, map: false });

/** 🛰️ A `message <name>` of a nested object, or `undefined` when a property has no protobuf name. */
function protoMessage(name: string, node: Dict, aux: Auxiliaries): string | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  const names = new Set<string>();
  let index = 1;
  for (const [field, shape] of entries) {
    const label = snake(field);
    if (!IDENTIFIER_NAME.test(label) || names.has(label)) return undefined;
    names.add(label);
    const rendered = protoType(asDict(shape), name, field, aux);
    const prefix = rendered.repeated ? "repeated " : rendered.map ? "" : required.has(field) && nullableOf(asDict(shape)) === undefined ? "" : "optional ";
    lines.push(`  ${prefix}${rendered.text} ${label} = ${index};`);
    index += 1;
  }
  return `message ${name} {\n${lines.join("\n")}\n}\n`;
}

/** 🛰️ The protobuf type of one schema node. Every node has one: a nested object becomes the message `<Parent><Field>`, a typed map `map<string, V>`, an array `repeated`, and a form protobuf cannot express (a union, any value, a tuple, a nullable array, an array of arrays) `bytes`, the JSON text of the value. */
function protoType(node: Dict, parent: string, field: string, aux: Auxiliaries): ProtoType {
  if (has(node, "$ref")) {
    const target = referenced(node);
    return target === undefined ? BYTES : single(target);
  }
  const present = nullableOf(node);
  if (present) {
    const inner = protoType(present, parent, field, aux);
    return inner.repeated || inner.map ? BYTES : inner;
  }
  const pinned = pinnedKind(node);
  const kind = pinned ?? (typeof node.type === "string" ? node.type : undefined);
  const declared = node.format;
  if (kind === "string") return single("string");
  if (kind === "boolean") return single("bool");
  if (kind === "integer") return single(["uint32", "uint64", "int32", "int64"].includes(declared as string) ? (declared as string) : "int64");
  if (kind === "number") return single(["double", "float"].includes(declared as string) ? (declared as string) : "double");
  if (kind === "array") {
    if (!isDict(node.items) || nullableOf(node.items) !== undefined) return BYTES;
    const inner = protoType(node.items, parent, field, aux);
    return inner.repeated || inner.map ? BYTES : { text: inner.text, repeated: true, map: false };
  }
  if (kind === "object") {
    if (objectEntries(node) !== undefined) {
      const name = `${parent}${pascal(field)}`;
      if (pascal(field) === "" || !aux.claim(name)) return BYTES;
      const text = protoMessage(name, node, aux);
      if (text === undefined) return BYTES;
      aux.put(name, text);
      return single(name);
    }
    const values = node.additionalProperties;
    if (isDict(values) && !isAny(values) && nullableOf(values) === undefined) {
      const inner = protoType(values, parent, field, aux);
      return inner.repeated || inner.map ? BYTES : { text: `map<string, ${inner.text}>`, repeated: false, map: true };
    }
  }
  return BYTES;
}

/** 🛰️ The protobuf messages of one export: the message itself (with the messages of its nested objects in front) or, for a value that is no object, a message whose one field carries the value. `undefined` only when a property has no protobuf name. */
export function renderProto(name: string, definition: Dict, taken: Taken = () => false): string | undefined {
  const aux = new Auxiliaries(taken, name);
  const entries = objectEntries(definition);
  if (entries !== undefined) {
    const text = protoMessage(name, definition, aux);
    return text === undefined ? undefined : [...aux.all(), text].join("\n");
  }
  if (isDict(definition.properties) && truthy(definition.properties)) return undefined;
  if (stringEnum(definition)) return `message ${name} {\n  string value = 1;\n}\n`;
  const rendered = protoType(definition, name, "value", aux);
  const field = rendered.map ? `${rendered.text} entries = 1;` : `${rendered.repeated ? "repeated " : ""}${rendered.text} value = 1;`;
  return [...aux.all(), `message ${name} {\n  ${field}\n}\n`].join("\n");
}
//#endregion 🔖️Protobuf

//#region 🔖️TypeScript
/** 🔎️ True when `text` has `operator` outside every bracket and string literal. */
function splitsAtTop(text: string, operator: string): boolean {
  let depth = 0;
  let quote: string | undefined;
  for (let at = 0; at < text.length; at += 1) {
    const char = text[at]!;
    if (quote !== undefined) {
      if (char === "\\") at += 1;
      else if (char === quote) quote = undefined;
    } else if (char === '"' || char === "'" || char === "`") quote = char;
    else if ("([{<".includes(char)) depth += 1;
    else if (")]}>".includes(char) && !(char === ">" && text[at - 1] === "=")) depth -= 1;
    else if (depth === 0 && text.startsWith(operator, at)) return true;
  }
  return false;
}

const group = (text: string): string => (splitsAtTop(text, " | ") || splitsAtTop(text, " & ") ? `(${text})` : text);

const propertyName = (field: string): string => (IDENTIFIER_NAME.test(field) ? field : jsonString(field, false));

function tsFields(node: Dict): string[] | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  for (const [field, shape] of entries) {
    const rendered = tsType(asDict(shape));
    if (rendered === undefined) return undefined;
    lines.push(`readonly ${propertyName(field)}${required.has(field) ? "" : "?"}: ${rendered}`);
  }
  return lines;
}

/** 🟦️ The TypeScript type text of one schema node, or `undefined` when it has no plain form: a reference outside the exports, or a nested object with a property that has no plain form. A union is the union of its alternatives, `allOf` an intersection, a positional `items` list a tuple, a nested object an object type, and a node that states no structure `unknown`. */
export function tsType(node: Dict): string | undefined {
  if (has(node, "$ref")) return referenced(node);
  const nullable = nullableOf(node);
  if (nullable) {
    const inner = tsType(nullable);
    return inner ? `${inner} | null` : undefined;
  }
  const members = stringEnum(node);
  if (members) return members.map((entry) => jsonString(entry, false)).join(" | ");
  if (has(node, "const") && isScalar(node.const)) return jsonValue(node.const, false);
  if (Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every(isScalar)) return node.enum.map((entry) => jsonValue(entry, false)).join(" | ");
  const kind = node.type;
  if (kind === undefined) {
    for (const key of ["oneOf", "anyOf"]) {
      const alternatives = node[key];
      if (Array.isArray(alternatives) && alternatives.length > 0) {
        const parts = alternatives.map((alternative) => (isDict(alternative) ? tsType(alternative) : undefined));
        return parts.every((part) => part !== undefined) ? parts.join(" | ") : undefined;
      }
    }
    if (Array.isArray(node.allOf) && node.allOf.length > 0) {
      const parts = node.allOf.map((part) => (isDict(part) ? tsType(part) : undefined));
      return parts.every((part) => part !== undefined) ? parts.map((part) => group(part!)).join(" & ") : undefined;
    }
    if (isDict(node.properties) && truthy(node.properties)) return tsType({ ...node, type: "object" });
    return isAny(node) ? "unknown" : undefined;
  }
  if (Array.isArray(kind)) {
    const parts = kind.map((entry) => (typeof entry === "string" ? tsType({ ...node, type: entry }) : undefined));
    return parts.every((part) => part !== undefined) ? parts.join(" | ") : undefined;
  }
  if (kind === "null") return "null";
  if (kind === "string") return "string";
  if (kind === "boolean") return "boolean";
  if (kind === "integer" || kind === "number") return "number";
  if (kind === "array") {
    if (Array.isArray(node.items)) {
      const parts = node.items.map((member) => (isDict(member) ? tsType(member) : undefined));
      return parts.every((part) => part !== undefined) ? `readonly [${parts.join(", ")}]` : undefined;
    }
    const inner = tsType(itemSchema(node));
    return inner ? `readonly ${group(inner)}[]` : undefined;
  }
  if (kind === "object") {
    if (!truthy(node.properties)) {
      const values = node.additionalProperties;
      const inner = isDict(values) && !isAny(values) ? tsType(values) : undefined;
      return inner ? `Readonly<Record<string, ${inner}>>` : "Readonly<Record<string, unknown>>";
    }
    const fields = tsFields(node);
    return fields === undefined ? undefined : `{ ${fields.join("; ")} }`;
  }
  return undefined;
}

/** 🟦️ An exported TypeScript interface or alias for one export, or `undefined` when the schema has no plain form. */
export function renderTsType(name: string, definition: Dict): string | undefined {
  const fields = tsFields(definition);
  if (fields !== undefined) return `export interface ${name} {\n${fields.map((line) => `  ${line};`).join("\n")}\n}\n`;
  const inline = tsType(definition);
  return inline ? `export type ${name} = ${inline};\n` : undefined;
}
//#endregion 🔖️TypeScript
