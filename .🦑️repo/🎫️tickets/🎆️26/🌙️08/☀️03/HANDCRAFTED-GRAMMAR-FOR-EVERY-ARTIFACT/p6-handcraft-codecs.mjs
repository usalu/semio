import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";

const ticket = path.dirname(fileURLToPath(import.meta.url));
const root = "/Users/ueli/Documents/semio";

function walk(dir, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (e.name === "node_modules" || e.name === "target" || e.name.startsWith(".")) continue;
    const f = path.join(dir, e.name);
    if (e.isDirectory()) walk(f, acc);
    else if (e.name.endsWith(".rs")) acc.push(f);
  }
  return acc;
}

const docImpl = (name) => `
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's \`__dsl_*\` helpers + parse/print, not derive emission.
impl store::DocumentDsl for ${name} {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via \`__dsl_*\` record lowering.
impl store::DocumentPack for ${name} {
    fn encode_pack(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Message(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack(bytes: &[u8]) -> Result<Self, store::PackError> {
        let (_env, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Message(e.to_string()))?;
        let record = store::pack_rt::decode_document(&Self::__dsl_spec(), inner)?;
        Self::__dsl_from_record(&record).map_err(store::PackError::from)
    }
}
`;

const opsImpl = (name) => `
/// ⚡️ Handcrafted OpText (P6): uses this type's \`__dsl_*\` helpers, not derive emission.
impl protocol::OpText for ${name} {
    fn encode_op_text(&self) -> String {
        dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Lines)
    }
    fn decode_op_text(text: &str) -> Result<Self, protocol::OpError> {
        let record = dsl::parse(
            text,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Lines },
        )
        .map_err(|e| protocol::OpError::Message(e.to_string()))?;
        Self::__dsl_from_record(&record).map_err(|e| protocol::OpError::Message(e.to_string()))
    }
}

/// 🎛 Handcrafted OpBinary (P6): binary twin of OpText via pack_rt-style op encoding.
impl protocol::OpBinary for ${name} {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::OpError> {
        store::pack_rt::encode_op(&Self::__dsl_spec(), &self.__dsl_to_record())
            .map_err(|e| protocol::OpError::Message(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::OpError> {
        let record = store::pack_rt::decode_op(&Self::__dsl_spec(), bytes)
            .map_err(|e| protocol::OpError::Message(e.to_string()))?;
        Self::__dsl_from_record(&record).map_err(|e| protocol::OpError::Message(e.to_string()))
    }
}
`;

// Inspect real trait method signatures from an existing file to avoid wrong APIs
const sample = fs.readFileSync(
  path.join(root, "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs"),
  "utf8",
);
const packIdx = sample.indexOf("impl store::DocumentPack for LowpolyProjection");
console.log("sample pack impl:\n", sample.slice(packIdx, packIdx + 900));

const opSampleFiles = walk(path.join(root, "✏️s")).filter((f) => {
  const t = fs.readFileSync(f, "utf8");
  return /impl protocol::OpText for|impl ::protocol::OpText for/.test(t);
});
console.log("opText samples", opSampleFiles.length, opSampleFiles[0]);
if (opSampleFiles[0]) {
  const t = fs.readFileSync(opSampleFiles[0], "utf8");
  const i = t.search(/impl .*OpText for/);
  console.log(t.slice(i, i + 1000));
}
