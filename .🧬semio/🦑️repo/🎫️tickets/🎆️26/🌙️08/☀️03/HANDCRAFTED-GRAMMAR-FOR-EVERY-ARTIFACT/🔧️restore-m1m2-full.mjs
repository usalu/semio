import fs from "fs";
import path from "path";

function findTicket() {
  function walk(dir) {
    for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, ent.name);
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
let src = fs.readFileSync(ticket + "/🧪grammar-RESTORE-partial.rs", "utf8");
const corrupt = fs.readFileSync(ticket + "/🧪grammar-CORRUPT.rs", "utf8");

// Load parser/printer helpers from a companion file if present; else embed critical replacements.
const extrasPath = ticket + "/🧪restore-extras.rs.txt";

// Strategy: cut HEAD parser from is_protocol_directive through ProtocolVerify endregion,
// insert complete protocol parser + print + walk from a generated block.

const cutStart = src.indexOf("fn is_protocol_directive_line");
const cutEnd = src.indexOf("//#endregion 🔖️ProtocolVerify");
if (cutStart < 0 || cutEnd < 0) {
  console.log("cutStart", cutStart, "cutEnd", cutEnd);
  // Maybe already partially transformed - look for alternatives
  console.log("has is_protocol", src.includes("is_protocol_directive_line"));
  console.log("has ProtocolVerify", src.includes("ProtocolVerify"));
  throw new Error("cut markers missing");
}
const afterVerify = src.indexOf("\n", cutEnd) + 1;

// Extract walk region from corrupt (already complete) 
let walk = corrupt.slice(corrupt.indexOf("//#region 📡️ProtocolWalk"));
// Remove trailing tests from walk - keep only through endregion ProtocolWalk
const walkEnd = walk.indexOf("//#endregion 📡️ProtocolWalk");
if (walkEnd < 0) throw new Error("no walk end");
walk = walk.slice(0, walkEnd + "//#endregion 📡️ProtocolWalk".length) + "\n";

// Ensure verify_protocol_source and envelope helpers exist
if (!walk.includes("verify_protocol_source")) {
  walk = walk.replace(
    "//#endregion 📡️ProtocolWalk",
    `/// @emoji 📡️ Parses handcrafted \`.protocol.semio\` source then verifies bytes.
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    verify_protocol_bytes(&spec, bytes)
}

/// @emoji 📡️ Shallow envelope check when a full ProtocolFile is unavailable.
pub fn verify_protocol_envelope(framing_hint: &str, bytes: &[u8]) -> Result<(), String> {
    let hint = framing_hint.to_ascii_lowercase();
    if hint.contains("frame") || hint.contains("pack") || hint.contains("magic") {
        if bytes.len() < 8 {
            return Err("pack bytes shorter than magic".into());
        }
        if &bytes[..8] != &[0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A] {
            return Err("SPK magic mismatch".into());
        }
        return Ok(());
    }
    if hint.contains("record") || hint.contains("spr") {
        if bytes.is_empty() {
            return Err("spr bytes empty".into());
        }
        return Ok(());
    }
    Err(format!("unknown framing hint '{framing_hint}'"))
}
//#endregion 📡️ProtocolWalk
`
  );
}

// The middle block (parser helpers + parse_protocol + print_protocol) is large - load from ticket file
const midPath = ticket + "/🧪restore-mid.rs.txt";
if (!fs.existsSync(midPath)) {
  console.error("MISSING", midPath, "- will write stub marker");
  fs.writeFileSync(midPath, "// PLACEHOLDER - run companion writer\n");
  throw new Error("need mid file");
}
const mid = fs.readFileSync(midPath, "utf8");

// Keep from start through just before is_protocol_directive_line, but we need Cursor helpers.
// Cut from is_protocol_directive - but keep skip_line removal. Also need expect_ident_or_int after expect_ident.

// Find expect_ident end to insert expect_ident_or_int before cut
const expectIdent = src.indexOf("fn expect_ident(&mut self");
const skipLine = src.indexOf("fn skip_line(&mut self)");
if (expectIdent < 0) throw new Error("no expect_ident");

// Rebuild:
// [0 .. skipLine) + expect_ident_or_int + mid + walk + tests from corrupt/custom
const beforeSkip = src.slice(0, skipLine);

const expectIdentOrInt = `    fn expect_ident_or_int(&mut self) -> Result<GToken, TextError> {
        match self.peek().kind {
            GKind::Ident | GKind::Int => Ok(self.advance()),
            other => Err(TextError::new(format!("expected ident or int, found {other:?}"), self.peek().span.clone())),
        }
    }
}

`;

// Remove the rest of Cursor impl from skipLine - mid should start after closing brace of Cursor
// Actually skipLine is inside Cursor impl. beforeSkip includes up to skip_line definition.
// We need to close Cursor after expect_ident_or_int.

// Better: find the closing brace of Cursor impl - it's after skip_line function
const afterSkipLineFn = src.indexOf("\n}\n\nfn is_protocol_directive_line");
if (afterSkipLineFn < 0) throw new Error("cursor close not found");
const beforeCursorClose = src.slice(0, afterSkipLineFn); // ends mid skip_line body area... 

// Simpler approach: find `fn is_protocol_directive_line` and replace from there to ProtocolVerify end
const head = src.slice(0, cutStart);
// Insert expect_ident_or_int into Cursor before the closing - cutStart is is_protocol which is AFTER Cursor closed.
// Looking at HEAD: skip_line is in Cursor, then } closes Cursor, then is_protocol_directive_line
// So head already ends with Cursor closed if cutStart is is_protocol... 
// Wait cutStart is is_protocol_directive_line which is AFTER Cursor. So we need to add expect_ident_or_int inside Cursor.

let head2 = src.slice(0, src.indexOf("fn skip_line(&mut self)"));
head2 += expectIdentOrInt; // this closes Cursor

const testsStart = corrupt.indexOf("//#region 🔖️Tests");
let tests = corrupt.slice(testsStart);
// Ensure self_hosting uses sibling path
tests = tests.replace('include_str!("../../📖️grammar.grammar.semio")', 'include_str!("📖️grammar.grammar.semio")');

const out = head2 + mid + "\n" + walk + "\n" + tests;
fs.writeFileSync(ticket + "/🧪grammar-RESTORE-built.rs", out);
console.log("built lines", out.split("\n").length);
console.log("has parse_protocol", out.includes("pub fn parse_protocol"));
console.log("has walk_protocol", out.includes("pub fn walk_protocol"));
console.log("has ProtocolFile", out.includes("pub struct ProtocolFile"));
