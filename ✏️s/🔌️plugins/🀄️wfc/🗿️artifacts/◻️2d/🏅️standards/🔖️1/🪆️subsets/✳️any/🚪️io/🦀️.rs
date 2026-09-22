//! 🚪️ IO `s.wfc.wfc2d` (1/✳️any) — `io() -> IoDeclaration`: the native codec and nothing else.
//!
//! This root owns three native-codec facets, each in its own child directory: `📸️snapshot/📝️text` +
//! `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack` impls for `Wfc2dSnapshot`),
//! `🧬️mutations/📝️text` + `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for
//! `Wfc2dMutation`), and `🔺️diff/📝️text` + `🔺️diff/💾️binary` (grammar/protocol spec assets only —
//! the diff never rides its own envelope; its algebra lives in `🧬️schema/🔺️diff/🦀️.rs`).
//!
//! `entries: &[]` is deliberate and honest: a WFC problem spec has no foreign interchange format to
//! import from or export to. Its own DSL and pack envelope ARE the format. When a `.wfc2d` board
//! should round-trip through an image or a drawing, that is a NEW typed `Serializer`/`Deserializer`
//! leaf under `📥️import`/`📤️export`, not a composer row on the retired `ComposerEntry` channel.

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &[]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &[]
}

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::{diff, mutations, snapshot};
    use crate::{Wfc2dMutation, Wfc2dSnapshot, WFC_2D_DOCUMENT_SCHEMA};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🗣️ The five hand-authored `dsl::LanguageSpec`s this subset carries — `OnceLock` because
    /// `dsl::passthrough_hooks` is not `const fn`. Indices: 0=document 1=op 2=diff 3=pack 4=spr.
    fn languages() -> &'static [dsl::LanguageSpec; 5] {
        static LANGUAGES: OnceLock<[dsl::LanguageSpec; 5]> = OnceLock::new();
        LANGUAGES.get_or_init(|| {
            [
                dsl::LanguageSpec {
                    id: "wfc.wfc2d",
                    extension: Some("wfc2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.wfc2d"),
                },
                dsl::LanguageSpec {
                    id: "wfc.wfc2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.wfc2d.op"),
                },
                dsl::LanguageSpec {
                    id: "wfc.wfc2d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("wfc.wfc2d.diff"),
                },
                dsl::LanguageSpec {
                    id: "wfc.wfc2d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.wfc2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "wfc.wfc2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.wfc2d.spr"),
                },
            ]
        })
    }

    let langs = languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Wfc2dSnapshot, Wfc2dMutation>(WFC_2D_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}
//#endregion 🔖️IoDeclaration

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
