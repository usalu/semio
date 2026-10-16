import { readFileSync } from "node:fs";
import { join } from "node:path";
import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readDirectoryBackendSource } from "/Users/ueli/Documents/semio/🌎️hub/🧪️testing/📖️source-readers/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const origRoot = process.argv[2]!;
function oldText(backend: string): string {
  const [root, ...children] = listModuleTree(join(origRoot, "🌎️hub/📇️directory", backend, "🦀️.rs")).map((file) => readFileSync(file, "utf8"));
  const adapter = root!.indexOf("\nimpl HubDirectory for ");
  const rows = backend === "🌐️neo4j" ? [readFileSync(join(origRoot, "🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs"), "utf8")] : [];
  return [root!.slice(0, adapter), ...children.map((text) => text.replaceAll("pub(super) ", "")), ...rows].join("\n");
}
const method = (source: string, name: string): string => {
  const start = source.indexOf(`async fn ${name}(`);
  return source.slice(start, source.indexOf("\n    async fn ", start + 1));
};
const between = (source: string, a: string, b: string) => source.slice(source.indexOf(a), source.indexOf(b));
const norm = (s: string) => s.split("\n").map((l) => l.trim()).filter((l) => l && !l.startsWith("//!") && !l.startsWith("//#") && l !== "use super::*;" && l !== "}" && !/^impl \w+ \{$/.test(l) && !/^impl \w+ for \w+ \{\}?$/.test(l) && !l.startsWith("use semio_hub_directory::{") && !l.startsWith("/// 🚫️")).join("\n");
let bad = 0;
const check = (label: string, a: string, b: string) => { const same = norm(a) === norm(b); if (!same) bad++; console.log((same ? "same " : "DIFF ") + label, a.length, b.length); };
const names = ["issue_share_token_as", "issue_share_token_as_with_admin_effect", "revoke_share_token_as", "revoke_invite_as", "list_invites", "append_decided_events_with_admin_effect", "events_since", "list_space_administration_members_page", "list_space_administration_invites_page", "redeem_invite_atomic", "persist_event_with_identity", "reserve_artifact_cas", "append_document_genesis", "admin_overview_counts", "rebuild_projections", "rebuild_projections_controlled", "get_role", "authenticate_share_binding", "socket_share_binding", "claim_or_read_directory_command_receipt"];
const multiset = (s: string) => { const m = new Map<string, number>(); for (const l of norm(s).split("\n")) m.set(l, (m.get(l) ?? 0) + 1); return m; };
for (const backend of ["🪶️sqlite", "🐘️postgres", "🌐️neo4j"]) {
  const o = oldText(backend);
  const n = readDirectoryBackendSource(repoRoot, backend);
  for (const name of names) {
    if (!o.includes(`async fn ${name}(`)) continue;
    check(`${backend} method ${name}`, method(o, name), method(n, name));
  }
  check(`${backend} auth`, between(o, "async fn authenticate_share_binding(", "async fn socket_share_binding("), between(n, "async fn authenticate_share_binding(", "async fn socket_share_binding("));
  const shareSchema = (s: string) => { const st = s.indexOf("CREATE TABLE IF NOT EXISTS hub_share_grant ("); return st < 0 ? "" : s.slice(st, s.indexOf("CREATE TABLE", st + 1)); };
  check(`${backend} shareSchema`, shareSchema(o), shareSchema(n));
  const inv = (s: string) => s.slice(s.indexOf("    async fn revoke_invite_as("), s.indexOf("    async fn list_invites("));
  check(`${backend} invite-scope slice`, inv(o), inv(n));
  const f = (s: string) => { const v = s.indexOf("validate_directory_event_page_event(&"); const ins = s.indexOf("INSERT INTO hub_directory_event", s.indexOf("fn persist_event_with_identity")); return [v >= 0, ins >= 0, v > ins]; };
  console.log(backend, "event-page order old", JSON.stringify(f(o)), "new", JSON.stringify(f(n)));
  const mo = multiset(o), mn = multiset(n);
  let missing = 0, extra = 0;
  for (const [k, v] of mo) if ((mn.get(k) ?? 0) < v) missing += v - (mn.get(k) ?? 0);
  for (const [k, v] of mn) if ((mo.get(k) ?? 0) < v) extra += v - (mo.get(k) ?? 0);
  console.log(backend, "whole-text normalized line multiset: missing", missing, "extra", extra);
  for (const [k, v] of mo) if ((mn.get(k) ?? 0) < v) console.log("   -", k.slice(0, 140));
  for (const [k, v] of mn) if ((mo.get(k) ?? 0) < v) console.log("   +", k.slice(0, 140));
}
console.log("bad", bad);
