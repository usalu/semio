/** 🧊 Re-admits the native renderer after a concurrent shared source generation changes. */
import { closeSync, openSync } from "node:fs";
import { join } from "node:path";

const generated = join(import.meta.dir,"🗑️generated","native-wgpu-save");
const runner = join(import.meta.dir,"📜️native-wgpu-run.ts");
let child: ReturnType<typeof Bun.spawn> | undefined;
let interrupted = false;
const cancel = () => { interrupted=true; child?.kill("SIGINT"); };
process.once("SIGINT",cancel);
process.once("SIGTERM",cancel);

async function phase(name: string, attempt: number): Promise<number> {
  const log = openSync(join(generated,`wgpu-host-retry-${name}-${attempt}.log`),"wx");
  try {
    child = Bun.spawn(["repo","tools","exec","bun","--",runner,name], {
      env:{...process.env,CARGO_NET_OFFLINE:"true"},stdout:log,stderr:log,
    });
    return await child.exited;
  } finally {
    closeSync(log);
    child=undefined;
  }
}

for (let attempt=1; attempt<=6 && !interrupted; attempt++) {
  console.log("[DEBUG] Admitting native renderer build",attempt);
  const status = await phase("host",attempt);
  console.log("[DEBUG] Renderer build result",status);
  if (interrupted) break;
  if (status === 0) {
    process.exitCode = await phase("host-receipt",attempt);
    break;
  }
  if (status !== 75 || attempt === 6) {
    process.exitCode=status;
    break;
  }
}
if (interrupted) process.exitCode=130;
