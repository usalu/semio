/** 🕸️ Starts the ticket's Chromium journey when the canonical Generation3d React route accepts connections. */
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
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
console.log("[DEBUG] Canonical React HTTP route is accepting requests");
const child = Bun.spawn([
  "repo", "tools", "exec", "--light", "bun", "--", join(import.meta.dir, "🔍️browser-probe.ts"),
  "--url=" + url, "--mode=interact", "--steps=example,hover,select,orbit", "--example=box-shell-preview",
  "--settle=180", "--label=canonical-react-live",
], { stdout: "inherit", stderr: "inherit" });
process.on("SIGINT", () => child.kill("SIGINT"));
process.exitCode = await child.exited;
