#!/usr/bin/env bun
/** 🌱️ H14 session 15 one-off: server-owned creation on a hub whose Space creation catalog may still be refused (t6 before row
 * 34 lands). The generation comes from `/trusted-catalog/plugin-modules`; the creatable kinds are derived from the published
 * bundle by the ONE creation rule (owner tier = targets bound to the package's own codec row, else the hosted tier; most
 * general dialect within the deciding tier — `🚪️io/🧫️fixtures/🌳️most-general-dialect`); each kind is POSTed to `ready`
 * (latency, parent dialect) and its editor open plan read (the package that executes it: the HOST for a hosted kind). Kinds the
 * rule calls ambiguous must be refused by the hub. Credentials only from env (`OS_HUB_PROBE_EMAIL` / `OS_HUB_PROBE_PASSWORD`).
 *   bun h14-t6-creation.ts <hub origin> <catalog root> [kind regex=.] [concurrency=1] */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeOpenPlan, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";

const [origin = "http://127.0.0.1:7800", catalogRoot = "", filter = ".", concurrencyText = "1"] = process.argv.slice(2);
const email = process.env.OS_HUB_PROBE_EMAIL ?? "";
const password = process.env.OS_HUB_PROBE_PASSWORD ?? "";
if (!email || !password || !catalogRoot) throw new Error("usage: OS_HUB_PROBE_EMAIL / OS_HUB_PROBE_PASSWORD + <origin> <catalog root>");
const log = (line: string): void => console.log(`[t6-creation] ${new Date().toISOString().slice(11, 23)} ${line}`);

type Dialect = Readonly<{ artifactKind: string; standard: string; subset: string }>;
type Target = Readonly<{ pluginId: string; owned: boolean; appId: string; parentDialect: Dialect; artifactSchema: string }>;
const covers = (general: Dialect, subset: Dialect): boolean => general.subset === "*" && subset.subset !== "*" && general.artifactKind === subset.artifactKind && general.standard === subset.standard;
const mostGeneral = (candidates: readonly Target[]): Target | "ambiguous" | "none" => {
  const uncovered = candidates.filter((candidate) => !candidates.some((other) => covers(other.parentDialect, candidate.parentDialect)));
  return uncovered.length === 0 ? "none" : uncovered.length === 1 ? uncovered[0]! : "ambiguous";
};

const current = JSON.parse(readFileSync(join(catalogRoot, "trusted-catalog", "current.json"), "utf8"));
const bundle = JSON.parse(readFileSync(join(catalogRoot, "trusted-catalog", "generations", current.generationId, "trusted-catalog.json"), "utf8"));
const profile = bundle.profiles.find((entry: any) => entry.id === current.profileId);
const packages = new Map<string, any>(bundle.packages.map((entry: any) => [entry.pluginId, entry]));
const editors = new Map<string, Target[]>();
for (const selected of profile.openTargets) {
  const target = selected.target;
  if (target.role !== "editor") continue;
  const record = packages.get(selected.package.pluginId);
  const owned = record.nativeCodecs.some((codec: any) => codec.artifactKind === target.artifactKind && codec.artifactSchema === target.artifactSchema && codec.packSchemaHash === target.packSchemaHash);
  editors.set(target.artifactKind, [...(editors.get(target.artifactKind) ?? []), { pluginId: selected.package.pluginId, owned, appId: target.appId, parentDialect: target.parentDialect, artifactSchema: target.artifactSchema }]);
}
const decisions = [...editors.entries()].sort(([left], [right]) => (left < right ? -1 : 1)).filter(([kind]) => new RegExp(filter, "u").test(kind)).map(([kind, candidates]) => {
  const owned = candidates.filter((candidate) => candidate.owned);
  return { kind, candidates, hosted: owned.length === 0, choice: mostGeneral(owned.length > 0 ? owned : candidates) };
});
log(`bundle ${current.generationId.slice(0, 12)} profile ${current.profileId.slice(0, 40)}… kinds-with-editors ${editors.size} selected ${decisions.length} creatable ${decisions.filter((entry) => typeof entry.choice !== "string").length} ambiguous ${decisions.filter((entry) => entry.choice === "ambiguous").map((entry) => entry.kind).join(",")}`);

const token = await hubProbeSignIn(origin, email, password, "h14-t6-creation-");
const spaceId = await hubProbeCreateSpace(origin, token, `H14 t6 creation ${Date.now()}`);
const index = await hubProbeCall(origin, "GET", "/trusted-catalog/plugin-modules", token);
const generationId = String(index.json?.generationId ?? "");
const catalog = await hubProbeCall(origin, "GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
log(`space ${spaceId} generation ${generationId.slice(0, 12)} (bundle ${generationId === current.generationId ? "=" : "≠"}) creation-catalog GET ${catalog.status} kinds ${catalog.json?.kinds?.length ?? "-"} withheld ${catalog.json?.withheld?.length ?? "-"} ${catalog.status === 200 ? "" : catalog.text.slice(0, 160)}`);

const results: string[] = [];
const run = async (entry: (typeof decisions)[number]): Promise<void> => {
  const started = Date.now();
  if (typeof entry.choice === "string") {
    const refused = await hubProbeCall(origin, "POST", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token, JSON.stringify({ schema: "semio.hub.space-artifact-create/v1", requestId: crypto.randomUUID().replaceAll("-", ""), expectedCatalogGenerationId: generationId, kindId: entry.kind, name: "H14 refused" }));
    results.push(`${entry.kind}\t${entry.choice}\tPOST ${refused.status}\t${refused.status === 202 ? "UNEXPECTED-ACCEPT" : "refused-as-expected"}\t${entry.candidates.map((candidate) => candidate.appId).join(" ")}`);
    return;
  }
  try {
    const created = await hubProbeCreateArtifact(origin, token, spaceId, generationId, entry.kind, `H14 ${entry.kind}`, 900_000);
    const plan = await hubProbeOpenPlan(origin, token, spaceId, created.artifactId, `h14-t6-${entry.kind}`);
    const surface = plan.json?.surface?.surfaceId ?? `plan ${plan.status}`;
    const executes = plan.json?.package?.pluginId ?? "-";
    results.push(`${entry.kind}\tcreated\t${created.ms} ms\t${entry.hosted ? "hosted" : "owned"} by ${entry.choice.pluginId}\texpected ${entry.choice.appId}\topened ${surface}\texecutes ${executes}\t${surface === entry.choice.appId && executes === entry.choice.pluginId ? "OK" : "MISMATCH"}`);
  } catch (error) {
    results.push(`${entry.kind}\tFAILED\t${Date.now() - started} ms\t${entry.hosted ? "hosted" : "owned"} by ${entry.choice.pluginId}\t${String(error).slice(0, 240)}`);
  }
  log(results.at(-1)!);
};
const queue = [...decisions];
await Promise.all(Array.from({ length: Math.max(1, Number(concurrencyText)) }, async () => {
  for (let entry = queue.shift(); entry !== undefined; entry = queue.shift()) await run(entry);
}));
console.log(["kind\toutcome\tlatency\ttier\texpected\topened\texecutes\tverdict", ...results.sort()].join("\n"));
const failed = results.filter((row) => row.includes("FAILED") || row.includes("MISMATCH") || row.includes("UNEXPECTED")).length;
log(`done ${results.length} kinds, ${failed} failed/mismatched`);
process.exit(failed === 0 ? 0 : 1);
