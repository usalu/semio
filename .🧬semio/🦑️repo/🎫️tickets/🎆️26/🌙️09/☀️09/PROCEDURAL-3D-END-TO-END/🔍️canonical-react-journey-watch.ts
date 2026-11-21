/** 🧭️ Runs the canonical multi-role Chromium journey after the first live boot and interaction probe. */
import {readdir} from "node:fs/promises";
import {join} from "node:path";

const evidence=join(import.meta.dir,"🗑️generated");
async function firstProbeFinished() {
  const windows=(await readdir(evidence)).filter(name=>name.startsWith("probe-canonical-react-live-"));
  for (const run of windows) {
    if (await Bun.file(join(evidence,run,"summary.json")).exists()) return true;
  }
  return false;
}
while(!await firstProbeFinished()) await Bun.sleep(5000);
const url="http://127.0.0.1:6018/?plugin=generation3d";
const response=await fetch(url,{signal:AbortSignal.timeout(5000)});
await response.body?.cancel();
if(!response.ok) throw Error("The canonical React route closed before the multi-role journey");
console.log("[DEBUG] The first live probe finished. Checking all authored examples across the canonical roles.");
const child=Bun.spawn(["repo","tools","exec","--light","bun","--",join(import.meta.dir,"🐍️journey-probe.mjs")],
  {env:{...process.env,SEMIO_PROBE_URL:url,SEMIO_PROBE_OUT:"canonical-react-journey"},stdout:"inherit",stderr:"inherit"});
process.on("SIGINT",()=>child.kill("SIGINT"));
process.exitCode=await child.exited;
