import fs from "fs";
import { execSync } from "child_process";

function findTicket() {
  function walk(dir) {
    for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = dir + "/" + ent.name;
      if (ent.isDirectory()) {
        if (ent.name.includes("HANDCRAFTED-GRAMMAR")) return p;
        const f = walk(p);
        if (f) return f;
      }
    }
    return null;
  }
  return walk(".🦑️repo/🎫️tickets");
}

const ticket = findTicket();
const grammarPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs";
console.log("ticket", ticket);

let src = fs.readFileSync(ticket + "/🧪grammar-HEAD.rs", "utf8");
const corrupt = fs.readFileSync(ticket + "/🧪grammar-CORRUPT.rs", "utf8");

const protocolModel = `
//#region 📡️ProtocolModel
/// @emoji 📡️ One parsed \`.protocol.semio\` file: framing + typed body directives.
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolFile {
    pub id: String,
    pub version: u16,
    pub schema: String,
    pub start: String,
    pub uses: Vec<String>,
    pub framing: Framing,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Framing {
    Magic([u8; 8]),
    Record,
    Chunked,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Header(Vec<Field>),
    Segment { name: String, kind: Option<u8>, fields: Vec<Field> },
    Record { name: String, tag: Option<u64>, fields: Vec<Field> },
    Struct { name: String, fields: Vec<Field> },
    Enum { name: String, variants: Vec<(String, u64)> },
    Footer(usize),
    Chain(Prim),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Prim,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prim {
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    Varint,
    Zigzag,
    Bytes,
    Utf8,
    Fixed(usize),
    Array(Box<Prim>, Count),
    Ref(String),
    Tag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Fixed(usize),
    Varint,
    Field(String),
}

/// @emoji ✅️ Successful byte walk: every declared wire slot consumed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolTrace {
    pub consumed: usize,
}

/// @emoji ❌️ Spec/bytes disagreement at a concrete offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolMismatch {
    pub offset: usize,
    pub message: String,
}
//#endregion 📡️ProtocolModel

`;

if (!src.includes("//#endregion 🔖️Model\n")) throw new Error("Model end missing");
src = src.replace("//#endregion 🔖️Model\n", "//#endregion 🔖️Model\n" + protocolModel);

src = src.replace(
  "enum GKind {\n    Ident,\n    Text,",
  "enum GKind {\n    Ident,\n    Int,\n    Text,",
);

src = src.replace(
  "CoreKind::Ident | CoreKind::Placeholder => GKind::Ident,\n                CoreKind::Text => GKind::Text,",
  "CoreKind::Ident | CoreKind::Placeholder => GKind::Ident,\n                CoreKind::Int => GKind::Int,\n                CoreKind::Text => GKind::Text,",
);

fs.writeFileSync(ticket + "/🧪grammar-RESTORE-partial.rs", src);
console.log("ok partial", src.split("\n").length);
