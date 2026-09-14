// 🏗️ Regenerates the committed GraphQL codegen regions of the VS Code extension.
//
// The extension package declares no graphql-codegen pipeline: the codegen output is committed by
// hand into one big `🟦️.ts`, and it had drifted into spliced, unparsable text with no `documents`
// map at all, so every `graphql(...)` call resolved to `{}`. This script is the generator that was
// missing: it reads the operations out of the file's own `⌛️Queries` region, validates every one of
// them against `🔗️graphql/🧬️schema/🔣️schema.graphql` with the real `graphql` package, and rewrites
// the `🧬️CodegenGraphql` document constants and the whole `🧬️CodegenGql` region from the parsed
// ASTs. Run it from the repository root:
//
//   bun ./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🏗️build-vscode-codegen.ts

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import {
  buildSchema,
  GraphQLEnumType,
  GraphQLList,
  GraphQLNonNull,
  GraphQLObjectType,
  GraphQLScalarType,
  parse,
  print,
  typeFromAST,
  validate,
  type GraphQLNamedType,
  type GraphQLOutputType,
  type GraphQLSchema,
  type OperationDefinitionNode,
  type SelectionSetNode,
} from "graphql";

const root = process.cwd();
const outputDir = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🗑️generated/vscode-codegen`;
const extensionPath = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`;
const schemaPath = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧬️schema/🔣️schema.graphql`;

/** 📥️ Every `export const XDocument = graphql(`…`)` of the `⌛️Queries` region, in file order. */
const readOperations = (source: string): Array<{ name: string; text: string }> => {
  const region = source.slice(source.indexOf("// #region ⌛️Queries"), source.indexOf("// #endregion ⌛️Queries"));
  const found: Array<{ name: string; text: string }> = [];
  const pattern = /export const (\w+)Document = graphql\(`([\s\S]*?)`\);/g;
  for (let match = pattern.exec(region); match !== null; match = pattern.exec(region)) found.push({ name: match[1], text: match[2] });
  return found;
};

/** 🧾️ A JSON literal written the way the client preset writes it: one line, no source positions and
 * no empty `arguments`, `directives` or `variableDefinitions` members. */
const literal = (value: unknown): string =>
  JSON.stringify(value, (key, member) => {
    if (key === "loc") return undefined;
    if ((key === "arguments" || key === "directives" || key === "variableDefinitions") && Array.isArray(member) && member.length === 0) return undefined;
    return member;
  })
    .replace(/([{[,:])/g, "$1 ")
    .replace(/([}\]])/g, " $1")
    .replace(/\s+/g, " ")
    .replace(/\[ /g, "[")
    .replace(/ \]/g, "]")
    .trim();

/** 🏷️ The TypeScript spelling of one output type, in the shape the client preset emits. */
const outputType = (schema: GraphQLSchema, type: GraphQLOutputType, selectionSet: SelectionSetNode | undefined): string => {
  if (type instanceof GraphQLNonNull) return nonNullOutputType(schema, type.ofType, selectionSet);
  return `${nonNullOutputType(schema, type, selectionSet)} | null`;
};

const nonNullOutputType = (schema: GraphQLSchema, type: GraphQLOutputType, selectionSet: SelectionSetNode | undefined): string => {
  if (type instanceof GraphQLNonNull) return nonNullOutputType(schema, type.ofType, selectionSet);
  if (type instanceof GraphQLList) return `Array<${outputType(schema, type.ofType, selectionSet)}>`;
  if (type instanceof GraphQLObjectType || (typeof (type as { getTypes?: unknown }).getTypes === "function")) return selectionSet === undefined ? "unknown" : selectionType(schema, type as GraphQLNamedType, selectionSet);
  return namedType(type);
};

const namedType = (type: GraphQLNamedType | GraphQLOutputType): string => {
  if (type instanceof GraphQLScalarType) return scalar(type.name);
  if (type instanceof GraphQLEnumType) return type.name;
  return "unknown";
};

const scalar = (name: string): string => {
  if (name === "Int" || name === "Float") return "number";
  if (name === "Boolean") return "boolean";
  return "string";
};

/** 🧱️ The type one selection set projects out of one type: an object, or a union when the selection
 * narrows the type with inline fragments the way `SectionItem` is narrowed. */
const selectionType = (schema: GraphQLSchema, owner: GraphQLNamedType, selectionSet: SelectionSetNode): string => {
  const shared = selectionSet.selections.filter((selection) => selection.kind === "Field");
  const narrowed = selectionSet.selections.filter((selection) => selection.kind === "InlineFragment");
  if (narrowed.length === 0) return objectType(schema, owner, shared);
  return narrowed
    .map((fragment) => {
      const condition = fragment.typeCondition === undefined ? owner : schema.getType(fragment.typeCondition.name.value);
      if (condition === undefined || condition === null) throw new Error(`unknown inline fragment type ${fragment.typeCondition?.name.value}`);
      return objectType(schema, condition, [...shared, ...fragment.selectionSet.selections.filter((selection) => selection.kind === "Field")]);
    })
    .join(" | ");
};

/** 🧱️ The inline object type a field list projects out of one object type. */
const objectType = (schema: GraphQLSchema, owner: GraphQLNamedType, fields: Array<{ name: { value: string }; alias?: { value: string }; selectionSet?: SelectionSetNode }>): string => {
  if (!(owner instanceof GraphQLObjectType)) return "unknown";
  const members = [`__typename?: '${owner.name}'`];
  for (const selection of fields) {
    const name = selection.name.value;
    if (name === "__typename") continue;
    const field = owner.getFields()[name];
    if (field === undefined) continue;
    const alias = selection.alias?.value ?? name;
    const optional = field.type instanceof GraphQLNonNull ? "" : "?";
    members.push(`${alias}${optional}: ${outputType(schema, field.type, selection.selectionSet)}`);
  }
  return `{ ${members.join(", ")} }`;
};

/** 🎛️ The `Exact<…>` variables type of one operation. */
const variablesType = (schema: GraphQLSchema, operation: OperationDefinitionNode): string => {
  const definitions = operation.variableDefinitions ?? [];
  if (definitions.length === 0) return "Exact<{ [key: string]: never; }>";
  const members = definitions.map((definition) => {
    const resolved = typeFromAST(schema, definition.type as never);
    if (resolved === undefined) throw new Error(`unknown variable type ${print(definition.type)}`);
    const required = definition.type.kind === "NonNullType";
    const inner = required ? namedInput(resolved) : `InputMaybe<${namedInput(resolved)}>`;
    return `  ${definition.variable.name.value}${required ? "" : "?"}: ${inner};`;
  });
  return `Exact<{\n${members.join("\n")}\n}>`;
};

const namedInput = (type: unknown): string => {
  if (type instanceof GraphQLNonNull) return namedInput(type.ofType);
  if (type instanceof GraphQLList) return `Array<${namedInput(type.ofType)}> | ${namedInput(type.ofType)}`;
  if (type instanceof GraphQLScalarType) return `Scalars['${type.name}']['input']`;
  if (type instanceof GraphQLEnumType) return type.name;
  return (type as GraphQLNamedType).name;
};

const source = readFileSync(extensionPath, "utf8");
const schema = buildSchema(readFileSync(schemaPath, "utf8"));
const operations = readOperations(source);
if (operations.length === 0) throw new Error("no operations found in the ⌛️Queries region");

const documents: string[] = [];
const overloads: string[] = [];
const constants: string[] = [];
const types: string[] = [];
for (const operation of operations) {
  const document = parse(operation.text);
  const errors = validate(schema, document);
  if (errors.length > 0) throw new Error(`${operation.name}: ${errors.map((error) => error.message).join("; ")}`);
  const definition = document.definitions[0] as OperationDefinitionNode;
  const queryType = schema.getQueryType();
  if (queryType === undefined || queryType === null) throw new Error("the schema declares no Query type");
  types.push(`export type ${operation.name}QueryVariables = ${variablesType(schema, definition)};`);
  types.push("");
  types.push("");
  types.push(`export type ${operation.name}Query = ${selectionType(schema, queryType, definition.selectionSet)};`);
  types.push("");
  const key = JSON.stringify(operation.text);
  constants.push(`const ${operation.name}DocumentNode = ${literal(document)} as unknown as DocumentNode<${operation.name}Query, ${operation.name}QueryVariables>;`);
  documents.push(`  ${key}: ${operation.name}DocumentNode,`);
  overloads.push(`/**\n * 🕸️The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.\n */\nexport function graphql(source: ${key}): (typeof documents)[${key}];`);
}

const gqlRegion = [
  "// #region 🧬️CodegenGql",
  "",
  "/** 📚️Every operation this extension declares, keyed by the exact source the `graphql` tag is called with. */",
  "const documents = {",
  ...documents,
  "} as const;",
  "",
  ...overloads,
  "",
  "/**",
  " * 🕸️The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.",
  " */",
  "export function graphql(source: string): unknown;",
  "export function graphql(source: string) {",
  "  return (documents as Record<string, unknown>)[source] ?? {};",
  "}",
  "",
  "export type DocumentType<TDocumentNode extends DocumentNode> = TDocumentNode extends DocumentNode<infer TType, unknown> ? TType : never;",
  "// #endregion 🧬️CodegenGql",
].join("\n");

console.log(`[build-vscode-codegen] ${operations.length} operations validated against the SDL`);
console.log(operations.map((operation) => `  ${operation.name}`).join("\n"));
writeFileSync(`${outputDir}/vscode-codegen-gql.ts`, `${gqlRegion}\n`, "utf8");
writeFileSync(`${outputDir}/vscode-codegen-documents.ts`, `${types.join("\n")}\n${constants.join("\n")}\n`, "utf8");
