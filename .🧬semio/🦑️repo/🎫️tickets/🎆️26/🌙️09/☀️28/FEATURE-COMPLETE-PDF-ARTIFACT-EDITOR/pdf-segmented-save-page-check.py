"""🔧 Verify the staged Save adapter with the actual framework output queue and Original job caller."""
import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "save-page"
CACHE = Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)
target = OUT / ("check.exe" if os.name == "nt" else "check")
entry = TICKET / "pdf-segmented-save-page-test.rs"
if sys.argv[4:] == ["writer"]:
    subprocess.run(["repo", "tools", "exec", "python3", "--", str(TICKET / "paged-jpeg-writer-stage.py")], check=True)
    source = TICKET / "🗑️generated" / "paged-jpeg-writer"
    stage = TICKET / "🗑️generated" / "segmented-writer"
    stage.mkdir(exist_ok=True)
    for name in ("writer.rs", "🌊️dct-object.rs", "📖️dct-view.rs", "probe.rs", "identity.rs", "lexer.rs",
                 "filters.rs", "xref.rs", "byte-source.rs", "hash.rs"):
        text = (source / name).read_text(encoding="utf8").replace("semio_framework_value", "pdf_probe_paged_value")
        if name == "hash.rs":
            test_link = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'
            if text.count(test_link) != 1:
                raise ValueError("Unexpected writer hash test link")
            text = text.replace(test_link, "", 1)
        (stage / name).write_text(text, encoding="utf8")
    entry = stage / "segmented-probe.rs"
    modules = '''
#[path = "../../pdf-segmented-save-page-test.rs"]
mod output_stage;
#[path = "../../pdf-segmented-save-writer-test.rs"]
mod writer_stage;
'''
    entry.write_text((stage / "probe.rs").read_text(encoding="utf8") + modules, encoding="utf8")
args = ["--edition=2021", "--test", "--crate-name", "pdf_save_page_test", "-O", str(entry), "-o", str(target)]
selected = {
    "semio_framework_job": sys.argv[2],
    "semio_framework_value": sys.argv[3],
    "semio_plugin_decl_app": "*",
    "semio_plugin_decl_media": "*",
    "semio_plugin_job_receiving": "*",
}
for name, unit in selected.items():
    library = next(CACHE.glob(f"*/{unit}/out/lib{name}-*.rlib"))
    args += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
args += [f"-Ldependency={directory}" for directory in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib", ".dll", ".so")})]
response = OUT / "arguments"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
fixture = TICKET / "🗑️generated" / "paged-jpeg-assets" / "large.stream.pdf"
subprocess.run([str(target), "--nocapture"], env={**os.environ, "SEMIO_PDF_SAVE_PAGE_FIXTURE": str(fixture)}, check=True)
