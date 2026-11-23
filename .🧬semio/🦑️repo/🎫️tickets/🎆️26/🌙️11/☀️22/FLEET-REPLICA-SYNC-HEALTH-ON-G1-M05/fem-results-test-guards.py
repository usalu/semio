import hashlib, os, sys

path = sys.argv[1]
with open(path, encoding="utf-8", newline="") as handle:
    before = handle.read()
text = before
dsl = '<Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT).expect("default example snapshot")'
demo = 'semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo snapshot")'
dead = 'Some("dead".into())'
replacements = [
    ("use crate::context::Fem3dApp;\n", "use crate::context::Fem3dApp;\nuse crate::context::Retiring;\n"),
    ("let mut doc = Fem3dSnapshot::default();", "let mut doc = Retiring::new(Fem3dSnapshot::default());"),
    (f"= {dsl};", f"= Retiring::new({dsl});"),
    (f"= {demo};", f"= Retiring::new({demo});"),
    ("fem3d_model_extent(&Fem3dSnapshot::default())", "fem3d_model_extent(&Retiring::new(Fem3dSnapshot::default()))"),
    (dead, 'Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap())'),
]
for old, new in replacements:
    if old not in text:
        sys.exit(f"missing: {old[:80]}")
    text = text.replace(old, new)
with open(path, encoding="utf-8", newline="") as handle:
    if handle.read() != before:
        sys.exit("changed meanwhile")
temporary = path + ".fem-guards.tmp"
with open(temporary, "w", encoding="utf-8", newline="") as handle:
    handle.write(text)
os.replace(temporary, path)
print(hashlib.sha256(text.encode()).hexdigest()[:12], text.count("Retiring::new("), text.count("encode_result_source_parts"))
