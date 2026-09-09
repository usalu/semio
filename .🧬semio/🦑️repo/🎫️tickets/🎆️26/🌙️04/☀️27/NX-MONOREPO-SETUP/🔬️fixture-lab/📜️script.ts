import { createInterface } from "node:readline";
import { spawn, spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";

const probes: Record<string, string> = { "generator-receipt": "../🔬️generator-input-receipt/📜️script.ts", "source-inputs": "../🔬️source-inputs/📜️script.ts", "wgpu-boot": "../🔬️wgpu-boot-inputs/📜️script.ts" };
let child: ReturnType<typeof spawn> | undefined;
const stop = () => {
  if (!child?.pid) return;
  if (process.platform === "win32") spawnSync("taskkill", ["/pid", String(child.pid), "/t", "/f"], { stdio: "ignore" });
  else try { process.kill(-child.pid, "SIGTERM"); } catch {}
};
process.once("SIGINT", stop); process.once("SIGTERM", stop);
console.log("[DEBUG] Nx fixture lab ready; select a registered probe or exit");
const input = createInterface({ input: process.stdin });
for await (const line of input) {
  const name = line.trim(); if (name === "exit") break;
  if (!probes[name]) { console.log("[DEBUG] Unknown probe"); continue; }
  const started = Date.now(); let output = "";
  child = spawn(process.execPath, [resolve(import.meta.dir, probes[name])], { cwd: process.cwd(), detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
  child.stdout!.on("data", chunk => output += chunk.toString()); child.stderr!.on("data", chunk => output += chunk.toString());
  const progress = setInterval(() => console.log(`[DEBUG] ${name} running ${(Date.now() - started) / 1000}s`), 10000);
  const status = await new Promise<number | null>((accept, reject) => { child!.once("close", accept); child!.once("error", reject); });
  clearInterval(progress); child = undefined;
  writeFileSync(resolve(import.meta.dir, `../🗑️generated/lab-${name}-${started}.log`), output);
  console.log(output.slice(-2200)); console.log(`[DEBUG] ${name} exit=${status}, elapsed=${Date.now() - started}ms; fixture lab ready`);
}
input.close(); process.stdin.pause();
process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop);
