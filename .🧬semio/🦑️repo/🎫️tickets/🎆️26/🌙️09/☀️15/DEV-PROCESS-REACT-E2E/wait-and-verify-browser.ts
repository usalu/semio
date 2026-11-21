import { setTimeout as delay } from "node:timers/promises";

const renderer = process.argv[3] ?? "react";
if (renderer !== "react" && renderer !== "wgpu") throw Error("Unknown Process3d renderer");
const target = new URL(process.argv[2] ?? (renderer === "wgpu" ? "http://127.0.0.1:6122/" : "http://127.0.0.1:6022/"));
console.log(`[DEBUG] Waiting for the Process3d ${renderer} editor server`);
const serverResponds = async () => {
  try {
    const response = await fetch(target, { signal: AbortSignal.timeout(3000) });
    await response.body?.cancel();
    return response.ok;
  } catch {
    return false;
  }
};
while (!(await serverResponds())) await delay(10000);
console.log(`[DEBUG] Process3d ${renderer} editor server responded. Running the current browser probe.`);
if (renderer === "wgpu") await import("./verify-wgpu-placement.ts");
else await import("./verify-browser.ts");
