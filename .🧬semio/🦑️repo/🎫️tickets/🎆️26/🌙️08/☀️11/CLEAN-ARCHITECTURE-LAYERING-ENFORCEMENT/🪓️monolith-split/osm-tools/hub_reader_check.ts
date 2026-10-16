import { readFileSync } from "node:fs";
import { join } from "node:path";
import { mcpCredentialSourceOrderConforms } from "../../../../../../../../../🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";
import { readMcpGatewaySource } from "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const snap = `${repoRoot}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/osm-work/snap-mcp`;
const M = "🧰️framework/🛍️products/💻️os/🔨️modules";
const remoteRel = `${M}/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`;
const base = {
  entrypoint: readFileSync(join(repoRoot, `${M}/🌉️mcp/🏗️bootstrap/🦀️.rs`), "utf8"),
  workspace: readMcpGatewaySource(repoRoot),
  directory: readFileSync(join(repoRoot, `${M}/📇️directory/🔌️client/🦀️.rs`), "utf8"),
  runner: readFileSync(join(repoRoot, `${M}/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`), "utf8"),
  project: readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📋️project.json"), "utf8"),
};
const before = mcpCredentialSourceOrderConforms({ ...base, remote: readFileSync(join(snap, remoteRel), "utf8") } as never);
const after = mcpCredentialSourceOrderConforms({ ...base, remote: readModuleTreeSource(join(repoRoot, remoteRel), { crates: false }) } as never);
console.log("mcpCredentialSourceOrderConforms before(original remote text):", before, "after(module-tree text):", after);
