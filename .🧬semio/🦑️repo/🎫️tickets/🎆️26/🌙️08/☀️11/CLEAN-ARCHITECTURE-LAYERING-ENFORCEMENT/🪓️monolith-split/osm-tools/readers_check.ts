import { policyReadRustPolicySource, policyReadRustSourceEvidence } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const M = "🧰️framework/🛍️products/💻️os/🔨️modules";
const checks: [string, string[]][] = [
  [`${M}/🛎️services/🦀️.rs`, ["struct ScopeTable", "pub struct HttpPool", "pub struct EventRouter", "struct ComputeJobParamsPreparation", "fn schedule_http_refill_turn", "fn storage_try_dispatch"]],
  [`${M}/🌉️mcp/🚚️transport/🦀️.rs`, ["pub(crate) struct HttpTransportState", "fn websocket_accept", "pub(crate) fn origin_allowed"]],
  [`${M}/🌉️mcp/🧵️bridge/🦀️.rs`, ["pub struct ShellToGatewayMaterializeCursor", "pub enum GatewayToShell", "pub fn write_u8"]],
  [`${M}/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`, ["pub struct HubRemoteBinding", "document_execution_target_manifest", "authority_generation", "descriptor_byte_sha256", "checkpoint_resource_uri"]],
];
for (const [path, needles] of checks) {
  const text = policyReadRustPolicySource(repoRoot, path);
  const tree = readModuleTreeSource(`${repoRoot}/${path}`, { crates: false });
  console.log(path.slice(-40), "policy chars", text.length, "tree chars", tree.length, needles.map((n) => `${n}:${text.includes(n)}`).join(" "));
}
const ev = policyReadRustSourceEvidence(repoRoot, `${M}/🛎️services/🦀️.rs`);
console.log("services evidence tests:", ev.tests.map((t) => t.path.slice(-50) + " chars " + t.source.length), "production chars", ev.production.source.length);
