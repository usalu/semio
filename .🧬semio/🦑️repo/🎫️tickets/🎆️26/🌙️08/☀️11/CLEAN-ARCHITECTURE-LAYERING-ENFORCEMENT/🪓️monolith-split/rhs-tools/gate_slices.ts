import { readFileSync } from "node:fs";
import { join } from "node:path";
import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const T = import.meta.dir;
function newText(backend: string): string {
  const [root, ...children] = listModuleTree(join(repoRoot, "🌎️hub/📇️directory", backend, "🦀️.rs")).map((file) => readFileSync(file, "utf8"));
  const adapter = root!.indexOf("\nimpl HubDirectory for ");
  const rows = backend === "🌐️neo4j" ? [readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs"), "utf8")] : [];
  return [root!.slice(0, adapter), ...children.map((text) => text.replaceAll("pub(super) ", "")), ...rows].join("\n");
}
const old: Record<string, string> = {
  "🪶️sqlite": readFileSync(join(T, "orig/sqlite.rs.txt"), "utf8"),
  "🐘️postgres": readFileSync(join(T, "orig/postgres.rs.txt"), "utf8"),
  "🌐️neo4j": readFileSync(join(T, "orig/neo4j.rs.txt"), "utf8") + "\n" + readFileSync(join(T, "orig/neo4j-rows.rs.txt"), "utf8"),
};
const method = (source: string, name: string): string => {
  const start = source.indexOf(`async fn ${name}(`);
  return source.slice(start, source.indexOf("\n    async fn ", start + 1));
};
const between = (source: string, a: string, b: string) => source.slice(source.indexOf(a), source.indexOf(b));
const norm = (s: string) => s.split("\n").map((l) => l.trim()).filter((l) => l && !l.startsWith("//!") && l !== "use super::*;" && l !== "}" && !/^impl \w+ \{$/.test(l)).join("\n");
let bad = 0;
const check = (label: string, a: string, b: string) => { const same = norm(a) === norm(b); if (!same) bad++; console.log((same ? "same " : "DIFF ") + label, a.length, b.length); };
const names = ["issue_share_token_as", "issue_share_token_as_with_admin_effect", "revoke_share_token_as", "revoke_invite_as", "list_invites", "append_decided_events_with_admin_effect", "events_since", "list_space_administration_members_page", "list_space_administration_invites_page", "redeem_invite_atomic", "persist_event_with_identity", "reserve_artifact_cas", "append_document_genesis"];
for (const [backend, o] of Object.entries(old)) {
  const n = newText(backend);
  for (const name of names) {
    if (!o.includes(`async fn ${name}(`)) continue;
    check(`${backend} method ${name}`, method(o, name), method(n, name));
  }
  if (o.includes("async fn authenticate_share_binding(")) check(`${backend} auth`, between(o, "async fn authenticate_share_binding(", "async fn socket_share_binding("), between(n, "async fn authenticate_share_binding(", "async fn socket_share_binding("));
  const shareSchema = (s: string) => { const st = s.indexOf("CREATE TABLE IF NOT EXISTS hub_share_grant ("); return st < 0 ? "" : s.slice(st, s.indexOf("CREATE TABLE", st + 1)); };
  check(`${backend} shareSchema`, shareSchema(o), shareSchema(n));
  const inv = (s: string) => s.slice(s.indexOf("    async fn revoke_invite_as("), s.indexOf("    async fn list_invites("));
  check(`${backend} invite-scope slice`, inv(o), inv(n));
}
// order-sensitive predicate used by the event-page gate (sqlite)
for (const [backend, o] of Object.entries(old)) {
  const n = newText(backend);
  const f = (s: string) => { const v = s.indexOf("validate_directory_event_page_event(&"); const ins = s.indexOf("INSERT INTO hub_directory_event", s.indexOf("fn persist_event_with_identity")); return [v >= 0, ins >= 0, v > ins]; };
  console.log(backend, "event-page order old", JSON.stringify(f(o)), "new", JSON.stringify(f(n)));
}
console.log("bad", bad);
