import { readFileSync } from "node:fs";
import { join } from "node:path";
import { sourceDefinitionBodies } from "/Users/ueli/Documents/semio/🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";

const r = "/Users/ueli/Documents/semio";
const read = (p: string) => readFileSync(join(r, p), "utf8");
const s = {
  entrypoint: read("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs"),
  workspace: read("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"),
  remote: read("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"),
  directory: read("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"),
  runner: read("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts"),
  launch: read(".vscode/🧩️launch.seed.jsonc"),
};
const mains = sourceDefinitionBodies(s.entrypoint, /\bfn\s+main\s*\([^)]*\)\s*/gu).filter((b) => b.includes('claim_inherited_local_hub_credential("mcp")'));
const main = mains[0] ?? "";
const claim = main.indexOf('claim_inherited_local_hub_credential("mcp")');
const parsed = main.indexOf("parse_args()", claim);
const activate = main.indexOf("run_stdio(options)", parsed);
const pre = sourceDefinitionBodies(main.slice(0, claim), /\bif\s+std::env::args\(\)\.nth\(1\)\.as_deref\(\)\s*==\s*Some\("schemas"\)\s*/gu);
const prefix = main.slice(0, claim).replace(/\/\/[^\n]*/gu, "");
const seal = sourceDefinitionBodies(s.entrypoint, /\bfn\s+protected_credential_environment_is_absent\s*\([^)]*\)\s*(?:->[^\{]+)?/gu);
const openHub = s.workspace.indexOf("pub fn open_hub(");
const ic = s.workspace.indexOf("set_local_hub_credential(credential)", openHub);
const ig = s.workspace.indexOf("set_hub_socket_grant_source(grant_source)", openHub);
const rw = s.workspace.indexOf("Ok(workspace)", openHub);
const checks: Record<string, boolean> = {
  oneMain: mains.length === 1,
  preflight: pre.length === 1 && pre[0]!.includes("schema_mirror_json()") && pre[0]!.includes("return;") && (prefix.match(/\breturn\s*;/gu) ?? []).length === 1,
  parsedAfterClaim: parsed > claim, activateAfterParse: activate > parsed,
  sealCalled: main.includes("protected_credential_environment_is_absent"), seal: seal.length === 1 && seal[0]!.includes('return value == "3";'),
  noToken: !s.entrypoint.includes('"--token" => hub.'), noSetToken: !s.remote.includes("set_token("), noDirectNew: !s.remote.includes("DirectoryClient::new(transport, base_url)"),
  openHub: openHub >= 0, inject: ic > openHub, grant: ig > ic, ret: rw > ig,
  probeHash: /pub\s+const\s+PROBE_PACK_SCHEMA_HASH:\s*&str\s*=\s*"[0-9a-f]{64}"/u.test(s.workspace),
  known: s.workspace.includes("authenticated_probe_document_is_known"), spec: s.workspace.includes("Some(probe_record_spec())"),
  noSurface: !s.workspace.includes("probe_document_socket_surface"), noLease: !s.workspace.includes("set_document_execution_target_lease("),
  key: s.workspace.includes("artifact_document_key(artifact_id)"), surface: s.workspace.includes("surface: Some(PROBE_SURFACE_ID.to_string())"),
  grants: s.directory.includes('"/directory/socket-grants"'), since: s.directory.includes("/directory/socket/v1?since="), hello: s.directory.includes("directory_socket_hello_v1()"),
  noCargoRun: !s.runner.includes('runCmd("cargo", ["run"'), requireBin: s.runner.includes("runCmd(requireMcpBinary"), launch: s.launch.includes("os-hub:dev-secure-mcp"),
};
console.log(Object.entries(checks).filter(([, v]) => !v).map(([k]) => k), { claim, parsed, activate, pre: pre.length, returns: (prefix.match(/\breturn\s*;/gu) ?? []).length });
