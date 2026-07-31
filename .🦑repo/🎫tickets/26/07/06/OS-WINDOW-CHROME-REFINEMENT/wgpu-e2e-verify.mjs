#!/usr/bin/env node
/** @emoji 🧪 End-to-end smoke checks for wgpu trunk + plugin boot. */
import { spawn, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dirname, "../../../../../..");
const port = process.env.S_OS_PORT ?? "6199";
const host = "127.0.0.1";
const baseUrl = `http://${host}:${port}/`;

function run(args, opts = {}) {
  const result = spawnSync("bun", args, { cwd: repoRoot, stdio: "inherit", ...opts });
  if (result.status !== 0) throw new Error(`bun ${args.join(" ")} failed`);
}

async function waitForHttp(url, attempts = 60) {
  for (let i = 0; i < attempts; i++) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(2000) });
      if (res.ok) return res;
    } catch {
      /* retry */
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  throw new Error(`timeout waiting for ${url}`);
}

console.log("[DEBUG] building lowpoly wasm plugin");
run(["framework/product/os/dev/📜script.ts", "plugin", "lowpoly"], {
  env: { ...process.env, SEMIO_PLUGIN: "lowpoly", SKIP_ENGINE_BUILD: "1" },
});

console.log("[DEBUG] building wgpu trunk renderer");
run(["framework/renderer/wgpu/📜script.ts", "wasm"], {
  env: { ...process.env, NO_COLOR: undefined, FORCE_COLOR: undefined },
});

const dist = join(repoRoot, "framework/product/os/dev/renderer-modules/wgpu");
const pluginJs = join(dist, "plugin-modules/lowpoly/lowpoly_plugin.js");
if (!existsSync(pluginJs)) throw new Error(`missing plugin artifact: ${pluginJs}`);
if (!existsSync(join(dist, "index.html"))) throw new Error("missing trunk index.html");

console.log(`[DEBUG] starting trunk serve on ${baseUrl}`);
const trunk = spawn("trunk", ["serve", "--config", "Trunk.toml", "--port", port], {
  cwd: join(repoRoot, "framework/renderer/wgpu"),
  stdio: "inherit",
  env: { ...process.env, NO_COLOR: undefined, FORCE_COLOR: undefined },
});

try {
  const indexRes = await waitForHttp(`${baseUrl}?plugin=lowpoly`);
  const indexHtml = await indexRes.text();
  if (!indexHtml.includes('id="root"')) throw new Error("index.html missing #root");
  if (!indexHtml.includes("semio-framework-renderer-wgpu")) {
    throw new Error("index.html missing wgpu renderer module");
  }

  const bootRes = await fetch(`${baseUrl}boot.js`, { signal: AbortSignal.timeout(5000) });
  if (!bootRes.ok) throw new Error(`boot.js fetch failed: ${bootRes.status}`);
  const bootSource = await bootRes.text();
  if (!bootSource.includes("lowpoly")) throw new Error("boot.ts missing lowpoly plugin target");

  const pluginRes = await fetch(`${baseUrl}plugin-modules/lowpoly/lowpoly_plugin.js`, {
    signal: AbortSignal.timeout(5000),
  });
  if (!pluginRes.ok) throw new Error(`lowpoly plugin js missing: ${pluginRes.status}`);

  console.log(`[DEBUG] wgpu e2e verify passed (${baseUrl}?plugin=lowpoly)`);
} finally {
  trunk.kill("SIGTERM");
}
