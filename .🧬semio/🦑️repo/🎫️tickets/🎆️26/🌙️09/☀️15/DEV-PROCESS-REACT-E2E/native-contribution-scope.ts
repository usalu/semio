import assert from "node:assert/strict";
import Ajv from "ajv";
import { contributionSmokeEvidence } from "./verify-native-smoke.mjs";
import { consumedTopicsForApp, scopeContributionsJson } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🧾️contributions-json/🟦️.ts";

const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const runtime = os + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules";
const fixture = await Bun.file(os + "/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/📨️command-dispatch/🧪️tests/🧫️scope.json").json() as {
  receiver: string;
  app: string;
  cases: { name: string; consumes: string[]; providers: [string, string][] }[];
};
const schema = await Bun.file(runtime + "/🧬️schema/🔣️.json").json();
const { readNativeRuntimeManifestV1 } = await import(process.cwd() + "/" + runtime + "/📋️contract/🟦️.ts");
const validate = new Ajv({ strict: false }).compile(schema);
const smokeLog = process.argv[2] ? await Bun.file(process.argv[2]).text() : undefined;

for (const row of fixture.cases) {
  const loaded = await Promise.all(row.providers.map(async ([folder, pluginId]) => {
    const { manifest } = await Bun.file(`✏️s/🔌️plugins/🏭️process/🧩️extensions/${folder}/🧩️component/🔣️.json`).json();
    assert.equal(manifest.pluginId, pluginId);
    return { pluginId, manifest };
  }));
  const inventory = {
    version: 1,
    variant: "process3d",
    profile: "prototype",
    modules: [fixture.receiver, ...loaded.map(entry => entry.pluginId)].map(pluginId => ({
      pluginId,
      wasmSha256: "0".repeat(64),
      wasmPath: pluginId + "/📦️component.wasm",
      descriptorPath: pluginId + "/🔣️.json",
      consumes: pluginId === fixture.receiver ? row.consumes : [],
    })),
  };
  assert.ok(validate(inventory), JSON.stringify(validate.errors));
  const parsed = readNativeRuntimeManifestV1(inventory, "process3d", "prototype");
  assert.deepEqual(parsed, inventory);
  for (const invalid of [undefined, ["process.machines", "process.machines"], [""], ["x".repeat(257)]]) {
    const bad = { ...inventory, modules: [{ ...inventory.modules[0], consumes: invalid }] };
    assert.equal(validate(bad), false);
    assert.throws(() => readNativeRuntimeManifestV1(bad, "process3d", "prototype"));
  }
  const unrelated = { pluginId: "foreign", manifest: { topicContributions: [{ topic: "flow.extension", payload: {} }] } };
  const own = { pluginId: fixture.receiver, manifest: { topicContributions: [{ topic: "process.self", payload: {} }] } };
  const roster = [...loaded, own, unrelated];
  const consumed = consumedTopicsForApp(parsed.modules, loaded, fixture.receiver, fixture.app);
  const entries = JSON.parse(scopeContributionsJson(roster, fixture.receiver, consumed));
  assert.deepEqual(entries.map(entry => entry.pluginId), [...row.providers.map(([, id]) => id), fixture.receiver]);
  const enabled = roster.filter(entry => entry.pluginId !== loaded[0].pluginId);
  assert.deepEqual(JSON.parse(scopeContributionsJson(enabled, fixture.receiver, consumed)), entries.slice(1));
  const hosted = { pluginId: "app-owner", consumes: ["flow.extension"] };
  const appScoped = consumedTopicsForApp([...parsed.modules, hosted], [...loaded, { pluginId: hosted.pluginId, manifest: { apps: [{ id: fixture.app }] } }], fixture.receiver, fixture.app);
  assert.deepEqual(JSON.parse(scopeContributionsJson(roster, fixture.receiver, appScoped)), entries.slice(-1));
  assert.ok(Buffer.byteLength(JSON.stringify(entries)) < 49_152);
  if (smokeLog !== undefined) await contributionSmokeEvidence(smokeLog.split("\n"));
  console.log(`[DEBUG] ${row.name} providers ${entries.slice(0, -1).map(entry => entry.pluginId).join(", ")}`);
}
