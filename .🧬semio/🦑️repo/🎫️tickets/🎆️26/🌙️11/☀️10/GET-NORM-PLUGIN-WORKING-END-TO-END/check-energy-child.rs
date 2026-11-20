use pack::snapshot::ArtifactDsl;
use semio_plugin::{ArtifactEditor, ArtifactViewer};
use semio_s_artifact_norm_din18599_editor::Din18599PlayApp;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_viewer::Din18599Viewer;

fn main() {
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).unwrap();
        let parent = Din18599Snapshot::parse_dsl(&text).unwrap();
        let editing = Din18599PlayApp::child_restore_projection(&parent).unwrap();
        let viewing = Din18599Viewer::child_restore_projection(&parent).unwrap();
        let (slot, child) = editing.get(0).expect("editor climate table");
        let (other_slot, other_child) = viewing.get(0).expect("viewer climate table");
        assert_eq!((slot,child.artifact_id),(other_slot,other_child.artifact_id));
        assert_eq!((slot, child.artifact_kind, child.standard, child.subset),("climateTable","s.stdio.semio","v1","table"));
        assert!(editing.get(1).is_none() && viewing.get(1).is_none());
        let bytes = Din18599PlayApp::genesis_child_pack(&parent, slot, child.artifact_id).unwrap().unwrap();
        assert_eq!(Some(bytes.clone()), Din18599Viewer::genesis_child_pack(&parent, slot, child.artifact_id).unwrap());
        assert!(bytes.starts_with(b"\x89SEM"));
        assert!(Din18599Viewer::genesis_child_pack(&parent, slot, "other").unwrap().is_none());
        println!("PASS {path} child={} bytes={}",child.artifact_id,bytes.len());
    }
}
