/** 🏘️ H13: creates <n> spaces back to back on one hub and prints every non-202 answer (status + body) — the two-client e2e's
 * directory-page step failed with an empty body on the sqlite backend. usage: bun h13-space-burst.ts <origin> <n> */
import { hubProbeCreateSpace, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const [origin = "http://127.0.0.1:8010", countText = "60"] = process.argv.slice(2);
const token = await hubProbeSignIn(origin, process.env.OS_HUB_PROBE_EMAIL ?? "", process.env.OS_HUB_PROBE_PASSWORD ?? "", "h13-burst-");
let failures = 0;
const started = Date.now();
for (let index = 0; index < Number(countText); index += 1) {
  const at = Date.now();
  try {
    await hubProbeCreateSpace(origin, token, `Burst ${Date.now()} ${index}`);
  } catch (error) {
    failures += 1;
    console.log(`[space-burst] ${index} after ${Date.now() - at} ms: ${String(error).slice(0, 300)}`);
  }
}
console.log(`[space-burst] ${countText} spaces, ${failures} refused, ${Date.now() - started} ms`);
