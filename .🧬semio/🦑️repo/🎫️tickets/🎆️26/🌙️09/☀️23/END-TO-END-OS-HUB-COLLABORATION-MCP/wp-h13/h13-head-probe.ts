/** 🔎️ H13: reopens one document and prints its welcome frontier (head ordinal / edit id). usage: bun h13-head-probe.ts <origin> <space> <document> */
import { hubProbeOpenDocument, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const [origin, spaceId, documentId] = process.argv.slice(2);
const token = await hubProbeSignIn(origin!, process.env.OS_HUB_PROBE_EMAIL ?? "", process.env.OS_HUB_PROBE_PASSWORD ?? "", "h13-head-");
const started = Date.now();
const document = await hubProbeOpenDocument(origin!, token, spaceId!, documentId!, "h13-head");
const text = JSON.stringify(document.welcome);
console.log(`[head-probe] welcomed in ${Date.now() - started} ms: ${text.slice(0, 700)}`);
document.close();
