/** 🧭 Moves the existing VT trim-box codec laws into the package that owns the codecs. */
import assert from "node:assert/strict";
import { dirname, join, resolve } from "node:path";

const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const base = dirname(root);
const ticket = import.meta.dir;
const generated = join(ticket, "🗑️generated");
const mirror = join(base, "🧬️schema/🧬️mutations/🪄️replace-page/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs");
const owner = join(dirname(base), "🧾️vt/🚪️io/📦️packages/🦀️rust/🦀️.rs");
type Candidate = [path: string, before: string, body: string];
const prior = await Bun.file(join(generated, "pdf-compressed-byte-overlay.json")).json() as Candidate[];
const next = new Map(prior.map(row => [row[0], row]));
const fromOwner = await Bun.file(mirror).text();
const toOwner = await Bun.file(owner).text();
let former = fromOwner;
let current = toOwner;
const fixtureBodies = [];
function replace(body: string, from: string, to: string) {
  assert.equal(body.split(from).length, 2, from);
  return body.replace(from, to);
}

for (const [name, domain] of [["remove_trim_box", "✂️remove-trim-box"], ["set_trim_box", "📐️set-trim-box"]]) {
  const sourceModule = "../../📝️text/🧬️mutations/" + domain;
  const previous =
    "                            #[path = \".\"]\n"
    + "                            pub mod " + name + " {\n"
    + "                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::" + name + "::*;\n"
    + "                                #[cfg(test)]\n"
    + "                                #[path = \"../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/" + domain + "/🧪️tests/🔬️unit/🦀️.rs\"]\n"
    + "                                pub mod tests;\n"
    + "                            }";
  former = replace(former, previous,
    "                            pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::" + name + ";");
  const module =
    "                            #[path = \"" + sourceModule + "/🦀️.rs\"]\n"
    + "                            pub mod " + name + ";";
  const testPath = sourceModule + "/🧪️tests/🔬️unit/🦀️.rs";
  current = replace(current, module, module + "\n"
    + "                            #[cfg(test)]\n"
    + "                            #[path = \"" + testPath + "\"]\n"
    + "                            mod " + name + "_tests;");
  const fixture = await Bun.file(resolve(dirname(owner), testPath)).text();
  assert(fixture.includes("#[test]\nfn owned_payload_round_trips()"));
  fixtureBodies.push("mod " + name + " {\n" + fixture.replace("#[test]\nfn owned_payload_round_trips()", "pub(super) fn owned_payload_round_trips()") + "\n}");
}
for (const [path, before, body] of [[mirror, fromOwner, former], [owner, toOwner, current]]) {
  assert(resolve(path).normalize("NFD").length <= 256, path);
  const digest = new Bun.CryptoHasher("sha256").update(before).digest("hex");
  next.set(path, [path, digest, body]);
}
await Bun.write(join(generated, "pdf-compressed-byte-install-overlay.json"), JSON.stringify([...next.values()]));
await Bun.write(join(generated, "vt-trim-box-test-owner.rs"), [
  "pub use semio_s_artifact_stdio_pdf_mut_vt::standards;",
  ...fixtureBodies,
  "fn main() {",
  "    remove_trim_box::owned_payload_round_trips();",
  "    set_trim_box::owned_payload_round_trips();",
  '    println!("[DEBUG] Both original VT trim-box codec laws passed under WASI");',
  "}",
].join("\n"));
console.log("[DEBUG] Staged two unchanged VT codec laws under their existing owner", next.size);

