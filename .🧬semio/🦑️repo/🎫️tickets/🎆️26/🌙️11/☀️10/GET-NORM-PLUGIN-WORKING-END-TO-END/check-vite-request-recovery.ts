#!/usr/bin/env repo
import assert from "node:assert/strict";
import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const workspace = resolve(import.meta.dirname, "../../../../../../..");
const framework = join(workspace, "🧰️framework");
const { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } = await import(join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const { scriptCompilerAdmissionControlV1 } = await import(join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"));
const { serveVite } = await import(join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts"));

class RequestRecovery extends BundleScript {
  async run() {
    const root = await mkdtemp(join(import.meta.dirname, "🗑️generated/vite-recovery-"));
    const config = join(root, "vite.config.ts");
    const rejected = join(root, "rejected.txt");
    await Promise.all([
      writeFile(config, `import { writeFileSync } from "node:fs";
        export default { logLevel: "silent", optimizeDeps: { noDiscovery: true },
        plugins: [{ name: "reject-fixture", load(id) {
          if (id.split("?")[0].endsWith("/invalid.ts")) {
            writeFileSync(${JSON.stringify(rejected)}, id);
            throw new Error("Fixture load rejected");
          }
        } }] };`),
      writeFile(join(root, "index.html"), "<!doctype html><script type=module src=/ready.ts></script>"),
      writeFile(join(root, "ready.ts"), 'export const value: string = "Vite remains ready";'),
      writeFile(join(root, "invalid.ts"), "export {};"),
    ]);
    const controller = new AbortController();
    let announce!: (url: string) => void;
    const ready = new Promise<string>(resolve => { announce = resolve; });
    const server = serveVite({
      control: scriptCompilerAdmissionControlV1(this.invocation, "test:vite-request-recovery", 0),
      invocation: this.invocation, root, config, configLoader: "native",
      host: "127.0.0.1", port: 0, signal: controller.signal, ready: announce,
    }).catch(error => {
      if (!controller.signal.aborted || error?.name !== "AbortError") throw error;
    });
    try {
      const origin = await Promise.race([ready, server.then(() => { throw new Error("Vite stopped before readiness"); })]);
      const cases = ["/@fs/%E0%A4%A.ts", "/invalid.ts"];
      for (const pathname of cases) {
        const bad = await fetch(new URL(pathname, origin), { signal: AbortSignal.timeout(60_000) });
        const detail = await bad.text();
        assert.ok(bad.status >= 400, detail.slice(0, 300));
        if (pathname === "/invalid.ts") assert.match(await readFile(rejected, "utf8"), /\/invalid\.ts$/u);
        const valid = await fetch(new URL("/ready.ts", origin), { signal: AbortSignal.timeout(60_000) });
        assert.equal(valid.status, 200);
        assert.match(await valid.text(), /Vite remains ready/);
        console.log("[DEBUG] Vite request recovery", bad.status, valid.status);
      }
    } finally {
      controller.abort();
      await server;
    }
  }
}
const router = new ScriptRouter(import.meta.dirname).register("verify", RequestRecovery);
await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(router, { invocation }));
