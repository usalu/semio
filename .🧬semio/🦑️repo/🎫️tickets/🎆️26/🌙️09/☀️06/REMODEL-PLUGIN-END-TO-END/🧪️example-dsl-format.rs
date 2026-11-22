use pack::snapshot::ArtifactDsl;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{default_remodeling_scene, RemodelingSnapshot};

fn main() {
    let canonical = default_remodeling_scene().print_dsl();
    let handle = canonical.lines().find(|line| line.contains("child_id=remodeling-mesh-constant-box")).expect("canonical mesh handle");
    println!("{}", handle.trim());
    for path in std::env::args_os().skip(1) {
        let document = std::fs::read_to_string(&path).expect("example text");
        let original = document.lines().find(|line| line.contains("child_id=remodeling-mesh-constant-box")).expect("example mesh handle");
        assert_eq!(original, handle, "{path:?}");
        let parsed = RemodelingSnapshot::parse_dsl(&document).unwrap_or_else(|failure| panic!("{path:?}: {failure:?}"));
        assert_eq!(parsed.results.mesh.mesh.target.artifact_id, "remodeling-mesh-constant:box");
        assert_eq!(parsed.results.mesh.mesh.target.dialect.artifact_kind, "s.stdio.semio");
        let printed = parsed.print_dsl();
        assert!(printed.lines().any(|line| line == handle));
        RemodelingSnapshot::parse_dsl(&printed).expect("native example round trip");
    }
}
