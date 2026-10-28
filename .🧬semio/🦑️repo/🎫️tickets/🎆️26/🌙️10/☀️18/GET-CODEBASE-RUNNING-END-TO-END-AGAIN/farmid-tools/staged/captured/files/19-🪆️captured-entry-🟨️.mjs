import { GRAPH_FARM_WORKER_ROLE_V1 } from "../../../🪆️captured/🟨️.mjs";

/** 🧪️ The entry of a small captured snapshot: serves the graph farm worker role the way a verb kernel does, so a farm starts its workers through the real stream loader with the real worker loop and a stand-in service. */
export async function runCapturedBuildSourceV1(args) {
  if (args.length !== 1 || args[0] !== GRAPH_FARM_WORKER_ROLE_V1) throw Error("Graph farm worker role required");
  await import("../../../👷️worker/🟨️.mjs");
}
