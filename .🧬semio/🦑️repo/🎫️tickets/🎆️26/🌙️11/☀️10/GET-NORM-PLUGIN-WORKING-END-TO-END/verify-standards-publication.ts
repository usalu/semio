#!/usr/bin/env repo
import { guardedPlay, play, publishedModules, runPlayPlan, waitForPublication, type PlayPlan } from "./guarded-play.ts";

const required = ["norm", "norm-eurocode", "norm-eurocode-structural", "norm-eurocode-materials", "norm-energy"];

/** 🎡️ Verifies all five signed bundles and exercises the Standards host's representative probes. */
export const standardsPlan: PlayPlan = {
  scope: "standards", port: "6033", lanes: ["din4108", "din18599", "en1990", "en1992", "en1995"],
  laneLog: (lane, phase) => `standards-${phase}-${lane}`, serverLog: "standards-live-server", readyAttempts: 90,
  environment: { PLAY_ROUTE_CONCURRENCY: "1" },
  probes: [
    { script: "verify-play-components.mjs", log: "standards-signed-delivery", args: ["delivery"] },
    { script: "probe-archive-play.mjs", log: "standards-din4108-archive", args: ["http://127.0.0.1:6033/#din4108", "300"], cwd: play },
    { script: "probe-eurocode-viewports.mjs", log: "standards-en1990-en1992-capacity", args: ["en1990", "en1992"] },
    { script: "verify-play-components.mjs", log: "standards-all-browser-workflows", args: ["browser"] },
    { script: "probe-norm-catalogue-examples.mjs", log: "standards-catalogue-examples", cwd: play },
  ],
};

async function waitForStandards(_: string[], signal: AbortSignal) {
  const superseded = process.env.NORM_SUPERSEDED_WRAPPER_SHA;
  let ready: string[] = [];
  try {
    await waitForPublication(async () => {
      const receipts = new Map((await publishedModules()).map(receipt => [receipt.pluginId, receipt.wasmSha256]));
      const next = required.filter(id => receipts.has(id) && (id !== "norm" || receipts.get(id) !== superseded)).sort();
      if (next.join() !== ready.join()) console.log(`[DEBUG] Published Norm modules ${next.join(", ")}`);
      ready = next;
      return ready.length === required.length;
    }, 12000, "Missing signed Norm modules", signal);
  } catch (error) {
    signal.throwIfAborted();
    throw Error(`Missing signed Norm modules ${required.filter(id => !ready.includes(id)).sort().join(", ")}`, { cause: error });
  }
}

if (import.meta.main) await guardedPlay(import.meta.path, async (_, signal) => {
  await runPlayPlan(standardsPlan, signal);
  console.log("[DEBUG] All Standards browser verification completed");
}, waitForStandards);
