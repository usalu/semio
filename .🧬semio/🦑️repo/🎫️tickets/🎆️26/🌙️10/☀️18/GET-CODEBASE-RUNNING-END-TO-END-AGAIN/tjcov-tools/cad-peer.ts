import { readFileSync } from "node:fs";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const files = { root: `${S}/👥️presence/🦀️.rs`, io: `${S}/👥️presence/🚪️io/📬️publication/🦀️.rs`, eph: `${S}/🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs`, ret: `${S}/👥️presence/♻️retirement/🦀️.rs`, rej: `${S}/👥️presence/🚫️rejection/🦀️.rs` };
for (const [k, f] of Object.entries(files)) {
  const t = readFileSync(f, "utf8");
  for (const header of ["impl<P: Send + Sync + 'static> PresencePeersRetirement<P>", "impl<P: Send + Sync + 'static> PresencePeersPublication<P>", "pub fn publish_peer_commit(", "pub struct PresencePeersCommit<P>", "pub fn adopt(&mut self, actor: String, presence: P", "pub fn begin_retirement(", "pub fn close_step("]) {
    const at = t.indexOf(header);
    if (at < 0) continue;
    const body = toolJobRustBlock(t, t.indexOf("{", at))?.body ?? "";
    console.log(k, JSON.stringify(header).slice(0, 70), "bytes", body.length, "PresencePeersRetirement::new(Entries::one", (body.match(/PresencePeersRetirement::new\(PresencePeersRetiredEntries::one/g) ?? []).length);
  }
}
