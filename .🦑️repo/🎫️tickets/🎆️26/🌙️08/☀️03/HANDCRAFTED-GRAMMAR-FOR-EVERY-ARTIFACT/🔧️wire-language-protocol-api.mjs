#!/usr/bin/env bun
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function findTicket() {
  const month = join(".🦑️repo/🎫️tickets/🎆️26");
  for (const m of readdirSync(month)) {
    const p = join(month, m, "☀️03", "HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT");
    try {
      if (statSync(p).isDirectory()) return p;
    } catch {}
  }
  throw new Error("ticket not found");
}

const ticket = findTicket();
const fw = readdirSync(".").find((x) => x.includes("framework"));
const dsl = join(fw, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl");
const paths = [join(dsl, "🦀️component.rs"), join(dsl, "⚡️implementations/🦀️rust/📦️lib.rs")];

const ROLE_DOC = [
  "/// @emoji 🎭️ Which surface a registered [`LanguageSpec`] describes for the",
  "/// `handcrafted-grammar-for-every-artifact` program.",
  "///",
  "/// Text roles carry a `.grammar.semio` (`grammar` / `grammar_path`): `Document` (`🗣️dsl`),",
  "/// `Config`, `Ops` (`🔧️op`), `Embedded` (`Shape::Embed` idiom), and `Diff` (`🔺️diff`).",
  "/// Binary roles carry a `.protocol.semio` (`protocol` / `protocol_path`): `Pack` (`🎒️pack`)",
  "/// and `Spr` (`📡️spr`). Never put grammar files on pack/spr or protocol files on dsl/op/diff.",
].join("\n");

const SPEC_DOC = [
  "/// @emoji 📖️ One artifact facet language, registered once at plugin init: identity, the extension",
  "/// it opens (documents/configs only), optional hand-authored **grammar** text for text surfaces",
  "/// (`🗣️dsl` / `🔧️op` / `🔺️diff`, `dialect grammar`), optional hand-authored **protocol** text for",
  "/// binary surfaces (`🎒️pack` / `📡️spr`, `dialect protocol`), and the [`IdiomHooks`] vtable used by",
  "/// text hosts (`LanguageSession`, writer). Additive alongside `IdiomHooks`/`register_idiom`.",
].join("\n");

const IMPL_BLOCK = `impl LanguageSpec {
    /// @emoji 🧬️ Services a facet still on generic \`RecordSpec\`/\`DocumentDsl\` until its handcrafted
    /// \`.semio\` spec lands — same hooks/specs as the parent, distinct registry id.
    pub fn derived(parent: LanguageSpec, id: &'static str, role: LanguageRole) -> Self {
        Self {
            id,
            extension: None,
            role,
            grammar: parent.grammar,
            grammar_path: parent.grammar_path,
            protocol: parent.protocol,
            protocol_path: parent.protocol_path,
            hooks: parent.hooks,
        }
    }

    /// @emoji 📝 Whether this role is a text grammar surface (dsl/op/diff/config/embed).
    pub fn is_text_role(self) -> bool {
        matches!(self.role, LanguageRole::Document | LanguageRole::Config | LanguageRole::Ops | LanguageRole::Embedded | LanguageRole::Diff)
    }

    /// @emoji 📡️ Whether this role is a binary protocol surface (pack/spr).
    pub fn is_binary_role(self) -> bool {
        matches!(self.role, LanguageRole::Pack | LanguageRole::Spr)
    }

    /// @emoji 📖️ Parses \`grammar\` via [\`parse_grammar\`], requiring [\`SemioDialect::Grammar\`].
    pub fn parsed_grammar(&self) -> Result<Option<GrammarFile>, TextError> {
        let Some(text) = self.grammar else {
            return Ok(None);
        };
        let file = parse_grammar(text)?;
        if file.dialect != SemioDialect::Grammar {
            return Err(TextError::new("LanguageSpec.grammar requires dialect grammar", TextSpan::at(1, 1)));
        }
        Ok(Some(file))
    }

    /// @emoji 📡️ Parses \`protocol\` via [\`parse_grammar\`], requiring [\`SemioDialect::Protocol\`].
    pub fn parsed_protocol(&self) -> Result<Option<GrammarFile>, TextError> {
        let Some(text) = self.protocol else {
            return Ok(None);
        };
        let file = parse_grammar(text)?;
        if file.dialect != SemioDialect::Protocol {
            return Err(TextError::new("LanguageSpec.protocol requires dialect protocol", TextSpan::at(1, 1)));
        }
        Ok(Some(file))
    }

    /// @emoji ✅ Verifies encoded bytes against this language's protocol when protocol text is present.
    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {
        let Some(file) = self.parsed_protocol().map_err(|e| e.message.clone())? else {
            return Ok(());
        };
        verify_protocol_bytes(&file, bytes)
    }
}

/// @emoji 🪪 Pass-through [\`IdiomHooks\`] for binary facets (pack/spr) and text facets without a
/// dedicated \`DslIdiom\` yet — canonicalize is identity; classify/complete are empty.
pub fn passthrough_hooks(lang: &'static str) -> IdiomHooks {
    IdiomHooks {
        lang,
        canonicalize: |text| Ok(text.to_string()),
        classify: |_| Vec::new(),
        complete: |_, _| Vec::new(),
    }
}`;

const SEMIO_LOOKUP = `/// @emoji 🔍️ Resolves a registered language from \`.semio\` file bytes (content-derived envelope).
/// Text components (\`dsl\`/\`op\`) prefer grammar registrations; binary components (\`pack\`/\`spr\`)
/// prefer protocol registrations.
pub fn language_for_semio_content(bytes: &[u8]) -> Option<LanguageSpec> {
    let envelope = semio_format::sniff(bytes).ok()?;
    let base = envelope.envelope_id();
    let plugin = envelope.plugin.as_str();
    let artifact = envelope.artifact.as_str();
    match envelope.component {
        semio_format::Component::Dsl => language(&base).or_else(|| language_for_extension(artifact)).or_else(|| language_for_extension(plugin)),
        semio_format::Component::Op => language_for_suffix_candidates(&base, plugin, artifact, "op").or_else(|| {
            let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
            registry.values().find(|s| s.role == LanguageRole::Ops && s.extension == Some(artifact)).copied()
        }),
        semio_format::Component::Pack => language_for_suffix_candidates(&base, plugin, artifact, "pack")
            .or_else(|| language(&base).filter(|s| s.protocol.is_some())),
        semio_format::Component::Spr => language_for_suffix_candidates(&base, plugin, artifact, "spr"),
        _ => None,
    }
}

fn language_for_suffix_candidates(base: &str, plugin: &str, artifact: &str, suffix: &str) -> Option<LanguageSpec> {
    language(&format!("{base}.{suffix}"))
        .or_else(|| language(&format!("{plugin}.{suffix}")))
        .or_else(|| language(&format!("{artifact}.{suffix}")))
        .or_else(|| language(&format!("{plugin}.{artifact}.{suffix}")))
}`;

const evidence = [];

function replaceRoleDoc(t) {
  const re = /\/\/\/ @emoji 🎭️ Which of an app[\s\S]*?#\[derive\(Clone, Copy, Debug, PartialEq, Eq\)\]\npub enum LanguageRole \{/;
  if (!re.test(t)) return { t, ok: false };
  return { t: t.replace(re, ROLE_DOC + "\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum LanguageRole {"), ok: true };
}

function replaceSpecDoc(t) {
  const re = /\/\/\/ @emoji 📖️ One app[\s\S]*?is touched by adding it\.\n#\[derive\(Clone, Copy\)\]\npub struct LanguageSpec \{/;
  if (!re.test(t)) return { t, ok: false };
  return { t: t.replace(re, SPEC_DOC + "\n#[derive(Clone, Copy)]\npub struct LanguageSpec {"), ok: true };
}

function replaceImpl(t) {
  if (t.includes("fn parsed_grammar(")) return { t, ok: true, skipped: true };
  const re = /impl LanguageSpec \{\n    \/\/\/ @emoji 🧬️ Services a facet still on generic[\s\S]*?hooks: parent\.hooks \}\n    \}\n\}/;
  if (!re.test(t)) return { t, ok: false };
  return { t: t.replace(re, IMPL_BLOCK), ok: true };
}

function replaceLookup(t) {
  if (t.includes("language_for_suffix_candidates")) return { t, ok: true, skipped: true };
  const re = /\/\/\/ @emoji 🔍️ Resolves a registered DSL grammar from `\.semio` file bytes \(content-derived envelope\)\.\npub fn language_for_semio_content\(bytes: &\[u8\]\) -> Option<LanguageSpec> \{\n    let envelope = semio_format::sniff\(bytes\)\.ok\(\)\?;\n    if envelope\.component != semio_format::Component::Dsl \{\n        return None;\n    \}\n    language\(&envelope\.envelope_id\(\)\)\n\}/;
  if (!re.test(t)) return { t, ok: false };
  return { t: t.replace(re, SEMIO_LOOKUP), ok: true };
}

for (const p of paths) {
  let t = readFileSync(p, "utf8");
  const before = t;
  const item = { path: p };

  if (t.startsWith("pub use dsl_grammar::")) {
    const firstNl = t.indexOf("\n");
    const line = t.slice(0, firstNl);
    t = t.slice(firstNl + 1);
    if (!t.includes("pub use dsl_grammar::")) {
      t = t.replace(/pub use dsl_schema::\*;\n/, "pub use dsl_schema::*;\n" + line + "\n");
    }
    item.leadingPubUse = true;
  }

  let r = replaceRoleDoc(t); t = r.t; item.roleDoc = r.ok;
  r = replaceSpecDoc(t); t = r.t; item.specDoc = r.ok;
  r = replaceImpl(t); t = r.t; item.impl = r.ok; if (r.skipped) item.implSkipped = true;
  r = replaceLookup(t); t = r.t; item.lookup = r.ok; if (r.skipped) item.lookupSkipped = true;

  if (t !== before) {
    writeFileSync(p, t);
    item.status = "updated";
    item.delta = t.length - before.length;
  } else {
    item.status = "unchanged";
  }
  evidence.push(item);
}

writeFileSync(join(ticket, "🧪e2e-language-protocol-api-patch.json"), JSON.stringify(evidence, null, 2));
console.log(JSON.stringify(evidence, null, 2));
