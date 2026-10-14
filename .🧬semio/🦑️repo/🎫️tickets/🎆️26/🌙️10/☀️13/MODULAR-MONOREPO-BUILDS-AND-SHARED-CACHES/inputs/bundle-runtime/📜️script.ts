import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { findWorkspaceRoot, receiveScriptProcessInvocation } from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { bundleVirtualJavaScript } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️.ts";

await receiveScriptProcessInvocation(process.env, async invocation => {
  const workspace = findWorkspaceRoot(import.meta.dir), ticket = resolve(import.meta.dir, "../..");
  for (const mode of ["prototype", "release"] as const) {
    const request = JSON.parse(readFileSync(resolve(ticket, "inputs/bun-" + mode + ".json"), "utf8"));
    const result = await bundleVirtualJavaScript(request, { workspace, temporaryRoot: resolve(ticket, "🗑️generated"), invocation });
    const artifact = await import("data:text/javascript;base64," + Buffer.from(result.source).toString("base64"));
    if (artifact.answer !== 5) throw Error("Generated bundle runtime mismatch");
    console.log("[DEBUG] Bun generated " + mode + " artifact answer=" + artifact.answer + " bytes=" + Buffer.byteLength(result.source));
  }
});
