"""🧯️ S20 F1 (faults overlay): `FaultDefinition` + `AppDefinition.faults` in the manifest (Rust + owned TS projection +
generated TS). Idempotent. Usage: python3 f1-manifest.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
MANIFEST = ROOT / "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
PROJECTION = ROOT / "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
GENERATED = ROOT / "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"

DOC = [
    "🧯️ One refusal code an app raises, with the text a host shows the person in their locale and terminology — a",
    "refusal is rendered BY ITS CODE (`Fault.code`, `TypedOperationFault.code`) with its parameters filled into the text's",
    "`{name}` placeholders, never from free text. Declared with the builder's `.fault(code, text)`; `parameters` are the",
    "text's placeholder names. The global fault law (`verify faults`) holds every raised code declared and every declared",
    "code raised.",
]
RUST_TYPE = "\n".join(f"/// {line}" for line in DOC) + """
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FaultDefinition {
    pub code: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`.
    pub text: LocalizedLabel,
    pub parameters: Vec<String>,
}

/// 🧩️ The parameter names a fault text shows, in order of first appearance: every `{name}` placeholder whose name is
/// `[a-z][A-Za-z0-9]*` — the same grammar `Fault::with_parameter` names follow. The fault law holds the placeholder sets
/// of every locale × terminology cell equal.
pub fn fault_text_parameters(text: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else { break };
        let name = &after[..close];
        let well_formed = name.chars().next().is_some_and(|first| first.is_ascii_lowercase()) && name.chars().all(|character| character.is_ascii_alphanumeric());
        if well_formed && !names.iter().any(|known| known == name) {
            names.push(name.to_string());
        }
        rest = &after[close + 1..];
    }
    names
}

/// 📏️ Longest fault code, in bytes — the bound `TypedOperationFault` carries a code within.
pub const FAULT_CODE_BYTES: usize = 64;

/// 🧯️ The declaration half of the fault law, per app definition (`AppBuilder::try_build_definition` runs it): every
/// code is `[A-Za-z0-9._/-]`, 1..=[`FAULT_CODE_BYTES`] bytes, declared once; every locale × terminology cell of its text
/// is non-empty and names exactly `parameters` (at most [`dsl::FAULT_PARAMETERS_MAXIMUM`], each within
/// [`dsl::FAULT_PARAMETER_NAME_BYTES`]); the English and German native cells differ. The raise half — every raised code
/// declared, every declared code raised, raise parameters equal to the placeholders — is the census `verify faults`.
pub fn validate_fault_definitions(faults: &[FaultDefinition]) -> Result<(), String> {
    let mut codes = std::collections::BTreeSet::new();
    for fault in faults {
        let code = fault.code.as_str();
        if code.is_empty() || code.len() > FAULT_CODE_BYTES || !code.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'/' | b'-')) {
            return Err(format!("fault code {code:?} is not [A-Za-z0-9._/-] within {FAULT_CODE_BYTES} bytes"));
        }
        if !codes.insert(code) {
            return Err(format!("fault {code} is declared twice"));
        }
        if fault.parameters.len() > dsl::FAULT_PARAMETERS_MAXIMUM || fault.parameters.iter().any(|name| name.len() > dsl::FAULT_PARAMETER_NAME_BYTES) {
            return Err(format!("fault {code} names more than {} parameters or a parameter longer than {} bytes", dsl::FAULT_PARAMETERS_MAXIMUM, dsl::FAULT_PARAMETER_NAME_BYTES));
        }
        let mut declared = fault.parameters.clone();
        declared.sort();
        for terminology in Terminology::ALL {
            for locale in Locale::ALL {
                let text = fault.text.resolve(terminology, locale);
                let mut named = fault_text_parameters(text);
                named.sort();
                if text.trim().is_empty() || named != declared {
                    return Err(format!("fault {code} text {terminology:?}/{locale:?} is empty or does not name exactly the parameters {declared:?}"));
                }
            }
        }
        if fault.text.resolve(Terminology::Native, Locale::En) == fault.text.resolve(Terminology::Native, Locale::De) {
            return Err(format!("fault {code} has the same English and German text"));
        }
    }
    Ok(())
}

/// 🗂️ One framework fault catalog entry (`semio.fault-catalog.v1`): a code a framework crate raises and its en/de text.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultCatalogEntry {
    pub code: String,
    pub en: String,
    pub de: String,
}

/// 🗂️ The framework fault catalog — schema `⚠️diagnostic/🗂️catalog/🧬️schema/🔣️.json`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultCatalog {
    pub schema: String,
    pub faults: Vec<FaultCatalogEntry>,
}

/// 🗂️ The framework's own fault catalog (`⚠️diagnostic/🗂️catalog/🔣️.json`), parsed once. Its shape is held by its JSON
/// Schema (AJV) and the fault census; a catalog that does not parse is a broken build, not a runtime condition.
pub fn framework_fault_catalog() -> &'static FaultCatalog {
    static CATALOG: std::sync::OnceLock<FaultCatalog> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| serde_json::from_str(include_str!("../⚠️diagnostic/🗂️catalog/🔣️.json")).expect("the framework fault catalog satisfies semio.fault-catalog.v1"))
}

/// 🧯️ The text a person or agent reads for one fault, rendered BY ITS CODE in one terminology and locale: an app
/// refusal's (`app`) text as its plugin declares it (`declared`, the apps' `AppDefinition.faults`), any other origin's
/// text from `catalog`; `{name}` placeholders take the parameters (a placeholder without its parameter stays as written);
/// `None` for a code nothing declares. Cases: `⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️text.json` (the host twin is
/// `faultTextV1`, `🏛️ShellHost/🩺️fault/🟦️.ts`).
pub fn fault_text(code: &str, app: bool, parameters: &[dsl::FaultParameter], declared: &[FaultDefinition], catalog: &FaultCatalog, terminology: Terminology, locale: Locale) -> Option<String> {
    let text = if app {
        declared.iter().find(|fault| fault.code == code)?.text.resolve(terminology, locale).to_string()
    } else {
        let entry = catalog.faults.iter().find(|entry| entry.code == code)?;
        match locale {
            Locale::En => entry.en.clone(),
            Locale::De => entry.de.clone(),
        }
    };
    (!text.is_empty()).then(|| parameters.iter().fold(text, |text, parameter| text.replace(&format!("{{{}}}", parameter.name), &parameter.value)))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️fault-text/🦀️.rs"]
mod fault_text_tests;

"""
TS_TYPE = "/**\n" + "\n".join(f" * {line}" for line in DOC) + "\n */\nexport type FaultDefinition = { code: string,\n/**\n * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`.\n */\ntext: unknown, parameters: Array<string>, };"
FIELD_DOC = "🧯️ Every refusal code this app raises, each with the text a host renders by the code — see `FaultDefinition`."
HUNKS = [
    (MANIFEST, "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]\n#[serde(rename_all = \"camelCase\", deny_unknown_fields)]\n#[value(rename_all = \"camelCase\", deny_unknown_fields)]\npub struct AppDefinition {\n",
     RUST_TYPE + "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]\n#[serde(rename_all = \"camelCase\", deny_unknown_fields)]\n#[value(rename_all = \"camelCase\", deny_unknown_fields)]\npub struct AppDefinition {\n"),
    (MANIFEST, "    #[serde(default)]\n    #[value(default)]\n    pub io: AppIo,\n}\n",
     f"    #[serde(default)]\n    #[value(default)]\n    pub io: AppIo,\n    /// {FIELD_DOC}\n    #[serde(default)]\n    #[value(default)]\n    pub faults: Vec<FaultDefinition>,\n}}\n"),
    (PROJECTION, "io: AppIo, };\"####,\n",
     f"io: AppIo,\n/**\n * {FIELD_DOC}\n */\nfaults: Array<FaultDefinition>, }};\"####,\n"),
    (PROJECTION, "        SchemaMetadata {\n            name: \"GranularityDefinition\",\n",
     "        SchemaMetadata {\n            name: \"FaultDefinition\",\n            version: 1,\n            typescript: r####\"" + TS_TYPE + "\"####,\n        },\n        SchemaMetadata {\n            name: \"GranularityDefinition\",\n"),
    (GENERATED, "io: AppIo, };\n", f"io: AppIo,\n/**\n * {FIELD_DOC}\n */\nfaults: Array<FaultDefinition>, }};\n"),
    (GENERATED, "\n\n/**\n * 🔬️ One selectable/hoverable level of detail within a domain", "\n\n" + TS_TYPE + "\n\n/**\n * 🔬️ One selectable/hoverable level of detail within a domain"),
]


def main() -> None:
    texts: dict[Path, str] = {}
    for path, before, after in HUNKS:
        text = texts.setdefault(path, path.read_text())
        if text.count(after) == 1:
            continue
        assert text.count(before) == 1, (path.name, before[:70], text.count(before))
        texts[path] = text.replace(before, after)
    for path, text in texts.items():
        path.write_text(text)
    print("manifest: F1 applied")


if __name__ == "__main__":
    main()
