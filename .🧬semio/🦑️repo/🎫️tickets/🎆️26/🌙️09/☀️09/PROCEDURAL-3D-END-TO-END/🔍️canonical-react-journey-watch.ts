/** 🧭️ Runs the canonical multi-role Chromium journey after the first live boot and interaction probe. */
import {readdir} from "node:fs/promises";
import {join} from "node:path";
import {deadlineMs, reactEvidence, runId} from "./🔍️watch-evidence.mjs";

const id=runId(process.env.SEMIO_PROBE_RUN_ID);
const deadline=Date.now()+deadlineMs(process.env.SEMIO_PROBE_DEADLINE);

const evidence=join(import.meta.dir,"🗑️generated");
const marker=Bun.file(join(evidence,`canonical-react-${id}.started.json`));
async function firstProbeFinished() {
  if (!await marker.exists()) return false;
  const started=await marker.json();
  if (started.id!==id || !Number.isFinite(started.started) || !Number.isFinite(started.deadline) || Date.now()>started.deadline) throw Error("Expected React run marker is invalid or expired");
  const windows=(await readdir(evidence)).filter(name=>name.startsWith(`probe-canonical-react-${id}-`));
  for (const run of windows) {
    const summaryFile=Bun.file(join(evidence,run,"summary.json"));
    if (!await summaryFile.exists()) continue;
    if (summaryFile.lastModified < started.started) continue;
    const summary=await summaryFile.json();
    const hoverFile=Bun.file(join(evidence,run,"bidirectional-hover.json"));
    const hover=await hoverFile.exists()?await hoverFile.json():null;
    if (!reactEvidence(id,run,summary,hover,started,summaryFile.lastModified,Date.now())) throw Error("Expected React probe completed without successful fresh hover evidence");
    return true;
  }
  return false;
}
while(!await firstProbeFinished()) {
  if(Date.now()>=deadline) throw Error("Expected React probe did not finish before the deadline");
  await Bun.sleep(Math.min(5000,Math.max(1,deadline-Date.now())));
}
const url="http://127.0.0.1:6018/?plugin=generation3d";
const response=await fetch(url,{signal:AbortSignal.timeout(5000)});
await response.body?.cancel();
if(!response.ok) throw Error("The canonical React route closed before the multi-role journey");
console.log("[DEBUG] The first live probe finished. Checking all authored examples across the canonical roles.");
const child=Bun.spawn(["repo","tools","exec","--light","bun","--",join(import.meta.dir,"🐍️journey-probe.mjs")],
  {env:{...process.env,SEMIO_PROBE_URL:url,SEMIO_PROBE_OUT:"canonical-react-journey-"+id},stdout:"inherit",stderr:"inherit"});
process.on("SIGINT",()=>child.kill("SIGINT"));
let expired=false;
const timer=setTimeout(()=>{expired=true;child.kill("SIGTERM");setTimeout(()=>child.kill("SIGKILL"),5000).unref();},Math.max(1,deadline-Date.now()));
const code=await child.exited;clearTimeout(timer);process.exitCode=expired?1:code;
