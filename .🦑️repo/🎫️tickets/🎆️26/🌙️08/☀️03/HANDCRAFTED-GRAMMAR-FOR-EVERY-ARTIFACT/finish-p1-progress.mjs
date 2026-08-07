import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";

function findTicket(p) {
  for (const n of fs.readdirSync(p)) {
    const full = path.join(p, n);
    if (n.includes("HANDCRAFTED-GRAMMAR")) return full;
    if (fs.statSync(full).isDirectory()) {
      const r = findTicket(full);
      if (r) return r;
    }
  }
  return null;
}

const ticket = findTicket(path.join(root, ".🦑️repo", "🎫️tickets"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const file = path.join(
  root,
  fw,
  "🛍️products",
  "💻️os",
  "🔨️modules",
  "🗣️dsl",
  "📖️grammar",
  "🦀️component.rs",
);
const dsl = path.join(
  root,
  fw,
  "🛍️products",
  "💻️os",
  "🔨️modules",
  "🗣️dsl",
  "🦀️component.rs",
);
const progress = path.join(ticket, "progress-v2.md");

let s = fs.readFileSync(file, "utf8");
const old =
  'let is_pack = start == "frame" || id.contains("pack") || matches!(spec.dialect, SemioDialect::Protocol) && !is_spr;';
const neu =
  'let is_pack = start == "frame" || id.contains("pack") || (matches!(spec.dialect, SemioDialect::Protocol) && !is_spr);';
if (s.includes(old)) {
  s = s.replace(old, neu);
  fs.writeFileSync(file, s);
  console.log("fixed precedence");
} else {
  console.log(s.includes(neu) ? "parens ok" : "form unexpected");
}
console.log("verify", /pub fn verify_protocol_bytes\([^)]+\)/.exec(s)?.[0]);
console.log("envelope in grammar", s.includes("verify_protocol_envelope"));
console.log("envelope in dsl", fs.readFileSync(dsl, "utf8").includes("verify_protocol_envelope"));
console.log("body-as-rest", s.includes("record_body_as_rest"));
console.log("source walks", s.includes("walk_protocol(&spec, bytes)"));

let t = fs.readFileSync(progress, "utf8");
const marker = "verify API correction (session continue)";
if (!t.includes(marker)) {
  const block = [
    "",
    "### P1 protocol engine — " + marker,
    "- Restored required split: verify_protocol_bytes(&GrammarFile, bytes) = shallow any-0x89 (>=32 pack / non-empty spr); verify_protocol_source = parse + walk_protocol deep.",
    "- Removed verify_protocol_envelope from grammar + dsl re-exports.",
    "- walk_protocol: Framing::Record named records consume body-as-rest; magic/chunked skip named records; empty-name spr preamble fields walk normally (Prim::Bytes rest).",
    "- Tests: Shape A uses project_protocol for shallow + verify_protocol_source for deep; added verify_protocol_bytes_accepts_any_0x89_magic.",
    "- Policy: five P3 breaches already in VerifyScript.runGate via policyHandcraftedSpecP3Breaches.",
    "- cargo test still blocked by Xcode SDK license (cc/sccache exit 69) — tests not executed on this host.",
    "",
  ].join("\n");
  fs.writeFileSync(progress, t.trimEnd() + "\n" + block);
  console.log("progress appended");
} else {
  console.log("progress already noted");
}
