import { readFileSync } from "fs";
const f = "/Users/ueli/Documents/semio/🧰framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs";
const s = readFileSync(f, "utf8");
console.log("lines", s.split("\n").length, "bytes", s.length);
const checks = [
  "pub fn parse_protocol",
  "pub fn print_protocol",
  "pub fn walk_protocol",
  "pub fn verify_protocol_bytes",
  "pub fn verify_protocol_source",
  "verify_protocol_envelope",
  "from_record_spec",
  "terminal_for_shape",
  "record_body_as_rest",
  "fn default_macros",
  "Terminal::Bool",
  "DASHARROW",
  "BACKARROW",
  "EDGEARROW",
  "EQUALS",
  "QUANTITY",
];
for (const c of checks) {
  const i = s.indexOf(c);
  console.log(c, i >= 0 ? "YES@" + i : "NO");
}
const i = s.indexOf("pub fn verify_protocol_bytes");
console.log("--- verify_protocol_bytes ---");
console.log(s.slice(i, i + 800));
const w = s.indexOf("pub fn walk_protocol");
console.log("--- walk_protocol ---");
console.log(s.slice(w, w + 1400));
