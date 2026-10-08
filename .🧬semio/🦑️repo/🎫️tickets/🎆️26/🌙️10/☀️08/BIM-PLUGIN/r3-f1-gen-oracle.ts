#!/usr/bin/env bun
/** 🔮️ Registers the `s.bim.model@1` mutation vocabulary in the subset oracle manifest: derives catalog rows and manifest rows from the leaves on disk. `bun r3-f1-gen-oracle.ts` rewrites the two arrays in place. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../..");
const subset = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any");
const mutations = join(subset, "🧬️schema/🧬️mutations");
const fixtures = join(subset, "🧫️fixtures/🧬️mutations");
const manifestPath = join(subset, "🔮️oracles/🔣️.json");

const source = readFileSync(join(mutations, "🦀️.rs"), "utf8");
const order = [...source.slice(source.indexOf("pub const KINDS")).matchAll(/"([a-z-]+)",/g)].map((m) => m[1]);
const dirOf = (kind: string) => readdirSync(mutations).find((name) => name.endsWith(kind) && name.length > kind.length && !name.includes("."))!;
const variantOf = (kind: string) => kind.split("-").map((w) => w[0].toUpperCase() + w.slice(1)).join("");
const caseId = (dir: string) => dir.replace(/^[^a-z]+/u, "");

const vectors = order.map((kind) => {
  const dir = dirOf(kind);
  const cases = readdirSync(join(fixtures, dir)).sort();
  return { mutationId: kind, sourceMutationDirectoryName: dir, mutationDirectoryName: dir, scenarios: cases.map((directoryName) => ({ id: caseId(directoryName), directoryName })) };
});

const outcomesOf = (dir: string) => {
  const found = new Set<string>();
  for (const name of readdirSync(join(fixtures, dir))) found.add(JSON.parse(readFileSync(join(fixtures, dir, name, "🎯️outcome/🔣️.json"), "utf8")).status);
  return ["applied", "rejected"].filter((status) => found.has(status));
};

const catalogs = [{ id: "bim-1-any", capability: "bim-1-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", vectors, kinds: order }];
const manifests = [
  {
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact: "s.bim.model",
    standard: "1",
    subset: "any",
    standardDirectoryName: "🔖️1",
    subsetDirectoryName: "✳️any",
    mutations: order.map((kind) => ({
      id: kind,
      capability: "bim-1-mutate",
      payloadSchema: "🧬️schema/🔣️.json",
      outcomes: outcomesOf(dirOf(kind)),
      productionDispatch: { operation: kind, bridgeVersion: 1, variant: variantOf(kind) },
      oracleRequirements: [{ capability: "bim-1-mutate", qualifyingKind: "third-party-library" }],
    })),
  },
];

const before = readFileSync(manifestPath, "utf8");
const document = JSON.parse(before);
const next = JSON.stringify({ ...document, mutationCatalogs: catalogs, mutationManifests: manifests }, null, 2) + "\n";
if (next !== before) writeFileSync(manifestPath, next);
console.log(`registered ${order.length} kinds, ${vectors.reduce((n, v) => n + v.scenarios.length, 0)} scenarios${next === before ? " (already current)" : ""}`);
