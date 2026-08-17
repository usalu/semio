
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/📸️remodel"
ART = next((PLUGIN / "🗿️artifacts").iterdir())
MUT = ART / "🧬️mutations"

MUTATIONS = [
    ("🎞️", "set-streams", "SetStreams"),
    ("🖼️", "set-asset", "SetAsset"),
    ("📐️", "set-calibration", "SetCalibration"),
    ("📍️", "set-gcps", "SetGcps"),
    ("📥️", "set-ingest-params", "SetIngestParams"),
    ("🌟️", "set-feature-params", "SetFeatureParams"),
    ("🔗️", "set-match-params", "SetMatchParams"),
    ("🧭️", "set-sfm-params", "SetSfmParams"),
    ("🌫️", "set-dense-params", "SetDenseParams"),
    ("🎛", "set-mesh-params", "SetMeshParams"),
    ("🏃️", "set-motion-params", "SetMotionParams"),
    ("🗺️", "set-geo-params", "SetGeoParams"),
    ("🏭️", "set-job", "SetJob"),
    ("✨️", "set-sparse", "SetSparse"),
    ("🌧️", "set-dense", "SetDense"),
    ("📦️", "set-mesh-result", "SetMeshResult"),
    ("🛤️", "set-trajectory", "SetTrajectory"),
    ("📈️", "set-tracks", "SetTracks"),
    ("🌍️", "set-geo-products", "SetGeoProducts"),
    ("✅️", "set-qc", "SetQc"),
]

def write(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    if not content.endswith("\n"):
        content += "\n"
    path.write_text(content)
    print("W", path)

def ts_stub(label: str) -> str:
    return f"/** 🧩 {label} facade stub. */\nexport {{}};\n"

APPLY = {
    "SetStreams": ("streams: &Vec<crate::artifacts::remodel::MediaStream>", "streams", "next.streams = streams.clone();"),
    "SetAsset": ("key: &str, value: &Option<crate::artifacts::remodel::ImageAsset>", "key, value",
        "match value { Some(value) => { next.assets.insert(key.to_string(), value.clone()); } None => { next.assets.remove(key); } }"),
    "SetCalibration": ("calibration: &crate::artifacts::remodel::CalibrationState", "calibration", "next.calibration = calibration.clone();"),
    "SetGcps": ("gcps: &Vec<crate::artifacts::remodel::GroundControlPoint>", "gcps", "next.gcps = gcps.clone();"),
    "SetIngestParams": ("params: &crate::artifacts::remodel::IngestParams", "params", "next.params.ingest = params.clone();"),
    "SetFeatureParams": ("params: &crate::artifacts::remodel::FeatureParams", "params", "next.params.feature = params.clone();"),
    "SetMatchParams": ("params: &crate::artifacts::remodel::MatchParams", "params", "next.params.matching = params.clone();"),
    "SetSfmParams": ("params: &crate::artifacts::remodel::SfmParams", "params", "next.params.sfm = params.clone();"),
    "SetDenseParams": ("params: &crate::artifacts::remodel::DenseParams", "params", "next.params.dense = params.clone();"),
    "SetMeshParams": ("params: &crate::artifacts::remodel::MeshParams", "params", "next.params.mesh = params.clone();"),
    "SetMotionParams": ("params: &crate::artifacts::remodel::MotionParams", "params", "next.params.motion = params.clone();"),
    "SetGeoParams": ("params: &crate::artifacts::remodel::GeoParams", "params", "next.params.geo = params.clone();"),
    "SetJob": ("job: &crate::artifacts::remodel::ReconstructionJob", "job", "next.job = job.clone();"),
    "SetSparse": ("sparse: &Option<crate::artifacts::remodel::SparseCloud>", "sparse", "next.results.sparse = sparse.clone();"),
    "SetDense": ("dense: &Option<crate::artifacts::remodel::DenseCloud>", "dense", "next.results.dense = dense.clone();"),
    "SetMeshResult": ("mesh: &Box<crate::artifacts::remodel::RemodelMesh>", "mesh", "next.results.mesh = mesh.as_ref().clone();"),
    "SetTrajectory": ("trajectory: &Option<crate::artifacts::remodel::CameraTrajectory>", "trajectory", "next.results.trajectory = trajectory.clone();"),
    "SetTracks": ("tracks: &Vec<crate::artifacts::remodel::MotionTrackSummary>", "tracks", "next.results.tracks = tracks.clone();"),
    "SetGeoProducts": ("geo: &Option<crate::artifacts::remodel::GeoProducts>", "geo", "next.results.geo = geo.clone();"),
    "SetQc": ("qc: &Option<crate::artifacts::remodel::QcReportSnapshot>", "qc", "next.results.qc = qc.clone();"),
}

INVERSE = {
    "SetStreams": ("", "", "vec![RemodelMutation::SetStreams { streams: base.streams.clone() }]"),
    "SetAsset": ("key: &str", "key", "vec![RemodelMutation::SetAsset { key: key.to_string(), value: base.assets.get(key).cloned() }]"),
    "SetCalibration": ("", "", "vec![RemodelMutation::SetCalibration { calibration: base.calibration.clone() }]"),
    "SetGcps": ("", "", "vec![RemodelMutation::SetGcps { gcps: base.gcps.clone() }]"),
    "SetIngestParams": ("", "", "vec![RemodelMutation::SetIngestParams { params: base.params.ingest.clone() }]"),
    "SetFeatureParams": ("", "", "vec![RemodelMutation::SetFeatureParams { params: base.params.feature.clone() }]"),
    "SetMatchParams": ("", "", "vec![RemodelMutation::SetMatchParams { params: base.params.matching.clone() }]"),
    "SetSfmParams": ("", "", "vec![RemodelMutation::SetSfmParams { params: base.params.sfm.clone() }]"),
    "SetDenseParams": ("", "", "vec![RemodelMutation::SetDenseParams { params: base.params.dense.clone() }]"),
    "SetMeshParams": ("", "", "vec![RemodelMutation::SetMeshParams { params: base.params.mesh.clone() }]"),
    "SetMotionParams": ("", "", "vec![RemodelMutation::SetMotionParams { params: base.params.motion.clone() }]"),
    "SetGeoParams": ("", "", "vec![RemodelMutation::SetGeoParams { params: base.params.geo.clone() }]"),
    "SetJob": ("", "", "vec![RemodelMutation::SetJob { job: base.job.clone() }]"),
    "SetSparse": ("", "", "vec![RemodelMutation::SetSparse { sparse: base.results.sparse.clone() }]"),
    "SetDense": ("", "", "vec![RemodelMutation::SetDense { dense: base.results.dense.clone() }]"),
    "SetMeshResult": ("", "", "vec![RemodelMutation::SetMeshResult { mesh: Box::new(base.results.mesh.clone()) }]"),
    "SetTrajectory": ("", "", "vec![RemodelMutation::SetTrajectory { trajectory: base.results.trajectory.clone() }]"),
    "SetTracks": ("", "", "vec![RemodelMutation::SetTracks { tracks: base.results.tracks.clone() }]"),
    "SetGeoProducts": ("", "", "vec![RemodelMutation::SetGeoProducts { geo: base.results.geo.clone() }]"),
    "SetQc": ("", "", "vec![RemodelMutation::SetQc { qc: base.results.qc.clone() }]"),
}

# Rewrite all leaves
for emoji, kebab, variant in MUTATIONS:
    base = MUT / f"{emoji}{kebab}"
    sig, call, body = APPLY[variant]
    write(base / "🦠️mutation" / "🦀️component.rs", f"""//! {emoji} Remodel mutation — `{variant}` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, {sig}) {{
    {body}
}}
//#endregion 🔖️Mutation
""")
    write(base / "🦠️mutation" / "🟦️component.ts", ts_stub(f"remodel mutations {emoji}{kebab}/🦠️mutation"))
    isig, icall, ibody = INVERSE[variant]
    inv_args = "base: &RemodelProjection" + (f", {isig}" if isig else "")
    write(base / "↩️inverse" / "🦀️component.rs", f"""//! ↩️ Inverse for `{variant}`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse({inv_args}) -> Vec<RemodelMutation> {{
    {ibody}
}}
//#endregion 🔖️Inverse
""")
    write(base / "↩️inverse" / "🟦️component.ts", ts_stub(f"remodel mutations {emoji}{kebab}/↩️inverse"))
    write(base / "🔺️diff" / "🦀️component.rs", f"""//! 🔺️ Diff fragment yielded by `{variant}`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;
use protocol::MutationDiff;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `{variant}` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct {variant}Diff {{
    pub mutation: Option<RemodelMutation>,
}}

impl {variant}Diff {{
    pub fn from_mutation(mutation: RemodelMutation) -> Self {{
        Self {{ mutation: Some(mutation) }}
    }}
}}

impl MutationDiff<RemodelProjection> for {variant}Diff {{
    fn apply(&self, projection: &RemodelProjection) -> RemodelProjection {{
        match &self.mutation {{
            Some(m) => {{
                let mut next = projection.clone();
                crate::artifacts::remodel::mutations::apply_remodel_mutation_in_place(&mut next, m);
                next
            }}
            None => projection.clone(),
        }}
    }}

    fn absorb(&mut self, other: Self) {{
        if other.mutation.is_some() {{
            *self = other;
        }}
    }}
}}
//#endregion 🔖️Diff
""")
    write(base / "🔺️diff" / "🟦️component.ts", ts_stub(f"remodel mutations {emoji}{kebab}/🔺️diff"))

# Build root from original op
op_path = ART / "🔧️op" / "🦀️component.rs"
op_text = op_path.read_text()
# If already slimmed, read from git? Use backup from leaves - re-read if still has enum
if "pub enum RemodelOperation" not in op_text and "pub enum RemodelMutation" not in op_text:
    raise SystemExit("op file already slimmed without enum; cannot extract")

enum_src = op_text
if "pub enum RemodelMutation" in enum_src:
    em = re.search(r"pub enum RemodelMutation \{(?P<body>.*?)\n\}", enum_src, re.S)
else:
    em = re.search(r"pub enum RemodelOperation \{(?P<body>.*?)\n\}", enum_src, re.S)
enum_body = em.group("body").replace("RemodelOperation", "RemodelMutation")

# Extract tests from original if present
tests = ""
tm = re.search(r"(//#region 🧪️Tests.*)", op_text, re.S)
if tm:
    tests = tm.group(1)
    tests = tests.replace("RemodelOperation", "RemodelMutation")
    tests = tests.replace("apply_remodel_operation", "apply_remodel_mutation")
    tests = tests.replace(".backwards(", ".inverse(")
    # vcs::apply_operation -> apply_mutation
    tests = tests.replace("apply_operation", "apply_mutation")

apply_arms = []
inv_arms = []
diff_arms = []
for emoji, kebab, variant in MUTATIONS:
    mod = kebab.replace("-", "_")
    _, call, _ = APPLY[variant]
    apply_arms.append(f"        RemodelMutation::{variant} {{ {call} }} => super::{mod}::mutation::apply(next, {call}),")
    isig, icall, _ = INVERSE[variant]
    if variant == "SetAsset":
        inv_arms.append(f"        RemodelMutation::SetAsset {{ key, .. }} => super::{mod}::inverse::inverse(base, key),")
    else:
        inv_arms.append(f"        RemodelMutation::{variant} {{ .. }} => super::{mod}::inverse::inverse(base),")
    # diff mapping uses same field names as APPLY call
    # Build RemodelDiff::{variant} { fields: fields.clone() }
    fields = call
    clones = ", ".join(f"{f.strip()}: {f.strip()}.clone()" for f in fields.split(","))
    diff_arms.append(f"            RemodelMutation::{variant} {{ {fields} }} => RemodelDiff::{variant} {{ {clones} }},")

root = f"""//! 🧬️ Remodel artifact — document mutation dispatch enum.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{{
    CalibrationState, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary, QcReportSnapshot,
    ReconstructionJob, RemodelMesh, RemodelProjection, SfmParams, SparseCloud,
}};
use protocol::Mutation;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Mutations
/// @emoji 🧬️ The typed remodel document mutation — one LWW register setter per independent field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum RemodelMutation {{{enum_body}
}}

/// @emoji ▶️ Applies one mutation, returning the next projection.
pub fn apply_remodel_mutation(scene: &RemodelProjection, mutation: &RemodelMutation) -> RemodelProjection {{
    let mut next = scene.clone();
    apply_remodel_mutation_in_place(&mut next, mutation);
    next
}}

/// @emoji ▶️ Applies one mutation to the projection in place.
pub fn apply_remodel_mutation_in_place(next: &mut RemodelProjection, mutation: &RemodelMutation) {{
    match mutation {{
{chr(10).join(apply_arms)}
    }}
}}

/// @emoji ↩️ Computes the inverse mutations from pre-state.
pub fn inverse_remodel_mutation(base: &RemodelProjection, mutation: &RemodelMutation) -> Vec<RemodelMutation> {{
    match mutation {{
{chr(10).join(inv_arms)}
    }}
}}

impl Mutation<RemodelProjection> for RemodelMutation {{
    type Diff = RemodelDiff;

    fn diff(&self, _projection: &RemodelProjection) -> RemodelDiff {{
        match self {{
{chr(10).join(diff_arms)}
        }}
    }}

    fn inverse(&self, projection: &RemodelProjection) -> Vec<Self> {{
        inverse_remodel_mutation(projection, self)
    }}
}}
//#endregion 🔖️Mutations

{tests}
"""
write(MUT / "🦀️component.rs", root)
write(MUT / "🟦️component.ts", ts_stub("remodel 🧬️mutations WASM"))

# Slim op
write(ART / "🔧️op" / "🦀️component.rs", """//! ⚡️ Remodel artifact — OpText/OpBinary codecs + grammar for serializing `RemodelMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::artifacts::remodel::mutations::{
    apply_remodel_mutation, apply_remodel_mutation_in_place, inverse_remodel_mutation, RemodelMutation,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for RemodelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RemodelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
""")

# Grammar
gram_path = ART / "🔧️op" / "📖️component.grammar.semio"
gram = gram_path.read_text()
gram = gram.replace("start operation", "start mutation")
gram = re.sub(r"(?m)^operation\s*=", "mutation =", gram)
write(gram_path, gram)

# Protocols
for prot in list((ART / "📡️spr").glob("*.semio")) + list((ART / "🎒️pack").glob("*.semio")):
    t = prot.read_text()
    t2 = re.sub(r"(schema\s+\S+)\.operation\b", r"\1.mutation", t)
    if t2 != t:
        write(prot, t2)

# Diff facet
diff_path = ART / "🔺️diff" / "🦀️component.rs"
diff = diff_path.read_text()
diff = diff.replace("OperationDiff", "MutationDiff")
diff = diff.replace("Operation::Diff", "Mutation::Diff")
diff = diff.replace("operation diff", "mutation diff")
diff = diff.replace("RemodelOperation", "RemodelMutation")
diff = diff.replace("apply_remodel_operation", "apply_remodel_mutation")
diff = diff.replace("crate::artifacts::remodel::op::", "crate::artifacts::remodel::mutations::")
diff = diff.replace("use crate::artifacts::remodel::op::{", "use crate::artifacts::remodel::mutations::{")
# fix import if still using op path with apply
diff = re.sub(r"use crate::artifacts::remodel::op::\{([^}]+)\}", r"use crate::artifacts::remodel::mutations::{\1}", diff)
write(diff_path, diff)

# Engine ArtifactEngine
engine_path = ART / "⚙️engine" / "🦀️component.rs"
engine = engine_path.read_text()
if "ArtifactEngine" not in engine:
    engine += """

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent remodel artifact engine — owns the projection; every transition is a mutation.
pub struct RemodelEngine {
    projection: crate::artifacts::remodel::RemodelProjection,
}

impl RemodelEngine {
    pub fn new(projection: crate::artifacts::remodel::RemodelProjection) -> Self {
        Self { projection }
    }

    pub fn into_projection(self) -> crate::artifacts::remodel::RemodelProjection {
        self.projection
    }
}

impl protocol::ArtifactEngine for RemodelEngine {
    type Projection = crate::artifacts::remodel::RemodelProjection;
    type Mutation = crate::artifacts::remodel::mutations::RemodelMutation;
    type Diff = crate::artifacts::remodel::diff::RemodelDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::remodel::mutations::apply_remodel_mutation_in_place(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
"""
    write(engine_path, engine)

# Glue
glue_path = PLUGIN / "📦️packages" / "🦀️rust" / "📦️glue.rs"
glue = glue_path.read_text()
if "pub mod mutations" not in glue:
    block = ['\n        #[path = "."]\n        pub mod mutations {\n',
             '            #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🦀️component.rs"]\n',
             '            mod component;\n',
             '            pub use component::*;\n']
    for emoji, kebab, _variant in MUTATIONS:
        mod = kebab.replace("-", "_")
        dirname = f"{emoji}{kebab}"
        block.append(f"""
            #[path = "."]
            pub mod {mod} {{
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/{dirname}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/{dirname}/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/{dirname}/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }}
""")
    block.append("        }\n")
    block_s = "".join(block)
    if 'pub mod op;' in glue:
        glue = glue.replace("pub mod op;", "pub mod op;" + block_s, 1)
    else:
        glue = re.sub(r'(pub mod op;)', r'\1' + block_s, glue, count=1)
    write(glue_path, glue)

# TS index
idx_path = PLUGIN / "📦️packages" / "🟦️typescript" / "📦️index.ts"
idx = idx_path.read_text()
if "remodel_mutations" not in idx:
    line = 'export * as remodel_mutations from "../../🗿️artifacts/📸️remodel/🧬️mutations/🟦️component.ts";\n'
    idx = idx.replace(
        'export * as remodel_op from "../../🗿️artifacts/📸️remodel/🔧️op/🟦️component.ts";\n',
        'export * as remodel_op from "../../🗿️artifacts/📸️remodel/🔧️op/🟦️component.ts";\n' + line,
    )
    write(idx_path, idx)

# Bulk rename remaining Operation* in plugin (except Op brand)
EXTRA = [
    ("RemodelOperation", "RemodelMutation"),
    ("RemodelConfigOperation", "RemodelConfigMutation"),
    ("apply_remodel_operation", "apply_remodel_mutation"),
    ("invert_remodel_operation", "inverse_remodel_mutation"),
    ("DocumentApp::Operation", "DocumentApp::Mutation"),
    ("type Operation =", "type Mutation ="),
    ("type ConfigOperation =", "type ConfigMutation ="),
    ("type DraftOperation =", "type DraftMutation ="),
    ("NoDraftOperation", "NoDraftMutation"),
    ("NoConfigOperation", "NoConfigMutation"),
    ("document_operations", "document_mutations"),
    ("config_operations", "config_mutations"),
    ("draft_operations", "draft_mutations"),
    ("Emit::operations", "Emit::mutations"),
    ("CollectionOperation", "CollectionMutation"),
    ("apply_collection_operation", "apply_collection_mutation"),
    ("invert_collection_operation", "inverse_collection_mutation"),
    ("collection_diff_from_operation", "collection_diff_from_mutation"),
    ("OperationDiff", "MutationDiff"),
    ("apply_operation", "apply_mutation"),
    ('tag = "operation"', 'tag = "mutation"'),
    (".backwards(", ".inverse("),
    ("fn backwards(", "fn inverse("),
    ("impl Operation<", "impl Mutation<"),
    ("use protocol::Operation;", "use protocol::Mutation;"),
    ("use protocol::{Operation", "use protocol::{Mutation"),
    (", Operation,", ", Mutation,"),
    (", Operation}", ", Mutation}"),
]

def rename_text(text: str) -> str:
    # shield Op brand
    shields = {}
    def shield(pat, key):
        nonlocal text
        out = []
        last = 0
        for i, m in enumerate(re.finditer(pat, text)):
            ph = f"__SH_{key}_{i}__"
            shields[ph] = m.group(0)
            out.append(text[last:m.start()])
            out.append(ph)
            last = m.end()
        out.append(text[last:])
        text = "".join(out)
    for pat, key in [
        (r"\bOpText\b", "a"), (r"\bOpBinary\b", "b"), (r"\bprint_op\b", "c"),
        (r"\bparse_op\b", "d"), (r"\bencode_op\b", "e"), (r"\bdecode_op\b", "f"),
        (r"\bLanguageRole::Ops\b", "g"), (r"unknown mutation line", "h"),
    ]:
        shield(pat, key)
    for old, new in EXTRA:
        text = text.replace(old, new)
    text = re.sub(r"(schema\s+\S+)\.operation\b", r"\1.mutation", text)
    text = text.replace("start operation", "start mutation")
    text = re.sub(r"(?m)^operation\s*=", "mutation =", text)
    text = text.replace("unknown operation line", "unknown mutation line")
    for ph, orig in shields.items():
        text = text.replace(ph, orig)
    return text

for p in PLUGIN.rglob("*"):
    if not p.is_file() or p.suffix not in {".rs", ".ts", ".semio"}:
        continue
    try:
        t = p.read_text()
    except Exception:
        continue
    n = rename_text(t)
    if n != t:
        p.write_text(n)
        print("R", p)

print("REMODEL DONE")
