#!/usr/bin/env bun
/** [DEBUG] P6: inject handcrafted DocumentDsl/DocumentPack/OpText/OpBinary; fix store op_rt; expose pack helpers; empty exemptions. */
import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from "fs";
import { join, dirname, relative } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";

function findModule(emojiName) {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", emojiName);
    if (existsSync(p)) return p;
  }
  throw new Error("module not found: " + emojiName);
}

const DSL_DIR = findModule("🗣️dsl");
const STORE_FILE = join(findModule("🏪️store"), "🦀️component.rs");
const SCRIPT_TS = join(repo, "📜️script.ts");
const PLUGINS = join(repo, "✏️s", "🔌️plugins");

const report = {
  injectedDocs: [],
  injectedOps: [],
  skippedDocs: [],
  skippedOps: [],
  store: {},
  policy: {},
  errors: [],
};

function replaceOnce(hay, needle, repl, label) {
  const i = hay.indexOf(needle);
  if (i < 0) throw new Error("missing needle: " + label);
  const j = hay.indexOf(needle, i + 1);
  if (j >= 0) throw new Error("needle not unique: " + label);
  return hay.slice(0, i) + repl + hay.slice(i + needle.length);
}

function walkRs(dir, out = []) {
  if (!existsSync(dir)) return out;
  for (const name of readdirSync(dir)) {
    if (name === "target" || name === "node_modules") continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walkRs(p, out);
    else if (name.endsWith(".rs")) out.push(p);
  }
  return out;
}

function findTypeAfterDerive(content, deriveIdx) {
  const after = content.slice(deriveIdx);
  const m = after.match(/#\[derive\s*\(([^)]*)\)\]([\s\S]*?)(?:pub\s+)?(struct|enum)\s+([A-Za-z0-9_]+)/);
  if (!m) return null;
  return { attrs: m[1], kind: m[3], name: m[4], matchLen: m[0].length };
}

function hasImplFor(content, traitName, typeName) {
  const re = new RegExp(`impl(?:\\s*<[^>]*>)?\\s+(?:[\\w:]+::)*${traitName}\\s+for\\s+${typeName}\\b`);
  return re.test(content);
}

function documentImplBlock(typeName, pathPrefix) {
  // pathPrefix: "store::" for plugins, "crate::os_store::" or "super::" etc.
  const S = pathPrefix;
  const D = pathPrefix === "store::" ? "dsl::" : "crate::os_dsl::";
  return `
//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's \`__dsl_*\` helpers + parse/print, not derive emission.
impl ${S}DocumentDsl for ${typeName} {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, ${S}TextError> {
        let body = match ${S}semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = ${D}parse(
            body,
            &Self::__dsl_spec(),
            &${D}ParseOptions { limits: ${D}Limits::default(), mode: ${D}SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = ${D}print(&self.__dsl_to_record(), &Self::__dsl_spec(), ${D}JoinMode::Document);
        let envelope = ${S}semio_format::SemioEnvelope::from_envelope_id(
            <Self as ${S}DocumentDsl>::envelope_id(),
            ${S}semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        ${S}semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via \`__dsl_*\` record lowering.
impl ${S}DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &${S}PackEncodeOptions) -> Result<Vec<u8>, ${S}PackError> {
        let inner = ${S}pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = ${S}semio_format::SemioEnvelope::from_envelope_id(
            <Self as ${S}DocumentDsl>::envelope_id(),
            ${S}semio_format::Component::Pack,
            1,
        )
        .map_err(|e| ${S}PackError::Schema(e.to_string()))?;
        Ok(${S}semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &${S}PackDecodeOptions) -> Result<Self, ${S}PackError> {
        let (envelope, inner) = ${S}semio_format::unwrap_binary(bytes).map_err(|e| ${S}PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as ${S}DocumentDsl>::envelope_id() {
            return Err(${S}PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as ${S}DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = ${S}pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(${S}text_error_to_pack_error)
    }
    fn record_spec() -> Option<${D}RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec
`;
}

function opsImplBlock(typeName, pathPrefix) {
  const S = pathPrefix; // store::
  const D = pathPrefix === "store::" ? "dsl::" : "crate::os_dsl::";
  const P = pathPrefix === "store::" ? "protocol::" : "crate::os_spr::";
  // For store-internal: use crate paths
  const isStore = pathPrefix === "crate::" || pathPrefix.startsWith("crate::");
  const storePath = pathPrefix === "store::" ? "store::" : "";
  const dslPath = pathPrefix === "store::" ? "dsl::" : "crate::os_dsl::";
  const protoPath = pathPrefix === "store::" ? "protocol::" : "crate::os_spr::";
  const packPath = pathPrefix === "store::" ? "store::pack_rt::" : "crate::os_pack::";
  const textErr = pathPrefix === "store::" ? "store::TextError" : "crate::os_store::TextError";
  // Actually simplify: detect mode
  let dsl, store, proto, packRt, packCore, textError;
  if (pathPrefix === "plugin") {
    dsl = "dsl::"; store = "store::"; proto = "protocol::"; packRt = "store::pack_rt::"; packCore = "store::pack_rt::"; textError = "store::TextError";
  } else if (pathPrefix === "store") {
    dsl = "crate::os_dsl::"; store = ""; proto = "crate::os_spr::"; packRt = "pack_rt::"; packCore = "crate::os_pack::"; textError = "TextError";
  } else {
    throw new Error("bad prefix");
  }

  const docTrait = pathPrefix === "plugin" ? "store::DocumentDsl" : "DocumentDsl";
  const packTrait = pathPrefix === "plugin" ? "store::DocumentPack" : "DocumentPack";
  // For ops:
  const opText = pathPrefix === "plugin" ? "protocol::OpText" : "OpText";
  const opBin = pathPrefix === "plugin" ? "protocol::OpBinary" : "OpBinary";
  const protoErr = pathPrefix === "plugin" ? "protocol::ProtocolError" : "crate::os_spr::ProtocolError";

  return `
//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6): DslVariants keyword probe + inline parse/print.
impl ${opText} for ${typeName} {
    fn parse_op(line: &str) -> Result<Self, ${textError}> {
        let variants = <Self as ${dsl}DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = ${dsl}parse(
                    line,
                    &spec_fn(),
                    &${dsl}ParseOptions { limits: ${dsl}Limits::default(), mode: ${dsl}SourceMode::Inline },
                )?;
                return <Self as ${dsl}DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(${dsl}__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as ${dsl}DslVariants>::to_named_record(self);
        let variants = <Self as ${dsl}DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        ${dsl}print(&record, &spec_fn(), ${dsl}JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6): \`format u8 | ordinal varint | record body\` against DslVariants order.
impl ${opBin} for ${typeName} {
    fn encode_op(&self) -> Result<Vec<u8>, ${protoErr}> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as ${dsl}DslVariants>::to_named_record(self);
        let variants = <Self as ${dsl}DslVariants>::variants();
        let ordinal = variants
            .iter()
            .position(|(k, _)| *k == keyword)
            .ok_or(${protoErr}::Malformed {
                what: "op variant",
                offset: 0,
                detail: format!("keyword {keyword:?} is not a declared variant"),
            })?;
        let spec = (variants[ordinal].1)();
        let body = ${packCore}encode_record_body(&spec, &record, &${pathPrefix === "plugin" ? "store::PackEncodeOptions" : "PackEncodeOptions"}::default())
            .map_err(${protoErr}::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        ${packCore}write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ${protoErr}> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = ${packCore}ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(${protoErr}::Malformed {
                what: "op format",
                offset: 0,
                detail: format!("unsupported op format {format}"),
            });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as ${dsl}DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(${protoErr}::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = ${packCore}decode_record_body(body, &spec, &${pathPrefix === "plugin" ? "store::PackDecodeOptions" : "PackDecodeOptions"}::default())
            .map_err(${protoErr}::from)?;
        <Self as ${dsl}DslVariants>::from_named_record(keyword, &record).map_err(|error| ${protoErr}::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec
`;
}

function injectAfterType(content, typeName, block) {
  const idx = content.search(new RegExp(`(?:pub\\s+)?(?:struct|enum)\\s+${typeName}\\b`));
  if (idx < 0) return null;
  let i = content.indexOf("{", idx);
  if (i < 0) {
    const semi = content.indexOf(";", idx);
    if (semi < 0) return null;
    return content.slice(0, semi + 1) + "\n" + block + content.slice(semi + 1);
  }
  let depth = 0;
  for (; i < content.length; i++) {
    const c = content[i];
    if (c === "{") depth++;
    else if (c === "}") {
      depth--;
      if (depth === 0) {
        let end = i + 1;
        if (content[end] === ";") end++;
        return content.slice(0, end) + "\n" + block + content.slice(end);
      }
    }
  }
  return null;
}

function processFile(absPath, mode) {
  let content = readFileSync(absPath, "utf8");
  const rel = relative(repo, absPath);
  let changed = false;
  const deriveRe = /#\[derive\s*\(([^)]*)\)\]/g;
  let m;
  const sites = [];
  while ((m = deriveRe.exec(content)) !== null) {
    const attrs = m[1];
    const hasDoc = /\b(?:dsl::|crate::os_dsl::)?DslDocument\b/.test(attrs);
    const hasOps = /\b(?:dsl::|crate::os_dsl::)?DslOps\b/.test(attrs);
    if (!hasDoc && !hasOps) continue;
    const info = findTypeAfterDerive(content, m.index);
    if (!info) {
      report.errors.push({ rel, err: "resolve type failed", attrs });
      continue;
    }
    sites.push({ ...info, hasDoc, hasOps });
  }

  for (const site of sites) {
    if (site.hasDoc) {
      if (hasImplFor(content, "DocumentDsl", site.name)) {
        report.skippedDocs.push({ rel, name: site.name });
      } else {
        const block = mode === "plugin"
          ? documentImplBlock(site.name, "store::").replaceAll("store::DocumentDsl", "store::DocumentDsl").replaceAll("dsl::", "dsl::")
          : documentImplBlockStore(site.name);
        // rebuild document block properly
        const b = mode === "plugin" ? documentImplBlockPlugin(site.name) : documentImplBlockStore(site.name);
        const next = injectAfterType(content, site.name, b);
        if (!next) report.errors.push({ rel, err: "inject doc failed", name: site.name });
        else {
          content = next;
          changed = true;
          report.injectedDocs.push({ rel, name: site.name });
        }
      }
    }
    if (site.hasOps) {
      if (hasImplFor(content, "OpText", site.name)) {
        report.skippedOps.push({ rel, name: site.name });
      } else {
        const b = mode === "plugin" ? opsImplBlockPlugin(site.name) : opsImplBlockStore(site.name);
        const next = injectAfterType(content, site.name, b);
        if (!next) report.errors.push({ rel, err: "inject ops failed", name: site.name });
        else {
          content = next;
          changed = true;
          report.injectedOps.push({ rel, name: site.name });
        }
      }
    }
  }
  if (changed) writeFileSync(absPath, content);
}

function documentImplBlockPlugin(typeName) {
  return `
//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's \`__dsl_*\` helpers + parse/print, not derive emission.
impl store::DocumentDsl for ${typeName} {
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
impl store::DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec
`;
}

function documentImplBlockStore(typeName) {
  return `
//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6).
impl DocumentDsl for ${typeName} {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = crate::os_dsl::parse(
            body,
            &Self::__dsl_spec(),
            &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = semio_format::SemioEnvelope::from_envelope_id(
            <Self as DocumentDsl>::envelope_id(),
            semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6).
impl DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(
            <Self as DocumentDsl>::envelope_id(),
            semio_format::Component::Pack,
            1,
        )
        .map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as DocumentDsl>::envelope_id() {
            return Err(PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec
`;
}

function opsImplBlockPlugin(typeName) {
  return `
//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for ${typeName} {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for ${typeName} {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec
`;
}

function opsImplBlockStore(typeName) {
  return `
//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for ${typeName} {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for ${typeName} {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec
`;
}

// --- patch store pack_rt + remove op_rt reexport ---
{
  let t = readFileSync(STORE_FILE, "utf8");

  // Add helpers to pack_rt after decode_document
  const packHook = `    /// @emoji 🚪️ Forwards to \`crate::os_pack::decode_document\`.
    pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_document(bytes, spec, options)
    }
`;
  const packHookRepl = packHook + `
    /// @emoji 🎯️ P6: container-less record body helpers for handcrafted OpBinary impls.
    pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        crate::os_pack::encode_record_body(spec, record, options)
    }
    pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_record_body(bytes, spec, options)
    }
    pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
        crate::os_pack::write_varint_u64(out, value)
    }
    pub use crate::os_pack::ByteReader;
    /// @emoji 🎯️ Format byte every encoded operation starts with (handcrafted OpBinary convention).
    pub const OP_BINARY_FORMAT: u8 = 1;
`;
  if (!t.includes("pub fn encode_record_body(spec: &RecordSpec")) {
    t = replaceOnce(t, packHook, packHookRepl, "pack_rt helpers");
  }

  // Replace op_rt reexport
  const opRtRegion = `//#region 🔖️OpRt
/// @emoji 🎯️ Facade re-export of the \`OpBinary\` runtime (\`format u8 | variant ordinal
/// varint | record body\`) — the op-level mirror of \`pack_rt\` behind \`DocumentPack\`. Hosted in
/// \`dsl\` (the crate that owns the \`DslVariants\` bound) rather than here so \`dsl\`'s own test build
/// binds the same trait instance; re-exported so apps keep the one-facade rule (\`crate::os_store::op_rt\`).
pub use crate::os_dsl::op_rt;
//#endregion 🔖️OpRt
`;
  const opRtRepl = `//#region 🔖️OpRt
// P6: dsl::op_rt deleted. Handcrafted OpBinary impls use pack_rt record-body helpers; see OP_BINARY_FORMAT.
//#endregion 🔖️OpRt
`;
  t = replaceOnce(t, opRtRegion, opRtRepl, "store op_rt reexport");

  // Fix tests that call op_rt::
  t = t.replaceAll("op_rt::encode_op(&operation)", "operation.encode_op()");
  t = t.replaceAll("op_rt::encode_op(&operation)", "operation.encode_op()"); // noop if done
  t = t.replaceAll("let encoded = op_rt::encode_op(&operation).expect(\"op encode\");", "let encoded = operation.encode_op().expect(\"op encode\");");
  t = t.replaceAll("let encoded_again = op_rt::encode_op(&operation).expect(\"op re-encode\");", "let encoded_again = operation.encode_op().expect(\"op re-encode\");");
  t = t.replaceAll("assert_eq!(encoded[0], op_rt::OP_BINARY_FORMAT);", "assert_eq!(encoded[0], pack_rt::OP_BINARY_FORMAT);");
  t = t.replaceAll("let decoded: DemoOperation = op_rt::decode_op(&encoded).expect(\"op decode\");", "let decoded = DemoOperation::decode_op(&encoded).expect(\"op decode\");");
  t = t.replaceAll("let mut wrong_format = op_rt::encode_op(&operation).expect(\"op encode\");", "let mut wrong_format = operation.encode_op().expect(\"op encode\");");
  t = t.replaceAll("assert!(op_rt::decode_op::<DemoOperation>(&wrong_format).is_err(), \"format 9 must be rejected\");", "assert!(DemoOperation::decode_op(&wrong_format).is_err(), \"format 9 must be rejected\");");
  t = t.replaceAll("let out_of_range = [op_rt::OP_BINARY_FORMAT, 0x7E];", "let out_of_range = [pack_rt::OP_BINARY_FORMAT, 0x7E];");
  t = t.replaceAll("assert!(op_rt::decode_op::<DemoOperation>(&out_of_range).is_err(), \"ordinal beyond declared variants must be rejected\");", "assert!(DemoOperation::decode_op(&out_of_range).is_err(), \"ordinal beyond declared variants must be rejected\");");

  writeFileSync(STORE_FILE, t);
  report.store = { patched: true };
}

// Process plugins
for (const f of walkRs(PLUGINS)) {
  try {
    if (!relative(repo, f).includes("/🗿️artifacts/")) continue;
    processFile(f, "plugin");
  } catch (e) {
    report.errors.push({ file: relative(repo, f), err: String(e) });
  }
}

// Process store
try {
  processFile(STORE_FILE, "store");
} catch (e) {
  report.errors.push({ file: "store", err: String(e) });
}

// Process dsl component tests (DerivedDocument etc.)
{
  const dslComp = join(DSL_DIR, "🦀️component.rs");
  try {
    processFile(dslComp, "store"); // uses crate::os_* paths — WRONG for dsl crate
  } catch (e) {
    report.errors.push({ file: "dsl", err: String(e), note: "dsl may need separate path mode" });
  }
}

// Empty exemptions + retarget policy
{
  let script = readFileSync(SCRIPT_TS, "utf8");
  function emptySet(name) {
    const re = new RegExp(`(const ${name}[^=]*=\\s*new Set(?:<[^>]*>)?\\(\\s*\\[)([\\s\\S]*?)(\\]\\s*\\))`);
    if (!re.test(script)) throw new Error("set not found: " + name);
    script = script.replace(re, "$1$3");
  }
  for (const name of [
    "POLICY_SPEC_DISTINCTNESS_EXEMPTIONS",
    "POLICY_GENERIC_SPEC_EXEMPTIONS",
    "POLICY_DECLARED_USE_EXEMPTIONS",
    "POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS",
    "POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS",
    "POLICY_EMPTY_EXAMPLE_EXEMPTIONS",
    "POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS",
  ]) emptySet(name);

  const oldFnStart = script.indexOf("function policyGenericCodecDeriveBreaches");
  const oldFnEnd = script.indexOf("/** ⚖️Aggregates all P3/M4 handcrafted-grammar", oldFnStart);
  if (oldFnStart < 0 || oldFnEnd < 0) throw new Error("policy fn markers missing");
  const newFn = `function policyGenericCodecDeriveBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s/🔌️plugins"], (relPath, name) => {
    return relPath.includes("/🗿️artifacts/") && name.endsWith(".rs");
  });
  const banned = [
    { re: /dsl::__rt::parse_document_record|dsl::__rt::print_document_record|dsl::__rt::parse_inline_record|dsl::__rt::print_inline_record/g, label: "__rt codec wrapper" },
    { re: /dsl::op_rt::|store::op_rt::/g, label: "op_rt generic OpBinary" },
  ];
  for (const relPath of files) {
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    for (const { re, label } of banned) {
      re.lastIndex = 0;
      let match: RegExpExecArray | null;
      while ((match = re.exec(content)) !== null) {
        const before = content.slice(0, match.index);
        const line = before.split(/\\r?\\n/).length;
        breaches.push({
          id: \`generic-codec-runtime-\${relPath}-\${line}\`,
          summary: \`Residual generic codec path (\${label}) in "\${relPath}" (line \${line})\`,
          kind: "handcrafted-grammar/generic-codec-derive",
          scope: relPath,
          line,
          priority: "high",
          reason: "P6 deleted derive-emitted DocumentDsl/OpText/DocumentPack/OpBinary and their __rt/op_rt entrypoints; artifacts must use handcrafted codecs.",
          solution: \`Replace \${label} usage in \${relPath} with the artifact's handcrafted DocumentDsl/OpText/DocumentPack/OpBinary impl.\`,
        });
      }
    }
  }
  return breaches;
}

`;
  script = script.slice(0, oldFnStart) + newFn + script.slice(oldFnEnd);
  writeFileSync(SCRIPT_TS, script);
  report.policy = { emptied: true, retargeted: true };
}

writeFileSync(join(ticket, "🧪p6-inject-report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({
  injectedDocs: report.injectedDocs.length,
  injectedOps: report.injectedOps.length,
  skippedDocs: report.skippedDocs.length,
  skippedOps: report.skippedOps.length,
  errors: report.errors.length,
  errorSample: report.errors.slice(0, 8),
}, null, 2));
