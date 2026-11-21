from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated" / "gis-next"
dependencies = ticket / "🗑️generated" / "gis-projection-check" / "d"
externs = []
for name in ["semio_framework", "semio_framework_job", "semio_framework_pack_json", "semio_framework_surface_terrain", "semio_framework_value", "semio_plugin", "semio_plugin_decl_app"]:
    path, = dependencies.glob(f"lib{name}-*.rmeta")
    externs += ["--extern", f"{name}={path}"]
job = (generated / "job.rs").read_text()
start, end = job.index("impl<S: StructuredMediaSource> ToolJobFactory"), job.index("#[derive(semio_framework_value::RetireOwned)]")
job = job[:start] + job[end:]
job = job.replace('#[path = "🔤️json/🦀️.rs"]\nmod json;', 'mod json {\n' + (generated / "json.rs").read_text().split("\n", 1)[1] + '\n}')
source = """
pub use semio_plugin::{ArtifactApp, ArtifactMediaExportJobRequest, ArtifactOwnedToolJobFactory, ArtifactReservedJob, ArtifactReservedToolJob, ArtifactToolPublicationContract, MediaConstruction, app};
mod media_contract { pub use semio_plugin_decl_app::media_contract::ArtifactMediaExportRequestRetirement; }
mod job {
""" + job.split("\n", 1)[1] + "\n}"
subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "gis_projection_job_check",
    "--target=wasm32-wasip2", "--crate-type=rlib", "--emit=metadata", "-L", f"dependency={dependencies}",
    "--out-dir", str(generated), *externs, "-"], input=source, text=True, check=True)
