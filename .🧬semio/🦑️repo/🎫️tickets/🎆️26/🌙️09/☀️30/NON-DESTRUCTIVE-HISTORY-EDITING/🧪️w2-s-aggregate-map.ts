#!/usr/bin/env bun
/**
 * 🗺️ W2-S aggregate map: every `#[derive(Mutations)]` aggregate under the given roots with its wire layout (read by the lint's
 * own `rustMutationAggregates`), its aggregate schema document, and each variant's wire name and leaf (descriptor
 * `aggregateVariant`, nearest leaf directory) plus the leaf's wrapper enum when the variant's payload type is a wrapped leaf
 * (`#[mutation_leaf(payload = <Variant>)]` in the leaf's `🦀️.rs` or `🦠️mutation/🦀️.rs`, found exactly as the lint's `wrappersOf`).
 * Input for `🧪️w2-s-aggregate-rule.py`.
 *
 *   bun 🧪️w2-s-aggregate-map.ts <out.json> <root>…
 */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { mutationVariantWireName, rustMutationAggregates, rustValueEnums } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const [out, ...roots] = process.argv.slice(2);
const aggregates: { path: string; name: string; tag: string | null; content: string | null; renameAll: string | null; variants: [string, string, string | null][] }[] = [];
const leaves: { directory: string; variant: string; schema: string; wrappers: string[] }[] = [];
const wrappersOf = (leaf: string): string[] =>
  [`${leaf}/🦀️.rs`, `${leaf}/🦠️mutation/🦀️.rs`].flatMap((path) => {
    const source = existsSync(join(repo, path)) ? readFileSync(join(repo, path), "utf8") : "";
    return source.includes("mutation_leaf") ? rustValueEnums(path, source).filter((candidate) => candidate.payloadVariant !== null).map((candidate) => candidate.name) : [];
  });
const walk = (directory: string): void => {
  const entries = readdirSync(join(repo, directory), { withFileTypes: true });
  const name = directory.slice(directory.lastIndexOf("/") + 1);
  if (name === "🧬️mutations" && entries.some((entry) => entry.name === "🦀️.rs"))
    for (const aggregate of rustMutationAggregates(`${directory}/🦀️.rs`, readFileSync(join(repo, directory, "🦀️.rs"), "utf8")))
      aggregates.push({ path: aggregate.path, name: aggregate.name, ...aggregate.layout, variants: [...aggregate.variants].map(([variant, rename]) => [variant, mutationVariantWireName(variant, rename, aggregate.layout.renameAll), aggregate.payloadTypes.get(variant) ?? null]) });
  const segments = directory.split("/");
  const fixtures = segments.findLastIndex((segment) => segment.endsWith("fixtures"));
  if (entries.some((entry) => entry.name === "🔣️.json") && segments.slice(fixtures + 1, -1).includes("🧬️mutations")) {
    try {
      const descriptor = JSON.parse(readFileSync(join(repo, directory, "🔣️.json"), "utf8"));
      if (typeof descriptor.aggregateVariant === "string") leaves.push({ directory, variant: descriptor.aggregateVariant, schema: `${directory}/${descriptor.payloadSchema}`, wrappers: wrappersOf(directory) });
    } catch {}
  }
  for (const entry of entries) if (entry.isDirectory() && !["node_modules", "target", "🗑️generated"].includes(entry.name)) walk(`${directory}/${entry.name}`);
};
for (const root of roots) walk(root);
const shared = (left: string, right: string): number => {
  const [a, b] = [left.split("/"), right.split("/")];
  let count = 0;
  while (count < a.length && a[count] === b[count]) count += 1;
  return count;
};
const map = aggregates.map((aggregate) => ({
  ...aggregate,
  schema: `${aggregate.path.slice(0, aggregate.path.lastIndexOf("/"))}/🔣️.json`,
  variants: aggregate.variants.map(([variant, wire, payloadType]) => {
    const candidates = leaves.filter((leaf) => leaf.variant === variant);
    const best = Math.max(0, ...candidates.map((leaf) => shared(leaf.directory, aggregate.path)));
    const nearest = candidates.filter((leaf) => shared(leaf.directory, aggregate.path) === best);
    const wrapper = nearest.length === 1 && payloadType !== null && nearest[0]!.wrappers.includes(payloadType) ? payloadType : null;
    return { variant, wire, wrapper, leaves: nearest.map((leaf) => leaf.schema) };
  }),
}));
writeFileSync(out!, JSON.stringify(map, null, 1));
console.log(`[w2-s] ${map.length} aggregates, ${map.reduce((sum, aggregate) => sum + aggregate.variants.length, 0)} variants, ${map.flatMap((aggregate) => aggregate.variants).filter((variant) => variant.leaves.length !== 1).length} without exactly one leaf`);
