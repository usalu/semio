#!/usr/bin/env bun
/**
 * 🧬️ Regenerates the four facets of the mutation aggregate (`🔣️.json` oneOf, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`) from the payload schema
 * of every leaf directory. Idempotent; Wave M agents run it once after adding leaves instead of editing these files.
 */
import { readdirSync, readFileSync, rmSync, statSync, writeFileSync as writeRaw } from "node:fs";
const writeFileSync = (path: string, data: string) => {
  rmSync(path, { force: true });
  writeRaw(path, data);
};
import { join } from "node:path";
import { child, em, GQLF, JSONF, mutations, PROTOF, subset, TSF } from "./r3-f1-paths.ts";

const tagOf = (variant: string) => variant[0].toLowerCase() + variant.slice(1);
const NS = "https://json.schemas.assets.semio-tech.com/s/bim/model";
type Schema = { title: string; properties: Record<string, { type?: string; $ref?: string; const?: string; items?: { type?: string; $ref?: string } }>; required: string[] };
const leaves = readdirSync(mutations)
  .filter((name) => statSync(join(mutations, name)).isDirectory())
  .map((name) => ({ name, dir: join(mutations, name) }))
  .filter(({ dir }) => readdirSync(dir).some((n) => n === JSONF))
  .map(({ name, dir }) => ({
    kind: name.replace(/^[^a-z]+/, ""),
    descriptor: JSON.parse(readFileSync(join(dir, JSONF), "utf8")) as { binaryTag: number; aggregateVariant: string },
    schema: JSON.parse(readFileSync(join(dir, readdirSync(dir).find((n) => n.endsWith("schema"))!, JSONF), "utf8")) as Schema,
  }))
  .sort((a, b) => a.descriptor.binaryTag - b.descriptor.binaryTag);

const refName = (ref: string) => ref.split("/").pop()!;
const ts = (p: { type?: string; $ref?: string; items?: { type?: string; $ref?: string } }): string =>
  p.$ref ? refName(p.$ref) : p.type === "string" ? "string" : p.type === "boolean" ? "boolean" : p.type === "array" ? `${ts(p.items ?? {})}[]` : p.type === "object" ? "Record<string, unknown>" : "number";
const gql = (p: { type?: string; $ref?: string; items?: { type?: string; $ref?: string } }): string =>
  p.$ref ? refName(p.$ref) : p.type === "string" ? "String" : p.type === "boolean" ? "Boolean" : p.type === "integer" ? "Int" : p.type === "array" ? `[${gql(p.items ?? {})}!]` : p.type === "object" ? "JSON" : "Float";
const proto = (p: { type?: string; $ref?: string; items?: { type?: string; $ref?: string } }): string =>
  p.$ref ? refName(p.$ref) : p.type === "string" ? "string" : p.type === "boolean" ? "bool" : p.type === "integer" ? "int32" : p.type === "array" ? `repeated ${proto(p.items ?? {})}` : p.type === "object" ? "map<string, google.protobuf.Value>" : "double";
const unwrapped = (p: any) => (p.type === "object" ? p.properties?.value?.oneOf?.find((member: any) => member.$ref) ?? p : p);
const props = (s: Schema) => Object.entries(s.properties).filter(([name]) => name !== "mutation").map(([name, p]) => [name, unwrapped(p)] as [string, Schema["properties"][string]]);

const types = [...new Set(leaves.flatMap((l) => props(l.schema).map(([, p]) => (p.$ref ?? p.items?.$ref ? refName((p.$ref ?? p.items?.$ref)!) : null)).filter(Boolean) as string[]))].sort();

const usesObject = leaves.some((l) => props(l.schema).some(([, p]) => p.type === "object"));

writeFileSync(join(mutations, JSONF), JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", $id: `${NS}/mutations.json`, title: "ModelMutation", oneOf: leaves.map((l) => ({ $ref: `${NS}/mutation/${l.kind}/schema.json` })) }, null, 2) + "\n");
writeFileSync(
  join(mutations, TSF),
  `/** 🏙️ BIM model direct-mutation discriminated union. */\nimport type { ${types.join(", ")} } from "../${TSF}";\n\n${leaves.map((l) => `export interface ${l.descriptor.aggregateVariant} {\n  mutation: "${tagOf(l.descriptor.aggregateVariant)}";\n${props(l.schema).map(([n, p]) => `  ${n}${l.schema.required.includes(n) ? "" : "?"}: ${ts(p)};`).join("\n")}\n}\n`).join("\n")}\nexport type ModelMutation =\n${leaves.map((l) => `  | ${l.descriptor.aggregateVariant}`).join("\n")};\n`,
);
writeFileSync(
  join(mutations, GQLF),
  `# 🏙️ BIM model direct-mutation union.\n\n${usesObject ? "scalar JSON\n\n" : ""}${leaves.map((l) => `type ${l.descriptor.aggregateVariant} {\n${props(l.schema).map(([n, p]) => `  ${n}: ${gql(p)}${l.schema.required.includes(n) ? "!" : ""}`).join("\n")}\n}\n`).join("\n")}\nunion ModelMutation = ${leaves.map((l) => l.descriptor.aggregateVariant).join(" | ")}\n`,
);
writeFileSync(
  join(mutations, PROTOF),
  `syntax = "proto3";\npackage semio.s.bim.model.mutation;\n\n${usesObject ? 'import "google/protobuf/struct.proto";\n\n' : ""}// 🏙️ BIM model direct-mutation union; the wire tag of each member is its leaf \`binaryTag\`.\n\n${leaves.map((l) => `message ${l.descriptor.aggregateVariant} {\n${props(l.schema).map(([n, p], i) => `  ${l.schema.required.includes(n) || p.type === "array" ? "" : "optional "}${proto(p)} ${n} = ${i + 1};`).join("\n")}\n}\n`).join("\n")}\nmessage ModelMutation {\n  oneof mutation {\n${leaves.map((l) => `    ${l.descriptor.aggregateVariant} ${l.kind.replaceAll("-", "_")} = ${l.descriptor.binaryTag + 1};`).join("\n")}\n  }\n}\n`,
);
console.log(`mutation facets: ${leaves.length} leaves`);
void child;

//#region 🔖️WireProtocol
const io = child(subset, "io");
const binary = join(io, readdirSync(io).find((n) => n.endsWith("binary"))!);
const text = join(io, readdirSync(io).find((n) => n.endsWith("text"))!);
const facetDir = (parent: string) => join(parent, readdirSync(parent).find((n) => n.endsWith("mutations"))!);
const protocol = [
  "dialect protocol",
  "protocol bim.mutations",
  "version 1",
  "schema bim.model.mutations",
  "start record",
  "framing record",
  "",
  "# `ModelMutation` op frame (`dsl::tagged_value_binary`): `format u8` then `tag varint`, then the variant's `ToValue` tree without its",
  "# variant name. Each record is one mutation kind at its wire tag; this file is the only source of those tags (= the leaf `binaryTag`).",
  "header fixed 2",
  "field format u8",
  "field tag varint",
  ...leaves.flatMap((l) => [`record ${l.kind} tag=${l.descriptor.binaryTag}`, "field payload bytes"]),
  "",
].join("\n");
writeFileSync(join(facetDir(binary), em(0x1f4e1) + ".protocol.semio"), protocol);
const grammar = ["dialect grammar", "grammar bim.op", "extension bim", "start mutation", "", 'artifact-mark = "bim.model-op"', "mutation = artifact-mark model-op+", `model-op = ${leaves.map((l) => `"${l.kind}"`).join(" | ")} field+`, 'field = IDENT "=" TEXT', ""].join("\n");
writeFileSync(join(facetDir(text), em(0x1f5e3) + "mutations.grammar.semio"), grammar);
//#endregion 🔖️WireProtocol
