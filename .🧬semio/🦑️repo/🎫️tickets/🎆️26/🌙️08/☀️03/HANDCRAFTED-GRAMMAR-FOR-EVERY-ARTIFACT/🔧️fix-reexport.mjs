import fs from "fs";

const p = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs";
let t = fs.readFileSync(p, "utf8");
const old = `pub use crate::os_dsl::grammar::{
    canonicalize, parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source, walk_protocol,
    Block, Count, Field, Framing, GrammarFile, Prim, ProtocolFile, ProtocolMismatch, ProtocolTrace, Recognizer, SemioDialect,
};`;
const neu = `pub use crate::os_dsl::grammar::{
    parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source, walk_protocol,
    Block, Count, Field, Framing, GrammarFile, Prim, ProtocolFile, ProtocolMismatch, ProtocolTrace, Recognizer, SemioDialect,
};`;
if (!t.includes(old)) {
  const i = t.indexOf("pub use crate::os_dsl::grammar");
  console.log(t.slice(i, i + 400));
  throw new Error("pattern not found");
}
t = t.replace(old, neu);
fs.writeFileSync(p, t);
console.log("ok");
