#!/usr/bin/env bun
/** [DEBUG] P6: strip DocumentDsl/DocumentPack/OpText/OpBinary from dsl_derive. */
import { readFileSync, writeFileSync, existsSync, readdirSync } from "fs";
import { join, relative } from "path";

const repo = "/Users/ueli/Documents/semio";
const DSL_DIR = (() => {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl");
    if (existsSync(p)) return p;
  }
  throw new Error("dsl dir not found");
})();

const files = [
  join(DSL_DIR, "✨️derive", "🦀️component.rs"),
  join(DSL_DIR, "✨️derive", "📦️packages", "🦀️rust", "📦️glue.rs"),
];

function replaceOnce(hay, needle, repl, label) {
  const i = hay.indexOf(needle);
  if (i < 0) throw new Error("missing needle: " + label + "\n---\n" + needle.slice(0, 200));
  const j = hay.indexOf(needle, i + 1);
  if (j >= 0) throw new Error("needle not unique: " + label);
  return hay.slice(0, i) + repl + hay.slice(i + needle.length);
}

function patch(src) {
  let t = src;

  const helpersCloseAndDoc = `            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::store::DocumentDsl for #name {
            const EXTENSION: &'static str = #extension_suffix_lit;
            fn envelope_id() -> &'static str {
                #envelope_id_lit
            }
            fn parse_dsl(text: &str) -> Result<Self, ::store::TextError> {
                let body = match ::store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let record = ::dsl::__rt::parse_document_record(body, &Self::__dsl_spec())?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = ::dsl::__rt::print_document_record(&self.__dsl_to_record(), &Self::__dsl_spec());
                let envelope = ::store::semio_format::SemioEnvelope::from_envelope_id(
                    <Self as ::store::DocumentDsl>::envelope_id(),
                    ::store::semio_format::Component::Dsl,
                    1,
                ).expect("valid envelope_id");
                ::store::semio_format::wrap_text(&envelope, &body)
            }
        }
`;

  const helpersCloseRepl = `            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
            /// ✉️ Envelope constants for handcrafted DocumentDsl/DocumentPack wiring (P6: derive no longer emits those traits).
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id_lit;
            pub const __DSL_EXTENSION: &'static str = #extension_suffix_lit;
        }
`;
  t = replaceOnce(t, helpersCloseAndDoc, helpersCloseRepl, "DslDocument helpers+DocumentDsl");

  const packNeedle = `
        // 📦️ Binary counterpart of the \`store::DocumentDsl\` impl above — same \`__dsl_spec\`/
        // \`__dsl_to_record\`/\`__dsl_from_record\` trio, routed through \`pack\` instead of the DSL
        // grammar engine. \`store::text_error_to_pack_error\` (a free function, not \`PackError: From
        // <TextError>\` — that impl is an orphan-rule violation since neither type is local to
        // \`store\`) bridges \`__dsl_from_record\`'s \`TextError\` into \`PackError\`.
        impl ::store::DocumentPack for #name {
            fn encode_pack_with(&self, options: &::store::PackEncodeOptions) -> Result<Vec<u8>, ::store::PackError> {
                let inner = ::store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = ::store::semio_format::SemioEnvelope::from_envelope_id(
                    <Self as ::store::DocumentDsl>::envelope_id(),
                    ::store::semio_format::Component::Pack,
                    1,
                ).map_err(|e| ::store::PackError::Schema(e.to_string()))?;
                Ok(::store::semio_format::wrap_binary(&envelope, &inner))
            }
            fn decode_pack_with(bytes: &[u8], options: &::store::PackDecodeOptions) -> Result<Self, ::store::PackError> {
                let (envelope, inner) = ::store::semio_format::unwrap_binary(bytes)
                    .map_err(|e| ::store::PackError::Schema(e.to_string()))?;
                if envelope.envelope_id() != <Self as ::store::DocumentDsl>::envelope_id() {
                    return Err(::store::PackError::Schema(format!(
                        "pack envelope mismatch: expected {}, got {}",
                        <Self as ::store::DocumentDsl>::envelope_id(),
                        envelope.envelope_id()
                    )));
                }
                let (record, _report) = ::store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(::store::text_error_to_pack_error)
            }
            fn record_spec() -> Option<::dsl::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }
`;
  t = replaceOnce(t, packNeedle, "\n", "DocumentPack emission");

  t = t.replaceAll(
    "::dsl::__rt::print_inline_record(&self.__dsl_diff_to_record(), &Self::__dsl_diff_spec())",
    "::dsl::print(&self.__dsl_diff_to_record(), &Self::__dsl_diff_spec(), ::dsl::JoinMode::Inline)",
  );
  t = t.replaceAll(
    "let record = ::dsl::__rt::parse_inline_record(line, &Self::__dsl_diff_spec())?;",
    "let record = ::dsl::parse(line, &Self::__dsl_diff_spec(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;",
  );

  const opsNeedle = `    let expanded = quote! {
        #variants_impl

        // 🎞️ \`OpText\` lives in \`protocol_command\`, re-exported as \`protocol::OpText\` — every
        // \`#[derive(dsl::DslOps)]\` crate depends on \`protocol\` directly for its \`Operation\` impl
        // anyway, so this resolves without new Cargo.toml deps. The error type stays
        // \`::store::TextError\` (a transparent re-export of \`dsl_core::TextError\`, the exact type
        // \`protocol::OpText::parse_op\` declares) rather than switching to \`::dsl_core::TextError\`
        // directly, since not every deriving crate has \`dsl_core\` as a *direct* dependency.
        impl ::protocol::OpText for #name {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{} ", keyword);
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let record = ::dsl::__rt::parse_inline_record(line, &spec_fn())?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                ::dsl::__rt::print_inline_record(&record, &spec_fn())
            }
        }

        // 🎞️ Binary twin of the \`OpText\` impl above — same \`DslVariants\` lowering, byte layout
        // owned by \`::dsl::op_rt\` (\`format u8 | variant ordinal varint | record body\`), the op-level
        // mirror of the \`DocumentDsl\`/\`DocumentPack\` pairing. Resolves through \`dsl\` (not \`store\`)
        // because the runtime's bound is \`dsl::DslVariants\` itself — see \`dsl::op_rt\`'s doc.
        impl ::protocol::OpBinary for #name {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::op_rt::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::op_rt::decode_op(bytes)
            }
        }
    };
    expanded.into()
`;

  const opsRepl = `    // P6: DslOps emits DslVariants only — OpText/OpBinary must be handcrafted per artifact.
    variants_impl.into()
`;
  t = replaceOnce(t, opsNeedle, opsRepl, "DslOps OpText/OpBinary emission");

  // Update crate doc first lines
  t = t.replace(
    "`store::DocumentDsl`/`protocol::OpText` implementations (and the `dsl::DslField`/`dsl::DslVariants`",
    "`dsl::DslField`/`dsl::DslVariants`",
  );

  const checks = [
    ["DocumentDsl", "impl ::store::DocumentDsl for #name"],
    ["DocumentPack", "impl ::store::DocumentPack for #name"],
    ["OpText", "impl ::protocol::OpText for #name"],
    ["OpBinary", "impl ::protocol::OpBinary for #name"],
    ["op_rt", "op_rt::"],
    ["parse_document_record", "parse_document_record"],
    ["print_document_record", "print_document_record"],
    ["parse_inline_record", "parse_inline_record"],
    ["print_inline_record", "print_inline_record"],
  ];
  for (const [label, needle] of checks) {
    if (t.includes(needle)) throw new Error("still present after patch: " + label);
  }
  if (!t.includes("__DSL_ENVELOPE_ID")) throw new Error("envelope constants missing");
  return t;
}

for (const f of files) {
  const before = readFileSync(f, "utf8");
  const after = patch(before);
  writeFileSync(f, after);
  console.log("patched", relative(repo, f), before.length, "->", after.length);
}
