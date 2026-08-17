import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";

function patchRegister(path, rustBlock) {
  let t = readFileSync(path, "utf8");
  if (t.includes("fn register_pilot_languages")) { console.log("skip", path); return; }
  if (!t.includes("pub fn register()")) { console.log("no register", path); return; }
  t = t.replace(/pub fn register\(\) \{[\s\S]*?\n\}/, (m) => {
    if (m.includes("register_pilot_languages();")) return m;
    return m.replace("{", "{\n    register_pilot_languages();");
  });
  // insert block before endregion Register
  if (t.includes("//#endregion 🔖️Register")) {
    t = t.replace("//#endregion 🔖️Register", rustBlock + "\n//#endregion 🔖️Register");
  } else if (t.includes("// #endregion 🔖️Register")) {
    t = t.replace("// #endregion 🔖️Register", rustBlock + "\n// #endregion 🔖️Register");
  } else {
    t = t.replace(/pub fn register\(\) \{[\s\S]*?\n\}/, (m) => m + "\n\n" + rustBlock);
  }
  writeFileSync(path, t);
  console.log("patched", path);
}

function langBlock(mod, id) {
  return `
fn pilot_language_hooks(lang: \x26\x27static str) -> dsl::IdiomHooks {
    dsl::IdiomHooks {
        lang,
        canonicalize: |text| Ok(text.to_string()),
        classify: |_| Vec::new(),
        complete: |_, _| Vec::new(),
    }
}

/// \u{1F4CC}\u{FE0F} Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "${id}",
        extension: Some("${id}"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::${mod}::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: pilot_language_hooks("${id}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${id}.ops",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::${mod}::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::op::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: pilot_language_hooks("${id}.ops"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${id}.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::${mod}::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: pilot_language_hooks("${id}.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${id}.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_PATH),
        hooks: pilot_language_hooks("${id}.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${id}.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_PATH),
        hooks: pilot_language_hooks("${id}.spr"),
    });
}
`.replaceAll("${mod}", mod).replaceAll("${id}", id);
}

// fem3d
const fem3 = readdirSync("✏️s/🔌️plugins/🏗️fem/🗿️artifacts").find((a) => a.includes("3d"));
patchRegister(join("✏️s/🔌️plugins/🏗️fem/🗿️artifacts", fem3, "⚙️engine/🦀️component.rs"), langBlock("fem3d", "fem3d"));

// writer — append beside register_writer_languages
const writerEngine = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs";
{
  let t = readFileSync(writerEngine, "utf8");
  if (!t.includes("fn register_pilot_languages")) {
    t = t.replace("pub fn register() {\n    register_writer_languages();", "pub fn register() {\n    register_pilot_languages();\n    register_writer_languages();");
    const block = langBlock("writer", "writer");
    t = t.replace("fn register_writer_languages()", block + "\n\nfn register_writer_languages()");
    writeFileSync(writerEngine, t);
    console.log("patched writer");
  } else console.log("skip writer");
}
