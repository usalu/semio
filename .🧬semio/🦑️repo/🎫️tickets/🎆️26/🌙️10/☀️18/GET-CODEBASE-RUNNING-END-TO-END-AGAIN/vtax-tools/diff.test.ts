// Differential harness: the pre-split validateTaxonomy (orig/taxonomy-authority.abs.ts) against the decomposed one over the real taxonomy and deterministic mutated variants.
// env: VTAX_SHARD="i/n" (default 0/1), VTAX_OUT=<dir> for the JSON summary. Each variant runs both implementations on separate deep clones.
import { expect, test } from "bun:test";
import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { validateTaxonomy as validateNew } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { readFileSync } from "node:fs";
import { validateTaxonomy as validateOld } from "./orig/taxonomy-authority.abs.ts";

type Path = readonly (string | number)[];
type Variant = Readonly<{ id: string; path: Path; op: string; custom?: (root: Record<string, any>) => void }>;
const hash = (text: string): number => { let h = 2166136261; for (let i = 0; i < text.length; i++) { h ^= text.charCodeAt(i); h = Math.imul(h, 16777619); } return h >>> 0; };
const baseText = JSON.stringify(JSON.parse(readFileSync(process.env.VTAX_BASE ?? "/Users/ueli/.cache/vtax/taxonomy.base.json", "utf8")));
const base = JSON.parse(baseText) as Record<string, unknown>;
const at = (root: unknown, path: Path): unknown => path.reduce<unknown>((value, key) => (value as Record<string | number, unknown>)[key], root);

const opsFor = (value: unknown): string[] => {
  if (typeof value === "string") return ["delete", "null", "number", "append", "empty-string"];
  if (typeof value === "number") return ["delete", "null", "string", "increment"];
  if (typeof value === "boolean") return ["delete", "null", "flip", "string"];
  if (Array.isArray(value)) return ["delete", "null", "empty-array", "object", "reverse", "duplicate-first", "drop-first", "string"];
  if (value && typeof value === "object") return ["delete", "null", "empty-object", "array", "extra-key", "drop-first-key", "string"];
  return ["delete"];
};
const apply = (root: Record<string, unknown>, path: Path, op: string): void => {
  const parent = at(root, path.slice(0, -1)) as Record<string | number, unknown>, key = path[path.length - 1]!, value = parent[key];
  const set = (next: unknown): void => { parent[key] = next; };
  if (op === "delete") { if (Array.isArray(parent)) parent.splice(key as number, 1); else delete parent[key]; return; }
  switch (op) {
    case "null": return set(null);
    case "number": return set(0);
    case "string": return set("mutated");
    case "append": return set(`${value as string}~`);
    case "empty-string": return set("");
    case "increment": return set((value as number) + 1);
    case "flip": return set(!(value as boolean));
    case "empty-array": return set([]);
    case "object": return set({});
    case "reverse": return set([...(value as unknown[])].reverse());
    case "duplicate-first": return set([...(value as unknown[]), (value as unknown[])[0]]);
    case "drop-first": return set((value as unknown[]).slice(1));
    case "empty-object": return set({});
    case "array": return set([]);
    case "extra-key": return set({ ...(value as object), __extra__: 1 });
    case "bad-regex": return set("(");
    case "bad-glob": return set("[");
    case "drop-first-key": { const { [Object.keys(value as object)[0]!]: _dropped, ...rest } = value as Record<string, unknown>; return set(rest); }
    default: throw new Error(`unknown op ${op}`);
  }
};


const firstKeys = (value: unknown, count: number): string[] => Object.keys((value ?? {}) as object).slice(0, count);
function targeted(): Variant[] {
  const t = base as Record<string, any>, out: Variant[] = [];
  const add = (id: string, custom: (root: Record<string, any>) => void): void => { out.push({ id: `targeted/${id}`, path: [], op: "custom", custom }); };
  const template = { kind: "canonical-or-planned-package", projectionContractId: "nested-cargo-packages-v1", packageId: "jcoprobe-guest", sourceManifestPath: "a/Cargo.toml", destinationManifestPath: "b/Cargo.toml" };
  for (const id of ["jco-package-adapter", "wgpu-frame-worker", ...firstKeys(t.generatorContracts, 3)]) {
    add(`projection-activation/${id}/template`, (r) => { r.generatorContracts[id].projectionActivation = { ...template, packageId: id === "wgpu-frame-worker" ? "wgpu-renderer" : "jcoprobe-guest" }; });
    add(`projection-activation/${id}/kind`, (r) => { r.generatorContracts[id].projectionActivation = { ...template, kind: "other" }; });
    add(`projection-activation/${id}/extra`, (r) => { r.generatorContracts[id].projectionActivation = { ...template, extra: 1 }; });
    add(`projection-activation/${id}/paths`, (r) => { r.generatorContracts[id].projectionActivation = { ...template, sourceManifestPath: "/abs", destinationManifestPath: "../up" }; });
  }
  for (const id of firstKeys(t.fixedFilenameContracts, 6)) {
    add(`scope/${id}/directory-set`, (r) => { r.fixedFilenameContracts[id].scope = { kind: "fixed-directory-contract-set", fixedDirectoryContractIds: ["x"] }; });
    add(`scope/${id}/directory-set-empty`, (r) => { r.fixedFilenameContracts[id].scope = { kind: "fixed-directory-contract-set", fixedDirectoryContractIds: [] }; });
    add(`scope/${id}/named-set`, (r) => { r.fixedFilenameContracts[id].scope = { kind: "named-fixed-directory-contract-set", setId: "x", fixedDirectoryContractIds: ["x"] }; });
    add(`scope/${id}/bogus`, (r) => { r.fixedFilenameContracts[id].scope = { kind: "bogus" }; });
    add(`scope/${id}/repository-root-path`, (r) => { r.fixedFilenameContracts[id].scope = { kind: "repository-root", path: "a/*" }; });
  }
  for (const id of firstKeys(t.fixedDirectoryContracts, 4)) {
    add(`directory-scope/${id}/directory-set`, (r) => { r.fixedDirectoryContracts[id].scope = { kind: "fixed-directory-contract-set", fixedDirectoryContractIds: ["x"] }; });
    add(`directory-scope/${id}/bogus`, (r) => { r.fixedDirectoryContracts[id].scope = { kind: "bogus" }; });
  }
  for (const id of firstKeys(t.semanticPathProjectionReferenceConsumerContracts, 3)) {
    add(`reference-consumer/${id}/bad-regex`, (r) => { r.semanticPathProjectionReferenceConsumerContracts[id].sourcePathPattern = "("; });
    add(`reference-consumer/${id}/empty-identities`, (r) => { r.semanticPathProjectionReferenceConsumerContracts[id].sourcePathIdentities = []; });
  }
  const projected = firstKeys(t.semanticProjectedMemberKinds, 3);
  if (projected.length >= 2) {
    add("projected/cycle-two", (r) => { r.semanticProjectedMemberKinds[projected[0]!].ownerKindIds = [projected[1]!]; r.semanticProjectedMemberKinds[projected[1]!].ownerKindIds = [projected[0]!]; });
    add("projected/cycle-self", (r) => { r.semanticProjectedMemberKinds[projected[0]!].ownerKindIds = [projected[0]!]; });
  }
  const descendants = firstKeys(t.semanticDescendantContracts, 6);
  for (const source of descendants) {
    add(`descendant/draw-copy-of-${source}`, (r) => { r.semanticDescendantContracts["draw-editor-command-bundle-v1"] = JSON.parse(JSON.stringify(r.semanticDescendantContracts[source])); });
    add(`descendant/draw-copy-of-${source}-drop-node`, (r) => { const copy = JSON.parse(JSON.stringify(r.semanticDescendantContracts[source])); if (Array.isArray(copy.requiredNodes)) copy.requiredNodes.pop(); r.semanticDescendantContracts["draw-editor-command-bundle-v1"] = copy; });
  }
  const regexKeys: [string, string][] = [];
  const walk = (value: unknown, path: (string | number)[]): void => {
    if (!value || typeof value !== "object") return;
    for (const [key, child] of Object.entries(value)) {
      const next = [...path, Array.isArray(value) ? Number(key) : key];
      if (typeof child === "string" && /pattern/i.test(String(key)) && hash(next.join("\0")) % 5 === 0) regexKeys.push([next.join("\u0001"), key]);
      if (next.length < 6) walk(child, next);
    }
  };
  walk(t, []);
  for (const [joined] of regexKeys) {
    const path = joined.split("\u0001").map((part) => (/^\d+$/.test(part) ? Number(part) : part));
    out.push({ id: `targeted/bad-regex/${path.join("/")}`, path, op: "bad-regex" });
    out.push({ id: `targeted/bad-glob/${path.join("/")}`, path, op: "bad-glob" });
  }
  return out;
}

const sample = (depth: number): number => (depth <= 1 ? 1 : depth === 2 ? 6 : depth === 3 ? 36 : depth === 4 ? 30 : 8);
function variants(): Variant[] {
  const out: Variant[] = [{ id: "baseline", path: [], op: "none" }];
  const walk = (value: unknown, path: Path): void => {
    if (path.length > 0) {
      const text = path.join("\0"), depth = path.length;
      if (hash(text) % sample(depth) === 0) {
        const ops = opsFor(value), picks = depth <= 1 ? ops : [ops[hash(`${text}a`) % ops.length]!, ops[hash(`${text}b`) % ops.length]!, "delete"];
        for (const op of new Set(picks)) out.push({ id: `${path.join("/")}#${op}`, path, op });
      }
    }
    if (value && typeof value === "object") for (const [key, child] of Object.entries(value)) walk(child, [...path, Array.isArray(value) ? Number(key) : key]);
  };
  walk(base, []);
  return [...out, ...targeted()];
}
type Outcome = Readonly<{ ok: true; problems: readonly string[] } | { ok: false; error: string }>;
const run = (validate: (taxonomy: never) => string[], variant: Variant): Outcome => {
  const clone = JSON.parse(baseText) as Record<string, unknown>;
  if (variant.custom) variant.custom(clone);
  else if (variant.path.length > 0) apply(clone, variant.path, variant.op);
  try { return { ok: true, problems: validate(clone as never) }; } catch (error) { return { ok: false, error: `${(error as Error).name}: ${(error as Error).message}` }; }
};

const [shardIndex, shardCount] = (process.env.VTAX_SHARD ?? "0/1").split("/").map(Number) as [number, number];
const all = variants();
const mine = all.filter((variant, index) => index % shardCount === shardIndex && (!process.env.VTAX_ONLY || new RegExp(process.env.VTAX_ONLY).test(variant.id)));

test(`validateTaxonomy old and new agree on shard ${shardIndex}/${shardCount} (${mine.length} of ${all.length} variants)`, () => {
  const mismatches: unknown[] = [];
  let changed = 0, thrown = 0, diagnostics = 0, baselineProblems = 0;
  const messages = new Map<string, number>(), full = new Set<string>();
  const started = performance.now();
  for (const variant of mine) {
    const left = run(validateOld as never, variant), right = run(validateNew as never, variant);
    if (JSON.stringify(left) !== JSON.stringify(right)) mismatches.push({ variant, left, right });
    if (variant.id === "baseline") baselineProblems = left.ok ? left.problems.length : -1;
    if (!left.ok) thrown++;
    else if (left.problems.length > 0) { changed++; diagnostics += left.problems.length; for (const message of left.problems) { messages.set(message.replace(/"[^"]*"/g, '"…"').slice(0, 90), (messages.get(message.replace(/"[^"]*"/g, '"…"').slice(0, 90)) ?? 0) + 1); full.add(message); } }
  }
  const summary = { shard: `${shardIndex}/${shardCount}`, variants: mine.length, total: all.length, baselineProblems, variantsWithDiagnostics: changed, variantsThrowing: thrown, diagnostics, distinctMessages: messages.size, mismatches: mismatches.length, seconds: Math.round((performance.now() - started) / 100) / 10 };
  if (process.env.VTAX_OUT) { mkdirSync(process.env.VTAX_OUT, { recursive: true }); writeFileSync(join(resolve(process.env.VTAX_OUT), `vtax-shard-${shardIndex}.json`), JSON.stringify({ summary, mismatches: mismatches.slice(0, 20), messages: [...messages], fullMessages: [...full] }, null, 1)); }
  console.log(JSON.stringify(summary));
  expect(mismatches.slice(0, 3)).toEqual([]);
}, 3_000_000);
