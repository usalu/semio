/** 🧊 Stages the native PDF and Stdio modules as soon as their signed products exist. */
import assert from "node:assert/strict";
import { closeSync, openSync } from "node:fs";
import { join } from "node:path";

const profile = ".cache/b/p/m/browser/prototype/components";
const components = [
  { path:join(profile,"94d8ca0d8ba9c1594ac4ea4c"), plugin:"stdio-pdf" },
  { path:join(profile,"ff2dca26f35d2cadeb93c740"), plugin:"stdio" },
];
const deadline = Date.now() + 75 * 60 * 1000;
const published = async ({path,plugin}: typeof components[number]) => {
  const stamp = Bun.file(join(path,".semio-artifact.json"));
  const receipt = Bun.file(join(path,"🧾️receipt.json"));
  if (!await stamp.exists() || !await receipt.exists()) return false;
  const result = await receipt.json();
  assert.equal(result.pluginId,plugin);
  return /^[a-f0-9]{64}$/u.test(result.wasmSha256) && result.wasmBytes > 0;
};
while (!(await Promise.all(components.map(published))).every(Boolean)) {
  assert(Date.now() < deadline, "The signed main Stdio model did not publish");
  await Bun.sleep(4000);
}

const generated = join(import.meta.dir,"🗑️generated","native-wgpu-save");
const log = openSync(join(generated,"wgpu-modules-current-receipts.log"),"wx");
console.log("[DEBUG] Both native component receipts are present. Starting the official module publisher.");
const child = Bun.spawn(["repo","tools","exec","bun","--",join(import.meta.dir,"📜️native-wgpu-run.ts"),"modules"],{
  stdout:log,stderr:log,env:{...process.env,CARGO_NET_OFFLINE:"true"},
});
closeSync(log);
process.once("SIGINT",()=>child.kill("SIGINT"));
process.once("SIGTERM",()=>child.kill("SIGINT"));
process.exitCode=await child.exited;
console.log("[DEBUG] Official native module publisher completed",process.exitCode);
