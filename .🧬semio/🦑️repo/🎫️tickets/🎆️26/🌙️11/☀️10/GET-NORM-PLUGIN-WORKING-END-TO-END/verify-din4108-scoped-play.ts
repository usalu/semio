#!/usr/bin/env repo
import { statSync } from "node:fs";
import { open } from "node:fs/promises";
import { resolve } from "node:path";
import { guardedPlay, play, publishedModules, root, runPlayPlan, waitForPublication, type PlayPlan } from "./guarded-play.ts";

/** 🧱️ Runs archive and picker against the same DIN 4108 lane, verifying signed bytes before either probe. */
export function din4108Plan(env: NodeJS.ProcessEnv): PlayPlan {
  const preview = Boolean(env.PLAY_TEST_GUEST);
  if (preview !== Boolean(env.PLAY_TEST_GUEST_JS)) throw Error("The diagnostic preview needs both the linked core and its bindings");
  const prefix = preview ? "norm-din4108-linked" : "norm-din4108-scoped";
  const port = env.SEMIO_TECH_PLAY_PORT ?? "6035";
  const args = [`http://127.0.0.1:${port}/#din4108`, "300"];
  return {
    scope: "din4108", port, lanes: ["din4108"], laneLog: (_, phase) => `${prefix}-${phase}`,
    serverLog: `${prefix}-server`, readyAttempts: 60, environment: { ONLY_LANES: "din4108" },
    probes: [
      ...preview ? [] : [{ script: "verify-play-components.mjs", log: `${prefix}-delivery`, args: ["delivery", "din4108"] }],
      { script: "probe-archive-play.mjs", log: `${prefix}-archive`, args, cwd: play },
      { script: "probe-archive-play.mjs", log: `${prefix}-picker`, args, cwd: play, environment: { VERIFY_EXAMPLE_WORKFLOW: "1" } },
    ],
  };
}

async function waitForNorm([log, previous = ""]: string[], signal: AbortSignal) {
  if (!log) throw Error("A DIN 4108 build log path is required");
  din4108Plan(process.env);
  const core = process.env.PLAY_TEST_GUEST;
  const bindings = process.env.PLAY_TEST_GUEST_JS;
  await waitForPublication(async () => {
    const receipt = (await publishedModules()).find(row => row.pluginId === "norm");
    if (receipt && core && bindings && [core, bindings].every(path => statSync(resolve(root, path), { throwIfNoEntry: false })?.isFile())) {
      console.log("[DEBUG] The locally linked diagnostic guest is ready");
      return true;
    }
    if (receipt && !core && receipt.wasmSha256 !== previous) {
      console.log(`[DEBUG] Canonical norm receipt present for ${receipt.wasmSha256}`);
      return true;
    }
    if (!core) {
      const file = await open(resolve(root, log), "r").catch(() => undefined);
      if (file) try {
        const { size } = await file.stat();
        const bytes = Buffer.alloc(Math.min(32768, size));
        const { bytesRead } = await file.read(bytes, 0, bytes.length, size - bytes.length);
        if (bytes.toString("utf8", 0, bytesRead).includes("Native build failed:")) throw Error("DIN 4108 build failed before publication");
      } finally {
        await file.close();
      }
    }
    return false;
  }, 8000, "DIN 4108 did not publish during the build window", signal);
}

if (import.meta.main) await guardedPlay(import.meta.path, (_, signal) => runPlayPlan(din4108Plan(process.env), signal), waitForNorm);
