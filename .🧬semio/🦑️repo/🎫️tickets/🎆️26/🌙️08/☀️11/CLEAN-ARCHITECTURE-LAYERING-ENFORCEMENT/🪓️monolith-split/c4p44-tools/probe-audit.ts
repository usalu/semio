// 🔬️ c4p44 reader probe: the interactivity audit MCP bridge law on the live tree (module-tree session text) against the same law fed the pre-split snapshot text of the session file.
import {readFileSync} from "node:fs";
import {policyReadRustPolicySource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import {interactivityMcpHttpTransportFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🟦️.ts";
import {INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE, INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE, INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE, INTERACTIVITY_AUDIT_MCP_ROOT_FILE} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const transport = policyReadRustPolicySource(root, INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE);
const bridge = policyReadRustPolicySource(root, INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE);
const rootSrc = policyReadRustPolicySource(root, INTERACTIVITY_AUDIT_MCP_ROOT_FILE);
const sessionNew = policyReadRustPolicySource(root, INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE);
const sessionOrig = readFileSync(process.argv[2]!, "utf8");
const fNew = interactivityMcpHttpTransportFailures(transport, bridge + sessionNew, rootSrc);
const fOrig = interactivityMcpHttpTransportFailures(transport, bridge + sessionOrig, rootSrc);
console.log(JSON.stringify({sessionNewLines: sessionNew.split("\n").length, sessionOrigLines: sessionOrig.split("\n").length, failuresNew: fNew, failuresOrig: fOrig, equal: JSON.stringify(fNew) === JSON.stringify(fOrig)}));
