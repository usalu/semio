"""✂️ C12 window-3 set — range-text operations for concurrent typing (design: 📓️wp-c12.md "Item 2").

Idempotent: every new file is copied from `tree/` (refused when a different file already exists), every hunk replaces one
exact anchor that must occur exactly once (a hunk whose replacement is already present counts as applied).
usage: python3 c12-splice-patch.py [--apply]   (default: dry run; exit 1 when any hunk or file cannot be placed)"""
import os
import sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
TREE = os.path.join(HERE, "tree")
APPLY = "--apply" in sys.argv

SCENE = "🧰️framework/🔨️modules/🖱️ui/🎬️scene"
WRITER = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer"
ANY = WRITER + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
EDITOR = ANY + "/✏️editor/🦀️.rs"
TRANSIENT = ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧬️schema"
IO_BIN = ANY + "/🚪️io/🧬️mutations/💾️binary/🦀️.rs"
LEAF_ROOT = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️splice-text"

HUNKS = [
    (SCENE + "/📦️packages/🦀️rust/🦀️.rs",
     "pub use framing::{Canvas2dFraming, Canvas2dFrameCamera};\n",
     "pub use framing::{Canvas2dFraming, Canvas2dFrameCamera};\n\n#[path = \"../../✂️text-splice/🦀️.rs\"]\nmod text_splice;\npub use text_splice::{rebase_text_edits, AppliedTextSplice, LocatedTextSplice, TextSplice, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS};\n"),
    ("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs",
     "SceneDoc, TableScene, TextEditorScene, TiledMapScene,",
     "SceneDoc, TableScene, TextEditorScene, TextSplice, LocatedTextSplice, AppliedTextSplice, TEXT_SPLICE_CONTEXT_SCALARS, TiledMapScene,"),
    (SCENE + "/🟦️.ts",
     "import type { Viewport2d } from \"../🪟️viewport/◻️2d/🧬️schema/🟦️.ts\";\n",
     "import type { Viewport2d } from \"../🪟️viewport/◻️2d/🧬️schema/🟦️.ts\";\nexport * from \"./✂️text-splice/🟦️.ts\";\n"),
    (SCENE + "/🟦️.ts",
     "export const textEditorActions = {\n  edit: \"textEdit\",\n",
     "export const textEditorActions = {\n  edit: \"textEdit\",\n  splice: \"textSplice\",\n"),
    (SCENE + "/📦️packages/🟦️typescript/📜️script.ts",
     "resolve(import.meta.dir, \"../../🧪️tests/🚚️table-lanes/🟦️.test.ts\"),",
     "resolve(import.meta.dir, \"../../🧪️tests/🚚️table-lanes/🟦️.test.ts\"), resolve(import.meta.dir, \"../../🧪️tests/✂️text-splice/🟦️.test.ts\"),"),
    (WRITER + "/🦀️.rs",
     "                            #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-text/📝️text/🦀️.rs\"]\n                            pub mod text;\n                        }\n",
     "                            #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-text/📝️text/🦀️.rs\"]\n                            pub mod text;\n                        }\n"
     "                        #[path = \".\"]\n                        pub mod splice_text {\n"
     f"                            #[path = \"{LEAF_ROOT}/🦀️.rs\"]\n                            mod component;\n                            pub use component::*;\n"
     f"                            #[path = \"{LEAF_ROOT}/💾️binary/🦀️.rs\"]\n                            pub mod binary;\n"
     f"                            #[path = \"{LEAF_ROOT}/🔺️diff/🦀️.rs\"]\n                            pub mod diff;\n"
     f"                            #[path = \"{LEAF_ROOT}/↩️inverse/🦀️.rs\"]\n                            pub mod inverse;\n"
     f"                            #[path = \"{LEAF_ROOT}/📝️text/🦀️.rs\"]\n                            pub mod text;\n                        }}\n"),
    (WRITER + "/🦀️.rs",
     "            #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🦀️.rs\"]\n            pub mod text_edit;\n",
     "            #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🦀️.rs\"]\n            pub mod text_edit;\n            #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️text-splice/🦀️.rs\"]\n            pub mod text_splice;\n"),
    (MUT + "/🦀️.rs",
     "pub use super::edit_text::{edit_text, EditText};\n",
     "pub use super::edit_text::{edit_text, EditText};\npub use super::splice_text::{splice_text, SpliceText};\n"),
    (MUT + "/🦀️.rs",
     "    EditText(EditText),\n}\n",
     "    EditText(EditText),\n    SpliceText(SpliceText),\n}\n"),
    (MUT + "/🔣️.json",
     "      \"$ref\": \"https://json.schemas.assets.semio-tech.com/s/writer/writer/mutation/rename-writer/schema.json\"\n    }\n",
     "      \"$ref\": \"https://json.schemas.assets.semio-tech.com/s/writer/writer/mutation/rename-writer/schema.json\"\n    },\n    {\n      \"$ref\": \"https://json.schemas.assets.semio-tech.com/s/writer/writer/mutation/splice-text/schema.json\"\n    }\n"),
    (MUT + "/🟦️.ts",
     "import type { RenameWriter } from \"./🏷️rename-writer/🟦️.ts\";\n",
     "import type { RenameWriter } from \"./🏷️rename-writer/🟦️.ts\";\nimport type { SpliceText } from \"./✂️splice-text/🟦️.ts\";\n"),
    (MUT + "/🟦️.ts",
     "  | ({ mutation: \"editText\" } & EditText);",
     "  | ({ mutation: \"editText\" } & EditText)\n  | ({ mutation: \"spliceText\" } & SpliceText);"),
    (MUT + "/🛰️.proto",
     "message EditText { string text = 1; }\n",
     "message EditText { string text = 1; }\nmessage SpliceText { uint32 start = 1; string deleted = 2; string insert = 3; string before = 4; string after = 5; }\n"),
    (MUT + "/🛰️.proto",
     "    EditText edit_text = 4;\n",
     "    EditText edit_text = 4;\n    SpliceText splice_text = 5;\n"),
    (MUT + "/🕸️.graphql",
     "input EditTextInput { text: String! }\n",
     "input EditTextInput { text: String! }\ninput SpliceTextInput { start: Int! deleted: String! insert: String! before: String! after: String! }\n"),
    (MUT + "/🕸️.graphql",
     "  editText: EditTextInput\n}",
     "  editText: EditTextInput\n  spliceText: SpliceTextInput\n}"),
    (MUT + "/💾️binary/🦀️.rs",
     "(\"edit-text\", super::edit_text::binary::BINARY_TAG)];",
     "(\"edit-text\", super::edit_text::binary::BINARY_TAG), (\"splice-text\", super::splice_text::binary::BINARY_TAG)];"),
    (MUT + "/📝️text/🦀️.rs",
     "(\"edit-text\", super::edit_text::text::TEXT_OPCODE)];",
     "(\"edit-text\", super::edit_text::text::TEXT_OPCODE), (\"splice-text\", super::splice_text::text::TEXT_OPCODE)];"),
    (ANY + "/🚪️io/🧬️mutations/📝️text/🦀️.rs",
     "rename_writer, ChangeLanguage, ChangeUri, EditText, RenameWriter, WriterMutation};",
     "rename_writer, splice_text, ChangeLanguage, ChangeUri, EditText, RenameWriter, SpliceText, WriterMutation};"),
    (ANY + "/🔮️oracles/🔣️.json",
     "        \"change-language\",\n        \"edit-text\"\n      ]",
     "        \"change-language\",\n        \"edit-text\",\n        \"splice-text\"\n      ]"),
    # 🗂️ io binary codec: SpliceText's five fields and a multi-field payload per kind.
    (IO_BIN,
     "                WriterMutation::EditText(value) => &mut value.text,\n            };\n            if field.len() > maximum_bytes {\n                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n            }\n            let released_bytes = field.len();\n            drop(std::mem::take(field));\n",
     "                WriterMutation::EditText(value) => &mut value.text,\n                WriterMutation::SpliceText(value) => {\n                    let released_bytes = value.deleted.len() + value.insert.len() + value.before.len() + value.after.len();\n                    if released_bytes > maximum_bytes {\n                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n                    }\n                    for field in [&mut value.deleted, &mut value.insert, &mut value.before, &mut value.after] {\n                        drop(std::mem::take(field));\n                    }\n                    self.field_released = true;\n                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });\n                }\n            };\n            if field.len() > maximum_bytes {\n                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n            }\n            let released_bytes = field.len();\n            drop(std::mem::take(field));\n"),
    (IO_BIN,
     "    store::OwnedSchemaFieldSpec { id: 5, key: \"text\", required: false },\n];",
     "    store::OwnedSchemaFieldSpec { id: 5, key: \"text\", required: false },\n    store::OwnedSchemaFieldSpec { id: 6, key: \"start\", required: false },\n    store::OwnedSchemaFieldSpec { id: 7, key: \"deleted\", required: false },\n    store::OwnedSchemaFieldSpec { id: 8, key: \"insert\", required: false },\n    store::OwnedSchemaFieldSpec { id: 9, key: \"before\", required: false },\n    store::OwnedSchemaFieldSpec { id: 10, key: \"after\", required: false },\n];\n\n/// ✂️ `SpliceText`'s four string fields (ids 7–10) and its scalar `start` (id 6), gathered before the record completes.\n#[derive(Default)]\nstruct WriterSplicePayload {\n    start: Option<u32>,\n    strings: [Option<String>; 4],\n}"),
    (IO_BIN,
     "    EditText,\n}\n\nstruct WriterMutationString {",
     "    EditText,\n    SpliceText,\n}\n\nstruct WriterMutationString {"),
    (IO_BIN,
     "    payload: std::mem::ManuallyDrop<Option<String>>,\n    value: std::mem::ManuallyDrop<Option<WriterMutation>>,",
     "    payload: std::mem::ManuallyDrop<Option<String>>,\n    splice: WriterSplicePayload,\n    value: std::mem::ManuallyDrop<Option<WriterMutation>>,"),
    (IO_BIN,
     "            payload: std::mem::ManuallyDrop::new(None),\n            value: std::mem::ManuallyDrop::new(None),",
     "            payload: std::mem::ManuallyDrop::new(None),\n            splice: WriterSplicePayload::default(),\n            value: std::mem::ManuallyDrop::new(None),"),
    (IO_BIN,
     "                Some(\"editText\") => WriterMutationKind::EditText,\n",
     "                Some(\"editText\") => WriterMutationKind::EditText,\n                Some(\"spliceText\") => WriterMutationKind::SpliceText,\n"),
    (IO_BIN,
     "        if self.payload_field.replace(field_id).is_some() {\n            return Err(self.diagnostic(\"writer-envelope.duplicate-mutation-payload\", 0));\n        }\n        *self.payload = authority.take_string();\n        Ok(())\n    }\n",
     "        if (7..=10).contains(&field_id) {\n            let slot = &mut self.splice.strings[usize::from(field_id - 7)];\n            if slot.is_some() {\n                return Err(self.diagnostic(\"writer-envelope.duplicate-mutation-payload\", 0));\n            }\n            *slot = authority.take_string();\n            return Ok(());\n        }\n        if self.payload_field.replace(field_id).is_some() {\n            return Err(self.diagnostic(\"writer-envelope.duplicate-mutation-payload\", 0));\n        }\n        *self.payload = authority.take_string();\n        Ok(())\n    }\n\n    /// 🔢️ `SpliceText.start`: the one scalar field of the mutation record, a non-negative integer that fits `u32`.\n    fn finish_start(&mut self, token: store::OwnedSchemaToken, source: &store::OwnedSchemaRecordCursor) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {\n        let mut digits = [0u8; 10];\n        let length = usize::try_from(token.end - token.start).map_err(|_| self.diagnostic(\"writer-envelope.splice-start\", token.start))?;\n        if token.kind != store::OwnedSchemaTokenKind::Number || length == 0 || length > digits.len() || self.splice.start.is_some() {\n            return Err(self.diagnostic(\"writer-envelope.splice-start\", token.start));\n        }\n        source.copy_token_bytes(token, 0, &mut digits[..length]);\n        let text = std::str::from_utf8(&digits[..length]).map_err(|_| self.diagnostic(\"writer-envelope.splice-start\", token.start))?;\n        self.splice.start = Some(text.parse::<u32>().map_err(|_| self.diagnostic(\"writer-envelope.splice-start\", token.start))?);\n        Ok(())\n    }\n"),
    (IO_BIN,
     "            WriterMutationKind::EditText => 5,\n        };\n        if self.payload_field != Some(expected) {\n            return Err(self.diagnostic(\"writer-envelope.mutation-payload-mismatch\", 0));\n        }\n",
     "            WriterMutationKind::EditText => 5,\n            WriterMutationKind::SpliceText => {\n                let [deleted, insert, before, after] = std::mem::take(&mut self.splice.strings);\n                let (Some(start), Some(deleted), Some(insert), Some(before), Some(after), None) = (self.splice.start.take(), deleted, insert, before, after, self.payload_field) else {\n                    return Err(self.diagnostic(\"writer-envelope.mutation-payload-mismatch\", 0));\n                };\n                *self.value = Some(WriterMutation::SpliceText(schema::mutations::SpliceText { start, deleted, insert, before, after }));\n                return Ok(());\n            }\n        };\n        if self.payload_field != Some(expected) || self.splice.start.is_some() || self.splice.strings.iter().any(Option::is_some) {\n            return Err(self.diagnostic(\"writer-envelope.mutation-payload-mismatch\", 0));\n        }\n"),
    (IO_BIN,
     "            WriterMutationKind::EditText => WriterMutation::EditText(schema::mutations::EditText { text: payload }),\n",
     "            WriterMutationKind::EditText => WriterMutation::EditText(schema::mutations::EditText { text: payload }),\n            WriterMutationKind::SpliceText => return Err(self.diagnostic(\"writer-envelope.mutation-payload-mismatch\", 0)),\n"),
    (IO_BIN,
     "            store::OwnedSchemaNestedRecordStep::FieldToken { token, .. } => Err(self.diagnostic(\"writer-envelope.mutation-field-scalar\", token.start)),\n",
     "            store::OwnedSchemaNestedRecordStep::FieldToken { field_id: 6, token, .. } => {\n                self.finish_start(token, source)?;\n                Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)\n            }\n            store::OwnedSchemaNestedRecordStep::FieldToken { token, .. } => Err(self.diagnostic(\"writer-envelope.mutation-field-scalar\", token.start)),\n"),
    (IO_BIN,
     "        if let Some(payload) = self.payload.as_ref() {\n            if payload.len() > maximum_bytes {",
     "        if let Some(index) = self.splice.strings.iter().position(Option::is_some) {\n            let released_bytes = self.splice.strings[index].as_ref().map_or(0, String::len);\n            if released_bytes > maximum_bytes {\n                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n            }\n            drop(self.splice.strings[index].take());\n            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });\n        }\n        if let Some(payload) = self.payload.as_ref() {\n            if payload.len() > maximum_bytes {"),
    (IO_BIN,
     "        self.terminal && self.active.is_none() && self.payload.is_none() && self.value.is_none() && self.retirement.is_none()",
     "        self.terminal && self.active.is_none() && self.payload.is_none() && self.splice.strings.iter().all(Option::is_none) && self.value.is_none() && self.retirement.is_none()"),
    # ✏️ editor: the `textSplice` verb, its retained emission and its declarations.
    (EDITOR,
     "        \"engagementSubmit\" as \"engagement-submit\" => engagement_submit::EngagementSubmit,\n    }\n}",
     "        \"engagementSubmit\" as \"engagement-submit\" => engagement_submit::EngagementSubmit,\n        \"textSplice\" as \"text-splice\" => text_splice::TextSplice,\n    }\n}"),
    (EDITOR,
     "    \"engagementInput\",\n    \"engagementSubmit\",\n];\nconst WRITER_COMMAND_PAYLOAD_SCHEMA",
     "    \"engagementInput\",\n    \"engagementSubmit\",\n    \"textSplice\",\n];\nconst WRITER_COMMAND_PAYLOAD_SCHEMA"),
    (EDITOR,
     "            \"engagementInput\",\n            \"engagementSubmit\",\n                    ]",
     "            \"engagementInput\",\n            \"engagementSubmit\",\n            \"textSplice\",\n                    ]"),
    (EDITOR,
     "        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: \"setText\", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },\n",
     "        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: \"setText\", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },\n        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: \"textSplice\", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },\n"),
    (EDITOR,
     "            WriterCommand::SetText(payload) => emit = Emit::mutations(vec![WriterMutation::EditText(crate::op::EditText { text: payload.text })]),\n",
     "            WriterCommand::SetText(payload) => emit = Emit::mutations(vec![WriterMutation::EditText(crate::op::EditText { text: payload.text })]),\n            WriterCommand::TextSplice(payload) => {\n                let view = self.view_state.as_ref().ok_or(\"Writer typing requires its concrete window context\")?;\n                let selection = crate::WriterEditorSelection { start: payload.anchor, end: payload.caret, splice: payload.seq };\n                ephemeral.window_transient.push(\n                    main::transient::addressed(view, WriterMainWindowTransientMutation::SetEditorSelection(main::transient::SetEditorSelection { selection: Some(selection) })).map_err(|_| \"Writer typing rejected its concrete window context\")?,\n                );\n                emit = Emit::amend(vec![crate::op::splice_text(payload.splice())], \"writer-text-edit\");\n            }\n"),
    (EDITOR,
     "                let selection = crate::WriterEditorSelection { start: payload.start, end: payload.end };\n",
     "                let selection = crate::WriterEditorSelection { start: payload.start, end: payload.end, splice: payload.splice };\n"),
    (EDITOR,
     "            \"textSelect\" => Ok(WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: number_arg(&[\"start\"]).unwrap_or_default() as usize, end: number_arg(&[\"end\"]).unwrap_or_default() as usize })),\n",
     "            \"textSelect\" => Ok(WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: number_arg(&[\"start\"]).unwrap_or_default() as usize, end: number_arg(&[\"end\"]).unwrap_or_default() as usize, splice: number_arg(&[\"splice\"]).unwrap_or_default() as u64 })),\n"
     "            \"textSplice\" => Ok(WriterCommand::TextSplice(text_splice::TextSplice {\n                start: u32::try_from(number_arg(&[\"start\"]).unwrap_or_default() as u64).map_err(|_| Fault::from(\"writer textSplice start exceeds u32\"))?,\n                deleted: text_arg(&[\"deleted\"]).unwrap_or_default(),\n                insert: text_arg(&[\"insert\"]).unwrap_or_default(),\n                before: text_arg(&[\"before\"]).unwrap_or_default(),\n                after: text_arg(&[\"after\"]).unwrap_or_default(),\n                seq: number_arg(&[\"seq\"]).unwrap_or_default() as u64,\n                anchor: number_arg(&[\"anchor\"]).unwrap_or_default() as usize,\n                caret: number_arg(&[\"caret\"]).unwrap_or_default() as usize,\n            })),\n"),
    (EDITOR,
     "fn admit_writer_artifact_mutation(mutation: &WriterMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {\n    let WriterMutation::EditText(payload) = mutation else {\n        return Err(\"Writer retained Artifact preparation only admits the exact EditText cohort\".into());\n    };\n    if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES {\n        return Err(\"Writer EditText exceeds its fixed retained preparation envelope\".into());\n    }\n    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(payload.text.len()))\n}",
     "fn admit_writer_artifact_mutation(mutation: &WriterMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {\n    let bytes = match mutation {\n        WriterMutation::EditText(payload) => payload.text.len(),\n        WriterMutation::SpliceText(payload) => payload.deleted.len() + payload.insert.len() + payload.before.len() + payload.after.len(),\n        _ => return Err(\"Writer retained Artifact preparation only admits the exact EditText and SpliceText cohort\".into()),\n    };\n    if bytes > MAX_WRITER_COMMAND_TEXT_BYTES {\n        return Err(\"Writer text edit exceeds its fixed retained preparation envelope\".into());\n    }\n    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(bytes))\n}"),
    (EDITOR,
     "            .action_with(writer_hidden_operation(\"textEdit\", LocalizedLabel::native(\"Edit Text\", \"Text bearbeiten\"), \"typography\"))\n",
     "            .action_with(writer_hidden_operation(\"textEdit\", LocalizedLabel::native(\"Edit Text\", \"Text bearbeiten\"), \"typography\"))\n            // ✂️ The typing verb of a splice-typing host (`settingsJson.typing`): one range-text operation per typed run, relocated\n            // by its context, so two humans typing at once keep both runs (ticket 26/09/23 C12).\n            .action_with(writer_hidden_operation(\"textSplice\", LocalizedLabel::native(\"Type Text\", \"Text tippen\"), \"typography\"))\n"),
    (EDITOR,
     "            .action_interactive_job(\"textEdit\", InteractiveJobClassification::Migrated)\n",
     "            .action_interactive_job(\"textEdit\", InteractiveJobClassification::Migrated)\n            .action_interactive_job(\"textSplice\", InteractiveJobClassification::Migrated)\n"),
    (EDITOR,
     "            .action_audience(\"textEdit\", semio_framework_plugin::CapabilityAudience::Input)\n",
     "            .action_audience(\"textEdit\", semio_framework_plugin::CapabilityAudience::Input)\n            .action_audience(\"textSplice\", semio_framework_plugin::CapabilityAudience::Input)\n            .action_describe(\"textSplice\", LocalizedLabel::native(\"Replaces one range of the text as its author saw it; the change lands between the text its author saw around it, even when others edited the text meanwhile.\", \"Ersetzt einen Textbereich so, wie ihn sein Autor sah; die Änderung landet zwischen dem Text, den der Autor darum herum sah, auch wenn andere den Text inzwischen bearbeitet haben.\"))\n"),
    (EDITOR,
     "#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\npub(crate) mod unit_tests;\n",
     "#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\npub(crate) mod unit_tests;\n#[cfg(test)]\n#[path = \"🧪️tests/✂️concurrent-typing/🦀️.rs\"]\nmod concurrent_typing_tests;\n"),
    (ANY + "/✏️editor/🎮️commands/🗂️set-editor-selection/🦀️.rs",
     "pub struct SetEditorSelection {\n    pub start: usize,\n    pub end: usize,\n}",
     "pub struct SetEditorSelection {\n    pub start: usize,\n    pub end: usize,\n    pub splice: u64,\n}"),
    # 🫧️ window transient: the selection echoes the last host splice `seq` this window applied.
    (TRANSIENT + "/🦀️.rs",
     "pub struct WriterEditorSelection {\n    pub start: usize,\n    pub end: usize,\n}",
     "pub struct WriterEditorSelection {\n    pub start: usize,\n    pub end: usize,\n    /// ✂️ The last `textSplice` `seq` of this window's host that this window applied (0: none).\n    pub splice: u64,\n}"),
    (TRANSIENT + "/🔣️.json",
     "          \"required\": [\"start\", \"end\"],\n          \"properties\": {\n            \"start\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"end\": { \"type\": \"integer\", \"minimum\": 0 }\n          }",
     "          \"required\": [\"start\", \"end\", \"splice\"],\n          \"properties\": {\n            \"start\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"end\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"splice\": { \"type\": \"integer\", \"minimum\": 0 }\n          }"),
    (TRANSIENT + "/🟦️.ts",
     "export interface WriterEditorSelection { start: number; end: number }",
     "export interface WriterEditorSelection { start: number; end: number; splice: number }"),
    (TRANSIENT + "/🟦️.ts",
     "!Number.isInteger((selection as Record<string, unknown>).end)))",
     "!Number.isInteger((selection as Record<string, unknown>).end) || !Number.isInteger((selection as Record<string, unknown>).splice)))"),
    (TRANSIENT + "/🛰️.proto",
     "message WriterEditorSelection { uint64 start = 1; uint64 end = 2; }",
     "message WriterEditorSelection { uint64 start = 1; uint64 end = 2; uint64 splice = 3; }"),
    (TRANSIENT + "/🔗️.graphql",
     "type WriterEditorSelection { start: Int! end: Int! }",
     "type WriterEditorSelection { start: Int! end: Int! splice: Int! }"),
    (TRANSIENT + "/🧬️mutations/📐️set-editor-selection/🧬️schema/🔣️.json",
     "            \"start\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"end\": { \"type\": \"integer\", \"minimum\": 0 }\n          },\n          \"required\": [\"start\", \"end\"],",
     "            \"start\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"end\": { \"type\": \"integer\", \"minimum\": 0 },\n            \"splice\": { \"type\": \"integer\", \"minimum\": 0 }\n          },\n          \"required\": [\"start\", \"end\", \"splice\"],"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🟦️.ts",
     "export interface WriterEditorSelection {\n  start: number;\n  end: number;\n}",
     "export interface WriterEditorSelection {\n  start: number;\n  end: number;\n  splice: number;\n}"),
    # 🪟️ the writer text scene declares splice typing and echoes the applied splice `seq`.
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️.rs",
     "    let selection = transient.editor_selection.clone().unwrap_or(crate::WriterEditorSelection { start: 0, end: 0 });\n    let cursor = selection.end;\n    let selection_json = Some(json!({ \"start\": selection.start, \"end\": selection.end }).to_string());\n",
     "    let selection = transient.editor_selection.clone().unwrap_or(crate::WriterEditorSelection { start: 0, end: 0, splice: 0 });\n    let cursor = selection.end;\n    let selection_json = Some(json!({ \"start\": selection.start, \"end\": selection.end, \"splice\": selection.splice }).to_string());\n"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️.rs",
     "            settings_json: Some(serde_json::to_string(&config.editor_settings).unwrap_or_else(|_| \"{}\".into())),\n",
     "            settings_json: Some(writer_settings_json(config)),\n"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️.rs",
     "//#endregion 🔖️Render\n",
     "\n/// ⌨️ The editor settings plus the typing contract (`TextEditorTypingV1`, `semio.ui.scene.text-splice.v1`): this window takes\n/// one `textSplice` per typed run and echoes the applied `seq` in `selectionJson.splice`.\nfn writer_settings_json(config: &WriterMainWindowConfig) -> String {\n    let mut settings = serde_json::to_value(&config.editor_settings).unwrap_or_else(|_| json!({}));\n    if let Value::Object(map) = &mut settings {\n        map.insert(\"typing\".into(), json!({ \"mode\": \"splice\" }));\n    }\n    settings.to_string()\n}\n//#endregion 🔖️Render\n"),
    # 🧪️ struct literals and assertions that name the selection record.
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🦀️.rs",
     "(&right, WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 8 })),",
     "(&right, WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 8, splice: 0 })),"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🦀️.rs",
     "right_transient.editor_selection != Some(crate::WriterEditorSelection { start: 3, end: 8 })",
     "right_transient.editor_selection != Some(crate::WriterEditorSelection { start: 3, end: 8, splice: 0 })"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🦀️.rs",
     "!= serde_json::json!({ \"start\": 3, \"end\": 8 }) {",
     "!= serde_json::json!({ \"start\": 3, \"end\": 8, \"splice\": 0 }) {"),
    (ANY + "/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🦀️.rs",
     "!= serde_json::json!({ \"start\": 0, \"end\": 0 }) {",
     "!= serde_json::json!({ \"start\": 0, \"end\": 0, \"splice\": 0 }) {"),
    (ANY + "/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs",
     "set_editor_selection::SetEditorSelection { start, end: start })",
     "set_editor_selection::SetEditorSelection { start, end: start, splice: 0 })"),
    (ANY + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
     "WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 1, end: 2 }),",
     "WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 1, end: 2, splice: 0 }),"),
    (ANY + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
     "(\"editor-selection\", WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 0, end: 1 })),",
     "(\"editor-selection\", WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 0, end: 1, splice: 0 })),"),
    (ANY + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
     "WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 7 }),",
     "WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 7, splice: 0 }),"),
    (TRANSIENT + "/../📢️publication/🦀️.rs",
     "store::retirement::leaf((self.start, self.end))",
     "store::retirement::leaf((self.start, self.end, self.splice))"),
]


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def main():
    problems, planned = [], []
    for root, _, files in os.walk(TREE):
        for name in files:
            source = os.path.join(root, name)
            relative = os.path.relpath(source, TREE)
            target = os.path.join(REPO, relative)
            content = open(source, encoding="utf-8").read()
            if os.path.exists(target):
                if open(target, encoding="utf-8").read() == content:
                    planned.append(("file-present", relative))
                else:
                    problems.append(("file-differs", relative))
            else:
                planned.append(("file-new", relative))
    texts = {}
    for path, old, new in HUNKS:
        if not os.path.exists(os.path.join(REPO, path)):
            problems.append(("missing-file", path))
            continue
        text = texts.get(path) or read(path)
        if new in text and old not in text.replace(new, ""):
            planned.append(("hunk-present", path))
            continue
        count = text.count(old)
        if count != 1:
            problems.append((f"anchor-count-{count}", f"{path} :: {old[:80]!r}"))
            continue
        texts[path] = text.replace(old, new, 1)
        planned.append(("hunk", path))
    for kind, what in planned:
        print(f"OK   {kind:13} {what}")
    for kind, what in problems:
        print(f"FAIL {kind:13} {what}")
    print(f"{len(planned)} planned, {len(problems)} problem(s), mode={'apply' if APPLY else 'dry-run'}")
    if problems:
        sys.exit(1)
    if not APPLY:
        return
    for root, _, files in os.walk(TREE):
        for name in files:
            source = os.path.join(root, name)
            target = os.path.join(REPO, os.path.relpath(source, TREE))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(source, encoding="utf-8") as src, open(target, "w", encoding="utf-8") as dst:
                dst.write(src.read())
    for path, text in texts.items():
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(text)


main()
