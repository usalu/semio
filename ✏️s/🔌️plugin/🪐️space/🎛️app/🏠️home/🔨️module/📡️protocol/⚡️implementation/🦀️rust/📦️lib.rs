//! ⚖️ S Home launcher app — binary command protocol surface + laws (constitutional: protocol). Also
//! hosts `HomeCommand` — the app-engine `DocumentApp::Command` binary command envelope, one variant
//! per `create_home_app`'s declared action (B1: the space/home cutover).

use home_op::SHomeOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes an `SHomeOperation` to its binary command form.
pub fn encode_op(operation: &SHomeOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SHomeOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SHomeOperation, protocol::ProtocolError> {
    SHomeOperation::decode_op(bytes)
}

//#region 🔖️HomeCommand
/// 🎯️ B1: `HomeApp::Command` — the SOLE dispatch surface for the Home launcher's own behavior, one
/// variant per action declared in `create_home_app`'s manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum HomeCommand {
    #[dsl(key = "create-studio")]
    CreateStudio { name: String, kind: String, folder_path: Option<String> },
    #[dsl(key = "bind-space-file")]
    BindSpaceFile { space_id: String, file_path: String },
    #[dsl(key = "import-space")]
    ImportSpace { dsl: Option<String> },
    #[dsl(key = "open-space")]
    OpenSpace { space_id: String },
    #[dsl(key = "navigate-vfs-node")]
    NavigateVirtualFileSystemNode { node_id: String },
    #[dsl(key = "delete-vfs-node")]
    DeleteVirtualFileSystemNode { node_id: String },
    #[dsl(key = "go-home")]
    GoHome,
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
}
//#endregion 🔖️HomeCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use home::SHomeDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SHomeOperation::SetCatalogGeneration { value: 7 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn home_document_text_round_trips_through_the_store() {
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let envelope = store::create_document_envelope::<SHomeDocument, SHomeOperation>("s.home", "home", projection, None);
        let mut store: store::DocumentStore<SHomeDocument, SHomeOperation> = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![SHomeOperation::SetCatalogGeneration { value: 3 }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio { name: "Untitled".into(), kind: "catalog".into(), folder_path: None });
        store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio { name: "Untitled".into(), kind: "folder".into(), folder_path: Some("/tmp/x".into()) });
        store::test_support::assert_op_line_round_trip(&HomeCommand::BindSpaceFile { space_id: "s1".into(), file_path: "/tmp/x.os".into() });
        store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace { dsl: Some("programs=[]".into()) });
        store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace { dsl: None });
        store::test_support::assert_op_line_round_trip(&HomeCommand::OpenSpace { space_id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&HomeCommand::NavigateVirtualFileSystemNode { node_id: "studio:s1".into() });
        store::test_support::assert_op_line_round_trip(&HomeCommand::DeleteVirtualFileSystemNode { node_id: "studio:s1".into() });
        store::test_support::assert_op_line_round_trip(&HomeCommand::GoHome);
        store::test_support::assert_op_line_round_trip(&HomeCommand::SetActivePanelTab { tab_id: "tab-1".into() });
    }
}
//#endregion 🧪️Tests
