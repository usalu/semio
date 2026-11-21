import assert from "node:assert/strict";
import { createReadStream, readFileSync } from "node:fs";
import { createInterface } from "node:readline";

const target = "@semio-tech/framework-os-dev:smoke-process3d-native-prototype";
const output = new URL("./🗑️generated/", import.meta.url);
const loadJson = path => JSON.parse(readFileSync(path, "utf8"));
const fixture = loadJson(new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/📨️command-dispatch/🧪️tests/🧫️scope.json", import.meta.url));
const expected = fixture.cases[0].providers.map(([, id]) => id).sort();

export async function contributionSmokeEvidence(lines, providers = expected) {
  const prefix = "[TRACE] setContributions native settled ";
  let complete;
  let booted = false;
  let selectedApp;
  for await (const line of lines) {
    const offset = line.indexOf(prefix);
    if (offset < 0) {
      if (complete && /"booted"\s*:\s*true/.test(line)) booted = true;
      const app = line.match(/"appId"\s*:\s*("[^"]+")/);
      if (complete && app) selectedApp = JSON.parse(app[1]);
      continue;
    }
    let value;
    try { value = JSON.parse(line.slice(offset + prefix.length)); }
    catch { continue; }
    if (value.plugin !== fixture.receiver || value.app !== fixture.app || value.encoding !== "pack" || !Number.isSafeInteger(value.instanceId) || value.instanceId <= 0 || !Array.isArray(value.providers)) continue;
    if (JSON.stringify([...value.providers].sort()) === JSON.stringify(providers)) complete = value;
  }
  assert(complete, "Process3d native host did not settle all four extension deliveries");
  assert(booted, "Native smoke did not report a booted shell after contribution delivery");
  assert.equal(selectedApp, complete.app, "The settled roster must belong to the booted editor");
  return complete;
}

if (import.meta.main) {
  const rows = readFileSync(new URL("supervised-native-smoke.jsonl", output), "utf8").trim().split("\n").map(JSON.parse);
  const started = rows.findLast(row => row.pid);
  const ended = rows.at(-1);
  assert(started?.args.includes(target), "Expected the canonical native Process3d smoke target");
  assert.equal(ended.code, 0, "The native Process3d smoke has not completed successfully");
  const input = createReadStream(new URL("supervised-native-smoke.log", output), { start: started.logOffset });
  const settled = await contributionSmokeEvidence(createInterface({ input, crlfDelay: Infinity }));
  console.log("[DEBUG] Native Process3d smoke settled the editor roster", settled);
}
