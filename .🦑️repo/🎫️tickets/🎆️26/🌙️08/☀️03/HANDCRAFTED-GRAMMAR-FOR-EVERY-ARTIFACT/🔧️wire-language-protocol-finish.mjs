#!/usr/bin/env bun
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from "fs";
import { join } from "path";

function findTicket() {
  const month = join(".🦑️repo/🎫️tickets/🎆️26");
  for (const m of readdirSync(month)) {
    const p = join(month, m, "☀️03", "HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT");
    try { if (statSync(p).isDirectory()) return p; } catch {}
  }
  throw new Error("ticket not found");
}

const ticket = findTicket();
const fw = readdirSync(".").find((x) => x.includes("framework"));
const dslRoot = join(fw, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl");
const lib = join(dslRoot, "⚡️implementations", "🦀️rust", "📦️lib.rs");
const comp = join(dslRoot, "🦀️component.rs");
const evidence = { fixes: [], libExists: existsSync(lib), compExists: existsSync(comp) };

function removeDupPassthrough(p) {
  let t = readFileSync(p, "utf8");
  const matches = [...t.matchAll(/pub fn passthrough_hooks/g)];
  if (matches.length <= 1) {
    evidence.fixes.push({ path: p, fix: "passthrough-ok", count: matches.length });
    return;
  }
  // Remove the long form after LanguageSpec impl (🪪 emoji version)
  const before = t;
  t = t.replace(/\n\/\/\/ @emoji 🪪 Pass-through \[`IdiomHooks`\][\s\S]*?pub fn passthrough_hooks\(lang: &'static str\) -> IdiomHooks \{\n    IdiomHooks \{\n        lang,\n        canonicalize: \|text\| Ok\(text\.to_string\(\)\),\n        classify: \|_\| Vec::new\(\),\n        complete: \|_, _\| Vec::new\(\),\n    \}\n\}\n\nstatic LANGUAGE_REGISTRY/, "\n\nstatic LANGUAGE_REGISTRY");
  if (t === before) {
    // fallback: remove second occurrence block heuristically
    const idxs = [];
    let i = 0;
    while (true) {
      const j = t.indexOf("pub fn passthrough_hooks", i);
      if (j < 0) break;
      idxs.push(j);
      i = j + 1;
    }
    if (idxs.length >= 2) {
      // find start of doc comment before second
      let start = t.lastIndexOf("\n/// @emoji", idxs[1]);
      if (start < 0) start = idxs[1];
      const end = t.indexOf("\n}", idxs[1]);
      const end2 = t.indexOf("\n", end + 2);
      t = t.slice(0, start) + t.slice(end2);
    }
  }
  writeFileSync(p, t);
  evidence.fixes.push({ path: p, fix: "removed-dup-passthrough", remaining: (t.match(/pub fn passthrough_hooks/g) || []).length });
}

removeDupPassthrough(lib);
removeDupPassthrough(comp);

const sessionExtra = `
    /// @emoji 🩺 Text diagnostics from hooks + grammar dialect checks when \`grammar\` is present.
    pub fn diagnostics(&self) -> Vec<TextError> {
        let mut out = Vec::new();
        if self.spec.is_text_role() {
            if let Err(error) = self.canonicalize() {
                out.push(error);
            }
            if let Err(error) = self.spec.parsed_grammar() {
                out.push(error);
            }
        }
        out
    }

    /// @emoji 📡️ Byte-level protocol verification when \`protocol\` text is present on the spec.
    pub fn verify_protocol_bytes(&self, bytes: &[u8]) -> Result<(), String> {
        self.spec.verify_protocol(bytes)
    }

    /// @emoji 📖️ Parsed grammar file for text roles (\`None\` when unset).
    pub fn grammar_file(&self) -> Result<Option<GrammarFile>, TextError> {
        self.spec.parsed_grammar()
    }

    /// @emoji 📡️ Parsed protocol file for binary verification (\`None\` when unset).
    pub fn protocol_file(&self) -> Result<Option<GrammarFile>, TextError> {
        self.spec.parsed_protocol()
    }
`;

function wireLsp(p, importLine, grammarType) {
  if (!existsSync(p)) {
    evidence.fixes.push({ path: p, fix: "missing" });
    return;
  }
  let t = readFileSync(p, "utf8");
  const before = t;
  if (!t.includes("fn diagnostics(")) {
    if (t.includes(importLine)) {
      t = t.replace(importLine, importLine.replace("TokenClass}", "GrammarFile, TokenClass}").replace("TokenClass };", "GrammarFile, TokenClass };"));
      // more reliable:
      t = t.replace(
        /use (dsl|crate::os_dsl)::\{CompletionItem, LanguageSpec, TextError, TokenClass\};/,
        "use $1::{CompletionItem, GrammarFile, LanguageSpec, TextError, TokenClass};"
      );
    }
    const extra = sessionExtra.replaceAll("GrammarFile", grammarType === "dsl" ? "GrammarFile" : "GrammarFile");
    t = t.replace(
      /    pub fn canonicalize\(&self\) -> Result<String, TextError> \{\n        \(self\.spec\.hooks\.canonicalize\)\(&self\.text\)\n    \}\n\}/,
      `    pub fn canonicalize(&self) -> Result<String, TextError> {\n        (self.spec.hooks.canonicalize)(&self.text)\n    }\n${extra}\n}`
    );
  }
  if (t !== before) {
    writeFileSync(p, t);
    evidence.fixes.push({ path: p, fix: "lsp-wired" });
  } else {
    evidence.fixes.push({ path: p, fix: "lsp-unchanged", hasDiag: t.includes("fn diagnostics(") });
  }
}

wireLsp(join(dslRoot, "🧠️lsp/⚡️implementations/🦀️rust/📦️lib.rs"), "", "dsl");
wireLsp(join(dslRoot, "🧠️lsp/🦀️component.rs"), "", "crate");

function enrich(path, docId, opId, art) {
  if (!existsSync(path)) {
    evidence.fixes.push({ path, fix: "engine-missing" });
    return;
  }
  let t = readFileSync(path, "utf8");
  const before = t;
  const docPat = new RegExp(
    `(id: "${docId}"[\\s\\S]*?grammar_path: Some\\(crate::artifacts::${art}::dsl::COMPONENT_GRAMMAR_PATH\\),\\n\\s*)protocol: None,\\n(\\s*)protocol_path: None,`
  );
  if (docPat.test(t)) {
    t = t.replace(
      docPat,
      `$1protocol: Some(crate::artifacts::${art}::pack::COMPONENT_PROTOCOL_SEMIO),\n$2protocol_path: Some(crate::artifacts::${art}::pack::COMPONENT_PROTOCOL_PATH),`
    );
  }
  const opPat = new RegExp(
    `(id: "${opId}"[\\s\\S]*?grammar_path: Some\\(crate::artifacts::${art}::op::COMPONENT_GRAMMAR_PATH\\),\\n\\s*)protocol: None,\\n(\\s*)protocol_path: None,`
  );
  if (opPat.test(t)) {
    t = t.replace(
      opPat,
      `$1protocol: Some(crate::artifacts::${art}::spr::COMPONENT_PROTOCOL_SEMIO),\n$2protocol_path: Some(crate::artifacts::${art}::spr::COMPONENT_PROTOCOL_PATH),`
    );
  }
  if (t !== before) {
    writeFileSync(path, t);
    evidence.fixes.push({ path, fix: "carry-protocol", docId, opId });
  } else {
    evidence.fixes.push({
      path,
      fix: "enrich-no-change",
      ids: [...t.matchAll(/id: "([^"]+)"/g)].map((m) => m[1]),
      hasRegisterPilot: t.includes("register_pilot_languages"),
    });
  }
}

enrich("✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/⚙️engine/🦀️component.rs", "dag.document", "dag.op", "dag");
enrich("✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/⚙️engine/🦀️component.rs", "note.document", "note.op", "note");
enrich("✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs", "writer.document", "writer.op", "writer");
enrich("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/⚙️engine/🦀️component.rs", "fem.fem2d", "fem.fem2d.op", "fem2d");

// Writer: ensure register calls include facet regs; clean orphan stub
{
  const p = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs";
  let t = readFileSync(p, "utf8");
  const before = t;
  t = t.replace(/\n\/\/\/ 📌️ Registers handcrafted facet grammars \(text\) and protocols \(binary\) for in-process execution\.\n\n\nfn register_writer_languages/, "\n\nfn register_writer_languages");
  if (t !== before) {
    writeFileSync(p, t);
    evidence.fixes.push({ path: p, fix: "writer-orphan-comment-cleanup" });
  }
}

writeFileSync(join(ticket, "lsp-hook-contract.md"), `# LSP hook contract

## Registry

\`LanguageSpec\` in \`dsl\` facade:

- \`id\`, \`extension\`
- \`role\`: \`Document\` | \`Config\` | \`Ops\` | \`Embedded\` | \`Diff\` | \`Pack\` | \`Spr\`
- **Text:** \`grammar\` / \`grammar_path\` (\`.grammar.semio\`, \`dialect grammar\`) for \`🗣️dsl\` / \`🔧️op\` / \`🔺️diff\`
- **Binary:** \`protocol\` / \`protocol_path\` (\`.protocol.semio\`, \`dialect protocol\`) for \`🎒️pack\` / \`📡️spr\`
- \`hooks\` (\`IdiomHooks\`): \`canonicalize\`, \`classify\`, \`complete\`

Helpers: \`LanguageSpec::parsed_grammar\`, \`parsed_protocol\`, \`verify_protocol\`, \`passthrough_hooks\`, \`is_text_role\` / \`is_binary_role\`.

\`LanguageSpec::derived\` copies grammar+protocol fields from the parent.

## Hosts

- \`LanguageSession\` — in-process; writer calls synchronously.
  - Text: \`semantic_tokens_lsp\`, \`completions_at\`, \`canonicalize\`, \`diagnostics\` (hooks + grammar dialect check)
  - Binary: \`verify_protocol_bytes(bytes)\` when \`protocol\` is set
- \`dsl_lsp\` — JSON-RPC 3.17; \`semanticTokens/full\` returns \`{ data: number[] }\`
- \`s_language_bundle\` — \`✏️s/🔨️modules/🗣️lang/\`

## Writer boundary

Keep \`TextEditorScene\` JSON; map LSP results 1:1.

## Vendor

\`semio/documentContext\`, \`semio/editorExtras\`, \`semio/grammar\` optional.
`);
evidence.fixes.push({ fix: "lsp-hook-contract-updated" });

writeFileSync(join(ticket, "🧪e2e-language-protocol-finish.json"), JSON.stringify(evidence, null, 2));
console.log(JSON.stringify(evidence, null, 2));
