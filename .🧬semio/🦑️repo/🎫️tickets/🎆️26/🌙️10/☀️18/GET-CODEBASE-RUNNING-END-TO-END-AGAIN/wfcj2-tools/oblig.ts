import { readFileSync } from "node:fs";
import { plugin } from "./trace.ts";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const rows: [string, string][] = JSON.parse(readFileSync(import.meta.dir + "/obligations.json", "utf8")).map(([a, b]: [string, string]) => [a, JSON.parse(`"${b}"`)]);
const body = (text: string, name: string): string => {
  const owner = name === "has_runnable_typed_operations" ? "impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M>" : ["take_result_page", "reject_cancelled_publication"].includes(name) ? "impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>" : undefined;
  const ownerStart = owner ? text.indexOf(owner) : -1;
  const scope = owner ? ownerStart < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", ownerStart))?.body ?? "" : text;
  const start = scope.lastIndexOf(`fn ${name}(`);
  return start < 0 ? "" : toolJobRustBlock(scope, scope.indexOf("{", start))?.body ?? "";
};
let bad = 0;
for (const [name, token] of rows) {
  const b = body(plugin, name);
  const ok = b.includes(token);
  if (!ok) { bad++; console.log(b ? "MISSING-TOKEN" : "NO-BODY", name, "::", token.slice(0, 110)); }
}
console.log(rows.length, "obligations", bad, "failing");
