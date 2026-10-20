// 🚀 Runs `bun <args>` under a genuinely issued process envelope (as the real runner supplies it), cwd = the script's package directory.
import { dirname } from "node:path";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope, scriptInvocationEnvironment } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const root = process.cwd(), argv = process.argv.slice(2), script = argv.find(value => value.endsWith(".ts") && !value.includes("fs-trace"))!;
const selection = (process.env.BZL_LAZY_SELECTION ?? "run fixture:build").split(" "), capabilities = createNativeBuildCallerCapabilitiesV1(root, selection, "bazel", {});
await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "play-bzl:harness", maximumElapsedMilliseconds: 1800000 }, capabilities, Date.now()), async original => {
  const child = Bun.spawn([process.execPath, ...argv], { cwd: dirname(script), env: scriptInvocationEnvironment(original, { ...process.env }), stdout: "inherit", stderr: "inherit" });
  process.exitCode = await child.exited;
});
