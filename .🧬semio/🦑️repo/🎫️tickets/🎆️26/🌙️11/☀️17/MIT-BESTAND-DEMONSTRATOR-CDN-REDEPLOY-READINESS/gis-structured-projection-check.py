from pathlib import Path
import subprocess
import sys


ticket = Path(__file__).resolve().parent
lane = Path(sys.argv[1]).resolve()
scratch = ticket / "🗑️generated" / "gis-projection-check"
dependencies = scratch / "d"
dependencies.mkdir(parents=True, exist_ok=True)
for build, suffix in (("wasm32-wasip2/release/build", "*.rmeta"), ("release/build", "*.dylib")):
    for source in (lane / build).glob("*/*/out/" + suffix):
        link = dependencies / source.name
        if not link.is_symlink():
            link.symlink_to(source)

externs = []
for package in ["semio_framework", "semio_framework_job", "semio_framework_pack_json", "semio_framework_surface_terrain", "semio_framework_value", "semio_plugin"]:
    path, = dependencies.glob(f"lib{package}-*.rmeta")
    externs += ["--extern", f"{package}={path}"]
scope = """
use semio_framework::{MediaError, MediaType};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};
use semio_plugin::{ArtifactApp, ArtifactMediaExportJobRequest};
use semio_plugin::app::ArtifactMediaExportResult;
type Request<S> = ArtifactMediaExportJobRequest<<S as StructuredMediaSource>::App>;
type ResultPage = Result<ArtifactMediaExportResult, MediaError>;
trait StructuredMediaSource {
    type App: ArtifactApp;
    type Projection: Default + Send + semio_framework_value::RetireOwned;
    const PREPARATION_COPY_BYTES: usize;
    fn descriptor(port: &str) -> Option<(MediaType, &'static str)>;
    fn prepare(port: &str, snapshot: &<Self::App as ArtifactApp>::Snapshot, state: &mut Self::Projection) -> Result<bool, &'static str>;
    fn borrow<'a>(port: &str, snapshot: &'a <Self::App as ArtifactApp>::Snapshot, state: &'a Self::Projection)
        -> Result<impl semio_framework_pack_json::JsonWriteSource + 'a, ValueError>;
}
fn header(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}
"""
source = [scope]
units = ["structured-json-turn", "structured-prepare", "map-json-validation", "structured-media-buffers"]
for unit in units:
    code = (ticket / f"gis-{unit}.rs").read_text()
    if unit.startswith("structured-") and unit not in ("structured-media-buffers",):
        name = unit.replace("-", "_")
        source.append(f"mod {name} {{\n{code}\n}}\nuse {name}::*;")
    else:
        source.append(code)
views = {
    "terrain-mesh-projection": """
use semio_framework_surface_terrain::projection;
fn normalize3(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let length = (x * x + y * y + z * z).sqrt().max(1e-9);
    (x / length, y / length, z / length)
}
""",
    "terrain-json-view": "use super::terrain_mesh_projection::TerrainGridMeshProjection;",
    "map-json-view": """
struct MapFeature { data: semio_framework_value::DslValue }
struct GisMapSnapshot { positions: Vec<MapFeature>, routes: Vec<MapFeature>, regions: Vec<MapFeature> }
""",
    "elevation-json-view": """
#[derive(semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct ElevationSource { bounds: [f64; 4], rows: Vec<Vec<i16>> }
""",
}
for unit, prefix in views.items():
    name = unit.replace("-", "_")
    code = (ticket / f"gis-{unit}.rs").read_text()
    source.append(f"mod {name} {{\n{prefix}\n{code}\n}}")
subprocess.run(
    ["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "gis_structured_projection_check",
     "--target=wasm32-wasip2", "--crate-type=rlib", "--emit=metadata", "-L", f"dependency={dependencies}",
     "--out-dir", str(scratch), *externs, "-"], input="\n".join(source), text=True, check=True,
)
