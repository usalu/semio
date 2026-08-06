import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";
const fw = readdirSync(".").find((x) => x.includes("framework"));
const os = join(fw, "🛍️products", readdirSync(join(fw, "🛍️products")).find((x) => x.includes("os")));
const dsl = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("dsl")));
const grammar = join(dsl, "📖️grammar");
const paths = [
  join(grammar, "🦀️component.rs"),
  join(grammar, "⚡️implementations/🦀️rust/📦️lib.rs"),
];
const replacement = `pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    if spec.dialect != SemioDialect::Protocol {
        return Err("verify_protocol_bytes requires dialect protocol".to_string());
    }
    let is_pack = spec.start == "frame" || spec.id.contains("pack");
    let is_spr = spec.start == "record" || spec.id.contains("spr");
    if is_pack {
        if bytes.len() < 8 {
            return Err("pack bytes shorter than magic".to_string());
        }
        if &bytes[..8] != &[0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A] {
            return Err("SPK magic mismatch".to_string());
        }
        if bytes.len() < 32 {
            return Err("pack header requires 32 bytes".to_string());
        }
        return Ok(());
    }
    if is_spr {
        if bytes.is_empty() {
            return Err("spr bytes empty".to_string());
        }
        // record protocols: at least format u8 present
        return Ok(());
    }
    Err(format!("protocol spec start '{}' is neither frame nor record", spec.start))
}`;
for (const p of paths) {
  let t = readFileSync(p, "utf8");
  const re = /pub fn verify_protocol_bytes\(spec: &GrammarFile, bytes: &\[u8\]\) -> Result<\(\), String> \{[\s\S]*?\n\}/;
  if (!re.test(t)) {
    console.log("no match", p);
    continue;
  }
  t = t.replace(re, replacement);
  writeFileSync(p, t);
  console.log("patched verify", p);
}
