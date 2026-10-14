import { rustFindTopLevel, rustTokenPairs, rustTokens, type RustToken } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

/** 🧬️ Describes one actual defining Rust method and its mutable-slot contract. */
export interface LayoutMediaSourceDefinitionV1 {
  readonly implementation: string;
  readonly method: string;
  readonly parameters: string;
  readonly result: string;
}
/** 🎞️ Names the S-owned source adoption contract independently of trial data. */
export interface LayoutMediaAdoptionSourceV1 {
  readonly version: 1;
  readonly sources: Readonly<{ editor: "../../🦀️.rs"; export: "./🦀️.rs" }>;
  readonly adopter: LayoutMediaSourceDefinitionV1 & Readonly<{ body: string }>;
  readonly producer: Readonly<{ definition: LayoutMediaSourceDefinitionV1; phases: readonly Readonly<{ id: string; source: string }>[] }>;
}
type Definition = LayoutMediaSourceDefinitionV1;
export type LayoutMediaSourceInputsV1 = Readonly<Record<"editor" | "export", string>>;
type Method = Readonly<{ parameters: readonly RustToken[]; result: readonly RustToken[]; body: readonly RustToken[] }>;
const spelling = (tokens: readonly RustToken[]): string => JSON.stringify(tokens.map(token => [token.kind, token.text]));
const expected = (source: string): string => spelling(rustTokens(source));

/** 🦀️ Binds a unique direct method body to its exact physical root implementation. */
function definition(source: string, contract: Definition): Method | null {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), owners: [number, number][] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]!.text === "impl") {
      const open = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set(["{"])), close = pairs.get(open);
      if (open < 0 || close === undefined) return null;
      if (spelling(tokens.slice(index + 1, open)) === expected(contract.implementation)) owners.push([open + 1, close]);
      index = close;
    } else {
      const close = pairs.get(index);
      if (close !== undefined && close > index) index = close;
    }
  }
  if (owners.length !== 1) return null;
  const [start, end] = owners[0]!, methods: Method[] = [];
  for (let index = start; index < end; index++) {
    if (tokens[index]!.text === "fn" && tokens[index + 1]?.text === contract.method) {
      const open = index + 2, parametersEnd = pairs.get(open);
      if (tokens[open]?.text !== "(" || parametersEnd === undefined) return null;
      const body = rustFindTopLevel(tokens, pairs, parametersEnd + 1, end, new Set(["{", ";"])), close = pairs.get(body);
      if (body < 0 || tokens[body]?.text !== "{" || close === undefined || tokens[parametersEnd + 1]?.text !== "->") return null;
      methods.push({ parameters: tokens.slice(open + 1, parametersEnd), result: tokens.slice(parametersEnd + 2, body), body: tokens.slice(body + 1, close) });
      index = close;
    } else {
      const close = pairs.get(index);
      if (close !== undefined && close > index) index = close;
    }
  }
  return methods.length === 1 ? methods[0]! : null;
}

/** 🧩️ Compares authored tokens only within the proven defining method. */
export function inspectLayoutMediaAdoptionSourceV1(contract: LayoutMediaAdoptionSourceV1, sources: LayoutMediaSourceInputsV1): boolean {
  if (validateJsonSchemaSubset(schema, contract).length) return false;
  const adopter = definition(sources.editor, contract.adopter), producer = definition(sources.export, contract.producer.definition);
  if (!adopter || !producer) return false;
  for (const [method, signature] of [[adopter, contract.adopter], [producer, contract.producer.definition]] as const) {
    if (spelling(method.parameters) !== expected(signature.parameters) || spelling(method.result) !== expected(signature.result)) return false;
  }
  if (spelling(adopter.body) !== expected(contract.adopter.body)) return false;
  let frontier = 0;
  for (const phase of contract.producer.phases) {
    const wanted = rustTokens(phase.source), target = spelling(wanted);
    let found = -1;
    for (let index = frontier; index + wanted.length <= producer.body.length; index++) {
      if (spelling(producer.body.slice(index, index + wanted.length)) === target) { found = index; break; }
    }
    if (found < 0) return false;
    frontier = found + wanted.length;
  }
  return true;
}

