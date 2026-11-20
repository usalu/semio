use pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

fn main() {
    let pack = include_bytes!("🗑️generated/din4108-observed.pack");
    let spr = include_bytes!("🗑️generated/din4108-observed.spr");
    let model = Din4108Snapshot::decode_pack(pack).expect("DIN 4108 genesis");
    let text = model.print_dsl();
    let parsed = Din4108Snapshot::parse_dsl(&text).expect("canonical DIN 4108 text");
    assert_eq!(model, parsed);
    let history = semio_framework_async::poll::resolve_ready(semio_os_spr::decode_history(spr, &Default::default())).expect("SPR");
    assert_eq!(history.schema, "semio.norm.din4108/v1");
    let composition = history.composition.as_ref().expect("composition");
    assert!(composition.owner.is_none());
    assert_eq!(composition.dialect, Some(("s.norm.din4108".into(), "1".into(), "*".into())));
    assert_eq!((history.edits.len(), history.transitions.len(), history.conflicts.len()), (0, 0, 0));
    println!("DIN 4108 genesis decoded and round-tripped ({} DSL bytes), history {} for {} has no edits or transitions.", text.len(), history.schema, history.doc_id);
}
