# Native882 Local Suggestions

[
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs",
    "line": 1052,
    "original": [
      "    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as i64).unwrap_or(0)"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs",
    "line": 1052,
    "original": [
      "    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as i64).unwrap_or(0)"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs",
    "line": 1052,
    "original": [
      "    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as i64).unwrap_or(0)"
    ],
    "replacement": "0, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::needless_as_bytes",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs",
    "line": 120,
    "original": [
      "    if wire.schema != DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 || wire.uri.is_empty() || wire.uri.as_bytes().len() > DOCUMENT_BACKBONE_BINDING_URI_MAXIMUM_BYTES {"
    ],
    "replacement": "wire.uri.len()",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs",
    "line": 40,
    "original": [
      "        let retained_bytes = protocol::OpBinary::encode_op(&request.mutation).map(|bytes| bytes.len()).unwrap_or(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs",
    "line": 40,
    "original": [
      "        let retained_bytes = protocol::OpBinary::encode_op(&request.mutation).map(|bytes| bytes.len()).unwrap_or(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs",
    "line": 40,
    "original": [
      "        let retained_bytes = protocol::OpBinary::encode_op(&request.mutation).map(|bytes| bytes.len()).unwrap_or(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);"
    ],
    "replacement": "store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::derivable_impls",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs",
    "line": 252,
    "original": [
      "impl Default for WindowTransientOwnerRegistry {",
      "    fn default() -> Self {",
      "        Self { owners: BTreeMap::new() }",
      "    }",
      "}",
      ""
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::derivable_impls",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs",
    "line": 248,
    "original": [
      "pub struct WindowTransientOwnerRegistry {"
    ],
    "replacement": "#[derive(Default)]\n",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::derivable_impls",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs",
    "line": 516,
    "original": [
      "impl Default for WindowConfigOwnerRegistry {",
      "    fn default() -> Self {",
      "        Self { owners: BTreeMap::new() }",
      "    }",
      "}",
      ""
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::derivable_impls",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs",
    "line": 512,
    "original": [
      "pub struct WindowConfigOwnerRegistry {"
    ],
    "replacement": "#[derive(Default)]\n",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::question_mark",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 20457,
    "original": [
      "                        match self.window_config_store.begin(mounted.operation.operation, mounted.meta.actor.clone(), authority, mutation) {",
      "                            Ok(publication) => {",
      "                                mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowConfig(publication));",
      "                                return Ok(());",
      "                            }",
      "                            Err(error) => return Err(error),",
      "                        }"
    ],
    "replacement": "{\n                            let publication = self.window_config_store.begin(mounted.operation.operation, mounted.meta.actor.clone(), authority, mutation)?;\n                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowConfig(publication));\n                            return Ok(());\n                        }",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21052,
    "original": [
      "        fn close_retained_fields_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {"
    ],
    "replacement": "component::app::PluginCloseStep",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21054,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21061,
    "original": [
      "                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21064,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21070,
    "original": [
      "                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21073,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21080,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21083,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21087,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21091,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21094,
    "original": [
      "                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => return Ok(PluginCloseStep::Pending { released_items, released_bytes }),"
    ],
    "replacement": "PluginCloseStep::Pending { released_items, released_bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21095,
    "original": [
      "                store::SnapshotRetirementStep::Blocked => return Ok(PluginCloseStep::Blocked { reason: \"composition graph retirement is externally blocked\" }),"
    ],
    "replacement": "PluginCloseStep::Blocked { reason: \"composition graph retirement is externally blocked\" }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21102,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21105,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21110,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21114,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21119,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21124,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21131,
    "original": [
      "                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21134,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21138,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21143,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21147,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21154,
    "original": [
      "                            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21157,
    "original": [
      "                        return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: released });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: released }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21162,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21166,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21170,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21174,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21178,
    "original": [
      "                return Ok(registry_step);"
    ],
    "replacement": "registry_step",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21181,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21185,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21189,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21193,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21198,
    "original": [
      "                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21201,
    "original": [
      "                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });"
    ],
    "replacement": "PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 21203,
    "original": [
      "            Ok(PluginCloseStep::Complete)"
    ],
    "replacement": "PluginCloseStep::Complete",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::needless_pass_by_value",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 28031,
    "original": [
      "    fn document_backbone_receipt_effect(receipt: crate::document_backbone_binding::DocumentBackboneBindingReceiptV1) -> Effect {"
    ],
    "replacement": "&",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::large_enum_variant",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 28666,
    "original": [
      "        ClosingDecoded { owner: protocol::DecodedAppCommandOwner, fault: Fault },"
    ],
    "replacement": "Box<protocol::DecodedAppCommandOwner>",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::large_enum_variant",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 28670,
    "original": [
      "        Pending(PluginCommandIngress),"
    ],
    "replacement": "Box<PluginCommandIngress>",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::semicolon_if_nothing_returned",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "line": 29742,
    "original": [
      "                push_app_fault(&mut frames, Some(outcome.seq), fault).await"
    ],
    "replacement": "push_app_fault(&mut frames, Some(outcome.seq), fault).await;",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::manual_saturating_arithmetic",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 1142,
    "original": [
      "        let bytes = id.len().checked_add(size_of::<Instance3d>()).unwrap_or(usize::MAX);"
    ],
    "replacement": "id.len().saturating_add(size_of::<Instance3d>())",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::manual_saturating_arithmetic",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 1143,
    "original": [
      "        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).checked_add(bytes).unwrap_or(usize::MAX);"
    ],
    "replacement": "usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).saturating_add(bytes)",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 1270,
    "original": [
      "    WORLD_OPAQUE_QUARANTINE.lock().map(|quarantine| (usize::from(quarantine.len), quarantine.saturated)).unwrap_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX))"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 1270,
    "original": [
      "    WORLD_OPAQUE_QUARANTINE.lock().map(|quarantine| (usize::from(quarantine.len), quarantine.saturated)).unwrap_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX))"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 1270,
    "original": [
      "    WORLD_OPAQUE_QUARANTINE.lock().map(|quarantine| (usize::from(quarantine.len), quarantine.saturated)).unwrap_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX))"
    ],
    "replacement": "(WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX), ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::large_enum_variant",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 2478,
    "original": [
      "    MarqueePublish { job: WorldMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },"
    ],
    "replacement": "Box<WorldMarqueePublishJob>",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::needless_range_loop",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 2752,
    "original": [
      "        for index in 0..count {"
    ],
    "replacement": "(index, <item>)",
    "applicability": "HasPlaceholders"
  },
  {
    "code": "clippy::needless_range_loop",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 2752,
    "original": [
      "        for index in 0..count {"
    ],
    "replacement": "points.iter().enumerate().take(count)",
    "applicability": "HasPlaceholders"
  },
  {
    "code": "clippy::needless_range_loop",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 3130,
    "original": [
      "            for page in 0..usize::from(self.results.page_len) {"
    ],
    "replacement": "(page, <item>)",
    "applicability": "HasPlaceholders"
  },
  {
    "code": "clippy::needless_range_loop",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 3130,
    "original": [
      "            for page in 0..usize::from(self.results.page_len) {"
    ],
    "replacement": "credits.iter_mut().enumerate().take(usize::from(self.results.page_len))",
    "applicability": "HasPlaceholders"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4148,
    "original": [
      "            numbers: [object.map(|(_, id)| id as f64).unwrap_or(0.0), if object.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4148,
    "original": [
      "            numbers: [object.map(|(_, id)| id as f64).unwrap_or(0.0), if object.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4148,
    "original": [
      "            numbers: [object.map(|(_, id)| id as f64).unwrap_or(0.0), if object.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],"
    ],
    "replacement": "0.0, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4388,
    "original": [
      "            let pivot = state.gumball_target.map(|target| Vec3::new(target[0], target[1], target[2])).unwrap_or_else(|| self.sum.scale(1.0 / f32::from(self.selected_len)));"
    ],
    "replacement": "state.gumball_target.map_or_else(|| self.sum.scale(1.0 / f32::from(self.selected_len)), |target| Vec3::new(target[0], target[1], target[2]))",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4486,
    "original": [
      "        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map(|point| if handle.is_rotate() { point.sub(pivot) } else { point }).unwrap_or(pivot);"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4486,
    "original": [
      "        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map(|point| if handle.is_rotate() { point.sub(pivot) } else { point }).unwrap_or(pivot);"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 4486,
    "original": [
      "        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map(|point| if handle.is_rotate() { point.sub(pivot) } else { point }).unwrap_or(pivot);"
    ],
    "replacement": "pivot, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::needless_pass_by_value",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 6296,
    "original": [
      "    fn release(&mut self, key: K) -> bool {"
    ],
    "replacement": "&",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_lazy_evaluations",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 8536,
    "original": [
      "    authority.gumball.as_ref().or_else(|| match authority.active.as_ref() {",
      "        Some(WorldInteractionActive::GumballCommit { job, .. }) => Some(&job.gesture),",
      "        _ => None,",
      "    })"
    ],
    "replacement": "or(match authority.active.as_ref() {\n        Some(WorldInteractionActive::GumballCommit { job, .. }) => Some(&job.gesture),\n        _ => None,\n    })",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 8559,
    "original": [
      "        gesture.handle.axis_dir().map(|axis| gesture.pivot.add(rotate_vector(base.sub(gesture.pivot), axis, gesture.angle))).unwrap_or(base)"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 8559,
    "original": [
      "        gesture.handle.axis_dir().map(|axis| gesture.pivot.add(rotate_vector(base.sub(gesture.pivot), axis, gesture.angle))).unwrap_or(base)"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 8559,
    "original": [
      "        gesture.handle.axis_dir().map(|axis| gesture.pivot.add(rotate_vector(base.sub(gesture.pivot), axis, gesture.angle))).unwrap_or(base)"
    ],
    "replacement": "base, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10408,
    "original": [
      "    let dir = direction.map(|value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32)).unwrap_or(Vec3::new(0.0, 0.0, -1.0));"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10408,
    "original": [
      "    let dir = direction.map(|value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32)).unwrap_or(Vec3::new(0.0, 0.0, -1.0));"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10408,
    "original": [
      "    let dir = direction.map(|value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32)).unwrap_or(Vec3::new(0.0, 0.0, -1.0));"
    ],
    "replacement": "Vec3::new(0.0, 0.0, -1.0), ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10590,
    "original": [
      "    state.reference_pixels.get(url).map(|(width, height, _)| *width as f32 / (*height).max(1) as f32).unwrap_or(1.0).max(0.01)"
    ],
    "replacement": "map_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10590,
    "original": [
      "    state.reference_pixels.get(url).map(|(width, height, _)| *width as f32 / (*height).max(1) as f32).unwrap_or(1.0).max(0.01)"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10590,
    "original": [
      "    state.reference_pixels.get(url).map(|(width, height, _)| *width as f32 / (*height).max(1) as f32).unwrap_or(1.0).max(0.01)"
    ],
    "replacement": "1.0, ",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::map_unwrap_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10596,
    "original": [
      "    mesh_url.map(mesh_id_from_url).unwrap_or_else(|| \"box\".to_string())"
    ],
    "replacement": "mesh_url.map_or_else(|| \"box\".to_string(), mesh_id_from_url)",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::manual_saturating_arithmetic",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
    "line": 10742,
    "original": [
      "        let next = self.received_bytes.checked_add(page.bytes.len()).unwrap_or(usize::MAX);"
    ],
    "replacement": "self.received_bytes.saturating_add(page.bytes.len())",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::needless_pass_by_value",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs",
    "line": 62,
    "original": [
      "    fn path_element_to_kurbo(value: PathEl) -> kurbo::PathEl {"
    ],
    "replacement": "&",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_map_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs",
    "line": 246,
    "original": [
      "    block.condition.as_ref().map_or(true, |expr| eval_playbook_expr(expr, values).as_bool().unwrap_or(false))"
    ],
    "replacement": "is_none_or",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::unnecessary_map_or",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs",
    "line": 246,
    "original": [
      "    block.condition.as_ref().map_or(true, |expr| eval_playbook_expr(expr, values).as_bool().unwrap_or(false))"
    ],
    "replacement": "",
    "applicability": "MachineApplicable"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs",
    "line": 22,
    "original": [
      "    pub fn from_value(value: dsl::DslValue) -> Result<super::JsonValue, dsl::ValueError> {"
    ],
    "replacement": "serde_json::Value",
    "applicability": "MaybeIncorrect"
  },
  {
    "code": "clippy::unnecessary_wraps",
    "file": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs",
    "line": 23,
    "original": [
      "        Ok(super::JsonValue::from(value))"
    ],
    "replacement": "super::JsonValue::from(value)",
    "applicability": "MaybeIncorrect"
  }
]
