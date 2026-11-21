"""🎟️ Type-check the staged job's checkpoint and close signatures using exact cached first-party artifacts."""
from pathlib import Path
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "🗑️generated" / "classic-graph"
CACHE = Path(sys.argv[1])
publication = next(CACHE.glob("semio-framework-job-publication/*/out/libsemio_framework_job_publication-*.rlib"))
def dependencies(path):
    declaration = path.with_suffix(".rmeta.semio-deps")
    return {key.rsplit("-", 1)[0]: digest for key, digest in json.loads(declaration.read_text(encoding="utf8"))["dependencies"]}

expected = dependencies(publication)
def compatible(path):
    actual = dependencies(path)
    shared = expected.keys() & actual.keys()
    return bool(shared) and all(actual[key] == expected[key] for key in shared)

native = next(path for path in CACHE.glob("semio-framework-job/*/out/libsemio_framework_job-*.rlib") if compatible(path))
source = r'''
use semio_framework_job::{JobOutcomeBorrow, OriginalTurnFailure, StepContext,
    close_original_retained_effect_step};
use semio_framework_job_publication::{JobPublicationKind, RetainedJobPublication};

pub fn checkpoint<'a>(
    progress: &'a mut RetainedJobPublication, applied_progress: u64, cx: &mut StepContext<'_>,
) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
    progress.advance_from_source(JobPublicationKind::Checkpoint { applied_progress }, b"{\"stage\":\"pdf-save\"}", cx)
}

pub fn close(progress: &mut RetainedJobPublication, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let demand = progress.retirement_demands()?;
    close_original_retained_effect_step(cx, demand, |effect| progress.close_step(effect).result)?;
    let _ = progress.terminal_is_empty();
    Ok(())
}
'''
entry = OUT / "checkpoint-link.rs"
entry.write_text(source, encoding="utf8")
paths = sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".so", ".dylib", ".dll")})
response = OUT / "checkpoint-link.args"
args = ["--edition=2021", "--crate-name", "pdf_save_checkpoint_link", "--crate-type", "rlib",
    "--emit=metadata", str(entry), "-o", str(OUT / "checkpoint-link.rmeta"),
    "--extern", f"semio_framework_job_publication={publication.with_suffix('.rmeta')}", "--extern", f"semio_framework_job_publication={publication}",
    "--extern", f"semio_framework_job={native.with_suffix('.rmeta')}", "--extern", f"semio_framework_job={native}",
    *(f"-Ldependency={path}" for path in paths)]
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
print("[DEBUG] Staged PDF checkpoint and paid retirement signatures type-checked against exact repository job artifacts")
