#!/usr/bin/env bun
/** [DEBUG] P6 flag day: handcraft DocumentDsl/DocumentPack/OpText/OpBinary for all former exemptions. */
import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
const paths = JSON.parse(readFileSync(join(ticket, "🧪p6-exempt-paths.json"), "utf8"));
const log = [];
const note = (m) => {
  log.push(m);
  console.log(m);
};

function findMatchingBrace(src, openIdx) {
  let depth = 0;
  for (let i = openIdx; i < src.length; i++) {
    if (src[i] === "{") depth++;
    else if (src[i] === "}") {
      depth--;
      if (depth === 0) return i;
    }
  }
  return -1;
}

function extractContainerAttrs(text) {
  const attrs = [...text.matchAll(/#\[dsl\(([^)]*)\)\]/g)].map((m) => m[1]).join(", ");
  const id = attrs.match(/\bid\s*=\s*"([^"]+)"/)?.[1];
  const extension = attrs.match(/\bextension\s*=\s*"([^"]+)"/)?.[1];
  const envelopeId = id ?? extension ?? null;
  const extensionSuffix = envelopeId ? envelopeId.split(".").pop() : null;
  return { envelopeId, extensionSuffix };
}

function documentImpls(typeName, envelopeId, extensionSuffix) {
  return `
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for ${typeName} {
    const EXTENSION: &'static str = ${JSON.stringify(extensionSuffix)};
    fn envelope_id() -> &'static str { ${JSON.stringify(envelopeId)} }
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
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for ${typeName} {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs
`;
}

function opsImpls(typeName) {
  return `
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
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

impl protocol::OpBinary for ${typeName} {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
`;
}

function patchFile(relPath) {
  const abs = join(repo, relPath);
  if (!existsSync(abs)) {
    note("MISSING " + relPath);
    return { ok: false };
  }
  const src = readFileSync(abs, "utf8");
  const deriveRe = /#\[derive\s*\(([^)]*)\)\]/gs;
  let match;
  let out = "";
  let last = 0;
  let changed = false;
  const injected = [];
  while ((match = deriveRe.exec(src)) !== null) {
    const inner = match[1];
    const hasDoc = /\b(?:dsl::)?DslDocument\b/.test(inner);
    const hasOps = /\b(?:dsl::)?DslOps\b/.test(inner);
    if (!hasDoc && !hasOps) continue;
    const newInner = inner
      .replace(/\b(?:dsl::)?DslDocument\b/g, "dsl::DslRecord")
      .replace(/\b(?:dsl::)?DslOps\b/g, "dsl::DslEnum");
    const deriveStart = match.index;
    const deriveEnd = match.index + match[0].length;
    const after = src.slice(deriveEnd);
    const typeMatch = after.match(/^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s+)?(struct|enum)\s+([A-Za-z0-9_]+)/);
    if (!typeMatch) {
      out += src.slice(last, deriveStart) + `#[derive(${newInner})]`;
      last = deriveEnd;
      changed = true;
      continue;
    }
    const typeName = typeMatch[4];
    const typeHeaderOffset = deriveEnd + typeMatch[0].length;
    const braceOpen = src.indexOf("{", typeHeaderOffset - 1);
    const braceClose = findMatchingBrace(src, braceOpen);
    if (braceClose < 0) {
      note("WARN unmatched brace " + typeName + " in " + relPath);
      out += src.slice(last, deriveStart) + `#[derive(${newInner})]`;
      last = deriveEnd;
      changed = true;
      continue;
    }
    const between = src.slice(deriveEnd, deriveEnd + typeMatch.index + typeMatch[0].length);
    const { envelopeId, extensionSuffix } = extractContainerAttrs(between);
    const afterType = src.slice(braceClose + 1, braceClose + 120);
    const already =
      afterType.includes("//#region 🔖️HandcraftedDocumentCodecs") ||
      afterType.includes("//#region 🔖️HandcraftedOpCodecs");
    let implBlock = "";
    if (!already) {
      if (hasDoc) {
        const env = envelopeId ?? typeName.toLowerCase();
        const ext = extensionSuffix ?? env.split(".").pop();
        implBlock += documentImpls(typeName, env, ext);
        injected.push("doc:" + typeName);
      }
      if (hasOps) {
        implBlock += opsImpls(typeName);
        injected.push("ops:" + typeName);
      }
    }
    out += src.slice(last, deriveStart) + `#[derive(${newInner})]`;
    out += src.slice(deriveEnd, braceClose + 1);
    out += implBlock;
    last = braceClose + 1;
    changed = true;
  }
  out += src.slice(last);
  if (!changed) {
    note("NO-CHANGE " + relPath);
    return { ok: true, reason: "no-change" };
  }
  writeFileSync(abs, out);
  note("OK " + relPath + " :: " + (injected.join(",") || "derive-swap"));
  return { ok: true, injected };
}

const results = paths.map((rel) => ({ rel, ...patchFile(rel) }));
writeFileSync(join(ticket, "🧪p6-handcraft-log.json"), JSON.stringify({ results, log }, null, 2));
note("done " + results.filter((r) => r.ok).length + "/" + results.length);
