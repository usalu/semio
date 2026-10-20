/** 🧪 Loads the live and the staged target-policy module as overlay COPIES (one import re-pointed) and wires each with its own policy and cache schema. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const WS = resolve(import.meta.dir, "../../../../../../../..");
const L = join(WS, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const TP = "🏗️build/🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs";
const out = join(import.meta.dir, "../🗑️generated/cachefix-ov");
mkdirSync(out, { recursive: true });
const arguments_ = join(L, "🏗️build/📤️arguments/🟨️.mjs");

export async function load(kind: "live" | "staged") {
  const source = kind === "live" ? join(L, TP) : join(import.meta.dir, "staged", L.slice(WS.length + 1), TP);
  const policyPath = kind === "live" ? join(L, "⚡️caching/🔣️policy.json") : join(import.meta.dir, "staged", L.slice(WS.length + 1), "⚡️caching/🔣️policy.json");
  const cachePath = kind === "live" ? join(L, "🏗️build/🏃️execution/🧬️schema/💾️cache.json") : join(import.meta.dir, "staged", L.slice(WS.length + 1), "🏗️build/🏃️execution/🧬️schema/💾️cache.json");
  const text = readFileSync(source, "utf8").replace('"../../../📤️arguments/🟨️.mjs"', JSON.stringify(arguments_));
  if (text === readFileSync(source, "utf8")) throw Error("import anchor missing");
  const copy = join(out, `${kind}-target-policy-${Date.now()}.mjs`);
  writeFileSync(copy, text);
  const module = await import(copy);
  const POLICY = JSON.parse(readFileSync(policyPath, "utf8")), targetCacheSchema = JSON.parse(readFileSync(cachePath, "utf8")), targetExecutionSchema = JSON.parse(readFileSync(join(L, "🏗️build/🏃️execution/🧬️schema/🔣️.json"), "utf8"));
  module.wire({ coordinates: { POLICY, targetExecutionSchema, targetCacheSchema, DEFAULT_EXECUTOR: "repo:script" } });
  return { module, POLICY };
}
