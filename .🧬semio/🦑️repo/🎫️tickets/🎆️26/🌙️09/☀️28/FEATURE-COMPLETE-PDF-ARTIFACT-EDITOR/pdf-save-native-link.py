"""🧪 Checks the staged native Save package or its framework-owned boundary."""

from argparse import ArgumentParser
import json
import os
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
INPUTS = TICKET / "🗑️generated/classic-graph/registered-save-overlay.json"
OUT = TICKET / "🗑️generated/save-native-link"
PLUGIN = "semio_plugin"
CORE = ("semio_os_kernel", "semio_framework_value", "semio_framework_job_publication")
WITNESS = ("semio_framework_job", "semio_framework_job_testing_caller",
    "semio_plugin_decl_app", "semio_plugin_decl_media", "semio_plugin_job_receiving")
SCHEMA = "semio_s_artifact_stdio_pdf_schema_4"
SNAPSHOT = "semio_s_artifact_stdio_pdf_snapshot_2"
RUSTC = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]


def declarations(path):
    return dict(metadata_receipt(path)["dependencies"])


def metadata_receipt(path):
    return json.loads(path.with_suffix(".rmeta.semio-deps").read_bytes())


def metadata_key(path):
    return path.stem.removeprefix("lib")


def compatible(a, b):
    left, right = declarations(a), declarations(b)
    parent = {k.rsplit("-", 1)[0]: (k, digest) for k, digest in left.items()}
    child = {k.rsplit("-", 1)[0]: (k, digest) for k, digest in right.items()}
    return all(parent[key] == child[key] for key in parent.keys() & child.keys())


def prepare_sources():
    source = OUT / "save"
    for path, before, text in json.loads(INPUTS.read_bytes()):
        if "/💾️native-save/" not in path or not path.endswith(".rs"):
            continue
        assert before is None, path
        relative = path.split("/💾️native-save/", 1)[1]
        output = source / relative
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_bytes(text.encode())
    harness = OUT / "surface.rs"
    harness.write_text('''#[path = "save/📤️producer.rs"] mod producer;
#[path = "save/📄️page.rs"] mod page;
#[path = "save/🪟️window.rs"] mod window;
''', encoding="utf8")
    return source, harness


def prepare_witness(source, lower, with_snapshot):
    page_test = (TICKET / "pdf-segmented-save-page-test.rs").read_text()
    page_test = page_test.replace('path = "pdf-segmented-save-page.rs"', 'path = "save/📄️page.rs"')
    page_test = page_test.replace("OriginalJobDecodeOwner, OriginalJobFixtureCaller,",
        "OriginalJobDecodeOwner,").replace("use semio_framework_value::",
        "use semio_framework_job_testing_caller::OriginalJobFixtureCaller;\nuse semio_framework_value::", 1)
    (OUT / "page-witness.rs").write_text(page_test)
    window_test = (TICKET / "pdf-registered-save-window-test.rs").read_text()
    start = window_test.index("use output_stage::")
    (OUT / "window-witness.rs").write_text(window_test[start:].replace("use output_stage::", "use crate::output_stage::", 1)
        .replace("use std::io::ErrorKind;",
        "use std::io::ErrorKind;\nuse crate::{page, window};"))
    if with_snapshot:
        module = '#[path = "save/🏭️factory.rs"] mod factory;\npub use semio_s_artifact_stdio_pdf_snapshot_2::standards;'
    else:
        factory = (source / "🏭️factory.rs").read_text().split("pub struct PdfNativeExportFactory", 1)[0]
        factory = "\n".join(line for line in factory.splitlines() if not line.startswith("use crate::standards::"))
        raw_path = next(p.split("/💾️native-save/")[0] for p, _, _ in json.loads(INPUTS.read_bytes()) if p.endswith("/💾️native-save/🏭️factory.rs"))
        schema = Path(raw_path).parent / "🧬️schema/📸️snapshot/🦀️.rs"
        factory += "\n" + next(line for line in schema.read_text().splitlines() if line.startswith("pub const STDIO_PDF17_DOCUMENT_SCHEMA:"))
        (OUT / "factory-witness.rs").write_text(factory)
        module = '#[path = "factory-witness.rs"] mod factory;'
    entry = OUT / "witness.rs"
    entry.write_text(lower.read_text() + module + '''
#[path = "page-witness.rs"] mod output_stage;
#[path = "window-witness.rs"] mod window_stage;

#[test]
fn declares_the_exact_registered_native_pdf_route() {
    let io = factory::app_io();
    assert_eq!(io.artifact_schema, "stdio.pdf.1.7");
    assert_eq!(io.export_formats, ["s.stdio.pdf@1.7"]);
    assert_eq!(io.import_formats, ["s.stdio.pdf@1.7"]);
    assert_eq!(io.ports.len(), 1);
    assert_eq!(io.ports[0].id, producer::PORT);
    assert_eq!(io.ports[0].kind_id.as_deref(), Some(producer::FORMAT));
    let action = factory::export_action();
    assert_eq!(action.id, "exportMediaFile");
    assert!(!action.in_palette);
}
''')
    return entry


def compile_candidate(cache, scope):
    artifacts = {}
    for path in cache.glob("*/*/out/*.rmeta"):
        if path.with_suffix(".rmeta.semio-deps").is_file() and path.with_suffix(".rlib").is_file():
            artifacts.setdefault(metadata_key(path).rsplit("-", 1)[0], []).append(path)
    [plugin] = artifacts[PLUGIN]
    required = [plugin]
    frontier = declarations(plugin)
    linked = scope in ("witness", "factory")
    for name in CORE + (WITNESS if linked else ()):
        [path] = [p for p in artifacts[name] if metadata_key(p) in frontier]
        assert metadata_receipt(path)["svh"] == frontier[metadata_key(path)], path
        required.append(path)
    provider = SCHEMA if scope == "package" else SNAPSHOT if scope == "factory" else None
    if provider:
        candidates = [p for p in artifacts.get(provider, ()) if all(compatible(p, other) for other in required)]
        if len(candidates) != 1:
            raise SystemExit(f"Found {len(candidates)} coherent {provider} artifacts in {cache}")
        required.extend(candidates)
    native, lower = prepare_sources()
    if scope == "package":
        entry = native / "📦️packages/🦀️rust/🦀️.rs"
    elif linked:
        entry = prepare_witness(native, lower, scope == "factory")
    else:
        entry = lower
    target = OUT / ("witness.exe" if os.name == "nt" else "witness")
    mode = ["--test", "-Copt-level=1"] if linked else ["--crate-type=rlib", "--emit=metadata"]
    artifact = target if linked else OUT / f"{scope}.rmeta"
    options = ["--edition=2021", "--crate-name", "semio_s_artifact_stdio_pdf_native_save" if scope == "package" else "pdf_save_surface",
        *mode, "-Adead_code", "-Aunused_imports", str(entry), "-o", str(artifact)]
    for path in required:
        name = metadata_key(path).rsplit("-", 1)[0]
        options += ["--extern", f"{name}={path}", "--extern", f"{name}={path.with_suffix('.rlib')}"]
    directories = {path.parent for path in cache.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".so", ".dylib", ".dll")}
    options += [f"-Ldependency={p}" for p in sorted(directories)]
    response = OUT / f"{scope}.args"
    response.write_text("\n".join(options) + "\n", encoding="utf8")
    subprocess.run(RUSTC + ["@" + str(response)], check=True)
    print(f"[DEBUG] {scope} type checked using {', '.join(metadata_key(p) for p in required)}")
    if linked:
        fixture = TICKET / "🗑️generated/classic-graph/graph.pdf"
        subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(target), "--nocapture"],
            check=True, env={**os.environ, "SEMIO_PDF_SAVE_PAGE_FIXTURE": str(fixture)})


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("cache", type=Path)
    parser.add_argument("--scope", choices=("surface", "package", "witness", "factory"), default="surface")
    args = parser.parse_args()
    compile_candidate(args.cache.resolve(), args.scope)
