/** 🕸️ Starts the ticket's Chromium journey when the canonical Generation3d React route accepts connections. */
import { join } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { deadlineMs, runId } from "./🔍️watch-evidence.mjs";

const id = runId(process.env.SEMIO_PROBE_RUN_ID);
const started = Date.now();
const deadline = started + deadlineMs(process.env.SEMIO_PROBE_DEADLINE);
const evidence = join(import.meta.dir, "🗑️generated");
mkdirSync(evidence, { recursive: true });
writeFileSync(join(evidence, `canonical-react-${id}.started.json`), JSON.stringify({ id, started, deadline }), { flag: "wx" });

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
while (!await serves()) {
  if (Date.now() >= deadline) throw Error("Canonical route did not become ready before the deadline");
  await Bun.sleep(Math.min(5000, Math.max(1, deadline - Date.now())));
}
console.log("[DEBUG] Canonical React HTTP route is accepting requests");
const child = Bun.spawn([
  "repo", "tools", "exec", "--light", "bun", "--", join(import.meta.dir, "🔍️browser-probe.ts"),
  "--url=" + url, "--mode=interact", "--steps=example,hover,select,orbit", "--example=box-shell-preview",
  "--settle=180", "--label=canonical-react-" + id,
], { stdout: "inherit", stderr: "inherit" });
process.on("SIGINT", () => child.kill("SIGINT"));
let expired = false;
const timer = setTimeout(() => { expired = true; child.kill("SIGTERM"); setTimeout(() => child.kill("SIGKILL"), 5000).unref(); }, Math.max(1, deadline - Date.now()));
const code = await child.exited;
clearTimeout(timer);
process.exitCode = expired ? 1 : code;
