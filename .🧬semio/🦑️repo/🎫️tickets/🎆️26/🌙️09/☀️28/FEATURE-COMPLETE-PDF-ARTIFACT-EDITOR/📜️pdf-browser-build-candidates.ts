/** 🔁️ Re-admits the real Browser build when a shared-worktree source receipt has expired. */
import { openSync, closeSync } from "node:fs";
import { join } from "node:path";

const phases = process.argv.slice(2);
if (!phases.length || phases.some(phase => !["descriptorEmitter","components-pdf","components-stdio","session","assets"].includes(phase))) throw Error("Choose a Browser producer phase");
const runner = join(import.meta.dir,"📜️pdf-browser-direct.ts");
const run = process.env.SEMIO_PDF_BROWSER_RUN ?? "joined";
if (!/^[a-z0-9-]+$/u.test(run)) throw Error("Invalid Browser build log name");
let child: ReturnType<typeof Bun.spawn> | undefined;
let interrupted = false;
const cancel = () => {
  interrupted = true;
  child?.kill("SIGINT");
};
process.once("SIGINT", cancel);
process.once("SIGTERM", cancel);
try {
  for (const phase of phases) {
    for (let attempt = 1; ; attempt++) {
      if (interrupted) process.exit(130);
      const path = join(import.meta.dir,"🗑️generated","pdf-browser-"+run+"-"+phase+"-"+attempt+".log");
      const descriptor = openSync(path,"wx");
      console.log("[DEBUG]", phase, "admitting attempt", attempt, path);
      try {
        child = Bun.spawn(["repo","tools","exec","bun","--",runner,phase], {
          env: {...process.env,CARGO_NET_OFFLINE:"false"}, stdout:descriptor,stderr:descriptor,
        });
        const status = await child.exited;
        console.log("[DEBUG]", phase, "finished with", status);
        if (status === 0) break;
        if (status !== 75 || attempt === 6) process.exitCode = status;
      } finally {
        child = undefined;
        closeSync(descriptor);
      }
      if (process.exitCode || interrupted) break;
    }
    if (process.exitCode || interrupted) break;
  }
  if (interrupted) process.exitCode = 130;
} finally {
  process.removeListener("SIGINT",cancel);
  process.removeListener("SIGTERM",cancel);
}
