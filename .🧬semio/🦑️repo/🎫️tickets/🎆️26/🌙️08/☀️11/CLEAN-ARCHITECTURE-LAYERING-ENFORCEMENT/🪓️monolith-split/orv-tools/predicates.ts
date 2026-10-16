import { readFileSync } from "node:fs";
import { relative } from "node:path";
import { policyReadFileSafe, policyReadModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { policyReadOsStoreSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { readMcpGatewaySource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { toolJobArtifactStoreStructuralOwnersExact, toolJobPagedIngressExact } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const O = "🧰️framework/🛍️products/💻️os/🔨️modules";
const orig = (n: string) => readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/orv-tools/orig/" + n + ".rs.txt", "utf8");
const store = policyReadOsStoreSource(root);
const semio = policyReadFileSafe(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs");
const causal = policyReadFileSafe(root, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs");
const kernel = policyReadFileSafe(root, "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs");
const reactor = policyReadModuleTreeSource(root, O + "/🔌️plugin/⚛️reactor/🦀️.rs");
const plugin = policyReadFileSafe(root, O + "/🔌️plugin/🦀️.rs");
const pluginHost = policyReadModuleTreeSource(root, O + "/🔌️plugin/🖥️host/🦀️.rs");
const channel = policyReadModuleTreeSource(root, O + "/📡️spr/🧵️channel/🦀️.rs");
const componentWit = policyReadFileSafe(root, O + "/🔌️plugin/🧬️schema/📜️.wit");
const mcpWorkspace = readMcpGatewaySource(root, (file: string) => policyReadFileSafe(root, relative(root, file)));
const wgpuHost = policyReadModuleTreeSource(root, O + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
const vcsNow = policyReadModuleTreeSource(root, O + "/🌿️vcs/🦀️.rs");
const runNow = policyReadModuleTreeSource(root, O + "/🏃️run/🦀️.rs");
console.log(JSON.stringify({
  structuralOwners: { old: toolJobArtifactStoreStructuralOwnersExact(store, semio, causal, orig("vcs")), now: toolJobArtifactStoreStructuralOwnersExact(store, semio, causal, vcsNow), single_file_now: toolJobArtifactStoreStructuralOwnersExact(store, semio, causal, policyReadFileSafe(root, O + "/🌿️vcs/🦀️.rs")) },
  pagedIngress: { old: toolJobPagedIngressExact(kernel, reactor, plugin, pluginHost, channel, componentWit, mcpWorkspace, orig("run"), wgpuHost), now: toolJobPagedIngressExact(kernel, reactor, plugin, pluginHost, channel, componentWit, mcpWorkspace, runNow, wgpuHost), single_file_now: toolJobPagedIngressExact(kernel, reactor, plugin, pluginHost, channel, componentWit, mcpWorkspace, policyReadFileSafe(root, O + "/🏃️run/🦀️.rs"), wgpuHost) },
}));
