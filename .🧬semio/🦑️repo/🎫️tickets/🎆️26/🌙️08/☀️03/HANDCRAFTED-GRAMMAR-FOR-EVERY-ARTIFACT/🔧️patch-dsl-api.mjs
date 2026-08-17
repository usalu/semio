import fs from "fs";

const dslPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs";
let dsl = fs.readFileSync(dslPath, "utf8");

const oldUse = `pub use crate::os_dsl::grammar::{
    parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source, GrammarFile, ProtocolFile, SemioDialect,
    Recognizer,
};`;
const newUse = `pub use crate::os_dsl::grammar::{
    canonicalize, parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source, walk_protocol,
    Block, Count, Field, Framing, GrammarFile, Prim, ProtocolFile, ProtocolMismatch, ProtocolTrace, Recognizer, SemioDialect,
};`;
if (!dsl.includes(oldUse)) throw new Error("old use not found: " + dsl.slice(dsl.indexOf("pub use crate::os_dsl::grammar"), dsl.indexOf("pub use crate::os_dsl::grammar") + 300));
dsl = dsl.replace(oldUse, newUse);

const idx = dsl.indexOf("    pub fn parsed_protocol(&self)");
if (idx < 0) throw new Error("parsed_protocol not found");
const docStart = dsl.lastIndexOf("    /// @emoji", idx);
const verifyIdx = dsl.indexOf("    pub fn verify_protocol(&self", idx);
const afterVerify = dsl.indexOf("\n    }\n", verifyIdx);
const blockEnd = afterVerify + "\n    }\n".length;

const newParsed = [
  "    /// @emoji 📡️ Parses `protocol` via [`parse_protocol`].",
  "    pub fn parsed_protocol(&self) -> Result<Option<ProtocolFile>, TextError> {",
  "        let Some(text) = self.protocol else {",
  "            return Ok(None);",
  "        };",
  "        Ok(Some(parse_protocol(text)?))",
  "    }",
  "",
  "    /// @emoji ✅ Verifies encoded bytes against this language's protocol when protocol text is present.",
  "    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {",
  "        let Some(file) = self.parsed_protocol().map_err(|e| e.message.clone())? else {",
  "            return Ok(());",
  "        };",
  "        verify_protocol_bytes(&file, bytes)",
  "    }",
  "",
].join("\n");

dsl = dsl.slice(0, docStart) + newParsed + dsl.slice(blockEnd);
fs.writeFileSync(dslPath, dsl);
console.log("patched dsl");

const lspDirName = fs.readdirSync("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl").find((n) => n.includes("lsp"));
const lspPath = `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/${lspDirName}/🦀️component.rs`;
let lsp = fs.readFileSync(lspPath, "utf8");
if (!lsp.includes("ProtocolFile")) {
  lsp = lsp.replace(
    "use crate::os_dsl::{CompletionItem, GrammarFile, LanguageSpec, TextError, TokenClass};",
    "use crate::os_dsl::{CompletionItem, GrammarFile, LanguageSpec, ProtocolFile, TextError, TokenClass};",
  );
}
lsp = lsp.replace(
  "pub fn protocol_file(&self) -> Result<Option<GrammarFile>, TextError> {",
  "pub fn protocol_file(&self) -> Result<Option<ProtocolFile>, TextError> {",
);
fs.writeFileSync(lspPath, lsp);
console.log("patched lsp", lspPath);
