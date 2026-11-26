import { createConnection } from "node:net";
import { startAdmissionBrokerV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import { FleetPlacementFailedErrorV1, ParentComponentMemoryV1, runPassesInJobSlotsV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts";

const mb = 1024 ** 2, allowance = 8 * mb;
const request = { v: 1, op: "admit", pass: "test", pid: process.pid, crate: "semio_hub_probe_component", package: "semio-hub-probe-component", version: "0.1.0", target: "wasm32-wasip2", crateTypes: ["cdylib"], outDir: "/synthetic", extraFilename: "-probe", source: "/nonexistent/probe.rs", manifestDir: "/nonexistent", primary: true, buildScriptOutDir: null, externs: [] };
const exchange = (endpoint: string, message: object) => new Promise<Record<string, unknown>>((resolve, reject) => {
  const socket = createConnection(endpoint); let input = "";
  const timer = setTimeout(() => { socket.destroy(); reject(Error("broker timed out")); }, 5000);
  socket.on("error", error => { clearTimeout(timer); reject(error); });
  socket.on("connect", () => socket.write(JSON.stringify(message) + "\n"));
  socket.on("data", chunk => { input += chunk; const end = input.indexOf("\n"); if (end < 0) return; clearTimeout(timer); socket.destroy(); resolve(JSON.parse(input.slice(0, end))); });
});

const broker = await startAdmissionBrokerV1({ physicalBytes: 16 * 1024 * mb, parentMemoryAllowanceBytes: allowance, totalBytes: 16 * 1024 * mb, available: () => 15 * 1024 * mb, warden: () => "calm", cancels: () => [], history: new Map() });
try {
  broker.registerPass({ id: "test", profile: "prototype", target: "wasm32-wasip2" });
  const probe = await exchange(broker.endpoint, { ...request, op: "probe" });
  const result = await exchange(broker.endpoint, request);
  if (probe.wait !== true || result.op !== "refused" || result.reason !== "parent-memory" || Number(result.predictedBytes) <= allowance || result.allowanceBytes !== allowance) throw Error(`Unexpected broker decisions ${JSON.stringify({ probe, result })}`);
  if (broker.stats().running !== 0) throw Error("Oversize compiler was admitted");
  console.log("[DEBUG] broker", JSON.stringify({ probe, result }));
  broker.registerPass({ id: "remote", profile: "prototype", target: "wasm32-wasip2", placement: "fleet-only" });
  const fleetProbe = await exchange(broker.endpoint, { ...request, pass: "remote", op: "probe" });
  const fleetRefusal = await exchange(broker.endpoint, { ...request, pass: "remote" });
  if (fleetProbe.wait !== true || fleetRefusal.reason !== "fleet-placement" || !broker.placementFailed("remote") || broker.stats().running !== 0) throw Error(`Remote-only local attempt escaped: ${JSON.stringify({ fleetProbe, fleetRefusal })}`);
  console.log("[DEBUG] fleet broker", JSON.stringify({ fleetProbe, fleetRefusal }));
  const row = { id: "oversized", target: "wasm32-wasip2" }, local = { maximum: 1, floorMegabytes: 0, peak: () => 32, sample: async () => ({ availableMegabytes: 1024, ownedMegabytes: 0, pressure: "normal" as const }) };
  const starts: string[] = [];
  let refusal: unknown;
  try { await runPassesInJobSlotsV1([row], { jobs: 1, parentMemory: new ParentComponentMemoryV1(8), local, check: () => {}, run: async (_, slot) => { starts.push(slot); } }); } catch (error) { refusal = error; }
  if (starts.length || (refusal as { requiredMegabytes?: number; allowanceMegabytes?: number })?.requiredMegabytes !== 32 || (refusal as { allowanceMegabytes?: number })?.allowanceMegabytes !== 8) throw Error(`Unexpected local decision ${JSON.stringify({ starts, refusal })}`);
  const fleet = { maximum: 1, slots: async () => 1, footprintMegabytes: 1, floorMegabytes: 0, sample: local.sample };
  await runPassesInJobSlotsV1([row], { jobs: 2, parentMemory: new ParentComponentMemoryV1(8), local, fleet, check: () => {}, run: async (_, slot) => { starts.push(slot); } });
  if (starts.length !== 1 || starts[0] !== "fleet") throw Error(`Oversized pass did not use only fleet: ${starts}`);
  let fallback: unknown; const fallbackStarts: string[] = [];
  try { await runPassesInJobSlotsV1([row], { jobs: 2, parentMemory: new ParentComponentMemoryV1(8), local, fleet, check: () => {}, run: async (_, slot) => { fallbackStarts.push(slot); if (slot === "fleet") throw new FleetPlacementFailedErrorV1("synthetic unavailable worker"); } }); } catch (error) { fallback = error; }
  if (fallbackStarts.join(",") !== "fleet" || (fallback as { requiredMegabytes?: number })?.requiredMegabytes !== 32) throw Error(`Fleet failure escaped allowance: ${JSON.stringify({ fallbackStarts, fallback })}`);
  console.log("[DEBUG] job slots", JSON.stringify({ local: "refused", remote: starts, failedRemote: "refused locally" }));
} finally { await broker.close(); }
