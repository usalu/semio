#!/usr/bin/env bun
/** [DEBUG] P6: inject codecs ignoring comment false-positives for existing impls. */
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
const PLUGINS = join(repo, "✏️s", "🔌️plugins");

function findModule(emojiName) {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", emojiName);
    try { if (statSync(p).isDirectory()) return p; } catch {}
  }
  throw new Error("missing " + emojiName);
}
const STORE_FILE = join(findModule("🏪️store"), "🦀️component.rs");
const DSL_FILE = join(findModule("🗣️dsl"), "🦀️component.rs");

const report = { injectedDocs: [], injectedOps: [], skippedDocs: [], skippedOps: [], errors: [], missingBefore: 0 };

function walk(dir, out = []) {
  for (const n of readdirSync(dir)) {
    if (n === "target" || n === "node_modules") continue;
    const p = join(dir, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (n.endsWith(".rs")) out.push(p);
  }
  return out;
}

function codeLines(content) {
  return content.split(/\r?\n/).map((line) => {
    const t = line.trimStart();
    if (t.startsWith("//")) return "";
    // strip trailing line comments roughly
    const idx = line.indexOf("//");
    if (idx >= 0 && !line.slice(0, idx).includes('"')) return line.slice(0, idx);
    return line;
  }).join("\n");
}

function hasImplFor(content, traitName, typeName) {
  const code = codeLines(content);
  return new RegExp(`impl(?:\\s*<[^>]*>)?\\s+(?:[\\w:]+::)*${traitName}\\s+for\\s+${typeName}\\b`).test(code);
}

function findSites(content) {
  const sites = [];
  const lines = content.split(/\r?\n/);
  for (let li = 0; li < lines.length; li++) {
    const trimmed = lines[li].trimStart();
    if (trimmed.startsWith("//")) continue;
    const m = trimmed.match(/^#\[derive\s*\(([^)]*)\)\]/);
    if (!m) continue;
    const attrs = m[1];
    const hasDoc = /\b(?:dsl::|crate::os_dsl::)?DslDocument\b/.test(attrs);
    const hasOps = /\b(?:dsl::|crate::os_dsl::)?DslOps\b/.test(attrs);
    if (!hasDoc && !hasOps) continue;
    let name = null;
    for (let j = li + 1; j < Math.min(lines.length, li + 12); j++) {
      const t = lines[j].trim();
      if (t.startsWith("#[") || t.startsWith("//") || t === "") continue;
      const tm = t.match(/^(?:pub(?:\([^)]*\))?\s+)?(struct|enum)\s+([A-Za-z0-9_]+)/);
      if (tm) { name = tm[2]; break; }
      break;
    }
    if (name) sites.push({ hasDoc, hasOps, name, line: li + 1 });
    else report.errors.push({ line: li + 1, attrs, err: "no type" });
  }
  return sites;
}

function injectAfterType(content, typeName, block) {
  const idx = content.search(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?(?:struct|enum)\\s+${typeName}\\b`));
  if (idx < 0) return null;
  let i = content.indexOf("{", idx);
  if (i < 0) {
    const semi = content.indexOf(";", idx);
    if (semi < 0) return null;
    return content.slice(0, semi + 1) + "\n" + block + content.slice(semi + 1);
  }
  let depth = 0;
  for (; i < content.length; i++) {
    if (content[i] === "{") depth++;
    else if (content[i] === "}") {
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

function documentImplBlockPlugin(typeName) {
  return `
//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's \`__dsl_*\` helpers + parse/print, not derive emission.
impl store::DocumentDsl for ${typeName} {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) { Ok((_, rest)) => rest, Err(_) => text };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::DocumentDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📦️ Handcrafted DocumentPack (P6).
impl store::DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::DocumentDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::DocumentDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
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
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
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
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
`;
}

function documentImplBlockStore(typeName) {
  return `
//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6).
impl DocumentDsl for ${typeName} {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_format::split_text_preamble(text) { Ok((_, rest)) => rest, Err(_) => text };
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as DocumentDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_format::wrap_text(&envelope, &body)
    }
}
/// 📦️ Handcrafted DocumentPack (P6).
impl DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as DocumentDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as DocumentDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as DocumentDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️DocumentCodec
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
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
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
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
`;
}

function processFile(absPath, mode) {
  let content = readFileSync(absPath, "utf8");
  const rel = relative(repo, absPath);
  let changed = false;
  for (const site of findSites(content)) {
    if (site.hasDoc) {
      if (hasImplFor(content, "DocumentDsl", site.name)) report.skippedDocs.push({ rel, name: site.name });
      else {
        const b = mode === "plugin" ? documentImplBlockPlugin(site.name) : documentImplBlockStore(site.name);
        const next = injectAfterType(content, site.name, b);
        if (!next) report.errors.push({ rel, err: "inject doc failed", name: site.name });
        else { content = next; changed = true; report.injectedDocs.push({ rel, name: site.name }); }
      }
    }
    if (site.hasOps) {
      if (hasImplFor(content, "OpText", site.name)) report.skippedOps.push({ rel, name: site.name });
      else {
        const b = mode === "plugin" ? opsImplBlockPlugin(site.name) : opsImplBlockStore(site.name);
        const next = injectAfterType(content, site.name, b);
        if (!next) report.errors.push({ rel, err: "inject ops failed", name: site.name });
        else { content = next; changed = true; report.injectedOps.push({ rel, name: site.name }); }
      }
    }
  }
  if (changed) writeFileSync(absPath, content);
}

for (const f of walk(PLUGINS)) {
  if (!relative(repo, f).includes("/🗿️artifacts/")) continue;
  try { processFile(f, "plugin"); } catch (e) { report.errors.push({ file: relative(repo, f), err: String(e) }); }
}
try { processFile(STORE_FILE, "store"); } catch (e) { report.errors.push({ file: "store", err: String(e) }); }

writeFileSync(join(ticket, "🧪p6-inject-fix-report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({
  injectedDocs: report.injectedDocs.length,
  injectedOps: report.injectedOps.length,
  skippedDocs: report.skippedDocs.length,
  skippedOps: report.skippedOps.length,
  errors: report.errors.length,
  injectedDocsSample: report.injectedDocs.slice(0, 20),
  injectedOpsSample: report.injectedOps.slice(0, 20),
  errorsSample: report.errors.slice(0, 10),
}, null, 2));
