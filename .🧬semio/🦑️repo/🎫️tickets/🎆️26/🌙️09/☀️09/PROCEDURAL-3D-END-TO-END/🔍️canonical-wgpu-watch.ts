/** 🌐️ Starts the retained WGPU Chromium journey when the canonical route serves its page. */
import { join } from "node:path";

const url = "http://127.0.0.1:6118/?plugin=generation3d";
async function serves() {
  try {
    const response = await fetch(url, { signal: AbortSignal.timeout(2000) });
    const html = response.headers.get("content-type")?.includes("text/html");
    await response.body?.cancel();
    return response.ok && html;
  } catch {
    return false;
  }
}
while (!await serves()) await Bun.sleep(5000);
console.log("[DEBUG] Canonical WGPU HTTP route is accepting requests");
const child = Bun.spawn([
  "repo", "tools", "exec", "--light", "bun", "--", join(import.meta.dir, "🐍️wgpu-journey-probe.mjs"),
], {
  stdout: "inherit",
  stderr: "inherit",
  env: { ...process.env, SEMIO_PROBE_URL: url, SEMIO_PROBE_OUT: "wgpu-journey/canonical-live", SEMIO_PROBE_BOOT: "300" },
});
process.on("SIGINT", () => child.kill("SIGINT"));
process.exitCode = await child.exited;
