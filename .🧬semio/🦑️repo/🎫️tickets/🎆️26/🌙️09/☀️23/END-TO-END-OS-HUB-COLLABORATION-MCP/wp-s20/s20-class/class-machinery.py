"""🗂️ S20 fault classes (row 12, spec `📓️fault-localization-api.md` §7) — the machinery on the pass-1 faults overlay:
`FaultClass` (manifest, kebab wire), `FaultDefinition.class` + `FaultCatalogEntry.class`, `fault_class` (readers holding
declarations) + `declare_fault_classes`/`declared_fault_class` (the producing plugin; the catalog is scanned line by line,
never parsed into one allocation, so guests stay under their contiguous-request ceiling), the builder's
`.fault(code, class, text)` (+ registration in `try_build_definition`), the typed record's required `class` (Rust, JSON
Schema, fixture, TS decoder), the catalog schema, the owned TS projection + generated manifest TS, the MCP gateway (class on
its `Fault`, `details.fault.class`, guest faults mapped BY CLASS), and the census (`class-missing`, `class-unknown`,
declaration conflicts include the class). Declarations and catalog entries get their classes from `class-apply.py`.
Idempotent (an edit whose result is present is skipped; its anchor must match exactly once otherwise).
Usage: python3 class-machinery.py [--dry-run]"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
MANIFEST_LAW = "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️fault-text/🦀️.rs"
PROJECTION = "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
GENERATED = "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
CATALOG_SCHEMA = "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🧬️schema/🔣️.json"
FAULT_TEXT_FIXTURE = "🧰️framework/🔨️modules/⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️text.json"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
SDK = f"{PLUGIN}/🦀️.rs"
RETAINED = f"{PLUGIN}/🧵️retained-command/🦀️.rs"
RECORD_SCHEMA = f"{PLUGIN}/🧬️schema/🧯️typed-operation-fault/🔣️.json"
RECORD_FIXTURE = f"{PLUGIN}/🧫️fixtures/🧯️typed-operation-fault.json"
RECORD_LAW = f"{PLUGIN}/🧪️tests/🧯️typed-operation-fault/🦀️.rs"
WIRE_TURN = "🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts"
MCP_DISPATCH = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs"
MCP_WORKSPACE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"
CENSUS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts"
CENSUS_FIXTURE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json"
CLASSES = ["input-invalid", "precondition-failed", "conflict", "permission-denied", "unavailable", "cancelled", "internal"]
VARIANTS = ["InputInvalid", "PreconditionFailed", "Conflict", "PermissionDenied", "Unavailable", "Cancelled", "Internal"]
CLASS_DOC = [
    "🗂️ What kind of refusal a code is, declared with the code — how a host and the agent gateway answer it, never guessed",
    "from the code or its text: `input-invalid` (the request itself is wrong), `precondition-failed` (the current state does",
    "not allow it), `conflict` (someone else changed it meanwhile), `permission-denied` (the person or agent lacks the right),",
    "`unavailable` (transient; retrying later can succeed), `cancelled`, `internal` (a defect).",
]
CLASS_TS = "/**\n" + "".join(f" * {line}\n" for line in CLASS_DOC) + " */\nexport type FaultClass = " + " | ".join(f'"{name}"' for name in CLASSES) + ";"

RUST_CLASS = "\n".join(f"/// {line}" for line in CLASS_DOC) + """
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum FaultClass {
""" + "".join(f"    {variant},\n" for variant in VARIANTS) + """}

impl FaultClass {
    /// 🗂️ Every class, in declaration order.
    pub const ALL: [FaultClass; 7] = [""" + ", ".join(f"FaultClass::{variant}" for variant in VARIANTS) + """];

    /// 🏷️ The class's wire name (`input-invalid`, …).
    pub fn as_str(self) -> &'static str {
        match self {
""" + "".join(f'            FaultClass::{variant} => "{name}",\n' for variant, name in zip(VARIANTS, CLASSES)) + """        }
    }
}

"""

RUST_LOOKUPS = """
/// 🗂️ The class of one fault for a reader holding the declarations (hosts, the MCP gateway) — the twin of [`fault_text`]:
/// an app refusal's (`app`) class as its plugin declares it, any other origin's from `catalog`; `None` for a code nothing
/// declares.
pub fn fault_class(code: &str, app: bool, declared: &[FaultDefinition], catalog: &FaultCatalog) -> Option<FaultClass> {
    if app {
        declared.iter().find(|fault| fault.code == code).map(|fault| fault.class)
    } else {
        catalog.faults.iter().find(|entry| entry.code == code).map(|entry| entry.class)
    }
}

/// 🗂️ The refusal classes the plugin of this process declares, by code — filled by every app definition it assembles
/// ([`declare_fault_classes`]), read when a refusal crosses a boundary as its typed record ([`declared_fault_class`]).
fn declared_fault_classes() -> &'static std::sync::RwLock<std::collections::BTreeMap<String, FaultClass>> {
    static CLASSES: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<String, FaultClass>>> = std::sync::OnceLock::new();
    CLASSES.get_or_init(Default::default)
}

/// 🗂️ Records the class of every refusal code one app definition declares (`AppBuilder::try_build_definition`).
pub fn declare_fault_classes(faults: &[FaultDefinition]) {
    let mut classes = declared_fault_classes().write().unwrap_or_else(std::sync::PoisonError::into_inner);
    for fault in faults {
        classes.insert(fault.code.clone(), fault.class);
    }
}

/// 🗂️ The class of one framework catalog entry, read off the catalog's one-entry-per-line source (`{"code": …,
/// "class": …, "en": …, "de": …}`, the generator's fixed form the census holds) without parsing it into one allocation —
/// a guest answers it within its contiguous-request ceiling. The law `catalog_classes_scan_as_parsed` holds it equal to
/// the parsed catalog.
pub fn catalog_fault_class(code: &str) -> Option<FaultClass> {
    include_str!("../⚠️diagnostic/🗂️catalog/🔣️.json").lines().find_map(|line| {
        let rest = line.trim_start().strip_prefix("{\\"code\\": \\"")?;
        let (entry, rest) = rest.split_once('"')?;
        if entry != code {
            return None;
        }
        let class = rest.strip_prefix(", \\"class\\": \\"")?.split('"').next()?;
        FaultClass::ALL.into_iter().find(|candidate| candidate.as_str() == class)
    })
}

/// 🗂️ The class a refusal crosses a boundary with (its typed record's `class`): an app refusal's (`app`) as the plugin of
/// this process declares it, any other origin's from the framework catalog. A code nothing declares breaks the fault law
/// (`verify faults`), so it is a defect: [`FaultClass::Internal`].
pub fn declared_fault_class(code: &str, app: bool) -> FaultClass {
    let declared = if app { declared_fault_classes().read().unwrap_or_else(std::sync::PoisonError::into_inner).get(code).copied() } else { catalog_fault_class(code) };
    declared.unwrap_or(FaultClass::Internal)
}
"""

RECORD_TWIN = f"{PLUGIN}/🧪️tests/🧯️typed-operation-fault/🟦️.ts"
PUZZLE_RETAINED = "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs"
MCP_QUICK = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs"
EDITS: list[tuple[str, str, str]] = [
    (PUZZLE_RETAINED, "&semio_framework_plugin::app::TypedOperationFault::of_fault(fault).encode()", "&semio_framework_plugin::app::TypedOperationFault::of_declared(fault).encode()"),
    (MCP_QUICK, "        parameters: vec![semio_framework::FaultParameter { name: \"format\".to_string(), value: \"step\".to_string() }],\n        texts:",
     "        parameters: vec![semio_framework::FaultParameter { name: \"format\".to_string(), value: \"step\".to_string() }],\n        class: Some(semio_framework::FaultClass::InputInvalid),\n        texts:"),
    (MCP_QUICK, 'serde_json::json!({ "code": "process3d.media.export-format", "parameters":', 'serde_json::json!({ "code": "process3d.media.export-format", "class": "input-invalid", "parameters":'),
    (MCP_QUICK, 'serde_json::json!({ "code": VIEWER_READ_ONLY_FAULT_CODE, "parameters": [], "texts": null })', 'serde_json::json!({ "code": VIEWER_READ_ONLY_FAULT_CODE, "class": null, "parameters": [], "texts": null })'),
    (SDK, "        fault(code: &'static str, text: impl Into<LocalizedLabel>),\n", "        fault(code: &'static str, class: semio_framework::FaultClass, text: impl Into<LocalizedLabel>),\n"),
    (RECORD_TWIN, "code: spelled(value, \"code\"), origin: value.origin, message: spelled(value, \"message\"),", "code: spelled(value, \"code\"), origin: value.origin, class: value.class, message: spelled(value, \"message\"),"),
    (RECORD_TWIN, "origin: fault.origin, message: clip(spelled(fault, \"message\"), messageBytes),", "origin: fault.origin, class: fault.class, message: clip(spelled(fault, \"message\"), messageBytes),"),
    (CENSUS, 'for (const match of masked.matchAll(/(?<![\\w])FaultOrigin::App\\b/gu)) violate(match.index!, "framework-app-origin",',
     'for (const match of masked.matchAll(/(?:Fault::new|\\w*fault)\\s*\\(\\s*(?:[A-Za-z_]\\w*::)*FaultOrigin::App\\b/gu)) violate(match.index!, "framework-app-origin",'),
    (CATALOG_SCHEMA, '"required": ["code", "en", "de"],', '"required": ["code", "class", "en", "de"],'),
    (CATALOG_SCHEMA, '          "code": { "type": "string", "pattern": "^[A-Za-z0-9._/-]+$", "minLength": 1, "maxLength": 64 },\n',
     '          "code": { "type": "string", "pattern": "^[A-Za-z0-9._/-]+$", "minLength": 1, "maxLength": 64 },\n          "class": { "enum": [' + ", ".join(f'"{name}"' for name in CLASSES) + '] },\n'),
    (CATALOG_SCHEMA, "A host renders a fault BY ITS CODE:", "Every entry names its class — how a host and the agent gateway answer the code. A host renders a fault BY ITS CODE:"),
    (MANIFEST, "/// 🧯️ One refusal code an app raises, with the text a host shows the person in their locale and terminology — a\n", RUST_CLASS + "/// 🧯️ One refusal code an app raises, with the text a host shows the person in their locale and terminology — a\n"),
    (MANIFEST, "/// `{name}` placeholders, never from free text. Declared with the builder's `.fault(code, text)`; `parameters` are the\n/// text's placeholder names.",
     "/// `{name}` placeholders, never from free text, and answered by its `class`. Declared with the builder's\n/// `.fault(code, class, text)`; `parameters` are the text's placeholder names."),
    (MANIFEST, "pub struct FaultDefinition {\n    pub code: String,\n", "pub struct FaultDefinition {\n    pub code: String,\n    pub class: FaultClass,\n"),
    (MANIFEST, "/// 🗂️ One framework fault catalog entry (`semio.fault-catalog.v1`): a code a framework crate raises and its en/de text.",
     "/// 🗂️ One framework fault catalog entry (`semio.fault-catalog.v1`): a code a framework crate raises, its class and its\n/// en/de text."),
    (MANIFEST, "pub struct FaultCatalogEntry {\n    pub code: String,\n", "pub struct FaultCatalogEntry {\n    pub code: String,\n    pub class: FaultClass,\n"),
    (MANIFEST, "    (!text.is_empty()).then(|| parameters.iter().fold(text, |text, parameter| text.replace(&format!(\"{{{}}}\", parameter.name), &parameter.value)))\n}\n",
     "    (!text.is_empty()).then(|| parameters.iter().fold(text, |text, parameter| text.replace(&format!(\"{{{}}}\", parameter.name), &parameter.value)))\n}\n" + RUST_LOOKUPS),
    (SDK, "        pub async fn fault(mut self, code: &'static str, text: impl Into<LocalizedLabel>) -> Self {",
     "        pub async fn fault(mut self, code: &'static str, class: semio_framework::FaultClass, text: impl Into<LocalizedLabel>) -> Self {"),
    (SDK, "            self.faults.push(semio_framework::FaultDefinition { code: code.to_string(), text, parameters });",
     "            self.faults.push(semio_framework::FaultDefinition { code: code.to_string(), class, text, parameters });"),
    (SDK, "        /// in their locale and terminology — `{name}` placeholders take the values the refusal carries\n",
     "        /// in their locale and terminology and its `class` (how hosts and the agent gateway answer it) —\n        /// `{name}` placeholders take the values the refusal carries\n"),
    (SDK, "            semio_framework::validate_fault_definitions(&self.faults).map_err(|message| PluginAssemblyError::new(\"app-definition.invalid\", format!(\"app {}: {message}\", self.id)))?;\n",
     "            semio_framework::validate_fault_definitions(&self.faults).map_err(|message| PluginAssemblyError::new(\"app-definition.invalid\", format!(\"app {}: {message}\", self.id)))?;\n            semio_framework::declare_fault_classes(&self.faults);\n"),
    (SDK, "        pub origin: FaultOrigin,\n        pub message: String,\n        pub parameters: Vec<semio_framework::FaultParameter>,\n    }\n",
     "        pub origin: FaultOrigin,\n        pub class: semio_framework::FaultClass,\n        pub message: String,\n        pub parameters: Vec<semio_framework::FaultParameter>,\n    }\n"),
    (SDK, "    /// `parameters` the values that text shows, `origin` who refused, `message` the refuser's own detail. The\n",
     "    /// `parameters` the values that text shows, `origin` who refused, `class` how hosts and the agent gateway answer it\n    /// (the declaring party's, see `semio_framework::declared_fault_class`), `message` the refuser's own detail. The\n"),
    (SDK, "        pub fn of_fault(fault: &Fault) -> Self {", "        pub fn of_fault(fault: &Fault, class: semio_framework::FaultClass) -> Self {"),
    (SDK, "            Self { schema: TYPED_OPERATION_FAULT_SCHEMA.to_string(), code: code.to_string(), origin: fault.origin, message: fault.message[..end].to_string(), parameters }\n        }\n",
     "            Self { schema: TYPED_OPERATION_FAULT_SCHEMA.to_string(), code: code.to_string(), origin: fault.origin, class, message: fault.message[..end].to_string(), parameters }\n        }\n\n"
     "        /// 🗂️ The record of one `Fault` a party of this process raised, with the class its declaration gives the code\n"
     "        /// (`semio_framework::declared_fault_class`: the plugin's own declarations for an app refusal, else the catalog).\n"
     "        pub fn of_declared(fault: &Fault) -> Self {\n"
     "            Self::of_fault(fault, semio_framework::declared_fault_class(&fault.code.0, matches!(fault.origin, FaultOrigin::App)))\n"
     "        }\n"),
    (SDK, "        origin: FaultOrigin,\n        parameters: [BoundedFaultParameter; semio_framework::FAULT_PARAMETERS_MAXIMUM],\n",
     "        origin: FaultOrigin,\n        class: semio_framework::FaultClass,\n        parameters: [BoundedFaultParameter; semio_framework::FAULT_PARAMETERS_MAXIMUM],\n"),
    (SDK, "                Some(record) => Self::from_fault(&record.into_fault()),\n", "                Some(record) => Self::from_record(&record),\n"),
    (SDK, "        fn from_fault(fault: &Fault) -> Self {\n            let record = TypedOperationFault::of_fault(fault);\n",
     "        fn from_fault(fault: &Fault) -> Self {\n            Self::from_record(&TypedOperationFault::of_declared(fault))\n        }\n\n        fn from_record(record: &TypedOperationFault) -> Self {\n"),
    (SDK, "origin: record.origin, parameters: [empty;", "origin: record.origin, class: record.class, parameters: [empty;"),
    (SDK, "                origin: self.origin,\n                message: String::from_utf8_lossy(&self.bytes[..self.len]).into_owned(),\n",
     "                origin: self.origin,\n                class: self.class,\n                message: String::from_utf8_lossy(&self.bytes[..self.len]).into_owned(),\n"),
    (SDK, "let record = TypedOperationFault::of_fault(&Fault::new(FaultOrigin::Framework, FaultCode::new(\"interactive-job.cancelled\"),",
     "let record = TypedOperationFault::of_declared(&Fault::new(FaultOrigin::Framework, FaultCode::new(\"interactive-job.cancelled\"),"),
    (RETAINED, "&crate::app::TypedOperationFault::of_fault(fault).encode()", "&crate::app::TypedOperationFault::of_declared(fault).encode()"),
    (RECORD_LAW, "        TypedOperationFault { schema: TYPED_OPERATION_FAULT_SCHEMA.to_string(), code: spelled(value, \"code\"), origin: origin(value), message: spelled(value, \"message\"), parameters: parameters(value) }",
     "        TypedOperationFault { schema: TYPED_OPERATION_FAULT_SCHEMA.to_string(), code: spelled(value, \"code\"), origin: origin(value), class: class(value), message: spelled(value, \"message\"), parameters: parameters(value) }"),
    (RECORD_LAW, "    fn parameters(value: &serde_json::Value) -> Vec<semio_framework::FaultParameter> {",
     "    fn class(value: &serde_json::Value) -> semio_framework::FaultClass {\n        <semio_framework::FaultClass as protocol::FromValue>::from_value(DslValue::String(value[\"class\"].as_str().expect(\"class\").to_string())).expect(\"a known class\")\n    }\n\n    fn parameters(value: &serde_json::Value) -> Vec<semio_framework::FaultParameter> {"),
    (RECORD_LAW, "            let record = TypedOperationFault::of_fault(&fault);\n", "            let record = TypedOperationFault::of_fault(&fault, class(&case[\"fault\"]));\n"),
    (RECORD_LAW, "            assert_eq!(ArtifactBoundedToolFault::from_fault(&fault).record(), record, \"{name}: the retained owner publishes the same record\");\n            assert_eq!(TypedOperationFault::of_fault(&record.clone().into_fault()), record, \"{name}: the answered fault keeps the record\");\n",
     "            assert_eq!(ArtifactBoundedToolFault::from_record(&record).record(), record, \"{name}: the retained owner publishes the same record\");\n            assert_eq!(TypedOperationFault::of_fault(&record.clone().into_fault(), record.class), record, \"{name}: the answered fault keeps the record\");\n"),
    (WIRE_TURN, "export type TypedOperationFaultOriginV1 = (typeof TYPED_OPERATION_FAULT_ORIGINS_V1)[number];\n",
     "export type TypedOperationFaultOriginV1 = (typeof TYPED_OPERATION_FAULT_ORIGINS_V1)[number];\n/** 🗂️ How a host and the agent gateway answer a refusal — the wire names of Rust `FaultClass`, declared with the code. */\n"
     "export const TYPED_OPERATION_FAULT_CLASSES_V1 = [" + ", ".join(f'"{name}"' for name in CLASSES) + "] as const;\nexport type TypedOperationFaultClassV1 = (typeof TYPED_OPERATION_FAULT_CLASSES_V1)[number];\n"),
    (WIRE_TURN, "code: string; origin: TypedOperationFaultOriginV1; message: string;", "code: string; origin: TypedOperationFaultOriginV1; class: TypedOperationFaultClassV1; message: string;"),
    (WIRE_TURN, "/** 🔎️ Decodes one Fault-lane payload by type; `null` for bytes that are no record — exactly the five members, this\n * schema, a frozen code of 1…{@link TYPED_OPERATION_FAULT_CODE_BYTES} UTF-8 bytes, a known origin, a message of at",
     "/** 🔎️ Decodes one Fault-lane payload by type; `null` for bytes that are no record — exactly the six members, this\n * schema, a frozen code of 1…{@link TYPED_OPERATION_FAULT_CODE_BYTES} UTF-8 bytes, a known origin and class, a message of at"),
    (WIRE_TURN, "  const { schema, code, origin, message, parameters } = record;\n  if (Object.keys(record).length !== 5 ||",
     "  const { schema, code, origin, class: klass, message, parameters } = record;\n  if (Object.keys(record).length !== 6 ||"),
    (WIRE_TURN, "  const known = TYPED_OPERATION_FAULT_ORIGINS_V1.find((name) => name === origin);\n  return known === undefined ? null : { schema: TYPED_OPERATION_FAULT_SCHEMA_V1, code, origin: known, message, parameters: decoded };",
     "  const known = TYPED_OPERATION_FAULT_ORIGINS_V1.find((name) => name === origin);\n  const knownClass = TYPED_OPERATION_FAULT_CLASSES_V1.find((name) => name === klass);\n  return known === undefined || knownClass === undefined ? null : { schema: TYPED_OPERATION_FAULT_SCHEMA_V1, code, origin: known, class: knownClass, message, parameters: decoded };"),
    (WIRE_TURN, "code: TYPED_OPERATION_FAULT_PAGE_INVALID_CODE, origin: \"framework\", message:", "code: TYPED_OPERATION_FAULT_PAGE_INVALID_CODE, origin: \"framework\", class: \"internal\", message:"),
    (MCP_DISPATCH, "    /// 🧩️ The values the declared text of `code` shows (`Fault.parameters` of the guest's fault).\n    pub parameters: Vec<semio_framework::FaultParameter>,\n",
     "    /// 🧩️ The values the declared text of `code` shows (`Fault.parameters` of the guest's fault).\n    pub parameters: Vec<semio_framework::FaultParameter>,\n"
     "    /// 🗂️ The declared class of `code` (the plugin's `AppDefinition.faults` or the framework fault catalog) — a guest\n"
     "    /// fault is answered BY IT; `None` for a code this gateway raises itself (answered by its own code).\n"
     "    pub class: Option<semio_framework::FaultClass>,\n"),
    (MCP_DISPATCH, "        \"code\": fault.code,\n        \"parameters\":", "        \"code\": fault.code,\n        \"class\": fault.class.map(semio_framework::FaultClass::as_str),\n        \"parameters\":"),
    (MCP_DISPATCH, "fn map_fault(fault: &Fault) -> GatewayError {\n    with_fault_details(fault, match fault.code.as_str() {\n",
     "fn map_fault(fault: &Fault) -> GatewayError {\n    if let Some(class) = fault.class {\n        return with_fault_details(fault, fault_remedy(fault, class_error(class, fault.message.clone())));\n    }\n    with_fault_details(fault, match fault.code.as_str() {\n"),
    (MCP_DISPATCH, "/// 📮️ The error a commit answers when its hub document did not acknowledge it:",
     "/// 🗂️ The gateway error a declared fault class answers — one per class, no code-specific exceptions (spec §7).\n"
     "fn class_error(class: semio_framework::FaultClass, message: String) -> GatewayError {\n"
     "    match class {\n"
     "        semio_framework::FaultClass::InputInvalid => GatewayError::new(GatewayErrorCode::InputInvalid, message),\n"
     "        semio_framework::FaultClass::PreconditionFailed => GatewayError::new(GatewayErrorCode::PreconditionFailed, message),\n"
     "        semio_framework::FaultClass::Conflict => GatewayError::new(GatewayErrorCode::RevisionConflict, message),\n"
     "        semio_framework::FaultClass::PermissionDenied => GatewayError::new(GatewayErrorCode::PermissionDenied, message),\n"
     "        semio_framework::FaultClass::Unavailable => GatewayError::new(GatewayErrorCode::PluginUnavailable, message).retryable(),\n"
     "        semio_framework::FaultClass::Cancelled => GatewayError::new(GatewayErrorCode::Cancelled, message),\n"
     "        semio_framework::FaultClass::Internal => GatewayError::new(GatewayErrorCode::Internal, message),\n"
     "    }\n"
     "}\n\n"
     "/// 🩹️ The remedy the gateway adds for the declared faults an agent can resolve by itself (the command that changes\n"
     "/// nothing here, the lane an agent cannot carry) — details only, the error stays the class's.\n"
     "fn fault_remedy(fault: &Fault, error: GatewayError) -> GatewayError {\n"
     "    match fault.code.as_str() {\n"
     "        COMMAND_NO_EFFECT_FAULT_CODE => error.with_details(serde_json::json!({ \"faultCode\": fault.code, \"remedy\": { \"en\": COMMAND_NO_EFFECT_REMEDY.0, \"de\": COMMAND_NO_EFFECT_REMEDY.1 } })),\n"
     "        AGENT_LANE_UNCARRIED_FAULT_CODE | AGENT_LANE_PREVIEW_BUDGET_FAULT_CODE => error.with_details(serde_json::json!({ \"faultCode\": fault.code, \"remedy\": { \"en\": AGENT_LANE_UNCARRIED_REMEDY.0, \"de\": AGENT_LANE_UNCARRIED_REMEDY.1 } })),\n"
     "        _ => error,\n"
     "    }\n"
     "}\n\n"
     "/// 📮️ The error a commit answers when its hub document did not acknowledge it:"),
    (MCP_WORKSPACE, "            let texts = text(semio_framework::Locale::En).zip(text(semio_framework::Locale::De)).map(|(en, de)| crate::FaultTexts { en, de });\n            Fault { code, message, parameters, texts }\n",
     "            let texts = text(semio_framework::Locale::En).zip(text(semio_framework::Locale::De)).map(|(en, de)| crate::FaultTexts { en, de });\n"
     "            let class = Some(semio_framework::fault_class(&code, app, &declared, semio_framework::framework_fault_catalog()).unwrap_or(semio_framework::FaultClass::Internal));\n"
     "            Fault { code, message, parameters, class, texts }\n"),
    (CENSUS, "export type FaultDeclarationSite = Readonly<{ path: string; line: number; code: string; en: string; de: string }>;",
     "export type FaultDeclarationSite = Readonly<{ path: string; line: number; code: string; class: string; en: string; de: string }>;"),
    (CENSUS, "export type FaultCatalogEntry = Readonly<{ code: string; en: string; de: string }>;",
     "export type FaultCatalogEntry = Readonly<{ code: string; class: string; en: string; de: string }>;\n"
     "/** 🗂️ The fault classes (Rust `FaultClass`, wire names) — every declaration and catalog entry names one. */\n"
     "export const FAULT_CLASSES = [" + ", ".join(f'"{name}"' for name in CLASSES) + "] as const;\n"
     "const FAULT_CLASS_OF_VARIANT: ReadonlyMap<string, string> = new Map([" + ", ".join(f'["{variant}", "{name}"]' for variant, name in zip(VARIANTS, CLASSES)) + "]);"),
    (CENSUS, '  | "placeholder-mismatch" | "parameter-mismatch" | "framework-app-origin";', '  | "placeholder-mismatch" | "parameter-mismatch" | "framework-app-origin" | "class-missing" | "class-unknown";'),
    (CENSUS, """    const label = /^(?:[A-Za-z_]\\w*::)*LocalizedLabel::native\\s*\\(/u.exec(call.args[1] ?? "");
    const texts = label && call.args.length === 2 ? callArguments(call.args[1]!, label[0].length - 1) : null;
    const en = texts && texts.args.length === 2 && texts.end === call.args[1]!.length ? literalArgument(texts.args[0]) : null;
    const de = texts && texts.args.length === 2 && texts.end === call.args[1]!.length ? literalArgument(texts.args[1]) : null;
    if (en === null || de === null) violate(match.index!, "declaration-form", `.fault("${declared}", ${call.args[1] ?? ""}) — the text is LocalizedLabel::native("en", "de") of two literals`);
    else facts.declarations.push({ path, line: lineOf(match.index!), code: declared, en, de });""",
     """    const variant = /^(?:[A-Za-z_]\\w*::)*FaultClass::(\\w+)$/u.exec(call.args[1] ?? "");
    const klass = variant ? FAULT_CLASS_OF_VARIANT.get(variant[1]!) : undefined;
    if (call.args.length !== 3 || variant === null) {
      violate(match.index!, "class-missing", `.fault("${declared}", …) — the second argument is its FaultClass::<Class>`);
      continue;
    }
    if (klass === undefined) violate(match.index!, "class-unknown", `.fault("${declared}", FaultClass::${variant[1]}, …) — no such class`);
    const label = /^(?:[A-Za-z_]\\w*::)*LocalizedLabel::native\\s*\\(/u.exec(call.args[2] ?? "");
    const texts = label ? callArguments(call.args[2]!, label[0].length - 1) : null;
    const en = texts && texts.args.length === 2 && texts.end === call.args[2]!.length ? literalArgument(texts.args[0]) : null;
    const de = texts && texts.args.length === 2 && texts.end === call.args[2]!.length ? literalArgument(texts.args[1]) : null;
    if (en === null || de === null) violate(match.index!, "declaration-form", `.fault("${declared}", ${call.args[1]}, ${call.args[2] ?? ""}) — the text is LocalizedLabel::native("en", "de") of two literals`);
    else if (klass !== undefined) facts.declarations.push({ path, line: lineOf(match.index!), code: declared, class: klass, en, de });"""),
    (CENSUS, "      if (first && (first.en !== declaration.en || first.de !== declaration.de)) violations.push(",
     "      if (first && (first.class !== declaration.class || first.en !== declaration.en || first.de !== declaration.de)) violations.push("),
    (CENSUS, """    const { code, en, de } = (entry ?? {}) as Record<string, unknown>;
    if (typeof code !== "string" || typeof en !== "string" || typeof de !== "string") {
      violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-shape", detail: `entry ${JSON.stringify(entry).slice(0, 120)}` });
      continue;
    }
    if (entries.has(code)) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-duplicate", detail: code });
    entries.set(code, { code, en, de });""",
     """    const { code, class: klass, en, de } = (entry ?? {}) as Record<string, unknown>;
    if (typeof code !== "string" || typeof en !== "string" || typeof de !== "string") {
      violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-shape", detail: `entry ${JSON.stringify(entry).slice(0, 120)}` });
      continue;
    }
    if (typeof klass !== "string") violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "class-missing", detail: `${code} names no class` });
    else if (!(FAULT_CLASSES as readonly string[]).includes(klass)) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "class-unknown", detail: `${code}: ${klass}` });
    if (entries.has(code)) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-duplicate", detail: code });
    entries.set(code, { code, class: typeof klass === "string" ? klass : "", en, de });"""),
]


def projection(text: str) -> str:
    """📽️ The owned TS projection: a `FaultClass` entry before `FaultDefinition`, and `FaultDefinition` renders `class`."""
    entry = '        SchemaMetadata {\n            name: "FaultClass",\n            version: 1,\n            typescript: r####"' + CLASS_TS + '"####,\n        },\n'
    anchor = '        SchemaMetadata {\n            name: "FaultDefinition",\n'
    if 'name: "FaultClass"' not in text:
        assert text.count(anchor) == 1
        text = text.replace(anchor, entry + anchor)
    return render_definition(text)


def render_definition(text: str) -> str:
    old = "export type FaultDefinition = { code: string,\n/**\n * 🗣️ Manifest-level"
    new = "export type FaultDefinition = { code: string, class: FaultClass,\n/**\n * 🗣️ Manifest-level"
    doc_old = " * `{name}` placeholders, never from free text. Declared with the builder's `.fault(code, text)`; `parameters` are the\n * text's placeholder names."
    doc_new = " * `{name}` placeholders, never from free text, and answered by its `class`. Declared with the builder's\n * `.fault(code, class, text)`; `parameters` are the text's placeholder names."
    if new not in text:
        assert text.count(old) == 1, "FaultDefinition rendering"
        text = text.replace(old, new)
    if doc_new not in text:
        assert text.count(doc_old) == 1, "FaultDefinition doc"
        text = text.replace(doc_old, doc_new)
    return text


def generated(text: str) -> str:
    """🪪️ The generated manifest TS: `FaultClass` before `FaultDefinition`'s doc, `FaultDefinition` renders `class`."""
    if "export type FaultClass =" not in text:
        anchor = text.rindex("/**", 0, text.index("export type FaultDefinition"))
        text = text[:anchor] + CLASS_TS + "\n\n" + text[anchor:]
    return render_definition(text)


def record_schema(text: str) -> str:
    schema = json.loads(text)
    if "class" not in schema["properties"]:
        schema["required"].insert(schema["required"].index("origin") + 1, "class")
        properties = list(schema["properties"].items())
        at = [name for name, _ in properties].index("origin") + 1
        properties.insert(at, ("class", {"enum": CLASSES}))
        schema["properties"] = dict(properties)
        schema["description"] = schema["description"].replace("`origin` who refused;", "`origin` who refused; `class` how a host and the agent gateway answer it (declared with the code);")
    return json.dumps(schema, ensure_ascii=False, indent=2) + "\n"


def fault_text_fixture(text: str) -> str:
    """🧫️ The fault-text corpus (Rust `fault_text` law, host `faultTextV1`): every catalog entry and declaration names its class."""
    fixture = json.loads(text)
    classes = {"interactive-job.cancelled": "cancelled", "plugin.command.unknown": "input-invalid", "process3d.media.export-format": "input-invalid"}
    for entry in fixture["catalog"]["faults"]:
        entry.setdefault("class", classes.get(entry["code"], "precondition-failed"))
        entry.update({key: entry.pop(key) for key in ("en", "de")})
    for entry in fixture["declared"]:
        entry.setdefault("class", classes.get(entry["code"], "precondition-failed"))
        entry.update({"text": entry.pop("text")})
    return json.dumps(fixture, ensure_ascii=False, indent=2) + "\n"


def record_fixture(text: str) -> str:
    """🧫️ Every record case states the class of its fault and record; every decode case's bytes carry one; a record
    without a class, or with an unknown one, decodes to nothing."""
    fixture = json.loads(text)
    for case in fixture["records"]:
        case["fault"].setdefault("class", "internal" if case["record"].get("code") == fixture["limits"]["untypedCode"] else "precondition-failed")
        case["record"].setdefault("class", case["fault"]["class"])
    negative = ("a record without its class is no record", "a record with an unknown class is no record")
    for case in fixture["decodes"]:
        if case["name"] not in negative and '"class"' not in case["bytes"] and re.search(r'"origin":"[a-z]+"', case["bytes"]):
            case["bytes"] = re.sub(r'("origin":"[a-z]+")', r'\1,"class":"unavailable"', case["bytes"], count=1)
            if case["record"] is not None:
                case["record"].setdefault("class", "unavailable")
    for name, bytes_ in (("a record without its class is no record", '{"schema":"semio.typed-operation-fault.v1","code":"toolRun.busy","origin":"framework","message":"the fill run is busy","parameters":[]}'),
                         ("a record with an unknown class is no record", '{"schema":"semio.typed-operation-fault.v1","code":"toolRun.busy","origin":"framework","class":"busy","message":"the fill run is busy","parameters":[]}')):
        if all(case["name"] != name for case in fixture["decodes"]):
            fixture["decodes"].append({"name": name, "bytes": bytes_, "record": None})
    return json.dumps(fixture, ensure_ascii=False, indent=2) + "\n"


def census_fixture(text: str) -> str:
    """🧫️ The census self-test: every `.fault(` case declares a class, every catalog entry names one; two new cases."""
    text = re.sub(r'\.fault\((\\"[^\\"]+\\"), LocalizedLabel::native\(', r'.fault(\1, FaultClass::PreconditionFailed, LocalizedLabel::native(', text)
    text = re.sub(r'\{"code": ("[^"]+"), "en":', r'{"code": \1, "class": "precondition-failed", "en":', text)
    text = text.replace('.fault(\\"dice.cup.empty\\", label)', '.fault(\\"dice.cup.empty\\", FaultClass::PreconditionFailed, label)')
    origin_case = 'Fault::new(FaultOrigin::Framework, FaultCode::new(\\"app.example.unknown\\"), \\"m\\")\\n}\\n"'
    if origin_case in text and "fn is_app" not in text:
        text = text.replace(origin_case, 'Fault::new(FaultOrigin::Framework, FaultCode::new(\\"app.example.unknown\\"), \\"m\\")\\n}\\nfn is_app(fault: &Fault) -> bool {\\n    matches!(fault.origin, FaultOrigin::App)\\n}\\n"', 1)
    fixture = json.loads(text)
    names = {case["name"] for case in fixture["faultFacts"]}
    cases = [
        {"name": "a declaration names its class: a missing class and an unknown one break the law", "path": "✏️s/🔌️plugins/🎲️dice/🗿️artifacts/🎲️cup/🦀️.rs",
         "text": "fn d(b: AppBuilder) -> AppBuilder {\n    b.fault(\"dice.cup.empty\",LocalizedLabel::native(\"The cup is empty.\", \"Der Becher ist leer.\"))\n    .fault(\"dice.cup.full\", FaultClass::Busy, LocalizedLabel::native(\"The cup is full.\", \"Der Becher ist voll.\"))\n    .fault(\"dice.cup.lost\", semio_framework_plugin::FaultClass::Internal, LocalizedLabel::native(\"The cup is lost.\", \"Der Becher ist verloren.\"))\n}\n",
         "expected": {"raises": [], "constRaises": [], "consts": [], "declarations": [{"line": 4, "code": "dice.cup.lost", "en": "The cup is lost.", "de": "Der Becher ist verloren."}], "violations": [{"line": 2, "rule": "class-missing"}, {"line": 3, "rule": "class-unknown"}]}},
    ]
    added = [case for case in cases if case["name"] not in names]
    end = '\n  ],\n  "faultLaw": ['
    assert text.count(end) == 1
    return text.replace(end, "".join(",\n    " + json.dumps(case, ensure_ascii=False) for case in added) + end) if added else text


MANIFEST_LAW_EDIT = ("FaultDefinition { code: entry[\"code\"].as_str().expect(\"declared code\").to_string(), text,",
                     "FaultDefinition { code: entry[\"code\"].as_str().expect(\"declared code\").to_string(), class: serde_json::from_value(entry[\"class\"].clone()).expect(\"declared class\"), text,")
MANIFEST_LAW_ADD = """

/// ⚖️ LAW: the catalog class scanner (`catalog_fault_class`, what a guest reads) answers every entry exactly as the parsed
/// catalog does, and a code the catalog lacks has no class.
#[test]
fn catalog_classes_scan_as_parsed() {
    for entry in &framework_fault_catalog().faults {
        assert_eq!(catalog_fault_class(&entry.code), Some(entry.class), "{}", entry.code);
    }
    assert_eq!(catalog_fault_class("no.such.code"), None);
    assert_eq!(declared_fault_class("no.such.code", false), FaultClass::Internal);
}
"""


def main(dry_run: bool) -> None:
    texts: dict[str, str] = {}

    def read(rel: str) -> str:
        return texts.setdefault(rel, (OVERLAY / rel).read_text())

    for rel, old, new in EDITS:
        text = read(rel)
        if new in text and (old in new or old not in text):
            continue
        assert text.count(old) == 1, (rel, old[:100], text.count(old))
        texts[rel] = text.replace(old, new)
        print("apply", rel.split("/")[-2], new.splitlines()[0][:90] if new.strip() else "")
    for rel, transform in ((PROJECTION, projection), (GENERATED, generated), (RECORD_SCHEMA, record_schema), (FAULT_TEXT_FIXTURE, fault_text_fixture), (RECORD_FIXTURE, record_fixture), (CENSUS_FIXTURE, census_fixture)):
        before = read(rel)
        texts[rel] = transform(before)
        print("transform", rel.split("/")[-2], "changed" if texts[rel] != before else "unchanged")
    law = read(MANIFEST_LAW)
    if MANIFEST_LAW_EDIT[1] not in law:
        assert law.count(MANIFEST_LAW_EDIT[0]) == 1
        law = law.replace(MANIFEST_LAW_EDIT[0], MANIFEST_LAW_EDIT[1])
    if "fn catalog_classes_scan_as_parsed" not in law:
        law = law.rstrip("\n") + MANIFEST_LAW_ADD
    texts[MANIFEST_LAW] = law
    if not dry_run:
        for rel, text in texts.items():
            if (OVERLAY / rel).read_text() != text:
                (OVERLAY / rel).write_text(text)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
