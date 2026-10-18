/** 📐️ Renders one schema export as a GraphQL type, a protobuf message or a TypeScript type, derived from the normative JSON Schema alone.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { EXPORT_NAME, IDENTIFIER_NAME, asDict, has, isDict, jsonString, jsonValue, orderedEntries, snake, truthy, type Dict } from "../🔡️json-text/🟦️.ts";

//#region 🔖️Shapes
/** 🔎️ The one element schema of an array; a positional (tuple) `items` list has none. */
export const itemSchema = (node: Dict): Dict => (isDict(node.items) ? node.items : {});

/** 🔎️ The export a `$ref` names, if its last segment is an export name. */
const referenced = (node: Dict): string | undefined => {
  const target = String(node.$ref).split("/").at(-1)!;
  return EXPORT_NAME.test(target) ? target : undefined;
};

const isNull = (node: unknown): boolean => isDict(node) && node.type === "null";

/** 🔎️ The single non-null alternative of `type: [T, "null"]` or `anyOf|oneOf: [T, {"type": "null"}]`. */
export function nullableOf(node: Dict): Dict | undefined {
  const kinds = node.type;
  if (Array.isArray(kinds) && kinds.includes("null")) {
    const rest = kinds.filter((kind) => kind !== "null");
    return rest.length === 1 ? { ...Object.fromEntries(Object.entries(node).filter(([key]) => key !== "type")), type: rest[0] } : undefined;
  }
  for (const key of ["anyOf", "oneOf"]) {
    const alternatives = node[key];
    if (Array.isArray(alternatives) && alternatives.length === 2 && alternatives.filter(isNull).length === 1) {
      const other = alternatives.find((alternative) => !isNull(alternative));
      return isDict(other) ? other : undefined;
    }
  }
  return undefined;
}

const stringEnum = (node: Dict): readonly string[] | undefined => (Array.isArray(node.enum) && node.enum.every((entry) => typeof entry === "string") ? node.enum : undefined);

const isScalar = (value: unknown): value is string | number | bigint | boolean => typeof value === "string" || typeof value === "number" || typeof value === "bigint" || typeof value === "boolean";

const objectEntries = (node: Dict): [string, unknown][] | undefined => {
  if (node.type !== "object" || !isDict(node.properties) || !truthy(node.properties)) return undefined;
  return orderedEntries(node.properties) ?? undefined;
};
//#endregion 🔖️Shapes

//#region 🔖️GraphQL
function graphqlType(node: Dict): string | undefined {
  if (has(node, "$ref")) return referenced(node);
  const kind = node.type;
  if (kind === "string") return "String";
  if (kind === "boolean") return "Boolean";
  if (kind === "integer") return "Int";
  if (kind === "number") return "Float";
  if (kind === "array") {
    const inner = graphqlType(itemSchema(node));
    return inner ? `[${inner}!]` : undefined;
  }
  return undefined;
}

/** 🔗️ A GraphQL enum, object type or scalar for one export, or `undefined` when the schema has no GraphQL form. */
export function renderGraphql(name: string, definition: Dict): string | undefined {
  const members = stringEnum(definition);
  if (members) {
    if (members.every((entry) => IDENTIFIER_NAME.test(entry))) return `enum ${name} {\n${members.map((entry) => `  ${entry}`).join("\n")}\n}\n`;
    return `# 🔤️ ${name} is a string domain whose members are not GraphQL names; the JSON Schema enumerates them.\nscalar ${name}\n`;
  }
  const entries = objectEntries(definition);
  if (entries) {
    const required = new Set(Array.isArray(definition.required) ? definition.required : []);
    const lines: string[] = [];
    for (const [field, shape] of entries) {
      const rendered = graphqlType(asDict(shape));
      if (rendered === undefined) return undefined;
      lines.push(`  ${field}: ${rendered}${required.has(field) ? "!" : ""}`);
    }
    return `type ${name} {\n${lines.join("\n")}\n}\n`;
  }
  if (["string", "integer", "number", "boolean"].includes(definition.type as string) || has(definition, "oneOf") || has(definition, "anyOf")) return `scalar ${name}\n`;
  return undefined;
}
//#endregion 🔖️GraphQL

//#region 🔖️Protobuf
function protoType(node: Dict): [string, boolean] | undefined {
  if (has(node, "$ref")) {
    const target = referenced(node);
    return target === undefined ? undefined : [target, false];
  }
  const kind = node.type;
  const declared = node.format;
  if (kind === "string") return ["string", false];
  if (kind === "boolean") return ["bool", false];
  if (kind === "integer") return [["uint32", "uint64", "int32", "int64"].includes(declared as string) ? (declared as string) : "int64", false];
  if (kind === "number") return [["double", "float"].includes(declared as string) ? (declared as string) : "double", false];
  if (kind === "array") {
    const inner = protoType(itemSchema(node));
    return inner && !inner[1] ? [inner[0], true] : undefined;
  }
  return undefined;
}

/** 🛰️ A protobuf message for one export, or `undefined` when the schema has no protobuf form. */
export function renderProto(name: string, definition: Dict): string | undefined {
  const entries = objectEntries(definition);
  if (entries) {
    const required = new Set(Array.isArray(definition.required) ? definition.required : []);
    const lines: string[] = [];
    let index = 1;
    for (const [field, shape] of entries) {
      const rendered = protoType(asDict(shape));
      if (rendered === undefined) return undefined;
      const [kind, repeated] = rendered;
      const prefix = repeated ? "repeated " : required.has(field) ? "" : "optional ";
      lines.push(`  ${prefix}${kind} ${snake(field)} = ${index};`);
      index += 1;
    }
    return `message ${name} {\n${lines.join("\n")}\n}\n`;
  }
  const scalar = protoType(definition);
  if (scalar && !scalar[1]) return `message ${name} {\n  ${scalar[0]} value = 1;\n}\n`;
  if (stringEnum(definition)) return `message ${name} {\n  string value = 1;\n}\n`;
  return undefined;
}
//#endregion 🔖️Protobuf

//#region 🔖️TypeScript
/** 🟦️ The TypeScript type text of one schema node, or `undefined` when it has no plain form. */
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
  const kind = node.type;
  if (kind === "string") return "string";
  if (kind === "boolean") return "boolean";
  if (kind === "integer" || kind === "number") return "number";
  if (kind === "array") {
    const inner = tsType(itemSchema(node));
    return inner ? (inner.includes(" | ") ? `readonly (${inner})[]` : `readonly ${inner}[]`) : undefined;
  }
  if (kind === "object" && !truthy(node.properties)) return "Readonly<Record<string, unknown>>";
  return undefined;
}

/** 🟦️ An exported TypeScript interface or alias for one export, or `undefined` when the schema has no plain form. */
export function renderTsType(name: string, definition: Dict): string | undefined {
  const entries = objectEntries(definition);
  if (entries) {
    const required = new Set(Array.isArray(definition.required) ? definition.required : []);
    const lines: string[] = [];
    for (const [field, shape] of entries) {
      const rendered = tsType(asDict(shape));
      if (rendered === undefined) return undefined;
      lines.push(`  readonly ${field}${required.has(field) ? "" : "?"}: ${rendered};`);
    }
    return `export interface ${name} {\n${lines.join("\n")}\n}\n`;
  }
  const inline = tsType(definition);
  return inline ? `export type ${name} = ${inline};\n` : undefined;
}
//#endregion 🔖️TypeScript
