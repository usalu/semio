import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";

function findNamed(root, needle) {
  const entries = readdirSync(root);
  const exact = entries.find((e) => e === needle || e.endsWith(needle) || e.replace(/[^a-z0-9]/gi, "") === needle);
  if (exact) return join(root, exact);
  // prefer shortest match containing needle as whole ascii token
  const scored = entries
    .filter((e) => e.includes(needle))
    .map((e) => ({ e, score: e.length }))
    .sort((a, b) => a.score - b.score);
  if (scored[0] && (needle.length > 2 || scored[0].e.endsWith(needle))) return join(root, scored[0].e);
  throw new Error(`no ${needle} in ${root}: ${entries.slice(0, 20)}`);
}
function walkFind(root, pred, out = []) {
  if (!existsSync(root)) return out;
  for (const e of readdirSync(root, { withFileTypes: true })) {
    const p = join(root, e.name);
    if (e.isDirectory()) {
      if (e.name === "node_modules" || e.name === "target" || e.name === ".git") continue;
      walkFind(p, pred, out);
    } else if (pred(p, e.name)) out.push(p);
  }
  return out;
}

const s = readdirSync(".").find((e) => e.endsWith("s") && e.includes("s") && !e.includes("story") && e.length <= 4) || "✏️s";
const sRoot = join(".", s);
const plugins = findNamed(sRoot, "plugins");
const fw = findNamed(".", "framework");
const products = findNamed(fw, "products");
const os = findNamed(products, "os");
const modules = findNamed(os, "modules");
const dsl = findNamed(modules, "dsl");
const grammar = findNamed(dsl, "grammar");
const grammarLib = join(findNamed(findNamed(grammar, "implementations"), "rust"), "📦️lib.rs");
const grammarComp = join(grammar, "🦀️component.rs");
const dslLib = join(findNamed(findNamed(dsl, "implementations"), "rust"), "📦️lib.rs");
const dslComp = join(dsl, "🦀️component.rs");

const ticket = process.argv[2];

function must(cond, msg) { if (!cond) throw new Error(msg); }

// --- 1) passthrough_hooks in dsl ---
function patchDslPassthrough(path) {
  let t = readFileSync(path, "utf8");
  if (t.includes("fn passthrough_hooks(")) {
    console.log("skip passthrough", path);
    return;
  }
  const needle = "pub fn hooks_for<I: DslIdiom>() -> IdiomHooks {";
  must(t.includes(needle), `hooks_for missing in ${path}`);
  // insert after hooks_for function block
  const insertAfter = `pub fn hooks_for<I: DslIdiom>() -> IdiomHooks {
    IdiomHooks { lang: I::LANG, canonicalize: |text| I::parse(text).map(|ast| I::print(&ast)), classify: I::classify, complete: I::complete }
}`;
  // handle both crate::os_dsl and plain versions - find end of hooks_for
  const idx = t.indexOf("pub fn hooks_for");
  must(idx >= 0, "hooks_for fn");
  const end = t.indexOf("\n}", idx);
  must(end > idx, "hooks_for end");
  const after = end + 2;
  const helper = `

/// @emoji 🪞 Minimal hooks for binary/text facets that register a [\`LanguageSpec\`] without a custom
/// [\`DslIdiom\`] front-end — canonicalize is identity; classify/complete are empty.
pub fn passthrough_hooks(lang: &'static str) -> IdiomHooks {
    IdiomHooks { lang, canonicalize: |text| Ok(text.to_string()), classify: |_| Vec::new(), complete: |_, _| Vec::new() }
}
`;
  t = t.slice(0, after) + helper + t.slice(after);
  writeFileSync(path, t);
  console.log("patched passthrough", path);
}

patchDslPassthrough(dslLib);
if (existsSync(dslComp)) patchDslPassthrough(dslComp);

// Fix botched pub use at top of dslComp if present
if (existsSync(dslComp)) {
  let t = readFileSync(dslComp, "utf8");
  if (t.startsWith("pub use dsl_grammar::")) {
    const firstNl = t.indexOf("\n");
    const line = t.slice(0, firstNl);
    t = t.slice(firstNl + 1);
    // ensure reexport exists after other pub uses
    if (!t.includes("pub use dsl_grammar::") && !t.includes("parse_grammar, print_grammar, verify_protocol_bytes")) {
      // try insert near other pub uses
      const marker = "pub use crate::os_dsl::schema::{from_dsl_value, to_dsl_value};";
      if (t.includes(marker)) {
        t = t.replace(marker, marker + "\n" + line);
      } else {
        t = line + "\n" + t;
      }
    }
    writeFileSync(dslComp, t);
    console.log("fixed dslComp pub use placement");
  }
}

// --- 2) grammar verify tests ---
const VERIFY_TESTS = `
    #[test]
    fn parse_grammar_sets_dialect_grammar_vs_protocol() {
        let g = parse_grammar("dialect grammar\\ngrammar demo\\nstart doc\\ndoc = \\"x\\"\\n").expect("grammar");
        assert_eq!(g.dialect, SemioDialect::Grammar);
        let p = parse_grammar("dialect protocol\\nprotocol demo.pack\\nstart frame\\n").expect("protocol");
        assert_eq!(p.dialect, SemioDialect::Protocol);
        assert_eq!(p.start, "frame");
        assert_eq!(p.id, "demo.pack");
    }

    #[test]
    fn verify_protocol_bytes_branches_pack_spk_vs_spr_record() {
        let pack = parse_grammar("dialect protocol\\nprotocol demo.pack\\nstart frame\\n").expect("pack spec");
        let spr = parse_grammar("dialect protocol\\nprotocol demo.spr\\nstart record\\n").expect("spr spec");
        let mut spk = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
        spk.extend(std::iter::repeat_n(0u8, 24));
        verify_protocol_bytes(&pack, &spk).expect("pack accepts SPK header");
        let mut spr_magic = vec![0x89, b'S', b'P', b'R', 0x0D, 0x0A, 0x1A, 0x0A];
        spr_magic.extend(std::iter::repeat_n(0u8, 24));
        assert!(verify_protocol_bytes(&pack, &spr_magic).is_err(), "pack must reject SPR magic");
        assert!(verify_protocol_bytes(&spr, &[]).is_err(), "spr rejects empty");
        verify_protocol_bytes(&spr, &[1u8]).expect("spr record accepts non-empty op bytes without SPK");
        verify_protocol_bytes(&spr, &spk).expect("spr must not require SPK magic");
    }
`;

function patchGrammarTests(path) {
  let t = readFileSync(path, "utf8");
  if (t.includes("verify_protocol_bytes_branches_pack_spk_vs_spr_record")) {
    console.log("skip grammar tests", path);
    return;
  }
  // prefer insert before PluginSemioSpecSweep or before end of tests mod
  if (t.includes("//#region 🔖️PluginSemioSpecSweep")) {
    t = t.replace("//#region 🔖️PluginSemioSpecSweep", VERIFY_TESTS + "\n    //#region 🔖️PluginSemioSpecSweep");
  } else if (t.includes("//#endregion 🔖️FromRecordSpecTests")) {
    t = t.replace("//#endregion 🔖️FromRecordSpecTests", "//#endregion 🔖️FromRecordSpecTests\n" + VERIFY_TESTS);
  } else {
    throw new Error("no insert point in " + path);
  }
  // rustc 1.88 may not have repeat_n - use repeat().take()
  t = t.replaceAll("std::iter::repeat_n(0u8, 24)", "std::iter::repeat(0u8).take(24)");
  writeFileSync(path, t);
  console.log("patched grammar tests", path);
}

patchGrammarTests(grammarLib);
if (existsSync(grammarComp)) patchGrammarTests(grammarComp);

// --- 3) engine LanguageSpec registration ---
const engines = {
  dag: {
    plugin: "dag",
    artifactPathHints: ["dag"],
    registerSnippet: `
fn register_artifact_languages() {
    let doc_hooks = dsl::passthrough_hooks("dag.document");
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.document",
        extension: Some("dag"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dag::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: doc_hooks,
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::dag::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("dag.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::dag::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("dag.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dag::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.spr"),
    });
}
`,
    call: "register_artifact_languages();",
  },
};

function findEngine(pluginNeedle, artifactNeedle) {
  const plugin = findNamed(plugins, pluginNeedle);
  const artifacts = findNamed(plugin, "artifacts");
  const art = findNamed(artifacts, artifactNeedle);
  const engine = findNamed(art, "engine");
  return join(engine, "🦀️component.rs");
}

function patchEngineRegister(path, snippet, callLine) {
  let t = readFileSync(path, "utf8");
  if (t.includes("register_artifact_languages") || t.includes("register_fem2d_languages") || t.includes("register_fem3d_languages") || t.includes("register_note_languages") || (t.includes("writer.document") && t.includes("LanguageRole::Document"))) {
    console.log("skip engine langs", path);
    return;
  }
  // find pub fn register() { ... }
  const m = t.match(/pub fn register\(\) \{[\s\S]*?\n\}/);
  must(m, "register() in " + path);
  const old = m[0];
  const bodyInsert = old.replace(/\{\n/, `{\n    ${callLine}\n`);
  t = t.replace(old, bodyInsert);
  // insert snippet before register region end or after register fn
  if (t.includes("//#endregion 🔖️Register")) {
    t = t.replace("//#endregion 🔖️Register", snippet + "\n//#endregion 🔖️Register");
  } else if (t.includes("// #endregion 🔖️Register")) {
    t = t.replace("// #endregion 🔖️Register", snippet + "\n// #endregion 🔖️Register");
  } else {
    t = t.replace(bodyInsert, bodyInsert + "\n" + snippet);
  }
  writeFileSync(path, t);
  console.log("patched engine", path);
}

// DAG
patchEngineRegister(findEngine("dag", "dag"), engines.dag.registerSnippet, engines.dag.call);

// NOTE
patchEngineRegister(
  findEngine("note", "note"),
  `
fn register_note_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "note.document",
        extension: Some("note"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::note::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::note::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("note.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "note.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::note::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::note::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("note.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "note.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::note::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::note::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("note.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "note.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::note::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::note::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("note.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "note.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::note::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::note::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("note.spr"),
    });
}
`,
  "register_note_languages();",
);

// FEM 2d
patchEngineRegister(
  findEngine("fem", "2d"),
  `
fn register_fem2d_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d",
        extension: Some("fem2d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("fem.fem2d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("fem.fem2d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("fem.fem2d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem2d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.spr"),
    });
}
`,
  "register_fem2d_languages();",
);

// FEM 3d — read protocol ids
const fem3dPackProto = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "fem"), "artifacts"), "3d"), "pack"), "📡️component.protocol.semio"), "utf8");
const fem3dSprProto = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "fem"), "artifacts"), "3d"), "spr"), "📡️component.protocol.semio"), "utf8");
const fem3dPackId = fem3dPackProto.match(/^protocol\\s+(\\S+)/m)?.[1] ?? "3d.pack";
const fem3dSprId = fem3dSprProto.match(/^protocol\\s+(\\S+)/m)?.[1] ?? "3d.spr";
const fem3dDslId = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "fem"), "artifacts"), "3d"), "dsl"), "📖️component.grammar.semio"), "utf8").match(/^grammar\\s+(\\S+)/m)?.[1] ?? "fem.fem3d";
const fem3dOpId = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "fem"), "artifacts"), "3d"), "op"), "📖️component.grammar.semio"), "utf8").match(/^grammar\\s+(\\S+)/m)?.[1] ?? "fem.fem3d.op";
const fem3dDiffId = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "fem"), "artifacts"), "3d"), "diff"), "📖️component.grammar.semio"), "utf8").match(/^grammar\\s+(\\S+)/m)?.[1] ?? "fem.fem3d.diff";
console.log({ fem3dPackId, fem3dSprId, fem3dDslId, fem3dOpId, fem3dDiffId });

patchEngineRegister(
  findEngine("fem", "3d"),
  `
fn register_fem3d_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "${fem3dDslId}",
        extension: Some("fem3d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::fem3d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem3d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("${fem3dDslId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${fem3dOpId}",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::fem3d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem3d::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("${fem3dOpId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${fem3dDiffId}",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::fem3d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem3d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("${fem3dDiffId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${fem3dPackId}",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem3d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem3d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${fem3dPackId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${fem3dSprId}",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${fem3dSprId}"),
    });
}
`,
  "register_fem3d_languages();",
);

// WRITER — extend register_writer_languages
{
  const path = findEngine("writer", "writer");
  let t = readFileSync(path, "utf8");
  if (!t.includes('id: "writer.document"')) {
    const marker = `dsl::register_language(dsl::LanguageSpec {
        id: "jack",
        extension: None,
        role: dsl::LanguageRole::Embedded,
        grammar: None,
        grammar_path: None,
        protocol: None,
        protocol_path: None,
        hooks: jack_hooks,
    });`;
    must(t.includes(marker), "jack LanguageSpec block");
    const extra = `
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.document",
        extension: Some("writer"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::writer::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("writer.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::writer::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("writer.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::writer::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("writer.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.spr"),
    });`;
    // check writer protocol ids
    const wPack = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "writer"), "artifacts"), "writer"), "pack"), "📡️component.protocol.semio"), "utf8");
    const wSpr = readFileSync(join(findNamed(findNamed(findNamed(findNamed(plugins, "writer"), "artifacts"), "writer"), "spr"), "📡️component.protocol.semio"), "utf8");
    const wPackId = wPack.match(/^protocol\\s+(\\S+)/m)?.[1] ?? "writer.pack";
    const wSprId = wSpr.match(/^protocol\\s+(\\S+)/m)?.[1] ?? "writer.spr";
    let extraFixed = extra.replaceAll('"writer.pack"', `"${wPackId}"`).replaceAll('"writer.spr"', `"${wSprId}"`);
    t = t.replace(marker, marker + extraFixed);
    writeFileSync(path, t);
    console.log("patched writer langs", path, { wPackId, wSprId });
  } else {
    console.log("skip writer langs");
  }
}

// --- 4) verify_protocol_bytes in pack/spr conformance ---
const verifyTargets = walkFind(plugins, (p, name) => name === "🦀️component.rs" && (p.includes("/🎒️pack/") || p.includes("/📡️spr/") || p.includes("/pack/") || p.includes("/spr/")));
// filter pilots only
const pilotRoots = ["dag", "fem", "note", "writer"].map((n) => findNamed(plugins, n));
const pilotVerify = verifyTargets.filter((p) => pilotRoots.some((r) => p.startsWith(r + "/") || p.startsWith(r)));

function patchVerify(path) {
  let t = readFileSync(path, "utf8");
  if (t.includes("verify_protocol_bytes_against_encoded")) {
    console.log("skip verify", path);
    return;
  }
  const isPack = p => p.includes("pack");
  const pack = isPack(path);
  if (!t.includes("mod semio_protocol_conformance")) {
    console.log("no conformance mod", path);
    return;
  }
  let test;
  if (pack) {
    // try to infer encode call site
    if (t.includes("DAG_EXAMPLE_TEXT")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT).expect("parse fixture");
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
`;
    } else if (t.includes("SEMIO_NOTE_EXAMPLE_TEXT") || path.includes("note")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::note::dsl::parse_dsl(crate::artifacts::note::dsl::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse fixture");
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
`;
    } else if (path.includes("writer")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::writer::engine::empty_writer_projection();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
`;
    } else if (path.includes("2d") || path.includes("fem2d")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = Fem2dDocument::default();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
`;
    } else if (path.includes("3d") || path.includes("fem3d")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = Fem3dDocument::default();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
`;
    } else {
      console.log("unknown pack", path);
      return;
    }
  } else {
    // spr
    if (path.includes("dag")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = DagOperation::SetNodes { nodes: Vec::new() };
        let bytes = encode_op(&operation).expect("encode");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr/op bytes");
    }
`;
    } else if (path.includes("note")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = NoteOperation::SetGridSpacing { spacing: Some(24.0) };
        let bytes = encode_op(&operation).expect("encode");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr/op bytes");
    }
`;
    } else if (path.includes("writer")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = WriterOperation::SetText { text: "hello".into() };
        let bytes = encode_op(&operation).expect("encode");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr/op bytes");
    }
`;
    } else if (path.includes("2d")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        let bytes = encode_op(&operation).expect("encode");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr/op bytes");
    }
`;
    } else if (path.includes("3d")) {
      test = `
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        let bytes = encode_op(&operation).expect("encode");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr/op bytes");
    }
`;
    } else {
      console.log("unknown spr", path);
      return;
    }
  }
  // insert before closing of semio_protocol_conformance
  const re = /mod semio_protocol_conformance \{[\s\S]*?\n\}/;
  const mm = t.match(re);
  must(mm, "conformance block " + path);
  const block = mm[0];
  const patched = block.replace(/\n\}\s*$/, test + "}\n").replace(/\n\}$/, test + "}");
  // careful: only replace last }
  const lastBrace = block.lastIndexOf("}");
  const newBlock = block.slice(0, lastBrace) + test + block.slice(lastBrace);
  t = t.replace(block, newBlock);
  // For fem pack, Fem2dDocument may need import in conformance mod — it's in super::* if used in file
  writeFileSync(path, t);
  console.log("patched verify", path);
}

for (const p of pilotVerify) patchVerify(p);

// --- 5) evidence md ---
const pilotInclude = `# Pilot include_str + LanguageSpec registration

Date: 2026-08-06

## Rule
- Text facets (\`🗣️dsl\`, \`🔧️op\`, \`🔺️diff\`): \`COMPONENT_GRAMMAR_SEMIO\` via \`include_str!("📖️component.grammar.semio")\`
- Binary facets (\`🎒️pack\`, \`📡️spr\`): \`COMPONENT_PROTOCOL_SEMIO\` via \`include_str!("📡️component.protocol.semio")\`
- Engine \`register()\` wires each into \`dsl::LanguageSpec\` (\`grammar\`/\`grammar_path\` or \`protocol\`/\`protocol_path\`) with \`LanguageRole::{Document,Ops,Diff,Pack,Spr}\`

## Pilots
| Artifact | Facets with include_str | LanguageSpec register site |
|---|---|---|
| 🕸️dag | dsl/op/diff/pack/spr | \`⚙️engine::register_artifact_languages\` |
| 🏗️fem ◻2d | dsl/op/diff/pack/spr | \`⚙️engine::register_fem2d_languages\` |
| 🏗️fem 🧊️3d | dsl/op/diff/pack/spr | \`⚙️engine::register_fem3d_languages\` |
| 🗒️note | dsl/op/diff/pack/spr | \`⚙️engine::register_note_languages\` |
| ✒️writer | dsl/op/diff/pack/spr (+ jack Embedded) | \`⚙️engine::register_writer_languages\` |

## Conformance tests
Each text facet asserts \`parse_grammar(...).dialect == Grammar\`.
Each binary facet asserts \`Protocol\` and \`verify_protocol_bytes\` against \`encode\` / \`encode_op\` bytes.
`;

const wireMd = `# LanguageSpec protocol wire

## Facade (\`🗣️dsl\`)
- \`LanguageSpec\` fields: \`grammar\`, \`grammar_path\`, \`protocol\`, \`protocol_path\`
- \`LanguageRole::Pack\` / \`LanguageRole::Spr\` (alongside Document/Ops/Diff/Embedded/Config)
- \`LanguageSpec::derived\` copies protocol fields from parent
- \`passthrough_hooks(lang)\` for facets without a custom \`DslIdiom\`

## \`verify_protocol_bytes\`
- Requires \`SemioDialect::Protocol\`
- \`start frame\` / id containing \`pack\` → SPK magic + 32-byte header
- \`start record\` / id containing \`spr\` → non-empty op/record bytes (**does not** require SPK/SPR file magic)

## Unit tests (\`dsl_grammar\`)
- \`parse_grammar_sets_dialect_grammar_vs_protocol\`
- \`verify_protocol_bytes_branches_pack_spk_vs_spr_record\`
- Plugin facet sweep + dag pack/spr handcrafted parse checks

## Cargo / linker
Host may be blocked by Xcode license; record failures in this ticket rather than opening Xcode UI.
`;

writeFileSync(join(ticket, "🧪e2e-pilot-include-str.md"), pilotInclude);
writeFileSync(join(ticket, "🧪e2e-language-protocol-wire.md"), wireMd);
console.log("wrote evidence md");
console.log("done");
