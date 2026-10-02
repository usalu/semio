/** 🧾️ Norm wire twins witness: every committed `🦠️mutation` and `📸️snapshot` of the fifteen norm artifacts meets its
 * generated TypeScript twin exactly as it meets the strict Ajv oracle (`semioSchemaAjvV1`). An admitted wire decodes through
 * the twin and re-encodes byte for byte in the committed spelling (two-space JSON, Python float repr at `number` positions,
 * integers at `integer` positions), so the twin keeps every member, its order and its value; a refused one — the negative
 * witnesses — is refused by the twin too. Every admitted object wire also refuses an undeclared member and an unknown tag.
 * @see ../../📇️registry/🧬️contract/🟦️.ts
 * @see ../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-norm-ts-twins.ts */
import { describe, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type Ajv from "ajv";
import { semioSchemaAjvV1 } from "../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { NormWireRefusal, type NormWireReader } from "../../📇️registry/🧬️contract/🟦️.ts";

type Schema = Record<string, any>;
type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

const REPO = join(import.meta.dir, "../../../../..");
const ARTIFACTS = join(import.meta.dir, "../../🗿️artifacts");
const CATALOG = join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json");
const read = (path: string): Schema => JSON.parse(readFileSync(path, "utf8"));

//#region 🔤️Spelling
/** 🐍️ The committed float spelling: the shortest round-trip digits in Python `repr` layout (`40.0`, `5.135e-05`, `1e+16`). */
export const pythonFloat = (value: number): string => {
  if (!Number.isFinite(value)) throw new Error(`${value} has no JSON spelling`);
  if (value === 0) return Object.is(value, -0) ? "-0.0" : "0.0";
  const [mantissa, power] = value.toExponential().split("e") as [string, string];
  const exponent = Number(power);
  const digits = mantissa.replace("-", "").replace(".", "");
  const sign = value < 0 ? "-" : "";
  if (exponent < -4 || exponent >= 16) return `${sign}${digits[0]}${digits.length > 1 ? `.${digits.slice(1)}` : ""}e${exponent < 0 ? "-" : "+"}${String(Math.abs(exponent)).padStart(2, "0")}`;
  if (exponent < 0) return `${sign}0.${"0".repeat(-exponent - 1)}${digits}`;
  const whole = digits.slice(0, exponent + 1).padEnd(exponent + 1, "0");
  return `${sign}${whole}.${digits.slice(exponent + 1) || "0"}`;
};

/** 🧭️ Resolves `$ref`s across every loaded schema document by `$id` and local pointer. */
class Schemas {
  readonly byId = new Map<string, Schema>();
  add(schema: Schema): void {
    if (typeof schema.$id === "string") this.byId.set(schema.$id, schema);
  }
  resolve(node: Schema, base: Schema): { node: Schema; base: Schema } {
    let current = { node, base };
    while (typeof current.node.$ref === "string") {
      const [id, pointer] = (current.node.$ref as string).split("#") as [string, string | undefined];
      const document = id === "" ? current.base : this.byId.get(id);
      if (!document) throw new Error(`unresolved ${current.node.$ref}`);
      const target = (pointer ?? "").split("/").slice(1).reduce<Schema>((position, key) => position?.[key], document);
      if (!target) throw new Error(`unresolved ${current.node.$ref}`);
      current = { node: target, base: document };
    }
    return current;
  }
}

const kindOf = (value: Json): string => (value === null ? "null" : Array.isArray(value) ? "array" : typeof value === "number" ? "number" : typeof value);
const admits = (node: Schema, value: Json): boolean => {
  if (node.const !== undefined) return node.const === value;
  if (Array.isArray(node.enum)) return node.enum.includes(value);
  const types: string[] = Array.isArray(node.type) ? node.type : node.type ? [node.type] : [];
  const kind = kindOf(value);
  return types.length === 0 || types.some((type) => type === kind || (type === "integer" && kind === "number"));
};

/** ✍️️ The committed text of `value` at schema position `node`: `json.dumps(indent=2, ensure_ascii=False)` plus a newline. */
const encode = (schemas: Schemas, value: Json, node: Schema, base: Schema, indent = ""): string => {
  const resolved = schemas.resolve(node, base);
  const { node: position, base: owner } = resolved;
  const branches = (position.oneOf ?? position.anyOf) as Schema[] | undefined;
  if (branches) {
    const choices = branches.map((branch) => schemas.resolve(branch, owner));
    const tagged = value !== null && typeof value === "object" && !Array.isArray(value);
    const chosen =
      choices.find(({ node: branch }) => value === null && (branch.type === "null" || (Array.isArray(branch.type) && branch.type.includes("null")))) ??
      choices.find(({ node: branch }) => tagged && branch.properties && Object.keys(branch.properties).some((key) => branch.properties[key].const !== undefined && branch.properties[key].const === (value as Record<string, Json>)[key])) ??
      choices.find(({ node: branch }) => tagged && branch.required?.length === 1 && Object.keys(value as object).length === 1 && Object.hasOwn(value as object, branch.required[0])) ??
      choices.find(({ node: branch }) => admits(branch, value) && (!tagged || !branch.properties || Object.keys(value as object).every((key) => Object.hasOwn(branch.properties, key))));
    if (!chosen) throw new Error(`no branch admits ${JSON.stringify(value).slice(0, 80)}`);
    return encode(schemas, value, chosen.node, chosen.base, indent);
  }
  if (value === null || typeof value === "boolean" || typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number") {
    const types: string[] = Array.isArray(position.type) ? position.type : position.type ? [position.type] : [];
    return types.includes("integer") || (!types.includes("number") && Number.isInteger(value)) ? String(value) : pythonFloat(value);
  }
  const inner = `${indent}  `;
  if (Array.isArray(value)) return value.length === 0 ? "[]" : `[\n${value.map((item) => inner + encode(schemas, item, position.items ?? {}, owner, inner)).join(",\n")}\n${indent}]`;
  const entries = Object.entries(value);
  if (entries.length === 0) return "{}";
  const member = (key: string): Schema => position.properties?.[key] ?? (typeof position.additionalProperties === "object" ? position.additionalProperties : {});
  return `{\n${entries.map(([key, item]) => `${inner}${JSON.stringify(key)}: ${encode(schemas, item, member(key), owner, inner)}`).join(",\n")}\n${indent}}`;
};
//#endregion 🔤️Spelling

//#region 🗂️Corpus
interface Wire { readonly path: string; readonly text: string; readonly role: "mutation" | "snapshot" }
const bundles = (directory: string): string[] =>
  existsSync(join(directory, "🦠️mutation/🔣️.json")) ? [directory] : existsSync(directory) ? readdirSync(directory, { withFileTypes: true }).filter((entry) => entry.isDirectory()).flatMap((entry) => bundles(join(directory, entry.name))) : [];
const wires = (subset: string): Wire[] =>
  bundles(join(subset, "🧫️fixtures/🧬️mutations"))
    .sort()
    .flatMap((bundle) => [
      { path: join(bundle, "🦠️mutation/🔣️.json"), role: "mutation" as const },
      ...["⬅️before", "➡️after"].map((role) => ({ path: join(bundle, `📸️snapshot/${role}/🔣️.json`), role: "snapshot" as const })).filter((wire) => existsSync(wire.path)),
    ])
    .map((wire) => ({ ...wire, text: readFileSync(wire.path, "utf8") }));

const foreignSchemas = (schemas: Schemas, ajv: Ajv): void => {
  const scopes = Object.values(read(CATALOG).scopes as Record<string, { path: string; formats: Record<string, string> }>);
  const pending = (): string[] =>
    [...new Set([...schemas.byId.values()].flatMap((schema) => [...JSON.stringify(schema).matchAll(/"\$ref":"(https:[^"#]+)/gu)].map((match) => match[1]!)))].filter((id) => !schemas.byId.has(id));
  for (let missing = pending(); missing.length > 0; missing = pending()) {
    for (const id of missing) {
      const scope = scopes.find((candidate) => {
        const file = join(REPO, candidate.path, candidate.formats["🔣️jsonschema"] ?? "");
        return file.endsWith(".json") && existsSync(file) && read(file).$id === id;
      });
      if (!scope) throw new Error(`no catalogued schema owns ${id}`);
      const schema = read(join(REPO, scope.path, scope.formats["🔣️jsonschema"]!));
      schemas.add(schema);
      ajv.addSchema(schema);
    }
  }
};
//#endregion 🗂️Corpus

const splice = (wire: Json, external: boolean): Json => {
  const copy = structuredClone(wire) as Record<string, Json>;
  const target = external ? (Object.values(copy)[0] as Record<string, Json>) : copy;
  if (target !== null && typeof target === "object" && !Array.isArray(target)) target["x-undeclared"] = true;
  return copy;
};
const refuses = (read: NormWireReader<unknown>, wire: unknown): boolean => {
  try {
    read(wire);
    return false;
  } catch (error) {
    if (error instanceof NormWireRefusal) return true;
    throw error;
  }
};

const artifacts = readdirSync(ARTIFACTS, { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && existsSync(join(ARTIFACTS, entry.name, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json")))
  .map((entry) => entry.name)
  .sort();

interface Setup {
  readonly artifact: string;
  readonly subset: string;
  readonly schemas: Schemas;
  readonly twins: Readonly<Record<Wire["role"], NormWireReader<unknown>>>;
  readonly oracles: Readonly<Record<Wire["role"], (value: unknown) => unknown>>;
  readonly roots: Readonly<Record<Wire["role"], Schema>>;
  readonly external: boolean;
  readonly corpus: readonly Wire[];
}

/** 🧰️ One artifact's oracle, twins and committed corpus, built before any test registers. */
const setup = async (artifact: string): Promise<Setup> => {
  const subset = join(ARTIFACTS, artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any");
  const schema = join(subset, "🧬️schema");
  const aggregate = read(join(schema, "🧬️mutations/🔣️.json"));
  const snapshot = read(join(schema, "📸️snapshot/🔣️.json"));
  const prefix = (aggregate.title as string).replace(/Mutation$/u, "");
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
  const schemas = new Schemas();
  const documents = [snapshot, read(join(schema, "🔺️diff/🔣️.json")), read(join(schema, "🔣️.json"))];
  for (const leaf of readdirSync(join(schema, "🧬️mutations"), { withFileTypes: true }).filter((entry) => entry.isDirectory() && existsSync(join(schema, "🧬️mutations", entry.name, "🧬️schema/🔣️.json"))))
    documents.push(read(join(schema, "🧬️mutations", leaf.name, "🧬️schema/🔣️.json")));
  for (const document of [...documents, aggregate]) {
    schemas.add(document);
    ajv.addSchema(document);
  }
  foreignSchemas(schemas, ajv);
  return {
    artifact,
    subset,
    schemas,
    twins: {
      mutation: (await import(join(schema, "🧬️mutations/🟦️.ts")))[`parse${aggregate.title}`],
      snapshot: (await import(join(schema, "📸️snapshot/🟦️.ts")))[`parse${snapshot.title ?? `${prefix}Snapshot`}`],
    },
    oracles: { mutation: ajv.getSchema(aggregate.$id)!, snapshot: ajv.getSchema(snapshot.$id)! },
    roots: { mutation: aggregate, snapshot },
    external: (aggregate.oneOf as Schema[]).every((branch) => branch.properties && Object.keys(branch.properties).length === 1),
    corpus: wires(subset),
  };
};
const setups = await Promise.all(artifacts.map(setup));

describe("norm wire twins", () => {
  test("cover all fifteen artifacts", () => expect(setups).toHaveLength(15));
  for (const { artifact, subset, schemas, twins, oracles, roots, external, corpus } of setups) {
    describe(artifact, () => {
      test("commits a corpus with every role and a twin per role", () => {
        expect(corpus.some((wire) => wire.role === "mutation")).toBe(true);
        expect(corpus.some((wire) => wire.role === "snapshot")).toBe(true);
        expect(typeof twins.mutation).toBe("function");
        expect(typeof twins.snapshot).toBe("function");
      });
      for (const wire of corpus) {
        test(wire.path.slice(subset.length + 1), () => {
          const value = JSON.parse(wire.text) as Json;
          const admitted = oracles[wire.role](value) === true;
          expect(refuses(twins[wire.role], value)).toBe(!admitted);
          if (!admitted) return;
          const parsed = twins[wire.role](value) as Json;
          expect(`${encode(schemas, parsed, roots[wire.role], roots[wire.role])}\n`).toBe(wire.text);
          const broken = splice(value, wire.role === "mutation" && external);
          expect(oracles[wire.role](broken)).toBe(false);
          expect(refuses(twins[wire.role], broken)).toBe(true);
          if (wire.role === "mutation") {
            const unknown = external ? { NoSuchNormMutation: {} } : { ...(value as Record<string, Json>), mutation: "noSuchNormMutation" };
            expect(oracles.mutation(unknown)).toBe(false);
            expect(refuses(twins.mutation, unknown)).toBe(true);
          }
        });
      }
    });
  }
});
