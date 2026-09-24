"""📌️ G9 frozen-crate hunk: declares an inference's commit action on its published payload contract (framework plugin crate + manifest + TS projection), binds `s.wfc.bitmap.solve` to `pin-solution`, and switches the MCP resolver onto it. Apply only after the freeze lifts; compile-atomic set."""
import pathlib, re
ROOT = pathlib.Path("/Users/ueli/Documents/semio")

def edit(relative, pairs):
    path = ROOT / relative
    text = path.read_text()
    for old, new in pairs:
        assert text.count(old) == 1, (relative, old[:100], text.count(old))
        text = text.replace(old, new)
    pass
    print("would patch", relative)

edit("🧰️framework/🔨️modules/🛂️manifest/🦀️.rs", [
    ('''    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<InferenceArtifactBinding>,
}''', '''    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<InferenceArtifactBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<InferenceCommitBinding>,
}

/// 📌️ How one inference's result becomes a document edit: the artifact action that commits it. The
/// action receives the result's fields it declares as arguments, and runs through the ordinary edit
/// path — its own policy, undo and ledger — so a committed result is an edit like any other.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceCommitBinding {
    pub action: String,
}'''),
])

P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
edit(P, [
    ('''        pub progress_unit: &'static str,
        pub artifact_binding: Option<ArtifactInferenceDocumentBinding>,
    }''', '''        pub progress_unit: &'static str,
        pub artifact_binding: Option<ArtifactInferenceDocumentBinding>,
        pub commit: Option<ArtifactInferenceCommitBinding>,
    }

    /// 📌️ The `&'static str` twin of `semio_framework::InferenceCommitBinding`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ArtifactInferenceCommitBinding {
        pub action: &'static str,
    }'''),
    ('''                progress_unit: contract.progress_unit.to_owned(),
                artifact_binding: contract.artifact_binding.map(Into::into),
            }''', '''                progress_unit: contract.progress_unit.to_owned(),
                artifact_binding: contract.artifact_binding.map(Into::into),
                commit: contract.commit.map(|commit| semio_framework::InferenceCommitBinding { action: commit.action.to_owned() }),
            }'''),
    ('''        #[value(default, skip_serializing_if = "Option::is_none")]
        pub artifact_binding: Option<WireInferenceArtifactBinding>,
    }''', '''        #[value(default, skip_serializing_if = "Option::is_none")]
        pub artifact_binding: Option<WireInferenceArtifactBinding>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub commit: Option<WireInferenceCommitBinding>,
    }

    /// 📌️ Wire twin of [`ArtifactInferenceCommitBinding`].
    #[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
    #[value(rename_all = "camelCase", deny_unknown_fields)]
    pub struct WireInferenceCommitBinding {
        pub action: String,
    }'''),
    ('''                artifact_binding: contract.artifact_binding.map(|binding| WireInferenceArtifactBinding { field: binding.field.to_owned(), encoding: binding.encoding.to_owned(), required: binding.required }),
            }''', '''                artifact_binding: contract.artifact_binding.map(|binding| WireInferenceArtifactBinding { field: binding.field.to_owned(), encoding: binding.encoding.to_owned(), required: binding.required }),
                commit: contract.commit.map(|commit| WireInferenceCommitBinding { action: commit.action.to_owned() }),
            }'''),
    ('''                artifact_binding: contract.artifact_binding.map(|binding| semio_framework::InferenceArtifactBinding { field: binding.field, encoding: binding.encoding, required: binding.required }),
            }''', '''                artifact_binding: contract.artifact_binding.map(|binding| semio_framework::InferenceArtifactBinding { field: binding.field, encoding: binding.encoding, required: binding.required }),
                commit: contract.commit.map(|commit| semio_framework::InferenceCommitBinding { action: commit.action }),
            }'''),
    ('''    ArtifactInferenceDocumentBinding,
    ArtifactInferenceExecution,''', '''    ArtifactInferenceCommitBinding,
    ArtifactInferenceDocumentBinding,
    ArtifactInferenceExecution,'''),
    ('''    WireInferenceArtifactBinding,
    WireInferencePayloadContract,''', '''    WireInferenceArtifactBinding,
    WireInferenceCommitBinding,
    WireInferencePayloadContract,'''),
])

edit("🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs", [
    ('''export type InferencePayloadContract = { payloadSchemaId: string, inputSchema: string, outputSchema: string, progressUnit?: string, artifactBinding?: InferenceArtifactBinding, };"####,''',
     '''export type InferencePayloadContract = { payloadSchemaId: string, inputSchema: string, outputSchema: string, progressUnit?: string, artifactBinding?: InferenceArtifactBinding, commit?: InferenceCommitBinding, };"####,
        },
        SchemaMetadata {
            name: "InferenceCommitBinding",
            version: 1,
            typescript: r####"/**
 * 📌️ How one inference's result becomes a document edit: the artifact action that commits it.
 */
export type InferenceCommitBinding = { action: string, };"####,'''),
])

WFC = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/{}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"
for kind in ["◻️2d", "🔲️grid2d", "🧊️3d", "🧱️grid3d"]:
    path = ROOT / WFC.format(kind)
    text = path.read_text()
    updated = re.sub(r"(artifact_binding: Some\(semio_framework_plugin::ArtifactInferenceDocumentBinding \{[^}]*\}\),\n)(\};)", r"\1    commit: None,\n\2", text, count=1)
    assert updated != text, kind
    pass
    print("would patch", kind)
edit(WFC.format("🖼️bitmap"), [
    ('''    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
};''', '''    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: Some(semio_framework_plugin::ArtifactInferenceCommitBinding { action: BITMAP_INFERENCE_COMMIT_ACTION }),
};'''),
])

MCP = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
edit(MCP + "💡️inference/💼️jobs/🦀️.rs", [
    ('''    declared.payload.as_ref().and(None)''', '''    declared.payload.as_ref().and_then(|contract| contract.commit.as_ref()).map(|commit| commit.action.clone())'''),
])
edit(MCP + "💡️inference/🧪️tests/🔬️quick/🦀️.rs", [
    ('''            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".to_string(), encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64.to_string(), required }),''',
     '''            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".to_string(), encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64.to_string(), required }),
            commit: None,'''),
])
edit(MCP + "🏠️workspace/🧪️tests/🔬️quick/🦀️.rs", [
    ('''            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }),''',
     '''            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }),
            commit: None,'''),
])
