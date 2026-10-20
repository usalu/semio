/** 🎚️ Loads the target-policy module, the workspace policy and the closed schemas that live under one root (a checkout or the overlay), wired exactly as the inference wires them. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

export async function loadAt(root: string) {
  const L = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
  const module = await import(join(L, "🏗️build/🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs") + `?at=${Date.now()}`);
  const POLICY = JSON.parse(readFileSync(join(L, "⚡️caching/🔣️policy.json"), "utf8"));
  const targetCacheSchema = JSON.parse(readFileSync(join(L, "🏗️build/🏃️execution/🧬️schema/💾️cache.json"), "utf8"));
  const targetExecutionSchema = JSON.parse(readFileSync(join(L, "🏗️build/🏃️execution/🧬️schema/🔣️.json"), "utf8"));
  module.wire({ coordinates: { POLICY, targetExecutionSchema, targetCacheSchema, DEFAULT_EXECUTOR: "repo:script" } });
  return { module, POLICY };
}
