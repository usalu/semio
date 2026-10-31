import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { developmentRuntimeRoot } from "@semio-tech/framework-os-dev/activation";

const root = "/Users/ueli/Documents/semio";
const gate = await import(join(root, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts"));
const assets = await import(join(root, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts"));
const generated = join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
const plugins = JSON.parse(readFileSync(join(generated, "🔌️plugins.json"), "utf8"));
const playgrounds = JSON.parse(readFileSync(join(generated, "🎠️playgrounds.json"), "utf8"));

const wanted: string[] = assets.demonstratorRuntimeComponentIds();
const lanes: string[] = gate.demonstratorActivationLanes();
const sessionClosure = (variant: string): string[] => {
  const row = playgrounds.find((entry: { variant: string; aliases?: string[] }) => entry.variant === variant || (entry.aliases ?? []).includes(variant));
  const target = row?.pluginId ?? variant;
  return plugins.some((entry: { pluginId: string }) => entry.pluginId === target) ? runtimeComponentClosure(plugins, [{ id: target, appScoped: row?.app !== undefined }]) : [];
};
const perLane = Object.fromEntries(lanes.map(lane => [lane, sessionClosure(lane)]));
const union = [...new Set(Object.values(perLane).flat())].sort();
const missing = wanted.filter(id => !union.includes(id)), extra = union.filter(id => !wanted.includes(id));
const receiptPaths = lanes.map(lane => ({ lane, reader: gate.demonstratorActivationLaneReceiptDirectory(root, lane), writer: join(developmentRuntimeRoot(root, lane, "prototype", "react"), "activation") }));
const pathMismatch = receiptPaths.filter(row => row.reader !== row.writer);
const unknownLaneVariants = lanes.filter(lane => !playgrounds.some((entry: { variant: string }) => entry.variant === lane));
const bundleRow = plugins.find((entry: { pluginId: string }) => entry.pluginId === "demonstrator");
const report = { lanes, wantedCount: wanted.length, unionCount: union.length, missing, extra, pathMismatch, unknownLaneVariants, perLaneCounts: Object.fromEntries(Object.entries(perLane).map(([lane, ids]) => [lane, ids.length])), demonstratorConsumes: bundleRow?.consumes, demonstratorDependsOn: bundleRow?.dependsOn };
console.log(JSON.stringify(report, null, 1));
process.exit(missing.length + extra.length + pathMismatch.length + unknownLaneVariants.length === 0 ? 0 : 1);
