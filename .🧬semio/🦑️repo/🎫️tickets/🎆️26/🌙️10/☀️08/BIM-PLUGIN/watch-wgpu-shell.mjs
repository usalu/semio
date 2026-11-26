import { spawn } from "node:child_process";
import { resolve } from "node:path";
import { setTimeout } from "node:timers/promises";

const ticket = import.meta.dirname;
const servers = [["managed", "http://127.0.0.1:6402/"], ["captured", "http://127.0.0.1:16402/"]];
const resources = [
  "/renderer-modules/wgpu/semio-renderer-wgpu-component_bg.wasm", "/renderer-modules/wgpu/semio-renderer-wgpu-component.js",
  "/🔌️plugin-modules/🏬️bim/🌉️bridge.js", "/🔌️plugin-modules/🏬️bim/🛂️.descriptor.semio",
];
const attempted = new Set();
async function ready(address) {
  try {
    const page = await fetch(address, { signal: AbortSignal.timeout(2500) });
    if (!page.ok || !(await page.text()).includes("semio-native-catalog")) return false;
    const available = await Promise.all(resources.map(file => fetch(new URL(file, address), { method: "HEAD", signal: AbortSignal.timeout(2500) })));
    return available.every(file => file.ok && Number(file.headers.get("content-length")) > 8);
  } catch { return false; }
}
async function check(kind, address, language, role) {
  console.log("[DEBUG] Checking the", kind, role, language, "shell");
  const result = await new Promise((done, reject) => {
    const proc = spawn(process.execPath, [resolve(ticket, "bim-wgpu-browser.mjs"), "shell", language, role], {
      cwd: process.cwd(),
      stdio: "inherit",
      env: { ...process.env, SEMIO_WGPU_URL: address },
    });
    proc.once("error", reject);
    proc.once("exit", done);
  });
  return result === 0;
}
while (attempted.size < servers.length) {
  for (const [kind, address] of servers) {
    if (attempted.has(kind) || !(await ready(address))) continue;
    attempted.add(kind);
    const editor = await check(kind, address, "en", "editor");
    const viewer = await check(kind, address, "de", "viewer");
    console.log(JSON.stringify({ kind, address, editor, viewer }));
    if (editor && viewer) process.exit(0);
  }
  await setTimeout(5000);
}
process.exit(1);
