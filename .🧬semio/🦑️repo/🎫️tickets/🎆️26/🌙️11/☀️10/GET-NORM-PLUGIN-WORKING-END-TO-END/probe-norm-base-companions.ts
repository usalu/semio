#!/usr/bin/env repo
import { guardedPlay, play, publishedModules, runPlayPlan, runSweep, waitForPublication, type PlayPlan } from "./guarded-play.ts";

/** 🧭️ Builds the strict or family-specific browser checks for one Standards route. */
export function companionPlan(scope: string, env: NodeJS.ProcessEnv): PlayPlan {
  const strict = env.PLAY_TEST_PICKER_ROUNDTRIP === "1" && env.PLAY_TEST_OBSERVE_ARCHIVES === "1";
  const viewport = (scope === "en1990" || scope === "en1992") && !strict;
  return {
    scope, port: env.SEMIO_TECH_PLAY_PORT ?? "6036", lanes: [scope],
    laneLog: (_, phase) => `norm-base-preview-${phase}-${scope}`,
    serverLog: `norm-base-preview-service-${scope}`, readyAttempts: 30,
    probes: [
      ...env.VERIFY_SIGNED_MODULES === "1" ? [{ script: "verify-play-components.mjs", log: `norm-scoped-signed-delivery-${scope}`, args: ["delivery", scope] }] : [],
      viewport
        ? { script: "probe-eurocode-viewports.mjs", log: `norm-published-eurocode-viewport-${scope}`, args: [scope] }
        : { script: "probe-norm-routes.mjs", log: `norm-base-preview-route-${scope}`, args: [scope] },
      ...["din16798", "iso16757", "vdi3805"].includes(scope)
        ? [{ script: "probe-norm-catalogue-examples.mjs", log: `norm-base-preview-examples-${scope}`, args: [scope], cwd: play }] : [],
    ],
  };
}

if (import.meta.main) await guardedPlay(import.meta.path, async (args, signal) => {
  const scopes = args.length ? args : ["din16798", "iso16757", "vdi3805"];
  await runSweep(scopes, signal, async scope => {
    await runPlayPlan(companionPlan(scope, process.env), signal);
    console.log(`[DEBUG] ${scope} browser outcome 0`);
  });
}, async (_, signal) => {
  const id = process.env.NORM_WAIT_FOR_MODULE;
  if (id) await waitForPublication(async () => {
    const receipt = (await publishedModules()).find(row => row.pluginId === id);
    if (receipt) console.log(`[DEBUG] ${id} published ${receipt.wasmSha256}`);
    return receipt !== undefined;
  }, 8000, `${id} did not publish during the build window`, signal);
});
